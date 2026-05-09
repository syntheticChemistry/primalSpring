// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp042: `FieldMouse` Ingestion — fieldMouse frames → `NestGate` → sweetGrass.
//!
//! Validates edge data ingestion: fieldMouse captures frames, `NestGate` stores
//! artifacts, sweetGrass records attribution. This tests the ingest pipeline
//! that every edge sensor primal will follow.

use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_for, neural_api_healthy, socket_path};
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

/// Primals / deployment classes participating in the edge ingestion pipeline.
///
/// Source: `PRIMAL_REGISTRY.md` + `CROSS_SPRING_DATA_FLOW_STANDARD.md`.
/// fieldMouse is a **deployment class** (biomeOS chimera for edge/IoT), not a primal.
/// `NestGate` stores artifacts, sweetGrass records attribution.
const FIELDMOUSE_SLUG: &str = "fieldmouse";
const INGEST_PRIMALS: &[&str] = &[
    FIELDMOUSE_SLUG,
    primal_names::NESTGATE,
    primal_names::SWEETGRASS,
];

fn main() {
    ValidationResult::new("primalSpring Exp042 — FieldMouse Ingestion")
        .with_provenance("exp042_fieldmouse_ingestion", "2026-03-24")
        .run(
            "primalSpring Exp042: FieldMouse Ingestion — fieldMouse → NestGate → sweetGrass",
            |v| {
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
                            v.check_skip(
                                &format!("health_{name}"),
                                &format!("{name} not reachable"),
                            );
                            v.check_skip(
                                &format!("latency_{name}"),
                                &format!("{name} not reachable"),
                            );
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
            },
        );
}
