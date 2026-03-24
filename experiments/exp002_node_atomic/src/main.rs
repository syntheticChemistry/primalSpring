// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp002: Node Atomic — validates security + discovery + compute capabilities.
//!
//! Capability-based validation: discovers providers by what they offer,
//! not by name. Node = Tower + compute.

use primalspring::coordination::{AtomicType, check_capability_health};
use primalspring::ipc::discover::discover_by_capability;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp002 — Node Atomic")
        .with_provenance("exp002_node_atomic", "2026-03-24")
        .run(
            "primalSpring Exp002: Node Atomic (security + discovery + compute)",
            |v| {
                let node_caps = AtomicType::Node.required_capabilities();
                v.check_count("node_required_caps", node_caps.len(), 3);

                for cap in node_caps {
                    let disc = discover_by_capability(cap);
                    let provider = disc.resolved_primal.as_deref().unwrap_or("not found");
                    v.check_or_skip(
                        &format!("discover_{cap}"),
                        disc.socket.as_ref(),
                        &format!("{cap} provider not discovered — not running"),
                        |path, v| {
                            v.check_bool(
                                &format!("discover_{cap}"),
                                true,
                                &format!("{cap} provided by {provider} at {}", path.display()),
                            );
                        },
                    );
                }

                for cap in node_caps {
                    check_capability_health(v, cap);
                }
            },
        );
}
