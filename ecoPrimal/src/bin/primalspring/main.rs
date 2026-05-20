// SPDX-License-Identifier: AGPL-3.0-or-later

//! primalSpring UniBin — the eukaryotic cell.
//!
//! Single binary consolidating certification (guidestone organelle),
//! validation scenarios (experiment ribosomes), and IPC server (cell membrane).
//!
//! Evolved from the prokaryotic era of separate binaries during the
//! interstadial transition.

#![deny(unsafe_code)]

mod cli;

use clap::Parser;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let parsed = cli::Cli::parse();

    match parsed.command {
        cli::Commands::Certify { layer, bare, format: _ } => cmd_certify(layer, bare),
        cli::Commands::Validate {
            ref track,
            ref scenario,
            ref tier,
            list,
            format,
            ref provenance_dir,
        } => cmd_validate(
            track.as_deref(),
            scenario.as_deref(),
            tier.as_deref(),
            list,
            matches!(format, cli::OutputFormat::Json),
            provenance_dir.as_deref(),
        ),
        cli::Commands::Serve => cmd_serve(),
        cli::Commands::Status => cmd_status(),
        cli::Commands::Version => cmd_version(),
    }
}

fn cmd_certify(layer: Option<u8>, bare: bool) {
    let max_layer = if bare {
        0
    } else {
        layer.unwrap_or(primalspring::certification::MAX_LAYER)
    };

    let result = primalspring::certification::certify(max_layer);
    std::process::exit(result.exit_code());
}

fn cmd_validate(
    track: Option<&str>,
    scenario_id: Option<&str>,
    tier: Option<&str>,
    list: bool,
    json: bool,
    provenance_dir: Option<&str>,
) {
    use primalspring::validation::scenarios::{Tier, Track, build_registry};

    let registry = build_registry();

    if list {
        println!(
            "primalSpring Validation Scenarios ({} registered)\n",
            registry.len()
        );
        let hdr_scenario = "SCENARIO";
        let hdr_track = "TRACK";
        let hdr_tier = "TIER";
        let hdr_provenance = "PROVENANCE";
        println!("{hdr_scenario:<30} {hdr_track:<25} {hdr_tier:<6} {hdr_provenance}");
        println!("{}", "-".repeat(90));
        for s in registry.all() {
            println!(
                "{:<30} {:<25} {:<6} {}",
                s.meta.id, s.meta.track, s.meta.tier, s.meta.provenance_crate
            );
        }
        return;
    }

    let tier_filter: Option<Tier> = tier.map(|t| {
        Tier::from_str_loose(t).unwrap_or_else(|| {
            eprintln!("unknown tier: {t} (expected: rust, live, both)");
            std::process::exit(1);
        })
    });

    let track_filter: Option<Track> = track.and_then(|t| {
        Track::from_str_loose(t).or_else(|| {
            eprintln!("unknown track: {t}");
            std::process::exit(1);
        })
    });

    let mut v = primalspring::validation::ValidationResult::new(
        "primalSpring Validation — Scenario Runner",
    );
    let mut ctx = primalspring::composition::CompositionContext::discover();

    primalspring::validation::ValidationResult::print_banner(
        "primalSpring Validation — Scenario Runner",
    );

    let mut ran = 0usize;
    for s in registry.all() {
        if let Some(id) = scenario_id {
            if s.meta.id != id {
                continue;
            }
        }
        if let Some(track_f) = track_filter {
            if s.meta.track != track_f {
                continue;
            }
        }
        if let Some(tier_f) = tier_filter {
            if tier_f != Tier::Both && s.meta.tier != tier_f && s.meta.tier != Tier::Both {
                continue;
            }
        }

        v.section(&format!(
            "Scenario: {} [{}] ({})",
            s.meta.id, s.meta.track, s.meta.tier
        ));
        (s.run)(&mut v, &mut ctx);
        ran += 1;
    }

    if ran == 0 {
        eprintln!("no scenarios matched the filter criteria");
        std::process::exit(1);
    }

    if let Some(dir) = provenance_dir {
        write_provenance(&v, dir, ran);
    }

    if json {
        if let Ok(j) = v.to_json() {
            println!("{j}");
        } else {
            v.finish();
        }
    } else {
        v.finish();
    }
    std::process::exit(v.exit_code());
}

