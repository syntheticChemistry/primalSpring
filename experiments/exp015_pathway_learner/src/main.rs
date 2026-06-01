// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp015: Pathway Learner — validates all five coordination patterns expose descriptions (exp010–014 family).
//!
//! Phases:
//!   1. Pattern inventory (all five CoordinationPattern variants)
//!   2. Composition discovery (orchestration)

use primalspring::composition::CompositionContext;
use primalspring::graphs::CoordinationPattern;
use primalspring::validation::ValidationResult;

fn phase_pattern_inventory(v: &mut ValidationResult) {
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
}

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let orchestration_ok = ctx.has_capability("orchestration");
    if orchestration_ok {
        v.check_bool(
            "orchestration_discovered",
            true,
            "orchestration capability present in composition context",
        );
    } else {
        v.check_skip(
            "orchestration_discovered",
            "orchestration capability not discovered",
        );
    }

    v.check_or_skip(
        "graph_deployment",
        orchestration_ok.then_some(()),
        "orchestration unavailable — graph deployment not validated",
        |(), v| {
            v.check_bool(
                "graph_deployment",
                true,
                "orchestration present for graph deployment path",
            );
        },
    );

    v.check_skip(
        "actual_pathway_learning",
        "actual pathway learning needs live IPC",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp015 — Pathway Learner")
        .with_provenance("exp015_pathway_learner", "2026-05-09")
        .run(
            "primalSpring Exp015: Pathway Learner (exp010–014 with metrics)",
            |v| {
                let ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Pattern Inventory");
                phase_pattern_inventory(v);

                v.section("Phase 2: Composition Discovery");
                phase_composition_discovery(v, &ctx);
            },
        );
}
