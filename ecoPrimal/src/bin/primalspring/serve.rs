// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC 2.0 IPC server — the cell membrane of the eukaryotic UniBin.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;

use primalspring::ipc::method_gate::{CallerContext, MethodGate};
use primalspring::ipc::protocol::{JSONRPC_VERSION, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use primalspring::ipc::server_bind::{BindMode, BoundTransport, bind_transport};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

pub fn run() {
    let mode = BindMode::from_env();
    let gate = MethodGate::from_env();

    tracing::info!("{PRIMAL_NAME} server starting...");
    tracing::info!(domain = PRIMAL_DOMAIN);
    tracing::info!(bind_mode = ?mode, "transport bind mode");
    tracing::info!(auth_mode = gate.mode().as_str(), "method gate initialized");

    let bound = match bind_transport(PRIMAL_NAME, mode) {
        Ok(b) => b,
        Err(e) => {
            tracing::error!(error = %e, "failed to bind transport");
            std::process::exit(1);
        }
    };

    tracing::info!(
        endpoint = bound.endpoint_display(),
        "listening for JSON-RPC 2.0 connections"
    );

    match bound {
        BoundTransport::Unix(listener, sock_path) => {
            std::thread::spawn(move || {
                primalspring::niche::register_with_target(&sock_path);
            });
            serve_unix(&listener, &gate);
        }
        BoundTransport::Tcp(listener, _addr) => {
            serve_tcp(&listener, &gate);
        }
    }
}

fn serve_unix(listener: &UnixListener, gate: &MethodGate) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tracing::debug!("client connected (unix)");
                if let Err(e) = handle_unix_connection(&stream, gate) {
                    tracing::warn!(error = %e, "connection error");
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "accept failed");
            }
        }
    }
}

fn serve_tcp(listener: &std::net::TcpListener, gate: &MethodGate) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tracing::debug!("client connected (tcp)");
                if let Err(e) = handle_tcp_connection(&stream, gate) {
                    tracing::warn!(error = %e, "connection error");
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "accept failed");
            }
        }
    }
}

fn handle_unix_connection(
    stream: &std::os::unix::net::UnixStream,
    gate: &MethodGate,
) -> std::io::Result<()> {
    let caller = CallerContext::from_unix_stream(stream);
    let mut writer = stream;
    let mut reader = BufReader::new(stream);
    serve_rpc_lines(&mut reader, &mut writer, &caller, gate)
}

fn handle_tcp_connection(stream: &std::net::TcpStream, gate: &MethodGate) -> std::io::Result<()> {
    let caller = CallerContext::loopback();
    let mut writer = stream;
    let mut reader = BufReader::new(stream);
    serve_rpc_lines(&mut reader, &mut writer, &caller, gate)
}

fn serve_rpc_lines(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    caller: &CallerContext,
    gate: &MethodGate,
) -> std::io::Result<()> {
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let response = dispatch_gated(&line, caller, gate);
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

fn dispatch_gated(line: &str, base_caller: &CallerContext, gate: &MethodGate) -> JsonRpcResponse {
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
        other => {
            return JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("unknown auth method: {other}"),
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
            let caps = primalspring::coordination::AtomicType::FullNucleus.required_capabilities();
            let results = primalspring::ipc::discover::discover_capabilities_for(caps);
            let reachable = results.iter().filter(|r| r.socket.is_some()).count();
            let ready = reachable > 0;
            serde_json::json!({
                "status": if ready { "ok" } else { "degraded" },
                "primal": "primalspring",
                "ready": ready,
                "capabilities_discovered": reachable,
                "capabilities_total": caps.len(),
            })
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
