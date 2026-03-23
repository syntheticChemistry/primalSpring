// SPDX-License-Identifier: AGPL-3.0-or-later
//! exp069 — Graph-Driven Overlay Composition
//!
//! Validates the overlay composition model: tier-independent primals
//! (Squirrel, petalTongue) composed at any atomic tier via deploy
//! graphs rather than fixed enum gating.
//!
//! Tests:
//! 1. Graph structural validation for all overlay graphs
//! 2. Spawn filtering (spawn=true vs spawn=false)
//! 3. Capability map construction from overlay graphs
//! 4. Graph merge/compose (base + overlay)
//! 5. Live Tower + AI overlay composition (if binaries available)

use std::path::{Path, PathBuf};

use primalspring::coordination::AtomicType;
use primalspring::deploy::{
    graph_capability_map, graph_spawnable_primals, load_graph, merge_graphs, topological_waves,
    validate_structure,
};
use primalspring::harness::AtomicHarness;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn validate_overlay_structure(v: &mut ValidationResult) {
    println!("=== Phase 1: Overlay Graph Structural Validation ===\n");

    let overlay_graphs = [
        "tower_ai.toml",
        "tower_ai_viz.toml",
        "nest_viz.toml",
        "node_ai.toml",
    ];

    for name in &overlay_graphs {
        let path = graphs_dir().join(name);
        let result = validate_structure(&path);
        v.check_bool(
            &format!("parse_{name}"),
            result.parsed,
            &format!("{name} parses"),
        );
        v.check_bool(
            &format!("clean_{name}"),
            result.issues.is_empty(),
            &format!("{name} structural issues: {:?}", result.issues),
        );

        if result.parsed {
            let Some(graph) = load_graph(&path).ok() else {
                v.check_bool(
                    &format!("load_{name}"),
                    false,
                    &format!("load_graph failed for {name}"),
                );
                continue;
            };
            let Some(waves) = topological_waves(&graph).ok() else {
                v.check_bool(
                    &format!("topology_{name}"),
                    false,
                    &format!("topological_waves failed for {name}"),
                );
                continue;
            };
            v.check_minimum(&format!("waves_{name}"), waves.len(), 2);
            println!(
                "  {name}: {} nodes, {} waves, {} required",
                result.node_count,
                waves.len(),
                result.required_count
            );
        }
    }
}

fn validate_spawn_filtering(v: &mut ValidationResult) {
    println!("\n=== Phase 2: Spawn Filtering ===\n");

    let overlay_graphs = [
        "tower_ai.toml",
        "tower_ai_viz.toml",
        "nest_viz.toml",
        "node_ai.toml",
    ];

    for name in &overlay_graphs {
        let Some(graph) = load_graph(&graphs_dir().join(name)).ok() else {
            v.check_bool(
                &format!("load_{name}_phase2"),
                false,
                &format!("load_graph failed for {name}"),
            );
            continue;
        };
        let spawnable = graph_spawnable_primals(&graph);
        let total = graph.graph.node.len();
        let filtered = total - spawnable.len();
        v.check_bool(
            &format!("filter_{name}"),
            filtered > 0,
            &format!("{name}: {filtered} non-spawn nodes filtered"),
        );
        println!(
            "  {name}: {total} total, {} spawnable, {filtered} filtered",
            spawnable.len()
        );
    }
}

