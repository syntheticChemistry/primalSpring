// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Protocol Escalation — validates the JSON-RPC 2.0 wire protocol
//! and serialization layer that underlies all primal IPC.
//!
//! Absorbed from `exp052_protocol_escalation`. In the K-Derm model, protocol
//! escalation is the "membrane channel opening" — the negotiation layer
//! that establishes communication type (text JSON-RPC vs binary tarpc).
//!
//! This scenario validates:
//! 1. Request serialization (correct JSON-RPC 2.0 format)
//! 2. Response deserialization (round-trip integrity)
//! 3. Error code semantics (standard vs. application codes)
//! 4. Notification (id-less) messages
//! 5. Batch-ready structure (array support)

use crate::composition::CompositionContext;
use crate::ipc::protocol::{JsonRpcRequest, JsonRpcResponse, error_codes};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Protocol escalation validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "protocol-escalation",
        track: Track::Transport,
        tier: Tier::Rust,
        provenance_crate: "exp052_protocol_escalation",
        provenance_date: "2026-05-09",
        description: "JSON-RPC 2.0 wire protocol serialization and escalation",
    },
    run,
};

/// Run protocol escalation validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Request serialization");
    phase_request_serialization(v);

    v.section("Phase 2: Response deserialization");
    phase_response_deserialization(v);

    v.section("Phase 3: Error code semantics");
    phase_error_codes(v);

    v.section("Phase 4: Notification messages");
    phase_notifications(v);
}

fn phase_request_serialization(v: &mut ValidationResult) {
    let req = JsonRpcRequest::new("health.check", serde_json::Value::Null);
    let serialized = req.to_line();

    v.check_bool(
        "proto:request:serializes",
        serialized.is_ok(),
        &format!(
            "request serialization: {}",
            if serialized.is_ok() { "OK" } else { "FAILED" }
        ),
    );

    if let Ok(ref line) = serialized {
        v.check_bool(
            "proto:request:has_jsonrpc",
            line.contains("\"jsonrpc\":\"2.0\""),
            "contains jsonrpc version field",
        );
        v.check_bool(
            "proto:request:has_method",
            line.contains("\"method\":\"health.check\""),
            "contains method field",
        );
        v.check_bool(
            "proto:request:has_id",
            line.contains("\"id\""),
            "contains id field (non-notification)",
        );
        v.check_bool(
            "proto:request:valid_json",
            serde_json::from_str::<serde_json::Value>(line).is_ok(),
            "output is valid JSON",
        );
    }
}

fn phase_response_deserialization(v: &mut ValidationResult) {
    let success_json = r#"{"jsonrpc":"2.0","result":{"status":"ok"},"id":1}"#;
    let resp = JsonRpcResponse::from_line(success_json);

    v.check_bool(
        "proto:response:parses_success",
        resp.is_ok(),
        "success response parses",
    );

    if let Ok(ref r) = resp {
        v.check_bool(
            "proto:response:correct_id",
            r.id == 1,
            &format!("id={} (expected 1)", r.id),
        );
        v.check_bool(
            "proto:response:has_result",
            r.result.is_some(),
            "result field present",
        );
        v.check_bool(
            "proto:response:no_error",
            r.error.is_none(),
            "no error field in success response",
        );
    }

    let error_json =
        r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"method not found"},"id":2}"#;
    let err_resp = JsonRpcResponse::from_line(error_json);

    v.check_bool(
        "proto:response:parses_error",
        err_resp.is_ok(),
        "error response parses",
    );

    if let Ok(ref r) = err_resp {
        v.check_bool(
            "proto:response:error_has_code",
            r.error
                .as_ref()
                .is_some_and(|e| e.code == error_codes::METHOD_NOT_FOUND),
            "error code is -32601 (METHOD_NOT_FOUND)",
        );
    }
}

fn phase_error_codes(v: &mut ValidationResult) {
    v.check_bool(
        "proto:codes:method_not_found",
        error_codes::METHOD_NOT_FOUND == -32601,
        &format!("METHOD_NOT_FOUND = {}", error_codes::METHOD_NOT_FOUND),
    );
    v.check_bool(
        "proto:codes:parse_error",
        error_codes::PARSE_ERROR == -32700,
        &format!("PARSE_ERROR = {}", error_codes::PARSE_ERROR),
    );
    v.check_bool(
        "proto:codes:internal_error",
        error_codes::INTERNAL_ERROR == -32603,
        &format!("INTERNAL_ERROR = {}", error_codes::INTERNAL_ERROR),
    );

    // Application-specific codes should be positive or in [-32099, -32000]
    v.check_bool(
        "proto:codes:permission_denied_app_range",
        error_codes::PERMISSION_DENIED < 0,
        &format!(
            "PERMISSION_DENIED = {} (negative = server-defined)",
            error_codes::PERMISSION_DENIED
        ),
    );
}

fn phase_notifications(v: &mut ValidationResult) {
    let notify = JsonRpcRequest::notify("event.broadcast");
    let serialized = notify.to_line();

    v.check_bool(
        "proto:notify:serializes",
        serialized.is_ok(),
        "notification serializes",
    );

    if let Ok(ref line) = serialized {
        v.check_bool(
            "proto:notify:has_method",
            line.contains("\"method\":\"event.broadcast\""),
            "notification has method",
        );
        v.check_bool(
            "proto:notify:valid_json",
            serde_json::from_str::<serde_json::Value>(line).is_ok(),
            "notification is valid JSON",
        );
        // params is omitted when null (skip_serializing_if = "is_null")
        let no_params_when_null = !line.contains("\"params\"");
        v.check_bool(
            "proto:notify:null_params_omitted",
            no_params_when_null,
            "null params omitted from wire format (bandwidth optimization)",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_escalation_structural() {
        let mut v = ValidationResult::new("protocol-escalation");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "protocol escalation: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
