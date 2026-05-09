// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp003: Nest Atomic — validates security + discovery + storage + AI capabilities.
//!
//! Capability-based validation: discovers providers by what they offer,
//! not by name. Nest = Tower + storage + AI bridge.

use primalspring::coordination::{AtomicType, check_capability_health};
use primalspring::ipc::discover::discover_by_capability;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp003 — Nest Atomic")
        .with_provenance("exp003_nest_atomic", "2026-03-24")
        .run(
            "primalSpring Exp003: Nest Atomic (security + discovery + storage + ai)",
            |v| {
                let nest_caps = AtomicType::Nest.required_capabilities();
                v.check_count("nest_required_caps", nest_caps.len(), 4);

                for cap in nest_caps {
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

                for cap in nest_caps {
                    check_capability_health(v, cap);
                }
            },
        );
}
