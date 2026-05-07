// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

use primalspring::ipc::method_gate::{CallerContext, MethodGate};
use primalspring::ipc::protocol::{JSONRPC_VERSION, JsonRpcResponse};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

use crate::dispatch::dispatch_request;

pub fn server_socket_path() -> PathBuf {
    primalspring::ipc::discover::socket_path(PRIMAL_NAME)
}

/// Resolve the deploy graphs directory at runtime.
///
/// Priority: `PRIMALSPRING_GRAPHS_DIR` env var, then the binary's sibling
/// `graphs/` directory, then the build-time `CARGO_MANIFEST_DIR` fallback.
pub fn resolve_graphs_dir() -> PathBuf {
    if let Ok(dir) = std::env::var(primalspring::env_keys::PRIMALSPRING_GRAPHS_DIR) {
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

pub fn run_server() {
    let sock_path = server_socket_path();
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
        let response = dispatch_request_gated(&line, &caller, gate);
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

/// Pre-dispatch gate: extract the method name, run `MethodGate::check`,
/// then delegate to `dispatch_request` if allowed.
fn dispatch_request_gated(
    line: &str,
    caller: &CallerContext,
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

    if let Err(err) = gate.check(normalized, caller) {
        return JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: Some(err),
            id,
        };
    }

    // Handle auth introspection methods here (they need gate/caller context).
    match normalized {
        "auth.mode" => JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(serde_json::json!({
                "mode": gate.mode().as_str(),
            })),
            error: None,
            id,
        },
        "auth.check" => {
            let has_token = caller.bearer_token.is_some();
            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: Some(serde_json::json!({
                    "authenticated": has_token,
                    "enforcement": gate.mode().as_str(),
                })),
                error: None,
                id,
            }
        }
        "auth.peer_info" => JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(serde_json::json!({
                "origin": format!("{:?}", caller.origin),
                "peer": caller.peer.as_ref().map(|p| serde_json::json!({
                    "uid": p.uid,
                    "pid": p.pid,
                })),
            })),
            error: None,
            id,
        },
        _ => dispatch_request(line),
    }
}
