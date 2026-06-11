// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::neural_api_healthy;
use primalspring::ipc::protocol::{JSONRPC_VERSION, JsonRpcError, JsonRpcResponse, error_codes};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

use crate::handlers;
use crate::server::resolve_graphs_dir;

/// Tracks the number of JSON-RPC requests currently being processed.
static IN_FLIGHT: AtomicU64 = AtomicU64::new(0);

/// Server startup time — initialized once in `init_startup_time()`.
static STARTUP_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

/// Initialize the server startup timestamp. Call once at server start.
pub fn init_startup_time() {
    STARTUP_TIME.get_or_init(Instant::now);
}

/// Seconds since server startup.
fn uptime_seconds() -> u64 {
    STARTUP_TIME.get().map_or(0, |t| t.elapsed().as_secs())
}

/// Uniform handler signature: `(params, request_id) -> response`.
type Handler = fn(&serde_json::Value, u64) -> JsonRpcResponse;

/// Static dispatch table mapping normalized method names to handlers.
///
/// Built once at first request. Uses method constants from `ipc::methods`
/// where available; remaining entries use string literals matching
/// `capability_registry.toml` exactly.
static DISPATCH_TABLE: LazyLock<HashMap<&'static str, Handler>> = LazyLock::new(|| {
    let entries: &[(&str, Handler)] = &[
        // ── Health probes (HEALTH-01: bare "health" aliases to check) ──
        ("health", handle_health_check),
        ("health.check", handle_health_check),
        ("health.liveness", handle_health_check),
        ("health.version", handle_health_version),
        ("health.drain", handle_health_drain),
        ("health.readiness", handle_health_readiness),
        // ── Identity ──
        ("identity.get", handle_identity_get),
        // ── Capability advertisement ──
        ("capabilities.list", handle_capabilities_list),
        ("capability.list", handle_capabilities_list),
        // ── Coordination domain ──
        (
            "coordination.validate_composition",
            handlers::handle_validate_composition,
        ),
        (
            "coordination.validate_composition_by_capability",
            handlers::handle_validate_composition_by_capability,
        ),
        (
            "coordination.discovery_sweep",
            handlers::handle_discovery_sweep,
        ),
        ("coordination.probe_primal", handlers::handle_probe_primal),
        (
            "coordination.probe_capability",
            handlers::handle_probe_capability,
        ),
        ("coordination.deploy_atomic", handlers::handle_deploy_atomic),
        ("coordination.bonding_test", handlers::handle_bonding_test),
        ("coordination.neural_api_status", handle_neural_api_status),
        // ── Composition health (per-tier) ──
        ("composition.tower_health", handle_tower_health),
        ("composition.tower_ai_health", handle_tower_ai_health),
        ("composition.node_health", handle_node_health),
        ("composition.nest_health", handle_nest_health),
        ("composition.nucleus_health", handle_nucleus_health),
        // ── Lifecycle management ──
        ("nucleus.start", handle_nucleus_start),
        ("nucleus.stop", handle_nucleus_stop),
        ("lifecycle.status", handle_lifecycle_status),
        // ── MCP tool discovery ──
        ("mcp.tools.list", handle_mcp_tools_list),
        // ── Ionic bond negotiation ──
        ("bonding.propose", handlers::handle_bonding_propose),
        ("bonding.accept", handlers::handle_bonding_accept),
        ("bonding.terminate", handlers::handle_bonding_terminate),
        (
            "bonding.modify_scope",
            handlers::handle_bonding_modify_scope,
        ),
        ("bonding.status", handlers::handle_bonding_status),
        // ── Graph coordination ──
        ("graph.list", handle_graph_list),
        ("graph.validate", handlers::handle_graph_validate),
        ("graph.waves", handlers::handle_graph_waves),
        ("graph.capabilities", handlers::handle_graph_capabilities),
    ];

    entries.iter().copied().collect()
});

pub fn dispatch_request(line: &str) -> JsonRpcResponse {
    let req: serde_json::Value = match serde_json::from_str(line.trim()) {
        Ok(v) => v,
        Err(_) => {
            return JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: None,
                error: Some(JsonRpcError {
                    code: error_codes::PARSE_ERROR,
                    message: "Parse error".to_owned(),
                    data: None,
                }),
                id: 0,
            };
        }
    };

    let id = req["id"].as_u64().unwrap_or(0);
    let raw_method = req["method"].as_str().unwrap_or("");
    let method = primalspring::ipc::normalize_method(raw_method);

    IN_FLIGHT.fetch_add(1, Ordering::Relaxed);
    let response = DISPATCH_TABLE.get(method).map_or_else(
        || JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: Some(JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: format!("Method not found: {method}"),
                data: None,
            }),
            id,
        },
        |handler| handler(&req["params"], id),
    );
    IN_FLIGHT.fetch_sub(1, Ordering::Relaxed);
    response
}

