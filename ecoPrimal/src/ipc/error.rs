// SPDX-License-Identifier: AGPL-3.0-or-later

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
}

/// An [`IpcError`] annotated with the [`IpcErrorPhase`] where it occurred.
///
/// `source()` delegates to the inner `IpcError`'s error source, preserving
/// the io::Error chain for connection types while returning `None` for
/// protocol/application errors.
#[derive(Debug)]
pub struct PhasedIpcError {
    /// The phase of the IPC operation that failed.
    pub phase: IpcErrorPhase,
    /// The underlying error.
    pub error: IpcError,
}

impl std::fmt::Display for PhasedIpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.phase, self.error)
    }
}

impl std::error::Error for PhasedIpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
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

    /// Whether the server reported the method does not exist.
    #[must_use]
    pub const fn is_method_not_found(&self) -> bool {
        matches!(self, Self::MethodNotFound { .. })
    }

    /// Whether this is a connection-level failure (socket, refused, reset).
    #[must_use]
    pub const fn is_connection_error(&self) -> bool {
        matches!(
            self,
            Self::SocketNotFound { .. } | Self::ConnectionRefused(_) | Self::ConnectionReset(_)
        )
    }
}

impl From<JsonRpcError> for IpcError {
    fn from(err: JsonRpcError) -> Self {
        if err.code == error_codes::METHOD_NOT_FOUND {
            Self::MethodNotFound {
                method: err.message,
            }
        } else {
            Self::ApplicationError {
                code: err.code,
                message: err.message,
                data: err.data,
            }
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
mod tests {
    use super::*;

    #[test]
    fn socket_not_found_is_connection_error() {
        let err = IpcError::SocketNotFound {
            primal: "beardog".to_owned(),
        };
        assert!(err.is_connection_error());
        assert!(!err.is_retriable());
        assert!(!err.is_timeout_likely());
        assert!(!err.is_method_not_found());
    }

    #[test]
    fn connection_refused_is_connection_error_not_retriable() {
        let err = IpcError::ConnectionRefused(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "refused",
        ));
        assert!(err.is_connection_error());
        assert!(!err.is_retriable());
    }

    #[test]
    fn connection_reset_is_retriable_and_connection_error() {
        let err = IpcError::ConnectionReset(std::io::Error::new(
            std::io::ErrorKind::ConnectionReset,
            "reset",
        ));
        assert!(err.is_retriable());
        assert!(err.is_connection_error());
    }

    #[test]
    fn timeout_is_retriable_and_timeout_likely() {
        let err = IpcError::Timeout(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "timed out",
        ));
        assert!(err.is_retriable());
        assert!(err.is_timeout_likely());
        assert!(!err.is_connection_error());
    }

    #[test]
    fn method_not_found_queries() {
        let err = IpcError::MethodNotFound {
            method: "foo.bar".to_owned(),
        };
        assert!(err.is_method_not_found());
        assert!(!err.is_retriable());
        assert!(!err.is_connection_error());
    }

    #[test]
    fn application_error_is_not_retriable() {
        let err = IpcError::ApplicationError {
            code: -32_603,
            message: "internal".to_owned(),
            data: None,
        };
        assert!(!err.is_retriable());
        assert!(!err.is_method_not_found());
    }

    #[test]
    fn from_jsonrpc_method_not_found() {
        let rpc_err = JsonRpcError {
            code: error_codes::METHOD_NOT_FOUND,
            message: "compute.submit".to_owned(),
            data: None,
        };
        let err = IpcError::from(rpc_err);
        assert!(err.is_method_not_found());
    }

    #[test]
    fn from_jsonrpc_application_error() {
        let rpc_err = JsonRpcError {
            code: error_codes::INTERNAL_ERROR,
            message: "boom".to_owned(),
            data: None,
        };
        let err = IpcError::from(rpc_err);
        assert!(!err.is_method_not_found());
        assert!(matches!(err, IpcError::ApplicationError { .. }));
    }

    #[test]
    fn classify_io_connection_refused() {
        let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused");
        let err = classify_io_error(io_err);
        assert!(matches!(err, IpcError::ConnectionRefused(_)));
    }

    #[test]
    fn classify_io_timed_out() {
        let io_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "slow");
        let err = classify_io_error(io_err);
        assert!(matches!(err, IpcError::Timeout(_)));
    }

    #[test]
    fn classify_io_broken_pipe() {
        let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe");
        let err = classify_io_error(io_err);
        assert!(matches!(err, IpcError::ConnectionReset(_)));
    }

    #[test]
    fn display_socket_not_found() {
        let err = IpcError::SocketNotFound {
            primal: "beardog".to_owned(),
        };
        assert!(err.to_string().contains("beardog"));
    }

    #[test]
    fn display_method_not_found() {
        let err = IpcError::MethodNotFound {
            method: "foo.bar".to_owned(),
        };
        let s = err.to_string();
        assert!(s.contains("method not found"));
        assert!(s.contains("foo.bar"));
    }

    #[test]
    fn is_retriable_true_for_reset() {
        let err = IpcError::ConnectionReset(std::io::Error::new(
            std::io::ErrorKind::ConnectionReset,
            "reset",
        ));
        assert!(err.is_retriable());
        assert!(!err.is_timeout_likely());
        assert!(err.is_connection_error());
    }

    #[test]
    fn is_retriable_true_for_timeout() {
        let err = IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"));
        assert!(err.is_retriable());
        assert!(err.is_timeout_likely());
        assert!(!err.is_connection_error());
    }

