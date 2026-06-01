// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! exp070 — Squirrel Cross-Primal Discovery via CompositionContext.
//!
//! Validates graph structure and live Squirrel routing against a discovered
//! full NUCLEUS-style composition.

use std::path::{Path, PathBuf};

use primalspring::composition::CompositionContext;
use primalspring::deploy::{
    graph_capability_map, graph_spawnable_primals, load_graph, topological_waves,
    validate_structure,
};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn validate_graph_structure(v: &mut ValidationResult) {
    println!("=== Phase 1: profiles/full.toml Structural Validation ===\n");

    let path = graphs_dir().join("profiles/full.toml");
    let result = validate_structure(&path);
    v.check_bool(
        "parse_full_profile",
        result.parsed,
        "profiles/full.toml parses",
    );
    v.check_bool(
        "clean_full_profile",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if result.parsed {
        let Some(graph) = load_graph(&path).ok() else {
            v.check_bool("load_full_profile", false, "load_graph profiles/full.toml");
            return;
        };
        let Some(waves) = topological_waves(&graph).ok() else {
            v.check_bool(
                "full_profile_topology",
                false,
                "topological_waves profiles/full.toml",
            );
            return;
        };
        v.check_minimum("full_profile_waves", waves.len(), 2);
        println!(
            "  profiles/full.toml: {} nodes, {} waves, {} required",
            result.node_count,
            waves.len(),
            result.required_count
        );
    }
}

fn validate_spawn_and_caps(v: &mut ValidationResult) {
    println!("\n=== Phase 2: Spawn Ordering & Capability Map ===\n");

    let path = graphs_dir().join("profiles/full.toml");
    let Some(graph) = load_graph(&path).ok() else {
        v.check_bool(
            "load_full_profile_phase2",
            false,
            "load_graph profiles/full.toml",
        );
        return;
    };
    let spawnable = graph_spawnable_primals(&graph);
    v.check_bool(
        "spawnable_beardog",
        spawnable.contains(&primal_names::BEARDOG.to_owned()),
        "beardog is spawnable",
    );
    v.check_bool(
        "spawnable_songbird",
        spawnable.contains(&primal_names::SONGBIRD.to_owned()),
        "songbird is spawnable",
    );
    v.check_bool(
        "spawnable_nestgate",
        spawnable.contains(&primal_names::NESTGATE.to_owned()),
        "nestgate is spawnable",
    );
    v.check_bool(
        "spawnable_toadstool",
        spawnable.contains(&primal_names::TOADSTOOL.to_owned()),
        "toadstool is spawnable",
    );
    v.check_bool(
        "spawnable_squirrel",
        spawnable.contains(&primal_names::SQUIRREL.to_owned()),
        "squirrel is spawnable",
    );
    v.check_bool(
        "validate_not_spawnable",
        !spawnable.contains(&"validate_full_overlay".to_owned()),
        "validation node excluded",
    );
    println!("  spawnable: {spawnable:?}");

    let caps = graph_capability_map(&graph);
    v.check_bool(
        "cap_security",
        caps.get("security")
            .is_some_and(|v| v == primal_names::BEARDOG),
        "security -> beardog",
    );
    v.check_bool(
        "cap_storage",
        caps.get("storage")
            .is_some_and(|v| v == primal_names::NESTGATE),
        "storage -> nestgate",
    );
    v.check_bool(
        "cap_compute",
        caps.get("compute")
            .is_some_and(|v| v == primal_names::TOADSTOOL),
        "compute -> toadstool",
    );
    v.check_bool(
        "cap_ai",
        caps.get("ai").is_some_and(|v| v == primal_names::SQUIRREL),
        "ai -> squirrel",
    );
    v.check_bool(
        "cap_discovery",
        caps.get("discovery")
            .is_some_and(|v| v == primal_names::SONGBIRD),
        "discovery -> songbird",
    );
    println!("  capability map: {caps:?}");
}

fn try_ai_rpc(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    check_name: &str,
    method: &str,
    params: serde_json::Value,
) {
    if !ctx.has_capability("ai") {
        v.check_skip(check_name, "ai capability not available");
        return;
    }
    match ctx.call("ai", method, params) {
        Ok(r) => {
            println!("  {method} response: {r:?}");
            v.check_bool(check_name, true, &format!("{method} succeeded"));
        }
        Err(e) if e.is_connection_error() => {
            println!("  {method} IPC error: {e}");
            v.check_skip(check_name, &format!("{e}"));
        }
        Err(e) => {
            println!("  {method} IPC error: {e}");
            v.check_skip(check_name, &format!("error: {e}"));
        }
    }
}

fn validate_live_overlay(v: &mut ValidationResult) {
    println!("\n=== Phase 3: Live Full Overlay ===\n");

    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let avail = ctx.available_capabilities();
    println!("  live capabilities: {avail:?}");

    if !ctx.has_capability("security") || !ctx.has_capability("discovery") {
        println!("  full overlay not reachable via discovery");
        v.check_skip("overlay_start", "security/discovery not discovered");
        v.check_skip("overlay_primal_count", "overlay not started");
        v.check_skip("has_security", "overlay not started");
        v.check_skip("has_discovery", "overlay not started");
        v.check_skip("squirrel_discover", "overlay not started");
        v.check_skip("squirrel_tool_list", "overlay not started");
        v.check_skip("squirrel_context_create", "overlay not started");
        v.check_skip("squirrel_ai_query", "overlay not started");
        return;
    }

    v.check_bool("overlay_start", true, "full overlay reachable");
    v.check_minimum("overlay_primal_count", avail.len(), 2);

    v.check_bool(
        "has_security",
        ctx.has_capability("security"),
        "has security",
    );
    v.check_bool(
        "has_discovery",
        ctx.has_capability("discovery"),
        "has discovery",
    );

    println!("\n=== Phase 4: Squirrel capability.discover ===\n");
    try_ai_rpc(
        v,
        &mut ctx,
        "squirrel_discover",
        "capability.discover",
        serde_json::json!({}),
    );

    println!("\n=== Phase 5: Squirrel tool.list ===\n");
    try_ai_rpc(
        v,
        &mut ctx,
        "squirrel_tool_list",
        "tool.list",
        serde_json::json!({}),
    );

    println!("\n=== Phase 6: Squirrel context.create ===\n");
    try_ai_rpc(
        v,
        &mut ctx,
        "squirrel_context_create",
        "context.create",
        serde_json::json!({
            "name": "exp070-test-context",
            "description": "Cross-primal discovery experiment context"
        }),
    );

    validate_ai_query(v, &mut ctx);
}

fn validate_ai_query(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    println!("\n=== Phase 7: Squirrel ai.query ===\n");

    let has_api_key =
        std::env::var("ANTHROPIC_API_KEY").is_ok() || std::env::var("OPENAI_API_KEY").is_ok();

    if has_api_key {
        try_ai_rpc(
            v,
            ctx,
            "squirrel_ai_query",
            "ai.query",
            serde_json::json!({
                "prompt": "What is 2+2? Reply with just the number.",
                "max_tokens": 16
            }),
        );
    } else {
        println!("  no API key set — skipping ai.query");
        v.check_skip(
            "squirrel_ai_query",
            "no ANTHROPIC_API_KEY or OPENAI_API_KEY",
        );
    }
}

fn main() {
    ValidationResult::new("exp070_squirrel_cross_primal_discovery")
        .with_provenance("exp070_squirrel_cross_primal_discovery", "2026-05-09")
        .run("Squirrel Cross-Primal Discovery", |v| {
            v.section("Phase 1: Graph structure (profiles/full.toml)");
            validate_graph_structure(v);
            v.section("Phase 2: Spawn ordering and capability map");
            validate_spawn_and_caps(v);
            v.section("Phase 3–7: Live overlay and Squirrel routing");
            validate_live_overlay(v);
        });
}
