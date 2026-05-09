// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp042: `FieldMouse` Ingestion — edge frames → NestGate → sweetGrass (capability discovery).

use std::time::Instant;

use primalspring::composition::CompositionContext;
use primalspring::ipc::discover::extract_capability_names;
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    v.check_bool(
        "ingest_context_capabilities",
        !caps.is_empty(),
        &format!("capabilities: {}", caps.join(", ")),
    );
    v.check_bool(
        "nestgate_storage_capability",
        ctx.has_capability("storage"),
        "NestGate maps to storage capability",
    );
    v.check_bool(
        "sweetgrass_commit_or_attribution",
        ctx.has_capability("commit") || ctx.has_capability("attribution"),
        "sweetGrass maps to commit or attribution",
    );
    v.check_skip(
        "fieldmouse_deployment_class",
        "fieldMouse is an edge deployment class; not a single ALL_CAPS domain",
    );
}

fn phase_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let nest_ok = ctx.has_capability("storage");
    let grass_ok = ctx.has_capability("commit") || ctx.has_capability("attribution");
    let pipeline_online = nest_ok && grass_ok;

    if !pipeline_online {
        v.check_skip(
            "ingest_primals_discoverable",
            &format!(
                "need storage + (commit or attribution) for NestGate + sweetGrass — {}",
                ctx.available_capabilities().join(", ")
            ),
        );
        return;
    }

    v.check_bool(
        "ingest_primals_discoverable",
        true,
        "storage and sweetGrass capability aliases in context",
    );

    let mut probes: Vec<(&str, &str)> = vec![("storage", primal_names::NESTGATE)];
    if ctx.has_capability("commit") {
        probes.push(("commit", primal_names::SWEETGRASS));
    } else if ctx.has_capability("attribution") {
        probes.push(("attribution", primal_names::SWEETGRASS));
    }

    for (cap, label) in probes {
        let start = Instant::now();
        let health_ok = ctx.health_check(cap).unwrap_or(false);
        let latency_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);
        let n_caps = ctx
            .call(cap, "capabilities.list", serde_json::json!({}))
            .ok()
            .map_or(0, |j| extract_capability_names(Some(j)).len());
        v.check_bool(
            &format!("health_{label}"),
            health_ok,
            &format!("{label} health check"),
        );
        v.check_latency(
            &format!("latency_{label}"),
            latency_us,
            tolerances::HEALTH_CHECK_MAX_US,
        );
        v.check_minimum(&format!("caps_{label}"), n_caps, 1);
    }
}

fn phase_pipeline_skips(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let orch_ok = ctx.has_capability("orchestration")
        && ctx
            .call("orchestration", "health.liveness", serde_json::json!({}))
            .is_ok();

    if orch_ok {
        v.check_bool(
            "orchestration_reachable",
            true,
            "orchestration health.liveness ok",
        );
        v.check_skip(
            "fieldmouse_ingest_e2e",
            "end-to-end ingestion needs fieldMouse frames + NestGate + sweetGrass live",
        );
    } else {
        v.check_skip(
            "orchestration_reachable",
            "orchestration capability not available for substrate health",
        );
        v.check_skip(
            "fieldmouse_ingest_e2e",
            "needs live ingest primals + orchestration for pipeline validation",
        );
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp042 — FieldMouse Ingestion")
        .with_provenance("exp042_fieldmouse_ingestion", "2026-05-09")
        .run(
            "primalSpring Exp042: FieldMouse Ingestion — fieldMouse → NestGate → sweetGrass",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                v.section("Phase 1: Discovery");
                phase_discovery(v, &ctx);

                v.section("Phase 2: Health");
                phase_health(v, &mut ctx);

                v.section("Phase 3: Pipeline (skips)");
                phase_pipeline_skips(v, &mut ctx);
            },
        );
}
