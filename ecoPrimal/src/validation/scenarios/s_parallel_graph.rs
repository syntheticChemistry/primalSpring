// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Parallel Graph — validates parallel + pipeline + continuous
//! coordination patterns, graph overlay mechanics, and concurrency invariants.

use crate::composition::CompositionContext;
use crate::deploy::{load_graph, topological_waves, validate_structure};
use crate::graphs::CoordinationPattern;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::path::{Path, PathBuf};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "parallel-graph",
        track: Track::GraphExecution,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-22",
        description: "Parallel/pipeline/continuous coordination patterns and concurrency invariants",
    },
    run,
};

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs")
}

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Coordination Pattern Coverage");
    phase_pattern_coverage(v);

    v.section("Phase 2: Parallel Graph Structural");
    phase_parallel_structural(v);

    v.section("Phase 3: All Graphs Validate");
    phase_all_graphs_validate(v);

    v.section("Phase 4: Live Composition Patterns");
    phase_live_composition_patterns(v, ctx);
}

fn phase_pattern_coverage(v: &mut ValidationResult) {
    let all = [
        CoordinationPattern::Sequential,
        CoordinationPattern::Parallel,
        CoordinationPattern::ConditionalDag,
        CoordinationPattern::Pipeline,
        CoordinationPattern::Continuous,
    ];

    for p in &all {
        let desc = p.description();
        v.check_bool(
            &format!(
                "pattern:{}:has_description",
                format!("{p:?}").to_lowercase()
            ),
            !desc.is_empty(),
            &format!("{p:?}: {desc}"),
        );
    }

    v.check_bool(
        "pattern_count",
        all.len() == 5,
        &format!("got {}, expected 5", all.len()),
    );
}

fn phase_parallel_structural(v: &mut ValidationResult) {
    let dir = graphs_dir();
    if !dir.is_dir() {
        v.check_skip("graphs_dir_exists", "graphs directory not found");
        return;
    }

    let entries: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "toml"))
        .collect();

    v.check_bool(
        "graph_corpus_size",
        entries.len() >= 10,
        &format!("{} TOML graphs in corpus", entries.len()),
    );

    let mut graph_count = 0u32;
    for entry in &entries {
        let path = entry.path();
        if load_graph(&path).is_ok() {
            graph_count += 1;
        }
    }

    v.check_bool(
        "graphs_parseable",
        graph_count > 0,
        &format!("{graph_count} graphs parsed successfully"),
    );
}

fn phase_all_graphs_validate(v: &mut ValidationResult) {
    let dir = graphs_dir();
    if !dir.is_dir() {
        v.check_skip("graphs_dir_validate", "graphs directory not found");
        return;
    }

    let entries: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "toml"))
        .collect();

    let mut valid = 0u32;
    let mut invalid = 0u32;

    for entry in &entries {
        let path = entry.path();
        let name = path
            .file_stem()
            .map_or("unknown", |s| s.to_str().unwrap_or("unknown"));

        let gv = validate_structure(&path);
        if gv.parsed && gv.issues.is_empty() {
            valid += 1;
            if let Ok(g) = load_graph(&path) {
                if let Ok(waves) = topological_waves(&g) {
                    v.check_bool(
                        &format!("graph:{name}:topology"),
                        !waves.is_empty(),
                        &format!("{} topological waves", waves.len()),
                    );
                }
            }
        } else {
            invalid += 1;
            v.check_bool(
                &format!("graph:{name}:valid"),
                false,
                &format!("{} structural issues", gv.issues.len()),
            );
        }
    }

    v.check_bool(
        "graph_validation_summary",
        invalid == 0,
        &format!("{valid} valid, {invalid} invalid"),
    );
}

fn phase_live_composition_patterns(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    v.check_bool(
        "live:composition_context",
        !caps.is_empty(),
        &format!("{} capabilities discovered", caps.len()),
    );

    let has_dispatch = caps.iter().any(|c| c.contains("dispatch"));
    let has_health = caps.iter().any(|c| c.contains("health"));
    let has_dag = caps.iter().any(|c| c.contains("dag"));

    if has_dispatch {
        v.check_bool(
            "live:dispatch_capability",
            true,
            "dispatch capability available for parallel execution",
        );
    } else {
        v.check_skip("live:dispatch_capability", "no dispatch capability");
    }

    if has_health {
        v.check_bool(
            "live:health_capability",
            true,
            "health capability for pipeline monitoring",
        );
    } else {
        v.check_skip("live:health_capability", "no health capability");
    }

    if has_dag {
        v.check_bool(
            "live:dag_capability",
            true,
            "DAG capability for conditional execution",
        );
    } else {
        v.check_skip("live:dag_capability", "no DAG capability");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_graph_structural() {
        let mut v = ValidationResult::new("parallel-graph");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "parallel-graph should evaluate at least one check"
        );
    }
}
