// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp010: Sequential Graph — validates sequential coordination with graph structure and live capability probes.
//!
//! Phases:
//!   1. CoordinationPattern::Sequential constant checks
//!   2. Graph structure (load, validate, topological waves)
//!   3. Live composition via CompositionContext health probes
//!   4. Sequential ordering (security before discovery in probe order)

use std::path::{Path, PathBuf};

use primalspring::composition::CompositionContext;
use primalspring::deploy::{load_graph, topological_waves, validate_structure};
use primalspring::graphs::CoordinationPattern;
use primalspring::primal_names;
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
    let desc = CoordinationPattern::Sequential.description();
    v.check_bool(
        "sequential_description_non_empty",
        !desc.is_empty(),
        &format!("CoordinationPattern::Sequential.description() is non-empty: {desc}"),
    );
    v.check_bool(
        "sequential_description_meaningful",
        desc.len() > 10 && desc.to_ascii_lowercase().contains("order"),
        &format!("sequential description conveys ordering semantics: {desc}"),
    );
}

fn phase_graph_structure(v: &mut ValidationResult, graph_path: &Path) {
    let result = validate_structure(graph_path);
    v.check_bool(
        "graph_parses",
        result.parsed,
        "tower_atomic_bootstrap.toml parses",
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
                &format!("load tower_atomic_bootstrap graph: {e}"),
            );
            return;
        }
    };

    let waves = match topological_waves(&graph) {
        Ok(w) => w,
        Err(e) => {
            v.check_bool(
                "topological_waves",
                false,
                &format!("compute topological waves: {e}"),
            );
            return;
        }
    };

    v.check_minimum("topological_waves", waves.len(), 2);

    let beardog = primal_names::BEARDOG.to_owned();
    let songbird = primal_names::SONGBIRD.to_owned();
    v.check_bool(
        "beardog_before_songbird",
        waves.len() >= 2
            && waves[0].contains(&beardog)
            && waves.iter().skip(1).any(|w| w.contains(&songbird)),
        "beardog is in wave 0, songbird in a later wave",
    );
}

fn phase_live_composition(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let caps: Vec<String> = ctx
        .available_capabilities()
        .into_iter()
        .map(String::from)
        .collect();

    if caps.is_empty() {
        v.check_skip(
            "live_composition",
            "no capabilities discovered — cannot probe live composition",
        );
        return;
    }

    v.check_minimum("discovered_capability_count", caps.len(), 1);

    let mut live_count = 0usize;
    for cap in &caps {
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(ref result) if liveness_ok(result) => {
                live_count += 1;
                v.check_bool(
                    &format!("{cap}_liveness"),
                    true,
                    &format!("{cap} health.liveness ok"),
                );
            }
            Ok(result) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap} health.liveness not live: {result}"),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap} health.liveness call failed: {e}"),
                );
            }
        }
    }

    v.check_minimum("live_capability_count", live_count, 1);
}

fn phase_sequential_ordering(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mut caps: Vec<String> = ctx
        .available_capabilities()
        .into_iter()
        .map(String::from)
        .collect();

    if caps.is_empty() {
        v.check_skip(
            "sequential_security_before_discovery",
            "no capabilities — ordering check not applicable",
        );
        return;
    }

    caps.sort_by(|a, b| {
        fn rank(s: &str) -> u8 {
            match s {
                "security" => 0,
                "discovery" => 1,
                _ => 2,
            }
        }
        rank(a).cmp(&rank(b)).then_with(|| a.cmp(b))
    });

    if let (Some(i_sec), Some(i_disc)) = (
        caps.iter().position(|c| c == "security"),
        caps.iter().position(|c| c == "discovery"),
    ) {
        v.check_bool(
            "sequential_security_before_discovery",
            i_sec < i_disc,
            "security is ordered before discovery",
        );
    }

    for cap in &caps {
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(ref result) => {
                v.check_bool(
                    &format!("ordered_{cap}_liveness"),
                    liveness_ok(result),
                    &format!("ordered probe {cap}: {result}"),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("ordered_{cap}_liveness"),
                    false,
                    &format!("ordered probe {cap} failed: {e}"),
                );
            }
        }
    }
}

fn main() {
    let graph_path = graphs_dir().join("tower_atomic_bootstrap.toml");

    ValidationResult::new("primalSpring Exp010 — Sequential Graph")
        .with_provenance("exp010_sequential_graph", "2026-05-09")
        .run(
            "primalSpring Exp010: Sequential (tower_atomic_bootstrap.toml)",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Pattern Constants (CoordinationPattern::Sequential)");
                phase_pattern_constants(v);

                v.section("Phase 2: Graph Structure");
                phase_graph_structure(v, &graph_path);

                v.section("Phase 3: Live Composition");
                phase_live_composition(v, &mut ctx);

                v.section("Phase 4: Sequential Ordering");
                phase_sequential_ordering(v, &mut ctx);
            },
        );
}
