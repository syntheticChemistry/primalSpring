// SPDX-License-Identifier: AGPL-3.0-or-later
use super::*;

#[test]
fn all_bond_types_have_descriptions() {
    for bt in BondType::all() {
        assert!(!bt.description().is_empty(), "{bt:?} has empty description");
    }
}

#[test]
fn bond_type_count() {
    assert_eq!(BondType::all().len(), 5);
}

#[test]
fn bond_type_round_trip_json() {
    for bt in BondType::all() {
        let json = serde_json::to_string(bt).unwrap();
        let back: BondType = serde_json::from_str(&json).unwrap();
        assert_eq!(*bt, back);
    }
}

#[test]
fn electron_sharing() {
    assert!(BondType::Covalent.shares_electrons());
    assert!(BondType::Metallic.shares_electrons());
    assert!(!BondType::Ionic.shares_electrons());
    assert!(!BondType::Weak.shares_electrons());
}

#[test]
fn metering() {
    assert!(BondType::Ionic.is_metered());
    assert!(!BondType::Covalent.is_metered());
}

#[test]
fn constraint_permits() {
    let c = BondingConstraint {
        capability_allow: vec!["compute.*".to_owned()],
        capability_deny: vec!["compute.admin".to_owned()],
        bandwidth_limit_mbps: 100,
        max_concurrent_requests: 4,
    };
    assert!(c.permits("compute.submit"));
    assert!(c.permits("compute.status"));
    assert!(!c.permits("compute.admin"));
    assert!(!c.permits("storage.store"));
}

#[test]
fn constraint_empty_allow_permits_all() {
    let c = BondingConstraint::default();
    assert!(c.permits("anything"));
    assert!(c.permits("compute.submit"));
}

#[test]
fn policy_covalent_default_validates() {
    let p = BondingPolicy::covalent_default();
    assert!(p.validate().is_empty());
}

#[test]
fn policy_idle_compute_validates() {
    let p = BondingPolicy::idle_compute(vec!["22:00-06:00".to_owned()], 100);
    assert!(p.validate().is_empty());
    assert!(p.constraints.permits("compute.submit"));
    assert!(!p.constraints.permits("storage.store"));
    assert!(!p.constraints.permits("ai.query"));
}

#[test]
fn policy_validation_catches_covalent_without_nuclear() {
    let p = BondingPolicy {
        bond_type: BondType::Covalent,
        trust_model: TrustModel::Contractual,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "bad".to_owned(),
    };
    let errors = p.validate();
    assert!(!errors.is_empty());
    assert!(errors[0].contains("nuclear"));
}

#[test]
fn covalent_accepts_nuclear_lineage() {
    let p = BondingPolicy {
        bond_type: BondType::Covalent,
        trust_model: TrustModel::NuclearLineage,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: true,
        label: "nuclear-cov".to_owned(),
    };
    assert!(p.validate().is_empty());
}

#[test]
fn covalent_accepts_legacy_genetic_lineage() {
    let p = BondingPolicy::covalent_default();
    assert!(p.validate().is_empty());
}

#[test]
fn covalent_rejects_mito_only() {
    let p = BondingPolicy {
        bond_type: BondType::Covalent,
        trust_model: TrustModel::MitoBeaconFamily,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "mito-only".to_owned(),
    };
    let errors = p.validate();
    assert!(!errors.is_empty());
    assert!(errors[0].contains("nuclear"));
}

#[test]
fn metallic_accepts_mito_beacon() {
    let p = BondingPolicy {
        bond_type: BondType::Metallic,
        trust_model: TrustModel::MitoBeaconFamily,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "metallic-mito".to_owned(),
    };
    assert!(p.validate().is_empty());
}

#[test]
fn metallic_rejects_contractual() {
    let p = BondingPolicy {
        bond_type: BondType::Metallic,
        trust_model: TrustModel::Contractual,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "bad-metallic".to_owned(),
    };
    let errors = p.validate();
    assert!(!errors.is_empty());
    assert!(errors[0].contains("mito-beacon"));
}

