// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use primalspring::ipc::method_gate::{CallerContext, MethodGate};
use primalspring::ipc::platform::PlatformCapabilities;
use primalspring::ipc::protocol::{JSONRPC_VERSION, JsonRpcResponse};
use primalspring::ipc::server_bind::{BindMode, BoundTransport};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

use crate::dispatch::{self, dispatch_request};

/// Reads `PRIMALSPRING_SOCKET_MODE` env var (octal string, e.g. `"0660"`).
/// Falls back to `PRIMAL_SOCKET_MODE` for generic convention (SP-01).
fn resolve_socket_mode() -> Option<u32> {
    let raw = std::env::var(primalspring::env_keys::PRIMALSPRING_SOCKET_MODE)
        .or_else(|_| std::env::var(primalspring::env_keys::PRIMAL_SOCKET_MODE))
        .ok()?;
    u32::from_str_radix(raw.trim_start_matches('0'), 8).ok()
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

/// Resolve `BindMode` from CLI flag, env var, or platform detection.
fn resolve_bind_mode(cli_mode: Option<&str>) -> BindMode {
    if let Some(mode_str) = cli_mode {
        return match mode_str.to_lowercase().as_str() {
            "tcp_only" | "tcp" => BindMode::TcpOnly,
            "fallback" | "auto" => BindMode::Fallback,
            _ => BindMode::UdsOnly,
        };
    }
    let caps = PlatformCapabilities::detect();
    caps.log_summary();
    caps.recommended_bind_mode()
}

pub fn run_server(cli_bind_mode: Option<&str>, _cli_port: Option<u16>) {
    dispatch::init_startup_time();

    let gate = MethodGate::from_env();
    let mode = resolve_bind_mode(cli_bind_mode);

    tracing::info!("{PRIMAL_NAME} server starting...");
    tracing::info!(domain = PRIMAL_DOMAIN);
    tracing::info!(bind_mode = ?mode, "transport selection");
    tracing::info!(auth_mode = gate.mode().as_str(), "method gate initialized");

    let bound = match primalspring::ipc::server_bind::bind_transport(PRIMAL_NAME, mode) {
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

    if let BoundTransport::Unix(_, ref path) = bound {
        if let Some(mode) = resolve_socket_mode() {
            use std::os::unix::fs::PermissionsExt;
            if let Err(e) = std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)) {
                tracing::warn!(error = %e, mode = format!("{mode:04o}"), "failed to set socket permissions");
            } else {
                tracing::info!(mode = format!("{mode:04o}"), "socket permissions set");
            }
        }

        let register_path = path.clone();
        std::thread::spawn(move || {
            primalspring::niche::register_with_target(&register_path);
        });
    }

    match &bound {
        BoundTransport::Unix(listener, _) => serve_unix(listener, &gate),
        BoundTransport::Tcp(listener, _) => serve_tcp(listener, &gate),
    }
}

fn serve_unix(listener: &std::os::unix::net::UnixListener, gate: &MethodGate) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tracing::debug!("client connected (UDS)");
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
                tracing::debug!("client connected (TCP)");
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
    let reader = BufReader::new(stream);
    process_lines(reader, &mut writer, &caller, gate)
}

fn handle_tcp_connection(stream: &std::net::TcpStream, gate: &MethodGate) -> std::io::Result<()> {
    let caller = CallerContext::loopback();
    let mut writer = stream;
    let reader = BufReader::new(stream);
    process_lines(reader, &mut writer, &caller, gate)
}

fn process_lines<R: BufRead, W: Write>(
    mut reader: R,
    writer: &mut W,
    caller: &CallerContext,
    gate: &MethodGate,
) -> std::io::Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let response = dispatch_request_gated(&line, caller, gate);
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

/// Pre-dispatch gate: extract the method name, parse bearer token from
/// params, run `MethodGate::check` with scope validation, then delegate
/// to `dispatch_request` if allowed.
fn dispatch_request_gated(
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
            let verified = caller.verified.is_some();
            let mut result = serde_json::json!({
                "authenticated": has_token,
                "verified": verified,
                "enforcement": gate.mode().as_str(),
            });
            if let Some(ref v) = caller.verified {
                result["scopes"] = serde_json::json!(v.scopes);
                if let Some(ref sub) = v.subject {
                    result["subject"] = serde_json::json!(sub);
                }
                if let Some(exp) = v.expires_in {
                    result["expires_in"] = serde_json::json!(exp);
                }
            }
            JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: Some(result),
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