// ── Handler implementations ─────────────────────────────────────────

fn handle_health_check(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    success_response(
        serde_json::json!({
            "status": "healthy",
            "primal": PRIMAL_NAME,
            "version": env!("CARGO_PKG_VERSION"),
            "uptime_s": uptime_seconds(),
            "domain": PRIMAL_DOMAIN,
        }),
        id,
    )
}

fn handle_health_version(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    success_response(
        serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "build_hash": option_env!("PRIMALSPRING_BUILD_HASH").unwrap_or("dev"),
            "rust_version": env!("CARGO_PKG_RUST_VERSION"),
            "target": env!("TARGET"),
            "primal": PRIMAL_NAME,
        }),
        id,
    )
}

fn handle_health_drain(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let timeout_ms = params["timeout_ms"].as_u64().unwrap_or(5000);
    let current = IN_FLIGHT.load(Ordering::Relaxed);
    let status = if current <= 1 { "drained" } else { "draining" };
    success_response(
        serde_json::json!({
            "status": status,
            "in_flight": current.saturating_sub(1),
            "timeout_ms": timeout_ms,
            "primal": PRIMAL_NAME,
        }),
        id,
    )
}

fn handle_health_readiness(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let neural_ok = neural_api_healthy();
    let caps = AtomicType::FullNucleus.required_capabilities();
    let cap_results = primalspring::ipc::discover::discover_capabilities_for(caps);
    let reachable = cap_results.iter().filter(|r| r.socket.is_some()).count();
    success_response(
        serde_json::json!({
            "ready": reachable > 0,
            "neural_api": neural_ok,
            "capabilities_discovered": reachable,
            "capabilities_total": caps.len(),
        }),
        id,
    )
}

fn handle_identity_get(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    success_response(
        serde_json::json!({
            "id": PRIMAL_NAME,
            "display_name": "primalSpring",
            "version": env!("CARGO_PKG_VERSION"),
            "capabilities": primalspring::niche::LOCAL_CAPABILITIES,
            "phase": "running",
            "family_id": primalspring::env_keys::resolve_family_id(),
        }),
        id,
    )
}

fn handle_capabilities_list(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let routed: Vec<serde_json::Value> = primalspring::niche::ROUTED_CAPABILITIES
        .iter()
        .map(|(method, provider)| {
            serde_json::json!({ "method": method, "provider": provider.slug() })
        })
        .collect();
    success_response(
        serde_json::json!({
            "local_capabilities": primalspring::niche::LOCAL_CAPABILITIES,
            "routed_capabilities": routed,
            "capabilities": primalspring::niche::all_capabilities(),
            "semantic_mappings": primalspring::niche::coordination_semantic_mappings(),
            "operation_dependencies": primalspring::niche::operation_dependencies(),
            "cost_estimates": primalspring::niche::cost_estimates(),
        }),
        id,
    )
}

fn handle_neural_api_status(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let dispatcher = primalspring::composition::neural_dispatch::NeuralDispatcher::discover();
    let mut report = dispatcher.status_report();
    report["healthy"] = serde_json::json!(neural_api_healthy());
    success_response(report, id)
}

fn handle_tower_health(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_composition_health_by_capability(AtomicType::Tower, id)
}

fn handle_tower_ai_health(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_tower_ai_health(id)
}

fn handle_node_health(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_composition_health_by_capability(AtomicType::Node, id)
}

fn handle_nest_health(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_composition_health_by_capability(AtomicType::Nest, id)
}

fn handle_nucleus_health(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_composition_health_by_capability(AtomicType::FullNucleus, id)
}

fn handle_nucleus_start(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_nucleus_lifecycle("start", params, id)
}

fn handle_nucleus_stop(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    handlers::handle_nucleus_lifecycle("stop", params, id)
}

fn handle_lifecycle_status(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    success_response(
        serde_json::json!({
            "primal": PRIMAL_NAME,
            "version": env!("CARGO_PKG_VERSION"),
            "domain": PRIMAL_DOMAIN,
            "status": "running",
        }),
        id,
    )
}

fn handle_mcp_tools_list(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let tools = primalspring::ipc::mcp::list_tools();
    success_response(serde_json::to_value(tools).unwrap_or_default(), id)
}

fn handle_graph_list(_params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let graphs_dir = resolve_graphs_dir();
    let results = primalspring::deploy::validate_all_graphs(&graphs_dir);
    success_response(serde_json::to_value(results).unwrap_or_default(), id)
}

// ── Shared response constructors ────────────────────────────────────

pub fn parse_atomic_type(s: &str) -> Option<AtomicType> {
    s.parse().ok()
}

pub fn success_response(result: serde_json::Value, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_owned(),
        result: Some(result),
        error: None,
        id,
    }
}

pub fn error_response(code: i64, message: &str, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION.to_owned(),
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.to_owned(),
            data: None,
        }),
        id,
    }
}
