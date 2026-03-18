// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp033: Gate Failure — validates one gate drops and plasmodium degrades gracefully.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::{discover_primal, socket_path, DiscoverySource};

/// Source: bonding::BondType — 4 bond models defined in ecosystem architecture
/// (Covalent, Ionic, Weak, OrganoMetalSalt).
const BOND_TYPE_COUNT: usize = 4;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp033 — Gate Failure");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp033: One Gate Drops; Plasmodium Degrades Gracefully");
    println!("{}", "=".repeat(72));

    let variants_exist = [
        BondType::Covalent,
        BondType::Ionic,
        BondType::Weak,
        BondType::OrganoMetalSalt,
    ];
    v.check_bool(
        "bond_type_variants_for_all_models",
        variants_exist.len() == BOND_TYPE_COUNT
            && variants_exist.iter().all(|bt| !bt.description().is_empty()),
        "BondType variants exist for all bonding models with descriptions",
    );

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

    let songbird = discover_primal("songbird");
    v.check_bool(
        "discover_songbird_returns_result",
        songbird.primal == "songbird",
        "discover_primal returns DiscoveryResult for songbird",
    );

    let missing = discover_primal("nonexistent_primal_xyzzy_12345");
    v.check_bool(
        "discovery_graceful_for_missing_primal",
        missing.socket.is_none() && missing.source == DiscoverySource::NotFound,
        "discover_primal returns NotFound for missing primal without panic",
    );

    v.check_skip("gate_failure", "needs live Plasmodium with multiple gates");
    v.check_skip(
        "graceful_degradation",
        "needs live gate drop to test degradation",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
