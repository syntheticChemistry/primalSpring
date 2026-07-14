// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Drawbridge Bonds — validates drawbridge bond tiers and weak-bond
//! ingress semantics at the outer membrane boundary.

use crate::bonding::content_distribution::DistributionBondTier;
use crate::bonding::{BondType, TrustModel};
use crate::composition::{CompositionContext, capability_to_primal};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PORTS_TOML: &str = include_str!("../../../../config/ports.toml");

/// Drawbridge bond tier labels from the K-Derm weak-bond model.
const BOND_TIERS: &[(&str, BondType, TrustModel)] = &[
    ("scientific", BondType::Ionic, TrustModel::Contractual),
    ("community", BondType::Metallic, TrustModel::MitoBeaconFamily),
    ("commercial", BondType::Ionic, TrustModel::Contractual),
    ("municipal", BondType::Covalent, TrustModel::Organizational),
    ("untrusted", BondType::Weak, TrustModel::ZeroTrust),
];

/// Drawbridge bonds scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "drawbridge-bonds",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave138a_drawbridge_bonds",
        provenance_date: "2026-07-14",
        description: "Drawbridge bonds — bond tier semantics at the outer membrane drawbridge",
    },
    run,
};

/// Run drawbridge bonds validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Drawbridge methods in capability registry");

    let has_drawbridge = REGISTRY_TOML.contains("drawbridge")
        || REGISTRY_TOML.contains("http.proxy")
        || PORTS_TOML.contains("[gateway.drawbridge]");
    v.check_bool(
        "drawbridge:registered",
        has_drawbridge,
        "drawbridge crossing registered via http.proxy or ports.toml gateway.drawbridge",
    );

    let bonding_methods = ["bonding.propose", "bonding.accept", "bonding.status"];
    for method in bonding_methods {
        v.check_bool(
            &format!("bonding:{}", method.replace("bonding.", "")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered for drawbridge bond negotiation"),
        );
    }

    v.section("Phase 2: Bond tier semantics");

    for (tier_name, bond_type, trust_model) in BOND_TIERS {
        v.check_bool(
            &format!("tier:{tier_name}:bond_type"),
            !bond_type.description().is_empty(),
            &format!("{tier_name} → {} ({})", bond_type.description(), bond_type),
        );
        v.check_bool(
            &format!("tier:{tier_name}:trust_model"),
            !trust_model.is_genetic() || *tier_name != "untrusted",
            &format!("{tier_name} trust model: {trust_model:?}"),
        );
    }

    v.check_bool(
        "tier:untrusted:zero_trust",
        DistributionBondTier::Weak.required_trust() == TrustModel::ZeroTrust,
        "untrusted tier maps to Weak bond + ZeroTrust model",
    );

    v.section("Phase 3: Drawbridge owner and weak-bond crypto");

    let http_owner = capability_to_primal("http");
    v.check_bool(
        "drawbridge:songbird_owner",
        http_owner == "songbird",
        &format!("drawbridge http owner: {http_owner}"),
    );

    v.check_bool(
        "drawbridge:weak_bond_crypto",
        REGISTRY_TOML.contains("crypto.ionic_bond.verify_proposal"),
        "crypto.ionic_bond.verify_proposal for weak-bond ingress verification",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
