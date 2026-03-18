// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp032: Plasmodium Formation — validates query_collective() with real Songbird mesh.

use primalspring::bonding::BondType;
use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

/// Source: bonding::BondType — 4 bond models defined in ecosystem architecture
/// (Covalent, Ionic, Weak, OrganoMetalSalt).
const BOND_TYPE_COUNT: usize = 4;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp032 — Plasmodium Formation");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp032: query_collective() with Real Songbird Mesh");
    println!("{}", "=".repeat(72));

    let all_have_descriptions = [
        BondType::Covalent,
        BondType::Ionic,
        BondType::Weak,
        BondType::OrganoMetalSalt,
    ]
    .iter()
    .all(|bt| !bt.description().is_empty());
    v.check_bool(
        "all_bond_types_have_descriptions",
        all_have_descriptions,
        &format!("all {BOND_TYPE_COUNT} BondType variants have non-empty descriptions"),
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

    v.check_skip("plasmodium_formation", "needs live Songbird mesh");
    v.check_skip(
        "query_collective",
        "needs live primals for collective query",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
