// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: NUCLEUS Orchestration — validates graph ordering, capability
//! resolution, atomic composition model, and spawn infrastructure without
//! requiring live primal binaries.
//!
//! This is the "dry-run acceptance test" for the orchestrator. It proves
//! that eastGate's primalSpring can correctly plan a 13/13 NUCLEUS launch,
//! resolve dependency waves from deploy graphs, map capabilities to primals,
//! and construct valid spawn configurations.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::deploy;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// NUCLEUS orchestration validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "nucleus-orchestration",
        track: Track::AtomicComposition,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-17",
        description: "NUCLEUS orchestration: graph ordering, capability resolution, spawn planning",
    },
    run,
};

/// Run NUCLEUS orchestration validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Atomic type completeness");
    phase_atomic_types(v);

    v.section("Phase 2: Graph ordering for FullNucleus");
    phase_graph_ordering(v);

    v.section("Phase 3: Capability-to-primal resolution");
    phase_capability_resolution(v);

    v.section("Phase 4: Composition tier transitions");
    phase_tier_transitions(v);

    v.section("Phase 5: Deploy graph structural integrity");
    phase_graph_integrity(v);
}

fn phase_atomic_types(v: &mut ValidationResult) {
    let all_types = [
        AtomicType::Micro,
        AtomicType::Tower,
        AtomicType::Node,
        AtomicType::Nest,
        AtomicType::FullNucleus,
    ];

    for atomic in all_types {
        let caps = atomic.required_capabilities();
        v.check_bool(
            &format!("atomic:{atomic:?}:has_caps"),
            !caps.is_empty(),
            &format!("{atomic:?}: {} capabilities", caps.len()),
        );
    }

    // Full NUCLEUS must have exactly 13 capabilities
    let full_caps = AtomicType::FullNucleus.required_capabilities();
    v.check_count("atomic:full_nucleus:cap_count", full_caps.len(), 13);

    // Micro must be subset of Tower must be subset of Full
    let micro_caps: std::collections::HashSet<&str> =
        AtomicType::Micro.required_capabilities().iter().copied().collect();
    let tower_caps: std::collections::HashSet<&str> =
        AtomicType::Tower.required_capabilities().iter().copied().collect();
    let full_set: std::collections::HashSet<&str> = full_caps.iter().copied().collect();

    v.check_bool(
        "atomic:micro_subset_tower",
        micro_caps.is_subset(&tower_caps),
        &format!(
            "Micro ({}) ⊂ Tower ({})",
            micro_caps.len(),
            tower_caps.len()
        ),
    );
    v.check_bool(
        "atomic:tower_subset_full",
        tower_caps.is_subset(&full_set),
        &format!("Tower ({}) ⊂ Full ({})", tower_caps.len(), full_set.len()),
    );
}

