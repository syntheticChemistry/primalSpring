// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp022: RootPulse Diff Federate — validates Merkle diff and federation
//! via Neural API `capability.call`.
//!
//! When the provenance trio is running, exercises real diff and federate
//! operations through rhizoCrypt. When unavailable, validates structural
//! properties and skips live IPC gracefully.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::{discover_primal, neural_api_healthy};
use primalspring::ipc::provenance::{
    self, ProvenanceStatus, begin_experiment_session, record_experiment_step, rootpulse_diff,
    rootpulse_federate,
};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp022 — RootPulse Diff Federate");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp022: Merkle Diff + Federation");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    // ── Structural: graph presence ──

    let graphs = EmergentSystem::RootPulse.required_graphs();
    let has_diff = graphs.contains(&"rootpulse_diff");
    let has_federate = graphs.contains(&"rootpulse_federate");
    v.check_bool(
        "rootpulse_has_diff_and_federate",
        has_diff && has_federate,
        &format!(
            "EmergentSystem::RootPulse.required_graphs() contains rootpulse_diff and rootpulse_federate: {graphs:?}"
        ),
    );

    // ── Discovery: trio probe ──

    for (name, discovery) in [
        ("rhizocrypt", discover_primal("rhizocrypt")),
        ("loamspine", discover_primal("loamspine")),
        ("sweetgrass", discover_primal("sweetgrass")),
    ] {
        v.check_or_skip(
            &format!("probe_{name}"),
            discovery.socket.as_ref(),
            &format!("{name} socket not found"),
            |_, v| {
                let health = probe_primal(name);
                v.check_bool(
                    &format!("{name}_health"),
                    health.health_ok,
                    &format!("health ok: {}, latency: {}µs", health.health_ok, health.latency_us),
                );
                v.check_bool(
                    &format!("{name}_capabilities"),
                    !health.capabilities.is_empty(),
                    &format!("capabilities: {:?}", health.capabilities),
                );
            },
        );
    }

    // ── Diff/Federate operations via Neural API ──

    let neural_live = neural_api_healthy();
    let trio_health = provenance::trio_health();
    let trio_available = neural_live && trio_health.iter().all(|(_d, ok)| *ok);

    v.check_or_skip(
        "diff_federate_operations",
        trio_available.then_some(()),
        "diff/federate operations need live trio + Neural API",
        |(), v| {
            // Create two divergent sessions for diff
            let session_a = begin_experiment_session("exp022-diff-source");
            let session_b = begin_experiment_session("exp022-diff-target");

            if session_a.status == ProvenanceStatus::Unavailable
                || session_b.status == ProvenanceStatus::Unavailable
            {
                v.check_skip("diff_execute", "session creation failed");
                v.check_skip("federate_execute", "session creation failed");
                return;
            }

            // Record divergent work on each session
            let _step_a = record_experiment_step(
                &session_a.id,
                &serde_json::json!({ "action": "work_a", "data": "source_change" }),
            );
            let _step_b = record_experiment_step(
                &session_b.id,
                &serde_json::json!({ "action": "work_b", "data": "target_change" }),
            );

            // Diff between sessions
            let diff = rootpulse_diff(&session_a.id, &session_b.id);
            v.check_bool(
                "diff_execute",
                diff.status != ProvenanceStatus::Unavailable,
                &format!("diff computed: {}", diff.id),
            );

            // Federate session to a remote (best-effort — remote may not exist)
            let federate = rootpulse_federate(&session_a.id, "local://exp022-test");
            v.check_bool(
                "federate_execute",
                federate.status != ProvenanceStatus::Unavailable,
                &format!("federation: {}", federate.id),
            );
        },
    );

    v.finish();
    std::process::exit(v.exit_code());
}
