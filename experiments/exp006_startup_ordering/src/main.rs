// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp006: Startup Ordering — validates biomeOS germination waves from deploy graphs.
//!
//! Uses `topological_waves()` to compute the startup wave ordering from
//! each atomic graph's dependency edges, then validates that the waves
//! respect the Tower ⊂ Node ⊂ FullNucleus containment.

use std::path::Path;

use primalspring::coordination::AtomicType;
use primalspring::deploy::{load_graph, topological_waves};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp006 — Startup Ordering")
        .with_provenance("exp006_startup_ordering", "2026-03-24")
        .run(
            "primalSpring Exp006: Startup Ordering (topological waves from deploy graphs)",
            |v| {
                let tower_caps = AtomicType::Tower.required_capabilities();
                let node_caps = AtomicType::Node.required_capabilities();
                let tower_subset_node = tower_caps.iter().all(|c| node_caps.contains(c));
                v.check_bool(
                    "tower_caps_subset_of_node",
                    tower_subset_node,
                    "Tower capabilities are subset of Node",
                );

                let full_caps = AtomicType::FullNucleus.required_capabilities();
                let node_subset_full = node_caps.iter().all(|c| full_caps.contains(c));
                v.check_bool(
                    "node_caps_subset_of_full_nucleus",
                    node_subset_full,
                    "Node capabilities are subset of FullNucleus",
                );

                let graphs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs");

                for (name, expected_min_waves) in [
                    ("tower_atomic_bootstrap.toml", 2),
                    ("node_atomic_compute.toml", 3),
                    ("nucleus_complete.toml", 4),
                ] {
                    let path = graphs_dir.join(name);
                    match load_graph(&path) {
                        Ok(graph) => match topological_waves(&graph) {
                            Ok(waves) => {
                                v.check_minimum(
                                    &format!("waves_{name}"),
                                    waves.len(),
                                    expected_min_waves,
                                );
                                let first_wave = &waves[0];
                                v.check_bool(
                                    &format!("wave0_has_root_{name}"),
                                    !first_wave.is_empty(),
                                    &format!(
                                        "{name} wave 0 contains {} root node(s)",
                                        first_wave.len()
                                    ),
                                );
                            }
                            Err(e) => {
                                v.check_bool(
                                    &format!("waves_{name}"),
                                    false,
                                    &format!("{name} topological sort failed: {e}"),
                                );
                            }
                        },
                        Err(e) => {
                            v.check_skip(
                                &format!("waves_{name}"),
                                &format!("{name} not found: {e}"),
                            );
                        }
                    }
                }
            },
        );
}
