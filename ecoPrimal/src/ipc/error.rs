// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Typed IPC error with semantic classification and query methods.
//!
//! Converged ecosystem pattern: every spring and primal uses a typed
//! `IpcError` with `is_retriable()`, `is_connection_error()`, etc.
//! rather than raw `std::io::Error`. This enables `CircuitBreaker`
//! and `RetryPolicy` to make informed retry decisions.

use super::protocol::{JsonRpcError, error_codes};

/// Phase in which an IPC operation failed.
///
/// Absorbed from biomeOS v2.51 and loamSpine v0.9.5. Provides
/// diagnostic context without leaking implementation details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum IpcErrorPhase {
    /// Failure during socket connection (before any bytes sent).
    #[error("connect")]
    Connect,
    /// Failure while serializing the request payload.
    #[error("serialize")]
    Serialize,
    /// Failure while sending bytes over the socket.
    #[error("send")]
    Send,
    /// Failure while waiting for or reading the response.
    #[error("receive")]
    Receive,
    /// Failure while parsing the response payload.
    #[error("parse")]
    Parse,
}

/// Semantic IPC error — classifies failures by what happened rather
/// than where in the code path the failure occurred.
#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    /// No socket file found for the requested primal.
    #[error("no socket found for primal '{primal}'")]
    SocketNotFound {
        /// Primal name that was searched for.
        primal: String,
    },
    /// Socket exists but the connection was actively refused.
    #[error("connection refused: {0}")]
    ConnectionRefused(#[source] std::io::Error),
    /// Connection was established but dropped mid-communication.
    #[error("connection reset: {0}")]
    ConnectionReset(#[source] std::io::Error),
    /// Operation exceeded the configured timeout.
    #[error("timeout: {0}")]
    Timeout(#[source] std::io::Error),
    /// Wire-level protocol violation (malformed JSON, empty response, etc.).
    #[error("protocol error: {detail}")]
    ProtocolError {
        /// Human-readable description of the protocol violation.
        detail: String,
    },
    /// Server explicitly reported the method does not exist.
    #[error("method not found: {method}")]
    MethodNotFound {
        /// The method name or server message.
        method: String,
    },
    /// Server returned a JSON-RPC error that is not `MethodNotFound`.
    #[error("application error {code}: {message}")]
    ApplicationError {
        /// JSON-RPC error code.
        code: i64,
        /// Human-readable error message from the server.
        message: String,
        /// Optional structured error data.
        data: Option<serde_json::Value>,
    },
    /// Failed to serialize a request or deserialize a typed result.
    #[error("serialization error: {detail}")]
    SerializationError {
        /// Human-readable description of the serialization failure.
        detail: String,
    },
    /// Caller lacks permission to invoke the requested method.
    ///
    /// Returned when the `MethodGate` rejects a call because the caller's
    /// capability token does not authorize the method, or no token was
    /// presented for a protected method.
    #[error("permission denied for '{method}': {reason}")]
    PermissionDenied {
        /// The method the caller attempted to invoke.
        method: String,
        /// Why access was denied.
        reason: String,
    },
}

impl IpcError {
    /// Short discriminant label for structured error contexts.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::SocketNotFound { .. } => "socket_not_found",
            Self::ConnectionRefused(_) => "connection_refused",
            Self::ConnectionReset(_) => "connection_reset",
            Self::Timeout(_) => "timeout",
            Self::ProtocolError { .. } => "protocol_error",
            Self::MethodNotFound { .. } => "method_not_found",
            Self::ApplicationError { .. } => "application_error",
            Self::SerializationError { .. } => "serialization_error",
            Self::PermissionDenied { .. } => "permission_denied",
        }
    }
}

/// An [`IpcError`] annotated with the [`IpcErrorPhase`] where it occurred.
///
/// `source()` delegates to the inner `IpcError`'s error source, preserving
/// the `io::Error` chain for connection types while returning `None` for
/// protocol/application errors.
#[derive(Debug, thiserror::Error)]
#[error("[{phase}] {error}")]
pub struct PhasedIpcError {
    /// The phase of the IPC operation that failed.
    pub phase: IpcErrorPhase,
    /// The underlying error.
    #[source]
    pub error: IpcError,
}

impl IpcError {
    /// Wrap this error with phase context for diagnostics.
    #[must_use]
    pub const fn in_phase(self, phase: IpcErrorPhase) -> PhasedIpcError {
        PhasedIpcError { phase, error: self }
    }

