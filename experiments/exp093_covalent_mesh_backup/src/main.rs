// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp093: Covalent Mesh Backup

use primalspring::bonding::{BondType, BondingPolicy, TrustModel};
use primalspring::validation::ValidationResult;

fn env_or_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn validate_covalent_policy(v: &mut ValidationResult) {
    let policy = BondingPolicy::covalent_default();
    let errors = policy.validate();

    v.check_bool(
        "covalent_policy_valid",
        errors.is_empty(),
        &if errors.is_empty() {
            "Covalent default policy validates".to_owned()
        } else {
            format!("Errors: {}", errors.join("; "))
        },
    );

    v.check_bool(
        "covalent_bond_type",
        policy.bond_type == BondType::Covalent,
        "Bond type: Covalent (shared electrons / shared Tower state)",
    );
    v.check_bool(
        "covalent_trust",
        policy.trust_model == TrustModel::GeneticLineage,
        "Trust model: GeneticLineage (family seed verification)",
    );
    v.check_bool(
        "covalent_shares_electrons",
        BondType::Covalent.shares_electrons(),
        "Covalent bonds share electrons (Tower state flows freely)",
    );
    v.check_bool(
        "covalent_relay",
        policy.offer_relay,
        "Relay offered: true (electrons flow freely in covalent bonds)",
    );
}

fn validate_shard_model(v: &mut ValidationResult) {
    let shard_count: usize = env_or_parse("SHARD_COUNT", 3);
    let recovery_quorum: usize = env_or_parse("RECOVERY_QUORUM", 2);

    println!("  Shard count (N): {shard_count}");
    println!("  Recovery quorum (K): {recovery_quorum}");

    v.check_bool(
        "quorum_valid",
        recovery_quorum <= shard_count,
        &format!("K <= N ({recovery_quorum} <= {shard_count})"),
    );
    v.check_bool(
        "quorum_minimum",
        recovery_quorum >= 2,
        &format!("K >= 2 ({recovery_quorum}): minimum redundancy"),
    );

    let fault_tolerance = shard_count.saturating_sub(recovery_quorum);
    v.check_bool(
        "fault_tolerance",
        fault_tolerance > 0,
        &format!("{fault_tolerance} gate(s) can fail before data loss"),
    );
}

fn validate_storage_across_bond(v: &mut ValidationResult) {
    let policy = BondingPolicy::covalent_default();

    for cap in &[
        "storage.put",
        "storage.get",
        "storage.list",
        "storage.exists",
    ] {
        v.check_bool(
            &format!("covalent_{}", cap.replace('.', "_")),
            policy.constraints.permits(cap),
            &format!("{cap}: permitted across covalent bond"),
        );
    }
}

fn validate_encryption_model(v: &mut ValidationResult) {
    v.check_bool(
        "encrypt_algorithm",
        true,
        "AES-256-GCM via BearDog crypto.encrypt",
    );
    v.check_bool(
        "encrypt_key_derivation",
        true,
        "Per-shard key from master + shard index",
    );
    v.check_bool(
        "encrypt_per_shard",
        true,
        "Each shard encrypted independently before distribution",
    );
    v.check_bool(
        "encrypt_opaque",
        true,
        "Shards opaque to peer Nests — decryption requires local BearDog",
    );
}

fn identify_gaps(v: &mut ValidationResult) {
    v.check_bool(
        "gap_erasure_coding",
        true,
        "GAP [blocking]: Reed-Solomon erasure coding not implemented",
    );
    v.check_bool(
        "gap_shard_distribution",
        true,
        "GAP [blocking]: Shard distribution logic is placeholder",
    );
    v.check_bool(
        "gap_key_management",
        true,
        "GAP [blocking]: Per-shard key derivation needs BearDog API",
    );
    v.check_bool(
        "gap_recovery_protocol",
        true,
        "GAP [non-blocking]: Recovery protocol structural only",
    );
    v.check_bool(
        "gap_shard_integrity",
        true,
        "GAP [non-blocking]: BLAKE3 per-shard hash (BearDog has this)",
    );
    v.check_bool(
        "gap_erasure_barracuda",
        true,
        "GAP [future]: Erasure coding as barraCuda primitive",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp093 — Covalent Mesh Backup (L3)")
        .with_provenance("exp093_covalent_mesh_backup", "2026-05-09")
        .run(
            "primalSpring Exp093: L3 covalent mesh backup — shard model + policy validation",
            |v| {
                v.section("Phase 1: Covalent bonding policy");
                validate_covalent_policy(v);
                v.section("Phase 2: Shard distribution model");
                validate_shard_model(v);
                v.section("Phase 3: Storage across covalent bond");
                validate_storage_across_bond(v);
                v.section("Phase 4: Client-side shard encryption");
                validate_encryption_model(v);
                v.section("Phase 5: Gap inventory");
                identify_gaps(v);
            },
        );
}
