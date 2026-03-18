// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC 2.0 protocol types for primal IPC.
//!
//! Line-delimited JSON-RPC 2.0 over Unix sockets, following the
//! ecosystem `UNIVERSAL_IPC_STANDARD_V3.md`. Method names use
//! `domain.verb` semantic naming (e.g., `health.check`, `crypto.sign`).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// JSON-RPC protocol version string — always `"2.0"`.
pub const JSONRPC_VERSION: &str = "2.0";

/// Global request ID counter for multiplexed IPC.
static NEXT_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

/// Allocate the next unique request ID.
fn next_id() -> u64 {
    NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}

/// JSON-RPC 2.0 request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// Protocol version — always `"2.0"`.
    pub jsonrpc: String,
    /// Method name in `domain.verb` format (e.g. `"health.check"`).
    pub method: String,
    /// Method parameters (omitted from wire format when null).
    #[serde(skip_serializing_if = "is_null")]
    pub params: serde_json::Value,
    /// Auto-incremented request ID for multiplexing.
    pub id: u64,
}

fn is_null(v: &serde_json::Value) -> bool {
    v.is_null()
}

impl JsonRpcRequest {
    /// Create a new request with auto-incremented ID.
    #[must_use]
    pub fn new(method: &str, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            method: method.to_owned(),
            params,
            id: next_id(),
        }
    }

    /// Create a request with no parameters.
    #[must_use]
    pub fn notify(method: &str) -> Self {
        Self::new(method, serde_json::Value::Null)
    }

    /// Serialize to a newline-delimited JSON string.
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if serialization fails.
    pub fn to_line(&self) -> Result<String, serde_json::Error> {
        let mut s = serde_json::to_string(self)?;
        s.push('\n');
        Ok(s)
    }
}

/// JSON-RPC 2.0 response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// Protocol version — always `"2.0"`.
    pub jsonrpc: String,
    /// Successful result value (mutually exclusive with `error`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error object (mutually exclusive with `result`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID this response corresponds to.
    pub id: u64,
}

impl JsonRpcResponse {
    /// Whether this response indicates success.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.error.is_none()
    }

    /// Parse from a JSON string (line-delimited).
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if the input is not valid JSON-RPC.
    pub fn from_line(line: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(line.trim())
    }
}

/// JSON-RPC 2.0 error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Numeric error code (negative for protocol errors).
    pub code: i64,
    /// Human-readable error message.
    pub message: String,
    /// Optional structured error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON-RPC error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for JsonRpcError {}

/// Standard JSON-RPC 2.0 error codes.
pub mod error_codes {
    /// Invalid JSON was received by the server.
    pub const PARSE_ERROR: i64 = -32_700;
    /// The JSON sent is not a valid JSON-RPC request.
    pub const INVALID_REQUEST: i64 = -32_600;
    /// The method does not exist or is not available.
    pub const METHOD_NOT_FOUND: i64 = -32_601;
    /// Invalid method parameters.
    pub const INVALID_PARAMS: i64 = -32_602;
    /// Internal server error.
    pub const INTERNAL_ERROR: i64 = -32_603;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_ids_auto_increment() {
        let r1 = JsonRpcRequest::new("health.check", serde_json::Value::Null);
        let r2 = JsonRpcRequest::new("health.check", serde_json::Value::Null);
        assert!(r2.id > r1.id);
    }

    #[test]
    fn request_jsonrpc_version_is_2_0() {
        let req = JsonRpcRequest::new("test.method", serde_json::Value::Null);
        assert_eq!(req.jsonrpc, "2.0");
    }

    #[test]
    fn request_serializes_to_line() {
        let req = JsonRpcRequest::notify("health.check");
        let line = req.to_line().unwrap();
        assert!(line.ends_with('\n'));
        assert!(line.contains("\"method\":\"health.check\""));
        assert!(line.contains("\"jsonrpc\":\"2.0\""));
    }

