// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp088 — Storytelling Composition (live socket discovery)
//!
//! Validates the ludoSpring + esotericWebb + Tower + biomeOS storytelling
//! composition via standard socket discovery and biomeOS Neural API routing.
//!
//! Discovers all primals via `discover_primal` / `discover_by_capability`
//! (UDS sockets), not hardcoded TCP ports.

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{
    discover_by_capability, discover_primal, neural_api_healthy, neural_bridge,
};
use primalspring::ipc::neural_bridge::NeuralBridge;
use primalspring::ipc::protocol::JsonRpcResponse;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn phase_tower(v: &mut ValidationResult) {
    v.section("Tower Atomic");

    let bd = discover_primal(primal_names::BEARDOG);
    let Some(bd_sock) = bd.socket.as_ref() else {
        v.check_skip("beardog", "BearDog not discovered");
        v.check_skip("songbird", "requires BearDog");
        return;
    };
    let bd_healthy = PrimalClient::connect(bd_sock, primal_names::BEARDOG)
        .is_ok_and(|mut c| c.health_check().unwrap_or(false));
    v.check_bool("beardog", bd_healthy, "BearDog security healthy");

    let sb = discover_primal(primal_names::SONGBIRD);
    let sb_healthy = sb.socket.as_ref().is_some_and(|sock| {
        PrimalClient::connect(sock, primal_names::SONGBIRD)
            .is_ok_and(|mut c| c.health_check().unwrap_or(false))
    });
    v.check_bool("songbird", sb_healthy, "Songbird discovery healthy");
}

fn phase_biomeos(v: &mut ValidationResult) -> Option<NeuralBridge> {
    v.section("biomeOS Substrate");
    let healthy = neural_api_healthy();
    v.check_bool("neural_api", healthy, "biomeOS Neural API reachable");
    if !healthy {
        return None;
    }
    neural_bridge()
}

fn phase_ludospring(v: &mut ValidationResult, bridge: Option<&NeuralBridge>) {
    v.section("ludoSpring Game Science");

    let disc = discover_primal("ludospring");
    let Some(sock) = disc.socket.as_ref() else {
        v.check_skip("ludospring_health", "ludoSpring not discovered");
        v.check_skip("game.evaluate_flow", "requires ludoSpring");
        v.check_skip("game.fitts_cost", "requires ludoSpring");
        v.check_skip("game.generate_noise", "requires ludoSpring");
        return;
    };

    let healthy = PrimalClient::connect(sock, "ludospring")
        .is_ok_and(|mut c| c.health_check().unwrap_or(false));
    v.check_bool("ludospring_health", healthy, "ludoSpring healthy");

    let flow = PrimalClient::connect(sock, "ludospring").and_then(|mut c| {
        c.call(
            "game.evaluate_flow",
            serde_json::json!({"skill": 0.7, "challenge": 0.6}),
        )
    });
    v.check_bool(
        "game.evaluate_flow",
        flow.as_ref()
            .is_ok_and(|r| r.result.as_ref().and_then(|v| v.get("state")).is_some()),
        "flow evaluation returns state",
    );

    let fitts = PrimalClient::connect(sock, "ludospring").and_then(|mut c| {
        c.call(
            "game.fitts_cost",
            serde_json::json!({"distance": 100.0, "target_width": 20.0}),
        )
    });
    v.check_bool(
        "game.fitts_cost",
        fitts.as_ref().is_ok_and(|r| {
            r.result
                .as_ref()
                .and_then(|v| v.get("movement_time_ms"))
                .is_some()
        }),
        "Fitts cost returns movement_time_ms",
    );

    let noise = PrimalClient::connect(sock, "ludospring").and_then(|mut c| {
        c.call(
            "game.generate_noise",
            serde_json::json!({"x": 1.0, "y": 2.0}),
        )
    });
    v.check_bool(
        "game.generate_noise",
        noise.as_ref().is_ok_and(JsonRpcResponse::is_success),
        "noise generation returns value",
    );

    if let Some(b) = bridge {
        let routed = b.capability_call(
            "game",
            "evaluate_flow",
            &serde_json::json!({"skill": 0.5, "challenge": 0.5}),
        );
        v.check_bool(
            "biomeos_game_routing",
            routed.is_ok(),
            "biomeOS routes game.evaluate_flow → ludoSpring",
        );
    } else {
        v.check_skip("biomeos_game_routing", "biomeOS not available");
    }
}

