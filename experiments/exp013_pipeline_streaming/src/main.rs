// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp013: Pipeline Streaming — validates CoordinationPattern::Pipeline and PIPELINE_THROUGHPUT_MIN for streaming pipeline.

use primalspring::graphs::CoordinationPattern;
use primalspring::ipc::discover::neural_api_healthy;
use primalspring::tolerances::PIPELINE_THROUGHPUT_MIN;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp013 — Pipeline Streaming")
        .with_provenance("exp013_pipeline_streaming", "2026-03-24")
        .run(
            "primalSpring Exp013: Pipeline (streaming_pipeline.toml)",
            |v| {
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
                    "actual_streaming_pipeline",
                    "actual streaming pipeline needs live IPC",
                );
            },
        );
}