fn validate_capability_maps(v: &mut ValidationResult) {
    println!("\n=== Phase 3: Capability Map Construction ===\n");

    let Some(tower_ai) = load_graph(&graphs_dir().join("tower_ai.toml")).ok() else {
        v.check_bool("load_tower_ai", false, "load_graph tower_ai.toml");
        return;
    };
    let caps = graph_capability_map(&tower_ai);
    v.check_bool(
        "cap_security",
        caps.get("security").is_some_and(|v| v == "beardog"),
        "security -> beardog",
    );
    v.check_bool(
        "cap_discovery",
        caps.get("discovery").is_some_and(|v| v == "songbird"),
        "discovery -> songbird",
    );
    v.check_bool(
        "cap_ai",
        caps.get("ai").is_some_and(|v| v == "squirrel"),
        "ai -> squirrel",
    );
    println!("  tower_ai capability map: {caps:?}");

    let Some(node_ai) = load_graph(&graphs_dir().join("node_ai.toml")).ok() else {
        v.check_bool("load_node_ai", false, "load_graph node_ai.toml");
        return;
    };
    let node_caps = graph_capability_map(&node_ai);
    v.check_bool(
        "node_ai_has_compute",
        node_caps.contains_key("compute"),
        "node_ai has compute capability",
    );
    v.check_bool(
        "node_ai_has_ai",
        node_caps.contains_key("ai"),
        "node_ai has ai capability",
    );
}

fn validate_graph_merge(v: &mut ValidationResult) {
    println!("\n=== Phase 4: Graph Merge/Compose ===\n");

    let Some(base) = load_graph(&graphs_dir().join("tower_atomic_bootstrap.toml")).ok() else {
        v.check_bool(
            "load_base_graph",
            false,
            "load_graph tower_atomic_bootstrap.toml",
        );
        return;
    };
    let Some(overlay) = load_graph(&graphs_dir().join("tower_ai.toml")).ok() else {
        v.check_bool("load_overlay_graph", false, "load_graph tower_ai.toml");
        return;
    };
    let merged = merge_graphs(&base, &overlay);

    v.check_bool(
        "merge_name",
        merged.graph.name.contains('+'),
        "merged graph name contains +",
    );

    let Some(merged_waves) = topological_waves(&merged).ok() else {
        v.check_bool("merge_topology", false, "merged graph has invalid topology");
        return;
    };
    v.check_minimum("merge_waves", merged_waves.len(), 2);

    let all_names: Vec<String> = merged_waves.into_iter().flatten().collect();
    v.check_bool(
        "merge_has_beardog",
        all_names.contains(&"beardog".to_owned()),
        "merged has beardog",
    );
    v.check_bool(
        "merge_has_squirrel",
        all_names.contains(&"squirrel".to_owned()),
        "merged has squirrel (from overlay)",
    );
    println!("  merged: {} nodes, names: {all_names:?}", all_names.len());
}

fn validate_live_overlay(v: &mut ValidationResult) {
    println!("\n=== Phase 5: Live Tower + AI Overlay ===\n");

    let graph_path = graphs_dir().join("tower_ai.toml");
    let family_id = format!("exp069-{}", std::process::id());
    match AtomicHarness::with_graph(AtomicType::Tower, &graph_path).start(&family_id) {
        Ok(running) => {
            v.check_bool("overlay_start", true, "Tower+AI overlay started");
            v.check_minimum("overlay_primals", running.primal_count(), 2);

            let overlay_primals = running.overlay_primals();
            println!("  overlay primals: {overlay_primals:?}");

            let all_caps = running.all_capabilities();
            println!("  all capabilities: {all_caps:?}");
            v.check_bool(
                "overlay_has_security",
                all_caps.contains(&"security".to_owned()),
                "has security",
            );

            running.validate(v);

            if running.socket_for("ai").is_some() {
                println!("  AI capability socket resolved!");
                v.check_bool("overlay_ai_socket", true, "AI socket resolved");
            } else {
                v.check_skip(
                    "overlay_ai_socket",
                    "squirrel not available (binary may be missing)",
                );
            }
        }
        Err(e) => {
            println!("  overlay start failed (expected if squirrel binary missing): {e}");
            v.check_skip(
                "overlay_start",
                &format!("Tower+AI overlay could not start: {e}"),
            );
        }
    }
}

fn main() {
    ValidationResult::run_experiment(
        "exp069_graph_overlay_composition",
        "Graph-Driven Overlay Composition",
        |v| {
            validate_overlay_structure(v);
            validate_spawn_filtering(v);
            validate_capability_maps(v);
            validate_graph_merge(v);
            validate_live_overlay(v);
        },
    );
}
