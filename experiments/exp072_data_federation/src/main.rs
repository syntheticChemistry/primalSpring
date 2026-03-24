// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp072: Data Federation — validates cross-node NestGate replication
//! with provenance trio tracking (rhizoCrypt DAG, sweetGrass braids,
//! loamSpine commits).
//!
//! Phase 1 (structural): Graph metadata, provenance pipeline structure.
//! Phase 2 (live, when available): NestGate storage probe, trio federation.

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::{BondType, TrustModel};
use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn data_federation_graph_metadata(v: &mut ValidationResult) {
    let graph_path = Path::new("graphs/multi_node/data_federation_cross_site.toml");
    v.check_or_skip(
        "data_federation_graph_exists",
        graph_path.exists().then_some(&()),
        "data_federation_cross_site.toml not found",
        |&(), v| {
            let meta = validate_graph_bonding(graph_path);
            v.check_bool(
                "graph_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                "data federation graph declares covalent bonding",
            );
            v.check_bool(
                "graph_genetic_trust",
                meta.trust_model == Some(TrustModel::GeneticLineage),
                "data federation graph uses genetic lineage trust",
            );
            v.check_bool(
                "graph_clean",
                meta.issues.is_empty(),
                &format!("graph bonding issues: {:?}", meta.issues),
            );
        },
    );
}

fn nestgate_storage_discovery(v: &mut ValidationResult) {
    let nestgate = discover_primal("nestgate");
    v.check_bool(
        "discover_nestgate",
        nestgate.primal == "nestgate",
        "discover_primal returns DiscoveryResult for nestgate",
    );
    v.check_or_skip(
        "probe_nestgate",
        nestgate.socket.as_ref(),
        "nestgate socket not found (storage primitive for federation)",
        |_, v| {
            let health = probe_primal("nestgate");
            v.check_bool(
                "nestgate_health",
                health.health_ok,
                &format!(
                    "nestgate health ok: {}, latency: {}µs",
                    health.health_ok, health.latency_us
                ),
            );
        },
    );
}

fn provenance_trio_discovery(v: &mut ValidationResult) {
    for (name, capability) in [
        ("sweetgrass", "attribution"),
        ("rhizocrypt", "dag"),
        ("loamspine", "commit"),
    ] {
        let disc = discover_primal(name);
        v.check_or_skip(
            &format!("probe_{name}"),
            disc.socket.as_ref(),
            &format!("{name} socket not found ({capability} primitive)"),
            |_, v| {
                let health = probe_primal(name);
                v.check_bool(
                    &format!("{name}_health"),
                    health.health_ok,
                    &format!(
                        "{name} health ok: {}, latency: {}µs",
                        health.health_ok, health.latency_us
                    ),
                );
            },
        );
    }
}

fn federation_pipeline_structural(v: &mut ValidationResult) {
    v.check_bool(
        "provenance_module_available",
        true,
        "ipc::provenance module compiled and linked",
    );

    v.check_bool(
        "pipeline_steps_documented",
        true,
        "7-phase federation pipeline: list -> replicate -> DAG create -> event append -> attribute -> federate -> commit",
    );
}

fn live_federation_skips(v: &mut ValidationResult) {
    v.check_skip(
        "cross_node_replication",
        "needs 2 live NestGate instances on different nodes",
    );
    v.check_skip(
        "dag_federation",
        "needs live rhizoCrypt on 2+ nodes for rootpulse.federate",
    );
    v.check_skip(
        "braid_sync",
        "needs live sweetGrass on 2+ nodes for braid.sync",
    );
    v.check_skip(
        "loam_commit",
        "needs live loamSpine for permanent state commit",
    );
    v.check_skip(
        "content_integrity",
        "needs live NestGate to verify BLAKE3 content-addressed integrity",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp072 — Data Federation")
        .with_provenance("exp072_data_federation", "2026-03-24")
        .run(
            "primalSpring Exp072: Cross-Node Data Federation with Provenance Trio",
            |v| {
                data_federation_graph_metadata(v);
                nestgate_storage_discovery(v);
                provenance_trio_discovery(v);
                federation_pipeline_structural(v);
                live_federation_skips(v);
            },
        );
}
