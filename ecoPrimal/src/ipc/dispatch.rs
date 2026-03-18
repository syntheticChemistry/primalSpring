// SPDX-License-Identifier: AGPL-3.0-or-later

//! `DispatchOutcome` — three-way classification of JSON-RPC results.
//!
//! Converged ecosystem pattern from loamSpine, airSpring, and healthSpring.
//! Separates transport-level protocol errors from server-returned application
//! errors and successful results. Enables `should_retry()` without matching
//! against raw error variants.

use super::error::IpcError;

/// Three-way outcome of dispatching a JSON-RPC call.
#[derive(Debug)]
pub enum DispatchOutcome<T> {
    /// Call succeeded with a typed result.
    Success(T),
    /// Transport or protocol-level failure (socket, timeout, malformed JSON).
    ProtocolError(IpcError),
    /// Server returned a JSON-RPC error (method not found, invalid params, etc.).
    ApplicationError {
        /// JSON-RPC error code.
        code: i64,
        /// Human-readable error message from the server.
        message: String,
        /// Optional structured error data.
        data: Option<serde_json::Value>,
    },
}

impl<T> DispatchOutcome<T> {
    /// Whether this outcome warrants a retry.
    ///
    /// Protocol errors that are retriable (timeout, reset) return `true`.
    /// Application errors and successes return `false`.
    #[must_use]
    pub const fn should_retry(&self) -> bool {
        match self {
            Self::ProtocolError(e) => e.is_retriable(),
            Self::Success(_) | Self::ApplicationError { .. } => false,
        }
    }

    /// Whether this outcome is a success.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    /// Convert into a `Result`, folding both error variants into `IpcError`.
    ///
    /// # Errors
    ///
    /// Returns `IpcError` for both protocol and application errors.
    pub fn into_result(self) -> Result<T, IpcError> {
        match self {
            Self::Success(val) => Ok(val),
            Self::ProtocolError(e) => Err(e),
            Self::ApplicationError {
                code,
                message,
                data,
            } => Err(IpcError::ApplicationError {
                code,
                message,
                data,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_is_not_retriable() {
        let outcome: DispatchOutcome<i32> = DispatchOutcome::Success(42);
        assert!(!outcome.should_retry());
        assert!(outcome.is_success());
    }

    #[test]
    fn success_into_result() {
        let outcome = DispatchOutcome::Success(42);
        assert_eq!(outcome.into_result().unwrap(), 42);
    }

    #[test]
    fn protocol_error_timeout_is_retriable() {
        let outcome: DispatchOutcome<i32> = DispatchOutcome::ProtocolError(IpcError::Timeout(
            std::io::Error::new(std::io::ErrorKind::TimedOut, "slow"),
        ));
        assert!(outcome.should_retry());
        assert!(!outcome.is_success());
    }

    #[test]
    fn protocol_error_socket_not_found_is_not_retriable() {
        let outcome: DispatchOutcome<i32> =
            DispatchOutcome::ProtocolError(IpcError::SocketNotFound {
                primal: "beardog".to_owned(),
            });
        assert!(!outcome.should_retry());
    }

    #[test]
    fn application_error_is_not_retriable() {
        let outcome: DispatchOutcome<i32> = DispatchOutcome::ApplicationError {
            code: -32_603,
            message: "internal".to_owned(),
            data: None,
        };
        assert!(!outcome.should_retry());
        assert!(!outcome.is_success());
    }

    #[test]
    fn application_error_into_result() {
        let outcome: DispatchOutcome<i32> = DispatchOutcome::ApplicationError {
            code: -32_603,
            message: "internal".to_owned(),
            data: None,
        };
        let result = outcome.into_result();
        assert!(result.is_err());
    }

    #[test]
    fn protocol_error_into_result() {
        let outcome: DispatchOutcome<i32> =
            DispatchOutcome::ProtocolError(IpcError::SocketNotFound {
                primal: "x".to_owned(),
            });
        let result = outcome.into_result();
        assert!(result.is_err());
    }

    #[test]
    fn application_error_with_data_into_result() {
        let outcome: DispatchOutcome<i32> = DispatchOutcome::ApplicationError {
            code: -32_600,
            message: "invalid".to_owned(),
            data: Some(serde_json::json!({"detail": "bad param"})),
        };
        let result = outcome.into_result();
        assert!(matches!(
            result,
            Err(IpcError::ApplicationError { data: Some(_), .. })
        ));
    }

    #[test]
    fn connection_reset_protocol_error_is_retriable() {
        let outcome: DispatchOutcome<i32> =
            DispatchOutcome::ProtocolError(IpcError::ConnectionReset(std::io::Error::new(
                std::io::ErrorKind::ConnectionReset,
                "reset",
            )));
        assert!(outcome.should_retry());
    }
}
