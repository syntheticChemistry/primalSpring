// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp004: Full NUCLEUS — validate all 8 primals in composition.
//!
//! Discovers each primal at runtime, validates health and capabilities,
//! and checks the full composition. Gracefully degrades when primals
//! are not running — skips are honest, never faked passes.

use primalspring::coordination::{AtomicType, probe_primal, validate_composition};
use primalspring::ipc::discover::{discover_for, neural_api_healthy};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp004 — Full NUCLEUS");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp004: Full NUCLEUS (all primals)");
    println!("{}", "=".repeat(72));

    let required = AtomicType::FullNucleus.required_primals();
    v.check_count("full_nucleus_required_count", required.len(), 8);

    let discovered = discover_for(required);
    let found_count = discovered.iter().filter(|d| d.socket.is_some()).count();
    println!(
        "  [INFO] discovered {found_count}/{} primal sockets",
        required.len()
    );

    for name in required {
        let health = probe_primal(name);
        if health.socket_found {
            v.check_bool(
                &format!("health_{name}"),
                health.health_ok,
                &format!("{name} health.check"),
            );
            v.check_latency(
                &format!("latency_{name}"),
                health.latency_us,
                tolerances::HEALTH_CHECK_MAX_US,
            );
        } else {
            v.check_skip(&format!("health_{name}"), &format!("{name} not reachable"));
            v.check_skip(&format!("latency_{name}"), &format!("{name} not reachable"));
        }
    }

    if neural_api_healthy() {
        let comp = validate_composition(AtomicType::FullNucleus);
        v.check_bool(
            "composition_all_healthy",
            comp.all_healthy,
            "all primals healthy",
        );
        v.check_bool(
            "composition_discovery_ok",
            comp.discovery_ok,
            "all sockets discovered",
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

    v.finish();
    std::process::exit(v.exit_code());
}
