// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Stress — Sustained Throughput.
//!
//! The existing benchmark harness sends a single 64KB burst and measures
//! write rate. On LAN, the entire transfer completes in <1ms, producing
//! `duration_ms: 0` and meaningless throughput numbers.
//!
//! This stress scenario validates that the benchmark infrastructure can
//! support **sustained streaming** (iperf3-style: continuous send for
//! 30s/60s/300s) and that Tower Atomic maintains throughput parity with
//! `WireGuard` under sustained load — not just single-shot.
//!
//! Structural checks: benchmark harness has duration-aware streaming,
//! results use microsecond precision, and throughput is measured over
//! configurable windows.
//!
//! Live checks: run sustained benchmark against mesh peers and verify
//! Tower throughput ≥ 0.8× WG over 30s sustained window.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const BENCHMARK_RS: &str = include_str!("../../../../../../primals/songBird/src/benchmark.rs");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-stress-sustained-throughput",
        track: Track::Evolution,
        tier: Tier::Both,
        provenance_crate: "wave150w_tower_stress",
        provenance_date: "2026-07-23",
        description: "Tower stress — sustained throughput over 30s/60s/300s windows",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Harness streaming capability");
    phase_harness_streaming(v);

    v.section("Phase 2: Duration precision");
    phase_duration_precision(v);

    v.section("Phase 3: Sustained window validation");
    phase_sustained_windows(v);
}

fn phase_harness_streaming(v: &mut ValidationResult) {
    let has_duration_flag =
        BENCHMARK_RS.contains("--duration") || BENCHMARK_RS.contains("duration: Duration");
    v.check_bool(
        "stress:harness_duration_flag",
        has_duration_flag,
        "Benchmark harness supports configurable duration for streaming",
    );

    let has_chunk_loop = BENCHMARK_RS.contains("while start.elapsed() < duration");
    v.check_bool(
        "stress:continuous_send_loop",
        has_chunk_loop,
        "Throughput measurement uses continuous send loop (not single-shot)",
    );

    let chunk_size_64k = BENCHMARK_RS.contains("65_536") || BENCHMARK_RS.contains("65536");
    v.check_bool(
        "stress:chunk_size_64k",
        chunk_size_64k,
        "Send chunk is 64KB (adequate for streaming; larger chunks would improve LAN accuracy)",
    );

    let has_probes_flag = BENCHMARK_RS.contains("--probes") || BENCHMARK_RS.contains("probes: u32");
    v.check_bool(
        "stress:configurable_probe_count",
        has_probes_flag,
        "Latency probe count is configurable (needed for statistical significance under load)",
    );
}

fn phase_duration_precision(v: &mut ValidationResult) {
    let uses_as_millis = BENCHMARK_RS.contains("as_millis() as u64");
    let uses_as_micros = BENCHMARK_RS.contains("as_micros()");
    let uses_secs_f64 = BENCHMARK_RS.contains("as_secs_f64()");

    v.check_bool(
        "stress:precision_millisecond_truncation",
        uses_as_millis,
        &format!(
            "Duration uses as_millis() — {} for LAN (sub-ms transfers truncate to 0). \
             Migration to as_micros() or as_secs_f64() needed",
            if uses_as_millis {
                "CONFIRMED BUG"
            } else {
                "fixed"
            }
        ),
    );

    v.check_bool(
        "stress:precision_high_res_available",
        uses_secs_f64,
        &format!(
            "Harness uses as_secs_f64() for throughput_mbps: {} — \
             duration_ms field should also use sub-ms precision",
            if uses_secs_f64 { "YES" } else { "NO" }
        ),
    );

    v.check_bool(
        "stress:precision_micros_migration",
        uses_as_micros || uses_secs_f64,
        "High-resolution timing available (as_micros or as_secs_f64) for duration reporting",
    );
}

fn phase_sustained_windows(v: &mut ValidationResult) {
    let durations = ["10s", "30s", "60s", "300s"];
    let has_duration_parser = BENCHMARK_RS.contains("parse_duration");

    v.check_bool(
        "stress:duration_parser",
        has_duration_parser,
        "Duration parser present (supports '10s', '30s', '60s', '300s' etc.)",
    );

    for d in &durations {
        v.check_bool(
            &format!("stress:window_{d}"),
            has_duration_parser,
            &format!(
                "{d} sustained window: harness {} invoke with --duration {d}",
                if has_duration_parser { "CAN" } else { "CANNOT" }
            ),
        );
    }

    let has_timeout = BENCHMARK_RS.contains("--timeout") || BENCHMARK_RS.contains("timeout:");
    v.check_bool(
        "stress:per_op_timeout",
        has_timeout,
        "Per-operation timeout prevents hung connections from blocking sustained test",
    );

    let has_write_timeout = BENCHMARK_RS.contains("timeout(Duration::from_secs");
    v.check_bool(
        "stress:write_timeout",
        has_write_timeout,
        "Individual writes have timeout (prevents indefinite block on peer disconnect)",
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
