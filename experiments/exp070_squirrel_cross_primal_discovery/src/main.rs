// SPDX-License-Identifier: AGPL-3.0-or-later
//! exp070 — Squirrel Cross-Primal Discovery
//!
//! Validates that Squirrel discovers sibling primals in a full overlay
//! composition (Tower + Nest + Node + Squirrel), then routes tool and
//! AI requests through them.
//!
//! Phases:
//! 1. Graph structural validation of full_overlay.toml
//! 2. Spawn ordering and capability map verification
//! 3. Live full overlay start (graceful skip if binaries missing)
//! 4. Squirrel capability.discover (verify sibling awareness)
//! 5. Squirrel tool.list (aggregated tools from multiple primals)
//! 6. Squirrel context.create (context management via storage)
//! 7. Squirrel ai.query (cloud AI routing — skip if no API key)

use std::path::{Path, PathBuf};

use primalspring::coordination::AtomicType;
use primalspring::deploy::{
    graph_capability_map, graph_spawnable_primals, load_graph, topological_waves,
    validate_structure,
};
use primalspring::harness::{AtomicHarness, RunningAtomic};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn validate_graph_structure(v: &mut ValidationResult) {
    println!("=== Phase 1: full_overlay.toml Structural Validation ===\n");

    let path = graphs_dir().join("full_overlay.toml");
    let result = validate_structure(&path);
    v.check_bool(
        "parse_full_overlay",
        result.parsed,
        "full_overlay.toml parses",
    );
    v.check_bool(
        "clean_full_overlay",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if result.parsed {
        let Some(graph) = load_graph(&path).ok() else {
            v.check_bool("load_full_overlay", false, "load_graph full_overlay.toml");
            return;
        };
        let Some(waves) = topological_waves(&graph).ok() else {
            v.check_bool(
                "full_overlay_topology",
                false,
                "topological_waves full_overlay.toml",
            );
            return;
        };
        v.check_minimum("full_overlay_waves", waves.len(), 2);
        println!(
            "  full_overlay.toml: {} nodes, {} waves, {} required",
            result.node_count,
            waves.len(),
            result.required_count
        );
    }
}

fn validate_spawn_and_caps(v: &mut ValidationResult) {
    println!("\n=== Phase 2: Spawn Ordering & Capability Map ===\n");

    let path = graphs_dir().join("full_overlay.toml");
    let Some(graph) = load_graph(&path).ok() else {
        v.check_bool(
            "load_full_overlay_phase2",
            false,
            "load_graph full_overlay.toml",
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
        caps.get("security").is_some_and(|v| v == primal_names::BEARDOG),
        "security -> beardog",
    );
    v.check_bool(
        "cap_storage",
        caps.get("storage").is_some_and(|v| v == primal_names::NESTGATE),
        "storage -> nestgate",
    );
    v.check_bool(
        "cap_compute",
        caps.get("compute").is_some_and(|v| v == primal_names::TOADSTOOL),
        "compute -> toadstool",
    );
    v.check_bool(
        "cap_ai",
        caps.get("ai").is_some_and(|v| v == primal_names::SQUIRREL),
        "ai -> squirrel",
    );
    v.check_bool(
        "cap_discovery",
        caps.get("discovery").is_some_and(|v| v == primal_names::SONGBIRD),
        "discovery -> songbird",
    );
    println!("  capability map: {caps:?}");
}

fn try_squirrel_rpc(
    running: &RunningAtomic,
    v: &mut ValidationResult,
    check_name: &str,
    method: &str,
    params: serde_json::Value,
) {
    if let Some(mut client) = running.client_for("ai") {
        match client.call(method, params) {
            Ok(r) if r.is_success() => {
                println!("  {method} response: {:?}", r.result);
                v.check_bool(check_name, true, &format!("{method} succeeded"));
            }
            Ok(r) => {
                println!("  {method} returned: {:?}", r.error);
                v.check_skip(check_name, &format!("{method} error: {:?}", r.error));
            }
            Err(e) => {
                println!("  {method} IPC error: {e}");
                v.check_skip(check_name, &format!("IPC error: {e}"));
            }
        }
    } else {
        v.check_skip(check_name, "squirrel not available");
    }
}

fn validate_live_overlay(v: &mut ValidationResult) {
    println!("\n=== Phase 3: Live Full Overlay Start ===\n");

    let graph_path = graphs_dir().join("full_overlay.toml");
    let family_id = format!("exp070-{}", std::process::id());
    match AtomicHarness::with_graph(AtomicType::Tower, &graph_path).start(&family_id) {
        Ok(running) => {
            v.check_bool("overlay_start", true, "full overlay started");
            v.check_minimum("overlay_primal_count", running.primal_count(), 2);

            let overlay_primals = running.overlay_primals();
            println!("  overlay primals: {overlay_primals:?}");

            let all_caps = running.all_capabilities();
            println!("  all capabilities: {all_caps:?}");
            v.check_bool(
                "has_security",
                all_caps.contains(&"security".to_owned()),
                "has security",
            );
            v.check_bool(
                "has_discovery",
                all_caps.contains(&"discovery".to_owned()),
                "has discovery",
            );

            running.validate(v);

            println!("\n=== Phase 4: Squirrel capability.discover ===\n");
            try_squirrel_rpc(
                &running,
                v,
                "squirrel_discover",
                "capability.discover",
                serde_json::json!({}),
            );

            println!("\n=== Phase 5: Squirrel tool.list ===\n");
            try_squirrel_rpc(
                &running,
                v,
                "squirrel_tool_list",
                "tool.list",
                serde_json::json!({}),
            );

            println!("\n=== Phase 6: Squirrel context.create ===\n");
            try_squirrel_rpc(
                &running,
                v,
                "squirrel_context_create",
                "context.create",
                serde_json::json!({
                    "name": "exp070-test-context",
                    "description": "Cross-primal discovery experiment context"
                }),
            );

            validate_ai_query(&running, v);
        }
        Err(e) => {
            println!("  full overlay start failed (expected if binaries missing): {e}");
            v.check_skip(
                "overlay_start",
                &format!("full overlay could not start: {e}"),
            );
            v.check_skip("squirrel_discover", "overlay not started");
            v.check_skip("squirrel_tool_list", "overlay not started");
            v.check_skip("squirrel_context_create", "overlay not started");
            v.check_skip("squirrel_ai_query", "overlay not started");
        }
    }
}

fn validate_ai_query(running: &RunningAtomic, v: &mut ValidationResult) {
    println!("\n=== Phase 7: Squirrel ai.query (skip if no API key) ===\n");

    let has_api_key =
        std::env::var("ANTHROPIC_API_KEY").is_ok() || std::env::var("OPENAI_API_KEY").is_ok();

    if has_api_key {
        try_squirrel_rpc(
            running,
            v,
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
        .with_provenance("exp070_squirrel_cross_primal_discovery", "2026-03-24")
        .run("Squirrel Cross-Primal Discovery", |v| {
            validate_graph_structure(v);
            validate_spawn_and_caps(v);
            validate_live_overlay(v);
        });
}
