// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Bearer Token Auth — absorbed from exp054.

use crate::composition::CompositionContext;
use crate::ipc::protocol::JsonRpcRequest;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "bearer-token-auth",
        track: Track::Security,
        tier: Tier::Live,
        provenance_crate: "exp054_bearer_token_auth",
        provenance_date: "2026-05-09",
        description: "Bearer token auth — JSON-RPC structure and security → compute flow",
    },
    run,
};

fn extract_bearer_token(val: &serde_json::Value) -> Option<String> {
    val.get("token")
        .and_then(|v| v.as_str().map(std::string::ToString::to_string))
        .or_else(|| {
            val.get("access_token")
                .and_then(|v| v.as_str().map(std::string::ToString::to_string))
        })
        .or_else(|| {
            val.get("bearer_token")
                .and_then(|v| v.as_str().map(std::string::ToString::to_string))
        })
}

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural");
    phase_structural(v);

    v.section("Phase 2: Token Issuance");
    if ctx.has_capability("security") {
        v.check_bool(
            "has_security_capability",
            true,
            "security capability present in composition context",
        );
    } else {
        v.check_skip(
            "has_security_capability",
            "security capability not in composition context",
        );
    }
    phase_token_issuance(v, ctx);

    v.section("Phase 3: Authenticated Call");
    phase_authenticated_call(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let auth_req = JsonRpcRequest::new(
        "security.authenticate",
        serde_json::json!({"credentials": "bearer"}),
    );
    let auth_line = auth_req.to_line();
    let auth_ok = auth_line.is_ok();
    let auth_str = auth_line
        .as_ref()
        .map(std::string::String::as_str)
        .unwrap_or("");
    v.check_bool(
        "security_authenticate_request_serializes",
        auth_ok && auth_str.contains("security.authenticate"),
        "JsonRpcRequest for \"security.authenticate\" serializes",
    );
}

fn phase_token_issuance(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "security_authenticate",
            "security capability not in composition context",
        );
        return;
    }
    match ctx.call(
        "security",
        "security.authenticate",
        serde_json::json!({"credentials": "bearer"}),
    ) {
        Ok(val) => {
            if let Some(tok) = extract_bearer_token(&val) {
                ctx.set_bearer_token(tok);
                v.check_bool(
                    "authenticate_yields_token",
                    true,
                    "security.authenticate returned a bearer token field",
                );
            } else {
                v.check_bool(
                    "authenticate_yields_token",
                    false,
                    "security.authenticate ok but no extractable token field",
                );
            }
        }
        Err(e) if e.is_skippable() => v.check_skip("security_authenticate", &format!("{e}")),
        Err(e) => v.check_bool("security_authenticate", false, &format!("error: {e}")),
    }
}

fn phase_authenticated_call(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("compute") {
        v.check_skip(
            "authenticated_compute_submit",
            "compute capability not in composition context",
        );
        return;
    }
    if !ctx.has_bearer_token() {
        v.check_skip(
            "authenticated_compute_submit",
            "no bearer token (phase 2 did not issue one)",
        );
        return;
    }
    match ctx.call_authenticated(
        "compute",
        "compute.submit",
        serde_json::json!({"shader": "add.wgsl"}),
    ) {
        Ok(_) => v.check_bool(
            "authenticated_compute_submit",
            true,
            "compute.submit with bearer succeeded",
        ),
        Err(e) if e.is_skippable() => {
            v.check_skip("authenticated_compute_submit", &format!("{e}"));
        }
        Err(e) => v.check_bool(
            "authenticated_compute_submit",
            false,
            &format!("error: {e}"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_token_auth_no_panic() {
        let mut v = ValidationResult::new("bearer-token-auth");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
