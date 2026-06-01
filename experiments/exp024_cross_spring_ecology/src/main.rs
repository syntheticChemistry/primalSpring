// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp024: Cross-spring ecology — EmergentSystem graphs and composition discovery.

use primalspring::composition::CompositionContext;
use primalspring::emergent::EmergentSystem;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let graphs = EmergentSystem::CrossSpringEcology.required_graphs();
    v.check_bool(
        "cross_spring_ecology_graphs_non_empty",
        !graphs.is_empty(),
        &format!("EmergentSystem::CrossSpringEcology.required_graphs() is non-empty: {graphs:?}"),
    );
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    let joined = caps.join(", ");
    v.check_bool(
        "discover_airspring",
        caps.iter().any(|c| c.contains("air")),
        &format!("airspring is a spring (not a primal); capabilities: {joined}"),
    );
    v.check_bool(
        "discover_wetspring",
        caps.iter().any(|c| c.contains("wet")),
        &format!("wetspring is a spring (not a primal); capabilities: {joined}"),
    );
    v.check_bool(
        "discover_neuralspring",
        caps.iter().any(|c| c.contains("neural")),
        &format!("neuralspring is a spring (not a primal); capabilities: {joined}"),
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp024 — Cross-Spring Ecology")
        .with_provenance("exp024_cross_spring_ecology", "2026-05-09")
        .run(
            "primalSpring Exp024: cross_spring_ecology.toml with airSpring + wetSpring Data",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Composition Discovery");
                let ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.check_skip(
                    "cross_spring_pipeline",
                    "cross-spring pipeline (airSpring→wetSpring→neuralSpring) needs live IPC",
                );
            },
        );
}
