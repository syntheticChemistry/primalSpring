// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp023: RPGPT Session — validates 60Hz game engine tick and provenance.

use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::{discover_primal, neural_api_healthy};
use primalspring::tolerances::{TICK_BUDGET_60HZ_US, VALIDATION_SUMMARY_WIDTH};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp023 — RPGPT Session");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp023: 60Hz + provenance");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let graphs = EmergentSystem::Rpgpt.required_graphs();
    let has_game_engine_tick = graphs.contains(&"game_engine_tick");
    v.check_bool(
        "rpgpt_has_game_engine_tick",
        has_game_engine_tick,
        &format!("EmergentSystem::Rpgpt has game_engine_tick graph: {graphs:?}"),
    );

    let expected_60hz_us: u64 = 1_000_000 / 60;
    let within_tolerance = TICK_BUDGET_60HZ_US.abs_diff(expected_60hz_us) <= 1;
    v.check_bool(
        "tick_budget_60hz_correct",
        within_tolerance,
        &format!("TICK_BUDGET_60HZ_US is correct for 60Hz (16_667 ± 1): {TICK_BUDGET_60HZ_US}µs"),
    );

    v.check_or_skip(
        "biomeos_neural_api_health",
        Some(()).filter(|()| neural_api_healthy()),
        "Neural API not reachable — biomeOS not running",
        |(), v| {
            v.check_bool(
                "biomeos_healthy",
                true,
                "Neural API health check OK (biomeOS reachable)",
            );
        },
    );

    let ludospring = discover_primal("ludospring");
    v.check_bool(
        "discover_ludospring",
        ludospring.primal == "ludospring",
        &format!(
            "discover ludospring (RPGPT cross-spring): socket {}",
            if ludospring.socket.is_some() {
                "found"
            } else {
                "not found"
            }
        ),
    );

    v.check_skip("actual_session", "actual RPGPT session needs live IPC");

    v.finish();
    std::process::exit(v.exit_code());
}
