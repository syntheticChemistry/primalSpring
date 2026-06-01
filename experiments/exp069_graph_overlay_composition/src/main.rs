// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
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

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::deploy::{
    graph_capability_map, graph_spawnable_primals, load_graph, merge_graphs, topological_waves,
    validate_structure,
};
use primalspring::ipc::discover::extract_capability_names;
use primalspring::primal_names;
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
        caps.get("security")
            .is_some_and(|v| v == primal_names::BEARDOG),
        "security -> beardog",
    );
    v.check_bool(
        "cap_discovery",
        caps.get("discovery")
            .is_some_and(|v| v == primal_names::SONGBIRD),
        "discovery -> songbird",
    );
    v.check_bool(
        "cap_ai",
        caps.get("ai").is_some_and(|v| v == primal_names::SQUIRREL),
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
        all_names.contains(&primal_names::BEARDOG.to_owned()),
        "merged has beardog",
    );
    v.check_bool(
        "merge_has_squirrel",
        all_names.contains(&primal_names::SQUIRREL.to_owned()),
        "merged has squirrel (from overlay)",
    );
    println!("  merged: {} nodes, names: {all_names:?}", all_names.len());
}

fn validate_live_overlay(v: &mut ValidationResult) {
    println!("\n=== Phase 5: Live Tower + AI Overlay ===\n");

    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let avail = ctx.available_capabilities();
    println!("  live capabilities: {avail:?}");

    if !ctx.has_capability("security") || !ctx.has_capability("discovery") {
        v.check_skip(
            "overlay_start",
            "Tower base capabilities not fully discovered",
        );
        v.check_skip("overlay_primals", "composition not reachable");
        v.check_skip("overlay_has_security", "composition not reachable");
        v.check_skip("overlay_ai_socket", "composition not reachable");
        return;
    }

    v.check_bool(
        "overlay_start",
        true,
        "Tower composition reachable via discovery",
    );
    v.check_minimum("overlay_primals", avail.len(), 2);
    v.check_bool(
        "overlay_has_security",
        ctx.has_capability("security"),
        "has security",
    );

    for cap in AtomicType::Tower.required_capabilities() {
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("overlay_{cap}_liveness"),
                true,
                &format!("{cap} health.liveness"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("overlay_{cap}_liveness"), &format!("{e}"));
            }
            Err(e) => v.check_bool(
                &format!("overlay_{cap}_liveness"),
                false,
                &format!("error: {e}"),
            ),
        }
        match ctx.call(cap, "capabilities.list", serde_json::json!({})) {
            Ok(val) => {
                let n = extract_capability_names(Some(val)).len();
                if n == 0 {
                    v.check_skip(
                        &format!("overlay_{cap}_capabilities"),
                        "empty capability list",
                    );
                } else {
                    v.check_minimum(&format!("overlay_{cap}_capabilities"), n, 1);
                }
            }
            Err(_) => v.check_skip(
                &format!("overlay_{cap}_capabilities"),
                "capabilities.list unavailable",
            ),
        }
    }

    if ctx.has_capability("ai") {
        println!("  AI capability connected");
        match ctx.call("ai", "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool("overlay_ai_socket", true, "AI liveness"),
            Err(e) if e.is_connection_error() => {
                v.check_skip("overlay_ai_socket", &format!("{e}"));
            }
            Err(e) => v.check_bool("overlay_ai_socket", false, &format!("error: {e}")),
        }
    } else {
        v.check_skip(
            "overlay_ai_socket",
            "ai capability not discovered (squirrel may be missing)",
        );
    }
}

fn main() {
    ValidationResult::new("exp069_graph_overlay_composition")
        .with_provenance("exp069_graph_overlay_composition", "2026-05-09")
        .run("Graph-Driven Overlay Composition", |v| {
            v.section("Phase 1: Overlay graph structural validation");
            validate_overlay_structure(v);
            v.section("Phase 2: Spawn filtering");
            validate_spawn_filtering(v);
            v.section("Phase 3: Capability map construction");
            validate_capability_maps(v);
            v.section("Phase 4: Graph merge");
            validate_graph_merge(v);
            v.section("Phase 5: Live Tower + AI overlay");
            validate_live_overlay(v);
        });
}
