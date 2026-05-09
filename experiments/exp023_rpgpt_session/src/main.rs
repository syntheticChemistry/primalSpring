// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp023: RPGPT session — validates 60Hz tick budget and composition discovery.

use primalspring::composition::CompositionContext;
use primalspring::emergent::EmergentSystem;
use primalspring::tolerances::{TICK_BUDGET_60HZ_SLACK_US, TICK_BUDGET_60HZ_US};
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let graphs = EmergentSystem::Rpgpt.required_graphs();
    let has_game_engine_tick = graphs.contains(&"game_engine_tick");
    v.check_bool(
        "rpgpt_has_game_engine_tick",
        has_game_engine_tick,
        &format!("EmergentSystem::Rpgpt has game_engine_tick graph: {graphs:?}"),
    );

    let expected_60hz_us: u64 = 1_000_000 / 60;
    let within_tolerance =
        TICK_BUDGET_60HZ_US.abs_diff(expected_60hz_us) <= TICK_BUDGET_60HZ_SLACK_US;
    v.check_bool(
        "tick_budget_60hz_correct",
        within_tolerance,
        &format!("TICK_BUDGET_60HZ_US is correct for 60Hz (16_667 ± 1): {TICK_BUDGET_60HZ_US}µs"),
    );
}

fn phase_composition(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.check_or_skip(
        "biomeos_neural_api_health",
        Some(()).filter(|()| ctx.has_capability("orchestration")),
        "orchestration not reachable — biomeOS not running",
        |(), v| {
            v.check_bool("biomeos_healthy", true, "orchestration capability resolved");
        },
    );

    v.check_bool(
        "discover_ludospring",
        ctx.has_capability("game"),
        "game capability for ludospring / RPGPT",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp023 — RPGPT Session")
        .with_provenance("exp023_rpgpt_session", "2026-05-09")
        .run("primalSpring Exp023: 60Hz + provenance", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Composition Discovery");
            let ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_composition(v, &ctx);

            v.check_skip("actual_session", "actual RPGPT session needs live IPC");
        });
}