    #[test]
    fn is_retriable_false_for_socket_not_found() {
        let err = IpcError::SocketNotFound {
            primal: "beardog".to_owned(),
        };
        assert!(!err.is_retriable());
        assert!(err.is_connection_error());
    }

    #[test]
    fn is_retriable_false_for_application_error() {
        let err = IpcError::ApplicationError {
            code: -32603,
            message: "internal".to_owned(),
            data: None,
        };
        assert!(!err.is_retriable());
        assert!(!err.is_connection_error());
        assert!(!err.is_method_not_found());
    }

    #[test]
    fn error_source_returns_io_error_for_connection_types() {
        use std::error::Error;
        let err = IpcError::ConnectionRefused(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "refused",
        ));
        assert!(err.source().is_some());

        let err2 = IpcError::ProtocolError {
            detail: "bad".to_owned(),
        };
        assert!(err2.source().is_none());
    }

    #[test]
    fn from_jsonrpc_error_method_not_found() {
        let rpc_err = JsonRpcError {
            code: error_codes::METHOD_NOT_FOUND,
            message: "health.check".to_owned(),
            data: None,
        };
        let ipc_err: IpcError = rpc_err.into();
        assert!(ipc_err.is_method_not_found());
    }

    #[test]
    fn from_jsonrpc_error_application_error() {
        let rpc_err = JsonRpcError {
            code: error_codes::INTERNAL_ERROR,
            message: "boom".to_owned(),
            data: Some(serde_json::json!({"detail": "stack trace"})),
        };
        let ipc_err: IpcError = rpc_err.into();
        assert!(!ipc_err.is_method_not_found());
        assert!(matches!(ipc_err, IpcError::ApplicationError { .. }));
    }

    #[test]
    fn display_all_variants() {
        let variants: Vec<IpcError> = vec![
            IpcError::SocketNotFound {
                primal: "x".to_owned(),
            },
            IpcError::ConnectionRefused(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "r",
            )),
            IpcError::ConnectionReset(std::io::Error::new(
                std::io::ErrorKind::ConnectionReset,
                "r",
            )),
            IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "t")),
            IpcError::ProtocolError {
                detail: "bad".to_owned(),
            },
            IpcError::MethodNotFound {
                method: "m".to_owned(),
            },
            IpcError::ApplicationError {
                code: -1,
                message: "e".to_owned(),
                data: None,
            },
            IpcError::SerializationError {
                detail: "s".to_owned(),
            },
        ];
        for v in &variants {
            assert!(!v.to_string().is_empty());
        }
    }

    #[test]
    fn ipc_error_phase_display() {
        assert_eq!(IpcErrorPhase::Connect.to_string(), "connect");
        assert_eq!(IpcErrorPhase::Serialize.to_string(), "serialize");
        assert_eq!(IpcErrorPhase::Send.to_string(), "send");
        assert_eq!(IpcErrorPhase::Receive.to_string(), "receive");
        assert_eq!(IpcErrorPhase::Parse.to_string(), "parse");
    }

    #[test]
    fn phased_error_display_includes_phase() {
        let err = IpcError::ProtocolError {
            detail: "bad json".to_owned(),
        };
        let phased = err.in_phase(IpcErrorPhase::Parse);
        let display = phased.to_string();
        assert!(display.starts_with("[parse]"));
        assert!(display.contains("bad json"));
    }

    #[test]
    fn phased_error_preserves_source() {
        use std::error::Error;
        let err = IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"));
        let phased = err.in_phase(IpcErrorPhase::Receive);
        assert!(phased.source().is_some());
        assert_eq!(phased.phase, IpcErrorPhase::Receive);
    }

    #[test]
    fn phased_error_no_source_for_protocol_error() {
        use std::error::Error;
        let err = IpcError::ProtocolError {
            detail: "x".to_owned(),
        };
        let phased = err.in_phase(IpcErrorPhase::Connect);
        assert!(phased.source().is_none());
    }

    // ── is_recoverable tests ──

    #[test]
    fn is_recoverable_true_for_connection_refused() {
        let err = IpcError::ConnectionRefused(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "refused",
        ));
        assert!(err.is_recoverable());
    }

    #[test]
    fn is_recoverable_true_for_connection_reset() {
        let err = IpcError::ConnectionReset(std::io::Error::new(
            std::io::ErrorKind::ConnectionReset,
            "reset",
        ));
        assert!(err.is_recoverable());
        assert!(err.is_retriable());
    }

    #[test]
    fn is_recoverable_true_for_timeout() {
        let err = IpcError::Timeout(std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"));
        assert!(err.is_recoverable());
        assert!(err.is_retriable());
    }

    #[test]
    fn is_recoverable_true_for_application_error() {
        let err = IpcError::ApplicationError {
            code: -32_603,
            message: "internal".to_owned(),
            data: None,
        };
        assert!(err.is_recoverable());
        assert!(!err.is_retriable());
    }

    #[test]
    fn is_recoverable_false_for_method_not_found() {
        let err = IpcError::MethodNotFound {
            method: "foo.bar".to_owned(),
        };
        assert!(!err.is_recoverable());
    }

    #[test]
    fn is_recoverable_false_for_serialization_error() {
        let err = IpcError::SerializationError {
            detail: "bad json".to_owned(),
        };
        assert!(!err.is_recoverable());
    }

    #[test]
    fn is_recoverable_false_for_socket_not_found() {
        let err = IpcError::SocketNotFound {
            primal: "beardog".to_owned(),
        };
        assert!(!err.is_recoverable());
    }

    #[test]
    fn is_recoverable_false_for_protocol_error() {
        let err = IpcError::ProtocolError {
            detail: "bad".to_owned(),
        };
        assert!(!err.is_recoverable());
    }
}
