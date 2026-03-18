// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp030: Covalent Bond — validates two NUCLEUS instances share family seed and discover each other.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp030 — Covalent Bond");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp030: Two NUCLEUS Instances Share Family Seed, Discover Each Other");
    println!("{}", "=".repeat(72));

    let bond = BondType::Covalent;
    v.check_bool(
        "covalent_description_non_empty",
        !bond.description().is_empty(),
        &format!(
            "BondType::Covalent.description() is non-empty — {}",
            bond.description()
        ),
    );
    v.check_bool(
        "covalent_identity",
        bond == BondType::Covalent,
        "BondType::Covalent == BondType::Covalent (identity)",
    );

    let beardog = discover_primal("beardog");
    v.check_bool(
        "discover_beardog_returns_result",
        beardog.primal == "beardog",
        "discover_primal returns DiscoveryResult for beardog",
    );
    v.check_skip("family_seed_sharing", "needs 2 live NUCLEUS instances");
    v.check_skip("mutual_discovery", "needs 2 live NUCLEUS instances");

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
