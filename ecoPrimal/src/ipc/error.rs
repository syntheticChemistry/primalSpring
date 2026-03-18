// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed IPC error with semantic classification and query methods.
//!
//! Converged ecosystem pattern: every spring and primal uses a typed
//! `IpcError` with `is_retriable()`, `is_connection_error()`, etc.
//! rather than raw `std::io::Error`. This enables `CircuitBreaker`
//! and `RetryPolicy` to make informed retry decisions.

use super::protocol::{JsonRpcError, error_codes};

/// Semantic IPC error — classifies failures by what happened rather
/// than where in the code path the failure occurred.
#[derive(Debug)]
pub enum IpcError {
    /// No socket file found for the requested primal.
    SocketNotFound {
        /// Primal name that was searched for.
        primal: String,
    },
    /// Socket exists but the connection was actively refused.
    ConnectionRefused(std::io::Error),
    /// Connection was established but dropped mid-communication.
    ConnectionReset(std::io::Error),
    /// Operation exceeded the configured timeout.
    Timeout(std::io::Error),
    /// Wire-level protocol violation (malformed JSON, empty response, etc.).
    ProtocolError {
        /// Human-readable description of the protocol violation.
        detail: String,
    },
    /// Server explicitly reported the method does not exist.
    MethodNotFound {
        /// The method name or server message.
        method: String,
    },
    /// Server returned a JSON-RPC error that is not `MethodNotFound`.
    ApplicationError {
        /// JSON-RPC error code.
        code: i64,
        /// Human-readable error message from the server.
        message: String,
        /// Optional structured error data.
        data: Option<serde_json::Value>,
    },
    /// Failed to serialize a request or deserialize a typed result.
    SerializationError {
        /// Human-readable description of the serialization failure.
        detail: String,
    },
}

impl IpcError {
    /// Whether a retry is likely to succeed (transient failures).
    #[must_use]
    pub const fn is_retriable(&self) -> bool {
        matches!(self, Self::ConnectionReset(_) | Self::Timeout(_))
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

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SocketNotFound { primal } => {
                write!(f, "no socket found for primal '{primal}'")
            }
            Self::ConnectionRefused(e) => write!(f, "connection refused: {e}"),
            Self::ConnectionReset(e) => write!(f, "connection reset: {e}"),
            Self::Timeout(e) => write!(f, "timeout: {e}"),
            Self::ProtocolError { detail } => write!(f, "protocol error: {detail}"),
            Self::MethodNotFound { method } => write!(f, "method not found: {method}"),
            Self::ApplicationError { code, message, .. } => {
                write!(f, "application error {code}: {message}")
            }
            Self::SerializationError { detail } => write!(f, "serialization error: {detail}"),
        }
    }
}

impl std::error::Error for IpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConnectionRefused(e) | Self::ConnectionReset(e) | Self::Timeout(e) => Some(e),
            _ => None,
        }
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
}