fn phase_esotericwebb(v: &mut ValidationResult) {
    v.section("esotericWebb Narrative Product");

    let disc = discover_primal("esotericwebb");
    let Some(sock) = disc.socket.as_ref() else {
        v.check_skip("webb_liveness", "esotericWebb not discovered");
        v.check_skip("session.state", "requires esotericWebb");
        v.check_skip("webb.scene.current", "requires esotericWebb");
        v.check_skip("capabilities.list", "requires esotericWebb");
        return;
    };

    let liveness = PrimalClient::connect(sock, "esotericwebb")
        .and_then(|mut c| c.call("webb.liveness", serde_json::Value::Null));
    v.check_bool(
        "webb_liveness",
        liveness.as_ref().is_ok_and(JsonRpcResponse::is_success),
        "esotericWebb liveness responds",
    );

    let state = PrimalClient::connect(sock, "esotericwebb")
        .and_then(|mut c| c.call("session.state", serde_json::Value::Null));
    let has_session = state.as_ref().is_ok_and(|r| {
        r.result
            .as_ref()
            .and_then(|v| v.get("current_node"))
            .is_some()
    });
    v.check_bool(
        "session.state",
        has_session,
        "active session with current_node",
    );

    let scene = PrimalClient::connect(sock, "esotericwebb")
        .and_then(|mut c| c.call("webb.scene.current", serde_json::Value::Null));
    let has_scene = scene
        .as_ref()
        .is_ok_and(|r| r.result.as_ref().and_then(|v| v.get("scene")).is_some());
    v.check_bool("webb.scene.current", has_scene, "current scene available");

    let caps = PrimalClient::connect(sock, "esotericwebb")
        .and_then(|mut c| c.call("capabilities.list", serde_json::json!({})));
    let cap_count = caps.as_ref().ok().and_then(|r| {
        r.result
            .as_ref()
            .and_then(|v| v.get("capabilities"))
            .and_then(|c| c.as_array())
            .map(Vec::len)
    });
    v.check_minimum("webb_capabilities", cap_count.unwrap_or(0), 15);
}

fn phase_composition(v: &mut ValidationResult) {
    v.section("Full Composition Discovery");

    let game = discover_by_capability("game");
    v.check_bool(
        "discover_game",
        game.socket.is_some(),
        &format!(
            "game → {}",
            game.resolved_primal.as_deref().unwrap_or("unknown")
        ),
    );

    let narrative = discover_primal("esotericwebb");
    v.check_bool(
        "discover_narrative",
        narrative.socket.is_some(),
        "esotericWebb discoverable via socket path",
    );

    let security = discover_by_capability("security");
    v.check_bool(
        "discover_security",
        security.socket.is_some(),
        &format!(
            "security → {}",
            security.resolved_primal.as_deref().unwrap_or("unknown")
        ),
    );

    let discovery = discover_by_capability("discovery");
    v.check_bool(
        "discover_discovery",
        discovery.socket.is_some(),
        &format!(
            "discovery → {}",
            discovery.resolved_primal.as_deref().unwrap_or("unknown")
        ),
    );
}

fn main() {
    ValidationResult::new("Storytelling Composition")
        .with_provenance("exp088_storytelling_composition", "2026-03-30")
        .run(
            "ecoPrimals Storytelling: Tower + ludoSpring + esotericWebb via biomeOS",
            |v| {
                phase_tower(v);
                let bridge = phase_biomeos(v);
                phase_ludospring(v, bridge.as_ref());
                phase_esotericwebb(v);
                phase_composition(v);
            },
        );
}
