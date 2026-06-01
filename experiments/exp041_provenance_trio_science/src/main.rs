// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp041: Provenance Trio Science — rhizoCrypt + loamSpine + sweetGrass.

use std::time::Instant;

use primalspring::composition::CompositionContext;
use primalspring::ipc::discover::extract_capability_names;
use primalspring::ipc::provenance::ProvenanceStatus;
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;
use primalspring_trio_ops::{
    begin_experiment_session, complete_experiment, record_experiment_step, trio_health,
};

fn sweetgrass_cap(ctx: &CompositionContext) -> Option<&'static str> {
    if ctx.has_capability("commit") {
        Some("commit")
    } else if ctx.has_capability("attribution") {
        Some("attribution")
    } else {
        None
    }
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let present = [
        ctx.has_capability("dag"),
        ctx.has_capability("ledger"),
        sweetgrass_cap(ctx).is_some(),
    ]
    .iter()
    .filter(|&&x| x)
    .count();
    v.check_count("trio_capabilities_present", present, 3);
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let trio_ok =
        ctx.has_capability("dag") && ctx.has_capability("ledger") && sweetgrass_cap(ctx).is_some();

    if !trio_ok {
        v.check_skip(
            "trio_all_discoverable",
            &format!(
                "need dag, ledger, and commit or attribution — have {}: {}",
                ctx.available_capabilities().len(),
                ctx.available_capabilities().join(", ")
            ),
        );
        return;
    }

    v.check_bool(
        "trio_all_discoverable",
        true,
        "dag, ledger, and sweetGrass-cap aliases present in context",
    );

    let sg = sweetgrass_cap(ctx).unwrap_or("commit");
    let probes: [(&str, &str); 3] = [
        ("dag", primal_names::RHIZOCRYPT),
        ("ledger", primal_names::LOAMSPINE),
        (sg, primal_names::SWEETGRASS),
    ];

    for (cap, display_name) in probes {
        let start = Instant::now();
        let health_ok = ctx.health_check(cap).unwrap_or(false);
        let latency_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);
        let n_caps = ctx
            .call(cap, "capabilities.list", serde_json::json!({}))
            .ok()
            .map_or(0, |j| extract_capability_names(Some(j)).len());

        v.check_bool(
            &format!("health_{display_name}"),
            health_ok,
            &format!("{display_name} health check"),
        );
        v.check_latency(
            &format!("latency_{display_name}"),
            latency_us,
            tolerances::HEALTH_CHECK_MAX_US,
        );
        v.check_minimum(&format!("caps_{display_name}"), n_caps, 1);
    }
}

fn phase_orchestration_and_trio_api(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
) -> bool {
    let orchestration_ok = if ctx.has_capability("orchestration") {
        ctx.call("orchestration", "health.liveness", serde_json::json!({}))
            .is_ok()
    } else {
        false
    };

    v.check_or_skip(
        "orchestration",
        orchestration_ok.then_some(()),
        "orchestration capability not healthy",
        |(), v| {
            v.check_bool(
                "orchestration_reachable",
                true,
                "orchestration health.liveness ok",
            );
        },
    );

    let trio_api = trio_health();
    let trio_caps_healthy = orchestration_ok && trio_api.iter().all(|(_d, ok)| *ok);

    v.check_or_skip(
        "trio_capability_health",
        trio_caps_healthy.then_some(()),
        "trio capabilities not all healthy via Neural API",
        |(), v| {
            for (domain, healthy) in &trio_api {
                v.check_bool(
                    &format!("cap_health_{domain}"),
                    *healthy,
                    &format!("{domain} capability domain healthy"),
                );
            }
        },
    );

    trio_caps_healthy
}

fn validate_e2e_chain(v: &mut ValidationResult) {
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
        .with_provenance("exp041_provenance_trio_science", "2026-05-09")
        .run(
            "primalSpring Exp041: Provenance Trio Science — rhizoCrypt + loamSpine + sweetGrass",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Discovery");
                phase_discovery(v, &ctx);

                v.section("Phase 2: Health");
                phase_health(v, &mut ctx);

                v.section("Phase 3: Trio Capability Health");
                let trio_caps_healthy = phase_orchestration_and_trio_api(v, &mut ctx);

                v.section("Phase 4: E2E Provenance Chain");
                v.check_or_skip(
                    "provenance_chain_e2e",
                    trio_caps_healthy.then_some(()),
                    "needs live trio primals + Neural API for chain validation",
                    |(), v| validate_e2e_chain(v),
                );
            },
        );
}
