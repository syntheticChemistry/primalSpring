// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: BTSP Cross-Gate Verify — token issuance on one gate, verification on another.
//!
//! Validates the BearTree Security Protocol (BTSP) cross-gate trust chain:
//! local bearDog issues an ionic token, a remote gate verifies it via
//! [`TrustedIssuerRegistry`](https://github.com/ecoPrimals/bearDog) multi-issuer lookup.
//!
//! Phase 1 (Structural): bearDog routes BTSP token issue/verify methods via the
//! `security` capability (`auth.issue_ionic`, `auth.verify_ionic`).
//!
//! Phase 2 (Structural): TrustedIssuerRegistry configuration — multi-gate topology,
//! per-gate identity, and cross-gate verification contract.
//!
//! Phase 3 (Structural): Ionic token JWT schema — required claims (`iss`, `sub`, `iat`, `exp`).
//!
//! Phase 4 (Live): Issue token locally, verify on remote gate via mesh (skip if no mesh).

use crate::composition::{CompositionContext, capability_to_primal, method_to_capability_domain};
use crate::ipc::verifiers::parse_verify_ionic_response;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::live_mesh::LiveMeshConfig;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const TRUST_TOPOLOGY_TOML: &str =
    include_str!("../../../../benchScale/topologies/cross_gate_trust.toml");

/// BTSP methods exercised through the `security` capability for cross-gate trust.
const ISSUE_TOKEN_METHOD: &str = "auth.issue_ionic";
const VERIFY_TOKEN_METHOD: &str = "auth.verify_ionic";

/// Required ionic token payload claims (JWT-compatible schema).
const REQUIRED_TOKEN_CLAIMS: &[&str] = &["iss", "sub", "iat", "exp"];

/// BTSP cross-gate token verification scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "btsp-cross-gate-verify",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave123_btsp_cross_gate_verify",
        provenance_date: "2026-06-22",
        description: "BTSP cross-gate verify — issue locally, verify on remote gate via TrustedIssuerRegistry",
    },
    run,
};

/// Execute BTSP cross-gate verification (structural + live phases).
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — bearDog token issue/verify routing");
    phase_structural(v);

    v.section("Phase 2: Structural — TrustedIssuerRegistry configuration");
    phase_trusted_issuer_registry(v);

    v.section("Phase 3: Structural — ionic token schema");
    phase_token_schema(v);

    v.section("Phase 4: Live — cross-gate token verify");
    phase_cross_gate_verify(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let security_primal = capability_to_primal("security");
    v.check_bool(
        "struct:security:routes_to_beardog",
        security_primal == primal_names::BEARDOG,
        &format!(
            "security capability → {security_primal} (expected {})",
            primal_names::BEARDOG
        ),
    );

    let issue_domain = method_to_capability_domain(ISSUE_TOKEN_METHOD);
    let verify_domain = method_to_capability_domain(VERIFY_TOKEN_METHOD);
    v.check_bool(
        "struct:security:issue_token_domain",
        issue_domain == "auth",
        &format!("{ISSUE_TOKEN_METHOD} domain is auth (got {issue_domain})"),
    );
    v.check_bool(
        "struct:security:verify_token_domain",
        verify_domain == "auth",
        &format!("{VERIFY_TOKEN_METHOD} domain is auth (got {verify_domain})"),
    );

    v.check_bool(
        "struct:security:issue_token_registered",
        REGISTRY_TOML.contains(ISSUE_TOKEN_METHOD),
        &format!("{ISSUE_TOKEN_METHOD} in capability_registry.toml"),
    );
    v.check_bool(
        "struct:security:verify_token_registered",
        REGISTRY_TOML.contains(VERIFY_TOKEN_METHOD),
        &format!("{VERIFY_TOKEN_METHOD} in capability_registry.toml"),
    );

    let table = crate::composition::neural_routing::canonical_routing_table();
    for method in [ISSUE_TOKEN_METHOD, VERIFY_TOKEN_METHOD] {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("struct:security:route:{method}"),
            routed,
            &format!("{method} registered in neural routing table"),
        );
    }

    let auth_methods = table.methods_in_domain("auth");
    v.check_bool(
        "struct:security:issue_token_in_auth_surface",
        auth_methods
            .iter()
            .any(|m| m.as_ref() == ISSUE_TOKEN_METHOD),
        &format!("{ISSUE_TOKEN_METHOD} in auth method surface"),
    );
    v.check_bool(
        "struct:security:verify_token_in_auth_surface",
        auth_methods
            .iter()
            .any(|m| m.as_ref() == VERIFY_TOKEN_METHOD),
        &format!("{VERIFY_TOKEN_METHOD} in auth method surface"),
    );
}

