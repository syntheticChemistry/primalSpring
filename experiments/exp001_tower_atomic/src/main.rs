// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp001: Tower Atomic — security + discovery capability validation.
//!
//! Validates the minimal NUCLEUS composition by **capability** rather than
//! primal identity. Discovers whoever provides "security" and "discovery"
//! at runtime, probes health, and validates the composition via the
//! capability-based path.

use primalspring::coordination::{
    AtomicType, check_capability_health, validate_composition_by_capability,
};
use primalspring::ipc::discover::{discover_by_capability, neural_api_healthy};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::run_experiment(
        "primalSpring Exp001 — Tower Atomic",
        "primalSpring Exp001: Tower Atomic (security + discovery capabilities)",
        |v| {
            let tower_caps = AtomicType::Tower.required_capabilities();
            v.check_count("tower_required_caps", tower_caps.len(), 2);

            for cap in tower_caps {
                let disc = discover_by_capability(cap);
                v.check_or_skip(
                    &format!("discover_{cap}"),
                    disc.socket.as_ref(),
                    &format!("{cap} provider not discovered — not running"),
                    |path, v| {
                        let provider = disc.resolved_primal.as_deref().unwrap_or("unknown");
                        v.check_bool(
                            &format!("discover_{cap}"),
                            true,
                            &format!("{cap} provided by {provider} at {}", path.display()),
                        );
                    },
                );
            }

            for cap in tower_caps {
                check_capability_health(v, cap);
            }

            let neural_ok = neural_api_healthy();
            if neural_ok {
                v.check_bool("neural_api", true, "Neural API reachable");
                let comp = validate_composition_by_capability(AtomicType::Tower);
                v.check_bool(
                    "composition_healthy",
                    comp.all_healthy,
                    "Tower composition all healthy (capability-based)",
                );
                v.check_bool(
                    "composition_discovery",
                    comp.discovery_ok,
                    "Tower discovery complete (capability-based)",
                );
                v.check_minimum("composition_caps", comp.total_capabilities, 2);
            } else {
                v.check_skip(
                    "neural_api",
                    "Neural API not reachable — biomeOS not running",
                );
                v.check_skip("composition_healthy", "requires Neural API");
                v.check_skip("composition_discovery", "requires Neural API");
                v.check_skip("composition_caps", "requires Neural API");
            }
        },
    );
}
