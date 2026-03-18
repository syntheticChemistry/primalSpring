// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp010: Sequential Graph — validates CoordinationPattern::Sequential description for rootpulse_commit.toml.

use primalspring::graphs::CoordinationPattern;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp010 — Sequential Graph");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp010: Sequential (rootpulse_commit.toml)");
    println!("{}", "=".repeat(72));

    let desc = CoordinationPattern::Sequential.description();
    v.check_bool(
        "sequential_description_non_empty",
        !desc.is_empty(),
        &format!("CoordinationPattern::Sequential.description() is non-empty: {desc}"),
    );
    let expected = "Nodes in dependency order (A -> B -> C)";
    v.check_bool(
        "sequential_description_matches",
        desc == expected,
        &format!("sequential pattern matches expected: {expected}"),
    );

    v.check_skip(
        "actual_graph_execution",
        "actual graph execution with rootpulse_commit.toml needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
