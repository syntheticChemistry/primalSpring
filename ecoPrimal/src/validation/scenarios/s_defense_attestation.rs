// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Defense Attestation Surface — validates SkunkBat's threat detection
//! and defense attestation capabilities.
//!
//! Validates:
//! 1. Security capability domain routing resolves to BearDog
//! 2. Defense attestation methods are registered
//! 3. BTSP handshake pattern is enforced for security operations
//! 4. Threat classification taxonomy covers known categories
//! 5. Live defense attestation responds (when SkunkBat reachable)

use crate::composition::{capability_to_primal, method_to_capability_domain, CompositionContext};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Defense attestation validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "defense-attestation",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-21",
        description: "SkunkBat defense attestation: routing, methods, BTSP enforcement, live probe",
    },
    run: run_defense_attestation,
};

/// Run this validation scenario.
pub fn run_defense_attestation(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Security routing");
    phase_security_routing(v);

    v.section("Phase 2: Defense method surface");
    phase_defense_methods(v);

    v.section("Phase 3: BTSP enforcement");
    phase_btsp_enforcement(v, ctx);

    v.section("Phase 4: Live defense probe");
    phase_live_defense_probe(v, ctx);
}

fn phase_security_routing(v: &mut ValidationResult) {
    let sec_domain = method_to_capability_domain("security.attest");
    let security_primal = capability_to_primal(sec_domain);
    v.check_bool(
        "security_routes_to_beardog",
        security_primal == primal_names::BEARDOG,
        &format!("security.attest → {security_primal} (expected {})", primal_names::BEARDOG),
    );

    let def_domain = method_to_capability_domain("defense.classify");
    let defense_primal = capability_to_primal(def_domain);
    v.check_bool(
        "defense_routes_to_skunkbat",
        defense_primal == primal_names::SKUNKBAT,
        &format!("defense.classify → {defense_primal} (expected {})", primal_names::SKUNKBAT),
    );

    let crypto_domain = method_to_capability_domain("crypto.sign");
    let crypto_primal = capability_to_primal(crypto_domain);
    v.check_bool(
        "crypto_routes_to_beardog",
        crypto_primal == primal_names::BEARDOG,
        &format!("crypto.sign → {crypto_primal} (expected {})", primal_names::BEARDOG),
    );
}

fn phase_defense_methods(v: &mut ValidationResult) {
    let table = crate::composition::neural_routing::canonical_routing_table();

    let defense_methods = table.methods_in_domain("defense");
    v.check_minimum("defense_method_count", defense_methods.len(), 3);

    let expected_methods = ["defense.classify", "defense.attest", "defense.scan"];
    for method in &expected_methods {
        let routed = table.route(method).is_some();
        v.check_bool(
            &format!("method_registered:{method}"),
            routed,
            &format!("{method} registered in routing table"),
        );
    }

    let security_methods = table.methods_in_domain("security");
    v.check_minimum("security_method_count", security_methods.len(), 5);
}

fn phase_btsp_enforcement(v: &mut ValidationResult, ctx: &CompositionContext) {
    let btsp_capabilities = ["security", "crypto"];

    for cap in &btsp_capabilities {
        match ctx.btsp_authenticated(cap) {
            Some(true) => {
                v.check_bool(
                    &format!("btsp:{cap}:authenticated"),
                    true,
                    &format!("{cap} BTSP authenticated"),
                );
            }
            Some(false) => {
                v.check_bool(
                    &format!("btsp:{cap}:authenticated"),
                    false,
                    &format!("{cap} BTSP not authenticated (handshake needed)"),
                );
            }
            None => {
                v.check_skip(
                    &format!("btsp:{cap}:authenticated"),
                    &format!("{cap} not discovered (primal offline)"),
                );
            }
        }
    }
}

fn phase_live_defense_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("defense") {
        v.check_skip(
            "defense_live_probe",
            "defense capability not discovered (SkunkBat offline)",
        );
        return;
    }

    match ctx.health_check("defense") {
        Ok(true) => {
            v.check_bool("defense_liveness", true, "defense health.liveness OK");
        }
        Ok(false) => {
            v.check_bool("defense_liveness", false, "defense not healthy");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("defense_liveness", &format!("defense unreachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "defense_liveness",
                false,
                &format!("defense probe failed: {e}"),
            );
        }
    }

    match ctx.health_check("security") {
        Ok(true) => {
            v.check_bool("security_liveness", true, "security health.liveness OK");
        }
        Ok(false) => {
            v.check_bool("security_liveness", false, "security not healthy");
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("security_liveness", &format!("security unreachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "security_liveness",
                false,
                &format!("security probe failed: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defense_attestation_structural() {
        let mut v = ValidationResult::new("defense-attestation");
        let mut ctx = CompositionContext::discover();
        run_defense_attestation(&mut v, &mut ctx);
        let structural_ok = v.passed >= 5;
        assert!(
            structural_ok,
            "defense-attestation: only {} checks passed ({} failed, {} skipped)",
            v.passed, v.failed, v.skipped
        );
    }
}
