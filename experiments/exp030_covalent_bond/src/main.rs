// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp030: Covalent Bond — validates two NUCLEUS instances share family seed
//! and discover each other via BirdSong mesh.
//!
//! Phase 1 (structural): BondType properties, family-scoped sockets, graph metadata.
//! Phase 2 (live, when available): mesh.auto_discover finds second gate.

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::{BondType, BondingPolicy, TrustModel};
use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp030 — Covalent Bond");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp030: Covalent Bond — Family Seed, Mesh Discovery, Graph Metadata");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // --- Phase 1: BondType structural ---
    let bond = BondType::Covalent;
    v.check_bool(
        "covalent_description_non_empty",
        !bond.description().is_empty(),
        &format!("BondType::Covalent — {}", bond.description()),
    );
    v.check_bool(
        "covalent_shares_electrons",
        bond.shares_electrons(),
        "Covalent bonds share electrons (Tower state)",
    );
    v.check_bool(
        "covalent_not_metered",
        !bond.is_metered(),
        "Covalent bonds are not metered (cooperative)",
    );

    // --- Phase 2: Family-scoped socket paths ---
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let path_beardog = socket_path("beardog");
    let path_contains_family = path_beardog
        .to_string_lossy()
        .contains(&format!("-{family_id}.sock"));
    v.check_bool(
        "socket_path_includes_family_id",
        path_contains_family,
        &format!(
            "socket path includes FAMILY_ID: {} (path: {})",
            family_id,
            path_beardog.display()
        ),
    );

    // --- Phase 3: BondingPolicy for covalent default ---
    let policy = BondingPolicy::covalent_default();
    let policy_errors = policy.validate();
    v.check_bool(
        "covalent_default_policy_valid",
        policy_errors.is_empty(),
        &format!(
            "BondingPolicy::covalent_default() validates cleanly (errors: {})",
            policy_errors.len()
        ),
    );
    v.check_bool(
        "covalent_policy_offers_relay",
        policy.offer_relay,
        "Covalent default policy offers relay to family",
    );
    v.check_bool(
        "covalent_policy_genetic_trust",
        policy.trust_model == TrustModel::GeneticLineage,
        "Covalent policy uses GeneticLineage trust",
    );

    // --- Phase 4: Graph metadata validation ---
    let graph_path = Path::new("graphs/multi_node/basement_hpc_covalent.toml");
    v.check_or_skip(
        "covalent_graph_metadata",
        graph_path.exists().then_some(&()),
        "basement_hpc_covalent.toml not found",
        |_, v| {
            let meta = validate_graph_bonding(graph_path);
            v.check_bool(
                "graph_bond_type_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                &format!(
                    "graph internal_bond_type = {:?}",
                    meta.internal_bond_type
                ),
            );
            v.check_bool(
                "graph_trust_is_genetic",
                meta.trust_model == Some(TrustModel::GeneticLineage),
                &format!("graph trust_model = {:?}", meta.trust_model),
            );
            v.check_bool(
                "graph_no_bonding_issues",
                meta.issues.is_empty(),
                &format!("graph bonding issues: {:?}", meta.issues),
            );
        },
    );

    // --- Phase 5: Tower discovery + live probing ---
    let beardog = discover_primal("beardog");
    v.check_bool(
        "discover_beardog_returns_result",
        beardog.primal == "beardog",
        "discover_primal returns DiscoveryResult for beardog",
    );
    let songbird = discover_primal("songbird");
    for (name, discovery) in [("beardog", beardog), ("songbird", songbird)] {
        v.check_or_skip(
            &format!("probe_{name}"),
            discovery.socket.as_ref(),
            &format!("{name} socket not found (Tower primitive)"),
            |_, v| {
                let health = probe_primal(name);
                v.check_bool(
                    &format!("{name}_health"),
                    health.health_ok,
                    &format!(
                        "health ok: {}, latency: {}µs",
                        health.health_ok, health.latency_us
                    ),
                );
            },
        );
    }

    // --- Phase 6: Multi-node (needs 2 live NUCLEUS) ---
    v.check_skip("family_seed_sharing", "needs 2 live NUCLEUS instances");
    v.check_skip(
        "mesh_auto_discover_second_gate",
        "needs live Songbird mesh with 2+ gates",
    );
    v.check_skip(
        "cross_gate_capability_call",
        "needs live Plasmodium routing between gates",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