    #[test]
    fn request_skips_null_params() {
        let req = JsonRpcRequest::notify("health.check");
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("\"params\""));
    }

    #[test]
    fn request_includes_non_null_params() {
        let req = JsonRpcRequest::new("compute.submit", serde_json::json!({"shader": "add.wgsl"}));
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"params\""));
    }

    #[test]
    fn response_success_round_trip() {
        let json = r#"{"jsonrpc":"2.0","result":{"status":"ok"},"id":1}"#;
        let resp = JsonRpcResponse::from_line(json).unwrap();
        assert!(resp.is_success());
        assert_eq!(resp.id, 1);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn response_error_round_trip() {
        let json =
            r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"method not found"},"id":2}"#;
        let resp = JsonRpcResponse::from_line(json).unwrap();
        assert!(!resp.is_success());
        let err = resp.error.unwrap();
        assert_eq!(err.code, error_codes::METHOD_NOT_FOUND);
    }

    #[test]
    fn error_display() {
        let err = JsonRpcError {
            code: -32_601,
            message: "method not found".to_owned(),
            data: None,
        };
        assert_eq!(err.to_string(), "JSON-RPC error -32601: method not found");
    }

    mod proptest_fuzz {
        use super::*;
        use proptest::prelude::*;

        fn arb_method() -> impl Strategy<Value = String> {
            prop::string::string_regex("[a-z]{1,20}\\.[a-z]{1,20}").expect("valid regex")
        }

        fn arb_params() -> impl Strategy<Value = serde_json::Value> {
            prop_oneof![
                Just(serde_json::Value::Null),
                any::<bool>().prop_map(serde_json::Value::Bool),
                any::<i64>().prop_map(|n| serde_json::Value::Number(serde_json::Number::from(n))),
                "[a-zA-Z0-9 _-]{0,50}".prop_map(serde_json::Value::String),
            ]
        }

        proptest! {
            #[test]
            fn request_round_trips_through_json(
                method in arb_method(),
                params in arb_params(),
            ) {
                let req = JsonRpcRequest::new(&method, params.clone());
                let line = req.to_line().unwrap();
                let parsed: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
                prop_assert_eq!(parsed["method"].as_str().unwrap(), method.as_str());
                prop_assert_eq!(parsed["jsonrpc"].as_str().unwrap(), "2.0");
                if !params.is_null() {
                    prop_assert_eq!(&parsed["params"], &params);
                }
            }

            #[test]
            fn response_from_line_never_panics(
                input in "\\PC{0,500}",
            ) {
                let _ = JsonRpcResponse::from_line(&input);
            }

            #[test]
            fn success_response_always_parses(
                id in 0u64..1_000_000,
                result_str in "[a-zA-Z0-9]{0,50}",
            ) {
                let json = format!(
                    r#"{{"jsonrpc":"2.0","result":"{result_str}","id":{id}}}"#,
                );
                let resp = JsonRpcResponse::from_line(&json).unwrap();
                prop_assert!(resp.is_success());
                prop_assert_eq!(resp.id, id);
            }

            #[test]
            fn error_response_always_parses(
                id in 0u64..1_000_000,
                code in -40_000i64..-30_000,
                msg in "[a-zA-Z ]{1,50}",
            ) {
                let json = format!(
                    r#"{{"jsonrpc":"2.0","error":{{"code":{code},"message":"{msg}"}},"id":{id}}}"#,
                );
                let resp = JsonRpcResponse::from_line(&json).unwrap();
                prop_assert!(!resp.is_success());
                let err = resp.error.unwrap();
                prop_assert_eq!(err.code, code);
            }

            #[test]
            fn request_notify_has_null_params(
                method in arb_method(),
            ) {
                let req = JsonRpcRequest::notify(&method);
                let json = serde_json::to_string(&req).unwrap();
                prop_assert!(!json.contains("\"params\""));
            }
        }
    }
}
