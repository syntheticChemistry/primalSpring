// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp032: Plasmodium Formation — validates collective formation from
//! covalently bonded NUCLEUS instances via Songbird mesh.
//!
//! Phase 1 (structural): All bond types, Metallic properties, graph metadata.
//! Phase 2 (live, when available): mesh.peers, PlasmodiumState, cross-gate routing.

use std::path::Path;

use primalspring::bonding::graph_metadata::validate_graph_bonding;
use primalspring::bonding::BondType;
use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp032 — Plasmodium Formation");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp032: Plasmodium — Collective from Covalent Mesh");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // --- Phase 1: All bond types present and complete ---
    let all_variants = BondType::all();
    v.check_bool(
        "bond_type_count",
        all_variants.len() == 5,
        &format!("BondType::all() has 5 variants (got {})", all_variants.len()),
    );
    v.check_bool(
        "all_bond_types_have_descriptions",
        all_variants.iter().all(|bt| !bt.description().is_empty()),
        "all BondType variants have non-empty descriptions",
    );

    // Metallic (electron sea) — key for Plasmodium specialization
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

    // --- Phase 2: Family-scoped socket paths ---
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let path_songbird = socket_path("songbird");
    let path_contains_family = path_songbird
        .to_string_lossy()
        .contains(&format!("-{family_id}.sock"));
    v.check_bool(
        "socket_path_includes_family_id",
        path_contains_family,
        &format!(
            "socket path includes FAMILY_ID: {} (path: {})",
            family_id,
            path_songbird.display()
        ),
    );

    // --- Phase 3: Graph metadata for basement HPC ---
    let graph_path = Path::new("graphs/multi_node/basement_hpc_covalent.toml");
    v.check_or_skip(
        "hpc_graph_metadata",
        graph_path.exists().then_some(&()),
        "basement_hpc_covalent.toml not found",
        |_, v| {
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

    // --- Phase 4: Live Songbird probing ---
    let songbird = discover_primal("songbird");
    v.check_bool(
        "discover_songbird_returns_result",
        songbird.primal == "songbird",
        "discover_primal returns DiscoveryResult for songbird",
    );
    v.check_or_skip(
        "probe_songbird",
        songbird.socket.as_ref(),
        "songbird socket not found (needed for multi-NUCLEUS mesh)",
        |_, v| {
            let health = probe_primal("songbird");
            v.check_bool(
                "songbird_health",
                health.health_ok,
                &format!(
                    "health ok: {}, latency: {}µs",
                    health.health_ok, health.latency_us
                ),
            );
        },
    );

    // --- Phase 5: Live multi-node (needs 2+ NUCLEUS) ---
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

    v.finish();
    std::process::exit(v.exit_code());
}
