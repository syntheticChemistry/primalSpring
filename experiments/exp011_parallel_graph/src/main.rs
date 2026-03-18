// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp011: Parallel Graph — validates CoordinationPattern::Parallel description for parallel capability burst.

use primalspring::graphs::CoordinationPattern;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp011 — Parallel Graph");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp011: Parallel (parallel_capability_burst.toml)");
    println!("{}", "=".repeat(72));

    let desc = CoordinationPattern::Parallel.description();
    v.check_bool(
        "parallel_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::Parallel.description() exists: {desc}"),
    );

    v.check_skip(
        "actual_parallel_execution",
        "actual parallel execution needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
