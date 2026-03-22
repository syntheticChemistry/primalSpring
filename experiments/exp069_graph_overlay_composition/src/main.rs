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

fn main() {
    let mut v = ValidationResult::new("exp069_graph_overlay_composition");

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
            let graph = load_graph(&path).unwrap();
            let waves = topological_waves(&graph).unwrap();
            v.check_minimum(&format!("waves_{name}"), waves.len(), 2);
            println!(
                "  {name}: {} nodes, {} waves, {} required",
                result.node_count,
                waves.len(),
                result.required_count
            );
        }
    }

    println!("\n=== Phase 2: Spawn Filtering ===\n");

    for name in &overlay_graphs {
        let graph = load_graph(&graphs_dir().join(name)).unwrap();
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

    println!("\n=== Phase 3: Capability Map Construction ===\n");

    let tower_ai = load_graph(&graphs_dir().join("tower_ai.toml")).unwrap();
    let caps = graph_capability_map(&tower_ai);
    v.check_bool(
        "cap_security",
        caps.get("security").map_or(false, |v| v == "beardog"),
        "security -> beardog",
    );
    v.check_bool(
        "cap_discovery",
        caps.get("discovery").map_or(false, |v| v == "songbird"),
        "discovery -> songbird",
    );
    v.check_bool(
        "cap_ai",
        caps.get("ai").map_or(false, |v| v == "squirrel"),
        "ai -> squirrel",
    );
    println!("  tower_ai capability map: {caps:?}");

    let node_ai = load_graph(&graphs_dir().join("node_ai.toml")).unwrap();
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

    println!("\n=== Phase 4: Graph Merge/Compose ===\n");

    let base = load_graph(&graphs_dir().join("tower_atomic_bootstrap.toml")).unwrap();
    let overlay = load_graph(&graphs_dir().join("tower_ai.toml")).unwrap();
    let merged = merge_graphs(&base, &overlay);

    v.check_bool(
        "merge_name",
        merged.graph.name.contains('+'),
        "merged graph name contains +",
    );

    let merged_waves = topological_waves(&merged).unwrap();
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

            running.validate(&mut v);

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

    println!("\n=== Summary ===\n");
    v.summary();
}
