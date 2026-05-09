// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp032: Plasmodium Formation — collective formation from covalent mesh (structural + songbird-cap health).

use std::path::Path;

use primalspring::bonding::BondType;
use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn phase_bond_types(v: &mut ValidationResult) {
    let all_variants = BondType::all();
    v.check_bool(
        "bond_type_count",
        all_variants.len() == 5,
        &format!(
            "BondType::all() has 5 variants (got {})",
            all_variants.len()
        ),
    );
    v.check_bool(
        "all_bond_types_have_descriptions",
        all_variants.iter().all(|bt| !bt.description().is_empty()),
        "all BondType variants have non-empty descriptions",
    );

    let metallic = BondType::Metallic;
    v.check_bool(
        "metallic_shares_electrons",
        metallic.shares_electrons(),
        "Metallic bonds share electrons (delocalized Tower pool)",
    );
    v.check_bool(
        "metallic_not_metered",
        !metallic.is_metered(),
        "Metallic bonds are internal allocation, not billed",
    );
}

fn phase_graph_metadata(v: &mut ValidationResult) {
    let graph_path = Path::new("graphs/multi_node/basement_hpc_covalent.toml");
    v.check_or_skip(
        "hpc_graph_metadata",
        graph_path.exists().then_some(&()),
        "basement_hpc_covalent.toml not found",
        |&(), v| {
            let meta = validate_graph_bonding(graph_path);
            v.check_bool(
                "hpc_graph_is_covalent",
                meta.internal_bond_type == Some(BondType::Covalent),
                "basement HPC graph declares covalent bonding",
            );
            v.check_bool(
                "hpc_graph_clean",
                meta.issues.is_empty(),
                &format!("graph validation issues: {:?}", meta.issues),
            );
        },
    );
}

fn phase_live_discovery(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    const CAP: &str = "discovery";
    if !ctx.has_capability(CAP) {
        v.check_skip(
            "health_liveness_discovery",
            "discovery capability not in composition context",
        );
        return;
    }
    match ctx.call(CAP, "health.liveness", serde_json::json!({})) {
        Ok(_) => v.check_bool(
            "health_liveness_discovery",
            true,
            "discovery (Songbird) health.liveness ok",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "health_liveness_discovery",
                &format!("discovery not reachable: {e}"),
            );
        }
        Err(e) => v.check_bool("health_liveness_discovery", false, &format!("error: {e}")),
    }
}

fn phase_multi_node_skips(v: &mut ValidationResult) {
    v.check_skip(
        "plasmodium_formation",
        "needs live Songbird mesh with 2+ covalent gates",
    );
    v.check_skip(
        "query_collective",
        "needs live Plasmodium for cross-gate capability.call",
    );
    v.check_skip(
        "capability_aggregation",
        "needs live mesh.peers to aggregate capabilities",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp032 — Plasmodium Formation")
        .with_provenance("exp032_plasmodium_formation", "2026-05-09")
        .run(
            "primalSpring Exp032: Plasmodium — Collective from Covalent Mesh",
            |v| {
                v.section("Phase 1: Bond Types");
                phase_bond_types(v);

                v.section("Phase 2: Graph Metadata");
                phase_graph_metadata(v);

                v.section("Phase 3: Live Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_live_discovery(v, &mut ctx);

                v.section("Phase 4: Multi-Node (skips)");
                phase_multi_node_skips(v);
            },
        );
}
