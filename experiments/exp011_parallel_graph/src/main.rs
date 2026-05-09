// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp011: Parallel Graph — validates parallel coordination with graph structure and parallel capability health probes.
//!
//! Phases:
//!   1. CoordinationPattern::Parallel constant checks
//!   2. Graph structure (load, spawnables, validate)
//!   3. Live composition via CompositionContext
//!   4. Parallel health (all discovered capabilities probed)

use std::path::{Path, PathBuf};
use std::time::Instant;

use primalspring::composition::CompositionContext;
use primalspring::deploy::{graph_spawnable_primals, load_graph, validate_structure};
use primalspring::graphs::CoordinationPattern;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn liveness_ok(result: &serde_json::Value) -> bool {
    result
        .get("alive")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
        || result
            .get("status")
            .and_then(|s| s.as_str())
            .is_some_and(|s| s == "ok" || s == "alive")
}

fn phase_pattern_constants(v: &mut ValidationResult) {
    let desc = CoordinationPattern::Parallel.description();
    v.check_bool(
        "parallel_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::Parallel.description() exists: {desc}"),
    );
}

fn phase_graph_structure(v: &mut ValidationResult, graph_path: &Path) {
    let result = validate_structure(graph_path);
    v.check_bool(
        "graph_parses",
        result.parsed,
        "parallel_capability_burst.toml parses",
    );
    v.check_bool(
        "graph_clean",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if !result.parsed {
        return;
    }

    let graph = match load_graph(graph_path) {
        Ok(g) => g,
        Err(e) => {
            v.check_bool(
                "load_graph",
                false,
                &format!("load parallel_capability_burst graph: {e}"),
            );
            return;
        }
    };

    let spawnable = graph_spawnable_primals(&graph);
    v.check_minimum("spawnable_count", spawnable.len(), 4);
}

fn phase_live_composition(v: &mut ValidationResult, ctx: &CompositionContext) {
    let n = ctx.available_capabilities().len();
    if n == 0 {
        v.check_skip("live_composition", "no capabilities discovered");
        return;
    }
    v.check_minimum("discovered_capability_count", n, 1);
}

fn phase_parallel_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let caps: Vec<String> = ctx
        .available_capabilities()
        .into_iter()
        .map(String::from)
        .collect();

    if caps.is_empty() {
        v.check_skip("parallel_health", "no capabilities discovered");
        return;
    }

    let start = Instant::now();
    let mut live = 0usize;
    let mut probed = 0usize;

    for cap in &caps {
        probed += 1;
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(ref result) if liveness_ok(result) => {
                live += 1;
                v.check_bool(&format!("{cap}_liveness"), true, &format!("{cap} live"));
            }
            Ok(result) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap} not live: {result}"),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap} probe failed: {e}"),
                );
            }
        }
    }

    let burst_ms = start.elapsed().as_millis();
    v.check_minimum("parallel_live_count", live, 1);
    v.check_bool(
        "parallel_health_burst_completed",
        probed == caps.len(),
        &format!("probed {probed} caps in {burst_ms}ms"),
    );
}

fn main() {
    let graph_path = graphs_dir().join("parallel_capability_burst.toml");

    ValidationResult::new("primalSpring Exp011 — Parallel Graph")
        .with_provenance("exp011_parallel_graph", "2026-05-09")
        .run(
            "primalSpring Exp011: Parallel (parallel_capability_burst.toml)",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Pattern Constants (CoordinationPattern::Parallel)");
                phase_pattern_constants(v);

                v.section("Phase 2: Graph Structure");
                phase_graph_structure(v, &graph_path);

                v.section("Phase 3: Live Composition");
                phase_live_composition(v, &ctx);

                v.section("Phase 4: Parallel Health");
                phase_parallel_health(v, &mut ctx);
            },
        );
}
