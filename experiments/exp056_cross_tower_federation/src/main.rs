// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp056: Cross-Tower Federation — validates BYOB manifest, NAT traversal
//! graph structure, gossip federation metadata, and STUN tier escalation.
//!
//! Phase 1 (structural): AtomicType superset, friend graph bonding metadata.
//! Phase 2 (live, when available): gossip federation flow, STUN tier probing.

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::{BondType, TrustModel};
use primalspring::coordination::AtomicType;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp056 — Cross Tower Federation");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp056: Cross-Tower Federation — BYOB, NAT, Gossip");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // --- Phase 1: AtomicType structural ---
    let variants = [
        AtomicType::Tower,
        AtomicType::Node,
        AtomicType::Nest,
        AtomicType::FullNucleus,
    ];
    let all_have_required = variants.iter().all(|t| !t.required_primals().is_empty());
    v.check_bool(
        "all_atomic_types_have_required_primals",
        all_have_required,
        "all AtomicType variants have required_primals",
    );

    let full_primals: std::collections::HashSet<&str> = AtomicType::FullNucleus
        .required_primals()
        .iter()
        .copied()
        .collect();
    let tower_primals = AtomicType::Tower.required_primals();
    let full_is_superset = tower_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "full_nucleus_superset_of_tower",
        full_is_superset,
        "FullNucleus primals is superset of Tower primals",
    );

    // --- Phase 2: Friend remote covalent graph metadata ---
    let friend_graph = Path::new("graphs/multi_node/friend_remote_covalent.toml");
    v.check_or_skip(
        "friend_graph_metadata",
        friend_graph.exists().then_some(&()),
        "friend_remote_covalent.toml not found",
        |_, v| {
            let meta = validate_graph_bonding(friend_graph);
            v.check_bool(
                "friend_graph_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                "friend graph declares covalent bonding",
            );
            v.check_bool(
                "friend_graph_genetic_trust",
                meta.trust_model == Some(TrustModel::GeneticLineage),
                "friend graph uses genetic lineage trust",
            );
            v.check_bool(
                "friend_graph_has_policy",
                meta.policy.is_some(),
                "friend graph declares a bonding policy",
            );
            if let Some(policy) = &meta.policy {
                v.check_bool(
                    "friend_policy_compute_only",
                    policy.constraints.permits("compute.submit")
                        && !policy.constraints.permits("storage.store"),
                    "friend policy allows compute.*, denies storage.*",
                );
                v.check_bool(
                    "friend_policy_has_time_windows",
                    !policy.active_windows.is_empty(),
                    &format!(
                        "friend policy has active_windows: {:?}",
                        policy.active_windows
                    ),
                );
            }
            v.check_bool(
                "friend_graph_clean",
                meta.issues.is_empty(),
                &format!("graph validation issues: {:?}", meta.issues),
            );
        },
    );

    // --- Phase 3: Idle compute federation graph metadata ---
    let idle_graph = Path::new("graphs/multi_node/idle_compute_federation.toml");
    v.check_or_skip(
        "idle_federation_graph_metadata",
        idle_graph.exists().then_some(&()),
        "idle_compute_federation.toml not found",
        |_, v| {
            let meta = validate_graph_bonding(idle_graph);
            v.check_bool(
                "idle_graph_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                "idle federation graph declares covalent bonding",
            );
            v.check_bool(
                "idle_graph_clean",
                meta.issues.is_empty(),
                &format!("graph validation issues: {:?}", meta.issues),
            );
        },
    );

    // --- Phase 4: Data federation graph metadata ---
    let data_graph = Path::new("graphs/multi_node/data_federation_cross_site.toml");
    v.check_or_skip(
        "data_federation_graph_metadata",
        data_graph.exists().then_some(&()),
        "data_federation_cross_site.toml not found",
        |_, v| {
            let meta = validate_graph_bonding(data_graph);
            v.check_bool(
                "data_graph_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                "data federation graph declares covalent bonding",
            );
            v.check_bool(
                "data_graph_clean",
                meta.issues.is_empty(),
                &format!("graph validation issues: {:?}", meta.issues),
            );
        },
    );

    // --- Phase 5: Live federation (needs 2+ towers) ---
    v.check_skip(
        "cross_tower_discovery",
        "cross-tower discovery needs live primals on 2+ machines",
    );
    v.check_skip(
        "stun_tier_escalation",
        "STUN tier escalation needs live Songbird with NAT",
    );
    v.check_skip(
        "gossip_beacon_exchange",
        "gossip federation needs live BirdSong beacon exchange",
    );
    v.check_skip(
        "timeout_handling",
        "timeout handling needs live federation across NAT",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
