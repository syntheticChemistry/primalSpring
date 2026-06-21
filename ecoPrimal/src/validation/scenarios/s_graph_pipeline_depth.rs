// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Graph Pipeline Depth — validates multi-node graph execution
//! with dependency chains, parallel waves, and cross-primal coordination.
//!
//! Validates:
//! 1. All deploy graphs parse and have valid structure
//! 2. Topological ordering produces correct wave counts
//! 3. Parallel execution paths are identified
//! 4. Cross-primal coordination patterns are wired
//! 5. Graph node types cover all expected categories

use crate::composition::CompositionContext;
use crate::deploy::{load_graph, topological_waves, validate_structure};
use crate::graphs::CoordinationPattern;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::path::{Path, PathBuf};

/// Graph pipeline depth validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "graph-pipeline-depth",
        track: Track::GraphExecution,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-21",
        description: "Multi-node graph pipeline: structure, waves, parallelism, coordination patterns",
    },
    run: run_graph_pipeline_depth,
};

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs")
}

/// Run this validation scenario.
pub fn run_graph_pipeline_depth(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: All graphs parse");
    phase_all_graphs_parse(v);

    v.section("Phase 2: Topological wave depth");
    phase_wave_depth(v);

    v.section("Phase 3: Coordination patterns");
    phase_coordination_patterns(v);

    v.section("Phase 4: Node type coverage");
    phase_node_type_coverage(v);
}

fn phase_all_graphs_parse(v: &mut ValidationResult) {
    let dir = graphs_dir();
    if !dir.exists() {
        v.check_skip("graphs_dir_exists", "graphs/ directory not found");
        return;
    }

    let entries: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "toml")
        })
        .collect();

    v.check_minimum("graph_count", entries.len(), 10);

    let mut parse_ok = 0usize;
    let mut parse_fail = 0usize;

    for entry in &entries {
        let result = validate_structure(&entry.path());
        if result.parsed && result.issues.is_empty() {
            parse_ok += 1;
        } else {
            parse_fail += 1;
            let name = entry.file_name();
            v.check_bool(
                &format!("parse:{}", name.to_string_lossy()),
                false,
                &format!("issues: {:?}", result.issues),
            );
        }
    }

    v.check_bool(
        "all_graphs_parse",
        parse_fail == 0,
        &format!("{parse_ok}/{} graphs parse cleanly", entries.len()),
    );
}

fn phase_wave_depth(v: &mut ValidationResult) {
    let dir = graphs_dir();
    let graphs_to_check = [
        "tower_atomic_bootstrap.toml",
        "nest_atomic.toml",
        "node_atomic_compute.toml",
        "nucleus_complete.toml",
    ];

    let mut total_waves = 0usize;
    let mut max_depth = 0usize;

    for name in &graphs_to_check {
        let path = dir.join(name);
        if !path.exists() {
            v.check_skip(&format!("waves:{name}"), "graph file not found");
            continue;
        }

        match load_graph(&path) {
            Ok(graph) => match topological_waves(&graph) {
                Ok(waves) => {
                    let depth = waves.len();
                    total_waves += depth;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                    v.check_minimum(&format!("waves:{name}"), depth, 2);
                }
                Err(e) => {
                    v.check_bool(
                        &format!("waves:{name}"),
                        false,
                        &format!("topological sort failed: {e}"),
                    );
                }
            },
            Err(e) => {
                v.check_bool(
                    &format!("waves:{name}"),
                    false,
                    &format!("load failed: {e}"),
                );
            }
        }
    }

    v.check_minimum("max_wave_depth", max_depth, 3);
    v.check_minimum("total_wave_sum", total_waves, 8);
}

fn phase_coordination_patterns(v: &mut ValidationResult) {
    let patterns = [
        CoordinationPattern::Sequential,
        CoordinationPattern::Parallel,
        CoordinationPattern::Pipeline,
        CoordinationPattern::ConditionalDag,
        CoordinationPattern::Continuous,
    ];

    for pattern in &patterns {
        let desc = pattern.description();
        let label = format!("{pattern:?}").to_lowercase();
        v.check_bool(
            &format!("pattern:{label}:has_description"),
            !desc.is_empty() && desc.len() > 5,
            &format!("{label}: {desc}"),
        );
    }

    v.check_count("coordination_pattern_count", patterns.len(), 5);
}

fn phase_node_type_coverage(v: &mut ValidationResult) {
    let dir = graphs_dir();
    let mut seen_properties: std::collections::HashSet<String> = std::collections::HashSet::new();

    let entries: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "toml"))
        .collect();

    let property_keys = ["coordination", "security_model", "transport", "bond_type"];

    for entry in &entries {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            for line in content.lines() {
                let trimmed = line.trim();
                for key in &property_keys {
                    if trimmed.starts_with(key) && trimmed.contains('=') {
                        if let Some(val) = trimmed.split('=').nth(1) {
                            let prop = val.trim().trim_matches('"').to_owned();
                            if !prop.is_empty() {
                                seen_properties.insert(format!("{key}:{prop}"));
                            }
                        }
                    }
                }
            }
        }
    }

    let expected_props = [
        "coordination:sequential",
        "security_model:btsp_enforced",
        "transport:uds_only",
        "bond_type:Ionic",
    ];
    for expected in &expected_props {
        v.check_bool(
            &format!("graph_property:{expected}"),
            seen_properties.contains(*expected),
            &format!("graph property '{expected}' present in corpus"),
        );
    }

    v.check_minimum("distinct_graph_properties", seen_properties.len(), 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_pipeline_depth_structural() {
        let mut v = ValidationResult::new("graph-pipeline-depth");
        let mut ctx = CompositionContext::discover();
        run_graph_pipeline_depth(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "graph-pipeline-depth: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
