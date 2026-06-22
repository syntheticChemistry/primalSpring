// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp088: Storytelling Composition — Tower + ludoSpring + esotericWebb via biomeOS.
//!
//! Uses [`CompositionContext::discover`] for full escalation (Songbird, UDS,
//! Neural API, TCP) and capability-keyed calls instead of ad hoc socket discovery.

use primalspring::composition::CompositionContext;
use primalspring::ipc::IpcError;
use primalspring::validation::ValidationResult;

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Composition discovery");
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    v.check_bool("has_security", ctx.has_capability("security"), "security");
    v.check_bool(
        "has_discovery",
        ctx.has_capability("discovery"),
        "discovery",
    );
}

fn orchestration_route(
    ctx: &mut CompositionContext,
    domain: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": domain,
            "operation": operation,
            "args": args,
        }),
    )
}

fn phase_tower(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 2: Tower Atomic");
    match ctx.health_check("security") {
        Ok(h) => v.check_bool("beardog", h, "BearDog security healthy"),
        Err(e) => v.check_skip("beardog", &format!("{e}")),
    }
    match ctx.health_check("discovery") {
        Ok(h) => v.check_bool("songbird", h, "Songbird discovery healthy"),
        Err(e) => v.check_skip("songbird", &format!("{e}")),
    }
}

fn phase_biomeos(v: &mut ValidationResult, ctx: &mut CompositionContext) -> bool {
    v.section("Phase 3: biomeOS substrate");
    if !ctx.has_capability("orchestration") {
        v.check_bool(
            "neural_api",
            false,
            "orchestration (biomeOS) not in composition",
        );
        return false;
    }
    match ctx.health_check("orchestration") {
        Ok(h) => {
            v.check_bool("neural_api", h, "biomeOS Neural API reachable");
            h
        }
        Err(e) => {
            v.check_skip("neural_api", &format!("{e}"));
            false
        }
    }
}

fn phase_ludospring(v: &mut ValidationResult, ctx: &mut CompositionContext, bridge_ok: bool) {
    v.section("Phase 4: ludoSpring game science");

    if ctx.has_capability("game") {
        let flow = ctx.call(
            "game",
            "game.evaluate_flow",
            serde_json::json!({"skill": 0.7, "challenge": 0.6}),
        );
        v.check_bool(
            "game.evaluate_flow",
            flow.as_ref().is_ok_and(|r| r.get("state").is_some()),
            "flow evaluation returns state",
        );

        let fitts = ctx.call(
            "game",
            "game.fitts_cost",
            serde_json::json!({"distance": 100.0, "target_width": 20.0}),
        );
        v.check_bool(
            "game.fitts_cost",
            fitts
                .as_ref()
                .is_ok_and(|r| r.get("movement_time_ms").is_some()),
            "Fitts cost returns movement_time_ms",
        );

        let noise = ctx.call(
            "game",
            "game.generate_noise",
            serde_json::json!({"x": 1.0, "y": 2.0}),
        );
        v.check_bool(
            "game.generate_noise",
            noise.is_ok(),
            "noise generation returns value",
        );
    } else {
        v.check_skip(
            "ludospring_health",
            "game capability not in CompositionContext",
        );
        v.check_skip("game.evaluate_flow", "requires game capability");
        v.check_skip("game.fitts_cost", "requires game capability");
        v.check_skip("game.generate_noise", "requires game capability");
    }

    if bridge_ok {
        match orchestration_route(
            ctx,
            "game",
            "evaluate_flow",
            &serde_json::json!({"skill": 0.5, "challenge": 0.5}),
        ) {
            Ok(_) => v.check_bool(
                "biomeos_game_routing",
                true,
                "biomeOS routes game.evaluate_flow → ludoSpring",
            ),
            Err(e) => v.check_skip("biomeos_game_routing", &format!("{e}")),
        }
    } else {
        v.check_skip(
            "biomeos_game_routing",
            "biomeOS orchestration not available",
        );
    }
}

fn call_narrative(
    ctx: &mut CompositionContext,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    if ctx.has_capability("narrative") {
        ctx.call("narrative", method, params.clone())
    } else if ctx.has_capability("orchestration") {
        orchestration_route(ctx, "narrative", method, params)
    } else {
        Err(IpcError::SocketNotFound {
            primal: "narrative".to_owned(),
        })
    }
}

fn phase_esotericwebb(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 5: esotericWebb narrative product");

    let liveness = call_narrative(ctx, "webb.liveness", &serde_json::Value::Null);
    match &liveness {
        Ok(_) => v.check_bool("webb_liveness", true, "esotericWebb liveness responds"),
        Err(e) => v.check_skip("webb_liveness", &format!("{e}")),
    }

    let state = call_narrative(ctx, "session.state", &serde_json::Value::Null);
    let has_session = state.as_ref().is_ok_and(|r| {
        r.get("current_node").is_some()
            || r.get("result")
                .and_then(|x| x.get("current_node"))
                .is_some()
    });
    match &state {
        Ok(_) => v.check_bool(
            "session.state",
            has_session,
            "active session with current_node",
        ),
        Err(e) => v.check_skip("session.state", &format!("{e}")),
    }

    let scene = call_narrative(ctx, "webb.scene.current", &serde_json::Value::Null);
    let has_scene = scene.as_ref().is_ok_and(|r| {
        r.get("scene").is_some() || r.get("result").and_then(|x| x.get("scene")).is_some()
    });
    match &scene {
        Ok(_) => v.check_bool("webb.scene.current", has_scene, "current scene available"),
        Err(e) => v.check_skip("webb.scene.current", &format!("{e}")),
    }

    let caps = call_narrative(ctx, "capabilities.list", &serde_json::json!({}));
    let cap_count = caps.ok().and_then(|r| {
        r.get("capabilities")
            .or_else(|| r.get("result").and_then(|x| x.get("capabilities")))
            .and_then(|c| c.as_array())
            .map(Vec::len)
    });
    match cap_count {
        Some(n) => v.check_minimum("webb_capabilities", n, 15),
        None => v.check_skip("webb_capabilities", "capabilities list unavailable"),
    }
}

fn phase_full_composition(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 6: Full composition mapping");
    if ctx.has_capability("game") {
        v.check_bool("discover_game", true, "game capability present");
    } else {
        v.check_skip(
            "discover_game",
            "game not registered in context (overlay may use orchestration only)",
        );
    }

    v.check_bool(
        "discover_narrative",
        ctx.has_capability("narrative") || ctx.has_capability("orchestration"),
        "narrative path or orchestration for esotericWebb",
    );
    v.check_bool(
        "discover_security",
        ctx.has_capability("security"),
        "security capability present",
    );
    v.check_bool(
        "discover_discovery_cap",
        ctx.has_capability("discovery"),
        "discovery capability present",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp088 — Storytelling Composition")
        .with_provenance("exp088_storytelling_composition", "2026-05-09")
        .run("Tower + ludoSpring + esotericWebb via biomeOS", |v| {
            let mut ctx = CompositionContext::discover();
            phase_composition_discovery(v, &ctx);
            phase_tower(v, &mut ctx);
            let bridge_ok = phase_biomeos(v, &mut ctx);
            phase_ludospring(v, &mut ctx, bridge_ok);
            phase_esotericwebb(v, &mut ctx);
            phase_full_composition(v, &ctx);
        });
}
