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
use crate::btsp::phase3::SessionKeys;
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
///
/// When Phase 3 cipher negotiation succeeds, `session_keys` holds the
/// derived encryption keys and all `call()` traffic is encrypted with
/// length-prefixed ChaCha20-Poly1305 frames.
#[derive(Debug)]
pub struct Transport {
    inner: TransportInner,
    btsp_authenticated: bool,
    session_keys: Option<SessionKeys>,
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
            session_keys: None,
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

        let handshake = super::btsp_handshake::client_handshake(&mut stream, family_seed)?;
        tracing::debug!(
            session_id = %handshake.session_id,
            path = %path.display(),
            "BTSP authenticated"
        );

        let session_keys = super::btsp_handshake::negotiate_phase3(&mut stream, &handshake)
            .unwrap_or_else(|e| {
                tracing::debug!(
                    err = %e,
                    "BTSP Phase 3 negotiate failed — continuing with null cipher"
                );
                None
            });

        if session_keys.is_some() {
            tracing::info!(
                path = %path.display(),
                "BTSP Phase 3: encrypted channel active (ChaCha20-Poly1305)"
            );
        }

        stream
            .set_read_timeout(Some(ipc_timeout))
            .map_err(classify_io_error)?;
        stream
            .set_write_timeout(Some(ipc_timeout))
            .map_err(classify_io_error)?;

        Ok(Self {
            inner: TransportInner::Unix(BufReader::new(stream)),
            btsp_authenticated: true,
            session_keys,
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
            session_keys: None,
        })
    }

    /// Send a JSON-RPC request and read the response.
    ///
    /// When Phase 3 session keys are active, the request is encrypted as a
    /// length-prefixed frame: `[4 bytes: len (BE u32)] [encrypted payload]`.
    /// The response is read and decrypted using the same framing.
    ///
    /// When no session keys are present (NULL cipher), uses plaintext
    /// newline-delimited JSON as before.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on serialization, I/O, encryption, or parse failure.
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

        if self.session_keys.is_some() {
            self.call_encrypted(&line)
        } else {
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
    }

    #[expect(
        clippy::expect_used,
        reason = "session_keys presence verified by caller (is_some check in call())"
    )]
    fn call_encrypted(&mut self, json_line: &str) -> Result<JsonRpcResponse, IpcError> {
        let keys = self.session_keys.as_ref().expect("Phase 3 keys required");
        let encrypted = keys.encrypt(json_line.trim_end().as_bytes())?;
        let len = u32::try_from(encrypted.len()).map_err(|_| IpcError::ProtocolError {
            detail: "BTSP Phase 3: frame too large".to_owned(),
        })?;

        self.write_all(&len.to_be_bytes())?;
        self.write_all(&encrypted)?;
        let resp_frame = self.read_encrypted_frame()?;

        let keys = self.session_keys.as_ref().expect("Phase 3 keys required");
        let decrypted = keys.decrypt(&resp_frame)?;
        let response_str = String::from_utf8(decrypted).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP Phase 3 decrypted response not UTF-8: {e}"),
        })?;

        JsonRpcResponse::from_line(&response_str).map_err(|e| IpcError::ProtocolError {
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

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), IpcError> {
        use std::io::Read;
        match &mut self.inner {
            TransportInner::Unix(reader) => reader.read_exact(buf).map_err(classify_io_error),
            TransportInner::Tcp(reader) => reader.read_exact(buf).map_err(classify_io_error),
        }
    }

    fn read_encrypted_frame(&mut self) -> Result<Vec<u8>, IpcError> {
        let mut len_buf = [0u8; 4];
        self.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > 16 * 1024 * 1024 {
            return Err(IpcError::ProtocolError {
                detail: format!("BTSP Phase 3: frame too large ({len} bytes)"),
            });
        }
        let mut frame = vec![0u8; len];
        self.read_exact(&mut frame)?;
        Ok(frame)
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

    /// Whether this transport has an active Phase 3 encrypted channel.
    #[must_use]
    pub const fn is_encrypted(&self) -> bool {
        self.session_keys.is_some()
    }

    /// Transport type as a display string.
    #[must_use]
    pub const fn transport_type(&self) -> &str {
        match (
            &self.inner,
            self.btsp_authenticated,
            self.session_keys.is_some(),
        ) {
            (TransportInner::Unix(_), true, true) => "unix+btsp+chacha20",
            (TransportInner::Unix(_), true, false) => "unix+btsp",
            (TransportInner::Unix(_), false, _) => "unix",
            (TransportInner::Tcp(_), true, true) => "tcp+btsp+chacha20",
            (TransportInner::Tcp(_), true, false) => "tcp+btsp",
            (TransportInner::Tcp(_), false, _) => "tcp",
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
