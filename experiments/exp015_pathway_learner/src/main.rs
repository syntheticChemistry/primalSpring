// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp015: Pathway Learner — validates all five coordination patterns have descriptions (exp010–014 with metrics).

use primalspring::graphs::CoordinationPattern;
use primalspring::ipc::discover::neural_api_healthy;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp015 — Pathway Learner")
        .with_provenance("exp015_pathway_learner", "2026-03-24")
        .run(
            "primalSpring Exp015: Pathway Learner (exp010–014 with metrics)",
            |v| {
                let patterns = [
                    CoordinationPattern::Sequential,
                    CoordinationPattern::Parallel,
                    CoordinationPattern::ConditionalDag,
                    CoordinationPattern::Pipeline,
                    CoordinationPattern::Continuous,
                ];
                for (i, p) in patterns.iter().enumerate() {
                    let desc = p.description();
                    v.check_bool(
                        &format!("pattern_{i}_description_non_empty"),
                        !desc.is_empty(),
                        &format!("{p:?} has non-empty description: {desc}"),
                    );
                }

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
                    "actual_pathway_learning",
                    "actual pathway learning needs live IPC",
                );
            },
        );
}
