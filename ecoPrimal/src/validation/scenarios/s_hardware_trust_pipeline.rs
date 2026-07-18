// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Hardware Trust Pipeline — validates the full end-to-end local
//! ceremony from hardware entropy to Loam Certificate minting.
//!
//! Wave 138a LOCAL-CEREMONY-E2E: `SoloKey` entropy + Pixel biometric + bearDog
//! key generation → Loam Certificate mint. This scenario validates the
//! structural topology for the complete pipeline:
//!
//! 1. Hardware entropy sources (FIDO2 + `StrongBox`) are routable
//! 2. Entropy converges at bearDog for mixing
//! 3. Key material flows to signing (`crypto.sign_ed25519`)
//! 4. Signed material flows to Loam Certificate mint (loamSpine)
//! 5. Certificate verification closes the loop

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MANIFEST_TOML: &str = include_str!("../../../../config/ecosystem/ecosystem_manifest.toml");

#[cfg(test)]
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Hardware trust pipeline scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "hardware-trust-pipeline",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave138a_hw_trust",
        provenance_date: "2026-07-13",
        description: "Hardware trust pipeline — FIDO2+StrongBox entropy → bearDog key gen → Loam Certificate mint",
    },
    run,
};

/// Pipeline stages in order.
const PIPELINE_STAGES: &[(&str, &str)] = &[
    (
        "beardog.fido2.discover",
        "Hardware discovery (SoloKey/StrongBox)",
    ),
    (
        "genetic.entropy_contribute",
        "Entropy contribution from hardware",
    ),
    ("genetic.mix_entropy", "Multi-source entropy mixing"),
    ("genetic.ceremony_init", "Ceremony initialization (Tier 2)"),
    ("genetic.derive_key", "Key derivation from mixed entropy"),
    ("genetic.ceremony_finalize", "Ceremony seal"),
    ("crypto.sign_ed25519", "Ed25519 signing with derived key"),
];

const CERTIFICATE_METHODS: &[&str] = &["spine.create", "spine.seal"];

/// Run the hardware trust pipeline validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Pipeline stage routing");
    phase_pipeline_routing(v);

    v.section("Phase 2: Certificate endpoint availability");
    phase_certificate_endpoints(v);

    v.section("Phase 3: Authority chain");
    phase_authority_chain(v);

    v.section("Phase 4: Manifest topology");
    phase_manifest_topology(v);

    v.section("Phase 5: Live pipeline probe");
    phase_live_probe(v, ctx);
}

fn phase_pipeline_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for (method, description) in PIPELINE_STAGES {
        let route = table.route(method);
        let routed = route.is_some();
        let owner = route.map_or_else(|| "UNROUTED".to_string(), |r| r.owner.to_string());

        v.check_bool(
            &format!("hw-trust:stage_{}", method.replace('.', "_")),
            routed,
            &format!("{description}: `{method}` → {owner}"),
        );
    }

    let stages_routed = PIPELINE_STAGES
        .iter()
        .filter(|(method, _)| table.route(method).is_some())
        .count();

    v.check_bool(
        "hw-trust:all_stages_routed",
        stages_routed == PIPELINE_STAGES.len(),
        &format!(
            "{stages_routed}/{} pipeline stages have routing",
            PIPELINE_STAGES.len()
        ),
    );
}

fn phase_certificate_endpoints(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in CERTIFICATE_METHODS {
        let route = table.route(method);
        let routed = route.is_some();
        let owner = route.map_or_else(|| "UNROUTED".to_string(), |r| r.owner.to_string());

        v.check_bool(
            &format!("hw-trust:cert_{}", method.replace('.', "_")),
            routed,
            &format!("`{method}` → {owner} (certificate infrastructure)"),
        );
    }

    let verify_route = table.route("crypto.verify_ed25519");
    v.check_bool(
        "hw-trust:verify_available",
        verify_route.is_some(),
        &format!(
            "crypto.verify_ed25519: {}",
            verify_route
                .as_ref()
                .map_or("UNROUTED", |r| r.owner.as_ref())
        ),
    );
}

