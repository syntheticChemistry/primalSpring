// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp005: Atomic Subtraction — validates `AtomicType` hierarchy for graceful degradation.
//!
//! Phases:
//!   1. Type hierarchy — Tower ⊂ Node ⊂ FullNucleus, Nest ⊂ FullNucleus
//!   2. Capability containment — structural relationships between tiers

use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn phase_type_hierarchy(v: &mut ValidationResult) {
    let tower_primals = AtomicType::Tower.required_primals();
    let node_primals = AtomicType::Node.required_primals();
    let tower_in_node = tower_primals.iter().all(|p| node_primals.contains(p));
    v.check_bool(
        "tower_subset_of_node",
        tower_in_node,
        "Tower primals are subset of Node primals",
    );

    let full_primals = AtomicType::FullNucleus.required_primals();
    let node_in_full = node_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "node_subset_of_full_nucleus",
        node_in_full,
        "Node primals are subset of FullNucleus primals",
    );

    let nest_primals = AtomicType::Nest.required_primals();
    let nest_in_full = nest_primals.iter().all(|p| full_primals.contains(p));
    v.check_bool(
        "nest_subset_of_full_nucleus",
        nest_in_full,
        "Nest primals are subset of FullNucleus primals",
    );

    v.check_skip("live_degradation", "needs live primals");
}

fn phase_capability_containment(v: &mut ValidationResult) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    let node_caps = AtomicType::Node.required_capabilities();
    let full_caps = AtomicType::FullNucleus.required_capabilities();

    let tower_in_node = tower_caps.iter().all(|c| node_caps.contains(c));
    v.check_bool(
        "tower_caps_subset_of_node",
        tower_in_node,
        "Tower capabilities are subset of Node",
    );

    let nest_caps = AtomicType::Nest.required_capabilities();
    let nest_in_full = nest_caps.iter().all(|c| full_caps.contains(c));
    v.check_bool(
        "nest_caps_subset_of_full_nucleus",
        nest_in_full,
        "Nest capabilities are subset of FullNucleus",
    );

    let node_in_full = node_caps.iter().all(|c| full_caps.contains(c));
    v.check_bool(
        "node_caps_subset_of_full_nucleus",
        node_in_full,
        "Node capabilities are subset of FullNucleus",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp005 — Atomic Subtraction")
        .with_provenance("exp005_atomic_subtraction", "2026-05-09")
        .run(
            "primalSpring Exp005: Atomic Subtraction (graceful degradation)",
            |v| {
                v.section("Phase 1: Type hierarchy");
                phase_type_hierarchy(v);

                v.section("Phase 2: Capability containment");
                phase_capability_containment(v);
            },
        );
}
