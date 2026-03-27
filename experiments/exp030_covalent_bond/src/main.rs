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
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn bond_type_structural(v: &mut ValidationResult) {
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
}

fn family_scoped_socket_paths(v: &mut ValidationResult) {
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let path_beardog = socket_path(primal_names::BEARDOG);
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
}

fn covalent_bonding_policy(v: &mut ValidationResult) {
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
}

fn covalent_graph_metadata(v: &mut ValidationResult) {
    let graph_path = Path::new("graphs/multi_node/basement_hpc_covalent.toml");
    v.check_or_skip(
        "covalent_graph_metadata",
        graph_path.exists().then_some(&()),
        "basement_hpc_covalent.toml not found",
        |&(), v| {
            let meta = validate_graph_bonding(graph_path);
            v.check_bool(
                "graph_bond_type_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                &format!("graph internal_bond_type = {:?}", meta.internal_bond_type),
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
}

fn tower_discovery_probing(v: &mut ValidationResult) {
    let beardog = discover_primal(primal_names::BEARDOG);
    v.check_bool(
        "discover_beardog_returns_result",
        beardog.primal == primal_names::BEARDOG,
        "discover_primal returns DiscoveryResult for beardog",
    );
    let songbird = discover_primal(primal_names::SONGBIRD);
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
}

fn multi_node_skips(v: &mut ValidationResult) {
    v.check_skip("family_seed_sharing", "needs 2 live NUCLEUS instances");
    v.check_skip(
        "mesh_auto_discover_second_gate",
        "needs live Songbird mesh with 2+ gates",
    );
    v.check_skip(
        "cross_gate_capability_call",
        "needs live Plasmodium routing between gates",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp030 — Covalent Bond")
        .with_provenance("exp030_covalent_bond", "2026-03-24")
        .run(
            "primalSpring Exp030: Covalent Bond — Family Seed, Mesh Discovery, Graph Metadata",
            |v| {
                bond_type_structural(v);
                family_scoped_socket_paths(v);
                covalent_bonding_policy(v);
                covalent_graph_metadata(v);
                tower_discovery_probing(v);
                multi_node_skips(v);
            },
        );
}
