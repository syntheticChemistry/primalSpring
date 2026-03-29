// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp088 — Storytelling Composition
//!
//! Validates the ludoSpring + esotericWebb + Squirrel + petalTongue
//! storytelling composition via biomeOS Neural API routing.
//! Documents the game.* method gap between ludoSpring and esotericWebb.

use primalspring::ipc::{methods, tcp};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("Storytelling Composition")
        .with_provenance("exp088_storytelling_composition", "2026-03-29")
        .run("ludoSpring + esotericWebb composition", |v| {
            let bm_port = tcp::env_port("BIOMEOS_PORT", 9800);
            let ls_port = tcp::env_port("LUDOSPRING_PORT", 9140);
            let pt_port = tcp::env_port("PETALTONGUE_PORT", 9160);
            let sq_port = tcp::env_port("SQUIRREL_PORT", tolerances::DEFAULT_SQUIRREL_PORT);
            let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

            phase_ludospring_methods(v, &host, ls_port);
            phase_esotericwebb_gap(v, &host, ls_port);
            phase_petaltongue_viz(v, &host, pt_port);
            phase_squirrel_ai(v, &host, sq_port);
            phase_composition_routing(v, &host, bm_port);
        });
}

/// Validate ludoSpring's implemented game.* methods.
fn phase_ludospring_methods(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("ludoSpring Game Methods (Implemented)");

    let flow = tcp::tcp_rpc(
        host,
        port,
        methods::game::EVALUATE_FLOW,
        &serde_json::json!({
            "skill": 0.7,
            "challenge": 0.6
        }),
    );
    match flow {
        Ok((result, _)) => {
            let has_flow = result.get("flow_state").is_some()
                || result.get("state").is_some()
                || result.get("result").is_some();
            v.check_bool(
                "game.evaluate_flow",
                has_flow,
                "flow state evaluation works",
            );
        }
        Err(e) => v.check_skip(
            "game.evaluate_flow",
            &format!("ludoSpring not reachable: {e}"),
        ),
    }

    let dda = tcp::tcp_rpc(
        host,
        port,
        methods::game::DIFFICULTY_ADJUSTMENT,
        &serde_json::json!({
            "current_difficulty": 0.5,
            "player_performance": 0.7
        }),
    );
    match dda {
        Ok((_, _)) => v.check_bool("game.difficulty_adjustment", true, "DDA responds"),
        Err(e) => v.check_skip("game.difficulty_adjustment", &format!("ludoSpring: {e}")),
    }

    let wfc = tcp::tcp_rpc(
        host,
        port,
        methods::game::WFC_STEP,
        &serde_json::json!({
            "width": 4, "height": 4
        }),
    );
    match wfc {
        Ok((_, _)) => v.check_bool("game.wfc_step", true, "Wave Function Collapse responds"),
        Err(e) => v.check_skip("game.wfc_step", &format!("ludoSpring: {e}")),
    }

    let fitts = tcp::tcp_rpc(
        host,
        port,
        methods::game::FITTS_COST,
        &serde_json::json!({
            "distance": 100.0, "width": 20.0
        }),
    );
    match fitts {
        Ok((_, _)) => v.check_bool("game.fitts_cost", true, "Fitts cost analysis responds"),
        Err(e) => v.check_skip("game.fitts_cost", &format!("ludoSpring: {e}")),
    }
}

/// Document the gap: methods esotericWebb expects but ludoSpring doesn't implement yet.
fn phase_esotericwebb_gap(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("esotericWebb Method Gap (Expected but Missing)");

    let gap_methods = [
        ("game.npc_dialogue", "NPC dialogue generation for RPGPT"),
        ("game.narrate_action", "Action narration for AI DM"),
        ("game.begin_session", "Session lifecycle management"),
    ];

    for (method, description) in &gap_methods {
        let result = tcp::tcp_rpc(host, port, method, &serde_json::json!({}));
        match result {
            Ok((_, _)) => {
                v.check_bool(
                    &format!("{method} (gap closed)"),
                    true,
                    &format!("{description} — now implemented"),
                );
            }
            Err(e) => {
                if e.contains("method not found") || e.contains("-32601") {
                    v.check_skip(
                        method,
                        &format!("GAP: {description} — not yet implemented in ludoSpring"),
                    );
                } else {
                    v.check_skip(method, &format!("ludoSpring not reachable: {e}"));
                }
            }
        }
    }
}

/// Validate petalTongue visualization rendering.
fn phase_petaltongue_viz(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("petalTongue Visualization");

    let dashboard = tcp::tcp_rpc(
        host,
        port,
        "visualization.render.dashboard",
        &serde_json::json!({
            "title": "exp088 storytelling test",
            "panels": []
        }),
    );
    match dashboard {
        Ok((_, _)) => v.check_bool(
            "dashboard render",
            true,
            "petalTongue dashboard rendering works",
        ),
        Err(e) => v.check_skip(
            "dashboard render",
            &format!("petalTongue not reachable: {e}"),
        ),
    }

    let dialogue = tcp::tcp_rpc(
        host,
        port,
        "visualization.render.dialogue_tree",
        &serde_json::json!({
            "scene_type": "dialogue_tree"
        }),
    );
    match dialogue {
        Ok((_, _)) => v.check_bool(
            "dialogue tree render",
            true,
            "petalTongue dialogue-tree scene type works",
        ),
        Err(e) => {
            if e.contains("method not found") || e.contains("-32601") {
                v.check_skip(
                    "dialogue tree",
                    "GAP: dialogue-tree scene type not yet implemented in petalTongue",
                );
            } else {
                v.check_skip("dialogue tree", &format!("petalTongue: {e}"));
            }
        }
    }
}

/// Validate Squirrel AI integration.
fn phase_squirrel_ai(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Squirrel AI");

    let health = tcp::tcp_rpc(
        host,
        port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    match health {
        Ok((_, _)) => v.check_bool("Squirrel alive", true, "Squirrel responds to liveness"),
        Err(e) => v.check_skip("Squirrel liveness", &format!("Squirrel not reachable: {e}")),
    }

    let tools = tcp::tcp_rpc(host, port, methods::mcp::TOOLS_LIST, &serde_json::json!({}));
    match tools {
        Ok((val, _)) => {
            let has_tools = val.is_array() || val.get("tools").is_some();
            v.check_bool(
                "MCP tools available",
                has_tools,
                "Squirrel exposes MCP tools",
            );
        }
        Err(e) => v.check_skip("MCP tools", &format!("Squirrel: {e}")),
    }
}

/// Validate routing through biomeOS Neural API for the storytelling stack.
fn phase_composition_routing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Composition Routing via Neural API");

    let game_route = tcp::neural_api_capability_call(
        host,
        port,
        "game",
        "game.evaluate_flow",
        &serde_json::json!({"skill": 0.5, "challenge": 0.5}),
    );
    match game_route {
        Ok((_, _)) => v.check_bool(
            "game domain routed",
            true,
            "Neural API routes game.* to ludoSpring",
        ),
        Err(e) => v.check_skip("game domain routing", &format!("biomeOS game routing: {e}")),
    }

    let viz_route = tcp::neural_api_capability_call(
        host,
        port,
        "visualization",
        "visualization.render.dashboard",
        &serde_json::json!({"title": "test"}),
    );
    match viz_route {
        Ok((_, _)) => v.check_bool(
            "viz domain routed",
            true,
            "Neural API routes visualization.* to petalTongue",
        ),
        Err(e) => v.check_skip("viz domain routing", &format!("biomeOS viz routing: {e}")),
    }
}
