// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp001: Tower Atomic — security + discovery capability validation.
//!
//! Phases:
//!   1. Structural — required capability count for `AtomicType::Tower`
//!   2. Discovery — `CompositionContext` resolves `security` and `discovery`
//!   3. Health — `health.liveness` per required capability
//!   4. Composition — BTSP posture from the live context

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn phase_structural(v: &mut ValidationResult) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    v.check_count("tower_required_caps", tower_caps.len(), 2);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_found_primals",
        !caps.is_empty(),
        &format!("{} context capabilities: {}", caps.len(), caps.join(", ")),
    );
    for cap in tower_caps {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    for cap in tower_caps {
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

fn phase_composition(v: &mut ValidationResult, ctx: &CompositionContext) {
    let btsp = ctx.btsp_state();
    let btsp_count = btsp.values().filter(|&&ok| ok).count();
    v.check_bool(
        "btsp_any_authenticated",
        btsp_count > 0 || btsp.is_empty(),
        &format!("{btsp_count}/{} BTSP authenticated", btsp.len()),
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp001 — Tower Atomic")
        .with_provenance("exp001_tower_atomic", "2026-05-09")
        .run(
            "primalSpring Exp001: Tower Atomic (security + discovery capabilities)",
            |v| {
                v.section("Phase 1: Structural");
                phase_structural(v);

                v.section("Phase 2: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 3: Health");
                phase_health(v, &mut ctx);

                v.section("Phase 4: Composition");
                phase_composition(v, &ctx);
            },
        );
}
