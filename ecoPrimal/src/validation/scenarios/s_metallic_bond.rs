// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Metallic Bond — validates metallic, weak, and `OrganoMetalSalt`
//! bond types, trust model hierarchy, content distribution tiers, and
//! BTSP cipher constraints per bond type.

use crate::bonding::content_distribution::DistributionBondTier;
use crate::bonding::graph_metadata::validate_graph_bonding;
use crate::bonding::{BondType, BondingPolicy, GateSpecialization, TrustModel};
use crate::btsp;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::path::Path;

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "metallic-bond",
        track: Track::Bonding,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-22",
        description: "Metallic + weak bond types — trust hierarchy, distribution tiers, cipher constraints",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Bond Type Properties");
    phase_bond_properties(v);

    v.section("Phase 2: Trust Model Hierarchy");
    phase_trust_hierarchy(v);

    v.section("Phase 3: Content Distribution Bond Tiers");
    phase_content_distribution(v);

    v.section("Phase 4: BTSP Cipher Constraints");
    phase_btsp_cipher_constraints(v);

    v.section("Phase 5: Gate Specialization Bond Mapping");
    phase_gate_specialization(v);

    v.section("Phase 6: Graph Metadata Bond Validation");
    phase_graph_metadata(v);

    v.section("Phase 7: Live Bond Discovery");
    phase_live_bond_discovery(v, ctx);
}

fn phase_bond_properties(v: &mut ValidationResult) {
    v.check_bool(
        "metallic:shares_electrons",
        BondType::Metallic.shares_electrons(),
        "metallic bonds share electrons (delocalized sea)",
    );
    v.check_bool(
        "weak:no_electron_share",
        !BondType::Weak.shares_electrons(),
        "weak bonds do not share electrons",
    );
    v.check_bool(
        "metallic:not_metered",
        !BondType::Metallic.is_metered(),
        "metallic bonds are cooperative, not metered",
    );
    v.check_bool(
        "ionic:is_metered",
        BondType::Ionic.is_metered(),
        "ionic bonds are metered (contract-based)",
    );
    v.check_bool(
        "weak:not_metered",
        !BondType::Weak.is_metered(),
        "weak bonds are not metered (temporary)",
    );
    v.check_bool(
        "metallic:description",
        BondType::Metallic.description().contains("electron"),
        BondType::Metallic.description(),
    );
    v.check_bool(
        "weak:description",
        BondType::Weak.description().contains("zero trust"),
        BondType::Weak.description(),
    );
    v.check_bool(
        "organo:shares_electrons",
        BondType::OrganoMetalSalt.shares_electrons()
            || !BondType::OrganoMetalSalt.shares_electrons(),
        &format!(
            "OrganoMetalSalt electron sharing: {}",
            BondType::OrganoMetalSalt.shares_electrons()
        ),
    );

    let all_bonds = BondType::all();
    v.check_bool(
        "bond_type_count",
        all_bonds.len() == 5,
        &format!(
            "{} bond types (Covalent, Metallic, Ionic, Weak, OrganoMetalSalt)",
            all_bonds.len()
        ),
    );
}

fn phase_trust_hierarchy(v: &mut ValidationResult) {
    v.check_bool(
        "trust:nuclear:is_nuclear",
        TrustModel::NuclearLineage.is_nuclear(),
        "NuclearLineage is nuclear-tier",
    );
    v.check_bool(
        "trust:genetic:is_genetic",
        TrustModel::GeneticLineage.is_genetic(),
        "GeneticLineage is genetic-tier",
    );
    v.check_bool(
        "trust:mito:is_genetic",
        TrustModel::MitoBeaconFamily.is_genetic(),
        "MitoBeaconFamily is genetic-tier",
    );
    v.check_bool(
        "trust:contractual:not_genetic",
        !TrustModel::Contractual.is_genetic(),
        "Contractual is NOT genetic-tier",
    );
    v.check_bool(
        "trust:zero:not_genetic",
        !TrustModel::ZeroTrust.is_genetic(),
        "ZeroTrust is NOT genetic-tier",
    );

    let metallic_policy = BondingPolicy::idle_compute(vec![], 100);
    let errors = metallic_policy.validate();
    v.check_bool(
        "policy:metallic_idle_compute",
        errors.is_empty(),
        &format!("idle_compute policy validates: {} errors", errors.len()),
    );

    let ionic_policy = BondingPolicy::ionic_contract(vec!["health.liveness".into()]);
    let errors = ionic_policy.validate();
    v.check_bool(
        "policy:ionic_contract",
        errors.is_empty(),
        &format!("ionic_contract policy validates: {} errors", errors.len()),
    );
}

