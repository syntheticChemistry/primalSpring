// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Ionic Token Federation — absorbed from exp108.

use crate::composition::CompositionContext;
use crate::ipc::method_gate::{
    CallerContext, ConnectionOrigin, EnforcementMode, MethodGate, PermissiveVerifier, VerifiedToken,
    scope_permits_method,
};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "token-federation",
        track: Track::Security,
        tier: Tier::Live,
        provenance_crate: "exp108_token_federation",
        provenance_date: "2026-05-09",
        description: "Token federation — scope rules, MethodGate, ionic issue/verify, tensor call",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Scope Validation (pure)");
    phase_scope_validation(v);

    v.section("Phase 2: Gate Enforcement (pure)");
    phase_gate_enforcement(v);

    v.section("Phase 3: Bearer Extraction");
    phase_bearer_extraction(v);

    v.section("Phase 4: Live Token Issuance");
    phase_live_token_issuance(v, ctx);

    v.section("Phase 5: Authenticated Composition");
    phase_authenticated_composition(v, ctx);
}

fn phase_scope_validation(v: &mut ValidationResult) {
    let wide = vec!["*".to_owned()];
    v.check_bool(
        "wildcard_permits_stats",
        scope_permits_method(&wide, "stats.mean"),
        "* matches stats.mean",
    );
    v.check_bool(
        "wildcard_permits_storage",
        scope_permits_method(&wide, "storage.store"),
        "* matches storage.store",
    );

    let narrow = vec!["stats.*".to_owned()];
    v.check_bool(
        "domain_wildcard_permits",
        scope_permits_method(&narrow, "stats.mean"),
        "stats.* matches stats.mean",
    );
    v.check_bool(
        "domain_wildcard_denies_other",
        !scope_permits_method(&narrow, "storage.store"),
        "stats.* does not match storage.store",
    );

    let exact = vec!["stats.mean".to_owned()];
    v.check_bool(
        "exact_permits_match",
        scope_permits_method(&exact, "stats.mean"),
        "stats.mean matches stats.mean",
    );
    v.check_bool(
        "exact_denies_sibling",
        !scope_permits_method(&exact, "stats.variance"),
        "stats.mean does not match stats.variance",
    );

    let multi = vec!["stats.*".to_owned(), "tensor.*".to_owned()];
    v.check_bool(
        "multi_union_stats",
        scope_permits_method(&multi, "stats.mean"),
        "stats.*|tensor.* matches stats.mean",
    );
    v.check_bool(
        "multi_union_tensor",
        scope_permits_method(&multi, "tensor.matmul"),
        "stats.*|tensor.* matches tensor.matmul",
    );
    v.check_bool(
        "multi_union_denies_other",
        !scope_permits_method(&multi, "storage.store"),
        "stats.*|tensor.* does not match storage.store",
    );

    let empty: Vec<String> = vec![];
    v.check_bool(
        "empty_denies_all",
        !scope_permits_method(&empty, "stats.mean"),
        "empty scope set denies all",
    );
}

fn phase_gate_enforcement(v: &mut ValidationResult) {
    let gate = MethodGate::new(EnforcementMode::Enforced);

    let no_token = CallerContext::loopback();
    v.check_bool(
        "public_passes_without_token",
        gate.check("health.check", &no_token).is_ok(),
        "health.check passes in enforced mode without token",
    );
    v.check_bool(
        "protected_rejected_without_token",
        gate.check("stats.mean", &no_token).is_err(),
        "stats.mean rejected in enforced mode without token",
    );

    let matching = CallerContext {
        bearer_token: Some("test-token".to_owned()),
        verified: Some(VerifiedToken {
            scopes: vec!["stats.*".to_owned()],
            subject: Some("test-user".to_owned()),
            expires_in: Some(3600),
        }),
        peer: None,
        origin: ConnectionOrigin::Unix,
    };
    v.check_bool(
        "matching_scope_passes",
        gate.check("stats.mean", &matching).is_ok(),
        "stats.* scope permits stats.mean",
    );

    let wrong = CallerContext {
        bearer_token: Some("test-token".to_owned()),
        verified: Some(VerifiedToken {
            scopes: vec!["storage.*".to_owned()],
            subject: None,
            expires_in: None,
        }),
        peer: None,
        origin: ConnectionOrigin::Unix,
    };
    let result = gate.check("stats.mean", &wrong);
    v.check_bool(
        "wrong_scope_rejected",
        result.is_err(),
        "storage.* scope rejects stats.mean",
    );
    if let Err(ref err) = result {
        let is_scope_mismatch = err
            .data
            .as_ref()
            .and_then(|d| d.get("reason"))
            .and_then(serde_json::Value::as_str)
            == Some("scope_mismatch");
        v.check_bool(
            "rejection_reason_scope_mismatch",
            is_scope_mismatch,
            "error reason = scope_mismatch",
        );
    }
}

