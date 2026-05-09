// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp103 — ludoSpring Expanded IPC
//!
//! Validates the 6 new IPC methods esotericWebb requires from ludoSpring
//! plus the existing 8 methods.
//!
//! Phase 56 — Desktop Substrate (LUDOSPRING_IPC_EXPANSION_PHASE56_APR28_2026.md)

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

fn phase_existing_methods(v: &mut ValidationResult) {
    v.section("Existing IPC Methods (8)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("game") && !ctx.has_capability("orchestration") {
        v.check_skip("existing_methods", "ludoSpring not discovered");
        return;
    }

    let methods: &[(&str, serde_json::Value)] = &[
        (
            "game.evaluate_flow",
            serde_json::json!({"engagement": 0.7, "challenge": 0.5}),
        ),
        (
            "game.fitts_cost",
            serde_json::json!({"distance": 100.0, "width": 20.0}),
        ),
        (
            "game.engagement",
            serde_json::json!({"session_duration_secs": 600}),
        ),
        (
            "game.difficulty_adjustment",
            serde_json::json!({"player_skill": 0.6, "current_difficulty": 0.5}),
        ),
        (
            "game.wfc_step",
            serde_json::json!({"grid_width": 4, "grid_height": 4}),
        ),
        (
            "game.generate_noise",
            serde_json::json!({"width": 8, "height": 8, "seed": 42}),
        ),
        ("game.analyze_ui", serde_json::json!({"elements": []})),
        ("game.accessibility", serde_json::json!({"mode": "check"})),
    ];

    for (method, params) in methods {
        let resp = call_game(&mut ctx, method, params.clone());
        v.check_bool(
            &method.replace('.', "_"),
            resp.is_ok(),
            &format!("{method} responds"),
        );
    }
}

fn phase_new_ipc_dialogue(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let resp = call_game(
        ctx,
        "game.begin_session",
        serde_json::json!({
            "session_name": "exp103-test",
            "world": "test",
            "player_name": "Validator",
            "tick_hz": 60,
            "provenance": false
        }),
    );
    v.check_bool("begin_session", resp.is_ok(), "game.begin_session");

    let resp = call_game(
        ctx,
        "game.narrate_action",
        serde_json::json!({
            "action": "look_around",
            "actor": "player",
            "scene_id": "tavern",
            "flow_state": {"engagement": 0.7, "challenge": 0.4, "pacing": "steady"},
            "dda_context": {"player_skill_estimate": 0.5, "recent_failures": 0},
            "resolved_predicates": [],
            "dice_result": null
        }),
    );
    v.check_bool("narrate_action", resp.is_ok(), "game.narrate_action");

    let resp = call_game(
        ctx,
        "game.npc_dialogue",
        serde_json::json!({
            "npc_id": "innkeeper",
            "scene_id": "tavern",
            "player_stats": {"charisma": 12},
            "relationship": {"trust": 0.5, "encounters": 1},
            "available_options": [
                {"id": 1, "text": "Hello.", "skill_check": null}
            ]
        }),
    );
    v.check_bool("npc_dialogue", resp.is_ok(), "game.npc_dialogue");
}

fn phase_new_ipc_scene_completion(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let resp = call_game(
        ctx,
        "game.voice_check",
        serde_json::json!({
            "ability_id": "perception",
            "actor_stats": {"perception": 14, "level": 3},
            "target_dc": 12,
            "modifiers": [],
            "context": {"fitts_distance": 0.2, "time_pressure": false, "previous_attempts": 0}
        }),
    );
    v.check_bool("voice_check", resp.is_ok(), "game.voice_check");

    let resp = call_game(
        ctx,
        "game.push_scene",
        serde_json::json!({
            "session_id": "exp103-test",
            "scene": {"type": "dialogue", "background": "tavern", "narration": "Test scene."},
            "overlays": {"flow_indicator": 0.7}
        }),
    );
    v.check_bool("push_scene", resp.is_ok(), "game.push_scene");

    let resp = call_game(
        ctx,
        "game.complete_session",
        serde_json::json!({
            "session_id": "exp103-test",
            "outcome": "test",
            "stats": {"duration_secs": 10, "scenes_visited": 1, "choices_made": 0}
        }),
    );
    v.check_bool("complete_session", resp.is_ok(), "game.complete_session");
}

fn phase_new_methods(v: &mut ValidationResult) {
    v.section("New IPC Methods (6 for esotericWebb)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("game") && !ctx.has_capability("orchestration") {
        v.check_skip("new_methods", "ludoSpring not discovered");
        return;
    }

    phase_new_ipc_dialogue(v, &mut ctx);
    phase_new_ipc_scene_completion(v, &mut ctx);
}

fn main() {
    ValidationResult::new("primalSpring Exp103 — ludoSpring Expanded IPC")
        .with_provenance("exp103_ludospring_expanded_ipc", "2026-05-09")
        .run(
            "Exp103: Validate 14 ludoSpring IPC methods (8 existing + 6 new)",
            |v| {
                phase_existing_methods(v);
                phase_new_methods(v);
            },
        );
}
