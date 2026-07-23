// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Atomic Parity — Live Benchmark Results.
//!
//! Validates actual benchmark measurements from `songbird benchmark` against
//! the relative parity targets defined in the convergence brief.
//!
//! Reads JSON benchmark reports from `benchScale/tower_parity/`:
//! - `lan_tower.json` — Tower Atomic LAN results
//! - `lan_wg.json` — `WireGuard` LAN baseline
//! - `wan_tower.json` — Tower Atomic WAN results (via TURN relay)
//! - `wan_wg.json` — `WireGuard` WAN baseline
//!
//! Parity targets (relative to WG baseline):
//! - Throughput: ≥80% of WG
//! - LAN latency (p95): ≤2x WG
//! - WAN latency (p95): ≤1.5x WG
//! - Setup time: ≤500ms
//!
//! This is a Live-tier scenario — SKIP if benchmark data is absent.

use std::path::Path;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const BENCH_DIR: &str = "benchScale/tower_parity";

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-atomic-parity-live",
        track: Track::Evolution,
        tier: Tier::Live,
        provenance_crate: "wave150v_tower_parity_live",
        provenance_date: "2026-07-23",
        description: "Tower Atomic parity LIVE — benchmark results vs WG baseline (relative targets)",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let bench_path = Path::new(BENCH_DIR);
    if !bench_path.exists() {
        v.check_skip(
            "live:bench_data_present",
            "benchScale/tower_parity/ absent — run songbird benchmark first",
        );
        return;
    }

    v.section("Phase 1: Benchmark data presence");
    let lan_tower = read_benchmark(bench_path, "lan_tower.json");
    let lan_wg = read_benchmark(bench_path, "lan_wg.json");
    let wan_tower = read_benchmark(bench_path, "wan_tower.json");
    let wan_wg = read_benchmark(bench_path, "wan_wg.json");

    v.check_bool(
        "live:lan_tower_data",
        lan_tower.is_some(),
        "LAN Tower benchmark data present (lan_tower.json)",
    );
    v.check_bool(
        "live:lan_wg_data",
        lan_wg.is_some(),
        "LAN WireGuard baseline data present (lan_wg.json)",
    );
    v.check_bool(
        "live:wan_tower_data",
        wan_tower.is_some(),
        "WAN Tower benchmark data present (wan_tower.json)",
    );
    v.check_bool(
        "live:wan_wg_data",
        wan_wg.is_some(),
        "WAN WireGuard baseline data present (wan_wg.json)",
    );

    let (Some(lt), Some(lw)) = (lan_tower.as_ref(), lan_wg.as_ref()) else {
        v.check_skip(
            "live:lan_parity",
            "LAN benchmark data incomplete — cannot assess",
        );
        v.check_skip(
            "live:wan_parity",
            "WAN benchmark data incomplete — cannot assess",
        );
        return;
    };

    v.section("Phase 2: LAN parity assessment (sporeGate ↔ eastGate)");
    assess_latency_parity(v, "lan", lt, lw, 2.0);
    assess_throughput_parity(v, "lan", lt, lw);
    assess_setup_time(v, "lan", lt);

    if let (Some(wt), Some(ww)) = (wan_tower.as_ref(), wan_wg.as_ref()) {
        v.section("Phase 3: WAN parity assessment (sporeGate → TURN → flockGate)");
        assess_latency_parity(v, "wan", wt, ww, 1.5);
        assess_throughput_parity(v, "wan", wt, ww);
        assess_setup_time(v, "wan", wt);
    } else {
        v.check_skip(
            "live:wan_parity",
            "WAN benchmark data incomplete — skipping",
        );
    }

    v.section("Phase 4: Convergence verdict");
    let lan_pass = lt.latency_p95_ms <= lw.latency_p95_ms * 2.0
        && lt.throughput_mbps >= lw.throughput_mbps * 0.8;
    v.check_bool(
        "live:lan_parity_verdict",
        lan_pass,
        &format!(
            "LAN PARITY: Tower p95={:.1}ms (WG={:.1}ms, limit={:.1}ms), \
             throughput={:.0}Mbps (WG={:.0}Mbps, floor={:.0}Mbps)",
            lt.latency_p95_ms,
            lw.latency_p95_ms,
            lw.latency_p95_ms * 2.0,
            lt.throughput_mbps,
            lw.throughput_mbps,
            lw.throughput_mbps * 0.8,
        ),
    );

    if let (Some(wt), Some(ww)) = (wan_tower.as_ref(), wan_wg.as_ref()) {
        let wan_pass = wt.latency_p95_ms <= ww.latency_p95_ms * 1.5
            && wt.throughput_mbps >= ww.throughput_mbps * 0.8;
        v.check_bool(
            "live:wan_parity_verdict",
            wan_pass,
            &format!(
                "WAN PARITY: Tower p95={:.1}ms (WG={:.1}ms, limit={:.1}ms), \
                 throughput={:.0}Mbps (WG={:.0}Mbps, floor={:.0}Mbps)",
                wt.latency_p95_ms,
                ww.latency_p95_ms,
                ww.latency_p95_ms * 1.5,
                wt.throughput_mbps,
                ww.throughput_mbps,
                ww.throughput_mbps * 0.8,
            ),
        );
    }
}