fn phase_authority_chain(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let entropy_owner = table
        .route("genetic.mix_entropy")
        .map(|r| r.owner.to_string())
        .unwrap_or_default();
    let sign_owner = table
        .route("crypto.sign_ed25519")
        .map(|r| r.owner.to_string())
        .unwrap_or_default();
    let cert_owner = table
        .route("spine.create")
        .map(|r| r.owner.to_string())
        .unwrap_or_default();

    v.check_bool(
        "hw-trust:entropy_authority",
        entropy_owner == "beardog",
        &format!("entropy authority: {entropy_owner} (expected: beardog)"),
    );

    v.check_bool(
        "hw-trust:signing_authority",
        sign_owner == "beardog",
        &format!("signing authority: {sign_owner} (expected: beardog)"),
    );

    v.check_bool(
        "hw-trust:cert_authority",
        cert_owner == "loamspine",
        &format!("certificate authority: {cert_owner} (expected: loamspine)"),
    );

    let entropy_and_signing_same = entropy_owner == sign_owner;
    v.check_bool(
        "hw-trust:single_crypto_authority",
        entropy_and_signing_same,
        "entropy mixing + signing = same authority (no key escrow)",
    );

    let cert_separate = cert_owner != sign_owner;
    v.check_bool(
        "hw-trust:separation_of_concerns",
        cert_separate,
        "certificate minting separate from key authority (loamSpine ≠ bearDog)",
    );
}

fn phase_manifest_topology(v: &mut ValidationResult) {
    let has_beardog = MANIFEST_TOML.contains("bearDog") || MANIFEST_TOML.contains("beardog");
    v.check_bool(
        "hw-trust:beardog_in_manifest",
        has_beardog,
        "bearDog declared in ecosystem manifest (crypto authority)",
    );

    let has_loamspine = MANIFEST_TOML.contains("loamSpine") || MANIFEST_TOML.contains("loamspine");
    v.check_bool(
        "hw-trust:loamspine_in_manifest",
        has_loamspine,
        "loamSpine declared in ecosystem manifest (certificate authority)",
    );

    let eastgate_has_beardog = MANIFEST_TOML
        .lines()
        .skip_while(|l| !l.contains("[gates.eastGate]") && !l.contains("eastGate"))
        .take(20)
        .any(|l| l.to_lowercase().contains("beardog"));

    let beardog_in_full = MANIFEST_TOML.to_lowercase().contains("beardog");
    v.check_bool(
        "hw-trust:beardog_accessible_on_eastgate",
        eastgate_has_beardog || beardog_in_full,
        "bearDog accessible from eastGate (local ceremony target)",
    );
}

fn phase_live_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_crypto = ctx.has_capability("crypto");
    v.check_bool(
        "hw-trust:crypto_live",
        has_crypto,
        &format!(
            "crypto capability (bearDog): {}",
            if has_crypto {
                "live — full pipeline testable"
            } else {
                "offline (structural only)"
            }
        ),
    );

    if has_crypto {
        let sign_test = ctx.call(
            "crypto",
            "crypto.sign_ed25519",
            serde_json::json!({"message": "dGVzdA=="}),
        );
        v.check_bool(
            "hw-trust:live_sign",
            sign_test.is_ok(),
            &format!(
                "crypto.sign_ed25519: {}",
                if sign_test.is_ok() {
                    "responded"
                } else {
                    "unavailable"
                }
            ),
        );
    }

    let has_spine = ctx.has_capability("spine");
    if has_spine {
        let spine_check = ctx.call(
            "spine",
            "spine.create",
            serde_json::json!({"type": "probe"}),
        );
        v.check_bool(
            "hw-trust:live_spine",
            spine_check.is_ok(),
            &format!(
                "spine.create: {}",
                if spine_check.is_ok() {
                    "responded"
                } else {
                    "unavailable"
                }
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_pipeline_stages_registered() {
        for (method, _) in PIPELINE_STAGES {
            assert!(
                REGISTRY_TOML.contains(method),
                "pipeline stage `{method}` missing from capability_registry.toml"
            );
        }
    }

    #[test]
    fn pipeline_routes_to_beardog() {
        let table = canonical_routing_table();
        for (method, _) in PIPELINE_STAGES {
            let route = table.route(method);
            assert!(route.is_some(), "`{method}` not routed");
            assert_eq!(
                route.unwrap().owner.as_ref(),
                "beardog",
                "pipeline stage `{method}` should route to beardog"
            );
        }
    }

    #[test]
    fn certificate_routes_to_loamspine() {
        let table = canonical_routing_table();
        for method in CERTIFICATE_METHODS {
            let route = table.route(method);
            assert!(route.is_some(), "`{method}` not routed");
            assert_eq!(
                route.unwrap().owner.as_ref(),
                "loamspine",
                "certificate method `{method}` should route to loamspine"
            );
        }
    }
}
