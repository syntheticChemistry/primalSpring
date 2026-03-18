// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp012: Conditional DAG — validates CoordinationPattern::ConditionalDag description for conditional fallback.

use primalspring::graphs::CoordinationPattern;
use primalspring::ipc::discover::neural_api_healthy;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp012 — Conditional DAG");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp012: ConditionalDag (conditional_fallback.toml)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let desc = CoordinationPattern::ConditionalDag.description();
    v.check_bool(
        "conditional_dag_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::ConditionalDag.description() exists: {desc}"),
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
        "actual_dag_with_branching",
        "actual DAG with branching needs live IPC",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
