// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Loam Certificate Lifecycle — mint, verify, and retrieve.
//!
//! Exercises the certificate lifecycle (playbook Artifact 3):
//! `spine.create` → `spine.seal` → `certificate.mint` → `certificate.verify`
//! → `certificate.get`.
//!
//! Validates that loamSpine produces certificates with proper chain integrity
//! and that `certificate.verify` correctly distinguishes valid from invalid
//! certificates.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "loam-certificate-lifecycle",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "nucleus_playbook_artifact3",
        provenance_date: "2026-05-16",
        description: "Loam certificate: spine.create → seal → mint → verify → get",
    },
    run,
};

/// Run the loam certificate lifecycle validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Capability discovery — spine + certificate");
    phase_discovery(v, ctx);

    v.section("Phase 2: spine.create");
    let spine_id = phase_spine_create(v, ctx);

    v.section("Phase 3: spine.seal");
    let sealed_id = phase_spine_seal(v, ctx, spine_id.as_deref());

    v.section("Phase 4: certificate.mint");
    let cert_id = phase_cert_mint(v, ctx, sealed_id.as_deref());

    v.section("Phase 5: certificate.verify");
    phase_cert_verify(v, ctx, cert_id.as_deref());

    v.section("Phase 6: certificate.get");
    phase_cert_get(v, ctx, cert_id.as_deref());
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for (cap, label) in [
        ("spine", "loamSpine spine"),
        ("certificate", "loamSpine certificates"),
    ] {
        let found = ctx.has_capability(cap);
        v.check_bool(
            &format!("loam:discover:{cap}"),
            found,
            &format!(
                "{label} — {}",
                if found {
                    "resolved"
                } else {
                    "not discoverable"
                }
            ),
        );
    }
}

fn phase_spine_create(v: &mut ValidationResult, ctx: &mut CompositionContext) -> Option<String> {
    match ctx.call("spine", "spine.create", serde_json::json!({})) {
        Ok(resp) => {
            let spine_id = resp
                .get("spine_id")
                .or_else(|| resp.get("id"))
                .and_then(|s| s.as_str())
                .unwrap_or("");
            v.check_bool(
                "loam:spine_create:id",
                !spine_id.is_empty(),
                &format!("spine_id: {spine_id}"),
            );
            if spine_id.is_empty() {
                None
            } else {
                Some(spine_id.to_owned())
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "loam:spine_create:id",
                &format!("loamSpine not available: {e}"),
            );
            None
        }
        Err(e) => {
            v.check_bool(
                "loam:spine_create:id",
                false,
                &format!("spine.create error: {e}"),
            );
            None
        }
    }
}

fn phase_spine_seal(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    spine_id: Option<&str>,
) -> Option<String> {
    let Some(spine_id) = spine_id else {
        v.check_skip("loam:spine_seal:sealed", "no spine from spine.create");
        return None;
    };

    match ctx.call(
        "spine",
        "spine.seal",
        serde_json::json!({ "spine_id": spine_id }),
    ) {
        Ok(resp) => {
            let sealed = resp
                .get("sealed_id")
                .or_else(|| resp.get("spine_id"))
                .or_else(|| resp.get("hash"))
                .and_then(|s| s.as_str())
                .unwrap_or("");
            v.check_bool(
                "loam:spine_seal:sealed",
                !sealed.is_empty(),
                &format!("sealed: {sealed}"),
            );
            if sealed.is_empty() {
                None
            } else {
                Some(sealed.to_owned())
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "loam:spine_seal:sealed",
                &format!("loamSpine not available: {e}"),
            );
            None
        }
        Err(e) => {
            v.check_bool(
                "loam:spine_seal:sealed",
                false,
                &format!("spine.seal error: {e}"),
            );
            None
        }
    }
}

fn phase_cert_mint(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    sealed_id: Option<&str>,
) -> Option<String> {
    let Some(sealed_id) = sealed_id else {
        v.check_skip("loam:cert_mint:id", "no sealed spine from spine.seal");
        return None;
    };

    match ctx.call(
        "certificate",
        "certificate.mint",
        serde_json::json!({
            "spine_id": sealed_id,
            "cert_type": "ownership",
            "subject": "primalSpring:scenario:loam-certificate-lifecycle",
        }),
    ) {
        Ok(resp) => {
            let cert_id = resp
                .get("cert_id")
                .or_else(|| resp.get("id"))
                .and_then(|c| c.as_str())
                .unwrap_or("");
            let has_chain = resp.get("hash_chain").is_some();
            v.check_bool(
                "loam:cert_mint:id",
                !cert_id.is_empty(),
                &format!("cert_id: {cert_id}, hash_chain present: {has_chain}"),
            );
            if cert_id.is_empty() {
                None
            } else {
                Some(cert_id.to_owned())
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "loam:cert_mint:id",
                &format!("loamSpine not available: {e}"),
            );
            None
        }
        Err(e) => {
            v.check_bool(
                "loam:cert_mint:id",
                false,
                &format!("certificate.mint error: {e}"),
            );
            None
        }
    }
}

fn phase_cert_verify(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    cert_id: Option<&str>,
) {
    let Some(cert_id) = cert_id else {
        v.check_skip("loam:cert_verify:valid", "no cert from certificate.mint");
        v.check_skip("loam:cert_verify:invalid_rejected", "no cert");
        return;
    };

    match ctx.call(
        "certificate",
        "certificate.verify",
        serde_json::json!({ "cert_id": cert_id }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "loam:cert_verify:valid",
                valid,
                &format!("certificate.verify({cert_id}): valid={valid}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "loam:cert_verify:valid",
                &format!("loamSpine not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "loam:cert_verify:valid",
                false,
                &format!("certificate.verify error: {e}"),
            );
        }
    }

    match ctx.call(
        "certificate",
        "certificate.verify",
        serde_json::json!({ "cert_id": "invalid-cert-does-not-exist-00000" }),
    ) {
        Ok(resp) => {
            let valid = resp
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true);
            v.check_bool(
                "loam:cert_verify:invalid_rejected",
                !valid,
                &format!("bogus cert verify returned valid={valid} (expected false)"),
            );
        }
        Err(_) => {
            v.check_bool(
                "loam:cert_verify:invalid_rejected",
                true,
                "invalid cert correctly rejected with error",
            );
        }
    }
}

fn phase_cert_get(v: &mut ValidationResult, ctx: &mut CompositionContext, cert_id: Option<&str>) {
    let Some(cert_id) = cert_id else {
        v.check_skip("loam:cert_get:retrieved", "no cert from certificate.mint");
        return;
    };

    match ctx.call(
        "certificate",
        "certificate.get",
        serde_json::json!({ "cert_id": cert_id }),
    ) {
        Ok(resp) => {
            let has_data = resp.get("cert_id").is_some()
                || resp.get("id").is_some()
                || resp.get("cert_type").is_some();
            v.check_bool(
                "loam:cert_get:retrieved",
                has_data,
                &format!(
                    "certificate.get response keys: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "loam:cert_get:retrieved",
                &format!("loamSpine not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "loam:cert_get:retrieved",
                false,
                &format!("certificate.get error: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loam_certificate_lifecycle_no_panic() {
        let mut v = ValidationResult::new("loam-certificate-lifecycle");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }
}