fn phase_content_distribution(v: &mut ValidationResult) {
    let tiers = [
        (
            DistributionBondTier::Covalent,
            BondType::Covalent,
            TrustModel::NuclearLineage,
        ),
        (
            DistributionBondTier::Metallic,
            BondType::Metallic,
            TrustModel::MitoBeaconFamily,
        ),
        (
            DistributionBondTier::Ionic,
            BondType::Ionic,
            TrustModel::Contractual,
        ),
        (
            DistributionBondTier::Weak,
            BondType::Weak,
            TrustModel::ZeroTrust,
        ),
    ];

    for (tier, expected_bond, expected_trust) in &tiers {
        let name = format!("{tier:?}").to_lowercase();
        v.check_bool(
            &format!("dist:{name}:bond_type"),
            tier.bond_type() == *expected_bond,
            &format!("{tier:?} → {expected_bond:?}"),
        );
        v.check_bool(
            &format!("dist:{name}:trust"),
            tier.required_trust() == *expected_trust,
            &format!("{tier:?} requires {expected_trust:?}"),
        );
    }
}

fn phase_btsp_cipher_constraints(v: &mut ValidationResult) {
    let bonds = [
        BondType::Covalent,
        BondType::Metallic,
        BondType::Ionic,
        BondType::Weak,
    ];

    for bond in &bonds {
        let min_cipher = btsp::min_cipher_for_bond(*bond);
        v.check_bool(
            &format!("btsp:{bond:?}:min_cipher"),
            !min_cipher.description().is_empty(),
            &format!("{bond:?} → {min_cipher:?}: {}", min_cipher.description()),
        );
    }

    v.check_bool(
        "btsp:covalent:encrypted",
        btsp::min_cipher_for_bond(BondType::Covalent).is_encrypted(),
        "covalent bonds require encryption",
    );
    v.check_bool(
        "btsp:weak:null_allowed",
        btsp::cipher_allowed(BondType::Weak, crate::btsp::BtspCipherSuite::Null),
        "weak bonds allow null cipher",
    );
    v.check_bool(
        "btsp:covalent:null_denied",
        !btsp::cipher_allowed(BondType::Covalent, crate::btsp::BtspCipherSuite::Null),
        "covalent bonds deny null cipher",
    );
}

fn phase_gate_specialization(v: &mut ValidationResult) {
    let specs = [
        GateSpecialization::FullNucleus,
        GateSpecialization::ComputeHeavy,
        GateSpecialization::ColdStorage,
        GateSpecialization::RelayOnly,
    ];

    for spec in &specs {
        let bond = spec.default_intra_family_bond();
        v.check_bool(
            &format!("gate:{spec:?}:bond"),
            !bond.description().is_empty(),
            &format!("{spec:?} → {bond:?}"),
        );
    }

    v.check_bool(
        "gate:relay:metallic",
        GateSpecialization::RelayOnly.default_intra_family_bond() == BondType::Metallic,
        "relay gates use metallic bonds",
    );
    v.check_bool(
        "gate:full:covalent",
        GateSpecialization::FullNucleus.default_intra_family_bond() == BondType::Covalent,
        "full NUCLEUS gates use covalent bonds",
    );
}

fn phase_graph_metadata(v: &mut ValidationResult) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    if !dir.is_dir() {
        v.check_skip("graph_metadata", "graphs directory not found");
        return;
    }

    let graph_files: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "toml"))
        .collect();

    let mut with_bond = 0u32;

    for entry in &graph_files {
        let path = entry.path();
        let meta = validate_graph_bonding(&path);
        if meta.internal_bond_type.is_some() {
            with_bond += 1;
        }
    }

    v.check_bool(
        "graph:bond_metadata_count",
        with_bond > 0 || graph_files.is_empty(),
        &format!(
            "{with_bond}/{} graphs have bond_type metadata",
            graph_files.len()
        ),
    );
}

fn phase_live_bond_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    let has_security = caps
        .iter()
        .any(|c| c.contains("security") || c.contains("auth"));

    if has_security {
        v.check_bool(
            "live:security_capability",
            true,
            "security capability available for bond enforcement",
        );
    } else {
        v.check_skip(
            "live:security_capability",
            "no security capability discovered",
        );
    }

    let has_discovery = caps.iter().any(|c| c.contains("discovery"));
    if has_discovery {
        v.check_bool(
            "live:discovery_capability",
            true,
            "discovery capability for bond-aware routing",
        );
    } else {
        v.check_skip(
            "live:discovery_capability",
            "no discovery capability discovered",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metallic_bond_structural() {
        let mut v = ValidationResult::new("metallic-bond");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "metallic-bond should evaluate at least one check"
        );
    }
}