fn write_provenance(v: &primalspring::validation::ValidationResult, dir: &str, scenarios_run: usize) {
    use std::io::Write;
    let dir = std::path::Path::new(dir);
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("warning: could not create provenance dir {}: {e}", dir.display());
        return;
    }

    if let Ok(json) = v.to_json() {
        let path = dir.join("results.json");
        if let Ok(mut f) = std::fs::File::create(&path) {
            let _ = f.write_all(json.as_bytes());
            eprintln!("provenance: wrote {}", path.display());
        }
    }

    let today = {
        let d = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let days = d / 86400;
        let y = 1970 + (days * 400) / 146097;
        format!("{y}-{:02}-{:02}", (days % 365) / 30 + 1, (days % 365) % 30 + 1)
    };

    let provenance_toml = format!(
        r#"[run]
spring = "primalSpring"
version = "{version}"
date = "{today}"
threads = ["10"]
tier = 2
scenarios_run = {scenarios_run}

[environment]
gate = "irongate"
nucleus_composition = "full"
host = "{host}"

[results]
total_checks = {total}
passed = {passed}
failed = {failed}
skipped = {skipped}
"#,
        version = env!("CARGO_PKG_VERSION"),
        today = today,
        scenarios_run = scenarios_run,
        host = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "irongate-local".into()),
        total = v.evaluated(),
        passed = v.passed,
        failed = v.failed,
        skipped = v.skipped,
    );

    let path = dir.join("provenance.toml");
    if let Ok(mut f) = std::fs::File::create(&path) {
        let _ = f.write_all(provenance_toml.as_bytes());
        eprintln!("provenance: wrote {}", path.display());
    }
}

fn cmd_serve() {
    serve::run();
}

mod serve {
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::UnixListener;

    use primalspring::ipc::method_gate::{CallerContext, MethodGate};
    use primalspring::ipc::protocol::{
        JSONRPC_VERSION, JsonRpcError, JsonRpcRequest, JsonRpcResponse,
    };
    use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

