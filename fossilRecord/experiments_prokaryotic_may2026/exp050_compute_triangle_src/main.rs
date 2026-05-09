// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp050: Compute Triangle — coralReef → toadStool → barraCuda pipeline (shader → compute → tensor).

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

const COMPUTE_TRIANGLE: &[&str] = &["shader", "compute", "tensor"];

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    println!(
        "  [INFO] composition context: {} capability(ies) — {}",
        caps.len(),
        caps.join(", ")
    );

    for cap in COMPUTE_TRIANGLE {
        if ctx.has_capability(cap) {
            v.check_bool(
                &format!("has_{cap}"),
                true,
                &format!("{cap} capability discoverable"),
            );
        } else {
            v.check_skip(
                &format!("has_{cap}"),
                &format!("{cap} not present in context"),
            );
        }
    }
}

fn phase_health_capabilities(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in COMPUTE_TRIANGLE {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not present in context"),
            );
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("health_liveness_{cap}"), &format!("{cap}: {e}"));
            }
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("error: {e}"),
            ),
        }
    }
}

fn phase_pipeline(v: &mut ValidationResult) {
    v.check_skip(
        "compile_dispatch_pipeline",
        "actual compile+dispatch pipeline needs live primals",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp050 — Compute Triangle")
        .with_provenance("exp050_compute_triangle", "2026-05-09")
        .run(
            "primalSpring Exp050: coralReef → toadStool → barraCuda (capabilities)",
            |v| {
                v.section("Phase 1: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 2: Health + Capabilities");
                phase_health_capabilities(v, &mut ctx);

                v.section("Phase 3: Pipeline");
                phase_pipeline(v);
            },
        );
}
