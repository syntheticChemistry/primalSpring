// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp076: Cross-Gate Neural Routing

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::ipc::methods;
use primalspring::ipc::tcp::tcp_rpc;
use primalspring::validation::ValidationResult;

fn pixel_beardog_host_port() -> (String, u16) {
    let s = std::env::var("PIXEL_BEARDOG_TCP").unwrap_or_else(|_| "localhost:19100".to_owned());
    match s.rsplit_once(':') {
        Some((host, port_str)) => (host.to_owned(), port_str.parse().unwrap_or(19100)),
        None => (s, 19100),
    }
}

fn pixel_songbird_port() -> u16 {
    std::env::var("PIXEL_SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(19200)
}

fn local_songbird_port() -> u16 {
    std::env::var("SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(primalspring::tolerances::TCP_FALLBACK_SONGBIRD_PORT)
}

fn phase_pixel_tower(v: &mut ValidationResult) {
    v.section("Phase 1: Pixel tower TCP");
    let (bd_host, bd_port) = pixel_beardog_host_port();
    let songbird_port = pixel_songbird_port();

    let beardog_resp = tcp_rpc(
        &bd_host,
        bd_port,
        methods::health::CHECK,
        &serde_json::json!({}),
    );
    let beardog_ok = beardog_resp
        .as_ref()
        .ok()
        .and_then(|(r, _)| r.get("status"))
        .and_then(|s| s.as_str())
        .is_some_and(|s| s == "healthy");
    v.check_bool(
        "pixel_beardog_health",
        beardog_ok,
        &format!("Pixel BearDog at {bd_host}:{bd_port}"),
    );

    let songbird_ok = tcp_rpc(
        "localhost",
        songbird_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .is_ok();
    v.check_bool(
        "pixel_songbird_health",
        songbird_ok,
        &format!("Pixel Songbird at localhost:{songbird_port}"),
    );
}

fn phase_cross_gate_crypto(v: &mut ValidationResult) {
    v.section("Phase 2: Cross-gate crypto");
    let (bd_host, bd_port) = pixel_beardog_host_port();
    let resp = tcp_rpc(
        &bd_host,
        bd_port,
        "crypto.generate_keypair",
        &serde_json::json!({}),
    );
    let has_key = resp
        .as_ref()
        .ok()
        .and_then(|(r, _)| r.get("public_key"))
        .is_some();
    v.check_bool(
        "cross_gate_crypto",
        has_key,
        "Pixel BearDog crypto.generate_keypair via TCP",
    );
}

fn phase_cross_gate_beacon_exchange(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 3: Beacon exchange");

    let local_beacon = ctx
        .has_capability("discovery")
        .then(|| {
            ctx.call(
                "discovery",
                "birdsong.generate_encrypted_beacon",
                serde_json::json!({
                    "node_id": "eastgate-exp076",
                    "capabilities": ["security", "discovery"]
                }),
            )
            .map_or(None, |val| val.get("encrypted_beacon").cloned())
        })
        .flatten();

    v.check_bool(
        "local_beacon_generated",
        local_beacon.is_some(),
        "Eastgate Songbird birdsong beacon via CompositionContext",
    );

    let local_songbird = local_songbird_port();
    let pixel_songbird = pixel_songbird_port();
    let local_live = tcp_rpc(
        "localhost",
        local_songbird,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .is_ok();
    let pixel_live = tcp_rpc(
        "localhost",
        pixel_songbird,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .is_ok();

    v.check_bool(
        "local_songbird_live",
        local_live,
        &format!("Eastgate Songbird at :{local_songbird}"),
    );
    v.check_bool(
        "pixel_songbird_live",
        pixel_live,
        &format!("Pixel Songbird at :{pixel_songbird}"),
    );

    let both_reachable = local_live && pixel_live;
    v.check_bool(
        "cross_gate_songbird_pair",
        both_reachable,
        "both Songbird instances reachable for beacon exchange",
    );
}

fn phase_neural_api_substrate(v: &mut ValidationResult) {
    v.section("Phase 4: Neural API substrate");
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("neural_api_substrate", "biomeOS not running");
        return;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_health",
        health.is_ok(),
        "biomeOS neural-api healthy",
    );

    let crypto = bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({}));
    v.check_bool(
        "neural_routing_crypto",
        crypto.is_ok(),
        "biomeOS routes crypto to local BearDog",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp076 — Cross-Gate Neural Routing")
        .with_provenance("exp076_cross_gate_neural_routing", "2026-05-09")
        .run(
            "primalSpring Exp076: Cross-gate capability routing via biomeOS substrate",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_pixel_tower(v);
                phase_cross_gate_crypto(v);
                phase_cross_gate_beacon_exchange(v, &mut ctx);
                phase_neural_api_substrate(v);
            },
        );
}
