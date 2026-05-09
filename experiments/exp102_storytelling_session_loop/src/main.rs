// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp102 — Storytelling Session Loop
//!
//! Validates the full storytelling pipeline as a desktop application session:
//! esotericWebb → ludoSpring → Squirrel → petalTongue → Provenance trio
//!
//! Phase 56 — Desktop Substrate (STORYTELLING_EVOLUTION.md)

use primalspring::composition::CompositionContext;
use primalspring::ipc::IpcError;
use primalspring::validation::ValidationResult;

fn orchestration_route(
    ctx: &mut CompositionContext,
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": capability,
            "operation": operation,
            "args": args,
        }),
    )
}

fn call_game(
    ctx: &mut CompositionContext,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    if ctx.has_capability("game") {
        ctx.call("game", method, params)
    } else if ctx.has_capability("orchestration") {
        let op = method.strip_prefix("game.").unwrap_or(method);
        orchestration_route(ctx, "game", op, &params)
    } else {
        Err(IpcError::SocketNotFound {
            primal: "game".to_owned(),
        })
    }
}

fn phase_game_session(v: &mut ValidationResult) {
    v.section("Game Session Lifecycle (ludoSpring)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("game") && !ctx.has_capability("orchestration") {
        v.check_skip("game_begin", "ludoSpring not discovered");
        return;
    }

    let resp = call_game(
        &mut ctx,
        "game.begin_session",
        serde_json::json!({
            "session_name": "exp102-test",
            "world": "test_world",
            "player_name": "Validator",
            "tick_hz": 60,
            "provenance": false
        }),
    );

    v.check_bool("game_begin", resp.is_ok(), "ludoSpring game.begin_session");
}

fn phase_narration_pipeline(v: &mut ValidationResult) {
    v.section("Narration Pipeline");

    let mut ctx = CompositionContext::discover();

    if !ctx.has_capability("game") && !ctx.has_capability("orchestration") {
        v.check_skip("narrate_action", "ludoSpring not discovered");
    } else {
        let resp = call_game(
            &mut ctx,
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
            resp.is_ok(),
            "ludoSpring game.narrate_action",
        );
    }

    if !ctx.has_capability("ai") {
        v.check_skip("ai_narration", "Squirrel not discovered");
        return;
    }

    let sq_resp = ctx.call(
        "ai",
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
        sq_resp.is_ok(),
        "Squirrel ai.chat generates narration",
    );
}

fn phase_scene_rendering(v: &mut ValidationResult) {
    v.section("Scene Rendering (petalTongue)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("visualization") {
        v.check_skip("scene_render", "petalTongue not discovered");
        return;
    }

    let resp = ctx.call(
        "visualization",
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
        resp.is_ok(),
        "petalTongue renders narrative scene",
    );
}

fn phase_provenance(v: &mut ValidationResult) {
    v.section("Session Provenance (rhizoCrypt)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("dag") {
        v.check_skip("dag_session", "rhizoCrypt not discovered");
        return;
    }

    let resp = ctx.call(
        "dag",
        "dag.session.create",
        serde_json::json!({"name": "exp102-storytelling-provenance"}),
    );

    v.check_bool(
        "dag_session",
        resp.is_ok(),
        "rhizoCrypt DAG session created for storytelling provenance",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp102 — Storytelling Session Loop")
        .with_provenance("exp102_storytelling_session_loop", "2026-05-09")
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
