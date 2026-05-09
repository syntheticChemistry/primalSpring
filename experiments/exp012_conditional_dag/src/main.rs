// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp012: Conditional DAG — validates conditional coordination and graph spawn rules with live compute probes.
//!
//! Phases:
//!   1. CoordinationPattern::ConditionalDag constant checks
//!   2. Graph structure for conditional_fallback.toml
//!   3. Live composition via CompositionContext
//!   4. Conditional branch (compute availability / fallback semantics)

use std::path::{Path, PathBuf};

use primalspring::composition::CompositionContext;
use primalspring::deploy::{graph_spawnable_primals, load_graph, validate_structure};
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
    let desc = CoordinationPattern::ConditionalDag.description();
    v.check_bool(
        "conditional_dag_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::ConditionalDag.description() exists: {desc}"),
    );
}

fn phase_graph_structure(v: &mut ValidationResult, graph_path: &Path) {
    let result = validate_structure(graph_path);
    v.check_bool(
        "graph_parses",
        result.parsed,
        "conditional_fallback.toml parses",
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
                &format!("load conditional_fallback graph: {e}"),
            );
            return;
        }
    };

    let spawnable = graph_spawnable_primals(&graph);
    v.check_bool(
        "cpu_fallback_not_spawned",
        !spawnable.contains(&"cpu_fallback".to_owned()),
        "cpu_fallback (spawn=false) excluded from spawnable",
    );
    v.check_bool(
        "toadstool_spawnable",
        spawnable.contains(&primal_names::TOADSTOOL.to_owned()),
        "toadstool is spawnable (primary path)",
    );
}

fn phase_live_composition(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let n = ctx.available_capabilities().len();
    if n == 0 {
        v.check_skip("live_composition", "no capabilities discovered");
        return;
    }
    v.check_minimum("discovered_capability_count", n, 1);

    let caps: Vec<String> = ctx
        .available_capabilities()
        .into_iter()
        .map(String::from)
        .collect();

    for cap in &caps {
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(ref result) if liveness_ok(result) => {
                v.check_bool(&format!("{cap}_liveness"), true, &format!("{cap} live"));
            }
            Ok(result) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap}: {result}"),
                );
            }
            Err(e) => {
                v.check_bool(&format!("{cap}_liveness"), false, &format!("{cap}: {e}"));
            }
        }
    }
}

fn phase_conditional_branch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("compute") {
        v.check_skip(
            "primary_path_active",
            "compute capability not discovered — CPU fallback path",
        );
        return;
    }

    match ctx.call("compute", "health.liveness", serde_json::json!({})) {
        Ok(ref result) if liveness_ok(result) => {
            v.check_bool(
                "primary_path_active",
                true,
                "compute healthy — GPU-oriented dispatch path may be active",
            );
            v.check_bool(
                "compute_capability_live",
                true,
                "compute responds to health.liveness",
            );
        }
        Ok(result) => {
            v.check_skip(
                "primary_path_active",
                &format!("compute not live: {result}"),
            );
        }
        Err(e) => {
            v.check_skip(
                "primary_path_active",
                &format!("compute health probe failed: {e}"),
            );
        }
    }
}

fn main() {
    let graph_path = graphs_dir().join("conditional_fallback.toml");

    ValidationResult::new("primalSpring Exp012 — Conditional DAG")
        .with_provenance("exp012_conditional_dag", "2026-05-09")
        .run(
            "primalSpring Exp012: ConditionalDag (conditional_fallback.toml)",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Pattern Constants (CoordinationPattern::ConditionalDag)");
                phase_pattern_constants(v);

                v.section("Phase 2: Graph Structure (conditional_fallback.toml)");
                phase_graph_structure(v, &graph_path);

                v.section("Phase 3: Live Composition");
                phase_live_composition(v, &mut ctx);

                v.section("Phase 4: Conditional Branch");
                phase_conditional_branch(v, &mut ctx);
            },
        );
}
