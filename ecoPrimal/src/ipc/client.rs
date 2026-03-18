// SPDX-License-Identifier: AGPL-3.0-or-later

//! Synchronous JSON-RPC 2.0 client over Unix sockets.
//!
//! Pure Rust, zero async runtime required. Uses `std::os::unix::net`
//! for Unix domain socket I/O with line-delimited JSON-RPC 2.0.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use super::protocol::{JsonRpcRequest, JsonRpcResponse};

/// Default timeout for socket operations (5 seconds).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Errors from IPC client operations.
#[derive(Debug)]
pub enum IpcError {
    /// Socket connection failed.
    Connect(std::io::Error),
    /// Failed to write request to socket.
    Write(std::io::Error),
    /// Failed to read response from socket.
    Read(std::io::Error),
    /// Response JSON was malformed.
    Parse(serde_json::Error),
    /// Request JSON serialization failed.
    Serialize(serde_json::Error),
    /// Server returned a JSON-RPC error.
    Rpc(super::protocol::JsonRpcError),
    /// No response line received (empty read).
    EmptyResponse,
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(e) => write!(f, "IPC connect failed: {e}"),
            Self::Write(e) => write!(f, "IPC write failed: {e}"),
            Self::Read(e) => write!(f, "IPC read failed: {e}"),
            Self::Parse(e) => write!(f, "IPC response parse failed: {e}"),
            Self::Serialize(e) => write!(f, "IPC request serialize failed: {e}"),
            Self::Rpc(e) => write!(f, "IPC RPC error: {e}"),
            Self::EmptyResponse => write!(f, "IPC empty response"),
        }
    }
}

impl std::error::Error for IpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Connect(e) | Self::Write(e) | Self::Read(e) => Some(e),
            Self::Parse(e) | Self::Serialize(e) => Some(e),
            Self::Rpc(e) => Some(e),
            Self::EmptyResponse => None,
        }
    }
}

/// A synchronous JSON-RPC 2.0 client connected to a primal socket.
pub struct PrimalClient {
    stream: BufReader<UnixStream>,
    primal: String,
}

impl PrimalClient {
    /// Connect to a primal at the given socket path.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::Connect`] if the socket is unreachable.
    pub fn connect(socket: &Path, primal: &str) -> Result<Self, IpcError> {
        let stream = UnixStream::connect(socket).map_err(IpcError::Connect)?;
        stream
            .set_read_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(IpcError::Connect)?;
        stream
            .set_write_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(IpcError::Connect)?;
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
        let line = request.to_line().map_err(IpcError::Serialize)?;

        self.stream
            .get_mut()
            .write_all(line.as_bytes())
            .map_err(IpcError::Write)?;

        let mut response_line = String::new();
        self.stream
            .read_line(&mut response_line)
            .map_err(IpcError::Read)?;

        if response_line.is_empty() {
            return Err(IpcError::EmptyResponse);
        }

        JsonRpcResponse::from_line(&response_line).map_err(IpcError::Parse)
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

    /// Request the primal's capability list.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails.
    pub fn capabilities(&mut self) -> Result<serde_json::Value, IpcError> {
        let resp = self.call("capabilities.list", serde_json::Value::Null)?;
        if let Some(err) = resp.error {
            return Err(IpcError::Rpc(err));
        }
        Ok(resp.result.unwrap_or(serde_json::Value::Null))
    }
}

/// Attempt to connect to a primal by discovering its socket at runtime.
///
/// # Errors
///
/// Returns [`IpcError::Connect`] if discovery finds no socket or
/// connection fails.
pub fn connect_primal(primal: &str) -> Result<PrimalClient, IpcError> {
    let result = super::discover::discover_primal(primal);
    result.socket.map_or_else(
        || {
            Err(IpcError::Connect(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("no socket found for primal '{primal}'"),
            )))
        },
        |path| PrimalClient::connect(&path, primal),
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
