// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp075: biomeOS Neural API Live — validate capability routing through
//! a running biomeOS substrate.
//!
//! Connects to a live biomeOS neural-api instance (discovered via standard
//! socket paths or `NEURAL_API_SOCKET` env) and validates:
//!
//! - Neural API health and capability listing
//! - `crypto.generate_keypair` routing through biomeOS -> BearDog
//! - `beacon.generate` routing through biomeOS -> BearDog
//! - Birdsong encrypted beacon generation (direct Songbird RPC)
//! - Capability discovery for security, network, beacon domains
//!
//! Unlike exp060 which spawns biomeOS itself, this experiment expects
//! biomeOS to already be running (e.g. via manual start or deploy script).

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn validate_neural_api_connection(v: &mut ValidationResult) -> Option<NeuralBridge> {
    let bridge = NeuralBridge::discover();
    let has_bridge = bridge.is_some();

    v.check_bool(
        "neural_api_discovered",
        has_bridge,
        "biomeOS neural-api socket discovered",
    );

    if !has_bridge {
        v.check_skip(
            "neural_api_suite",
            "biomeOS not running — start with: biomeos neural-api --socket ... --graphs-dir ...",
        );
    }

    bridge
}

fn validate_capability_list(bridge: &NeuralBridge, v: &mut ValidationResult) {
    let mut client = match PrimalClient::connect(bridge.socket_path(), primal_names::BIOMEOS) {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("capability_list", false, &format!("connect failed: {e}"));
            return;
        }
    };

    let resp = client.call("capability.list", serde_json::json!({}));
    match resp {
        Ok(r) => {
            let count = r
                .result
                .as_ref()
                .and_then(|v| v.get("count"))
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            v.check_minimum("capability_domains", usize::try_from(count).unwrap_or(0), 5);
        }
        Err(e) => v.check_bool("capability_list", false, &format!("{e}")),
    }
}

fn validate_crypto_routing(bridge: &NeuralBridge, v: &mut ValidationResult) {
    let result = bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({}));
    match result {
        Ok(r) => {
            let has_key = r.value.get("public_key").is_some();
            v.check_bool(
                "crypto_generate_keypair",
                has_key,
                "biomeOS -> BearDog crypto.generate_keypair routing",
            );
        }
        Err(e) => v.check_bool("crypto_generate_keypair", false, &format!("{e}")),
    }
}

fn validate_beacon_routing(bridge: &NeuralBridge, v: &mut ValidationResult) {
    let result = bridge.capability_call("beacon", "generate", &serde_json::json!({}));
    match result {
        Ok(r) => {
            let has_id = r.value.get("beacon_id").is_some();
            v.check_bool(
                "beacon_generate",
                has_id,
                "biomeOS -> BearDog beacon.generate routing",
            );
        }
        Err(e) => v.check_bool("beacon_generate", false, &format!("{e}")),
    }
}

fn validate_capability_discover(bridge: &NeuralBridge, v: &mut ValidationResult) {
    for domain in &["security", "beacon", "crypto", "mesh", "discovery", "http"] {
        let result = bridge.discover_capability(domain);
        let key = format!("discover_{domain}");
        match result {
            Ok(val) => {
                let has_socket = val.get("primary_socket").is_some();
                v.check_bool(&key, has_socket, &format!("{domain} domain discoverable"));
            }
            Err(e) => v.check_bool(&key, false, &format!("{e}")),
        }
    }
}

fn validate_birdsong_beacon(v: &mut ValidationResult) {
    let songbird = discover::discover_primal(primal_names::SONGBIRD);
    let Some(socket) = songbird.socket else {
        v.check_skip("birdsong_beacon", "Songbird not discovered");
        return;
    };

    let mut client = match PrimalClient::connect(&socket, primal_names::SONGBIRD) {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("birdsong_connect", false, &format!("{e}"));
            return;
        }
    };

    let params = serde_json::json!({
        "node_id": "exp075-test",
        "capabilities": ["security", "discovery"]
    });
    let resp = client.call("birdsong.generate_encrypted_beacon", params);
    match resp {
        Ok(r) => {
            let has_beacon = r
                .result
                .as_ref()
                .and_then(|v| v.get("encrypted_beacon"))
                .is_some();
            v.check_bool(
                "birdsong_beacon",
                has_beacon,
                "Songbird birdsong.generate_encrypted_beacon",
            );
        }
        Err(e) => v.check_bool("birdsong_beacon", false, &format!("{e}")),
    }
}

fn validate_graph_list(bridge: &NeuralBridge, v: &mut ValidationResult) {
    let mut client = match PrimalClient::connect(bridge.socket_path(), primal_names::BIOMEOS) {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("graph_list", false, &format!("connect failed: {e}"));
            return;
        }
    };

    let resp = client.call("graph.list", serde_json::json!({}));
    match resp {
        Ok(r) => {
            let count = r
                .result
                .as_ref()
                .and_then(|v| v.as_array())
                .map_or(0, Vec::len);
            v.check_minimum("graph_count", count, 10);
        }
        Err(e) => v.check_bool("graph_list", false, &format!("{e}")),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp075 — biomeOS Neural API Live")
        .with_provenance("exp075_biomeos_neural_api_live", "2026-03-27")
        .run(
            "primalSpring Exp075: Live biomeOS substrate validation (capability routing, beacons, graphs)",
            |v| {
                let Some(bridge) = validate_neural_api_connection(v) else {
                    return;
                };

                validate_capability_list(&bridge, v);
                validate_crypto_routing(&bridge, v);
                validate_beacon_routing(&bridge, v);
                validate_capability_discover(&bridge, v);
                validate_birdsong_beacon(v);
                validate_graph_list(&bridge, v);
            },
        );
}
