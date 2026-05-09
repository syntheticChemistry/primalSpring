// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp022: RootPulse Merkle diff and federation via rhizoCrypt.

use primalspring::composition::CompositionContext;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::provenance::ProvenanceStatus;
use primalspring::validation::ValidationResult;
use primalspring_trio_ops::{
    begin_experiment_session, record_experiment_step, rootpulse_diff, rootpulse_federate,
};

fn phase_structural(v: &mut ValidationResult) {
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

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) -> bool {
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

fn phase_diff_federate(v: &mut ValidationResult, trio_ready: bool) {
    v.check_or_skip(
        "diff_federate_operations",
        trio_ready.then_some(()),
        "diff/federate operations need live orchestration and trio health",
        |(), v| {
            let session_a = begin_experiment_session("exp022-diff-source");
            let session_b = begin_experiment_session("exp022-diff-target");

            if session_a.status == ProvenanceStatus::Unavailable
                || session_b.status == ProvenanceStatus::Unavailable
            {
                v.check_skip("diff_execute", "session creation failed");
                v.check_skip("federate_execute", "session creation failed");
                return;
            }

            let _step_a = record_experiment_step(
                &session_a.id,
                &serde_json::json!({ "action": "work_a", "data": "source_change" }),
            );
            let _step_b = record_experiment_step(
                &session_b.id,
                &serde_json::json!({ "action": "work_b", "data": "target_change" }),
            );

            let diff = rootpulse_diff(&session_a.id, &session_b.id);
            v.check_bool(
                "diff_execute",
                diff.status != ProvenanceStatus::Unavailable,
                &format!("diff computed: {}", diff.id),
            );

            let federate = rootpulse_federate(&session_a.id, "local://exp022-test");
            v.check_bool(
                "federate_execute",
                federate.status != ProvenanceStatus::Unavailable,
                &format!("federation: {}", federate.id),
            );
        },
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp022 — RootPulse Diff Federate")
        .with_provenance("exp022_rootpulse_diff_federate", "2026-05-09")
        .run("primalSpring Exp022: Merkle Diff + Federation", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Discovery and Health");
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_discovery(v, &ctx);
            let trio_ready = phase_health(v, &mut ctx);

            v.section("Phase 3: Diff/Federate Operations");
            phase_diff_federate(v, trio_ready);
        });
}
