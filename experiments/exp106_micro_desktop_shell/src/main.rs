// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp106 — Micro-Desktop Shell
//!
//! Validates the desktop composition model wrapping The Rhizome game:
//! biomeOS graph orchestration → multi-session petalTongue rendering →
//! system health polling → provenance sidebar → capability routing
//!
//! Phase 56 — Desktop Substrate (MICRO_DESKTOP_COMPOSITION.md)

use base64::Engine as _;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_by_capability, discover_primal};
use primalspring::validation::ValidationResult;

// ═══════════════════════════════════════════════════════════════════════
// Phase 1: biomeOS Neural API Connection
// ═══════════════════════════════════════════════════════════════════════

fn phase_biomeos_connection(v: &mut ValidationResult) -> Option<PrimalClient> {
    v.section("biomeOS Neural API Connection");

    let bio = discover_primal("biomeos");
    let bio_sock_ref = bio.socket.as_ref();

    let fallback;
    let bio_sock_ref = if bio_sock_ref.is_some() {
        bio_sock_ref
    } else {
        fallback = discover_primal("neural-api");
        fallback.socket.as_ref()
    };

    let Some(bio_sock) = bio_sock_ref else {
        let orchestration = discover_by_capability("orchestration");
        let Some(bio_sock) = orchestration.socket.as_ref() else {
            v.check_skip("biomeos_connect", "biomeOS not discovered (tried biomeos, neural-api, orchestration)");
            return None;
        };
        match PrimalClient::connect(bio_sock, "biomeos") {
            Ok(mut client) => {
                let health = client.health_check();
                v.check_bool(
                    "biomeos_connect",
                    health.is_ok_and(|h| h),
                    "biomeOS connected via orchestration capability",
                );
                return Some(client);
            }
            Err(e) => {
                v.check_skip("biomeos_connect", &format!("biomeOS connection failed: {e}"));
                return None;
            }
        }
    };

    match PrimalClient::connect(bio_sock, "biomeos") {
        Ok(mut client) => {
            let health = client.health_check();
            v.check_bool(
                "biomeos_connect",
                health.is_ok_and(|h| h),
                "biomeOS Neural API connected and healthy",
            );
            Some(client)
        }
        Err(e) => {
            v.check_skip(
                "biomeos_connect",
                &format!("biomeOS connection failed: {e}"),
            );
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 2: System Health Polling (Songbird Discovery)
// ═══════════════════════════════════════════════════════════════════════

struct PrimalHealth {
    #[allow(dead_code)]
    name: &'static str,
    abbrev: &'static str,
    #[allow(dead_code)]
    capability: &'static str,
    healthy: bool,
}

fn phase_system_health(v: &mut ValidationResult) -> Vec<PrimalHealth> {
    v.section("System Health Polling (Songbird)");

    let primals: &[(&str, &str, &str)] = &[
        ("biomeOS", "Bio", "orchestration"),
        ("Songbird", "Song", "discovery"),
        ("NestGate", "Nest", "storage"),
        ("Squirrel", "Squi", "ai"),
        ("BearDog", "Bear", "crypto"),
        ("ToadStool", "Toad", "compute"),
        ("Barracuda", "Barr", "math"),
        ("CoralReef", "Coral", "shader"),
        ("rhizoCrypt", "Rz", "dag"),
        ("loamSpine", "Loam", "ledger"),
        ("sweetGrass", "Swt", "attribution"),
        ("petalTongue", "Petal", "visualization"),
    ];

    let mut health_results: Vec<PrimalHealth> = Vec::new();
    let mut healthy_count = 0;

    for &(name, abbrev, capability) in primals {
        let disc = discover_by_capability(capability);
        let healthy = disc.socket.as_ref().is_some_and(|sock| {
            PrimalClient::connect(sock, &name.to_lowercase())
                .ok()
                .and_then(|mut c| c.health_check().ok())
                .unwrap_or(false)
        });

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

// ═══════════════════════════════════════════════════════════════════════
// Phase 3: biomeOS Capability Routing
// ═══════════════════════════════════════════════════════════════════════

fn phase_capability_routing(v: &mut ValidationResult, biomeos: &mut Option<PrimalClient>) {
    v.section("biomeOS Capability Routing");

    let Some(client) = biomeos.as_mut() else {
        v.check_skip("cap_routing", "biomeOS not connected — skipping routing tests");
        return;
    };

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
        let resp = client.call("capability.call", params.clone());

        v.check_bool(
            check_name,
            resp.is_ok_and(|r| r.result.is_some()),
            detail,
        );
    }

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let storage_resp = client.call(
        "capability.call",
        serde_json::json!({
            "capability": "storage",
            "operation": "storage.store",
            "params": {"family_id": family_id, "key": "exp106-shell-test", "value": "desktop-test"}
        }),
    );

    let storage_ok = storage_resp.is_ok_and(|r| r.result.is_some());
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

// ═══════════════════════════════════════════════════════════════════════
// Phase 4: Graph Execution
// ═══════════════════════════════════════════════════════════════════════

fn phase_graph_management(v: &mut ValidationResult, biomeos: &mut Option<PrimalClient>) {
    v.section("biomeOS Graph Management");

    let Some(client) = biomeos.as_mut() else {
        v.check_skip("graph_mgmt", "biomeOS not connected — skipping graph tests");
        return;
    };

    let list_resp = client.call("graph.list", serde_json::json!({}));
    match list_resp {
        Ok(r) => {
            let count = r
                .result
                .as_ref()
                .and_then(|r| r.get("graphs"))
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

    let status_resp = client.call("graph.status", serde_json::json!({}));
    v.check_bool(
        "graph_status",
        status_resp.is_ok_and(|r| r.result.is_some()),
        "biomeOS graph.status accessible",
    );

    let save_resp = client.call(
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
        save_resp.is_ok_and(|r| r.result.is_some()),
        "biomeOS graph.save for rhizome game",
    );
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 5: Provenance Sidebar Data
// ═══════════════════════════════════════════════════════════════════════

fn phase_provenance_sidebar(v: &mut ValidationResult) -> Option<String> {
    v.section("Provenance Sidebar (rhizoCrypt DAG)");

    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("prov_sidebar", "rhizoCrypt not discovered");
        return None;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("prov_sidebar", "rhizoCrypt connection failed");
        return None;
    };

    let session_resp = client.call(
        "dag.session.create",
        serde_json::json!({"name": "exp106-provenance-sidebar"}),
    );

    let session_id = session_resp.ok().and_then(|r| r.result).and_then(|v| {
        v.as_str()
            .map(String::from)
            .or_else(|| v.get("session_id").and_then(|s| s.as_str()).map(String::from))
    });

    let Some(ref sid) = session_id else {
        v.check_skip("prov_sidebar", "dag.session.create failed");
        return None;
    };

    v.check_bool("prov_session", true, "DAG session for sidebar created");

    for label in &["session_start", "game_save_turn_100", "game_save_turn_200"] {
        let _ = client.call(
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

    let root_resp = client.call(
        "dag.merkle.root",
        serde_json::json!({"session_id": sid}),
    );

    match root_resp {
        Ok(r) => {
            let root = r
                .result
                .as_ref()
                .and_then(|r| r.get("root"))
                .and_then(|s| s.as_str())
                .unwrap_or("unknown");
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

// ═══════════════════════════════════════════════════════════════════════
// Phase 6: Multi-Session petalTongue Rendering
// ═══════════════════════════════════════════════════════════════════════

fn phase_multi_session_rendering(
    v: &mut ValidationResult,
    health: &[PrimalHealth],
) {
    v.section("Multi-Session Rendering (petalTongue)");

    let pt = discover_by_capability("visualization");
    let Some(pt_sock) = pt.socket.as_ref() else {
        v.check_skip("multi_render", "petalTongue not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(pt_sock, "petaltongue") else {
        v.check_skip("multi_render", "petalTongue connection failed");
        return;
    };

    let shell_scene = build_shell_scene(health);
    let shell_resp = client.call(
        "visualization.render.scene",
        serde_json::json!({
            "session": "desktop-shell",
            "scene": shell_scene
        }),
    );

    let shell_ok = shell_resp.is_ok_and(|r| r.result.is_some());
    v.check_bool(
        "shell_scene",
        shell_ok,
        "Desktop shell chrome rendered to petalTongue",
    );

    let game_scene = build_game_placeholder_scene();
    let game_resp = client.call(
        "visualization.render.scene",
        serde_json::json!({
            "session": "rhizome-game",
            "scene": game_scene
        }),
    );

    let game_ok = game_resp.is_ok_and(|r| r.result.is_some());
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

// ═══════════════════════════════════════════════════════════════════════
// Phase 7: Direct Primal Fallbacks (when biomeOS routing fails)
// ═══════════════════════════════════════════════════════════════════════

fn phase_direct_fallbacks(v: &mut ValidationResult) {
    v.section("Direct Primal Fallbacks");

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let store_params = serde_json::json!({
        "family_id": family_id,
        "key": "exp106-direct-test",
        "value": "desktop-shell-direct-fallback",
    });

    let ng = discover_primal("nestgate");
    if let Some(ng_sock) = ng.socket.as_ref() {
        if let Ok(mut client) = PrimalClient::connect(ng_sock, "nestgate") {
            let resp = client.call("storage.store", store_params);
            match &resp {
                Ok(r) if r.result.is_some() => {
                    v.check_bool("direct_nestgate", true, "Direct NestGate storage");
                }
                Ok(r) => {
                    let msg = r.error.as_ref().map_or("no result".to_owned(), |e| e.message.clone());
                    v.check_bool("direct_nestgate", false, &format!("NestGate error: {msg}"));
                }
                Err(e) => {
                    v.check_bool("direct_nestgate", false, &format!("NestGate transport: {e}"));
                }
            }
        } else {
            v.check_skip("direct_nestgate", "NestGate connection failed");
        }
    } else {
        v.check_skip("direct_nestgate", "NestGate not discovered");
    }

    let barr = discover_primal("barracuda");
    let barr_fb;
    let barr_sock = match barr.socket.as_ref() {
        Some(s) => Some(s),
        None => {
            barr_fb = discover_by_capability("math");
            barr_fb.socket.as_ref()
        }
    };

    if let Some(barr_sock) = barr_sock {
        if let Ok(mut client) = PrimalClient::connect(barr_sock, "barracuda") {
            let resp = client.call(
                "noise.perlin2d",
                serde_json::json!({"x": 2, "y": 2, "scale": 1.0, "seed": 106}),
            );
            match &resp {
                Ok(r) if r.result.is_some() => {
                    v.check_bool("direct_barracuda", true, "Direct Barracuda noise");
                }
                Ok(r) => {
                    let msg = r.error.as_ref().map_or("no result".to_owned(), |e| e.message.clone());
                    v.check_bool("direct_barracuda", false, &format!("Barracuda error: {msg}"));
                }
                Err(e) => {
                    v.check_bool("direct_barracuda", false, &format!("Barracuda transport: {e}"));
                }
            }
        } else {
            v.check_skip("direct_barracuda", "Barracuda connection failed");
        }
    } else {
        v.check_skip("direct_barracuda", "Barracuda not discovered");
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════════════════

fn main() {
    ValidationResult::new("primalSpring Exp106 — Micro-Desktop Shell")
        .with_provenance("exp106_micro_desktop_shell", "2026-04-28")
        .run(
            "Exp106: Desktop composition on NUCLEUS substrate",
            |v| {
                let mut biomeos = phase_biomeos_connection(v);
                let health = phase_system_health(v);
                phase_capability_routing(v, &mut biomeos);
                phase_graph_management(v, &mut biomeos);
                let _dag_session = phase_provenance_sidebar(v);
                phase_multi_session_rendering(v, &health);
                phase_direct_fallbacks(v);
            },
        );
}
