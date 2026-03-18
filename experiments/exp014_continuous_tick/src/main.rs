// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp014: Continuous Tick — validates CoordinationPattern::Continuous and TICK_BUDGET_60HZ_US at 60Hz.

use primalspring::graphs::CoordinationPattern;
use primalspring::tolerances::TICK_BUDGET_60HZ_US;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp014 — Continuous Tick");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp014: Continuous at 60Hz (continuous_tick.toml)");
    println!("{}", "=".repeat(72));

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

    v.check_skip("actual_tick_loop", "actual tick loop needs live IPC");

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
