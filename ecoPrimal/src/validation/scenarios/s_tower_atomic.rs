// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Tower Atomic — absorbed from exp001.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-atomic",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "exp001_tower_atomic",
        provenance_date: "2026-05-09",
        description: "Tower Atomic — security + discovery capability validation",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural");
    phase_structural(v);

    v.section("Phase 2: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 3: Health");
    phase_health(v, ctx);

    v.section("Phase 4: Composition");
    phase_composition(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    v.check_count("tower_required_caps", tower_caps.len(), 2);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_found_primals",
        !caps.is_empty(),
        &format!("{} context capabilities: {}", caps.len(), caps.join(", ")),
    );
    for cap in tower_caps {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    for cap in tower_caps {
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_skippable() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} health check error: {e}"),
            ),
        }
    }
}

fn phase_composition(v: &mut ValidationResult, ctx: &CompositionContext) {
    let btsp = ctx.btsp_state();
    let btsp_count = btsp.values().filter(|&&ok| ok).count();
    v.check_bool(
        "btsp_any_authenticated",
        btsp_count > 0 || btsp.is_empty(),
        &format!("{btsp_count}/{} BTSP authenticated", btsp.len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_atomic_no_panic() {
        let mut v = ValidationResult::new("tower-atomic");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }
}