#[test]
fn policy_min_btsp_cipher() {
    use crate::btsp::BtspCipherSuite;
    let cov = BondingPolicy::covalent_default();
    assert_eq!(cov.min_btsp_cipher(), BtspCipherSuite::Null);

    let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
    assert_eq!(ionic.min_btsp_cipher(), BtspCipherSuite::ChaCha20Poly1305);
}

#[test]
fn policy_btsp_cipher_allowed() {
    use crate::btsp::BtspCipherSuite;
    let cov = BondingPolicy::covalent_default();
    assert!(cov.btsp_cipher_allowed(BtspCipherSuite::Null));
    assert!(cov.btsp_cipher_allowed(BtspCipherSuite::ChaCha20Poly1305));

    let ionic = BondingPolicy::ionic_contract(vec![]);
    assert!(!ionic.btsp_cipher_allowed(BtspCipherSuite::Null));
    assert!(!ionic.btsp_cipher_allowed(BtspCipherSuite::HmacPlain));
    assert!(ionic.btsp_cipher_allowed(BtspCipherSuite::ChaCha20Poly1305));
}

#[test]
fn btsp_enforcer_connection_upgrade() {
    use crate::btsp::BtspCipherSuite;
    let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
    let decision = BtspEnforcer::evaluate_connection(&ionic, BtspCipherSuite::Null);
    assert!(decision.allowed);
    assert_eq!(decision.cipher, BtspCipherSuite::ChaCha20Poly1305);
    assert!(decision.reason.contains("upgraded"));
}

#[test]
fn btsp_enforcer_connection_accepted() {
    use crate::btsp::BtspCipherSuite;
    let cov = BondingPolicy::covalent_default();
    let decision = BtspEnforcer::evaluate_connection(&cov, BtspCipherSuite::Null);
    assert!(decision.allowed);
    assert_eq!(decision.cipher, BtspCipherSuite::Null);
    assert!(decision.reason.contains("granted"));
}

#[test]
fn btsp_enforcer_denies_covalent_without_nuclear() {
    use crate::btsp::BtspCipherSuite;
    let cov = BondingPolicy::covalent_default();
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &cov,
        BtspCipherSuite::Null,
        Some(TrustModel::MitoBeaconFamily),
    );
    assert!(!decision.allowed, "covalent should deny mito-only peer");
    assert!(decision.reason.contains("denied"));
}

#[test]
fn btsp_enforcer_allows_covalent_with_nuclear() {
    use crate::btsp::BtspCipherSuite;
    let cov = BondingPolicy::covalent_default();
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &cov,
        BtspCipherSuite::Null,
        Some(TrustModel::NuclearLineage),
    );
    assert!(decision.allowed, "covalent should allow nuclear peer");
}

#[test]
fn btsp_enforcer_denies_metallic_without_genetics() {
    use crate::btsp::BtspCipherSuite;
    let policy = BondingPolicy {
        bond_type: BondType::Metallic,
        trust_model: TrustModel::MitoBeaconFamily,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "metallic-test".to_owned(),
    };
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &policy,
        BtspCipherSuite::HmacPlain,
        Some(TrustModel::Contractual),
    );
    assert!(!decision.allowed, "metallic should deny non-genetic peer");
}

#[test]
fn btsp_enforcer_allows_metallic_with_mito() {
    use crate::btsp::BtspCipherSuite;
    let policy = BondingPolicy {
        bond_type: BondType::Metallic,
        trust_model: TrustModel::MitoBeaconFamily,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "metallic-test".to_owned(),
    };
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &policy,
        BtspCipherSuite::HmacPlain,
        Some(TrustModel::MitoBeaconFamily),
    );
    assert!(decision.allowed, "metallic should allow mito peer");
}

#[test]
fn btsp_enforcer_denies_ionic_with_zero_trust() {
    use crate::btsp::BtspCipherSuite;
    let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &ionic,
        BtspCipherSuite::ChaCha20Poly1305,
        Some(TrustModel::ZeroTrust),
    );
    assert!(!decision.allowed, "ionic should deny zero-trust peer");
}