    pub fn run() {
        let sock_path = primalspring::ipc::discover::socket_path(PRIMAL_NAME);
        let gate = MethodGate::from_env();

        tracing::info!("{PRIMAL_NAME} server starting...");
        tracing::info!(domain = PRIMAL_DOMAIN);
        tracing::info!(socket = %sock_path.display());
        tracing::info!(auth_mode = gate.mode().as_str(), "method gate initialized");

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
                    if let Err(e) = handle_connection(&stream, &gate) {
                        tracing::warn!(error = %e, "connection error");
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, "accept failed");
                }
            }
        }
    }

    fn handle_connection(
        stream: &std::os::unix::net::UnixStream,
        gate: &MethodGate,
    ) -> std::io::Result<()> {
        let caller = CallerContext::from_unix_stream(stream);
        let mut writer = stream;
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        while reader.read_line(&mut line)? > 0 {
            let response = dispatch_gated(&line, &caller, gate);
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

    fn dispatch_gated(
        line: &str,
        base_caller: &CallerContext,
        gate: &MethodGate,
    ) -> JsonRpcResponse {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(line.trim());
        let method = parsed
            .as_ref()
            .ok()
            .and_then(|v| v["method"].as_str())
            .unwrap_or("");
        let normalized = primalspring::ipc::normalize_method(method);
        let id = parsed
            .as_ref()
            .ok()
            .and_then(|v| v["id"].as_u64())
            .unwrap_or(0);

        let params = parsed
            .as_ref()
            .ok()
            .and_then(|v| v.get("params"))
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let caller = base_caller
            .clone()
            .with_params_token(&params, gate.verifier());

        if let Err(err) = gate.check(normalized, &caller) {
            return JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: None,
                error: Some(err),
                id,
            };
        }

        match normalized {
            "auth.mode" | "auth.check" | "auth.peer_info" => {
                dispatch_auth(normalized, &caller, gate, id)
            }
            _ => dispatch_request(line),
        }
    }

    fn dispatch_auth(
        method: &str,
        caller: &CallerContext,
        gate: &MethodGate,
        id: u64,
    ) -> JsonRpcResponse {
        let result = match method {
            "auth.mode" => serde_json::json!({ "mode": gate.mode().as_str() }),
            "auth.check" => {
                let has_token = caller.bearer_token.is_some();
                let verified = caller.verified.is_some();
                let mut r = serde_json::json!({
                    "authenticated": has_token,
                    "verified": verified,
                    "enforcement": gate.mode().as_str(),
                });
                if let Some(ref v) = caller.verified {
                    r["scopes"] = serde_json::json!(v.scopes);
                    if let Some(ref sub) = v.subject {
                        r["subject"] = serde_json::json!(sub);
                    }
                    if let Some(exp) = v.expires_in {
                        r["expires_in"] = serde_json::json!(exp);
                    }
                }
                r
            }
            "auth.peer_info" => serde_json::json!({
                "origin": format!("{:?}", caller.origin),
                "peer": caller.peer.as_ref().map(|p| serde_json::json!({
                    "uid": p.uid,
                    "pid": p.pid,
                })),
            }),
            _ => unreachable!(),
        };

        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(result),
            error: None,
            id,
        }
    }

    fn dispatch_request(raw_request: &str) -> JsonRpcResponse {
        let request: JsonRpcRequest = match serde_json::from_str(raw_request.trim()) {
            Ok(r) => r,
            Err(_) => {
                return JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_owned(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "parse error".to_owned(),
                        data: None,
                    }),
                    id: 0,
                };
            }
        };

        let method = primalspring::ipc::normalize_method(&request.method);
        let id = request.id;

        let result: serde_json::Value = match method {
            "health.check" | "health.liveness" => {
                serde_json::json!({"status": "ok", "primal": "primalspring"})
            }
            "health.readiness" => {
                serde_json::json!({"status": "ok", "primal": "primalspring", "ready": true})
            }
            "capabilities.list" | "capability.list" => {
                let caps = primalspring::niche::all_capabilities();
                serde_json::json!({
                    "capabilities": caps,
                    "count": caps.len(),
                    "primal": primalspring::PRIMAL_NAME,
                })
            }
            "coordination.status" => {
                serde_json::json!({
                    "primal": "primalspring",
                    "version": env!("CARGO_PKG_VERSION"),
                    "domain": primalspring::PRIMAL_DOMAIN,
                })
            }
            _ => {
                return JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_owned(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32601,
                        message: format!("method not found: {method}"),
                        data: None,
                    }),
                    id,
                };
            }
        };

        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(result),
            error: None,
            id,
        }
    }
}

fn cmd_status() {
    use primalspring::coordination::AtomicType;
    use primalspring::ipc::discover::{discover_capabilities_for, neural_api_healthy};
    use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

    println!("{PRIMAL_NAME} v{}", env!("CARGO_PKG_VERSION"));
    println!("domain: {PRIMAL_DOMAIN}");
    println!(
        "local methods: {} | routed: {}",
        primalspring::niche::LOCAL_CAPABILITIES.len(),
        primalspring::niche::ROUTED_CAPABILITIES.len(),
    );

    let neural_ok = neural_api_healthy();
    println!(
        "neural_api: {}",
        if neural_ok { "reachable" } else { "not found" }
    );

    let capabilities = AtomicType::FullNucleus.required_capabilities();
    let discovered = discover_capabilities_for(capabilities);
    let found = discovered.iter().filter(|d| d.socket.is_some()).count();
    println!("capabilities: {found}/{} discovered", capabilities.len());

    for d in &discovered {
        let status = if d.socket.is_some() { "UP" } else { "DOWN" };
        let provider = d.resolved_primal.as_deref().unwrap_or("unresolved");
        println!("  [{status}] {} (via {provider})", d.capability);
    }
}

fn cmd_version() {
    println!("primalspring {}", env!("CARGO_PKG_VERSION"));
}
