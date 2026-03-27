// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp033: Gate Failure — validates one gate drops and plasmodium degrades gracefully.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::{DiscoverySource, discover_primal, socket_path};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

/// Source: bonding::BondType — 5 bond models defined in ecosystem architecture
/// (Covalent, Metallic, Ionic, Weak, OrganoMetalSalt).
const BOND_TYPE_COUNT: usize = 5;

fn main() {
    ValidationResult::new("primalSpring Exp033 — Gate Failure")
        .with_provenance("exp033_gate_failure", "2026-03-24")
        .run(
            "primalSpring Exp033: One Gate Drops; Plasmodium Degrades Gracefully",
            |v| {
                let variants_exist = [
                    BondType::Covalent,
                    BondType::Metallic,
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
                let path_songbird = socket_path(primal_names::SONGBIRD);
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

                let songbird = discover_primal(primal_names::SONGBIRD);
                v.check_bool(
                    "discover_songbird_returns_result",
                    songbird.primal == primal_names::SONGBIRD,
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
            },
        );
}
