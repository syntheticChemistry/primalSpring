// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp054: Bearer Token Auth — structural JSON-RPC plus live security → compute flow.

use primalspring::composition::CompositionContext;
use primalspring::ipc::protocol::JsonRpcRequest;
use primalspring::validation::ValidationResult;

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

fn phase_structural(v: &mut ValidationResult) {
    let auth_req = JsonRpcRequest::new(
        "security.authenticate",
        serde_json::json!({"credentials": "bearer"}),
    );
    let auth_line = auth_req.to_line();
    let auth_ok = auth_line.is_ok();
    let auth_str = auth_line.as_deref().unwrap_or("");
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
        Err(e) if e.is_connection_error() => v.check_skip("security_authenticate", &format!("{e}")),
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
        Err(e) if e.is_connection_error() => {
            v.check_skip("authenticated_compute_submit", &format!("{e}"));
        }
        Err(e) => v.check_bool(
            "authenticated_compute_submit",
            false,
            &format!("error: {e}"),
        ),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp054 — Bearer Token Auth")
        .with_provenance("exp054_bearer_token_auth", "2026-05-09")
        .run(
            "primalSpring Exp054: BearDog security.authenticate → compute.submit",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Token Issuance");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
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
                phase_token_issuance(v, &mut ctx);

                v.section("Phase 3: Authenticated Call");
                phase_authenticated_call(v, &mut ctx);
            },
        );
}
