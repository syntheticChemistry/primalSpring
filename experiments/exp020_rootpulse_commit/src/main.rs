// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp020: RootPulse full commit — validates the commit pipeline against live composition.

use primalspring::composition::CompositionContext;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::provenance::ProvenanceStatus;
use primalspring::validation::ValidationResult;
use primalspring_trio_ops::{
    begin_experiment_session, complete_experiment, record_experiment_step,
};

fn phase_structural(v: &mut ValidationResult) {
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

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in ["dag", "ledger", "attribution"] {
        v.check_bool(
            &format!("discover_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved (provenance trio)"),
        );
    }
}

fn phase_trio_health(v: &mut ValidationResult, ctx: &mut CompositionContext) -> bool {
    let orch = ctx.has_capability("orchestration");
    if orch {
        v.check_bool(
            "orchestration_routable",
            true,
            "orchestration capability resolved",
        );
    } else {
        v.check_skip(
            "orchestration_routable",
            "orchestration capability not resolved",
        );
    }

    let mut all_ok = orch;
    for cap in ["dag", "ledger", "attribution"] {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("trio_health_{cap}"),
                &format!("{cap} not resolved"),
            );
            all_ok = false;
            continue;
        }
        match ctx.call(cap, "health.liveness", serde_json::json!({})) {
            Ok(_) => {
                v.check_bool(
                    &format!("trio_health_{cap}"),
                    true,
                    &format!("{cap} health.liveness"),
                );
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("trio_health_{cap}"), &format!("{cap}: {e}"));
                all_ok = false;
            }
            Err(e) => {
                v.check_bool(&format!("trio_health_{cap}"), false, &format!("error: {e}"));
                all_ok = false;
            }
        }
    }
    all_ok
}

fn commit_phase_dehydrate(v: &mut ValidationResult, trio_ready: bool) {
    v.check_or_skip(
        "phase_2_dehydrate",
        trio_ready.then_some(()),
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

fn commit_phase_sign(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.check_or_skip(
        "phase_3_sign",
        ctx.has_capability("orchestration").then_some(()),
        "orchestration not available for sign phase",
        |(), v| match ctx.call(
            "security",
            "crypto.sign",
            serde_json::json!({
                "data": "exp020-rootpulse-commit-validation",
                "algorithm": "ed25519",
            }),
        ) {
            Ok(_) => v.check_bool("crypto_sign", true, "security crypto.sign"),
            Err(e) if e.is_connection_error() => v.check_skip("crypto_sign", &format!("{e}")),
            Err(e) => v.check_bool("crypto_sign", false, &format!("error: {e}")),
        },
    );
}

fn commit_phase_store_commit(v: &mut ValidationResult, trio_ready: bool) {
    v.check_or_skip(
        "phase_4_5_store_commit",
        trio_ready.then_some(()),
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

fn phase_attribution(v: &mut ValidationResult, trio_ready: bool) {
    v.check_or_skip(
        "phase_6_attribute",
        trio_ready.then_some(()),
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
    ValidationResult::new("primalSpring Exp020 — RootPulse Full Commit")
        .with_provenance("exp020_rootpulse_commit", "2026-05-09")
        .run("primalSpring Exp020: RootPulse Full 6-Phase Commit", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Discovery");
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_discovery(v, &ctx);

            v.section("Phase 3: Health");
            let trio_ready = phase_trio_health(v, &mut ctx);

            v.section("Phase 4: Commit Pipeline");
            commit_phase_dehydrate(v, trio_ready);
            commit_phase_sign(v, &mut ctx);
            commit_phase_store_commit(v, trio_ready);

            v.section("Phase 5: Attribution");
            phase_attribution(v, trio_ready);
        });
}
