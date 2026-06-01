// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp033: Gate Failure — bond models + graceful discovery when a gate is absent.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::{DiscoverySource, discover_primal};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

const BOND_TYPE_COUNT: usize = 5;

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

fn main() {
    ValidationResult::new("primalSpring Exp033 — Gate Failure")
        .with_provenance("exp033_gate_failure", "2026-05-09")
        .run(
            "primalSpring Exp033: One Gate Drops; Plasmodium Degrades Gracefully",
            |v| {
                v.section("Phase 1: Bond Types");
                phase_bond_types(v);

                v.section("Phase 2: Graceful Discovery");
                phase_graceful_discovery(v);

                v.section("Phase 3: Gate Failure (skips)");
                phase_gate_failure_skips(v);
            },
        );
}
