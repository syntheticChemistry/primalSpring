// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp021: RootPulse branch and merge via rhizoCrypt DAG operations.

use primalspring::composition::CompositionContext;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::provenance::ProvenanceStatus;
use primalspring::validation::ValidationResult;
use primalspring_trio_ops::{
    begin_experiment_session, record_experiment_step, rootpulse_branch, rootpulse_merge,
};

fn phase_structural(v: &mut ValidationResult) {
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

fn phase_branch_merge(v: &mut ValidationResult, trio_ready: bool) {
    v.check_or_skip(
        "branch_merge_operations",
        trio_ready.then_some(()),
        "branch/merge operations need live orchestration and trio health",
        |(), v| {
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

            let branch_step = record_experiment_step(
                &branch.id,
                &serde_json::json!({ "action": "branch_work", "phase": "feature" }),
            );
            v.check_bool(
                "branch_record",
                branch_step.status != ProvenanceStatus::Unavailable,
                &format!("branch step recorded: {}", branch_step.id),
            );

            let merge = rootpulse_merge(&branch.id, &main_session.id);
            v.check_bool(
                "merge_execute",
                merge.status != ProvenanceStatus::Unavailable,
                &format!("merge completed: {}", merge.id),
            );
        },
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp021 — RootPulse Branch Merge")
        .with_provenance("exp021_rootpulse_branch_merge", "2026-05-09")
        .run("primalSpring Exp021: Branch + Merge", |v| {
            v.section("Phase 1: Structural");
            phase_structural(v);

            v.section("Phase 2: Discovery and Health");
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_discovery(v, &ctx);
            let trio_ready = phase_health(v, &mut ctx);

            v.section("Phase 3: Branch/Merge Operations");
            phase_branch_merge(v, trio_ready);
        });
}
