// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp013: Pipeline Streaming — validates CoordinationPattern::Pipeline and PIPELINE_THROUGHPUT_MIN for streaming pipeline.

use primalspring::graphs::CoordinationPattern;
use primalspring::tolerances::PIPELINE_THROUGHPUT_MIN;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp013 — Pipeline Streaming");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp013: Pipeline (streaming_pipeline.toml)");
    println!("{}", "=".repeat(72));

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

    v.check_skip(
        "actual_streaming_pipeline",
        "actual streaming pipeline needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
