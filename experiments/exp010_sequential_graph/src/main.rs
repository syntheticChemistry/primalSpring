// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp010: Sequential Graph — validates CoordinationPattern::Sequential description for rootpulse_commit.toml.

use primalspring::graphs::CoordinationPattern;
use primalspring::ipc::discover::neural_api_healthy;
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

    v.check_skip(
        "actual_graph_execution",
        "actual graph execution with rootpulse_commit.toml needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
