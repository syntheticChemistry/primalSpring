// SPDX-License-Identifier: AGPL-3.0-or-later

//! primalSpring `UniBin` — coordination and composition primal.
//!
//! Runs as a JSON-RPC 2.0 server over a Unix domain socket, exposing
//! coordination capabilities: composition validation, health aggregation,
//! and discovery sweep.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use primalspring::coordination::{AtomicType, validate_composition};
use primalspring::ipc::discover::{discover_for, neural_api_healthy, socket_path};
use primalspring::ipc::protocol::{JSONRPC_VERSION, JsonRpcError, JsonRpcResponse, error_codes};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

#[derive(Parser)]
#[command(
    name = "primalspring",
    version,
    about = "Coordination and composition validation primal"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the JSON-RPC 2.0 IPC server
    Server,
    /// Show health and capability info
    Status,
    /// Show version info
    Version,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Server => run_server(),
        Commands::Status => print_status(),
        Commands::Version => {
            println!("primalspring {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

fn server_socket_path() -> PathBuf {
    socket_path(PRIMAL_NAME)
}

/// Resolve the deploy graphs directory at runtime.
///
/// Priority: `PRIMALSPRING_GRAPHS_DIR` env var, then the binary's sibling
/// `graphs/` directory, then the build-time `CARGO_MANIFEST_DIR` fallback.
fn resolve_graphs_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("PRIMALSPRING_GRAPHS_DIR") {
        return PathBuf::from(dir);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let sibling = parent.join("graphs");
            if sibling.is_dir() {
                return sibling;
            }
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs")
}

fn run_server() {
    let sock_path = server_socket_path();
    tracing::info!("{PRIMAL_NAME} server starting...");
    tracing::info!(domain = PRIMAL_DOMAIN);
    tracing::info!(socket = %sock_path.display());

    if let Some(parent) = sock_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::error!(error = %e, "failed to create socket directory");
            std::process::exit(1);
        }
    }

    let _ = std::fs::remove_file(&sock_path);
    let listener = match UnixListener::bind(&sock_path) {
        Ok(l) => l,
        Err(e) => {
            tracing::error!(error = %e, "failed to bind Unix socket");
            std::process::exit(1);
        }
    };

    tracing::info!("listening for JSON-RPC 2.0 connections");

    std::thread::spawn(move || {
        primalspring::niche::register_with_target(&sock_path);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tracing::debug!("client connected");
                if let Err(e) = handle_connection(&stream) {
                    tracing::warn!(error = %e, "connection error");
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "accept failed");
            }
        }
    }
}

