// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp053: Multi-Primal Lifecycle — FullNucleus composition discovery and health.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

const LIFECYCLE_PARTICIPANT_COUNT: usize = 6;

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let required_caps = AtomicType::FullNucleus.required_capabilities();
    v.check_count("full_nucleus_capability_count", required_caps.len(), 13);

    let found = required_caps
        .iter()
        .filter(|&&c| ctx.has_capability(c))
        .count();
    println!(
        "  [INFO] discovered {found}/{} required capabilities",
        required_caps.len()
    );
}

fn phase_composition_health(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    required_caps: &[&'static str],
    found: usize,
) {
    if found < LIFECYCLE_PARTICIPANT_COUNT {
        v.check_skip(
            "lifecycle_participants",
            &format!("need >= {LIFECYCLE_PARTICIPANT_COUNT} live capabilities, found {found}"),
        );
        v.check_skip(
            "composition_health",
            "insufficient live capabilities for FullNucleus health pass",
        );
        return;
    }

    let mut ok_liveness = 0usize;
    for &cap in required_caps {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not in context"),
            );
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => {
                ok_liveness += 1;
                v.check_bool(
                    &format!("health_liveness_{cap}"),
                    true,
                    &format!("{cap} health.liveness ok"),
                );
            }
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

    v.check_minimum(
        "lifecycle_participants",
        ok_liveness,
        LIFECYCLE_PARTICIPANT_COUNT,
    );

    if required_caps.iter().all(|&c| ctx.has_capability(c)) {
        v.check_bool(
            "composition_discovery",
            true,
            "every FullNucleus capability has a client in context",
        );
    } else {
        v.check_skip(
            "composition_discovery",
            "not all FullNucleus capabilities discovered",
        );
    }
}

fn phase_lifecycle(v: &mut ValidationResult) {
    v.check_skip(
        "lifecycle_orchestration",
        "end-to-end lifecycle orchestration requires graph execution",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp053 — Multi-Primal Lifecycle")
        .with_provenance("exp053_multi_primal_lifecycle", "2026-05-09")
        .run(
            "primalSpring Exp053: 6-Primal Research Paper Lifecycle",
            |v| {
                v.section("Phase 1: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                let required_caps = AtomicType::FullNucleus.required_capabilities();
                let found = required_caps
                    .iter()
                    .filter(|&&c| ctx.has_capability(c))
                    .count();
                phase_discovery(v, &ctx);

                v.section("Phase 2: Composition Health");
                phase_composition_health(v, &mut ctx, required_caps, found);

                v.section("Phase 3: Lifecycle");
                phase_lifecycle(v);
            },
        );
}
