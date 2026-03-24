// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp041: Provenance Trio Science — rhizoCrypt + loamSpine + sweetGrass.
//!
//! Validates that the provenance trio primals are discoverable, compose
//! correctly, and can execute a full provenance chain: begin session →
//! record steps → dehydrate → commit → attribute.
//!
//! When trio is running, exercises the full chain via Neural API.
//! When unavailable, validates structural properties and skips live IPC.

use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_for, discover_primal, neural_api_healthy, socket_path};
use primalspring::ipc::provenance::{
    self, ProvenanceStatus, begin_experiment_session, complete_experiment, record_experiment_step,
};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

const TRIO_PRIMALS: &[&str] = &["rhizocrypt", "loamspine", "sweetgrass"];

fn validate_e2e_chain(v: &mut ValidationResult) {
    // Step 1: Begin session
    let session = begin_experiment_session("exp041-trio-science");
    v.check_bool(
        "chain_begin_session",
        session.status != ProvenanceStatus::Unavailable,
        &format!("DAG session: {}", session.id),
    );

    if session.status == ProvenanceStatus::Unavailable {
        v.check_skip("chain_record_steps", "session creation failed");
        v.check_skip("chain_pipeline", "session creation failed");
        return;
    }

    // Step 2: Record experiment steps
    let steps = [
        serde_json::json!({ "action": "discover_trio", "result": "3/3 found" }),
        serde_json::json!({ "action": "health_check", "result": "all healthy" }),
        serde_json::json!({ "action": "validate_capabilities", "result": "pass" }),
    ];
    let mut steps_ok = 0;
    for (i, step) in steps.iter().enumerate() {
        let result = record_experiment_step(&session.id, step);
        if result.status != ProvenanceStatus::Unavailable {
            steps_ok += 1;
        }
        v.check_bool(
            &format!("chain_step_{i}"),
            result.status != ProvenanceStatus::Unavailable,
            &format!("step {i}: vertex {}", result.id),
        );
    }
    v.check_count("chain_record_steps", steps_ok, steps.len());

    // Step 3: Complete pipeline (dehydrate → commit → attribute)
    let pipeline = complete_experiment(&session.id);
    v.check_bool(
        "chain_dehydrate",
        !pipeline.merkle_root.is_empty(),
        &format!("merkle_root: {}", pipeline.merkle_root),
    );
    v.check_bool(
        "chain_commit",
        !pipeline.commit_id.is_empty(),
        &format!("commit_id: {}", pipeline.commit_id),
    );
    v.check_bool(
        "chain_attribute",
        pipeline.status == ProvenanceStatus::Complete,
        &format!("braid_id: {}", pipeline.braid_id),
    );
    v.check_bool(
        "chain_pipeline_complete",
        pipeline.status == ProvenanceStatus::Complete
            && !pipeline.merkle_root.is_empty()
            && !pipeline.commit_id.is_empty(),
        "full provenance chain: session → steps → dehydrate → commit → attribute",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp041 — Provenance Trio Science")
        .with_provenance("exp041_provenance_trio_science", "2026-03-24")
        .run(
            "primalSpring Exp041: Provenance Trio Science — rhizoCrypt + loamSpine + sweetGrass",
            |v| {
                // ── Structural: socket path conventions ──

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

                // ── Discovery: trio reachability ──

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

                // ── Neural API health ──

                let neural_live = neural_api_healthy();
                v.check_or_skip(
                    "neural_api",
                    neural_live.then_some(()),
                    "Neural API not running",
                    |(), v| {
                        v.check_bool("neural_api_reachable", true, "Neural API reachable");
                    },
                );

                // ── Trio capability health via Neural API ──

                let trio_health = provenance::trio_health();
                let trio_caps_healthy = neural_live && trio_health.iter().all(|(_d, ok)| *ok);

                v.check_or_skip(
                    "trio_capability_health",
                    trio_caps_healthy.then_some(()),
                    "trio capabilities not all healthy via Neural API",
                    |(), v| {
                        for (domain, healthy) in &trio_health {
                            v.check_bool(
                                &format!("cap_health_{domain}"),
                                *healthy,
                                &format!("{domain} capability domain healthy"),
                            );
                        }
                    },
                );

                // ── E2E Provenance Chain ──

                v.check_or_skip(
                    "provenance_chain_e2e",
                    trio_caps_healthy.then_some(()),
                    "needs live trio primals + Neural API for chain validation",
                    |(), v| validate_e2e_chain(v),
                );
            },
        );
}
