// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cross-platform transport layer for IPC connections.
//!
//! Absorbed from airSpring V010 and healthSpring V42. Encapsulates Unix
//! domain socket and TCP socket transports behind a unified enum so
//! callers can connect without caring about the underlying mechanism.
//!
//! On platforms where Unix sockets are available (Linux, macOS), they are
//! preferred for zero-copy, low-latency local IPC. On Windows or remote
//! scenarios, TCP over loopback provides a fallback.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use super::error::{IpcError, classify_io_error};
use super::protocol::{JsonRpcRequest, JsonRpcResponse};
use crate::tolerances;

/// Wire-level transport kind.
#[derive(Debug)]
enum TransportInner {
    Unix(BufReader<UnixStream>),
    Tcp(BufReader<TcpStream>),
}

/// Connection transport — Unix domain socket or TCP, with BTSP state.
///
/// Absorbed from airSpring V010 `Transport` enum, healthSpring V42, and
/// groundSpring V121. Provides a unified read/write interface regardless
/// of the underlying socket type. Tracks whether a BTSP handshake was
/// completed so guidestone can report per-atomic security posture.
#[derive(Debug)]
pub struct Transport {
    inner: TransportInner,
    btsp_authenticated: bool,
}

impl Transport {
    /// Connect to a primal at the given Unix domain socket path (cleartext).
    ///
    /// This is a plain JSON-RPC connection with no BTSP handshake. For
    /// BTSP-authenticated connections, use [`connect_btsp`](Self::connect_btsp)
    /// or let [`upgrade_btsp_clients`](crate::composition) handle escalation.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure.
    pub fn connect(path: &Path) -> Result<Self, IpcError> {
        Self::unix(path)
    }

    /// Connect with an explicit BTSP seed, bypassing environment lookup.
    ///
    /// Used by the [`AtomicHarness`](crate::harness::AtomicHarness) where
    /// the seed is generated in-process and not available via `FAMILY_SEED`.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure or BTSP handshake failure.
    pub fn connect_btsp(path: &Path, seed: &[u8]) -> Result<Self, IpcError> {
        Self::unix_btsp(path, seed)
    }

    /// Connect via Unix domain socket (cleartext, no handshake).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure.
    pub fn unix(path: &Path) -> Result<Self, IpcError> {
        let timeout = Duration::from_secs(tolerances::IPC_SOCKET_TIMEOUT_SECS);
        let stream = UnixStream::connect(path).map_err(classify_io_error)?;
        stream
            .set_read_timeout(Some(timeout))
            .map_err(classify_io_error)?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(classify_io_error)?;
        Ok(Self {
            inner: TransportInner::Unix(BufReader::new(stream)),
            btsp_authenticated: false,
        })
    }

    /// Connect via Unix socket with BTSP handshake authentication.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection or handshake failure.
    pub fn unix_btsp(path: &Path, family_seed: &[u8]) -> Result<Self, IpcError> {
        let handshake_timeout = Duration::from_secs(tolerances::BTSP_HANDSHAKE_TIMEOUT_SECS);
        let ipc_timeout = Duration::from_secs(tolerances::IPC_SOCKET_TIMEOUT_SECS);
        let mut stream = UnixStream::connect(path).map_err(classify_io_error)?;
        stream
            .set_read_timeout(Some(handshake_timeout))
            .map_err(classify_io_error)?;
        stream
            .set_write_timeout(Some(handshake_timeout))
            .map_err(classify_io_error)?;

        let session_id = super::btsp_handshake::client_handshake(&mut stream, family_seed)?;
        tracing::debug!(session_id = %session_id, path = %path.display(), "BTSP authenticated");

        stream
            .set_read_timeout(Some(ipc_timeout))
            .map_err(classify_io_error)?;
        stream
            .set_write_timeout(Some(ipc_timeout))
            .map_err(classify_io_error)?;

        Ok(Self {
            inner: TransportInner::Unix(BufReader::new(stream)),
            btsp_authenticated: true,
        })
    }

    /// Connect via TCP to the given address (e.g. `"127.0.0.1:9100"`).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure.
    pub fn tcp(addr: &str) -> Result<Self, IpcError> {
        let timeout = Duration::from_secs(tolerances::IPC_SOCKET_TIMEOUT_SECS);
        let stream = TcpStream::connect(addr).map_err(classify_io_error)?;
        stream
            .set_read_timeout(Some(timeout))
            .map_err(classify_io_error)?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(classify_io_error)?;
        Ok(Self {
            inner: TransportInner::Tcp(BufReader::new(stream)),
            btsp_authenticated: false,
        })
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

        self.write_all(line.as_bytes())?;
        let response_line = self.read_line()?;

        if response_line.is_empty() {
            return Err(IpcError::ProtocolError {
                detail: "empty response".to_owned(),
            });
        }

        JsonRpcResponse::from_line(&response_line).map_err(|e| IpcError::ProtocolError {
            detail: e.to_string(),
        })
    }

