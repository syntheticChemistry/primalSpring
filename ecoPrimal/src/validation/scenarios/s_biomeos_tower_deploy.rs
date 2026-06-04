// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: biomeOS Tower Deploy — absorbed from exp060.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "biomeos-tower-deploy",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "exp060_biomeos_tower_deploy",
        provenance_date: "2026-05-09",
        description: "biomeOS Tower deploy — live Tower composition via CompositionContext",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Security & discovery liveness");
    phase_discovery_health(v, ctx);

    v.section("Phase 2: Crypto routing");
    phase_crypto_routing(v, ctx);
}

fn phase_discovery_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in ["security", "discovery"] {
        if !ctx.has_capability(cap) {
            v.check_skip(cap, &format!("{cap} capability not discovered"));
            continue;
        }
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("{cap}_liveness"),
                true,
                &format!("{cap} health.liveness"),
            ),
            Ok(false) => v.check_bool(
                &format!("{cap}_liveness"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("{cap}_liveness"), &format!("{e}"));
            }
            Err(e) => v.check_bool(&format!("{cap}_liveness"), false, &format!("error: {e}")),
        }
    }
}

fn phase_crypto_routing(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "capability_call_crypto",
            "security capability not discovered",
        );
        return;
    }
    match ctx.call("security", "generate_keypair", serde_json::json!({})) {
        Ok(r) => v.check_bool(
            "capability_call_crypto",
            !r.is_null(),
            "security.generate_keypair returned data",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip("capability_call_crypto", &format!("{e}"));
        }
        Err(e) => {
            let msg = format!("{e}");
            let expected = msg.contains("not found")
                || msg.contains("not registered")
                || e.is_method_not_found();
            v.check_bool(
                "capability_call_crypto",
                expected,
                &format!("capability routing attempt: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biomeos_tower_deploy_no_panic() {
        let mut v = ValidationResult::new("biomeos-tower-deploy");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
