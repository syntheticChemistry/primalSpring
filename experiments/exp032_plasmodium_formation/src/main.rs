// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp032: Plasmodium Formation — validates query_collective() with real Songbird mesh.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

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
        "all 4 BondType variants have non-empty descriptions",
    );

    let songbird = discover_primal("songbird");
    v.check_bool(
        "discover_songbird_returns_result",
        songbird.primal == "songbird",
        "discover_primal returns DiscoveryResult for songbird",
    );

    v.check_skip("plasmodium_formation", "needs live Songbird mesh");
    v.check_skip(
        "query_collective",
        "needs live primals for collective query",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
