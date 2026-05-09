// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp099 — Agentic Loop Substrate
//!
//! Validates the full three-way feedback loop:
//! petalTongue → biomeOS → Squirrel → biomeOS → springs → petalTongue
//!
//! Phase 56 — Desktop Substrate (AGENTIC_TRIO_EVOLUTION.md)

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_by_capability, discover_primal};
use primalspring::validation::ValidationResult;

fn phase_trio_discovery(v: &mut ValidationResult) {
    v.section("Agentic Trio Discovery");

    let biomeos = discover_by_capability("orchestration");
    v.check_bool(
        "biomeos_discovered",
        biomeos.socket.is_some(),
        "biomeOS Neural API discovered via orchestration capability",
    );

    let squirrel = discover_primal("squirrel");
    v.check_bool(
        "squirrel_discovered",
        squirrel.socket.is_some(),
        "Squirrel discovered",
    );

    let petaltongue = discover_by_capability("visualization");
    v.check_bool(
        "petaltongue_discovered",
        petaltongue.socket.is_some(),
        "petalTongue discovered via visualization capability",
    );
}

fn phase_sensor_to_intent(v: &mut ValidationResult) {
    v.section("Sensor to Intent (petalTongue afferent)");

    let pt = discover_by_capability("visualization");
    let Some(pt_sock) = pt.socket.as_ref() else {
        v.check_skip("sensor_stream", "petalTongue not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(pt_sock, "petaltongue") else {
        v.check_skip("sensor_stream", "petalTongue connection failed");
        return;
    };

    let resp = client.call("proprioception.get", serde_json::json!({}));
    v.check_bool(
        "proprioception",
        resp.is_ok_and(|r| r.result.is_some()),
        "petalTongue proprioception.get responds",
    );
}

fn phase_intent_routing(v: &mut ValidationResult) {
    v.section("Intent Routing (biomeOS to Squirrel)");

    let biomeos = discover_by_capability("orchestration");
    let Some(bio_sock) = biomeos.socket.as_ref() else {
        v.check_skip("ai_routing", "biomeOS not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(bio_sock, "biomeos") else {
        v.check_skip("ai_routing", "biomeOS connection failed");
        return;
    };

    let resp = client.call(
        "capability.call",
        serde_json::json!({
            "capability": "ai",
            "operation": "models",
            "args": {}
        }),
    );

    match resp {
        Ok(r) => {
            v.check_bool(
                "ai_routing",
                r.result.is_some(),
                "biomeOS routes ai.models to Squirrel via capability.call",
            );
        }
        Err(e) => {
            v.check_skip(
                "ai_routing",
                &format!("capability.call to ai domain failed: {e}"),
            );
        }
    }
}

fn phase_render_feedback(v: &mut ValidationResult) {
    v.section("Render Feedback (Squirrel to petalTongue)");

    let biomeos = discover_by_capability("orchestration");
    let Some(bio_sock) = biomeos.socket.as_ref() else {
        v.check_skip("render_feedback", "biomeOS not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(bio_sock, "biomeos") else {
        v.check_skip("render_feedback", "biomeOS connection failed");
        return;
    };

    let resp = client.call(
        "capability.call",
        serde_json::json!({
            "capability": "visualization",
            "operation": "render.dashboard",
            "args": {
                "session": "exp099-test",
                "data": {"title": "Agentic Loop Test", "status": "validating"}
            }
        }),
    );

    v.check_bool(
        "render_feedback",
        resp.is_ok_and(|r| r.result.is_some()),
        "biomeOS routes visualization.render.dashboard to petalTongue",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp099 — Agentic Loop Substrate")
        .with_provenance("exp099_agentic_loop_substrate", "2026-04-28")
        .run(
            "Exp099: Full three-way agentic loop on Desktop NUCLEUS",
            |v| {
                phase_trio_discovery(v);
                phase_sensor_to_intent(v);
                phase_intent_routing(v);
                phase_render_feedback(v);
            },
        );
}