fn handle_connection(stream: &std::os::unix::net::UnixStream) -> std::io::Result<()> {
    let mut writer = stream;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let response = dispatch_request(&line);
        let response_json = match serde_json::to_string(&response) {
            Ok(json) => json,
            Err(e) => {
                tracing::error!(error = %e, "failed to serialize JSON-RPC response");
                r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"internal serialization error"},"id":0}"#.to_owned()
            }
        };
        writer.write_all(response_json.as_bytes())?;
        writer.write_all(b"\n")?;
        line.clear();
    }

    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "table-driven dispatch; splitting loses readability"
)]
fn dispatch_request(line: &str) -> JsonRpcResponse {
    let req: serde_json::Value = match serde_json::from_str(line.trim()) {
        Ok(v) => v,
        Err(_) => {
            return JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: None,
                error: Some(JsonRpcError {
                    code: error_codes::PARSE_ERROR,
                    message: "Parse error".to_owned(),
                    data: None,
                }),
                id: 0,
            };
        }
    };

    let id = req["id"].as_u64().unwrap_or(0);
    let method = req["method"].as_str().unwrap_or("");

    match method {
        // ── Health probes ──
        "health.check" | "health.liveness" => success_response(
            serde_json::json!({
                "status": "healthy",
                "primal": PRIMAL_NAME,
                "domain": PRIMAL_DOMAIN,
                "version": env!("CARGO_PKG_VERSION"),
            }),
            id,
        ),
        "health.readiness" => {
            let neural_ok = neural_api_healthy();
            let required = AtomicType::FullNucleus.required_primals();
            let discovered = discover_for(required);
            let reachable = discovered.iter().filter(|d| d.socket.is_some()).count();
            success_response(
                serde_json::json!({
                    "ready": reachable > 0,
                    "neural_api": neural_ok,
                    "primals_discovered": reachable,
                    "primals_total": required.len(),
                }),
                id,
            )
        }

        // ── Identity (sourDough compliance) ──
        "identity.get" => success_response(
            serde_json::json!({
                "id": PRIMAL_NAME,
                "display_name": "primalSpring",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": primalspring::niche::CAPABILITIES,
                "phase": "running",
                "family_id": std::env::var("FAMILY_ID")
                    .or_else(|_| std::env::var("BIOMEOS_FAMILY_ID"))
                    .unwrap_or_else(|_| "default".to_owned()),
            }),
            id,
        ),

        // ── Capability advertisement ──
        "capabilities.list" | "capability.list" => {
            let caps: Vec<&str> = primalspring::niche::CAPABILITIES.to_vec();
            success_response(
                serde_json::json!({
                    "capabilities": caps,
                    "semantic_mappings": primalspring::niche::coordination_semantic_mappings(),
                    "operation_dependencies": primalspring::niche::operation_dependencies(),
                    "cost_estimates": primalspring::niche::cost_estimates(),
                }),
                id,
            )
        }

        // ── Coordination domain ──
        "coordination.validate_composition" => handle_validate_composition(&req["params"], id),
        "coordination.discovery_sweep" => handle_discovery_sweep(&req["params"], id),
        "coordination.probe_primal" => handle_probe_primal(&req["params"], id),
        "coordination.deploy_atomic" => handle_deploy_atomic(&req["params"], id),
        "coordination.bonding_test" => handle_bonding_test(&req["params"], id),
        "coordination.neural_api_status" => {
            success_response(serde_json::json!({ "healthy": neural_api_healthy() }), id)
        }

        // ── Composition health (per-tier) ──
        "composition.tower_health" => handle_composition_health(AtomicType::Tower, id),
        "composition.node_health" => handle_composition_health(AtomicType::Node, id),
        "composition.nest_health" => handle_composition_health(AtomicType::Nest, id),
        "composition.nucleus_health" => handle_composition_health(AtomicType::FullNucleus, id),

        // ── Lifecycle management ──
        "nucleus.start" => handle_nucleus_lifecycle("start", &req["params"], id),
        "nucleus.stop" => handle_nucleus_lifecycle("stop", &req["params"], id),
        "lifecycle.status" => success_response(
            serde_json::json!({
                "primal": PRIMAL_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "domain": PRIMAL_DOMAIN,
                "status": "running",
            }),
            id,
        ),

        // ── MCP tool discovery ──
        "mcp.tools.list" => {
            let tools = primalspring::ipc::mcp::list_tools();
            success_response(serde_json::to_value(tools).unwrap_or_default(), id)
        }

        // ── Graph coordination ──
        "graph.list" => {
            let graphs_dir = resolve_graphs_dir();
            let results = primalspring::deploy::validate_all_graphs(&graphs_dir);
            success_response(serde_json::to_value(results).unwrap_or_default(), id)
        }
        "graph.validate" => handle_graph_validate(&req["params"], id),

        _ => JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: Some(JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: format!("Method not found: {method}"),
                data: None,
            }),
            id,
        },
    }
}

fn handle_validate_composition(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("Tower");
    let Some(atomic) = parse_atomic_type(atomic_str) else {
        return error_response(
            error_codes::INVALID_PARAMS,
            &format!("Unknown atomic type: {atomic_str}"),
            id,
        );
    };

    let result = validate_composition(atomic);
    match serde_json::to_value(result) {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

fn handle_discovery_sweep(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("FullNucleus");
    let primals = match atomic_str {
        "Tower" => AtomicType::Tower.required_primals(),
        "Node" => AtomicType::Node.required_primals(),
        "Nest" => AtomicType::Nest.required_primals(),
        _ => AtomicType::FullNucleus.required_primals(),
    };

    let results = discover_for(primals);
    let summary: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "primal": r.primal,
                "socket": r.socket.as_ref().map(|p| p.display().to_string()),
                "source": format!("{:?}", r.source),
            })
        })
        .collect();

    success_response(serde_json::json!({ "primals": summary }), id)
}