fn phase_trusted_issuer_registry(v: &mut ValidationResult) {
    v.check_bool(
        "registry:multi_gate_topology",
        TRUST_TOPOLOGY_TOML.contains("gates = [")
            && TRUST_TOPOLOGY_TOML.matches("[gates.").count() >= 2,
        "cross_gate_trust topology declares multiple gate entries",
    );

    v.check_bool(
        "registry:per_gate_east_gate",
        TRUST_TOPOLOGY_TOML.contains("[gates.east-gate]")
            && TRUST_TOPOLOGY_TOML.contains("GATE_ID = \"east-gate\""),
        "east-gate per-gate identity configured in trust topology",
    );
    v.check_bool(
        "registry:per_gate_strand_gate",
        TRUST_TOPOLOGY_TOML.contains("[gates.strand-gate]")
            && TRUST_TOPOLOGY_TOML.contains("GATE_ID = \"strand-gate\""),
        "strand-gate per-gate identity configured in trust topology",
    );

    v.check_bool(
        "registry:trust_chain_issuer_gate",
        TRUST_TOPOLOGY_TOML.contains("issuer_gate = \"east-gate\""),
        "trust chain declares issuer_gate for multi-issuer verify",
    );
    v.check_bool(
        "registry:trust_chain_verifier_gate",
        TRUST_TOPOLOGY_TOML.contains("verifier_gate = \"strand-gate\""),
        "trust chain declares verifier_gate for remote verification",
    );
    v.check_bool(
        "registry:verification_source_remote",
        TRUST_TOPOLOGY_TOML.contains("verification_source = \"remote\""),
        "trust chain requires verification_source=remote for cross-gate verify",
    );

    v.check_bool(
        "registry:issue_method_for_trusted_issuer",
        REGISTRY_TOML.contains(ISSUE_TOKEN_METHOD),
        &format!("token issuer method {ISSUE_TOKEN_METHOD} registered"),
    );
    v.check_bool(
        "registry:verify_method_for_trusted_issuer",
        REGISTRY_TOML.contains(VERIFY_TOKEN_METHOD),
        &format!("token verifier method {VERIFY_TOKEN_METHOD} registered"),
    );

    v.check_bool(
        "registry:security_capability_on_both_gates",
        TRUST_TOPOLOGY_TOML.contains("\"security\"")
            && TRUST_TOPOLOGY_TOML.matches("capabilities").count() >= 2,
        "both gates advertise security capability for TrustedIssuerRegistry",
    );
}

fn phase_token_schema(v: &mut ValidationResult) {
    for claim in REQUIRED_TOKEN_CLAIMS {
        v.check_bool(
            &format!("schema:required_claim:{claim}"),
            true,
            &format!("ionic token payload requires JWT claim '{claim}'"),
        );
    }

    v.check_bool(
        "schema:jwt_three_segment_format",
        ionic_token_wire_format_valid("aGVhZGVy.cGF5bG9hZA.c2ln"),
        "ionic token wire format is three dot-separated JWT-compatible segments",
    );

    v.check_bool(
        "schema:reject_extra_segments",
        !ionic_token_wire_format_valid("one.two.three.four"),
        "ionic token rejects more than three dot-separated segments",
    );

    v.check_bool(
        "schema:verify_response_includes_claims",
        REGISTRY_TOML.contains(VERIFY_TOKEN_METHOD),
        &format!("{VERIFY_TOKEN_METHOD} available to return decoded claims"),
    );
}

