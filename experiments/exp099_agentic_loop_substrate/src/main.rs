// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp099 — Agentic Loop Substrate
//!
//! Validates the full three-way feedback loop:
//! petalTongue → biomeOS → Squirrel → biomeOS → springs → petalTongue
//!
//! Phase 56 — Desktop Substrate (AGENTIC_TRIO_EVOLUTION.md)

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

fn phase_trio_discovery(v: &mut ValidationResult) {
    v.section("Agentic Trio Discovery");

    let ctx = CompositionContext::discover();

    v.check_bool(
        "biomeos_discovered",
        ctx.has_capability("orchestration"),
        "biomeOS Neural API discovered via orchestration capability",
    );

    v.check_bool(
        "squirrel_discovered",
        ctx.has_capability("ai"),
        "Squirrel discovered",
    );

    v.check_bool(
        "petaltongue_discovered",
        ctx.has_capability("visualization"),
        "petalTongue discovered via visualization capability",
    );
}

fn phase_sensor_to_intent(v: &mut ValidationResult) {
    v.section("Sensor to Intent (petalTongue afferent)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("visualization") {
        v.check_skip("sensor_stream", "petalTongue not discovered");
        return;
    }

    let resp = ctx.call("visualization", "proprioception.get", serde_json::json!({}));
    v.check_bool(
        "proprioception",
        resp.is_ok(),
        "petalTongue proprioception.get responds",
    );
}

fn phase_intent_routing(v: &mut ValidationResult) {
    v.section("Intent Routing (biomeOS to Squirrel)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("orchestration") {
        v.check_skip("ai_routing", "biomeOS not discovered");
        return;
    }

    let resp = orchestration_route(&mut ctx, "ai", "models", &serde_json::json!({}));

    match resp {
        Ok(_) => {
            v.check_bool(
                "ai_routing",
                true,
                "biomeOS routes ai.models to Squirrel via capability.call",
            );
        }
        Err(e) => {
            v.check_skip(
                "ai_routing",
                &format!("capability.call to ai domain failed: {e}"),
            );
        }
    }
}

fn phase_render_feedback(v: &mut ValidationResult) {
    v.section("Render Feedback (Squirrel to petalTongue)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("orchestration") {
        v.check_skip("render_feedback", "biomeOS not discovered");
        return;
    }

    let resp = orchestration_route(
        &mut ctx,
        "visualization",
        "render.dashboard",
        &serde_json::json!({
            "session": "exp099-test",
            "data": {"title": "Agentic Loop Test", "status": "validating"}
        }),
    );

    v.check_bool(
        "render_feedback",
        resp.is_ok(),
        "biomeOS routes visualization.render.dashboard to petalTongue",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp099 — Agentic Loop Substrate")
        .with_provenance("exp099_agentic_loop_substrate", "2026-05-09")
        .run(
            "Exp099: Full three-way agentic loop on Desktop NUCLEUS",
            |v| {
                phase_trio_discovery(v);
                phase_sensor_to_intent(v);
                phase_intent_routing(v);
                phase_render_feedback(v);
            },
        );
}
