// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp020: RootPulse Full Commit — validates the 6-phase commit pipeline
//! via Neural API `capability.call`.
//!
//! Phases: health → dehydrate → sign → store → commit → attribute.
//!
//! When the provenance trio is running, exercises real capability routing.
//! When unavailable, phases degrade to SKIP — no false failures.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::{discover_primal, neural_api_healthy};
use primalspring::ipc::provenance::{
    self, ProvenanceStatus, begin_experiment_session, complete_experiment, record_experiment_step,
};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn rootpulse_required_graphs(v: &mut ValidationResult) {
    let graphs = EmergentSystem::RootPulse.required_graphs();
    v.check_bool(
        "rootpulse_has_five_required_graphs",
        graphs.len() == 5,
        &format!(
            "EmergentSystem::RootPulse has {} required graphs: {graphs:?}",
            graphs.len()
        ),
    );
}

fn provenance_trio_discovery(v: &mut ValidationResult) {
    for &name in &["rhizocrypt", "loamspine", "sweetgrass"] {
        let disc = discover_primal(name);
        v.check_bool(
            &format!("discover_{name}"),
            disc.primal == name,
            &format!("discover {name} (provenance trio)"),
        );
        v.check_or_skip(
            &format!("probe_{name}"),
            disc.socket.as_ref(),
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
}

fn commit_phase_health(
    v: &mut ValidationResult,
    trio_all_healthy: bool,
    trio_health: &[(&'static str, bool)],
) {
    v.check_or_skip(
        "phase_1_health",
        trio_all_healthy.then_some(()),
        "trio not available for health phase",
        |(), v| {
            for (domain, healthy) in trio_health {
                v.check_bool(
                    &format!("trio_health_{domain}"),
                    *healthy,
                    &format!("{domain} domain health via capability.call"),
                );
            }
        },
    );
}

fn commit_phase_dehydrate(v: &mut ValidationResult, trio_all_healthy: bool) {
    v.check_or_skip(
        "phase_2_dehydrate",
        trio_all_healthy.then_some(()),
        "trio not available for dehydrate phase",
        |(), v| {
            let session = begin_experiment_session("exp020-rootpulse-commit");
            let session_ok = session.status != ProvenanceStatus::Unavailable;
            v.check_bool(
                "dag_create_session",
                session_ok,
                "rhizoCrypt session created",
            );

            if session_ok {
                let step = record_experiment_step(
                    &session.id,
                    &serde_json::json!({
                        "action": "validate",
                        "phase": "rootpulse_commit",
                        "result": "in_progress",
                    }),
                );
                v.check_bool(
                    "dag_append_event",
                    step.status != ProvenanceStatus::Unavailable,
                    "event appended to DAG",
                );
            }
        },
    );
}

fn commit_phase_sign(v: &mut ValidationResult, neural_api_live: bool) {
    v.check_or_skip(
        "phase_3_sign",
        neural_api_live.then_some(()),
        "Neural API not available for sign phase",
        |(), v| {
            let sign_result = primalspring::ipc::discover::capability_call(
                "crypto",
                "sign",
                &serde_json::json!({
                    "data": "exp020-rootpulse-commit-validation",
                    "algorithm": "ed25519",
                }),
            );
            v.check_bool(
                "crypto_sign",
                sign_result.is_some(),
                "beardog crypto.sign via capability.call",
            );
        },
    );
}

fn commit_phase_store_commit(v: &mut ValidationResult, trio_all_healthy: bool) {
    v.check_or_skip(
        "phase_4_5_store_commit",
        trio_all_healthy.then_some(()),
        "trio not available for store/commit phase",
        |(), v| {
            let session = begin_experiment_session("exp020-commit-pipeline");
            if session.status == ProvenanceStatus::Unavailable {
                v.check_skip("pipeline_complete", "session creation failed");
                return;
            }

            let pipeline = complete_experiment(&session.id);
            v.check_bool(
                "pipeline_dehydrate",
                !pipeline.merkle_root.is_empty(),
                &format!("merkle_root: {}", pipeline.merkle_root),
            );
            v.check_bool(
                "pipeline_commit",
                !pipeline.commit_id.is_empty(),
                &format!("commit_id: {}", pipeline.commit_id),
            );
        },
    );
}

fn commit_phase_attribute(v: &mut ValidationResult, trio_all_healthy: bool) {
    v.check_or_skip(
        "phase_6_attribute",
        trio_all_healthy.then_some(()),
        "trio not available for attribute phase",
        |(), v| {
            let session = begin_experiment_session("exp020-attribution");
            if session.status == ProvenanceStatus::Unavailable {
                v.check_skip("braid_create", "session creation failed");
                return;
            }

            let pipeline = complete_experiment(&session.id);
            v.check_bool(
                "braid_create",
                pipeline.status == ProvenanceStatus::Complete,
                &format!("braid_id: {}", pipeline.braid_id),
            );
        },
    );
}

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp020 — RootPulse Full Commit");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp020: RootPulse Full 6-Phase Commit");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    rootpulse_required_graphs(&mut v);
    provenance_trio_discovery(&mut v);

    let neural_api_live = neural_api_healthy();
    let trio_health = provenance::trio_health();
    let trio_all_healthy = neural_api_live && trio_health.iter().all(|(_domain, ok)| *ok);

    commit_phase_health(&mut v, trio_all_healthy, &trio_health);
    commit_phase_dehydrate(&mut v, trio_all_healthy);
    commit_phase_sign(&mut v, neural_api_live);
    commit_phase_store_commit(&mut v, trio_all_healthy);
    commit_phase_attribute(&mut v, trio_all_healthy);

    v.finish();
    std::process::exit(v.exit_code());
}