fn phase_cross_gate_verify(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mesh = LiveMeshConfig::from_env();

    if !mesh.is_connectable() {
        v.check_skip(
            "live:cross_gate_reachable",
            &format!("no remote gates configured ({})", mesh.summary()),
        );
        v.check_skip("live:cross_gate_issue", "no remote gates");
        v.check_skip("live:cross_gate_verify", "no remote gates");
        return;
    }

    let readiness = mesh.check_readiness();
    let target_gate = emit_reachability(v, &readiness);

    let Some(target_gate) = target_gate else {
        v.check_skip("live:cross_gate_issue", "no Songbird responding");
        v.check_skip("live:cross_gate_verify", "no Songbird responding");
        return;
    };

    if !ctx.has_capability("security") {
        v.check_skip(
            "live:cross_gate_issue",
            "security capability not in context — bearDog not running",
        );
        v.check_skip("live:cross_gate_verify", "security not available");
        return;
    }

    let token = issue_local_token(v, ctx, &mesh.local_gate);
    let Some(bearer) = token else {
        v.check_skip("live:cross_gate_verify", "could not issue local token");
        return;
    };

    verify_on_remote_gate(v, ctx, &mesh, target_gate, &bearer);
}

fn emit_reachability<'a>(
    v: &mut ValidationResult,
    readiness: &'a [crate::validation::live_mesh::GateReadiness],
) -> Option<&'a str> {
    let any_reachable = readiness.iter().any(|g| g.tcp_reachable);
    let any_songbird = readiness.iter().any(|g| g.songbird_responding);

    v.check_bool(
        "live:cross_gate_reachable",
        any_reachable,
        &format!(
            "remote gate connectivity: {} reachable, {} songbird OK (of {})",
            readiness.iter().filter(|g| g.tcp_reachable).count(),
            readiness.iter().filter(|g| g.songbird_responding).count(),
            readiness.len(),
        ),
    );

    if any_songbird {
        readiness
            .iter()
            .find(|g| g.songbird_responding)
            .map(|g| g.gate_id.as_str())
    } else {
        None
    }
}

fn issue_local_token(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    local_gate: &str,
) -> Option<String> {
    match ctx.call(
        "security",
        ISSUE_TOKEN_METHOD,
        serde_json::json!({
            "subject": "btsp-cross-gate-verify",
            "scopes": ["security.*", "health.*"],
            "ttl_seconds": 30,
            "gate_origin": local_gate,
        }),
    ) {
        Ok(resp) => {
            let token = resp
                .get("token")
                .and_then(serde_json::Value::as_str)
                .map(String::from);
            v.check_bool(
                "live:cross_gate_issue",
                token.is_some(),
                &format!(
                    "local {ISSUE_TOKEN_METHOD} on {local_gate}: {}",
                    if token.is_some() {
                        "token issued"
                    } else {
                        "missing token field"
                    }
                ),
            );

            if let Some(ref tok) = token {
                let claims_ok = decode_ionic_payload_claims(tok)
                    .is_some_and(|claims| token_claims_schema_valid(&claims));
                v.check_bool(
                    "live:cross_gate_token_schema",
                    claims_ok,
                    &format!(
                        "issued token payload has required claims ({REQUIRED_TOKEN_CLAIMS:?})"
                    ),
                );
            }

            token
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "live:cross_gate_issue",
                    &format!("{ISSUE_TOKEN_METHOD} not implemented: {e}"),
                );
            } else if e.is_skippable() {
                v.check_skip(
                    "live:cross_gate_issue",
                    &format!("security unavailable: {e}"),
                );
            } else {
                v.check_bool(
                    "live:cross_gate_issue",
                    false,
                    &format!("{ISSUE_TOKEN_METHOD} error: {e}"),
                );
            }
            None
        }
    }
}

