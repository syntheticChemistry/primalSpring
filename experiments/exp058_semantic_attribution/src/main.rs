// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp058: Semantic Attribution — commit and dag capabilities (sweetGrass / rhizoCrypt path).

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

const ATTRIBUTION_CAPS: &[&str] = &["attribution", "dag"];

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in ATTRIBUTION_CAPS {
        if ctx.has_capability(cap) {
            v.check_bool(
                &format!("has_{cap}"),
                true,
                &format!("{cap} capability present"),
            );
        } else {
            v.check_skip(&format!("has_{cap}"), &format!("{cap} not in context"));
        }
    }
}

fn phase_attribution(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mut any = false;
    for cap in ATTRIBUTION_CAPS {
        if !ctx.has_capability(cap) {
            continue;
        }
        any = true;
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &format!("attribution_{cap}_liveness"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("attribution_{cap}_liveness"), &format!("{e}"));
            }
            Err(e) => v.check_bool(
                &format!("attribution_{cap}_liveness"),
                false,
                &format!("error: {e}"),
            ),
        }
    }
    if !any {
        v.check_skip("attribution_live", "no attribution/dag clients in context");
    }

    v.check_skip(
        "semantic_tracking",
        "semantic tracking needs live sweetgrass",
    );
    v.check_skip("braid_formation", "braid formation needs live primals");
}

fn main() {
    ValidationResult::new("primalSpring Exp058 — Semantic Attribution")
        .with_provenance("exp058_semantic_attribution", "2026-05-09")
        .run(
            "primalSpring Exp058: sweetGrass Semantic Attribution (RootPulse)",
            |v| {
                v.section("Phase 1: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 2: Attribution");
                phase_attribution(v, &mut ctx);
            },
        );
}
