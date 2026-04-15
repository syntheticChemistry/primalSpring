// SPDX-License-Identifier: AGPL-3.0-or-later

//! Synchronous JSON-RPC 2.0 client over IPC.
//!
//! Pure Rust, zero async runtime required. Uses [`super::transport::Transport`]
//! for Unix domain socket I/O (and the same transport stack supports TCP when
//! used directly). Line-delimited JSON-RPC 2.0.

use std::path::Path;

use super::error::IpcError;
use super::protocol::JsonRpcResponse;
use super::transport::Transport;

/// A synchronous JSON-RPC 2.0 client connected to a primal socket.
#[derive(Debug)]
pub struct PrimalClient {
    transport: Transport,
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
        Ok(Self {
            transport: Transport::connect(socket)?,
            primal: primal.to_owned(),
        })
    }

    /// Connect to a primal at the given TCP address (e.g. `"127.0.0.1:9100"`).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::ConnectionRefused`] or [`IpcError::Timeout`]
    /// if the address is unreachable.
    pub fn connect_tcp(addr: &str, primal: &str) -> Result<Self, IpcError> {
        Ok(Self {
            transport: Transport::tcp(addr)?,
            primal: primal.to_owned(),
        })
    }

    /// Connect with an explicit BTSP seed (bypasses environment lookup).
    ///
    /// Used by the harness when connecting to primals that enforce BTSP
    /// (e.g. BearDog in Production mode).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection or BTSP handshake failure.
    pub fn connect_btsp(socket: &Path, primal: &str, seed: &[u8]) -> Result<Self, IpcError> {
        Ok(Self {
            transport: Transport::connect_btsp(socket, seed)?,
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
        self.transport.call(method, params)
    }

    /// Send a `health.check` request and return whether the primal is healthy.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails.
    pub fn health_check(&mut self) -> Result<bool, IpcError> {
        let resp = self.call("health.liveness", serde_json::json!({}))?;
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
                                let prefixed = format!("{}.health", self.primal);
                                self.call(&prefixed, serde_json::Value::Null).map(|_| true)
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

    /// Call a method and deserialize the result into a typed value.
    ///
    /// Combines `call()` + `extract_rpc_result()` in one step. Springs use
    /// this when they know the full result schema (e.g. a struct with known
    /// fields).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure, JSON-RPC error, or
    /// deserialization mismatch.
    pub fn call_extract<T: serde::de::DeserializeOwned>(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, IpcError> {
        let response = self.call(method, params)?;
        super::extract::extract_rpc_result(&response)
    }

    /// Call a method and extract a single `f64` from the result by key.
    ///
    /// Handles the common ecosystem pattern where results are returned as
    /// `{"value": 42.0}` or `{"result": 3.14}`. The `result_key` parameter
    /// specifies which field to extract.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure, JSON-RPC error, missing
    /// key, or type mismatch.
    pub fn call_extract_f64(
        &mut self,
        method: &str,
        params: serde_json::Value,
        result_key: &str,
    ) -> Result<f64, IpcError> {
        let response = self.call(method, params)?;
        let result = response.result.ok_or_else(|| {
            IpcError::ProtocolError {
                detail: response
                    .error
                    .as_ref()
                    .map_or_else(|| "no result".to_owned(), |e| e.message.clone()),
            }
        })?;
        result
            .get(result_key)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| IpcError::SerializationError {
                detail: format!("key '{result_key}' not found or not a number in {result}"),
            })
    }

    /// Call a method and extract a `Vec<f64>` from the result by key.
    ///
    /// For tensor/array results like `{"values": [1.0, 2.0, 3.0]}`.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure, JSON-RPC error, missing
    /// key, or type mismatch.
    pub fn call_extract_vec_f64(
        &mut self,
        method: &str,
        params: serde_json::Value,
        result_key: &str,
    ) -> Result<Vec<f64>, IpcError> {
        let response = self.call(method, params)?;
        let result = response.result.ok_or_else(|| {
            IpcError::ProtocolError {
                detail: response
                    .error
                    .as_ref()
                    .map_or_else(|| "no result".to_owned(), |e| e.message.clone()),
            }
        })?;
        let arr = result
            .get(result_key)
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| IpcError::SerializationError {
                detail: format!("key '{result_key}' not found or not an array in {result}"),
            })?;
        arr.iter()
            .map(|v| {
                v.as_f64().ok_or_else(|| IpcError::SerializationError {
                    detail: format!("array element is not a number: {v}"),
                })
            })
            .collect()
    }

    /// Request the primal's capability list.
    ///
    /// Tries `capabilities.list` first, then falls back to `capability.list`
    /// and `primal.capabilities` if the primal returns `METHOD_NOT_FOUND`.
    /// This handles the ecosystem naming drift where different primals
    /// register different method names for the same operation.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if all three method names fail.
    pub fn capabilities(&mut self) -> Result<serde_json::Value, IpcError> {
        const METHODS: &[&str] = &[
            "capabilities.list",
            "capability.list",
            "primal.capabilities",
        ];
        let mut last_err = None;
        for method in METHODS {
            match self.call(method, serde_json::json!({})) {
                Ok(resp) => {
                    if let Some(err) = resp.error {
                        let ipc_err = IpcError::from(err);
                        if ipc_err.is_method_not_found() {
                            last_err = Some(ipc_err);
                            continue;
                        }
                        return Err(ipc_err);
                    }
                    return Ok(resp.result.unwrap_or(serde_json::Value::Null));
                }
                Err(e) if e.is_method_not_found() => {
                    last_err = Some(e);
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err.unwrap_or_else(|| IpcError::MethodNotFound {
            method: "capabilities.list".to_owned(),
        }))
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
    use std::io::{BufRead, BufReader, Write};
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
    fn connect_by_capability_fails_when_no_provider() {
        let result = super::connect_by_capability("nonexistent_capability_xyzzy_12345");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.is_connection_error());
    }

    #[test]
    fn primal_client_primal_name_accessor() {
        let dir = std::env::temp_dir().join("primalspring-accessor-test");
        std::fs::create_dir_all(&dir).unwrap();
        let sock = dir.join("accessor-test.sock");
        let _ = std::fs::remove_file(&sock);
        let listener = UnixListener::bind(&sock).unwrap();

        let client = PrimalClient::connect(&sock, "my-primal").unwrap();
        assert_eq!(client.primal(), "my-primal");

        drop(client);
        drop(listener);
        let _ = std::fs::remove_file(&sock);
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