fn handle_probe_primal(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let name = params["primal"].as_str().unwrap_or("beardog");
    let health = primalspring::coordination::probe_primal(name);
    match serde_json::to_value(health) {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

fn handle_deploy_atomic(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("Tower");
    let Some(atomic) = parse_atomic_type(atomic_str) else {
        return error_response(
            error_codes::INVALID_PARAMS,
            &format!("Unknown atomic type: {atomic_str}"),
            id,
        );
    };

    let graphs_dir = resolve_graphs_dir();
    let graph_file = graphs_dir.join(format!("{}.toml", atomic.graph_name()));

    let structure_ok = if graph_file.exists() {
        let result = primalspring::deploy::validate_structure(&graph_file);
        serde_json::to_value(&result).ok()
    } else {
        None
    };

    let composition = validate_composition(atomic);
    success_response(
        serde_json::json!({
            "atomic": atomic_str,
            "graph": atomic.graph_name(),
            "graph_exists": graph_file.exists(),
            "graph_path": graph_file.display().to_string(),
            "graph_validation": structure_ok,
            "composition": serde_json::to_value(&composition).unwrap_or_default(),
        }),
        id,
    )
}

fn handle_bonding_test(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let bond_str = params["bond_type"].as_str().unwrap_or("Covalent");
    let bond = match bond_str {
        "Covalent" => primalspring::bonding::BondType::Covalent,
        "Ionic" => primalspring::bonding::BondType::Ionic,
        "Weak" => primalspring::bonding::BondType::Weak,
        "OrganoMetalSalt" => primalspring::bonding::BondType::OrganoMetalSalt,
        _ => {
            return error_response(
                error_codes::INVALID_PARAMS,
                &format!("Unknown bond type: {bond_str}"),
                id,
            );
        }
    };

    let required = AtomicType::FullNucleus.required_primals();
    let discovered = discover_for(required);
    let gates = discovered.iter().filter(|d| d.socket.is_some()).count();

    success_response(
        serde_json::json!({
            "bond_type": bond_str,
            "description": bond.description(),
            "gates_discovered": gates,
            "total_primals": required.len(),
            "status": if gates >= 2 { "ready" } else { "insufficient_primals" },
        }),
        id,
    )
}

fn handle_composition_health(atomic: AtomicType, id: u64) -> JsonRpcResponse {
    let result = validate_composition(atomic);
    match serde_json::to_value(result) {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

fn handle_nucleus_lifecycle(action: &str, params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("FullNucleus");
    let atomic = parse_atomic_type(atomic_str).unwrap_or(AtomicType::FullNucleus);

    let graphs_dir = resolve_graphs_dir();
    let graph_file = graphs_dir.join(format!("{}.toml", atomic.graph_name()));

    success_response(
        serde_json::json!({
            "action": action,
            "atomic": atomic_str,
            "graph": atomic.graph_name(),
            "graph_exists": graph_file.exists(),
            "required_primals": atomic.required_primals(),
            "status": if graph_file.exists() { "graph_ready" } else { "graph_missing" },
            "note": format!("nucleus.{action} queued — biomeOS orchestrates actual deployment via deploy graph"),
        }),
        id,
    )
}

fn parse_atomic_type(s: &str) -> Option<AtomicType> {
    match s {
        "Tower" => Some(AtomicType::Tower),
        "Node" => Some(AtomicType::Node),
        "Nest" => Some(AtomicType::Nest),
        "FullNucleus" | "Full" => Some(AtomicType::FullNucleus),
        _ => None,
    }
}

fn error_response(code: i64, message: &str, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_owned(),
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.to_owned(),
            data: None,
        }),
        id,
    }
}

fn handle_graph_validate(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let Some(path_str) = params["path"].as_str() else {
        return error_response(
            error_codes::INVALID_PARAMS,
            "missing required 'path' parameter",
            id,
        );
    };
    let live = params["live"].as_bool().unwrap_or(false);
    let path = std::path::Path::new(path_str);
    let result = if live {
        serde_json::to_value(primalspring::deploy::validate_live(path))
    } else {
        serde_json::to_value(primalspring::deploy::validate_structure(path))
    };
    match result {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

fn success_response(result: serde_json::Value, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_owned(),
        result: Some(result),
        error: None,
        id,
    }
}

fn print_status() {
    println!("{PRIMAL_NAME} v{}", env!("CARGO_PKG_VERSION"));
    println!("domain: {PRIMAL_DOMAIN}");
    println!("tracks: 6 (atomic, graph, emergent, bonding, coralforge, cross-spring)");

    let neural_ok = neural_api_healthy();
    println!(
        "neural_api: {}",
        if neural_ok { "reachable" } else { "not found" }
    );

    let required = AtomicType::FullNucleus.required_primals();
    let discovered = discover_for(required);
    let found = discovered.iter().filter(|d| d.socket.is_some()).count();
    println!("primals: {found}/{} discovered", required.len());

    for d in &discovered {
        let status = if d.socket.is_some() { "UP" } else { "DOWN" };
        println!("  [{status}] {}", d.primal);
    }
}
