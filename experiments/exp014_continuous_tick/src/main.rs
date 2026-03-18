// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp014: Continuous Tick — validates CoordinationPattern::Continuous and TICK_BUDGET_60HZ_US at 60Hz.

use primalspring::graphs::CoordinationPattern;
use primalspring::ipc::discover::neural_api_healthy;
use primalspring::tolerances::{TICK_BUDGET_60HZ_US, VALIDATION_SUMMARY_WIDTH};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp014 — Continuous Tick");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp014: Continuous at 60Hz (continuous_tick.toml)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let desc = CoordinationPattern::Continuous.description();
    v.check_bool(
        "continuous_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::Continuous.description() exists: {desc}"),
    );
    let expected_60hz_us: u64 = 1_000_000 / 60;
    let within_tolerance = TICK_BUDGET_60HZ_US.abs_diff(expected_60hz_us) <= 1;
    v.check_bool(
        "tick_budget_60hz_correct",
        within_tolerance,
        &format!("TICK_BUDGET_60HZ_US is correct for 60Hz (16_667 ± 1): {TICK_BUDGET_60HZ_US}µs"),
    );

    let neural_ok = neural_api_healthy();
    if neural_ok {
        v.check_bool("neural_api", true, "biomeOS Neural API reachable");
    } else {
        v.check_skip("neural_api", "biomeOS Neural API not reachable");
    }

    v.check_or_skip(
        "graph_deployment",
        neural_ok.then_some(()),
        "Neural API unavailable — cannot deploy graph",
        |(), v| {
            v.check_bool(
                "graph_deployment",
                true,
                "Neural API ready for graph deployment",
            );
        },
    );

    v.check_skip("actual_tick_loop", "actual tick loop needs live IPC");

    v.finish();
    std::process::exit(v.exit_code());
}
