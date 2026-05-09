// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp003: Nest Atomic — validates security, discovery, storage, and AI capabilities.
//!
//! Phases:
//!   1. Structural — required capability count for `AtomicType::Nest`
//!   2. Discovery — `CompositionContext` resolves each Nest capability
//!   3. Health — `health.liveness` per required capability

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let nest_caps = AtomicType::Nest.required_capabilities();
    v.check_count("nest_required_caps", nest_caps.len(), 4);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let nest_caps = AtomicType::Nest.required_capabilities();
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_context_nonempty",
        !caps.is_empty(),
        &format!("{} context capabilities: {}", caps.len(), caps.join(", ")),
    );
    for cap in nest_caps {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let nest_caps = AtomicType::Nest.required_capabilities();
    for cap in nest_caps {
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Err(e) if e.is_connection_error() => v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not reachable: {e}"),
            ),
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} health.liveness error: {e}"),
            ),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp003 — Nest Atomic")
        .with_provenance("exp003_nest_atomic", "2026-05-09")
        .run(
            "primalSpring Exp003: Nest Atomic (security + discovery + storage + ai)",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 3: Health");
                phase_health(v, &mut ctx);
            },
        );
}
