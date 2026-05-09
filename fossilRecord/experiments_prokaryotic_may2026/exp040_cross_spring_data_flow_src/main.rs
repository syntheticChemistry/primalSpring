// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp040: Cross Spring Data Flow — ecology routing endpoints via `CompositionContext`.

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_context_non_empty",
        !caps.is_empty(),
        &format!("capabilities: {}", caps.join(", ")),
    );

    for cap in ["visualization", "ai"] {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} in context"),
        );
    }
}

fn phase_routing_skips(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if ctx.has_capability("orchestration") {
        match ctx.call("orchestration", "health.liveness", serde_json::json!({})) {
            Ok(_) => {
                v.check_bool(
                    "orchestration_reachable",
                    true,
                    "orchestration health.liveness ok",
                );
                v.check_skip(
                    "cross_spring_data_flow",
                    "end-to-end flow requires airSpring + wetSpring + neuralSpring registered",
                );
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip("orchestration_reachable", &format!("{e}"));
                v.check_skip(
                    "cross_spring_data_flow",
                    "needs live spring primals for capability routing",
                );
            }
            Err(e) => {
                v.check_bool("orchestration_reachable", false, &format!("error: {e}"));
                v.check_skip(
                    "cross_spring_data_flow",
                    "needs live spring primals for capability routing",
                );
            }
        }
    } else {
        v.check_skip(
            "orchestration_reachable",
            "orchestration capability not in context",
        );
        v.check_skip(
            "cross_spring_data_flow",
            "needs live spring primals for capability routing",
        );
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp040 — Cross Spring Data Flow")
        .with_provenance("exp040_cross_spring_data_flow", "2026-05-09")
        .run(
            "primalSpring Exp040: Cross Spring Data Flow (ecology pipeline via capability routing)",
            |v| {
                v.section("Phase 1: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 2: Routing (skips)");
                phase_routing_skips(v, &mut ctx);
            },
        );
}