fn assess_latency_parity(
    v: &mut ValidationResult,
    label: &str,
    tower: &BenchmarkResult,
    wg: &BenchmarkResult,
    multiplier: f64,
) {
    let limit = wg.latency_p95_ms * multiplier;
    let pass = tower.latency_p95_ms <= limit;
    v.check_bool(
        &format!("live:{label}_latency_p95"),
        pass,
        &format!(
            "{label} latency p95: Tower={:.2}ms, WG={:.2}ms, limit=WG*{multiplier:.1}={:.2}ms",
            tower.latency_p95_ms, wg.latency_p95_ms, limit,
        ),
    );
}

fn assess_throughput_parity(
    v: &mut ValidationResult,
    label: &str,
    tower: &BenchmarkResult,
    wg: &BenchmarkResult,
) {
    let floor = wg.throughput_mbps * 0.8;
    let pass = tower.throughput_mbps >= floor;
    v.check_bool(
        &format!("live:{label}_throughput"),
        pass,
        &format!(
            "{label} throughput: Tower={:.0}Mbps, WG={:.0}Mbps, floor=WG*0.8={:.0}Mbps",
            tower.throughput_mbps, wg.throughput_mbps, floor,
        ),
    );
}

fn assess_setup_time(v: &mut ValidationResult, label: &str, tower: &BenchmarkResult) {
    let pass = tower.setup_ms <= 500.0;
    v.check_bool(
        &format!("live:{label}_setup_time"),
        pass,
        &format!(
            "{label} setup time: Tower={:.0}ms (target ≤500ms)",
            tower.setup_ms,
        ),
    );
}

struct BenchmarkResult {
    latency_p95_ms: f64,
    throughput_mbps: f64,
    setup_ms: f64,
}

fn read_benchmark(dir: &Path, filename: &str) -> Option<BenchmarkResult> {
    let path = dir.join(filename);
    let content = std::fs::read_to_string(&path).ok()?;
    parse_benchmark_json(&content)
}

fn parse_benchmark_json(json: &str) -> Option<BenchmarkResult> {
    let parsed: serde_json::Value = serde_json::from_str(json).ok()?;

    let latency_p95_ms = parsed
        .get("latency")
        .and_then(|l| l.get("p95_ms"))
        .and_then(serde_json::Value::as_f64)
        .or_else(|| {
            parsed
                .get("latency_p95_ms")
                .and_then(serde_json::Value::as_f64)
        })?;

    let throughput_mbps = parsed
        .get("throughput")
        .and_then(|t| t.get("mbps"))
        .and_then(serde_json::Value::as_f64)
        .or_else(|| {
            parsed
                .get("throughput_mbps")
                .and_then(serde_json::Value::as_f64)
        })
        .unwrap_or(0.0);

    let setup_ms = parsed
        .get("setup")
        .and_then(|s| s.get("ms"))
        .and_then(serde_json::Value::as_f64)
        .or_else(|| parsed.get("setup_ms").and_then(serde_json::Value::as_f64))
        .unwrap_or(0.0);

    Some(BenchmarkResult {
        latency_p95_ms,
        throughput_mbps,
        setup_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_skips_without_data() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "live scenario should skip gracefully without benchmark data"
        );
    }

    #[test]
    fn parse_nested_json() {
        let json = r#"{"latency":{"p50_ms":0.5,"p95_ms":1.2,"p99_ms":2.0},"throughput":{"mbps":890.5},"setup":{"ms":45.0}}"#;
        let result = parse_benchmark_json(json).unwrap();
        assert!((result.latency_p95_ms - 1.2).abs() < 0.001);
        assert!((result.throughput_mbps - 890.5).abs() < 0.1);
        assert!((result.setup_ms - 45.0).abs() < 0.1);
    }

    #[test]
    fn parse_flat_json() {
        let json = r#"{"latency_p95_ms":2.5,"throughput_mbps":750.0,"setup_ms":120.0}"#;
        let result = parse_benchmark_json(json).unwrap();
        assert!((result.latency_p95_ms - 2.5).abs() < 0.001);
        assert!((result.throughput_mbps - 750.0).abs() < 0.1);
        assert!((result.setup_ms - 120.0).abs() < 0.1);
    }
}