fn phase_bearer_extraction(v: &mut ValidationResult) {
    let ctx = CallerContext::loopback();
    let params = serde_json::json!({
        "_bearer_token": "ionic-test-abc",
        "values": [1, 2, 3],
    });
    let enriched = ctx.with_params_token(&params, &PermissiveVerifier);
    v.check_bool(
        "bearer_extracted",
        enriched.bearer_token.as_deref() == Some("ionic-test-abc"),
        "extracted ionic-test-abc from _bearer_token param",
    );
    v.check_bool(
        "noop_verifier_populates_claims",
        enriched.verified.is_some(),
        "PermissiveVerifier produces VerifiedToken",
    );

    let ctx2 = CallerContext::loopback();
    let no_bearer = serde_json::json!({ "values": [1, 2, 3] });
    let plain = ctx2.with_params_token(&no_bearer, &PermissiveVerifier);
    v.check_bool(
        "absent_bearer_stays_none",
        plain.bearer_token.is_none(),
        "no _bearer_token param means None",
    );
    v.check_bool(
        "absent_bearer_no_claims",
        plain.verified.is_none(),
        "no token means no verified claims",
    );
}

fn phase_live_token_issuance(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "live_token_issuance",
            "BearDog not available — live token issuance requires running Tower",
        );
        return;
    }

    let issue_result = ctx.call(
        "security",
        "auth.issue_ionic",
        serde_json::json!({
            "scope": "stats.*",
            "purpose": "exp108_test",
            "ttl_seconds": 300,
        }),
    );

    match issue_result {
        Ok(result) => {
            let token = result.get("token").and_then(serde_json::Value::as_str);
            v.check_bool(
                "ionic_token_issued",
                token.is_some(),
                &format!("response: {result}"),
            );

            if let Some(tok) = token {
                match ctx.call(
                    "security",
                    "auth.verify_ionic",
                    serde_json::json!({ "token": tok }),
                ) {
                    Ok(vr) => {
                        let valid = vr
                            .get("valid")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(false);
                        v.check_bool(
                            "issued_token_verifies",
                            valid,
                            &format!("verify response: {vr}"),
                        );
                    }
                    Err(e) => v.check_skip(
                        "issued_token_verifies",
                        &format!("auth.verify_ionic not available: {e}"),
                    ),
                }
            }
        }
        Err(e) => v.check_skip(
            "ionic_token_issued",
            &format!("auth.issue_ionic not available: {e}"),
        ),
    }
}

fn phase_authenticated_composition(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") || !ctx.has_capability("tensor") {
        v.check_skip(
            "authenticated_composition",
            "BearDog or barraCuda not available — requires running NUCLEUS",
        );
        return;
    }

    let issue_result = ctx.call(
        "security",
        "auth.issue_ionic",
        serde_json::json!({
            "scope": "stats.*",
            "purpose": "exp108_composition",
            "ttl_seconds": 60,
        }),
    );

    match issue_result {
        Ok(result) => {
            if let Some(tok) = result.get("token").and_then(serde_json::Value::as_str) {
                ctx.set_bearer_token(tok);
                let math_result = ctx.call_authenticated(
                    "tensor",
                    "stats.mean",
                    serde_json::json!({ "values": [1.0, 2.0, 3.0, 4.0, 5.0] }),
                );
                v.check_bool(
                    "authenticated_stats_mean",
                    math_result.is_ok(),
                    &format!("result: {math_result:?}"),
                );
            } else {
                v.check_skip("authenticated_stats_mean", "no token in issue response");
            }
        }
        Err(e) => v.check_skip(
            "authenticated_stats_mean",
            &format!("auth.issue_ionic not available: {e}"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_federation_no_panic() {
        let mut v = ValidationResult::new("token-federation");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
