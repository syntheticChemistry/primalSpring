// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp041: Provenance Trio Science — rhizoCrypt + loamSpine + sweetGrass.
//!
//! Validates that the provenance trio primals (rhizoCrypt for derivation
//! tracking, loamSpine for immutable anchoring, sweetGrass for attribution)
//! are discoverable and compose correctly. Any spring experiment that
//! produces a result should be able to route through the trio for full
//! provenance.

use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_for, discover_primal, neural_api_healthy, socket_path};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

const TRIO_PRIMALS: &[&str] = &["rhizocrypt", "loamspine", "sweetgrass"];

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp041 — Provenance Trio Science");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp041: Provenance Trio Science");
    println!("  rhizoCrypt (derivation) + loamSpine (anchoring) + sweetGrass (attribution)");
    println!("{}", "=".repeat(72));

    for &name in TRIO_PRIMALS {
        let path = socket_path(name);
        let valid = path.to_string_lossy().contains("biomeos")
            && path.to_string_lossy().contains(name)
            && path.to_string_lossy().ends_with(".sock");
        v.check_bool(
            &format!("socket_path_{name}"),
            valid,
            &format!("socket_path({name}) = {}", path.display()),
        );
    }

    let results = discover_for(TRIO_PRIMALS);
    v.check_count("trio_discovery_count", results.len(), TRIO_PRIMALS.len());

    let reachable: Vec<_> = results.iter().filter(|r| r.socket.is_some()).collect();
    let trio_online = reachable.len() == TRIO_PRIMALS.len();

    if trio_online {
        v.check_bool(
            "trio_all_discoverable",
            true,
            "all three provenance primals have sockets",
        );

        for &name in TRIO_PRIMALS {
            let health = probe_primal(name);
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
            v.check_minimum(&format!("caps_{name}"), health.capabilities.len(), 1);
        }
    } else {
        v.check_skip(
            "trio_all_discoverable",
            &format!(
                "{}/{} trio primals reachable — need all three running",
                reachable.len(),
                TRIO_PRIMALS.len()
            ),
        );
        for &name in TRIO_PRIMALS {
            let disc = discover_primal(name);
            if disc.socket.is_none() {
                v.check_skip(&format!("health_{name}"), &format!("{name} not reachable"));
                v.check_skip(&format!("latency_{name}"), &format!("{name} not reachable"));
                v.check_skip(&format!("caps_{name}"), &format!("{name} not reachable"));
            }
        }
    }

    if neural_api_healthy() {
        v.check_bool("neural_api", true, "Neural API reachable");
        v.check_skip(
            "provenance_chain_e2e",
            "end-to-end provenance chain requires experiment result + trio coordination",
        );
    } else {
        v.check_skip("neural_api", "Neural API not running");
        v.check_skip(
            "provenance_chain_e2e",
            "needs live trio primals + Neural API for chain validation",
        );
    }

    v.finish();
    std::process::exit(v.exit_code());
}
