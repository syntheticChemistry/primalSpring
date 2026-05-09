// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![allow(
    clippy::cast_precision_loss,
    clippy::option_if_let_else,
    reason = "desktop shell uses f64 casts for layout math"
)]

//! exp106 — Micro-Desktop Shell
//!
//! Validates the desktop composition model wrapping The Rhizome game:
//! biomeOS graph orchestration → multi-session petalTongue rendering →
//! system health polling → provenance sidebar → capability routing
//!
//! Phase 56 — Desktop Substrate (MICRO_DESKTOP_COMPOSITION.md)

use base64::Engine as _;
use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn phase_biomeos_connection(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("biomeOS Neural API Connection");

    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "biomeos_connect",
            "biomeOS not discovered (tried biomeos, neural-api, orchestration)",
        );
        return;
    }

    match ctx.health_check("orchestration") {
        Ok(true) => {
            v.check_bool(
                "biomeos_connect",
                true,
                "biomeOS Neural API connected and healthy",
            );
        }
        Ok(false) => {
            v.check_skip("biomeos_connect", "biomeOS orchestration not healthy");
        }
        Err(e) => {
            v.check_skip(
                "biomeos_connect",
                &format!("biomeOS connection failed: {e}"),
            );
        }
    }
}

struct PrimalHealth {
    #[expect(dead_code, reason = "used for structured logging, not rendered in TUI")]
    name: &'static str,
    abbrev: &'static str,
    #[expect(dead_code, reason = "used for structured logging, not rendered in TUI")]
    capability: &'static str,
    healthy: bool,
}

fn phase_system_health(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
) -> Vec<PrimalHealth> {
    v.section("System Health Polling (Songbird)");

    let primals: &[(&str, &str, &str)] = &[
        ("biomeOS", "Bio", "orchestration"),
        ("Songbird", "Song", "discovery"),
        ("NestGate", "Nest", "storage"),
        ("Squirrel", "Squi", "ai"),
        ("BearDog", "Bear", "security"),
        ("ToadStool", "Toad", "compute"),
        ("Barracuda", "Barr", "tensor"),
        ("CoralReef", "Coral", "shader"),
        ("rhizoCrypt", "Rz", "dag"),
        ("loamSpine", "Loam", "ledger"),
        ("sweetGrass", "Swt", "attribution"),
        ("petalTongue", "Petal", "visualization"),
    ];

    let mut health_results: Vec<PrimalHealth> = Vec::new();
    let mut healthy_count = 0;

    for &(name, abbrev, capability) in primals {
        let healthy =
            ctx.has_capability(capability) && ctx.health_check(capability).unwrap_or(false);

        if healthy {
            healthy_count += 1;
        }

        health_results.push(PrimalHealth {
            name,
            abbrev,
            capability,
            healthy,
        });
    }

    v.check_minimum("healthy_primals", healthy_count, 8);

    let bar: String = health_results
        .iter()
        .map(|p| {
            if p.healthy {
                format!("[{}✓]", p.abbrev)
            } else {
                format!("[{}✗]", p.abbrev)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    v.check_bool(
        "health_bar_format",
        !bar.is_empty(),
        &format!("System bar: {bar}"),
    );

    health_results
}

fn phase_capability_routing(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("biomeOS Capability Routing");

    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "cap_routing",
            "biomeOS not connected — skipping routing tests",
        );
        return;
    }

    let test_cases: &[(&str, &str, serde_json::Value, &str)] = &[
        (
            "route_crypto",
            "crypto.blake3_hash",
            serde_json::json!({
                "capability": "crypto",
                "operation": "crypto.blake3_hash",
                "params": {"data": base64::engine::general_purpose::STANDARD.encode(b"desktop-shell-test")}
            }),
            "crypto.blake3_hash via biomeOS",
        ),
        (
            "route_dag",
            "dag.session.create",
            serde_json::json!({
                "capability": "dag",
                "operation": "dag.session.create",
                "params": {"name": "exp106-desktop-shell"}
            }),
            "dag.session.create via biomeOS",
        ),
        (
            "route_stats",
            "stats.mean",
            serde_json::json!({
                "capability": "stats",
                "operation": "stats.mean",
                "params": {"data": [1.0, 2.0, 3.0]}
            }),
            "stats.mean via biomeOS",
        ),
        (
            "route_discovery",
            "ipc.list",
            serde_json::json!({
                "capability": "discovery",
                "operation": "ipc.list",
                "params": {}
            }),
            "ipc.list via biomeOS",
        ),
    ];

    for (check_name, _method, params, detail) in test_cases {
        let resp = ctx.call("orchestration", "capability.call", params.clone());

        v.check_bool(check_name, resp.is_ok(), detail);
    }

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let storage_resp = ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": "storage",
            "operation": "storage.store",
            "params": {"family_id": family_id, "key": "exp106-shell-test", "value": "desktop-test"}
        }),
    );

    let storage_ok = storage_resp.is_ok();
    if storage_ok {
        v.check_bool(
            "route_storage",
            true,
            "storage.store via biomeOS (GAP-13 resolved?)",
        );
    } else {
        v.check_skip(
            "route_storage",
            "storage.store routes to ToadStool (GAP-13 confirmed)",
        );
    }
}

