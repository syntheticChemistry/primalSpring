// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp006: Startup Ordering — validates biomeOS dependency resolution (Tower⊂Node⊂FullNucleus).

use primalspring::coordination::AtomicType;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp006 — Startup Ordering");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp006: Startup Ordering (biomeOS dependency resolution)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // Real check: Tower primals are subset of Node
    let tower_primals = AtomicType::Tower.required_primals();
    let node_primals = AtomicType::Node.required_primals();
    let tower_subset_node = tower_primals.iter().all(|p| node_primals.contains(p));
    v.check_bool(
        "tower_subset_of_node",
        tower_subset_node,
        "Tower primals are subset of Node (dependency ordering)",
    );

    // Real check: Node primals are subset of FullNucleus
    let full_primals = AtomicType::FullNucleus.required_primals();
    let node_subset_full = node_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "node_subset_of_full_nucleus",
        node_subset_full,
        "Node primals are subset of FullNucleus (dependency ordering)",
    );

    // Skip: actual startup ordering with live primals
    v.check_skip(
        "dependency_ordering",
        "needs live primals for actual startup sequence",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
