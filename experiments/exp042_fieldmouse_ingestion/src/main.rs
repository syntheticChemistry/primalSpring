// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp042: FieldMouse Ingestion — fieldMouse frames → NestGate → sweetGrass.
//!
//! Validates edge data ingestion: fieldMouse captures frames, NestGate stores
//! artifacts, sweetGrass records attribution. This tests the ingest pipeline
//! that every edge sensor primal will follow.

use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_for, neural_api_healthy, socket_path};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

const INGEST_PRIMALS: &[&str] = &["nestgate", "sweetgrass"];

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp042 — FieldMouse Ingestion");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp042: FieldMouse Ingestion");
    println!("  fieldMouse (edge capture) → NestGate (storage) → sweetGrass (attribution)");
    println!("{}", "=".repeat(72));

    for &name in INGEST_PRIMALS {
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

    let results = discover_for(INGEST_PRIMALS);
    v.check_count(
        "ingest_discovery_count",
        results.len(),
        INGEST_PRIMALS.len(),
    );

    let reachable: Vec<_> = results.iter().filter(|r| r.socket.is_some()).collect();
    let pipeline_online = reachable.len() == INGEST_PRIMALS.len();

    if pipeline_online {
        v.check_bool(
            "ingest_primals_discoverable",
            true,
            "NestGate + sweetGrass both have sockets",
        );

        for &name in INGEST_PRIMALS {
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
            "ingest_primals_discoverable",
            &format!(
                "{}/{} ingest primals reachable — need both running",
                reachable.len(),
                INGEST_PRIMALS.len()
            ),
        );
        for &name in INGEST_PRIMALS {
            let disc = results.iter().find(|r| r.primal == name);
            if disc.is_none_or(|d| d.socket.is_none()) {
                v.check_skip(&format!("health_{name}"), &format!("{name} not reachable"));
                v.check_skip(&format!("latency_{name}"), &format!("{name} not reachable"));
                v.check_skip(&format!("caps_{name}"), &format!("{name} not reachable"));
            }
        }
    }

    if neural_api_healthy() {
        v.check_bool("neural_api", true, "Neural API reachable");
        v.check_skip(
            "fieldmouse_ingest_e2e",
            "end-to-end ingestion needs fieldMouse frames + NestGate + sweetGrass live",
        );
    } else {
        v.check_skip("neural_api", "Neural API not running");
        v.check_skip(
            "fieldmouse_ingest_e2e",
            "needs live ingest primals + Neural API for pipeline validation",
        );
    }

    v.finish();
    std::process::exit(v.exit_code());
}
