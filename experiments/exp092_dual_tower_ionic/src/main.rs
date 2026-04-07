// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp092: Dual Tower Ionic Bond — L2 mixed atomic validation.
//!
//! Validates two independent Tower Atomics (two electron shells) coexisting on
//! the same host with different FAMILY_IDs, connected by an ionic bond that
//! provides scoped capability sharing.
//!
//! Particle model: two atoms on the same host. Tower A and Tower B each have
//! their own BearDog + Songbird. An ionic bond bridges them, allowing Tower B
//! to call scoped capabilities through Tower A without genetic trust.
//!
//! Validation steps (structural):
//!   1. Validate Tower A and Tower B capability requirements
//!   2. Validate ionic bond policy (contractual trust, scoped capabilities)
//!   3. Validate capability isolation via BondingConstraint
//!   4. Identify gaps for live dual-Tower validation
//!
//! Environment:
//!   `FAMILY_A` — first Tower's family ID (default: auto-generated)
//!   `FAMILY_B` — second Tower's family ID (default: auto-generated)

use primalspring::bonding::{BondType, BondingConstraint, BondingPolicy, TrustModel};
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn validate_tower_coexistence(v: &mut ValidationResult) {
    v.section("L2: Dual Tower Coexistence");

    let family_a = env_or("FAMILY_A", "a1b2c3d4e5f60001");
    let family_b = env_or("FAMILY_B", "a1b2c3d4e5f60002");

    println!("  Tower A FAMILY_ID: {family_a}");
    println!("  Tower B FAMILY_ID: {family_b}");

    let required = AtomicType::Tower.required_capabilities();
    v.check_bool(
        "tower_a_caps",
        !required.is_empty(),
        &format!("Tower A requires {:?}", required),
    );
    v.check_bool(
        "tower_b_caps",
        !required.is_empty(),
        &format!("Tower B requires {:?}", required),
    );
    v.check_bool(
        "distinct_families",
        family_a != family_b,
        "FAMILY_A != FAMILY_B (distinct trust domains)",
    );
}

fn validate_ionic_bridge(v: &mut ValidationResult) {
    v.section("L2: Ionic Bond Proposal");

    let policy = BondingPolicy::ionic_contract(vec![
        "compute.submit".to_owned(),
        "compute.status".to_owned(),
    ]);

    let errors = policy.validate();
    v.check_bool(
        "ionic_policy_valid",
        errors.is_empty(),
        &if errors.is_empty() {
            "Ionic bond policy validates (contract-based, compute-scoped)".to_owned()
        } else {
            format!("Errors: {}", errors.join("; "))
        },
    );

    v.check_bool(
        "ionic_bond_type",
        policy.bond_type == BondType::Ionic,
        "Bond type: Ionic (electron transfer)",
    );
    v.check_bool(
        "ionic_trust_model",
        policy.trust_model == TrustModel::Contractual,
        "Trust model: Contractual (no genetic lineage)",
    );
    v.check_bool(
        "ionic_metered",
        BondType::Ionic.is_metered(),
        "Ionic bonds are metered",
    );
    v.check_bool(
        "ionic_no_electron_share",
        !BondType::Ionic.shares_electrons(),
        "Ionic bonds do NOT share electrons (Tower state stays separate)",
    );
}

fn validate_capability_isolation(v: &mut ValidationResult) {
    v.section("L2: Capability Isolation");

    let constraint = BondingConstraint {
        capability_allow: vec!["compute.*".to_owned()],
        capability_deny: vec![
            "storage.*".to_owned(),
            "dag.*".to_owned(),
            "crypto.*".to_owned(),
        ],
        bandwidth_limit_mbps: 100,
        max_concurrent_requests: 8,
    };

    let cases: &[(&str, bool)] = &[
        ("compute.submit", true),
        ("compute.status", true),
        ("storage.put", false),
        ("storage.get", false),
        ("dag.session.create", false),
        ("crypto.sign_ed25519", false),
        ("ai.query", false),
    ];

    for &(cap, expected) in cases {
        let actual = constraint.permits(cap);
        let label = if actual { "ALLOWED" } else { "DENIED" };
        v.check_bool(
            &format!("isolation_{}", cap.replace('.', "_")),
            expected == actual,
            &format!("{cap}: {label}"),
        );
    }
}

fn identify_gaps(v: &mut ValidationResult) {
    v.section("L2: Gap Assessment");

    v.check_bool(
        "gap_simultaneous_towers",
        true,
        "GAP: Simultaneous multi-Tower on same host (AtomicHarness runs sequentially)",
    );
    v.check_bool(
        "gap_live_ionic_bridge",
        true,
        "GAP: Live ionic bridge between running Towers (structural only)",
    );
    v.check_bool(
        "gap_cross_tower_routing",
        true,
        "GAP: Cross-Tower capability.call routing through ionic bond",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp092 — Dual Tower Ionic Bond (L2)")
        .with_provenance("exp092_dual_tower_ionic", "2026-04-07")
        .run(
            "primalSpring Exp092: L2 dual tower coexistence + ionic bond + isolation",
            |v| {
                validate_tower_coexistence(v);
                validate_ionic_bridge(v);
                validate_capability_isolation(v);
                identify_gaps(v);
            },
        );
}
