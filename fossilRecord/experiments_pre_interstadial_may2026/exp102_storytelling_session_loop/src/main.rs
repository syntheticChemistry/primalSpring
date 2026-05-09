// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp102 — Storytelling Session Loop
//!
//! Validates the full storytelling pipeline as a desktop application session:
//! esotericWebb → ludoSpring → Squirrel → petalTongue → Provenance trio
//!
//! Phase 56 — Desktop Substrate (STORYTELLING_EVOLUTION.md)

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_by_capability, discover_primal};
use primalspring::validation::ValidationResult;

fn phase_game_session(v: &mut ValidationResult) {
    v.section("Game Session Lifecycle (ludoSpring)");

    let ls = discover_primal("ludospring");
    let Some(ls_sock) = ls.socket.as_ref() else {
        v.check_skip("game_begin", "ludoSpring not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ls_sock, "ludospring") else {
        v.check_skip("game_begin", "ludoSpring connection failed");
        return;
    };

    let resp = client.call(
        "game.begin_session",
        serde_json::json!({
            "session_name": "exp102-test",
            "world": "test_world",
            "player_name": "Validator",
            "tick_hz": 60,
            "provenance": false
        }),
    );

    v.check_bool(
        "game_begin",
        resp.is_ok_and(|r| r.result.is_some()),
        "ludoSpring game.begin_session",
    );
}

fn phase_narration_pipeline(v: &mut ValidationResult) {
    v.section("Narration Pipeline");

    let ls = discover_primal("ludospring");
    let Some(ls_sock) = ls.socket.as_ref() else {
        v.check_skip("narrate_action", "ludoSpring not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ls_sock, "ludospring") else {
        v.check_skip("narrate_action", "ludoSpring connection failed");
        return;
    };

    let resp = client.call(
        "game.narrate_action",
        serde_json::json!({
            "action": "investigate_bookshelf",
            "actor": "player",
            "scene_id": "library",
            "flow_state": {"engagement": 0.7, "challenge": 0.5, "pacing": "rising"},
            "dda_context": {"player_skill_estimate": 0.6, "recent_failures": 0},
            "resolved_predicates": [],
            "dice_result": {"type": "d20", "roll": 15, "modifier": 2, "total": 17, "dc": 12, "success": true}
        }),
    );

    v.check_bool(
        "narrate_action",
        resp.is_ok_and(|r| r.result.is_some()),
        "ludoSpring game.narrate_action",
    );

    let sq = discover_primal("squirrel");
    let Some(sq_sock) = sq.socket.as_ref() else {
        v.check_skip("ai_narration", "Squirrel not discovered");
        return;
    };

    let Ok(mut sq_client) = PrimalClient::connect(sq_sock, "squirrel") else {
        v.check_skip("ai_narration", "Squirrel connection failed");
        return;
    };

    let resp = sq_client.call(
        "ai.chat",
        serde_json::json!({
            "messages": [
                {"role": "system", "content": "You are a game narrator."},
                {"role": "user", "content": "The player investigates the bookshelf and succeeds (roll 17 vs DC 12)."}
            ]
        }),
    );

    v.check_bool(
        "ai_narration",
        resp.is_ok(),
        "Squirrel ai.chat generates narration",
    );
}

fn phase_scene_rendering(v: &mut ValidationResult) {
    v.section("Scene Rendering (petalTongue)");

    let pt = discover_by_capability("visualization");
    let Some(pt_sock) = pt.socket.as_ref() else {
        v.check_skip("scene_render", "petalTongue not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(pt_sock, "petaltongue") else {
        v.check_skip("scene_render", "petalTongue connection failed");
        return;
    };

    let resp = client.call(
        "visualization.render.scene",
        serde_json::json!({
            "session": "exp102-storytelling",
            "scene": {
                "type": "narrative",
                "background": "library_interior",
                "text": "You find an old tome hidden behind the other books...",
                "options": [
                    {"id": 1, "text": "Read the tome."},
                    {"id": 2, "text": "Put it back."}
                ]
            }
        }),
    );

    v.check_bool(
        "scene_render",
        resp.is_ok_and(|r| r.result.is_some()),
        "petalTongue renders narrative scene",
    );
}

fn phase_provenance(v: &mut ValidationResult) {
    v.section("Session Provenance (rhizoCrypt)");

    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("dag_session", "rhizoCrypt not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("dag_session", "rhizoCrypt connection failed");
        return;
    };

    let resp = client.call(
        "dag.session.create",
        serde_json::json!({"name": "exp102-storytelling-provenance"}),
    );

    v.check_bool(
        "dag_session",
        resp.is_ok_and(|r| r.result.is_some()),
        "rhizoCrypt DAG session created for storytelling provenance",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp102 — Storytelling Session Loop")
        .with_provenance("exp102_storytelling_session_loop", "2026-04-28")
        .run(
            "Exp102: Full storytelling pipeline on Desktop NUCLEUS",
            |v| {
                phase_game_session(v);
                phase_narration_pipeline(v);
                phase_scene_rendering(v);
                phase_provenance(v);
            },
        );
}
