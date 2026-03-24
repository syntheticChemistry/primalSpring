// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp024: Cross-Spring Ecology — validates cross_spring_ecology.toml with airSpring + wetSpring data.

use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp024 — Cross-Spring Ecology")
        .with_provenance("exp024_cross_spring_ecology", "2026-03-24")
        .run(
            "primalSpring Exp024: cross_spring_ecology.toml with airSpring + wetSpring Data",
            |v| {
                let graphs = EmergentSystem::CrossSpringEcology.required_graphs();
                v.check_bool(
                    "cross_spring_ecology_graphs_non_empty",
                    !graphs.is_empty(),
                    &format!("EmergentSystem::CrossSpringEcology.required_graphs() is non-empty: {graphs:?}"),
                );

                let airspring = discover_primal("airspring");
                v.check_bool(
                    "discover_airspring",
                    airspring.primal == "airspring",
                    &format!(
                        "discover airspring (cross-spring ecology): socket {}",
                        if airspring.socket.is_some() {
                            "found"
                        } else {
                            "not found"
                        }
                    ),
                );
                let wetspring = discover_primal("wetspring");
                v.check_bool(
                    "discover_wetspring",
                    wetspring.primal == "wetspring",
                    &format!(
                        "discover wetspring (cross-spring ecology): socket {}",
                        if wetspring.socket.is_some() {
                            "found"
                        } else {
                            "not found"
                        }
                    ),
                );
                let neuralspring = discover_primal("neuralspring");
                v.check_bool(
                    "discover_neuralspring",
                    neuralspring.primal == "neuralspring",
                    &format!(
                        "discover neuralspring (cross-spring ecology): socket {}",
                        if neuralspring.socket.is_some() {
                            "found"
                        } else {
                            "not found"
                        }
                    ),
                );

                v.check_skip(
                    "cross_spring_pipeline",
                    "cross-spring pipeline (airSpring→wetSpring→neuralSpring) needs live IPC",
                );
            },
        );
}
