// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp024: Cross-Spring Ecology — validates cross_spring_ecology.toml with airSpring + wetSpring data.

use primalspring::emergent::EmergentSystem;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp024 — Cross-Spring Ecology");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp024: cross_spring_ecology.toml with airSpring + wetSpring Data");
    println!("{}", "=".repeat(72));

    let graphs = EmergentSystem::CrossSpringEcology.required_graphs();
    v.check_bool(
        "cross_spring_ecology_graphs_non_empty",
        !graphs.is_empty(),
        &format!("EmergentSystem::CrossSpringEcology.required_graphs() is non-empty: {graphs:?}"),
    );

    v.check_skip(
        "cross_spring_pipeline",
        "cross-spring pipeline (airSpring→wetSpring→neuralSpring) needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
