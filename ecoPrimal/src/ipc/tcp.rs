// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP JSON-RPC client for cross-gate remote probing.
//!
//! On a single machine, primals communicate via Unix sockets discovered by
//! biomeOS. When probing a **remote** gate (another machine's NUCLEUS),
//! experiments use TCP JSON-RPC. This module extracts the shared TCP RPC
//! pattern that was duplicated across experiments (exp063, exp073, exp074,
//! exp076, exp081, exp082, exp083, exp084).
//!
//! Timeouts are sourced from [`crate::tolerances`] — no magic numbers.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use super::error::{IpcError, classify_io_error};
use super::protocol::JsonRpcError;
use crate::tolerances;

/// Result of a TCP JSON-RPC call: the parsed result value and round-trip latency.
pub type TcpRpcResult = Result<(serde_json::Value, Duration), IpcError>;

/// Send a single JSON-RPC 2.0 request over TCP and return the result.
///
/// Uses centralized timeouts from [`tolerances`]. Connection, write, and read
/// timeouts are applied. The TCP connection is shut down for writing after
/// the request is sent (half-close), then the first JSON-RPC response line
/// is parsed and returned.
///
/// # Errors
///
/// Returns [`IpcError`] with semantic classification: `ConnectionRefused` /
/// `Timeout` for transport failures, `ApplicationError` / `MethodNotFound`
/// for JSON-RPC errors, `ProtocolError` for malformed responses.
pub fn tcp_rpc(host: &str, port: u16, method: &str, params: &serde_json::Value) -> TcpRpcResult {
    tcp_rpc_with_timeout(
        host,
        port,
        method,
        params,
        Duration::from_secs(tolerances::TCP_CONNECT_TIMEOUT_SECS),
    )
}

/// Like [`tcp_rpc`] but with a custom connect timeout.
///
/// Read and write timeouts still use the centralized tolerances values.
///
/// # Errors
///
/// Same as [`tcp_rpc`].
pub fn tcp_rpc_with_timeout(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
    connect_timeout: Duration,
) -> TcpRpcResult {
    let addr = format!("{host}:{port}");
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| IpcError::ProtocolError {
            detail: format!("invalid address {addr}: {e}"),
        })?,
        connect_timeout,
    )
    .map_err(classify_io_error)?;
    stream
        .set_read_timeout(Some(Duration::from_secs(tolerances::TCP_READ_TIMEOUT_SECS)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(
            tolerances::TCP_WRITE_TIMEOUT_SECS,
        )))
        .ok();

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = format!("{req}\n");
    stream
        .write_all(msg.as_bytes())
        .map_err(classify_io_error)?;
    stream.flush().map_err(classify_io_error)?;

    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line) {
            let elapsed = start.elapsed();
            if let Some(result) = parsed.get("result") {
                return Ok((result.clone(), elapsed));
            }
            if let Some(err_val) = parsed.get("error") {
                if let Ok(rpc_err) = serde_json::from_value::<JsonRpcError>(err_val.clone()) {
                    return Err(IpcError::from(rpc_err));
                }
                return Err(IpcError::ApplicationError {
                    code: -1,
                    message: err_val.to_string(),
                    data: None,
                });
            }
        }
    }
    Err(IpcError::ProtocolError {
        detail: format!("no JSON-RPC response from {addr}"),
    })
}

/// HTTP health probe for primals that serve HTTP (e.g. Songbird).
///
/// Sends `GET /health HTTP/1.1` and checks for a 200 OK response.
///
/// # Errors
///
/// Returns [`IpcError`] on connection failure or non-OK response.
#[deprecated(
    since = "0.71.0",
    note = "All primals expose JSON-RPC `health.liveness`. Use `tcp_rpc(host, port, \"health.liveness\", &json!({}))` instead. Songbird no longer exposes HTTP /health on a port — Tower Atomic owns all HTTP."
)]
pub fn http_health_probe(host: &str, port: u16) -> TcpRpcResult {
    let addr = format!("{host}:{port}");
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| IpcError::ProtocolError {
            detail: format!("invalid address {addr}: {e}"),
        })?,
        Duration::from_secs(tolerances::TCP_CONNECT_TIMEOUT_SECS),
    )
    .map_err(classify_io_error)?;
    stream
        .set_read_timeout(Some(Duration::from_secs(tolerances::TCP_READ_TIMEOUT_SECS)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(
            tolerances::TCP_WRITE_TIMEOUT_SECS,
        )))
        .ok();

    let http_req = format!("GET /health HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    stream
        .write_all(http_req.as_bytes())
        .map_err(classify_io_error)?;

    let mut buf = String::new();
    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        buf.push_str(&line);
        buf.push('\n');
    }
    let elapsed = start.elapsed();

    if buf.contains("200 OK")
        || buf.contains("200 Ok")
        || buf.contains("\nOK\n")
        || buf.ends_with("OK\n")
    {
        Ok((
            serde_json::json!({"status": "alive", "protocol": "http"}),
            elapsed,
        ))
    } else {
        Err(IpcError::ProtocolError {
            detail: format!("HTTP health: non-OK response from {addr}"),
        })
    }
}

