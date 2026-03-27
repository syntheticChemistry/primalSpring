// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp021: RootPulse Branch Merge — validates branch and merge operations
//! via Neural API `capability.call`.
//!
//! When the provenance trio is running, exercises real branch/merge DAG
//! operations through rhizoCrypt. When unavailable, validates structural
//! properties and skips live IPC gracefully.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::{discover_primal, neural_api_healthy};
use primalspring::ipc::provenance::{
    self, ProvenanceStatus, begin_experiment_session, record_experiment_step, rootpulse_branch,
    rootpulse_merge,
};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp021 — RootPulse Branch Merge")
        .with_provenance("exp021_rootpulse_branch_merge", "2026-03-24")
        .run("primalSpring Exp021: Branch + Merge", |v| {
            // ── Structural: graph presence ──

            let graphs = EmergentSystem::RootPulse.required_graphs();
            let has_branch = graphs.contains(&"rootpulse_branch");
            let has_merge = graphs.contains(&"rootpulse_merge");
            v.check_bool(
                "rootpulse_has_branch_and_merge",
                has_branch && has_merge,
                &format!(
                    "EmergentSystem::RootPulse.required_graphs() contains rootpulse_branch and rootpulse_merge: {graphs:?}"
                ),
            );

            // ── Discovery: trio probe ──

            for (name, discovery) in [
                ("rhizocrypt", discover_primal(primal_names::RHIZOCRYPT)),
                ("loamspine", discover_primal(primal_names::LOAMSPINE)),
                ("sweetgrass", discover_primal(primal_names::SWEETGRASS)),
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
                            &format!(
                                "health ok: {}, latency: {}µs",
                                health.health_ok, health.latency_us
                            ),
                        );
                        v.check_bool(
                            &format!("{name}_capabilities"),
                            !health.capabilities.is_empty(),
                            &format!("capabilities: {:?}", health.capabilities),
                        );
                    },
                );
            }

            // ── Branch/Merge operations via Neural API ──

            let neural_live = neural_api_healthy();
            let trio_health = provenance::trio_health();
            let trio_available = neural_live && trio_health.iter().all(|(_d, ok)| *ok);

            v.check_or_skip(
                "branch_merge_operations",
                trio_available.then_some(()),
                "branch/merge operations need live trio + Neural API",
                |(), v| {
                    // Create a main session
                    let main_session = begin_experiment_session("exp021-branch-merge-main");
                    if main_session.status == ProvenanceStatus::Unavailable {
                        v.check_skip("branch_create", "main session creation failed");
                        v.check_skip("branch_record", "main session creation failed");
                        v.check_skip("merge_execute", "main session creation failed");
                        return;
                    }

                    let _step = record_experiment_step(
                        &main_session.id,
                        &serde_json::json!({ "action": "initial_commit", "phase": "main" }),
                    );

                    // Branch from main
                    let branch = rootpulse_branch(&main_session.id, "feature-exp021");
                    v.check_bool(
                        "branch_create",
                        branch.status != ProvenanceStatus::Unavailable,
                        &format!("branch created: {}", branch.id),
                    );

                    if branch.status == ProvenanceStatus::Unavailable {
                        v.check_skip("branch_record", "branch creation failed");
                        v.check_skip("merge_execute", "branch creation failed");
                        return;
                    }

                    // Record work on branch
                    let branch_step = record_experiment_step(
                        &branch.id,
                        &serde_json::json!({ "action": "branch_work", "phase": "feature" }),
                    );
                    v.check_bool(
                        "branch_record",
                        branch_step.status != ProvenanceStatus::Unavailable,
                        &format!("branch step recorded: {}", branch_step.id),
                    );

                    // Merge branch back into main
                    let merge = rootpulse_merge(&branch.id, &main_session.id);
                    v.check_bool(
                        "merge_execute",
                        merge.status != ProvenanceStatus::Unavailable,
                        &format!("merge completed: {}", merge.id),
                    );
                },
            );
        });
}
