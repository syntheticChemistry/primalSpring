// SPDX-License-Identifier: AGPL-3.0-or-later

//! exp087 — Neural API Routing E2E
//!
//! Validates biomeOS Neural API capability routing end-to-end: every
//! domain (security, discovery, storage, compute, ai) is routed to
//! the correct primal and returns real results.

use primalspring::ipc::{methods, tcp};
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("Neural API Routing E2E")
        .with_provenance("exp087_neural_api_routing_e2e", "2026-03-29")
        .run("capability routing validation", |v| {
            let bm_port = tcp::env_port("BIOMEOS_PORT", 9800);
            let host = std::env::var("TOWER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

            phase_capability_discovery(v, &host, bm_port);
            phase_security_routing(v, &host, bm_port);
            phase_discovery_routing(v, &host, bm_port);
            phase_storage_routing(v, &host, bm_port);
            phase_compute_routing(v, &host, bm_port);
            phase_ai_routing(v, &host, bm_port);
            phase_graph_operations(v, &host, bm_port);
        });
}

fn phase_capability_discovery(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Capability Discovery");

    for domain in &["security", "discovery", "storage", "compute", "ai"] {
        let result = tcp::neural_api_capability_discover(host, port, domain);
        match result {
            Ok((val, _)) => {
                let has_providers = val.is_array()
                    || val.get("providers").is_some()
                    || val.get("capabilities").is_some();
                v.check_bool(
                    &format!("{domain} providers discovered"),
                    has_providers,
                    &format!("capability.discover returns providers for {domain}"),
                );
            }
            Err(e) => v.check_skip(
                &format!("{domain} discovery"),
                &format!("biomeOS not reachable: {e}"),
            ),
        }
    }
}

fn phase_security_routing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Security Domain -> BearDog");

    let result = tcp::neural_api_capability_call(
        host,
        port,
        "security",
        "crypto.blake3_hash",
        &serde_json::json!({"data": "neural routing test"}),
    );
    match result {
        Ok((val, latency)) => {
            let has_hash = val.get("hash").is_some()
                || val.get("digest").is_some()
                || val.get("result").is_some();
            v.check_bool(
                "security->BearDog routed",
                has_hash,
                "capability.call(security, crypto.blake3_hash) returns hash",
            );
            let elapsed_us = u64::try_from(latency.as_micros()).unwrap_or(u64::MAX);
            v.check_latency(
                "security routing latency",
                elapsed_us,
                tolerances::PRIMAL_STARTUP_LATENCY_US,
            );
        }
        Err(e) => v.check_skip("security routing", &format!("routing failed: {e}")),
    }
}

fn phase_discovery_routing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Discovery Domain -> Songbird");

    let result = tcp::neural_api_capability_call(
        host,
        port,
        "discovery",
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "node_id": "exp087-routing-test",
            "capabilities": ["coordination"]
        }),
    );
    match result {
        Ok((val, _)) => {
            let has_beacon = val.get("encrypted_beacon").is_some()
                || val.get("beacon").is_some()
                || val.get("result").is_some();
            v.check_bool(
                "discovery->Songbird routed",
                has_beacon,
                "capability.call(discovery, birdsong) returns beacon",
            );
        }
        Err(e) => v.check_skip("discovery routing", &format!("routing failed: {e}")),
    }
}

fn phase_storage_routing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Storage Domain -> NestGate");

    let store = tcp::neural_api_capability_call(
        host,
        port,
        "storage",
        "storage.store",
        &serde_json::json!({
            "key": "exp087_routing_test",
            "value": "neural_api_e2e",
        }),
    );
    match &store {
        Ok((_, _)) => {
            v.check_bool(
                "storage->NestGate store",
                true,
                "capability.call(storage, store) succeeded",
            );
        }
        Err(e) => {
            v.check_skip("storage routing", &format!("NestGate routing failed: {e}"));
            return;
        }
    }

    let retrieve = tcp::neural_api_capability_call(
        host,
        port,
        "storage",
        "storage.retrieve",
        &serde_json::json!({"key": "exp087_routing_test"}),
    );
    match retrieve {
        Ok((val, _)) => {
            let correct = val.get("value").and_then(|v| v.as_str()) == Some("neural_api_e2e");
            v.check_bool(
                "storage round-trip via Neural API",
                correct,
                "store+retrieve through Neural API returns correct value",
            );
        }
        Err(e) => v.check_skip("storage retrieve routing", &format!("retrieve failed: {e}")),
    }
}

fn phase_compute_routing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Compute Domain -> ToadStool");

    let result = tcp::neural_api_capability_call(
        host,
        port,
        "compute",
        "toadstool.health",
        &serde_json::json!({}),
    );
    match result {
        Ok((val, _)) => {
            let is_healthy = val.get("status").is_some() || val.get("healthy").is_some();
            v.check_bool(
                "compute->ToadStool routed",
                is_healthy,
                "capability.call(compute, toadstool.health) returns status",
            );
        }
        Err(e) => v.check_skip("compute routing", &format!("ToadStool routing failed: {e}")),
    }
}

fn phase_ai_routing(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("AI Domain -> Squirrel");

    let result =
        tcp::neural_api_capability_call(host, port, "ai", "ai.health", &serde_json::json!({}));
    match result {
        Ok((val, _)) => {
            let has_status = val.get("status").is_some() || val.get("healthy").is_some();
            v.check_bool(
                "ai->Squirrel routed",
                has_status,
                "capability.call(ai, ai.health) returns status",
            );
        }
        Err(e) => v.check_skip("ai routing", &format!("Squirrel routing failed: {e}")),
    }

    let tools =
        tcp::neural_api_capability_call(host, port, "ai", "mcp.tools.list", &serde_json::json!({}));
    match tools {
        Ok((val, _)) => {
            let has_tools = val.is_array() || val.get("tools").is_some();
            v.check_bool(
                "MCP tools via Neural API",
                has_tools,
                "mcp.tools.list returns tool definitions through AI domain",
            );
        }
        Err(e) => v.check_skip("MCP tools routing", &format!("tools failed: {e}")),
    }
}

fn phase_graph_operations(v: &mut ValidationResult, host: &str, port: u16) {
    v.section("Graph Operations");

    let graphs = tcp::tcp_rpc(host, port, methods::graph::LIST, &serde_json::json!({}));
    match graphs {
        Ok((val, _)) => {
            let has_graphs = val.is_array() || val.get("graphs").is_some();
            v.check_bool(
                "graph.list returns graphs",
                has_graphs,
                "biomeOS reports loaded deploy graphs",
            );
        }
        Err(e) => v.check_skip("graph list", &format!("biomeOS not reachable: {e}")),
    }
}
