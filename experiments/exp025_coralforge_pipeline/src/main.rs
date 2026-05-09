// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp025: CoralForge pipeline graph and composition readiness.

use primalspring::composition::CompositionContext;
use primalspring::emergent::EmergentSystem;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let graphs = EmergentSystem::CoralForge.required_graphs();
    let has_pipeline = graphs.contains(&"coralforge_pipeline");
    v.check_bool(
        "coralforge_has_pipeline_graph",
        has_pipeline,
        &format!(
            "EmergentSystem::CoralForge.required_graphs() contains coralforge_pipeline: {graphs:?}"
        ),
    );
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in ["compute", "shader", "storage"] {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability for CoralForge pipeline"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in ["compute", "shader", "storage"] {
        if !ctx.has_capability(cap) {
            v.check_skip(&format!("health_{cap}"), &format!("{cap} not resolved"));
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("health_{cap}"),
                true,
                &format!("{cap} health.liveness"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("health_{cap}"), &format!("{cap}: {e}"));
            }
            Err(e) => v.check_bool(&format!("health_{cap}"), false, &format!("error: {e}")),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp025 — CoralForge Pipeline")
        .with_provenance("exp025_coralforge_pipeline", "2026-05-09")
        .run(
            "primalSpring Exp025: Pipeline Graph over neuralSpring + wetSpring + toadStool",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 3: Health");
                phase_health(v, &mut ctx);

                v.check_skip(
                    "actual_pipeline_execution",
                    "actual pipeline execution needs live IPC",
                );
            },
        );
}