#[test]
fn btsp_enforcer_allows_ionic_with_contractual() {
    use crate::btsp::BtspCipherSuite;
    let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &ionic,
        BtspCipherSuite::ChaCha20Poly1305,
        Some(TrustModel::Contractual),
    );
    assert!(decision.allowed, "ionic should allow contractual peer");
}

#[test]
fn btsp_enforcer_weak_allows_anything() {
    use crate::btsp::BtspCipherSuite;
    let weak = BondingPolicy {
        bond_type: BondType::Weak,
        trust_model: TrustModel::ZeroTrust,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "weak-test".to_owned(),
    };
    let decision = BtspEnforcer::evaluate_connection_with_trust(
        &weak,
        BtspCipherSuite::ChaCha20Poly1305,
        Some(TrustModel::ZeroTrust),
    );
    assert!(decision.allowed, "weak should allow zero-trust peer");
}

#[test]
fn btsp_enforcer_no_trust_backward_compat() {
    use crate::btsp::BtspCipherSuite;
    let cov = BondingPolicy::covalent_default();
    let decision = BtspEnforcer::evaluate_connection_with_trust(&cov, BtspCipherSuite::Null, None);
    assert!(
        decision.allowed,
        "None trust (legacy caller) should still allow"
    );
}

#[test]
fn btsp_enforcer_request_filtering() {
    use crate::btsp::BtspCipherSuite;
    let policy = BondingPolicy::idle_compute(vec![], 100);
    let allowed = BtspEnforcer::evaluate_request(
        &policy,
        "compute.submit",
        BtspCipherSuite::ChaCha20Poly1305,
    );
    assert!(allowed.capability_permitted);

    let denied =
        BtspEnforcer::evaluate_request(&policy, "storage.store", BtspCipherSuite::ChaCha20Poly1305);
    assert!(!denied.capability_permitted);
}

#[test]
fn bonding_result_round_trip_json() {
    let result = BondingResult {
        bond_type: BondType::Covalent,
        gates_discovered: 2,
        capabilities_shared: 8,
        trust_verified: true,
        degradation_graceful: true,
    };
    let json = serde_json::to_string(&result).unwrap();
    let back: BondingResult = serde_json::from_str(&json).unwrap();
    assert_eq!(back.bond_type, BondType::Covalent);
    assert_eq!(back.gates_discovered, 2);
}

#[test]
fn trust_model_round_trip_json() {
    for tm in [
        TrustModel::GeneticLineage,
        TrustModel::MitoBeaconFamily,
        TrustModel::NuclearLineage,
        TrustModel::Contractual,
        TrustModel::Organizational,
        TrustModel::ZeroTrust,
    ] {
        let json = serde_json::to_string(&tm).unwrap();
        let back: TrustModel = serde_json::from_str(&json).unwrap();
        assert_eq!(tm, back);
    }
}

#[test]
fn trust_model_is_genetic() {
    assert!(TrustModel::GeneticLineage.is_genetic());
    assert!(TrustModel::MitoBeaconFamily.is_genetic());
    assert!(TrustModel::NuclearLineage.is_genetic());
    assert!(!TrustModel::Contractual.is_genetic());
    assert!(!TrustModel::Organizational.is_genetic());
    assert!(!TrustModel::ZeroTrust.is_genetic());
}

#[test]
fn trust_model_is_nuclear() {
    assert!(TrustModel::GeneticLineage.is_nuclear());
    assert!(TrustModel::NuclearLineage.is_nuclear());
    assert!(!TrustModel::MitoBeaconFamily.is_nuclear());
    assert!(!TrustModel::Contractual.is_nuclear());
}

#[test]
fn trust_model_normalize() {
    assert_eq!(
        TrustModel::GeneticLineage.normalize(),
        TrustModel::NuclearLineage
    );
    assert_eq!(
        TrustModel::NuclearLineage.normalize(),
        TrustModel::NuclearLineage
    );
    assert_eq!(
        TrustModel::MitoBeaconFamily.normalize(),
        TrustModel::MitoBeaconFamily
    );
    assert_eq!(TrustModel::Contractual.normalize(), TrustModel::Contractual);
}
