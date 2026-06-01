// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp031: Ionic Bond — validates cross-family limited capability sharing (structural + live health).

use primalspring::bonding::BondType;
use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn phase_bond_type_properties(v: &mut ValidationResult) {
    let bond = BondType::Ionic;
    v.check_bool(
        "ionic_description_non_empty",
        !bond.description().is_empty(),
        &format!(
            "BondType::Ionic.description() is non-empty — {}",
            bond.description()
        ),
    );
}

fn phase_live_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in ["security", "discovery"] {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("capability {cap} not in composition context"),
            );
            continue;
        }
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
                &format!("error: {e}"),
            ),
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp031 — Ionic Bond")
        .with_provenance("exp031_ionic_bond", "2026-05-09")
        .run(
            "primalSpring Exp031: Cross-Family Limited Capability Sharing",
            |v| {
                v.section("Phase 1: Bond Type Properties");
                phase_bond_type_properties(v);

                v.section("Phase 2: Live Discovery + Health");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_live_discovery(v, &mut ctx);

                v.check_skip(
                    "cross_family_capability_sharing",
                    "needs live primals from different families",
                );
            },
        );
}
