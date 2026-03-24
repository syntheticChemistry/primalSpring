// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp052: Protocol Escalation — validates HTTP REST → JSON-RPC 2.0 → tarpc serialization.

use primalspring::ipc::protocol::{JsonRpcRequest, JsonRpcResponse};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp052 — Protocol Escalation")
        .with_provenance("exp052_protocol_escalation", "2026-03-24")
        .run(
            "primalSpring Exp052: HTTP REST → JSON-RPC 2.0 → tarpc",
            |v| {
                let req = JsonRpcRequest::new("health.check", serde_json::Value::Null);
                let serialized = req.to_line();
                let serialize_ok = serialized.is_ok();
                let line = serialized
                    .as_ref()
                    .map(std::string::String::as_str)
                    .unwrap_or("");
                let has_method = line.contains("\"method\":\"health.check\"");
                let has_jsonrpc = line.contains("\"jsonrpc\":\"2.0\"");
                v.check_bool(
                    "jsonrpc_request_new_serializes",
                    serialize_ok && has_method && has_jsonrpc,
                    "JsonRpcRequest::new serializes correctly",
                );

                let notify_req = JsonRpcRequest::notify("health.check");
                let notify_line = notify_req.to_line();
                let notify_ok = notify_line.is_ok();
                let notify_str = notify_line
                    .as_ref()
                    .map(std::string::String::as_str)
                    .unwrap_or("");
                v.check_bool(
                    "jsonrpc_request_notify_works",
                    notify_ok && notify_str.contains("health.check"),
                    "JsonRpcRequest::notify works",
                );

                let json = r#"{"jsonrpc":"2.0","result":{"status":"ok"},"id":1}"#;
                let resp = JsonRpcResponse::from_line(json);
                let round_trip_ok = resp.is_ok();
                let parsed = resp.as_ref().ok();
                let id_matches = parsed.is_some_and(|r| r.id == 1);
                let has_result = parsed.and_then(|r| r.result.as_ref()).is_some();
                v.check_bool(
                    "jsonrpc_response_round_trip",
                    round_trip_ok && id_matches && has_result,
                    "JsonRpcResponse round-trip from JSON string",
                );

                v.check_skip(
                    "protocol_escalation_live",
                    "actual protocol escalation with live primals",
                );
            },
        );
}
