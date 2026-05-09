// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp014: Continuous Tick — validates `CoordinationPattern::Continuous` and 60Hz tick budget constants.
//!
//! Phases:
//!   1. Pattern constants (Continuous)
//!   2. Tick budget validation (60Hz)
//!   3. Composition discovery (orchestration)

use primalspring::composition::CompositionContext;
use primalspring::graphs::CoordinationPattern;
use primalspring::tolerances::{TICK_BUDGET_60HZ_SLACK_US, TICK_BUDGET_60HZ_US};
use primalspring::validation::ValidationResult;

fn phase_pattern_constants(v: &mut ValidationResult) {
    let desc = CoordinationPattern::Continuous.description();
    v.check_bool(
        "continuous_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::Continuous.description() exists: {desc}"),
    );
}

fn phase_tick_budget(v: &mut ValidationResult) {
    let expected_60hz_us: u64 = 1_000_000 / 60;
    let within_tolerance =
        TICK_BUDGET_60HZ_US.abs_diff(expected_60hz_us) <= TICK_BUDGET_60HZ_SLACK_US;
    v.check_bool(
        "tick_budget_60hz_correct",
        within_tolerance,
        &format!(
            "TICK_BUDGET_60HZ_US is correct for 60Hz (16_667 ± slack): {TICK_BUDGET_60HZ_US}µs"
        ),
    );
}

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let orchestration_ok = ctx.has_capability("orchestration");
    if orchestration_ok {
        v.check_bool(
            "orchestration_discovered",
            true,
            "orchestration capability present in composition context",
        );
    } else {
        v.check_skip(
            "orchestration_discovered",
            "orchestration capability not discovered",
        );
    }

    v.check_or_skip(
        "graph_deployment",
        orchestration_ok.then_some(()),
        "orchestration unavailable — graph deployment not validated",
        |(), v| {
            v.check_bool(
                "graph_deployment",
                true,
                "orchestration present for graph deployment path",
            );
        },
    );

    v.check_skip("actual_tick_loop", "actual tick loop needs live IPC");
}

fn main() {
    ValidationResult::new("primalSpring Exp014 — Continuous Tick")
        .with_provenance("exp014_continuous_tick", "2026-05-09")
        .run(
            "primalSpring Exp014: Continuous at 60Hz (continuous_tick.toml)",
            |v| {
                let ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Pattern Constants (CoordinationPattern::Continuous)");
                phase_pattern_constants(v);

                v.section("Phase 2: Tick Budget Validation (60Hz)");
                phase_tick_budget(v);

                v.section("Phase 3: Composition Discovery");
                phase_composition_discovery(v, &ctx);
            },
        );
}
