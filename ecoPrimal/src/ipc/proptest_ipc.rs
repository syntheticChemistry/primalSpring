// SPDX-License-Identifier: AGPL-3.0-or-later

//! Consolidated IPC property tests — cross-cutting fuzz for the full protocol surface.
//!
//! Absorbed from healthSpring V41 `proptest_ipc.rs` pattern. Each IPC submodule
//! has co-located unit-level proptests; this module provides **cross-cutting**
//! property tests that span discovery → protocol → extraction → dispatch.
//!
//! Invariants tested:
//! - Capability extraction never panics on arbitrary JSON
//! - `extract_rpc_result` and `extract_rpc_dispatch` produce consistent outcomes
//! - Multi-format capability parsing round-trips (Formats A–D)
//! - Error classification is consistent between `extract` and `dispatch` paths

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::ipc::discover::extract_capability_names;
    use crate::ipc::dispatch::DispatchOutcome;
    use crate::ipc::extract::{extract_rpc_dispatch, extract_rpc_result};
    use crate::ipc::protocol::{JSONRPC_VERSION, JsonRpcError, JsonRpcResponse, error_codes};

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

    fn arb_capability_string() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-z]{1,10}\\.[a-z]{1,10}").expect("valid regex")
    }

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

    fn arb_json_value() -> impl Strategy<Value = serde_json::Value> {
        prop_oneof![
            Just(serde_json::Value::Null),
            any::<bool>().prop_map(serde_json::Value::Bool),
            any::<i64>().prop_map(|n| serde_json::json!(n)),
            "[a-zA-Z0-9 _-]{0,30}".prop_map(serde_json::Value::String),
        ]
    }

    proptest! {
        /// extract_rpc_result and extract_rpc_dispatch agree on success vs error.
        #[test]
        fn extract_result_dispatch_consistency(val in arb_json_value()) {
            let resp = success_response(val);
            let result_ok = extract_rpc_result::<serde_json::Value>(&resp).is_ok();
            let dispatch_ok: DispatchOutcome<serde_json::Value> = extract_rpc_dispatch(&resp);
            prop_assert_eq!(result_ok, dispatch_ok.is_success());
        }

        /// Error responses classify consistently across both extraction paths.
        #[test]
        fn error_paths_agree(
            code in arb_error_code(),
            msg in "[a-zA-Z ]{1,30}",
        ) {
            let resp = error_response(code, &msg);
            let result_err = extract_rpc_result::<serde_json::Value>(&resp).is_err();
            let dispatch: DispatchOutcome<serde_json::Value> = extract_rpc_dispatch(&resp);
            prop_assert!(result_err);
            prop_assert!(!dispatch.is_success());
        }

        /// Multi-format capability parsing round-trips through Format A (flat array).
        #[test]
        fn format_a_capability_round_trip(
            caps in prop::collection::vec(arb_capability_string(), 0..20),
        ) {
            let val = serde_json::json!(caps);
            let names = extract_capability_names(Some(val));
            prop_assert_eq!(names.len(), caps.len());
        }

        /// Multi-format capability parsing round-trips through Format B (object array).
        #[test]
        fn format_b_capability_round_trip(
            caps in prop::collection::vec(arb_capability_string(), 0..20),
        ) {
            let arr: Vec<serde_json::Value> = caps
                .iter()
                .map(|c| serde_json::json!({"method": c}))
                .collect();
            let val = serde_json::Value::Array(arr);
            let names = extract_capability_names(Some(val));
            prop_assert_eq!(names.len(), caps.len());
        }

        /// Format C round-trips through method_info extraction.
        #[test]
        fn format_c_capability_round_trip(
            caps in prop::collection::vec(arb_capability_string(), 0..20),
        ) {
            let info: Vec<serde_json::Value> = caps
                .iter()
                .map(|c| serde_json::json!({"name": c, "params": []}))
                .collect();
            let val = serde_json::json!({"method_info": info});
            let names = extract_capability_names(Some(val));
            prop_assert_eq!(names.len(), caps.len());
        }

        /// Capability extraction never panics on arbitrary strings.
        #[test]
        fn capability_extraction_never_panics(input in "[\\PC]{0,200}") {
            let val = serde_json::from_str::<serde_json::Value>(&input).ok();
            let _ = extract_capability_names(val);
        }

        /// Capability extraction on None returns empty.
        #[test]
        fn capability_none_returns_empty(_dummy in 0..10u32) {
            let names = extract_capability_names(None);
            prop_assert!(names.is_empty());
        }
    }
}
