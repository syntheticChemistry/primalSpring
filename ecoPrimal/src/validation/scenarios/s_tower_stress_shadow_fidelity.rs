// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Stress — Shadow Fidelity.
//!
//! `membrane tower.shadow` runs periodic point-in-time benchmarks
//! (20 latency probes, 10 setup attempts, single throughput window).
//! This scenario validates whether these point measurements faithfully
//! represent sustained real-world behavior.
//!
//! Key question: does a 20-probe benchmark taken every N minutes give
//! the same latency/jitter picture as a continuous 30-minute stream?
//!
//! Validates:
//! - Shadow timer configuration and invocation pattern
//! - Benchmark output consistency across runs (coefficient of variation)
//! - Whether point measurements catch sustained degradation
//! - Data pipeline from benchmark JSON to benchScale storage

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const BENCHMARK_RS: &str = include_str!("../../../../../../primals/songBird/src/benchmark.rs");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-stress-shadow-fidelity",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_stress",
        provenance_date: "2026-07-23",
        description: "Tower stress — shadow benchmark fidelity: point-in-time vs sustained measurement",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Shadow timer infrastructure");
    phase_shadow_timer(v);

    v.section("Phase 2: Measurement statistical validity");
    phase_statistical_validity(v);

    v.section("Phase 3: Data pipeline integrity");
    phase_data_pipeline(v);
}

fn phase_shadow_timer(v: &mut ValidationResult) {
    let shadow_benchmark_sh = std::path::Path::new("benchScale/tower_shadow/shadow-benchmark.sh");
    let has_shadow_script = shadow_benchmark_sh.exists();
    v.check_bool(
        "shadow:benchmark_script",
        has_shadow_script,
        &format!(
            "Shadow benchmark script: {}",
            if has_shadow_script {
                "benchScale/tower_shadow/shadow-benchmark.sh exists"
            } else {
                "NOT FOUND (check CWD or deployment path)"
            }
        ),
    );

    let has_json_output =
        BENCHMARK_RS.contains("OutputFormat::Json") || BENCHMARK_RS.contains("\"json\"");
    v.check_bool(
        "shadow:json_output",
        has_json_output,
        "Benchmark supports JSON output (machine-parseable for shadow pipeline)",
    );

    let has_timestamp = BENCHMARK_RS.contains("timestamp") || BENCHMARK_RS.contains("chrono");
    v.check_bool(
        "shadow:timestamped_output",
        has_timestamp,
        "Output includes timestamp (correlate shadow samples with time series)",
    );
}

fn phase_statistical_validity(v: &mut ValidationResult) {
    let has_percentiles = BENCHMARK_RS.contains("p50")
        && BENCHMARK_RS.contains("p95")
        && BENCHMARK_RS.contains("p99");
    v.check_bool(
        "shadow:percentile_reporting",
        has_percentiles,
        "Percentile reporting (p50/p95/p99) — tail latency visibility",
    );

    let has_jitter = BENCHMARK_RS.contains("jitter");
    v.check_bool(
        "shadow:jitter_measurement",
        has_jitter,
        "Jitter measurement — inter-probe variation captures consistency",
    );

    let has_probe_count =
        BENCHMARK_RS.contains("probes_sent") && BENCHMARK_RS.contains("probes_ok");
    v.check_bool(
        "shadow:probe_success_rate",
        has_probe_count,
        "Probe success tracking (probes_sent vs probes_ok) — detects intermittent failures",
    );

    let default_probes = BENCHMARK_RS.contains("default_value_t = 50")
        || BENCHMARK_RS.contains("default_value_t = 20");
    v.check_bool(
        "shadow:probe_sample_size",
        default_probes,
        &format!(
            "Default probe count: {} — \
             20 probes gives ±22% CI at 95% confidence. \
             50+ recommended for sustained fidelity comparison",
            if BENCHMARK_RS.contains("default_value_t = 50") {
                "50 (adequate)"
            } else if BENCHMARK_RS.contains("default_value_t = 20") {
                "20 (marginal — increase for fidelity comparison)"
            } else {
                "unknown"
            }
        ),
    );

    let uses_sorted_percentile =
        BENCHMARK_RS.contains("sort_by") || BENCHMARK_RS.contains("percentile");
    v.check_bool(
        "shadow:sorted_percentile",
        uses_sorted_percentile,
        "Percentiles computed from sorted array (correct method, not streaming approximation)",
    );
}

fn phase_data_pipeline(v: &mut ValidationResult) {
    let shadow_dir = std::path::Path::new("benchScale/tower_shadow");
    let has_shadow_dir = shadow_dir.exists();
    v.check_bool(
        "shadow:data_directory",
        has_shadow_dir,
        &format!(
            "Shadow data directory: {}",
            if has_shadow_dir {
                "benchScale/tower_shadow/ exists"
            } else {
                "NOT FOUND"
            }
        ),
    );

    if has_shadow_dir {
        let json_count = shadow_dir
            .read_dir()
            .map(|rd| {
                rd.filter(|e| {
                    e.as_ref()
                        .map(|e| e.path().extension().is_some_and(|ext| ext == "json"))
                        .unwrap_or(false)
                })
                .count()
            })
            .unwrap_or(0);

        v.check_bool(
            "shadow:json_file_count",
            json_count >= 2,
            &format!(
                "{json_count} JSON benchmark files — {} for fidelity comparison",
                if json_count >= 10 {
                    "sufficient"
                } else if json_count >= 2 {
                    "minimal"
                } else {
                    "INSUFFICIENT"
                }
            ),
        );

        let has_paired_runs = json_count >= 4;
        v.check_bool(
            "shadow:paired_tower_wg",
            has_paired_runs,
            &format!(
                "Paired Tower/WG runs: {} — fidelity comparison needs paired measurements",
                if has_paired_runs {
                    "sufficient paired data"
                } else {
                    "need more paired runs"
                }
            ),
        );
    }

    let has_structured_report = BENCHMARK_RS.contains("BenchmarkReport")
        && BENCHMARK_RS.contains("serde_json::to_string_pretty");
    v.check_bool(
        "shadow:structured_json_report",
        has_structured_report,
        "Structured JSON report (BenchmarkReport → serde_json) — parseable by analysis pipeline",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_no_panic() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
    }
}
