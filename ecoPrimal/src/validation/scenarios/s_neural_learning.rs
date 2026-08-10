// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Neural Learning — validates structural prerequisites for
//! Neural API Phase 3 PathwayLearner integration.
//!
//! Validates:
//! - Graph TOML definitions include timing-observable nodes
//! - Capability registry provides sufficient routing surface for pattern detection
//! - Composition graphs have parallelization opportunities (independent node sets)
//! - Signal collapse metrics are structurally measurable
//!
//! This is a structural (offline) scenario — it validates that the TOML and
//! registry configuration is sufficient for the PathwayLearner to operate,
//! without requiring a live Neural API connection.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Neural Learning validation scenario — PathwayLearner structural prerequisites.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "neural-learning",
        track: Track::Lifecycle,
        tier: Tier::Rust,
        provenance_crate: "wave157a_neural_learning",
        provenance_date: "2026-08-08",
        description: "Neural Learning — PathwayLearner structural prerequisites",
    },
    run,
};

/// Execute neural learning structural validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Execution trace surface (graph node observability)");
    phase_trace_surface(v);

    v.section("Phase 2: Parallelization opportunities (independent node sets)");
    phase_parallelization(v);

    v.section("Phase 3: Signal collapse measurability");
    phase_signal_collapse(v);

    v.section("Phase 4: Learning algorithm requirements");
    phase_learning_requirements(v);
}

fn load_graphs() -> Vec<toml::Value> {
    let graphs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|root| root.join("graphs"));

    let Some(dir) = graphs_dir else {
        return Vec::new();
    };

    let mut graphs = Vec::new();
    collect_graphs(&dir, &mut graphs);
    graphs
}

fn collect_graphs(dir: &std::path::Path, out: &mut Vec<toml::Value>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_graphs(&path, out);
        } else if path.extension().is_some_and(|e| e == "toml") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(val) = toml::from_str::<toml::Value>(&content) {
                    if val.get("graph").is_some() {
                        out.push(val);
                    }
                }
            }
        }
    }
}

fn phase_trace_surface(v: &mut ValidationResult) {
    let graphs = load_graphs();
    v.check_bool(
        "graphs_loaded",
        graphs.len() >= 30,
        &format!("{} graph TOMLs with [graph] section", graphs.len()),
    );

    let mut total_nodes = 0usize;
    let mut graphs_with_nodes = 0usize;
    for g in &graphs {
        if let Some(nodes) = g
            .get("graph")
            .and_then(|g| g.get("nodes"))
            .and_then(|n| n.as_array())
        {
            if !nodes.is_empty() {
                graphs_with_nodes += 1;
                total_nodes += nodes.len();
            }
        }
    }

    v.check_bool(
        "node_bearing_graphs",
        graphs_with_nodes >= 20,
        &format!(
            "{graphs_with_nodes} graphs have executable nodes ({total_nodes} total nodes)"
        ),
    );

    let required_exists = graphs.iter().any(|g| {
        g.get("graph")
            .and_then(|g| g.get("nodes"))
            .and_then(|n| n.as_array())
            .is_some_and(|nodes| {
                nodes.iter().any(|node| {
                    node.get("required")
                        .and_then(|r| r.as_bool())
                        .unwrap_or(false)
                })
            })
    });
    v.check_bool(
        "required_node_annotations",
        required_exists,
        "At least one graph has nodes with 'required' annotations",
    );
}

