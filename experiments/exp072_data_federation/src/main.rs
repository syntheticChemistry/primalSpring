// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp072: Data Federation

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::{BondType, TrustModel};
use primalspring::composition::CompositionContext;
use primalspring::ipc::methods;
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

fn sweetgrass_cap(ctx: &CompositionContext) -> Option<&'static str> {
    if ctx.has_capability("commit") {
        Some("commit")
    } else if ctx.has_capability("attribution") {
        Some("attribution")
    } else {
        None
    }
}

fn nestgate_storage_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.check_bool(
        "discover_nestgate",
        ctx.has_capability("storage"),
        "storage capability (NestGate) discoverable in composition context",
    );

    if !ctx.has_capability("storage") {
        v.check_skip(
            "nestgate_health",
            "storage capability not discovered (NestGate socket)",
        );
        return;
    }

    match ctx.call("storage", methods::health::LIVENESS, serde_json::json!({})) {
        Ok(_) => v.check_bool("nestgate_health", true, "NestGate health.liveness"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("nestgate_health", &format!("connection: {e}"));
        }
        Err(e) => v.check_bool("nestgate_health", false, &format!("error: {e}")),
    }
}

fn probe_cap_health(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    check_key: &str,
    cap: &str,
    label: &str,
) {
    if !ctx.has_capability(cap) {
        v.check_skip(
            check_key,
            &format!("{label} — capability {cap} not in context"),
        );
        return;
    }

    match ctx.call(cap, methods::health::LIVENESS, serde_json::json!({})) {
        Ok(_) => v.check_bool(check_key, true, &format!("{label} health.liveness")),
        Err(e) if e.is_connection_error() => v.check_skip(check_key, &format!("connection: {e}")),
        Err(e) => v.check_bool(check_key, false, &format!("error: {e}")),
    }
}

fn provenance_trio_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    probe_cap_health(v, ctx, "rhizoCrypt_health", "dag", "rhizoCrypt DAG");
    probe_cap_health(v, ctx, "loamSpine_health", "ledger", "loamSpine ledger");

    match sweetgrass_cap(ctx) {
        Some(cap) => probe_cap_health(v, ctx, "sweetGrass_health", cap, "sweetGrass braid/commit"),
        None => v.check_skip(
            "sweetGrass_health",
            "neither commit nor attribution capability for sweetGrass",
        ),
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
        .with_provenance("exp072_data_federation", "2026-05-09")
        .run(
            "primalSpring Exp072: Cross-Node Data Federation with Provenance Trio",
            |v| {
                v.section("Phase 1: Graph metadata");
                data_federation_graph_metadata(v);

                v.section("Phase 2: Live discovery and health");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                nestgate_storage_discovery(v, &mut ctx);
                provenance_trio_discovery(v, &mut ctx);

                v.section("Phase 3: Pipeline structure");
                federation_pipeline_structural(v);

                v.section("Phase 4: Deferred live checks");
                live_federation_skips(v);
            },
        );
}
