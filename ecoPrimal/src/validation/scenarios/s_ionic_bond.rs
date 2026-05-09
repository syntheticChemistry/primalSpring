// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Ionic Bond — absorbed from exp031.

use crate::bonding::BondType;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "ionic-bond",
        track: Track::Bonding,
        tier: Tier::Both,
        provenance_crate: "exp031_ionic_bond",
        provenance_date: "2026-05-09",
        description: "Ionic bond — cross-family limited capability sharing",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Bond Type Properties");
    phase_bond_type_properties(v);

    v.section("Phase 2: Live Discovery + Health");
    phase_live_discovery(v, ctx);

    v.check_skip(
        "cross_family_capability_sharing",
        "needs live primals from different families",
    );
}

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
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
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