/// Send a JSON-RPC 2.0 request via HTTP POST to a primal's `/jsonrpc` endpoint.
///
/// Some primals (notably Songbird) expose an HTTP server on TCP rather than
/// raw JSON-RPC framing. This function wraps the request in an HTTP POST to
/// `/jsonrpc` and parses the JSON-RPC response from the HTTP body.
///
/// # Errors
///
/// Returns [`IpcError`] on connection failure, timeout, HTTP error, or
/// JSON-RPC error.
pub fn http_json_rpc(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> TcpRpcResult {
    let addr = format!("{host}:{port}");
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| IpcError::ProtocolError {
            detail: format!("invalid address {addr}: {e}"),
        })?,
        Duration::from_secs(tolerances::TCP_CONNECT_TIMEOUT_SECS),
    )
    .map_err(classify_io_error)?;
    stream
        .set_read_timeout(Some(Duration::from_secs(tolerances::TCP_READ_TIMEOUT_SECS)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(
            tolerances::TCP_WRITE_TIMEOUT_SECS,
        )))
        .ok();

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    })
    .to_string();

    let http_req = format!(
        "POST /jsonrpc HTTP/1.1\r\n\
         Host: {addr}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {body}",
        body.len()
    );
    stream
        .write_all(http_req.as_bytes())
        .map_err(classify_io_error)?;

    let reader = BufReader::new(&stream);
    let mut in_body = false;
    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            in_body = true;
            continue;
        }
        if in_body {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line) {
                let elapsed = start.elapsed();
                if let Some(result) = parsed.get("result") {
                    return Ok((result.clone(), elapsed));
                }
                if let Some(err_val) = parsed.get("error") {
                    if let Ok(rpc_err) = serde_json::from_value::<JsonRpcError>(err_val.clone()) {
                        return Err(IpcError::from(rpc_err));
                    }
                    return Err(IpcError::ApplicationError {
                        code: -1,
                        message: err_val.to_string(),
                        data: None,
                    });
                }
            }
        }
    }
    Err(IpcError::ProtocolError {
        detail: format!("no HTTP JSON-RPC response from {addr}"),
    })
}

/// Try raw TCP JSON-RPC first, then fall back to HTTP `/jsonrpc`.
///
/// Useful when the caller doesn't know whether the target primal uses raw
/// TCP framing (most primals) or HTTP (Songbird).
///
/// # Errors
///
/// Returns the HTTP [`IpcError`] if both raw TCP and HTTP fail. If the raw
/// TCP attempt returned a semantic RPC error (application/method-not-found),
/// that error is returned immediately without HTTP fallback.
pub fn tcp_rpc_multi_protocol(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> TcpRpcResult {
    match tcp_rpc(host, port, method, params) {
        ok @ Ok(_) => ok,
        Err(e) if e.is_method_not_found() || matches!(e, IpcError::ApplicationError { .. }) => {
            Err(e)
        }
        Err(_transport_err) => http_json_rpc(host, port, method, params),
    }
}

/// Read a TCP port from an environment variable, falling back to a default.
///
/// Common pattern across cross-gate experiments: `BEARDOG_PORT=9100`, etc.
#[must_use]
pub fn env_port(key: &str, default: u16) -> u16 {
    std::env::var(key)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(default)
}

/// Invoke an operation via the biomeOS Neural API `capability.call` pattern.
///
/// Wraps the `capability.call` JSON-RPC method, routing to the appropriate
/// primal based on `domain` and `operation`.
///
/// # Errors
///
/// Returns [`IpcError`] on connection/timeout failure or if the Neural API
/// returns an RPC error.
pub fn neural_api_capability_call(
    host: &str,
    port: u16,
    domain: &str,
    operation: &str,
    params: &serde_json::Value,
) -> TcpRpcResult {
    let call_params = serde_json::json!({
        "domain": domain,
        "operation": operation,
        "params": params
    });
    tcp_rpc(
        host,
        port,
        crate::ipc::methods::capability::CALL,
        &call_params,
    )
}

/// Discover primals for a capability domain via the Neural API.
///
/// # Errors
///
/// Returns [`IpcError`] on connection failure or RPC error.
pub fn neural_api_capability_discover(host: &str, port: u16, domain: &str) -> TcpRpcResult {
    let params = serde_json::json!({ "domain": domain });
    tcp_rpc(
        host,
        port,
        crate::ipc::methods::capability::DISCOVER,
        &params,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_port_returns_default_when_unset() {
        let port = env_port("PRIMALSPRING_TCP_TEST_PORT_NONEXISTENT_XYZ", 9999);
        assert_eq!(port, 9999);
    }

    #[test]
    fn tcp_rpc_fails_gracefully_on_unreachable_host() {
        let result = tcp_rpc("127.0.0.1", 1, "health.liveness", &serde_json::json!({}));
        assert!(result.is_err());
    }

    #[test]
    #[expect(deprecated, reason = "testing deprecated fn until removal")]
    fn http_health_fails_gracefully_on_unreachable_host() {
        let result = http_health_probe("127.0.0.1", 1);
        assert!(result.is_err());
    }

    #[test]
    fn neural_api_capability_call_fails_on_unreachable() {
        let result = neural_api_capability_call(
            "127.0.0.1",
            1,
            "security",
            "crypto.sign_ed25519",
            &serde_json::json!({}),
        );
        assert!(result.is_err());
    }

    #[test]
    fn neural_api_capability_discover_fails_on_unreachable() {
        let result = neural_api_capability_discover("127.0.0.1", 1, "security");
        assert!(result.is_err());
    }
}
