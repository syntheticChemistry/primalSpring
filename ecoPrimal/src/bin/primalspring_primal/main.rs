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
use primalspring::ipc::protocol::{JsonRpcError, JsonRpcResponse, error_codes};
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

fn dispatch_request(line: &str) -> JsonRpcResponse {
    let req: serde_json::Value = match serde_json::from_str(line.trim()) {
        Ok(v) => v,
        Err(_) => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_owned(),
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
            let required = primalspring::coordination::AtomicType::FullNucleus.required_primals();
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
        "capabilities.list" => success_response(
            serde_json::json!({
                "coordination": {
                    "validate_composition": "Validate an atomic composition (Tower/Node/Nest/FullNucleus)",
                    "probe_primal": "Health-check a single primal",
                    "discovery_sweep": "Discover all primals in a composition",
                },
                "health": {
                    "liveness": "Am I alive?",
                    "readiness": "Am I ready to serve? (Neural API status + discovered primals)",
                },
                "lifecycle": {
                    "status": "Report primalSpring status",
                },
            }),
            id,
        ),
        "coordination.validate_composition" => handle_validate_composition(&req["params"], id),
        "coordination.discovery_sweep" => handle_discovery_sweep(&req["params"], id),
        "coordination.neural_api_status" => {
            success_response(serde_json::json!({ "healthy": neural_api_healthy() }), id)
        }
        "lifecycle.status" => success_response(
            serde_json::json!({
                "primal": PRIMAL_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "domain": PRIMAL_DOMAIN,
                "status": "running",
            }),
            id,
        ),
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_owned(),
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
    let atomic = match atomic_str {
        "Tower" => AtomicType::Tower,
        "Node" => AtomicType::Node,
        "Nest" => AtomicType::Nest,
        "FullNucleus" => AtomicType::FullNucleus,
        _ => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_owned(),
                result: None,
                error: Some(JsonRpcError {
                    code: error_codes::INVALID_PARAMS,
                    message: format!("Unknown atomic type: {atomic_str}"),
                    data: None,
                }),
                id,
            };
        }
    };

    let result = validate_composition(atomic);
    success_response(serde_json::to_value(result).unwrap_or_default(), id)
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

fn success_response(result: serde_json::Value, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_owned(),
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
