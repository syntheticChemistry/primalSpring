// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp056: Cross-Tower Federation — BYOB manifest, graph bonding metadata, live federation (skipped).

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::{BondType, TrustModel};
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn atomic_type_structural(v: &mut ValidationResult) {
    let variants = [
        AtomicType::Tower,
        AtomicType::Node,
        AtomicType::Nest,
        AtomicType::FullNucleus,
    ];
    let all_have_required = variants
        .iter()
        .all(|t| !t.required_capabilities().is_empty());
    v.check_bool(
        "all_atomic_types_have_required_capabilities",
        all_have_required,
        "all AtomicType variants have required_capabilities",
    );

    let full_caps: std::collections::HashSet<&str> = AtomicType::FullNucleus
        .required_capabilities()
        .iter()
        .copied()
        .collect();
    let tower_caps = AtomicType::Tower.required_capabilities();
    let full_is_superset = tower_caps.iter().all(|c| full_caps.contains(c));
    v.check_bool(
        "full_nucleus_superset_of_tower",
        full_is_superset,
        "FullNucleus capabilities is superset of Tower capabilities",
    );
}

fn friend_remote_covalent_graph_metadata(v: &mut ValidationResult) {
    let friend_graph = Path::new("graphs/multi_node/friend_remote_covalent.toml");
    v.check_or_skip(
        "friend_graph_metadata",
        friend_graph.exists().then_some(&()),
        "friend_remote_covalent.toml not found",
        |&(), v| {
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
}

fn idle_compute_federation_graph_metadata(v: &mut ValidationResult) {
    let idle_graph = Path::new("graphs/multi_node/idle_compute_federation.toml");
    v.check_or_skip(
        "idle_federation_graph_metadata",
        idle_graph.exists().then_some(&()),
        "idle_compute_federation.toml not found",
        |&(), v| {
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
}

fn data_federation_graph_metadata(v: &mut ValidationResult) {
    let data_graph = Path::new("graphs/multi_node/data_federation_cross_site.toml");
    v.check_or_skip(
        "data_federation_graph_metadata",
        data_graph.exists().then_some(&()),
        "data_federation_cross_site.toml not found",
        |&(), v| {
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
}

fn live_federation_skips(v: &mut ValidationResult) {
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
}

fn main() {
    ValidationResult::new("primalSpring Exp056 — Cross Tower Federation")
        .with_provenance("exp056_cross_tower_federation", "2026-05-09")
        .run(
            "primalSpring Exp056: Cross-Tower Federation — BYOB, NAT, Gossip",
            |v| {
                v.section("Phase 1: Atomic Type Structure");
                atomic_type_structural(v);

                v.section("Phase 2: Federation Graphs");
                friend_remote_covalent_graph_metadata(v);
                idle_compute_federation_graph_metadata(v);
                data_federation_graph_metadata(v);

                v.section("Phase 3: Live Federation");
                live_federation_skips(v);
            },
        );
}
