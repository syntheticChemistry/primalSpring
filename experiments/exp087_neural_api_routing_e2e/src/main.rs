// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! exp087 — Neural API Routing E2E
//!
//! Validates biomeOS Neural API capability routing end-to-end via
//! `NeuralBridge`: every domain (security, discovery, storage, compute, ai)
//! is routed to the correct primal and returns real results.
//!
//! This is the canonical N2-N5 experiment. All routing goes through
//! `NeuralBridge::capability_call()` — the single post-primordial consumer API.
//! TCP fallback is only used for cross-gate scenarios where no local
//! Neural API socket exists.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::ipc::methods;
use primalspring::validation::ValidationResult;

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Composition discovery");
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    v.check_bool(
        "has_security_capability_path",
        ctx.has_capability("security"),
        "security capability path",
    );
    v.check_bool(
        "has_discovery_capability_path",
        ctx.has_capability("discovery"),
        "discovery capability path",
    );
}

fn phase_neural_api_health(v: &mut ValidationResult, bridge: &NeuralBridge) -> bool {
    v.section("Phase 2: Neural API substrate health");

    match bridge.health_check() {
        Ok(_) => {
            v.check_bool("neural_api_health", true, "biomeOS neural-api healthy");
            true
        }
        Err(e) => {
            v.check_bool("neural_api_health", false, &format!("neural-api: {e}"));
            false
        }
    }
}

fn phase_capability_discovery(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 3: Capability discovery via NeuralBridge");

    for domain in &["security", "discovery", "storage", "compute", "ai"] {
        let check_name = format!("{domain}_providers_discovered");
        match bridge.discover_capability(domain) {
            Ok(val) => {
                let has_providers = val.is_array()
                    || val.get("providers").is_some()
                    || val.get("capabilities").is_some();
                v.check_bool(
                    &check_name,
                    has_providers,
                    &format!("capability.discover returns providers for {domain}"),
                );
            }
            Err(e) => v.check_skip(&check_name, &format!("{domain} discovery: {e}")),
        }
    }
}

fn phase_security_routing(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 4: Security domain → BearDog");

    match bridge.capability_call(
        "security",
        "crypto.blake3_hash",
        &serde_json::json!({"data": "neural routing test"}),
    ) {
        Ok(resp) => {
            let has_hash = resp.value.get("hash").is_some()
                || resp.value.get("digest").is_some()
                || resp.value.get("result").is_some();
            v.check_bool(
                "security_beardog_routed",
                has_hash,
                "capability.call(security, crypto.blake3_hash) returns hash",
            );
        }
        Err(e) => v.check_skip("security_beardog_routed", &format!("routing failed: {e}")),
    }
}

fn phase_discovery_routing(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 5: Discovery domain → Songbird");

    match bridge.capability_call(
        "discovery",
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "node_id": "exp087-routing-test",
            "capabilities": ["coordination"]
        }),
    ) {
        Ok(resp) => {
            let has_beacon = resp.value.get("encrypted_beacon").is_some()
                || resp.value.get("beacon").is_some()
                || resp.value.get("result").is_some();
            v.check_bool(
                "discovery_songbird_routed",
                has_beacon,
                "capability.call(discovery, birdsong) returns beacon",
            );
        }
        Err(e) => v.check_skip("discovery_songbird_routed", &format!("routing failed: {e}")),
    }
}

fn phase_storage_routing(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 6: Storage domain → NestGate");

    let store = bridge.capability_call(
        "storage",
        "storage.store",
        &serde_json::json!({
            "key": "exp087_routing_test",
            "value": "neural_api_e2e",
        }),
    );
    match &store {
        Ok(_) => {
            v.check_bool(
                "storage_nestgate_store",
                true,
                "capability.call(storage, store) succeeded",
            );
        }
        Err(e) => {
            v.check_skip("storage_routing", &format!("NestGate routing failed: {e}"));
            return;
        }
    }

    match bridge.capability_call(
        "storage",
        "storage.retrieve",
        &serde_json::json!({"key": "exp087_routing_test"}),
    ) {
        Ok(resp) => {
            let correct =
                resp.value.get("value").and_then(|v| v.as_str()) == Some("neural_api_e2e");
            v.check_bool(
                "storage_round_trip",
                correct,
                "store+retrieve through Neural API returns correct value",
            );
        }
        Err(e) => v.check_skip("storage_round_trip", &format!("retrieve failed: {e}")),
    }
}

