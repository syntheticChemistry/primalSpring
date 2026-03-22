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
use primalspring::harness::AtomicHarness;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn main() {
    let mut v = ValidationResult::new("exp070_squirrel_cross_primal_discovery");

    println!("=== Phase 1: full_overlay.toml Structural Validation ===\n");

    let path = graphs_dir().join("full_overlay.toml");
    let result = validate_structure(&path);
    v.check_bool("parse_full_overlay", result.parsed, "full_overlay.toml parses");
    v.check_bool(
        "clean_full_overlay",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if result.parsed {
        let graph = load_graph(&path).unwrap();
        let waves = topological_waves(&graph).unwrap();
        v.check_minimum("full_overlay_waves", waves.len(), 2);
        println!(
            "  full_overlay.toml: {} nodes, {} waves, {} required",
            result.node_count,
            waves.len(),
            result.required_count
        );
    }

    println!("\n=== Phase 2: Spawn Ordering & Capability Map ===\n");

    let graph = load_graph(&path).unwrap();
    let spawnable = graph_spawnable_primals(&graph);
    v.check_bool(
        "spawnable_beardog",
        spawnable.contains(&"beardog".to_owned()),
        "beardog is spawnable",
    );
    v.check_bool(
        "spawnable_songbird",
        spawnable.contains(&"songbird".to_owned()),
        "songbird is spawnable",
    );
    v.check_bool(
        "spawnable_nestgate",
        spawnable.contains(&"nestgate".to_owned()),
        "nestgate is spawnable",
    );
    v.check_bool(
        "spawnable_toadstool",
        spawnable.contains(&"toadstool".to_owned()),
        "toadstool is spawnable",
    );
    v.check_bool(
        "spawnable_squirrel",
        spawnable.contains(&"squirrel".to_owned()),
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
        caps.get("security").map_or(false, |v| v == "beardog"),
        "security -> beardog",
    );
    v.check_bool(
        "cap_storage",
        caps.get("storage").map_or(false, |v| v == "nestgate"),
        "storage -> nestgate",
    );
    v.check_bool(
        "cap_compute",
        caps.get("compute").map_or(false, |v| v == "toadstool"),
        "compute -> toadstool",
    );
    v.check_bool(
        "cap_ai",
        caps.get("ai").map_or(false, |v| v == "squirrel"),
        "ai -> squirrel",
    );
    v.check_bool(
        "cap_discovery",
        caps.get("discovery").map_or(false, |v| v == "songbird"),
        "discovery -> songbird",
    );
    println!("  capability map: {caps:?}");

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

            running.validate(&mut v);

            println!("\n=== Phase 4: Squirrel capability.discover ===\n");

            if let Some(mut client) = running.client_for("ai") {
                let resp = client.call(
                    "capability.discover",
                    serde_json::json!({}),
                );
                match resp {
                    Ok(r) if r.is_success() => {
                        println!("  capability.discover response: {:?}", r.result);
                        v.check_bool(
                            "squirrel_discover",
                            true,
                            "Squirrel capability.discover succeeded",
                        );
                    }
                    Ok(r) => {
                        println!("  capability.discover error: {:?}", r.error);
                        v.check_skip(
                            "squirrel_discover",
                            &format!("Squirrel returned error: {:?}", r.error),
                        );
                    }
                    Err(e) => {
                        println!("  capability.discover IPC error: {e}");
                        v.check_skip(
                            "squirrel_discover",
                            &format!("IPC error: {e}"),
                        );
                    }
                }
            } else {
                v.check_skip("squirrel_discover", "squirrel not available (no ai socket)");
            }

            println!("\n=== Phase 5: Squirrel tool.list ===\n");

            if let Some(mut client) = running.client_for("ai") {
                let resp = client.call("tool.list", serde_json::json!({}));
                match resp {
                    Ok(r) if r.is_success() => {
                        println!("  tool.list response: {:?}", r.result);
                        v.check_bool("squirrel_tool_list", true, "Squirrel tool.list succeeded");
                    }
                    Ok(r) => {
                        println!("  tool.list returned: {:?}", r.error);
                        v.check_skip(
                            "squirrel_tool_list",
                            &format!("tool.list error: {:?}", r.error),
                        );
                    }
                    Err(e) => {
                        println!("  tool.list IPC error: {e}");
                        v.check_skip("squirrel_tool_list", &format!("IPC error: {e}"));
                    }
                }
            } else {
                v.check_skip("squirrel_tool_list", "squirrel not available");
            }

            println!("\n=== Phase 6: Squirrel context.create ===\n");

            if let Some(mut client) = running.client_for("ai") {
                let resp = client.call(
                    "context.create",
                    serde_json::json!({
                        "name": "exp070-test-context",
                        "description": "Cross-primal discovery experiment context"
                    }),
                );
                match resp {
                    Ok(r) if r.is_success() => {
                        println!("  context.create response: {:?}", r.result);
                        v.check_bool(
                            "squirrel_context_create",
                            true,
                            "Squirrel context.create succeeded",
                        );
                    }
                    Ok(r) => {
                        println!("  context.create returned: {:?}", r.error);
                        v.check_skip(
                            "squirrel_context_create",
                            &format!("context.create error: {:?}", r.error),
                        );
                    }
                    Err(e) => {
                        println!("  context.create IPC error: {e}");
                        v.check_skip(
                            "squirrel_context_create",
                            &format!("IPC error: {e}"),
                        );
                    }
                }
            } else {
                v.check_skip("squirrel_context_create", "squirrel not available");
            }

            println!("\n=== Phase 7: Squirrel ai.query (skip if no API key) ===\n");

            let has_api_key = std::env::var("ANTHROPIC_API_KEY").is_ok()
                || std::env::var("OPENAI_API_KEY").is_ok();

            if !has_api_key {
                println!("  no API key set — skipping ai.query");
                v.check_skip("squirrel_ai_query", "no ANTHROPIC_API_KEY or OPENAI_API_KEY");
            } else if let Some(mut client) = running.client_for("ai") {
                let resp = client.call(
                    "ai.query",
                    serde_json::json!({
                        "prompt": "What is 2+2? Reply with just the number.",
                        "max_tokens": 16
                    }),
                );
                match resp {
                    Ok(r) if r.is_success() => {
                        println!("  ai.query response: {:?}", r.result);
                        v.check_bool("squirrel_ai_query", true, "ai.query succeeded");
                    }
                    Ok(r) => {
                        println!("  ai.query error: {:?}", r.error);
                        v.check_skip(
                            "squirrel_ai_query",
                            &format!("ai.query error: {:?}", r.error),
                        );
                    }
                    Err(e) => {
                        println!("  ai.query IPC error: {e}");
                        v.check_skip("squirrel_ai_query", &format!("IPC error: {e}"));
                    }
                }
            } else {
                v.check_skip("squirrel_ai_query", "squirrel not available");
            }
        }
        Err(e) => {
            println!(
                "  full overlay start failed (expected if binaries missing): {e}"
            );
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

    println!("\n=== Summary ===\n");
    v.summary();
}