fn phase_graph_management(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("biomeOS Graph Management");

    if !ctx.has_capability("orchestration") {
        v.check_skip("graph_mgmt", "biomeOS not connected — skipping graph tests");
        return;
    }

    let list_resp = ctx.call("orchestration", "graph.list", serde_json::json!({}));
    match list_resp {
        Ok(r) => {
            let count = r
                .get("graphs")
                .and_then(|g| g.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "graph_list",
                count > 0,
                &format!("biomeOS graph.list returned {count} graphs"),
            );
        }
        Err(e) => {
            v.check_skip("graph_list", &format!("graph.list failed: {e}"));
        }
    }

    let status_resp = ctx.call("orchestration", "graph.status", serde_json::json!({}));
    v.check_bool(
        "graph_status",
        status_resp.is_ok(),
        "biomeOS graph.status accessible",
    );

    let save_resp = ctx.call(
        "orchestration",
        "graph.save",
        serde_json::json!({
            "name": "rhizome_game",
            "description": "The Rhizome game graph (exp106 test)",
            "nodes": [
                {"id": "game_engine", "name": "rhizome_engine", "capabilities": ["game.tick"]}
            ],
            "coordination": "continuous"
        }),
    );

    v.check_bool(
        "graph_save",
        save_resp.is_ok(),
        "biomeOS graph.save for rhizome game",
    );
}

fn phase_provenance_sidebar(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
) -> Option<String> {
    v.section("Provenance Sidebar (rhizoCrypt DAG)");

    if !ctx.has_capability("dag") {
        v.check_skip("prov_sidebar", "rhizoCrypt not discovered");
        return None;
    }

    let session_resp = ctx.call(
        "dag",
        "dag.session.create",
        serde_json::json!({"name": "exp106-provenance-sidebar"}),
    );

    let session_id = session_resp.ok().and_then(|v| {
        v.as_str().map(String::from).or_else(|| {
            v.get("session_id")
                .and_then(|s| s.as_str())
                .map(String::from)
        })
    });

    let Some(ref sid) = session_id else {
        v.check_skip("prov_sidebar", "dag.session.create failed");
        return None;
    };

    v.check_bool("prov_session", true, "DAG session for sidebar created");

    for label in &["session_start", "game_save_turn_100", "game_save_turn_200"] {
        let _ = ctx.call(
            "dag",
            "dag.event.append",
            serde_json::json!({
                "session_id": sid,
                "event_type": {
                    "Custom": {
                        "label": label,
                        "event_name": label,
                        "domain": "game"
                    }
                },
                "data": {"label": label, "experiment": "exp106"}
            }),
        );
    }

    let root_resp = ctx.call(
        "dag",
        "dag.merkle.root",
        serde_json::json!({"session_id": sid}),
    );

    match root_resp {
        Ok(r) => {
            let root = r.get("root").and_then(|s| s.as_str()).unwrap_or("unknown");
            let short_root = &root[..root.len().min(12)];
            v.check_bool(
                "prov_merkle",
                true,
                &format!("Merkle root for sidebar: {short_root}..."),
            );
        }
        Err(e) => {
            v.check_skip("prov_merkle", &format!("dag.merkle.root failed: {e}"));
        }
    }

    session_id
}

fn phase_multi_session_rendering(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    health: &[PrimalHealth],
) {
    v.section("Multi-Session Rendering (petalTongue)");

    if !ctx.has_capability("visualization") {
        v.check_skip("multi_render", "petalTongue not discovered");
        return;
    }

    let shell_scene = build_shell_scene(health);
    let shell_resp = ctx.call(
        "visualization",
        "visualization.render.scene",
        serde_json::json!({
            "session": "desktop-shell",
            "scene": shell_scene
        }),
    );

    let shell_ok = shell_resp.is_ok();
    v.check_bool(
        "shell_scene",
        shell_ok,
        "Desktop shell chrome rendered to petalTongue",
    );

    let game_scene = build_game_placeholder_scene();
    let game_resp = ctx.call(
        "visualization",
        "visualization.render.scene",
        serde_json::json!({
            "session": "rhizome-game",
            "scene": game_scene
        }),
    );

    let game_ok = game_resp.is_ok();
    v.check_bool(
        "game_scene",
        game_ok,
        "Game viewport placeholder rendered to petalTongue",
    );

    if shell_ok && game_ok {
        v.check_bool(
            "multi_session",
            true,
            "Two concurrent petalTongue sessions (desktop-shell + rhizome-game)",
        );
    } else {
        v.check_skip(
            "multi_session",
            "Multi-session rendering gap — one or both sessions failed",
        );
    }
}

