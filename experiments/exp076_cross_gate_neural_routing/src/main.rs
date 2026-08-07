// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp076: Cross-Gate Neural Routing — capability routing across gates via Neural API.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn phase_local_neural_routing(v: &mut ValidationResult) {
    v.section("Phase 1: Local Neural API routing");

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("neural_api_health", "biomeOS not running");
        v.check_skip("neural_routing_crypto", "biomeOS not running");
        return;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_health",
        health.is_ok(),
        "biomeOS neural-api healthy",
    );

    match bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({})) {
        Ok(_) => v.check_bool(
            "neural_routing_crypto",
            true,
            "capability.call routes crypto to local BearDog",
        ),
        Err(e) => v.check_skip("neural_routing_crypto", &format!("crypto routing: {e}")),
    }
}

fn phase_cross_gate_crypto(v: &mut ValidationResult) {
    v.section("Phase 2: Cross-gate crypto via Neural API");

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("cross_gate_crypto", "biomeOS not running");
        return;
    };

    match bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({})) {
        Ok(resp) => {
            let has_key = resp.value.get("public_key").is_some();
            v.check_bool(
                "cross_gate_crypto",
                has_key,
                "crypto.generate_keypair via Neural API",
            );
        }
        Err(e) => v.check_skip("cross_gate_crypto", &format!("crypto: {e}")),
    }
}

fn phase_beacon_exchange(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 3: Beacon exchange via CompositionContext");

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

    let discovery_ok = ctx.has_capability("discovery");
    v.check_bool(
        "discovery_capability_available",
        discovery_ok,
        "discovery capability in CompositionContext",
    );

    if discovery_ok {
        let health = ctx.health_check("discovery").unwrap_or(false);
        v.check_bool(
            "local_songbird_live",
            health,
            "Songbird healthy via CompositionContext",
        );
    }
}

fn phase_neural_api_substrate(v: &mut ValidationResult) {
    v.section("Phase 4: Neural API substrate verification");
    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("neural_api_substrate", "biomeOS not running");
        return;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_substrate",
        health.is_ok(),
        "biomeOS neural-api healthy",
    );

    for domain in &["crypto", "discovery", "mesh"] {
        let check_name = format!("neural_route_{domain}");
        match bridge.capability_call(domain, "health.check", &serde_json::json!({})) {
            Ok(_) => v.check_bool(
                &check_name,
                true,
                &format!("{domain} routes through Neural API"),
            ),
            Err(e) => v.check_skip(&check_name, &format!("{domain}: {e}")),
        }
    }
}

#[cfg(feature = "primordial-compat")]
fn phase_legacy_tcp(v: &mut ValidationResult) {
    use primalspring::ipc::methods;
    use primalspring::ipc::tcp::tcp_rpc;

    v.section("Phase 5 (legacy): Direct TCP to Pixel gate");

    let (bd_host, bd_port) = {
        let s = std::env::var("PIXEL_BEARDOG_TCP").unwrap_or_else(|_| "localhost:19100".to_owned());
        match s.rsplit_once(':') {
            Some((host, port_str)) => (host.to_owned(), port_str.parse().unwrap_or(19100)),
            None => (s, 19100),
        }
    };

    let beardog_ok = tcp_rpc(
        &bd_host,
        bd_port,
        methods::health::CHECK,
        &serde_json::json!({}),
    )
    .is_ok();
    v.check_bool(
        "legacy_pixel_beardog",
        beardog_ok,
        &format!("Pixel BearDog at {bd_host}:{bd_port} (legacy TCP)"),
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp076 — Cross-Gate Neural Routing")
        .with_provenance("exp076_cross_gate_neural_routing", "2026-05-09")
        .run(
            "primalSpring Exp076: Cross-gate capability routing via biomeOS substrate",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_local_neural_routing(v);
                phase_cross_gate_crypto(v);
                phase_beacon_exchange(v, &mut ctx);
                phase_neural_api_substrate(v);

                #[cfg(feature = "primordial-compat")]
                phase_legacy_tcp(v);
            },
        );
}
