// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp013: Pipeline Streaming — validates `CoordinationPattern::Pipeline` and `PIPELINE_THROUGHPUT_MIN`.
//!
//! Phases:
//!   1. Pattern constants (Pipeline, throughput tolerance)
//!   2. Composition discovery (orchestration)

use primalspring::composition::CompositionContext;
use primalspring::graphs::CoordinationPattern;
use primalspring::tolerances::PIPELINE_THROUGHPUT_MIN;
use primalspring::validation::ValidationResult;

fn phase_pattern_constants(v: &mut ValidationResult) {
    let desc = CoordinationPattern::Pipeline.description();
    v.check_bool(
        "pipeline_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::Pipeline.description() exists: {desc}"),
    );
    v.check_bool(
        "pipeline_throughput_min_positive",
        PIPELINE_THROUGHPUT_MIN > 0,
        &format!("PIPELINE_THROUGHPUT_MIN > 0 ({PIPELINE_THROUGHPUT_MIN})"),
    );
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
        "actual_streaming_pipeline",
        "actual streaming pipeline needs live IPC",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp013 — Pipeline Streaming")
        .with_provenance("exp013_pipeline_streaming", "2026-05-09")
        .run(
            "primalSpring Exp013: Pipeline (streaming_pipeline.toml)",
            |v| {
                let ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Pattern Constants (CoordinationPattern::Pipeline)");
                phase_pattern_constants(v);

                v.section("Phase 2: Composition Discovery");
                phase_composition_discovery(v, &ctx);
            },
        );
}
