// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: biomeOS Neural API — absorbed from exp075.

use crate::composition::CompositionContext;
use crate::ipc::NeuralBridge;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "biomeos-neural-api",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "exp075_biomeos_neural_api_live",
        provenance_date: "2026-05-09",
        description: "biomeOS Neural API live — routing, beacons, graph catalog",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(bridge) = phase_neural_bridge(v) else {
        return;
    };

    v.section("Phase 1: Orchestration RPC");
    phase_capability_list(v, ctx);

    v.section("Phase 2: Neural routing");
    validate_crypto_routing(&bridge, v);
    validate_beacon_routing(&bridge, v);

    v.section("Phase 3: Capability discover");
    validate_capability_discover(&bridge, v);

    v.section("Phase 4: Birdsong");
    validate_birdsong_beacon(v, ctx);

    v.section("Phase 5: Graph list");
    validate_graph_list(v, ctx);
}

fn phase_neural_bridge(v: &mut ValidationResult) -> Option<NeuralBridge> {
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

fn phase_capability_list(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.call("orchestration", "capability.list", serde_json::json!({})) {
        Ok(r) => {
            let count = r
                .get("count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            v.check_minimum("capability_domains", usize::try_from(count).unwrap_or(0), 5);
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("capability_list", &format!("{e}"));
        }
        Err(e) => v.check_bool("capability_list", false, &format!("error: {e}")),
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
                let has_socket =
                    val.get("primary_endpoint").is_some() || val.get("primary_socket").is_some();
                v.check_bool(&key, has_socket, &format!("{domain} domain discoverable"));
            }
            Err(e) => v.check_bool(&key, false, &format!("{e}")),
        }
    }
}

fn validate_birdsong_beacon(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip("birdsong_beacon", "discovery capability not in context");
        return;
    }

    let params = serde_json::json!({
        "node_id": "exp075-test",
        "capabilities": ["security", "discovery"]
    });
    match ctx.call("discovery", "birdsong.generate_encrypted_beacon", params) {
        Ok(val) => {
            let has_beacon = val.get("encrypted_beacon").is_some();
            v.check_bool(
                "birdsong_beacon",
                has_beacon,
                "Songbird birdsong.generate_encrypted_beacon",
            );
        }
        Err(e) if e.is_connection_error() => v.check_skip("birdsong_beacon", &format!("{e}")),
        Err(e) => v.check_bool("birdsong_beacon", false, &format!("error: {e}")),
    }
}

fn validate_graph_list(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.call("orchestration", "graph.list", serde_json::json!({})) {
        Ok(v_json) => {
            let graphs: Vec<serde_json::Value> = match v_json {
                serde_json::Value::Array(a) => a,
                other => serde_json::from_value(other).unwrap_or_default(),
            };
            let count = graphs.len();
            v.check_minimum("graph_count", count, 10);
        }
        Err(e) if e.is_connection_error() => v.check_skip("graph_list", &format!("{e}")),
        Err(e) => v.check_bool("graph_list", false, &format!("error: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biomeos_neural_api_no_panic() {
        let mut v = ValidationResult::new("biomeos-neural-api");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
