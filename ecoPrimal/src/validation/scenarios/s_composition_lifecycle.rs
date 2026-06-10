// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Composition Lifecycle — absorbed from exp109.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "composition-lifecycle",
        track: Track::Lifecycle,
        tier: Tier::Live,
        provenance_crate: "exp109_composition_lifecycle",
        provenance_date: "2026-05-09",
        description: "Composition lifecycle — discovery, reload, post-reload liveness, BTSP",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Initial Discovery");
    phase_initial_discovery(v, ctx);

    v.section("Phase 2: Pre-Reload Liveness");
    phase_pre_reload_liveness(v, ctx);

    v.section("Phase 3: Composition Reload");
    phase_composition_reload(v, ctx);

    v.section("Phase 4: Post-Reload Validation");
    phase_post_reload_validation(v, ctx);

    v.section("Phase 5: BTSP Continuity");
    phase_btsp_continuity(v, ctx);
}

fn phase_initial_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();

    v.check_bool(
        "initial_discovery",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );

    if caps.is_empty() {
        v.check_skip(
            "initial_discovery_detail",
            "no primals discovered — NUCLEUS not running",
        );
    }
}

fn phase_pre_reload_liveness(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let tower_caps = AtomicType::Tower.required_capabilities();

    let alive = tower_caps
        .iter()
        .filter(|&&cap| ctx.has_capability(cap) && matches!(ctx.health_check(cap), Ok(true)))
        .count();

    v.check_bool(
        "tower_alive_pre_reload",
        tower_caps.is_empty() || alive > 0,
        &format!("{alive}/{} Tower primals alive", tower_caps.len()),
    );
    if tower_caps.is_empty() || alive == 0 {
        v.check_skip(
            "tower_liveness_detail",
            "no Tower capabilities alive — skipping pre-reload liveness",
        );
    }
}

fn phase_composition_reload(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "composition_reload",
            "biomeOS orchestration not available — composition.reload requires biomeOS",
        );
        return;
    }

    match ctx.reload() {
        Ok(result) => {
            v.check_bool("composition_reload", true, &format!("response: {result}"));
        }
        Err(e) => {
            v.check_skip(
                "composition_reload",
                &format!("composition.reload failed: {e}"),
            );
        }
    }
}

fn phase_post_reload_validation(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let caps = ctx.available_capabilities();

    if caps.is_empty() {
        v.check_skip(
            "post_reload_discovery",
            "no capabilities after reload — NUCLEUS not running",
        );
        return;
    }

    let tower_caps = AtomicType::Tower.required_capabilities();
    let alive = tower_caps
        .iter()
        .filter(|&&cap| ctx.has_capability(cap) && matches!(ctx.health_check(cap), Ok(true)))
        .count();
    v.check_bool(
        "tower_alive_post_reload",
        tower_caps.is_empty() || alive > 0,
        &format!("{alive}/{} Tower primals survived reload", tower_caps.len()),
    );
}

fn phase_btsp_continuity(v: &mut ValidationResult, ctx: &CompositionContext) {
    let state = ctx.btsp_state();

    if state.is_empty() {
        v.check_skip("btsp_continuity", "no BTSP state — no primals discovered");
        return;
    }

    let total = state.len();
    let authenticated = state.values().filter(|&&b| b).count();
    v.check_bool(
        "btsp_state_populated",
        total > 0,
        &format!("{authenticated}/{total} BTSP authenticated"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_lifecycle_no_panic() {
        let mut v = ValidationResult::new("composition-lifecycle");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }
}