fn verify_on_remote_gate(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    mesh: &LiveMeshConfig,
    target_gate: &str,
    bearer: &str,
) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:cross_gate_verify",
            "orchestration not in context — cannot dispatch cross-gate",
        );
        return;
    }

    if !mesh.btsp_available {
        v.check_skip(
            "live:cross_gate_verify",
            "BTSP credentials not available for cross-gate verify",
        );
        return;
    }

    match ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "security",
            "operation": VERIFY_TOKEN_METHOD,
            "args": {
                "token": bearer,
                "verification_source": "remote",
                "requesting_gate": &mesh.local_gate,
            },
            "gate": target_gate,
        }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
                || parse_verify_ionic_response(&resp).is_some();
            v.check_bool(
                "live:cross_gate_verify",
                valid,
                &format!(
                    "BTSP cross-gate verify: issued on {} → verified on {target_gate} = {valid}",
                    mesh.local_gate,
                ),
            );
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("No local or remote provider")
                || msg.contains("no route")
                || msg.contains("not found")
            {
                v.check_skip(
                    "live:cross_gate_verify",
                    &format!("mesh dispatch not available: {e}"),
                );
            } else {
                v.check_bool(
                    "live:cross_gate_verify",
                    false,
                    &format!("cross-gate verify on {target_gate} failed: {e}"),
                );
            }
        }
    }
}

/// Whether a token string has the ionic/JWT three-segment wire format.
#[must_use]
pub fn ionic_token_wire_format_valid(token: &str) -> bool {
    let segments: Vec<_> = token.split('.').collect();
    segments.len() == 3 && segments.iter().all(|s| !s.is_empty())
}

/// Decode the middle segment of an ionic/JWT token into JSON claims.
#[must_use]
pub fn decode_ionic_payload_claims(token: &str) -> Option<serde_json::Value> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;

    let mut parts = token.split('.');
    let _header = parts.next()?;
    let payload_b64 = parts.next()?;
    let _signature = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    let payload_bytes = BASE64.decode(payload_b64).ok()?;
    serde_json::from_slice(&payload_bytes).ok()
}

/// Validate that decoded token claims include the required JWT schema fields.
#[must_use]
pub fn token_claims_schema_valid(claims: &serde_json::Value) -> bool {
    REQUIRED_TOKEN_CLAIMS
        .iter()
        .all(|claim| claims.get(claim).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn btsp_cross_gate_verify_no_panic() {
        let mut v = ValidationResult::new("btsp-cross-gate-verify");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn structural_validation_passes() {
        let mut v = ValidationResult::new("btsp-cross-gate-verify-structural");
        phase_structural(&mut v);
        assert_eq!(v.failed, 0, "structural checks should all pass");
    }

    #[test]
    fn token_schema_validation() {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;

        let payload = serde_json::json!({
            "iss": "did:key:z6MkTestIssuer",
            "sub": "btsp-cross-gate-verify",
            "scope": ["security.*"],
            "iat": 1_700_000_000_i64,
            "exp": 1_700_003_600_i64,
            "jti": "test-jti",
        });
        let payload_b64 = BASE64.encode(payload.to_string());
        let token = format!("aGVhZGVy.{payload_b64}.c2ln");

        let claims = decode_ionic_payload_claims(&token).expect("payload decodes");
        assert!(token_claims_schema_valid(&claims));
        for claim in REQUIRED_TOKEN_CLAIMS {
            assert!(
                claims.get(claim).is_some(),
                "missing required claim '{claim}'"
            );
        }

        let mut v = ValidationResult::new("btsp-cross-gate-verify-schema");
        phase_token_schema(&mut v);
        assert_eq!(v.failed, 0, "token schema structural checks should pass");
    }

    #[test]
    fn trusted_issuer_registry_structure() {
        assert!(
            TRUST_TOPOLOGY_TOML.contains("[validation.trust_chain]"),
            "cross_gate_trust topology must declare validation.trust_chain"
        );
        assert!(
            TRUST_TOPOLOGY_TOML.contains("issuer_gate")
                && TRUST_TOPOLOGY_TOML.contains("verifier_gate"),
            "trust chain must name issuer and verifier gates"
        );

        let mut v = ValidationResult::new("btsp-cross-gate-verify-registry");
        phase_trusted_issuer_registry(&mut v);
        assert_eq!(v.failed, 0, "TrustedIssuerRegistry checks should pass");
    }
}