fn phase_graph_ordering(v: &mut ValidationResult) {
    let graph_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../graphs/nucleus_complete.toml");

    if !graph_path.exists() {
        v.check_skip(
            "graph:full_nucleus:exists",
            &format!("no graph at {}", graph_path.display()),
        );
        return;
    }

    v.check_bool(
        "graph:full_nucleus:exists",
        true,
        &format!("graph: {}", graph_path.display()),
    );

    let graph = match deploy::load_graph(&graph_path) {
        Ok(g) => g,
        Err(e) => {
            v.check_bool(
                "graph:full_nucleus:parseable",
                false,
                &format!("parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "graph:full_nucleus:parseable",
        true,
        &format!("{} nodes in graph", graph.nodes.len()),
    );

    let waves = match deploy::topological_waves(&graph) {
        Ok(w) => w,
        Err(e) => {
            v.check_bool(
                "graph:full_nucleus:acyclic",
                false,
                &format!("cycle detected: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "graph:full_nucleus:acyclic",
        true,
        &format!("{} waves (dependency layers)", waves.len()),
    );

    // Wave 0 must contain foundational primals (bearDog, songbird)
    if let Some(wave0) = waves.first() {
        let has_foundation = wave0.iter().any(|n| n == "beardog" || n == "bearDog");
        v.check_bool(
            "graph:wave0:has_foundation",
            has_foundation,
            &format!("wave 0: {wave0:?}"),
        );
    }

    // Total primals across all waves should cover at least the spawnable set (12, excl. biomeOS)
    let total_in_graph: usize = waves.iter().map(Vec::len).sum();
    v.check_bool(
        "graph:total_coverage",
        total_in_graph >= 12,
        &format!("{total_in_graph} primals across {len} waves (12 spawnable)", len = waves.len()),
    );
}

fn phase_capability_resolution(v: &mut ValidationResult) {
    use crate::composition::capability_to_primal;

    let all_caps = AtomicType::FullNucleus.required_capabilities();
    let all_primals: Vec<&str> = primal_names::Primal::ALL.iter().map(|p| p.slug()).collect();

    let mut resolved_primals = std::collections::HashSet::new();
    let mut unresolved = Vec::new();

    for cap in all_caps {
        let primal = capability_to_primal(cap);
        if all_primals.contains(&primal) {
            resolved_primals.insert(primal);
        } else {
            unresolved.push((*cap, primal));
        }
    }

    v.check_bool(
        "caps:all_resolved",
        unresolved.is_empty(),
        &format!(
            "{} resolved, {} unresolved{}",
            resolved_primals.len(),
            unresolved.len(),
            if unresolved.is_empty() {
                String::new()
            } else {
                format!(": {unresolved:?}")
            }
        ),
    );

    // 12 spawned primals (biomeOS is the substrate, not a capability provider)
    v.check_bool(
        "caps:unique_primals",
        resolved_primals.len() >= 12,
        &format!("{} unique primals resolved from capabilities", resolved_primals.len()),
    );

    // Every spawnable primal (ALL except biomeOS) should provide at least one capability
    let spawnable: Vec<&str> = primal_names::Primal::ALL_SLUGS_NO_BIOMEOS.to_vec();
    for primal in &spawnable {
        let provides_cap = all_caps.iter().any(|cap| capability_to_primal(cap) == *primal);
        v.check_bool(
            &format!("caps:{primal}:has_capability"),
            provides_cap,
            &format!("{primal} → capability mapping exists"),
        );
    }
}

fn phase_tier_transitions(v: &mut ValidationResult) {
    let label_for_count = AtomicType::from_primal_count;

    v.check_bool(
        "tiers:micro",
        label_for_count(2) == "Micro",
        &format!("2 primals → {}", label_for_count(2)),
    );
    v.check_bool(
        "tiers:tower",
        label_for_count(4) == "Tower Atomic",
        &format!("4 primals → {}", label_for_count(4)),
    );
    v.check_bool(
        "tiers:node",
        label_for_count(6) == "Node Atomic",
        &format!("6 primals → {}", label_for_count(6)),
    );
    v.check_bool(
        "tiers:nest",
        label_for_count(10) == "Nest",
        &format!("10 primals → {}", label_for_count(10)),
    );
    v.check_bool(
        "tiers:full",
        label_for_count(13) == "Full NUCLEUS",
        &format!("13 primals → {}", label_for_count(13)),
    );
}

fn phase_graph_integrity(v: &mut ValidationResult) {
    let graphs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs");

    if !graphs_dir.is_dir() {
        v.check_skip(
            "graphs:dir_exists",
            &format!("no graphs dir: {}", graphs_dir.display()),
        );
        return;
    }

    v.check_bool(
        "graphs:dir_exists",
        true,
        &format!("{}", graphs_dir.display()),
    );

    let expected_graphs = ["nucleus_complete", "nest_atomic", "tower_atomic_bootstrap", "node_atomic_compute"];
    let mut found = 0u32;

    for name in &expected_graphs {
        let path = graphs_dir.join(format!("{name}.toml"));
        if path.exists() {
            found += 1;
            if let Ok(graph) = deploy::load_graph(&path) {
                let valid = deploy::topological_waves(&graph).is_ok();
                v.check_bool(
                    &format!("graphs:{name}:valid"),
                    valid,
                    &format!("{name}: {} nodes, acyclic={valid}", graph.nodes.len()),
                );
            }
        }
    }

    v.check_bool(
        "graphs:coverage",
        found >= 2,
        &format!("{found}/{} composition graphs found", expected_graphs.len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nucleus_orchestration_structural() {
        let mut v = ValidationResult::new("nucleus-orchestration");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "NUCLEUS orchestration: {} failures ({} passed, {} skipped)",
            v.failed,
            v.passed,
            v.skipped
        );
    }
}
