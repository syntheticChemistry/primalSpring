// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp071: Idle Compute Policy — validates BondingPolicy capability masks,
//! time windows, bandwidth limits, and graph metadata for idle compute
//! federation scenarios.
//!
//! Validates the friend/family idle compute pattern: covalent trust,
//! compute-only sharing, time-windowed availability, bandwidth caps.

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::{BondType, BondingConstraint, BondingPolicy, TrustModel};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp071 — Idle Compute Policy");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp071: BondingPolicy — Capability Masks, Time Windows, Bandwidth");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // === Phase 1: BondingConstraint capability filtering ===

    let compute_only = BondingConstraint {
        capability_allow: vec!["compute.*".to_owned()],
        capability_deny: vec!["compute.admin".to_owned()],
        bandwidth_limit_mbps: 100,
        max_concurrent_requests: 4,
    };

    v.check_bool(
        "permits_compute_submit",
        compute_only.permits("compute.submit"),
        "compute.submit allowed by compute.* pattern",
    );
    v.check_bool(
        "permits_compute_status",
        compute_only.permits("compute.status"),
        "compute.status allowed by compute.* pattern",
    );
    v.check_bool(
        "denies_compute_admin",
        !compute_only.permits("compute.admin"),
        "compute.admin denied by explicit deny list",
    );
    v.check_bool(
        "denies_storage",
        !compute_only.permits("storage.store"),
        "storage.store not in allow list — denied",
    );
    v.check_bool(
        "denies_ai",
        !compute_only.permits("ai.query"),
        "ai.query not in allow list — denied",
    );
    v.check_bool(
        "bandwidth_cap",
        compute_only.bandwidth_limit_mbps == 100,
        "bandwidth limited to 100 Mbps",
    );
    v.check_bool(
        "concurrency_cap",
        compute_only.max_concurrent_requests == 4,
        "max concurrent requests limited to 4",
    );

    // Empty allow = permit everything (no restrictions)
    let unrestricted = BondingConstraint::default();
    v.check_bool(
        "empty_allow_permits_all",
        unrestricted.permits("anything.at.all"),
        "empty capability_allow permits everything",
    );

    // === Phase 2: BondingPolicy idle compute preset ===

    let idle = BondingPolicy::idle_compute(vec!["22:00-06:00".to_owned()], 100);
    let idle_errors = idle.validate();
    v.check_bool(
        "idle_policy_validates",
        idle_errors.is_empty(),
        &format!("idle compute policy validates (errors: {idle_errors:?})"),
    );
    v.check_bool(
        "idle_policy_covalent",
        idle.bond_type == BondType::Covalent,
        "idle compute policy uses Covalent bond type",
    );
    v.check_bool(
        "idle_policy_genetic_trust",
        idle.trust_model == TrustModel::GeneticLineage,
        "idle compute policy uses GeneticLineage trust",
    );
    v.check_bool(
        "idle_policy_no_relay",
        !idle.offer_relay,
        "idle compute peers don't offer relay by default",
    );
    v.check_bool(
        "idle_policy_compute_only",
        idle.constraints.permits("compute.submit")
            && !idle.constraints.permits("storage.store")
            && !idle.constraints.permits("ai.query"),
        "idle policy: compute allowed, storage and AI denied",
    );
    v.check_bool(
        "idle_policy_time_window",
        idle.active_windows == vec!["22:00-06:00"],
        "idle policy active 22:00-06:00",
    );
    v.check_bool(
        "idle_policy_bandwidth",
        idle.constraints.bandwidth_limit_mbps == 100,
        "idle policy bandwidth capped at 100 Mbps",
    );

    // === Phase 3: BondingPolicy covalent default ===

    let full = BondingPolicy::covalent_default();
    let full_errors = full.validate();
    v.check_bool(
        "covalent_default_validates",
        full_errors.is_empty(),
        "covalent default policy validates cleanly",
    );
    v.check_bool(
        "covalent_default_offers_relay",
        full.offer_relay,
        "covalent default offers relay to family",
    );
    v.check_bool(
        "covalent_default_no_time_window",
        full.active_windows.is_empty(),
        "covalent default has no time restrictions",
    );
    v.check_bool(
        "covalent_default_permits_all",
        full.constraints.permits("compute.submit")
            && full.constraints.permits("storage.store")
            && full.constraints.permits("ai.query"),
        "covalent default permits all capabilities",
    );

    // === Phase 4: BondingPolicy ionic contract ===

    let ionic = BondingPolicy::ionic_contract(vec!["compute.submit".to_owned(), "compute.status".to_owned()]);
    let ionic_errors = ionic.validate();
    v.check_bool(
        "ionic_contract_validates",
        ionic_errors.is_empty(),
        &format!("ionic contract policy validates (errors: {ionic_errors:?})"),
    );
    v.check_bool(
        "ionic_contract_type",
        ionic.bond_type == BondType::Ionic,
        "ionic contract uses Ionic bond type",
    );
    v.check_bool(
        "ionic_contract_trust",
        ionic.trust_model == TrustModel::Contractual,
        "ionic contract uses Contractual trust model",
    );

    // === Phase 5: Policy validation catches inconsistencies ===

    let bad_covalent = BondingPolicy {
        bond_type: BondType::Covalent,
        trust_model: TrustModel::Contractual,
        constraints: BondingConstraint::default(),
        active_windows: Vec::new(),
        offer_relay: false,
        label: "bad".to_owned(),
    };
    let bad_errors = bad_covalent.validate();
    v.check_bool(
        "catches_covalent_trust_mismatch",
        !bad_errors.is_empty() && bad_errors.iter().any(|e| e.contains("GeneticLineage")),
        "validation catches covalent + contractual trust mismatch",
    );

    let bad_weak = BondingPolicy {
        bond_type: BondType::Weak,
        trust_model: TrustModel::ZeroTrust,
        constraints: BondingConstraint {
            capability_allow: vec!["compute.*".to_owned()],
            ..BondingConstraint::default()
        },
        active_windows: Vec::new(),
        offer_relay: false,
        label: "bad-weak".to_owned(),
    };
    let weak_errors = bad_weak.validate();
    v.check_bool(
        "catches_weak_with_allow",
        !weak_errors.is_empty() && weak_errors.iter().any(|e| e.contains("capability_allow")),
        "validation catches weak bond with capability_allow list",
    );

    // === Phase 6: Graph metadata for idle compute federation ===

    let graph_path = Path::new("graphs/multi_node/idle_compute_federation.toml");
    v.check_or_skip(
        "idle_graph_exists",
        graph_path.exists().then_some(&()),
        "idle_compute_federation.toml not found",
        |_, v| {
            let meta = validate_graph_bonding(graph_path);
            v.check_bool(
                "idle_graph_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                "graph declares covalent bonding",
            );
            v.check_bool(
                "idle_graph_has_policy",
                meta.policy.is_some(),
                "graph declares a bonding policy",
            );
            if let Some(policy) = &meta.policy {
                v.check_bool(
                    "idle_graph_policy_compute_only",
                    policy.constraints.permits("compute.submit")
                        && !policy.constraints.permits("storage.store"),
                    "graph policy: compute allowed, storage denied",
                );
            }
            v.check_bool(
                "idle_graph_clean",
                meta.issues.is_empty(),
                &format!("graph validation issues: {:?}", meta.issues),
            );
        },
    );

    let friend_path = Path::new("graphs/multi_node/friend_remote_covalent.toml");
    v.check_or_skip(
        "friend_graph_exists",
        friend_path.exists().then_some(&()),
        "friend_remote_covalent.toml not found",
        |_, v| {
            let meta = validate_graph_bonding(friend_path);
            v.check_bool(
                "friend_graph_has_time_windows",
                meta.policy
                    .as_ref()
                    .is_some_and(|p| !p.active_windows.is_empty()),
                "friend graph policy has active time windows",
            );
            v.check_bool(
                "friend_graph_bandwidth_limited",
                meta.policy
                    .as_ref()
                    .is_some_and(|p| p.constraints.bandwidth_limit_mbps > 0),
                "friend graph policy has bandwidth limit",
            );
        },
    );

    v.finish();
    std::process::exit(v.exit_code());
}