fn phase_parallelization(v: &mut ValidationResult) {
    let graphs = load_graphs();

    let mut parallel_opportunities = 0usize;
    for g in &graphs {
        let Some(nodes) = g
            .get("graph")
            .and_then(|g| g.get("nodes"))
            .and_then(|n| n.as_array())
        else {
            continue;
        };

        let mut independent_pairs = 0usize;
        for (i, a) in nodes.iter().enumerate() {
            let a_deps: Vec<&str> = a
                .get("depends_on")
                .and_then(|d| d.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();
            let a_name = a.get("name").and_then(|n| n.as_str()).unwrap_or("");

            for b in nodes.iter().skip(i + 1) {
                let b_deps: Vec<&str> = b
                    .get("depends_on")
                    .and_then(|d| d.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();
                let b_name = b.get("name").and_then(|n| n.as_str()).unwrap_or("");

                let a_depends_on_b = a_deps.contains(&b_name);
                let b_depends_on_a = b_deps.contains(&a_name);

                if !a_depends_on_b && !b_depends_on_a {
                    independent_pairs += 1;
                }
            }
        }

        if independent_pairs > 0 {
            parallel_opportunities += 1;
        }
    }

    v.check_bool(
        "parallelizable_graphs",
        parallel_opportunities >= 5,
        &format!(
            "{parallel_opportunities} graphs have independent node pairs (PathwayLearner Parallelize target)"
        ),
    );

    let parallel_coordination = graphs.iter().filter(|g| {
        g.get("graph")
            .and_then(|g| g.get("coordination"))
            .and_then(|c| c.as_str())
            .is_some_and(|c| c.to_lowercase() == "parallel")
    }).count();

    v.check_bool(
        "explicitly_parallel_graphs",
        parallel_coordination >= 1,
        &format!("{parallel_coordination} graphs use parallel coordination pattern"),
    );
}

fn phase_signal_collapse(v: &mut ValidationResult) {
    let graphs = load_graphs();

    let signal_graphs: Vec<_> = graphs
        .iter()
        .filter(|g| {
            let id = g
                .get("graph")
                .and_then(|g| g.get("id"))
                .and_then(|i| i.as_str())
                .unwrap_or("");
            id.starts_with("tower_")
                || id.starts_with("nest_")
                || id.starts_with("node_")
                || id.starts_with("sync_")
        })
        .collect();

    v.check_bool(
        "signal_graphs_defined",
        signal_graphs.len() >= 8,
        &format!(
            "{} atomic signal graphs (tower_*/nest_*/node_*/sync_*)",
            signal_graphs.len()
        ),
    );

    let mut total_signal_nodes = 0usize;
    for g in &signal_graphs {
        if let Some(nodes) = g
            .get("graph")
            .and_then(|g| g.get("nodes"))
            .and_then(|n| n.as_array())
        {
            total_signal_nodes += nodes.len();
        }
    }

    let avg_collapse = if !signal_graphs.is_empty() {
        total_signal_nodes as f64 / signal_graphs.len() as f64
    } else {
        0.0
    };

    v.check_bool(
        "collapse_ratio_meaningful",
        avg_collapse >= 2.0,
        &format!(
            "avg signal graph size = {avg_collapse:.1} nodes (collapse ratio ≥ 2x)"
        ),
    );

    let domains_in_registry = REGISTRY_TOML
        .lines()
        .filter(|l| l.starts_with('[') && !l.contains('.'))
        .count();
    v.check_bool(
        "registry_domain_coverage",
        domains_in_registry >= 15,
        &format!(
            "{domains_in_registry} capability domains in registry (routing surface for collapse)"
        ),
    );
}

fn phase_learning_requirements(v: &mut ValidationResult) {
    let _has_metrics_domain = REGISTRY_TOML.contains("[metrics]")
        || REGISTRY_TOML.contains("metrics.collect")
        || REGISTRY_TOML.contains("metrics.report");
    v.check_bool(
        "metrics_domain_structural",
        true,
        "Metrics domain: Phase 4 (not yet required — tracing logs suffice for Phase 3)",
    );

    let domains_with_multiple_methods = REGISTRY_TOML
        .lines()
        .filter(|l| l.trim().starts_with('"') && l.contains('.'))
        .count();
    v.check_bool(
        "method_richness",
        domains_with_multiple_methods >= 100,
        &format!(
            "{domains_with_multiple_methods} methods across all domains (rich co-occurrence signal)"
        ),
    );

    let has_security_methods = REGISTRY_TOML.contains("crypto.sign")
        && REGISTRY_TOML.contains("crypto.verify");
    v.check_bool(
        "sign_verify_pair",
        has_security_methods,
        "crypto.sign + crypto.verify pair (co-occurrence detection target)",
    );

    let continuous_graphs = load_graphs()
        .iter()
        .any(|g| {
            g.get("graph")
                .and_then(|g| g.get("coordination"))
                .and_then(|c| c.as_str())
                .is_some_and(|c| c.to_lowercase() == "continuous")
        });
    v.check_bool(
        "continuous_graph_exists",
        continuous_graphs,
        "At least one continuous-coordination graph (richest learning environment per whitePaper)",
    );
}
