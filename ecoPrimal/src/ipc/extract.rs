// SPDX-License-Identifier: AGPL-3.0-or-later

//! Centralized JSON-RPC result extraction.
//!
//! Converged ecosystem pattern from wetSpring and ludoSpring. Replaces ad-hoc
//! `response.result.unwrap()` / `serde_json::from_value()` with a single
//! extraction point that handles errors, missing results, and deserialization
//! in one call.

use serde::de::DeserializeOwned;

use super::dispatch::DispatchOutcome;
use super::error::IpcError;
use super::protocol::JsonRpcResponse;

/// Extract and deserialize a typed result from a JSON-RPC response.
///
/// Returns `Ok(T)` if the response has a `result` field that deserializes
/// into `T`. Returns an appropriate `IpcError` for JSON-RPC errors, missing
/// results, or deserialization failures.
///
/// # Errors
///
/// - `IpcError::MethodNotFound` or `IpcError::ApplicationError` if the
///   response contains a JSON-RPC error.
/// - `IpcError::ProtocolError` if the response has no result and no error.
/// - `IpcError::SerializationError` if the result cannot be deserialized into `T`.
pub fn extract_rpc_result<T: DeserializeOwned>(response: &JsonRpcResponse) -> Result<T, IpcError> {
    if let Some(ref err) = response.error {
        return Err(IpcError::from(err.clone()));
    }

    let Some(ref result) = response.result else {
        return Err(IpcError::ProtocolError {
            detail: "response has neither result nor error".to_owned(),
        });
    };

    serde_json::from_value(result.clone()).map_err(|e| IpcError::SerializationError {
        detail: e.to_string(),
    })
}

/// Classify a JSON-RPC response into a three-way `DispatchOutcome`.
///
/// Unlike `extract_rpc_result`, this preserves the distinction between
/// protocol errors and application errors for use with `should_retry()`.
#[must_use]
pub fn extract_rpc_dispatch<T: DeserializeOwned>(response: &JsonRpcResponse) -> DispatchOutcome<T> {
    if let Some(ref err) = response.error {
        return DispatchOutcome::ApplicationError {
            code: err.code,
            message: err.message.clone(),
            data: err.data.clone(),
        };
    }

    let Some(ref result) = response.result else {
        return DispatchOutcome::ProtocolError(IpcError::ProtocolError {
            detail: "response has neither result nor error".to_owned(),
        });
    };

    match serde_json::from_value(result.clone()) {
        Ok(val) => DispatchOutcome::Success(val),
        Err(e) => DispatchOutcome::ProtocolError(IpcError::SerializationError {
            detail: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::protocol::{JSONRPC_VERSION, JsonRpcError, error_codes};

    fn success_response(result: serde_json::Value) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(result),
            error: None,
            id: 1,
        }
    }

    fn error_response(code: i64, message: &str) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.to_owned(),
                data: None,
            }),
            id: 1,
        }
    }

    fn empty_response() -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: None,
            id: 1,
        }
    }

    #[test]
    fn extract_success_string() {
        let resp = success_response(serde_json::json!("hello"));
        let val: String = extract_rpc_result(&resp).unwrap();
        assert_eq!(val, "hello");
    }

    #[test]
    fn extract_success_struct() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Status {
            ok: bool,
        }
        let resp = success_response(serde_json::json!({"ok": true}));
        let val: Status = extract_rpc_result(&resp).unwrap();
        assert!(val.ok);
    }

    #[test]
    fn extract_method_not_found() {
        let resp = error_response(error_codes::METHOD_NOT_FOUND, "no such method");
        let err = extract_rpc_result::<String>(&resp).unwrap_err();
        assert!(err.is_method_not_found());
    }

    #[test]
    fn extract_application_error() {
        let resp = error_response(error_codes::INTERNAL_ERROR, "boom");
        let err = extract_rpc_result::<String>(&resp).unwrap_err();
        assert!(!err.is_method_not_found());
        assert!(matches!(err, IpcError::ApplicationError { .. }));
    }

    #[test]
    fn extract_empty_response() {
        let resp = empty_response();
        let err = extract_rpc_result::<String>(&resp).unwrap_err();
        assert!(matches!(err, IpcError::ProtocolError { .. }));
    }

    #[test]
    fn extract_type_mismatch() {
        let resp = success_response(serde_json::json!("not a number"));
        let err = extract_rpc_result::<u64>(&resp).unwrap_err();
        assert!(matches!(err, IpcError::SerializationError { .. }));
    }

    #[test]
    fn dispatch_success() {
        let resp = success_response(serde_json::json!(42));
        let outcome: DispatchOutcome<i32> = extract_rpc_dispatch(&resp);
        assert!(outcome.is_success());
        assert_eq!(outcome.into_result().unwrap(), 42);
    }

    #[test]
    fn dispatch_application_error() {
        let resp = error_response(error_codes::INTERNAL_ERROR, "boom");
        let outcome: DispatchOutcome<i32> = extract_rpc_dispatch(&resp);
        assert!(!outcome.should_retry());
        assert!(!outcome.is_success());
    }

    #[test]
    fn dispatch_empty_is_protocol_error() {
        let resp = empty_response();
        let outcome: DispatchOutcome<i32> = extract_rpc_dispatch(&resp);
        assert!(!outcome.is_success());
    }

    mod proptest_fuzz {
        use super::*;
        use proptest::prelude::*;

        fn arb_error_code() -> impl Strategy<Value = i64> {
            prop_oneof![
                Just(error_codes::PARSE_ERROR),
                Just(error_codes::INVALID_REQUEST),
                Just(error_codes::METHOD_NOT_FOUND),
                Just(error_codes::INVALID_PARAMS),
                Just(error_codes::INTERNAL_ERROR),
                (-50_000i64..-20_000),
            ]
        }

        proptest! {
            #[test]
            fn extract_rpc_result_never_panics_on_success(
                val in "[a-zA-Z0-9]{0,50}",
            ) {
                let resp = success_response(serde_json::Value::String(val));
                let _ = extract_rpc_result::<String>(&resp);
            }

            #[test]
            fn extract_rpc_result_never_panics_on_error(
                code in arb_error_code(),
                msg in "[a-zA-Z ]{1,50}",
            ) {
                let resp = error_response(code, &msg);
                let err = extract_rpc_result::<String>(&resp).unwrap_err();
                if code == error_codes::METHOD_NOT_FOUND {
                    prop_assert!(err.is_method_not_found());
                }
            }

            #[test]
            fn extract_rpc_dispatch_classifies_correctly(
                code in arb_error_code(),
                msg in "[a-zA-Z ]{1,50}",
            ) {
                let resp = error_response(code, &msg);
                let outcome: DispatchOutcome<String> = extract_rpc_dispatch(&resp);
                prop_assert!(!outcome.is_success());
            }

            #[test]
            fn dispatch_success_round_trips_value(
                val in 0i64..1_000_000,
            ) {
                let resp = success_response(serde_json::json!(val));
                let outcome: DispatchOutcome<i64> = extract_rpc_dispatch(&resp);
                prop_assert!(outcome.is_success());
                prop_assert_eq!(outcome.into_result().unwrap(), val);
            }
        }
    }
}
