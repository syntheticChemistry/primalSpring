// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates cross-tower federation from `BearDog`'s BYOB manifest.
//! Source: phase1/beardog/showcase/04-advanced-features/09-cross-tower-federation

use primalspring::coordination::AtomicType;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp056 — Cross Tower Federation");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp056: BearDog Cross-Tower Federation (BYOB)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let variants = [
        AtomicType::Tower,
        AtomicType::Node,
        AtomicType::Nest,
        AtomicType::FullNucleus,
    ];
    let all_have_required = variants.iter().all(|t| !t.required_primals().is_empty());
    v.check_bool(
        "all_atomic_types_have_required_primals",
        all_have_required,
        "all AtomicType variants have required_primals",
    );

    let full_primals: std::collections::HashSet<&str> = AtomicType::FullNucleus
        .required_primals()
        .iter()
        .copied()
        .collect();
    let tower_primals = AtomicType::Tower.required_primals();
    let full_is_superset = tower_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "full_nucleus_superset_of_tower",
        full_is_superset,
        "FullNucleus primals is superset of Tower primals",
    );

    v.check_skip(
        "cross_tower_discovery",
        "cross-tower discovery needs live primals",
    );
    v.check_skip("timeout_handling", "timeout handling needs live federation");

    v.finish();
    std::process::exit(v.exit_code());
}
