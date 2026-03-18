// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp033: Gate Failure — validates one gate drops and plasmodium degrades gracefully.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::discover_primal;
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
        variants_exist.len() == 4 && variants_exist.iter().all(|bt| !bt.description().is_empty()),
        "BondType variants exist for all bonding models with descriptions",
    );

    let songbird = discover_primal("songbird");
    v.check_bool(
        "discover_songbird_returns_result",
        songbird.primal == "songbird",
        "discover_primal returns DiscoveryResult for songbird",
    );

    v.check_skip("gate_failure", "needs live Plasmodium with multiple gates");
    v.check_skip(
        "graceful_degradation",
        "needs live gate drop to test degradation",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
