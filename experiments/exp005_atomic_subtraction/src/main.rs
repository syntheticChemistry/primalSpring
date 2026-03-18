// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp005: Atomic Subtraction — validates AtomicType hierarchy (Tower⊂Node⊂FullNucleus, Nest⊂FullNucleus) for graceful degradation.

use primalspring::coordination::AtomicType;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp005 — Atomic Subtraction");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp005: Atomic Subtraction (graceful degradation)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // Real check: type system validates AtomicType hierarchy
    // Tower ⊂ Node (Node adds toadstool to Tower)
    let tower_primals = AtomicType::Tower.required_primals();
    let node_primals = AtomicType::Node.required_primals();
    let tower_in_node = tower_primals.iter().all(|p| node_primals.contains(p));
    v.check_bool(
        "tower_subset_of_node",
        tower_in_node,
        "Tower primals are subset of Node primals",
    );

    // Node ⊂ FullNucleus
    let full_primals = AtomicType::FullNucleus.required_primals();
    let node_in_full = node_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "node_subset_of_full_nucleus",
        node_in_full,
        "Node primals are subset of FullNucleus primals",
    );

    // Nest ⊂ FullNucleus
    let nest_primals = AtomicType::Nest.required_primals();
    let nest_in_full = nest_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "nest_subset_of_full_nucleus",
        nest_in_full,
        "Nest primals are subset of FullNucleus primals",
    );

    // Skip: live degradation testing needs live primals
    v.check_skip("live_degradation", "needs live primals");

    v.finish();
    std::process::exit(v.exit_code());
}
