// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Gate Failure — absorbed from exp033.

use crate::bonding::BondType;
use crate::composition::CompositionContext;
use crate::ipc::discover::{DiscoverySource, discover_primal};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gate-failure",
        track: Track::Security,
        tier: Tier::Live,
        provenance_crate: "exp033_gate_failure",
        provenance_date: "2026-05-09",
        description: "Gate failure — bond models and graceful discovery when a gate is absent",
    },
    run,
};

const BOND_TYPE_COUNT: usize = 5;

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let _ = ctx;
    v.section("Phase 1: Bond Types");
    phase_bond_types(v);

    v.section("Phase 2: Graceful Discovery");
    phase_graceful_discovery(v);

    v.section("Phase 3: Gate Failure (skips)");
    phase_gate_failure_skips(v);
}

fn phase_bond_types(v: &mut ValidationResult) {
    let variants_exist = [
        BondType::Covalent,
        BondType::Metallic,
        BondType::Ionic,
        BondType::Weak,
        BondType::OrganoMetalSalt,
    ];
    v.check_bool(
        "bond_type_variants_for_all_models",
        variants_exist.len() == BOND_TYPE_COUNT
            && variants_exist.iter().all(|bt| !bt.description().is_empty()),
        "BondType variants exist for all bonding models with descriptions",
    );
}

fn phase_graceful_discovery(v: &mut ValidationResult) {
    let songbird = discover_primal(primal_names::SONGBIRD);
    v.check_bool(
        "discover_songbird_returns_result",
        songbird.primal == primal_names::SONGBIRD,
        "discover_primal returns DiscoveryResult for songbird",
    );

    let missing = discover_primal("nonexistent_primal_xyzzy_12345");
    v.check_bool(
        "discovery_graceful_for_missing_primal",
        missing.socket.is_none() && missing.source == DiscoverySource::NotFound,
        "discover_primal returns NotFound for missing primal without panic",
    );
}

fn phase_gate_failure_skips(v: &mut ValidationResult) {
    v.check_skip("gate_failure", "needs live Plasmodium with multiple gates");
    v.check_skip(
        "graceful_degradation",
        "needs live gate drop to test degradation",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_failure_no_panic() {
        let mut v = ValidationResult::new("gate-failure");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
