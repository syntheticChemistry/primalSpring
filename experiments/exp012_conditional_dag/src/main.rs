// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp012: Conditional DAG — validates CoordinationPattern::ConditionalDag description for conditional fallback.

use primalspring::graphs::CoordinationPattern;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp012 — Conditional DAG");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp012: ConditionalDag (conditional_fallback.toml)");
    println!("{}", "=".repeat(72));

    let desc = CoordinationPattern::ConditionalDag.description();
    v.check_bool(
        "conditional_dag_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::ConditionalDag.description() exists: {desc}"),
    );

    v.check_skip(
        "actual_dag_with_branching",
        "actual DAG with branching needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
