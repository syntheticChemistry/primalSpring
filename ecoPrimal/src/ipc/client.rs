// SPDX-License-Identifier: AGPL-3.0-or-later

//! Synchronous JSON-RPC 2.0 client over Unix sockets.
//!
//! Pure Rust, zero async runtime required. Uses `std::os::unix::net`
//! for Unix domain socket I/O with line-delimited JSON-RPC 2.0.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use super::error::{IpcError, classify_io_error};
use super::protocol::{JsonRpcRequest, JsonRpcResponse};

/// Default timeout for socket operations (5 seconds).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// A synchronous JSON-RPC 2.0 client connected to a primal socket.
#[derive(Debug)]
pub struct PrimalClient {
    stream: BufReader<UnixStream>,
    primal: String,
}

impl PrimalClient {
    /// Connect to a primal at the given socket path.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::ConnectionRefused`] or [`IpcError::Timeout`]
    /// if the socket is unreachable.
    pub fn connect(socket: &Path, primal: &str) -> Result<Self, IpcError> {
        let stream = UnixStream::connect(socket).map_err(classify_io_error)?;
        stream
            .set_read_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(classify_io_error)?;
        stream
            .set_write_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(classify_io_error)?;
        Ok(Self {
            stream: BufReader::new(stream),
            primal: primal.to_owned(),
        })
    }

    /// The primal this client is connected to.
    #[must_use]
    pub fn primal(&self) -> &str {
        &self.primal
    }

    /// Send a JSON-RPC request and read the response.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on serialization, I/O, or parse failure.
    pub fn call(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<JsonRpcResponse, IpcError> {
        let request = JsonRpcRequest::new(method, params);
        let line = request
            .to_line()
            .map_err(|e| IpcError::SerializationError {
                detail: e.to_string(),
            })?;

        self.stream
            .get_mut()
            .write_all(line.as_bytes())
            .map_err(classify_io_error)?;

        let mut response_line = String::new();
        self.stream
            .read_line(&mut response_line)
            .map_err(classify_io_error)?;

        if response_line.is_empty() {
            return Err(IpcError::ProtocolError {
                detail: "empty response".to_owned(),
            });
        }

        JsonRpcResponse::from_line(&response_line).map_err(|e| IpcError::ProtocolError {
            detail: e.to_string(),
        })
    }

    /// Send a `health.check` request and return whether the primal is healthy.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails.
    pub fn health_check(&mut self) -> Result<bool, IpcError> {
        let resp = self.call("health.check", serde_json::Value::Null)?;
        Ok(resp.is_success())
    }

    /// Send a `health.liveness` probe — is the primal process alive?
    ///
    /// Kubernetes-style liveness probe. Returns `true` if the primal
    /// responds (even with an error — it is at least alive).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] only on transport-level failures (socket
    /// not found, connection refused, timeout).
    pub fn health_liveness(&mut self) -> Result<bool, IpcError> {
        match self.call("health.liveness", serde_json::Value::Null) {
            Ok(_) => Ok(true),
            Err(e) if e.is_method_not_found() => {
                // Fallback chain: health.check → health → {primal}.health
                match self.health_check() {
                    Ok(v) => Ok(v),
                    Err(e2) if e2.is_method_not_found() => {
                        match self.call("health", serde_json::Value::Null) {
                            Ok(_) => Ok(true),
                            Err(e3) if e3.is_method_not_found() => {
                                let prefixed =
                                    format!("{}.health", self.primal);
                                self.call(&prefixed, serde_json::Value::Null)
                                    .map(|_| true)
                            }
                            Err(e3) => Err(e3),
                        }
                    }
                    Err(e2) => Err(e2),
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Send a `health.readiness` probe — is the primal ready for work?
    ///
    /// Kubernetes-style readiness probe. A primal may be alive but not
    /// yet ready (still loading models, waiting for peers, etc.).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails.
    pub fn health_readiness(&mut self) -> Result<bool, IpcError> {
        match self.call("health.readiness", serde_json::Value::Null) {
            Ok(resp) => Ok(resp.is_success()),
            Err(e) if e.is_method_not_found() => {
                // Fall back to health.check if readiness is not implemented
                self.health_check()
            }
            Err(e) => Err(e),
        }
    }

    /// Request the primal's capability list.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails.
    pub fn capabilities(&mut self) -> Result<serde_json::Value, IpcError> {
        let resp = self.call("capabilities.list", serde_json::Value::Null)?;
        if let Some(err) = resp.error {
            return Err(IpcError::from(err));
        }
        Ok(resp.result.unwrap_or(serde_json::Value::Null))
    }
}

/// Attempt to connect to a primal by discovering its socket at runtime.
///
/// # Errors
///
/// Returns [`IpcError::SocketNotFound`] if discovery finds no socket, or
/// a connection-level error if the socket exists but cannot be reached.
pub fn connect_primal(primal: &str) -> Result<PrimalClient, IpcError> {
    let result = super::discover::discover_primal(primal);
    result.socket.map_or_else(
        || {
            Err(IpcError::SocketNotFound {
                primal: primal.to_owned(),
            })
        },
        |path| PrimalClient::connect(&path, primal),
    )
}

/// Connect to whatever primal provides a capability domain.
///
/// **Loose coupling**: the caller doesn't know or care which primal
/// implements the capability. The Neural API (or filesystem probing)
/// resolves the provider at runtime.
///
/// # Errors
///
/// Returns [`IpcError::SocketNotFound`] if no provider is discovered, or
/// a connection-level error if the socket exists but cannot be reached.
pub fn connect_by_capability(capability: &str) -> Result<PrimalClient, IpcError> {
    let result = super::discover::discover_by_capability(capability);
    let label = result.resolved_primal.as_deref().unwrap_or(capability);
    result.socket.map_or_else(
        || {
            Err(IpcError::SocketNotFound {
                primal: format!("capability:{capability}"),
            })
        },
        |path| PrimalClient::connect(&path, label),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;

    #[test]
    fn connect_fails_for_nonexistent_socket() {
        let result = PrimalClient::connect(Path::new("/nonexistent/socket.sock"), "test");
        assert!(result.is_err());
    }

    #[test]
    fn connect_primal_fails_when_no_socket_discovered() {
        let result = connect_primal("definitely_not_a_real_primal");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.is_connection_error());
    }

    #[test]
    fn round_trip_with_mock_server() {
        let dir = std::env::temp_dir().join("primalspring-test");
        std::fs::create_dir_all(&dir).unwrap();
        let sock_path = dir.join("test-roundtrip.sock");
        let _ = std::fs::remove_file(&sock_path);

        let listener = UnixListener::bind(&sock_path).unwrap();

        let sock_path_clone = sock_path.clone();
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();

            let req: serde_json::Value = serde_json::from_str(&line).unwrap();
            let id = req["id"].as_u64().unwrap();

            let response =
                format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"status\":\"ok\"}},\"id\":{id}}}\n");
            (&stream).write_all(response.as_bytes()).unwrap();
        });

        let mut client = PrimalClient::connect(&sock_path_clone, "test").unwrap();
        let resp = client
            .call("health.check", serde_json::Value::Null)
            .unwrap();
        assert!(resp.is_success());

        server.join().unwrap();
        let _ = std::fs::remove_file(&sock_path);
    }
}
