// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp004: Full NUCLEUS — validate all 8 capability domains.
//!
//! Discovers providers by capability, validates health and latency for
//! each, then runs the full capability-based composition validation.
//! Gracefully degrades when providers are not running.

use primalspring::coordination::{
    AtomicType, check_capability_health, validate_composition_by_capability,
};
use primalspring::ipc::discover::{discover_capabilities_for, neural_api_healthy};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp004 — Full NUCLEUS")
        .with_provenance("exp004_full_nucleus", "2026-04-13")
        .run(
            "primalSpring Exp004: Full NUCLEUS (all capability domains)",
            |v| {
                let required = AtomicType::FullNucleus.required_capabilities();
                v.check_count("full_nucleus_required_caps", required.len(), 13);

                let discovered = discover_capabilities_for(required);
                let found_count = discovered.iter().filter(|d| d.socket.is_some()).count();
                println!(
                    "  [INFO] discovered {found_count}/{} capability providers",
                    required.len()
                );

                for cap in required {
                    check_capability_health(v, cap);
                }

                if neural_api_healthy() {
                    let comp = validate_composition_by_capability(AtomicType::FullNucleus);
                    v.check_bool(
                        "composition_all_healthy",
                        comp.all_healthy,
                        "all capability providers healthy",
                    );
                    v.check_bool(
                        "composition_discovery_ok",
                        comp.discovery_ok,
                        "all capabilities discovered",
                    );
                    v.check_minimum("composition_total_caps", comp.total_capabilities, 8);
                    v.check_latency(
                        "composition_aggregate_latency",
                        comp.primals.iter().map(|p| p.latency_us).sum(),
                        tolerances::NUCLEUS_STARTUP_MAX_US,
                    );
                } else {
                    v.check_skip("composition_all_healthy", "Neural API not reachable");
                    v.check_skip("composition_discovery_ok", "Neural API not reachable");
                    v.check_skip("composition_total_caps", "Neural API not reachable");
                    v.check_skip("composition_aggregate_latency", "Neural API not reachable");
                }
            },
        );
}