    /// Whether a retry is likely to succeed (transient failures).
    #[must_use]
    pub const fn is_retriable(&self) -> bool {
        matches!(self, Self::ConnectionReset(_) | Self::Timeout(_))
    }

    /// Whether recovery is possible without operator intervention.
    ///
    /// Absorbed from neuralSpring V122 / wetSpring V133 / groundSpring V121.
    /// Broader than `is_retriable()` — includes transient failures (resets,
    /// timeouts) AND server-reported errors that may resolve if the primal
    /// is restarted or stabilizes. Excludes `MethodNotFound` (permanent) and
    /// `SerializationError` (client bug).
    #[must_use]
    pub const fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::ConnectionRefused(_)
                | Self::ConnectionReset(_)
                | Self::Timeout(_)
                | Self::ApplicationError { .. }
        )
    }

    /// Whether this error is likely caused by a timeout.
    #[must_use]
    pub const fn is_timeout_likely(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }

    /// Returns true if this error indicates the remote method was not found
    /// (JSON-RPC error code -32601).
    ///
    /// Checks the dedicated [`Self::MethodNotFound`] variant and structured
    /// JSON-RPC error codes first, then falls back to string matching for
    /// errors that lost structure through serialization.
    #[must_use]
    pub fn is_method_not_found(&self) -> bool {
        match self {
            Self::MethodNotFound { .. } => true,
            Self::ApplicationError { code, message, .. } => {
                *code == error_codes::METHOD_NOT_FOUND
                    || message.contains("-32601")
                    || message.contains("method not found")
                    || message.contains("Method not found")
            }
            Self::ProtocolError { detail } | Self::SerializationError { detail } => {
                detail.contains("-32601")
                    || detail.contains("method not found")
                    || detail.contains("Method not found")
            }
            _ => false,
        }
    }

    /// Whether this is a connection-level failure (socket, refused, reset).
    #[must_use]
    pub const fn is_connection_error(&self) -> bool {
        matches!(
            self,
            Self::SocketNotFound { .. } | Self::ConnectionRefused(_) | Self::ConnectionReset(_)
        )
    }

    /// Whether the failure is a wire-level protocol violation (e.g. HTTP
    /// response where raw JSON-RPC was expected).
    #[must_use]
    pub const fn is_protocol_error(&self) -> bool {
        matches!(self, Self::ProtocolError { .. })
    }

    /// Whether the server rejected the call due to insufficient permissions.
    #[must_use]
    pub const fn is_permission_denied(&self) -> bool {
        matches!(self, Self::PermissionDenied { .. })
    }

    /// Whether this error should be treated as a graceful skip in composition
    /// tests and validation scenarios. True for: unreachable primal, protocol
    /// mismatch, transport mismatch, BTSP permission gate, and method not
    /// found (primal may be an older version).
    #[must_use]
    pub fn is_skippable(&self) -> bool {
        self.is_connection_error()
            || self.is_protocol_error()
            || self.is_transport_mismatch()
            || self.is_permission_denied()
            || self.is_method_not_found()
    }

    /// Whether the failure is likely a transport mismatch (e.g. tarpc socket
    /// receiving raw JSON-RPC). Manifests as a timeout with EAGAIN because
    /// the server accepts the connection but never sends a JSON-RPC response.
    #[must_use]
    pub fn is_transport_mismatch(&self) -> bool {
        match self {
            Self::Timeout(e) => {
                let msg = e.to_string();
                msg.contains("temporarily") || msg.contains("Resource temporarily")
            }
            _ => false,
        }
    }
}

impl From<JsonRpcError> for IpcError {
    fn from(err: JsonRpcError) -> Self {
        match err.code {
            error_codes::METHOD_NOT_FOUND => Self::MethodNotFound {
                method: err.message,
            },
            error_codes::PERMISSION_DENIED | error_codes::UNAUTHORIZED => Self::PermissionDenied {
                method: err
                    .data
                    .as_ref()
                    .and_then(|d| d.get("method"))
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("unknown")
                    .to_owned(),
                reason: err.message,
            },
            _ => Self::ApplicationError {
                code: err.code,
                message: err.message,
                data: err.data,
            },
        }
    }
}

/// Classify a raw `io::Error` into a semantic `IpcError` variant.
#[must_use]
pub fn classify_io_error(err: std::io::Error) -> IpcError {
    match err.kind() {
        std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::NotFound => {
            IpcError::ConnectionRefused(err)
        }
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => IpcError::Timeout(err),
        _ => IpcError::ConnectionReset(err),
    }
}

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;