    fn write_all(&mut self, data: &[u8]) -> Result<(), IpcError> {
        match &mut self.inner {
            TransportInner::Unix(reader) => {
                reader.get_mut().write_all(data).map_err(classify_io_error)
            }
            TransportInner::Tcp(reader) => {
                reader.get_mut().write_all(data).map_err(classify_io_error)
            }
        }
    }

    fn read_line(&mut self) -> Result<String, IpcError> {
        let mut line = String::new();
        match &mut self.inner {
            TransportInner::Unix(reader) => {
                reader.read_line(&mut line).map_err(classify_io_error)?;
            }
            TransportInner::Tcp(reader) => {
                reader.read_line(&mut line).map_err(classify_io_error)?;
            }
        }
        Ok(line)
    }

    /// Whether this transport is a Unix domain socket.
    #[must_use]
    pub const fn is_unix(&self) -> bool {
        matches!(self.inner, TransportInner::Unix(_))
    }

    /// Whether this transport is a TCP socket.
    #[must_use]
    pub const fn is_tcp(&self) -> bool {
        matches!(self.inner, TransportInner::Tcp(_))
    }

    /// Whether this transport completed a BTSP handshake.
    #[must_use]
    pub const fn is_btsp_authenticated(&self) -> bool {
        self.btsp_authenticated
    }

    /// Transport type as a display string.
    #[must_use]
    pub const fn transport_type(&self) -> &str {
        match (&self.inner, self.btsp_authenticated) {
            (TransportInner::Unix(_), true) => "unix+btsp",
            (TransportInner::Unix(_), false) => "unix",
            (TransportInner::Tcp(_), true) => "tcp+btsp",
            (TransportInner::Tcp(_), false) => "tcp",
        }
    }
}

/// Parse a transport address string and connect.
///
/// Supports:
/// - `unix:/path/to/socket.sock` — Unix domain socket
/// - `tcp:127.0.0.1:9100` — TCP socket
/// - `/path/to/socket.sock` — implicit Unix socket (path starts with `/`)
/// - `127.0.0.1:9100` — implicit TCP (anything else)
///
/// # Errors
///
/// Returns [`IpcError`] if the address cannot be parsed or connection fails.
#[expect(
    clippy::option_if_let_else,
    reason = "nested strip_prefix chain is clearer as if-let"
)]
pub fn connect_transport(address: &str) -> Result<Transport, IpcError> {
    if let Some(path) = address.strip_prefix("unix:") {
        Transport::unix(Path::new(path))
    } else if let Some(addr) = address.strip_prefix("tcp:") {
        Transport::tcp(addr)
    } else if address.starts_with('/') {
        Transport::unix(Path::new(address))
    } else {
        Transport::tcp(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;

    #[test]
    fn unix_transport_connects() {
        let dir = std::env::temp_dir().join("primalspring-transport-test");
        std::fs::create_dir_all(&dir).unwrap();
        let sock = dir.join("transport-unix.sock");
        let _ = std::fs::remove_file(&sock);

        let listener = UnixListener::bind(&sock).unwrap();
        let sock_clone = sock.clone();

        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = std::io::BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();

            let req: serde_json::Value = serde_json::from_str(&line).unwrap();
            let id = req["id"].as_u64().unwrap();
            let resp = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"ok\":true}},\"id\":{id}}}\n");
            (&stream).write_all(resp.as_bytes()).unwrap();
        });

        let mut transport = Transport::unix(&sock_clone).unwrap();
        assert!(transport.is_unix());
        assert!(!transport.is_tcp());
        assert!(!transport.is_btsp_authenticated());
        assert_eq!(transport.transport_type(), "unix");

        let resp = transport
            .call("health.check", serde_json::Value::Null)
            .unwrap();
        assert!(resp.is_success());

        server.join().unwrap();
        let _ = std::fs::remove_file(&sock);
    }

    #[test]
    fn connect_transport_unix_explicit() {
        let result = connect_transport("unix:/nonexistent/socket.sock");
        assert!(result.is_err());
    }

    #[test]
    fn connect_transport_unix_implicit() {
        let result = connect_transport("/nonexistent/socket.sock");
        assert!(result.is_err());
    }

    #[test]
    fn connect_transport_tcp_explicit() {
        let result = connect_transport("tcp:127.0.0.1:1");
        assert!(result.is_err());
    }

    #[test]
    fn connect_transport_tcp_implicit() {
        let result = connect_transport("127.0.0.1:1");
        assert!(result.is_err());
    }
}