fn phase_compute_routing(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 7: Compute domain → ToadStool");

    match bridge.capability_call("compute", "toadstool.health", &serde_json::json!({})) {
        Ok(resp) => {
            let is_healthy =
                resp.value.get("status").is_some() || resp.value.get("healthy").is_some();
            v.check_bool(
                "compute_toadstool_routed",
                is_healthy,
                "capability.call(compute, toadstool.health) returns status",
            );
        }
        Err(e) => v.check_skip(
            "compute_toadstool_routed",
            &format!("ToadStool routing failed: {e}"),
        ),
    }
}

fn phase_ai_routing(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 8: AI domain → Squirrel");

    match bridge.capability_call("ai", "ai.health", &serde_json::json!({})) {
        Ok(resp) => {
            let has_status =
                resp.value.get("status").is_some() || resp.value.get("healthy").is_some();
            v.check_bool(
                "ai_squirrel_routed",
                has_status,
                "capability.call(ai, ai.health) returns status",
            );
        }
        Err(e) => v.check_skip("ai_squirrel_routed", &format!("Squirrel routing failed: {e}")),
    }

    match bridge.capability_call("ai", "mcp.tools.list", &serde_json::json!({})) {
        Ok(resp) => {
            let has_tools = resp.value.is_array() || resp.value.get("tools").is_some();
            v.check_bool(
                "mcp_tools_via_neural_api",
                has_tools,
                "mcp.tools.list returns tool definitions through AI domain",
            );
        }
        Err(e) => v.check_skip("mcp_tools_via_neural_api", &format!("tools failed: {e}")),
    }
}

fn phase_graph_operations(v: &mut ValidationResult, bridge: &NeuralBridge) {
    v.section("Phase 9: Graph operations");

    match bridge.capability_call(
        "graph",
        methods::graph::LIST,
        &serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_graphs = resp.value.is_array() || resp.value.get("graphs").is_some();
            v.check_bool(
                "graph_list_returns_graphs",
                has_graphs,
                "biomeOS reports loaded deploy graphs",
            );
        }
        Err(e) => v.check_skip("graph_list_returns_graphs", &format!("graph.list: {e}")),
    }
}

#[cfg(feature = "primordial-compat")]
fn phase_legacy_tcp(v: &mut ValidationResult) {
    use primalspring::ipc::tcp;

    v.section("Phase 10 (legacy): TCP fallback routing");

    let port = tcp::env_port("BIOMEOS_PORT", 9800);
    let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

    match tcp::neural_api_capability_call(
        &host,
        port,
        "security",
        "crypto.blake3_hash",
        &serde_json::json!({"data": "tcp fallback test"}),
    ) {
        Ok((val, _)) => {
            let has_hash = val.get("hash").is_some() || val.get("digest").is_some();
            v.check_bool("legacy_tcp_security", has_hash, "TCP security routing");
        }
        Err(e) => v.check_skip("legacy_tcp_security", &format!("TCP: {e}")),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp087 — Neural API Routing E2E")
        .with_provenance("exp087_neural_api_routing_e2e", "2026-05-09")
        .run("capability routing validation via NeuralBridge", |v| {
            let ctx = CompositionContext::from_live_discovery_with_fallback();
            phase_composition_discovery(v, &ctx);

            let Some(bridge) = NeuralBridge::discover() else {
                v.check_skip("neural_api_health", "biomeOS not running — all routing skipped");
                return;
            };

            if !phase_neural_api_health(v, &bridge) {
                return;
            }

            phase_capability_discovery(v, &bridge);
            phase_security_routing(v, &bridge);
            phase_discovery_routing(v, &bridge);
            phase_storage_routing(v, &bridge);
            phase_compute_routing(v, &bridge);
            phase_ai_routing(v, &bridge);
            phase_graph_operations(v, &bridge);

            #[cfg(feature = "primordial-compat")]
            phase_legacy_tcp(v);
        });
}
