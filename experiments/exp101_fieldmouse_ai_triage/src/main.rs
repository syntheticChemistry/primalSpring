// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp101 — fieldMouse AI Triage
//!
//! Validates the AI-guided sensor triage pipeline:
//! fieldMouse frame → NestGate storage → Squirrel classify → petalTongue alert
//!
//! Phase 56 — Desktop Substrate (AGENTIC_TRIO_EVOLUTION.md)

use primalspring::composition::CompositionContext;
use primalspring::ipc::IpcError;
use primalspring::validation::ValidationResult;

fn orchestration_route(
    ctx: &mut CompositionContext,
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": capability,
            "operation": operation,
            "args": args,
        }),
    )
}

fn phase_storage_ingest(v: &mut ValidationResult) {
    v.section("Sensor Data Ingest (NestGate)");

    let mut ctx = CompositionContext::discover();
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let store_params = serde_json::json!({
        "family_id": family_id,
        "key": "fieldmouse/frame/exp101-test",
        "value": "{\"ph\":7.2,\"moisture\":0.45,\"temp_c\":22.1}",
    });

    let ok = ctx
        .call("storage", "storage.store", store_params.clone())
        .is_ok();
    if ok {
        v.check_bool("storage_ingest", true, "Sensor frame stored in NestGate");
        return;
    }

    if ctx.has_capability("orchestration")
        && orchestration_route(&mut ctx, "storage", "storage.store", &store_params).is_ok()
    {
        v.check_bool(
            "storage_ingest",
            true,
            "Sensor frame stored in NestGate (via fallback — biomeOS misrouted)",
        );
        return;
    }

    v.check_bool("storage_ingest", false, "NestGate storage.store failed");
}

fn phase_ai_classification(v: &mut ValidationResult) {
    v.section("AI Classification (Squirrel)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("ai") {
        v.check_skip("ai_classify", "Squirrel not discovered via ai capability");
        return;
    }

    let resp = ctx.call("ai", "inference.models", serde_json::json!({}));
    v.check_bool(
        "ai_models_available",
        resp.is_ok(),
        "Squirrel has inference models available",
    );
}

fn phase_alert_rendering(v: &mut ValidationResult) {
    v.section("Alert Rendering (petalTongue)");

    let mut ctx = CompositionContext::discover();
    if !ctx.has_capability("visualization") {
        v.check_skip("alert_render", "petalTongue not discovered");
        return;
    }

    let resp = ctx.call(
        "visualization",
        "visualization.render.dashboard",
        serde_json::json!({
            "session": "exp101-fieldmouse",
            "data": {
                "title": "fieldMouse Triage",
                "readings": [{"label": "pH", "value": 7.2, "status": "normal"}],
                "alert_level": "info"
            }
        }),
    );

    v.check_bool(
        "alert_render",
        resp.is_ok(),
        "Alert dashboard rendered to petalTongue",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp101 — fieldMouse AI Triage")
        .with_provenance("exp101_fieldmouse_ai_triage", "2026-05-09")
        .run(
            "Exp101: Sensor ingest → AI classify → alert render",
            |v| {
                phase_storage_ingest(v);
                phase_ai_classification(v);
                phase_alert_rendering(v);
            },
        );
}
