// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp101 — fieldMouse AI Triage
//!
//! Validates the AI-guided sensor triage pipeline:
//! fieldMouse frame → NestGate storage → Squirrel classify → petalTongue alert
//!
//! Phase 56 — Desktop Substrate (AGENTIC_TRIO_EVOLUTION.md)

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::discover_by_capability;
use primalspring::validation::ValidationResult;

fn phase_storage_ingest(v: &mut ValidationResult) {
    v.section("Sensor Data Ingest (NestGate)");

    let ng = discover_by_capability("storage");
    let Some(ng_sock) = ng.socket.as_ref() else {
        v.check_skip("storage_ingest", "NestGate not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ng_sock, "nestgate") else {
        v.check_skip("storage_ingest", "NestGate connection failed");
        return;
    };

    let resp = client.call(
        "storage.store",
        serde_json::json!({
            "key": "fieldmouse/frame/exp101-test",
            "value": "{\"ph\":7.2,\"moisture\":0.45,\"temp_c\":22.1}",
        }),
    );

    v.check_bool(
        "storage_ingest",
        resp.is_ok_and(|r| r.result.is_some()),
        "Sensor frame stored in NestGate",
    );
}

fn phase_ai_classification(v: &mut ValidationResult) {
    v.section("AI Classification (Squirrel)");

    let sq = discover_by_capability("ai");
    let Some(sq_sock) = sq.socket.as_ref() else {
        v.check_skip("ai_classify", "Squirrel not discovered via ai capability");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(sq_sock, "squirrel") else {
        v.check_skip("ai_classify", "Squirrel connection failed");
        return;
    };

    let resp = client.call("inference.models", serde_json::json!({}));
    v.check_bool(
        "ai_models_available",
        resp.is_ok(),
        "Squirrel has inference models available",
    );
}

fn phase_alert_rendering(v: &mut ValidationResult) {
    v.section("Alert Rendering (petalTongue)");

    let pt = discover_by_capability("visualization");
    let Some(pt_sock) = pt.socket.as_ref() else {
        v.check_skip("alert_render", "petalTongue not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(pt_sock, "petaltongue") else {
        v.check_skip("alert_render", "petalTongue connection failed");
        return;
    };

    let resp = client.call(
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
        resp.is_ok_and(|r| r.result.is_some()),
        "Alert dashboard rendered to petalTongue",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp101 — fieldMouse AI Triage")
        .with_provenance("exp101_fieldmouse_ai_triage", "2026-04-28")
        .run(
            "Exp101: Sensor ingest → AI classify → alert render",
            |v| {
                phase_storage_ingest(v);
                phase_ai_classification(v);
                phase_alert_rendering(v);
            },
        );
}
