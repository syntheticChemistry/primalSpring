// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp031: Ionic Bond — validates cross-family limited capability sharing.

use primalspring::bonding::BondType;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp031 — Ionic Bond");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp031: Cross-Family Limited Capability Sharing");
    println!("{}", "=".repeat(72));

    let bond = BondType::Ionic;
    v.check_bool(
        "ionic_description_non_empty",
        !bond.description().is_empty(),
        &format!(
            "BondType::Ionic.description() is non-empty — {}",
            bond.description()
        ),
    );

    let beardog = discover_primal("beardog");
    v.check_bool(
        "discover_beardog_returns_result",
        beardog.primal == "beardog",
        "discover_primal returns DiscoveryResult for beardog",
    );

    v.check_skip(
        "cross_family_capability_sharing",
        "needs live primals from different families",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