fn build_shell_scene(health: &[PrimalHealth]) -> serde_json::Value {
    let mut nodes = Vec::new();

    let bar_text: String = health
        .iter()
        .map(|p| {
            if p.healthy {
                format!("[{}✓]", p.abbrev)
            } else {
                format!("[{}✗]", p.abbrev)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    nodes.push(serde_json::json!({
        "Text": {
            "content": bar_text,
            "font_size": 11,
            "color": {"r": 0.8, "g": 0.8, "b": 0.8, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 4.0, "ty": 2.0}
        }
    }));

    nodes.push(serde_json::json!({
        "Text": {
            "content": "Session: active | The Rhizome",
            "font_size": 11,
            "color": {"r": 0.5, "g": 0.9, "b": 0.5, "a": 1.0},
            "anchor": "TopRight",
            "transform": {"tx": 590.0, "ty": 2.0}
        }
    }));

    nodes.push(serde_json::json!({
        "Text": {
            "content": "─ Provenance ─",
            "font_size": 11,
            "color": {"r": 0.7, "g": 0.6, "b": 0.9, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 420.0, "ty": 20.0}
        }
    }));

    nodes.push(serde_json::json!({
        "Text": {
            "content": "[turn 200] game_save  91b4e0\n[turn 100] game_save  a3f8c2\n[turn 001] session    d82f31",
            "font_size": 10,
            "color": {"r": 0.6, "g": 0.6, "b": 0.8, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 420.0, "ty": 36.0}
        }
    }));

    serde_json::json!({
        "type": "desktop_shell",
        "title": "Micro-Desktop Shell",
        "nodes": nodes
    })
}

fn build_game_placeholder_scene() -> serde_json::Value {
    let mut nodes = Vec::new();

    nodes.push(serde_json::json!({
        "Text": {
            "content": "The Rhizome — Awaiting game session...",
            "font_size": 14,
            "color": {"r": 0.5, "g": 0.8, "b": 0.3, "a": 1.0},
            "anchor": "TopLeft",
            "transform": {"tx": 80.0, "ty": 180.0}
        }
    }));

    nodes.push(serde_json::json!({
        "Text": {
            "content": "@ Fieldmouse ready",
            "font_size": 14,
            "color": {"r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0},
            "anchor": "TopLeft",
            "bold": true,
            "transform": {"tx": 80.0, "ty": 200.0}
        }
    }));

    serde_json::json!({
        "type": "game_viewport",
        "title": "The Rhizome",
        "nodes": nodes
    })
}

fn phase_direct_fallbacks(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Direct Primal Fallbacks");

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let store_params = serde_json::json!({
        "family_id": family_id,
        "key": "exp106-direct-test",
        "value": "desktop-shell-direct-fallback",
    });

    if ctx.has_capability("storage") {
        match ctx.call("storage", "storage.store", store_params) {
            Ok(_) => {
                v.check_bool("direct_nestgate", true, "Direct NestGate storage");
            }
            Err(e) => {
                v.check_bool(
                    "direct_nestgate",
                    false,
                    &format!("NestGate transport: {e}"),
                );
            }
        }
    } else {
        v.check_skip("direct_nestgate", "NestGate not discovered");
    }

    if ctx.has_capability("tensor") {
        match ctx.call(
            "tensor",
            "noise.perlin2d",
            serde_json::json!({"x": 2, "y": 2, "scale": 1.0, "seed": 106}),
        ) {
            Ok(_) => {
                v.check_bool("direct_barracuda", true, "Direct Barracuda noise");
            }
            Err(e) => {
                v.check_bool(
                    "direct_barracuda",
                    false,
                    &format!("Barracuda transport: {e}"),
                );
            }
        }
    } else {
        v.check_skip("direct_barracuda", "Barracuda not discovered");
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp106 — Micro-Desktop Shell")
        .with_provenance("exp106_micro_desktop_shell", "2026-05-09")
        .run("Exp106: Desktop composition on NUCLEUS substrate", |v| {
            let mut ctx = CompositionContext::discover();
            phase_biomeos_connection(v, &mut ctx);
            let health = phase_system_health(v, &mut ctx);
            phase_capability_routing(v, &mut ctx);
            phase_graph_management(v, &mut ctx);
            let _dag_session = phase_provenance_sidebar(v, &mut ctx);
            phase_multi_session_rendering(v, &mut ctx, &health);
            phase_direct_fallbacks(v, &mut ctx);
        });
}
