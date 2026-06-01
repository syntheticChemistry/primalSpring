// SPDX-License-Identifier: AGPL-3.0-or-later

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::neural_api_healthy;
use primalspring::ipc::protocol::{JSONRPC_VERSION, JsonRpcError, JsonRpcResponse, error_codes};
use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

use crate::handlers;
use crate::server::resolve_graphs_dir;

#[expect(
    clippy::too_many_lines,
    reason = "table-driven dispatch; splitting loses readability"
)]
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

    match method {
        // ── Health probes ──
        "health.check" | "health.liveness" => success_response(
            serde_json::json!({
                "status": "healthy",
                "primal": PRIMAL_NAME,
                "domain": PRIMAL_DOMAIN,
                "version": env!("CARGO_PKG_VERSION"),
            }),
            id,
        ),
        "health.version" => success_response(
            serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "build_hash": option_env!("PRIMALSPRING_BUILD_HASH").unwrap_or("dev"),
                "rust_version": env!("CARGO_PKG_RUST_VERSION"),
                "target": env!("TARGET"),
                "primal": PRIMAL_NAME,
            }),
            id,
        ),
        "health.drain" => {
            let timeout_ms = req["params"]["timeout_ms"].as_u64().unwrap_or(5000);
            success_response(
                serde_json::json!({
                    "status": "drained",
                    "in_flight": 0,
                    "timeout_ms": timeout_ms,
                    "primal": PRIMAL_NAME,
                }),
                id,
            )
        }
        "health.readiness" => {
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

        // ── Identity (sourDough compliance) ──
        "identity.get" => success_response(
            serde_json::json!({
                "id": PRIMAL_NAME,
                "display_name": "primalSpring",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": primalspring::niche::LOCAL_CAPABILITIES,
                "phase": "running",
                "family_id": primalspring::env_keys::resolve_family_id(),
            }),
            id,
        ),

        // ── Capability advertisement ──
        "capabilities.list" | "capability.list" => {
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

        // ── Coordination domain ──
        "coordination.validate_composition" => {
            handlers::handle_validate_composition(&req["params"], id)
        }
        "coordination.validate_composition_by_capability" => {
            handlers::handle_validate_composition_by_capability(&req["params"], id)
        }
        "coordination.discovery_sweep" => handlers::handle_discovery_sweep(&req["params"], id),
        "coordination.probe_primal" => handlers::handle_probe_primal(&req["params"], id),
        "coordination.probe_capability" => handlers::handle_probe_capability(&req["params"], id),
        "coordination.deploy_atomic" => handlers::handle_deploy_atomic(&req["params"], id),
        "coordination.bonding_test" => handlers::handle_bonding_test(&req["params"], id),
        "coordination.neural_api_status" => {
            let dispatcher = primalspring::composition::neural_dispatch::NeuralDispatcher::discover();
            let mut report = dispatcher.status_report();
            report["healthy"] = serde_json::json!(neural_api_healthy());
            success_response(report, id)
        }

        // ── Composition health (per-tier, capability-based) ──
        "composition.tower_health" => {
            handlers::handle_composition_health_by_capability(AtomicType::Tower, id)
        }
        "composition.tower_squirrel_health" => handlers::handle_tower_squirrel_health(id),
        "composition.node_health" => {
            handlers::handle_composition_health_by_capability(AtomicType::Node, id)
        }
        "composition.nest_health" => {
            handlers::handle_composition_health_by_capability(AtomicType::Nest, id)
        }
        "composition.nucleus_health" => {
            handlers::handle_composition_health_by_capability(AtomicType::FullNucleus, id)
        }

        // ── Lifecycle management ──
        "nucleus.start" => handlers::handle_nucleus_lifecycle("start", &req["params"], id),
        "nucleus.stop" => handlers::handle_nucleus_lifecycle("stop", &req["params"], id),
        "lifecycle.status" => success_response(
            serde_json::json!({
                "primal": PRIMAL_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "domain": PRIMAL_DOMAIN,
                "status": "running",
            }),
            id,
        ),

        // ── MCP tool discovery ──
        "mcp.tools.list" => {
            let tools = primalspring::ipc::mcp::list_tools();
            success_response(serde_json::to_value(tools).unwrap_or_default(), id)
        }

        // ── Ionic bond negotiation (Track 4, WS-1) ──
        "bonding.propose" => handlers::handle_bonding_propose(&req["params"], id),
        "bonding.accept" => handlers::handle_bonding_accept(&req["params"], id),
        "bonding.terminate" => handlers::handle_bonding_terminate(&req["params"], id),
        "bonding.modify_scope" => handlers::handle_bonding_modify_scope(&req["params"], id),
        "bonding.status" => handlers::handle_bonding_status(&req["params"], id),

        // ── Graph coordination ──
        "graph.list" => {
            let graphs_dir = resolve_graphs_dir();
            let results = primalspring::deploy::validate_all_graphs(&graphs_dir);
            success_response(serde_json::to_value(results).unwrap_or_default(), id)
        }
        "graph.validate" => handlers::handle_graph_validate(&req["params"], id),
        "graph.waves" => handlers::handle_graph_waves(&req["params"], id),
        "graph.capabilities" => handlers::handle_graph_capabilities(&req["params"], id),

        _ => JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: Some(JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: format!("Method not found: {method}"),
                data: None,
            }),
            id,
        },
    }
}

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
