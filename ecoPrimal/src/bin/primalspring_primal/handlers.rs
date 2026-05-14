// SPDX-License-Identifier: AGPL-3.0-or-later

use primalspring::coordination::{AtomicType, validate_composition_ctx};
use primalspring::ipc::discover::{discover_capabilities_for, discover_for};
use primalspring::ipc::protocol::{JsonRpcResponse, error_codes};

use crate::dispatch::{error_response, parse_atomic_type, success_response};
use crate::server::resolve_graphs_dir;

pub fn handle_validate_composition(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("Tower");
    let Some(atomic) = parse_atomic_type(atomic_str) else {
        return error_response(
            error_codes::INVALID_PARAMS,
            &format!("Unknown atomic type: {atomic_str}"),
            id,
        );
    };

    let result = validate_composition_ctx(atomic);
    match serde_json::to_value(result) {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

pub fn handle_discovery_sweep(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("FullNucleus");
    let atomic = parse_atomic_type(atomic_str).unwrap_or(AtomicType::FullNucleus);
    let mode = params["mode"].as_str().unwrap_or("capability");

    if mode == "identity" {
        let primals = atomic.required_primals();
        let results = discover_for(primals);
        let summary: Vec<serde_json::Value> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "primal": r.primal,
                    "socket": r.socket.as_ref().map(|p| p.display().to_string()),
                    "source": format!("{:?}", r.source),
                })
            })
            .collect();
        success_response(
            serde_json::json!({ "primals": summary, "mode": "identity" }),
            id,
        )
    } else {
        let capabilities = atomic.required_capabilities();
        let results = discover_capabilities_for(capabilities);
        let summary: Vec<serde_json::Value> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "capability": r.capability,
                    "resolved_primal": r.resolved_primal,
                    "socket": r.socket.as_ref().map(|p| p.display().to_string()),
                    "source": format!("{:?}", r.source),
                })
            })
            .collect();
        success_response(
            serde_json::json!({ "capabilities": summary, "mode": "capability" }),
            id,
        )
    }
}

pub fn handle_probe_primal(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let name = params["primal"]
        .as_str()
        .unwrap_or(primalspring::primal_names::BEARDOG);

    use primalspring::composition::CompositionContext;
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let has = ctx.has_capability(name);
    let health_ok = if has {
        ctx.health_check(name).unwrap_or(false)
    } else {
        false
    };
    let result = serde_json::json!({
        "name": name,
        "socket_found": has,
        "health_ok": health_ok,
        "capabilities": ctx.available_capabilities(),
        "latency_us": 0,
    });
    success_response(result, id)
}

pub fn handle_deploy_atomic(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("Tower");
    let Some(atomic) = parse_atomic_type(atomic_str) else {
        return error_response(
            error_codes::INVALID_PARAMS,
            &format!("Unknown atomic type: {atomic_str}"),
            id,
        );
    };

    let graphs_dir = resolve_graphs_dir();
    let graph_file = graphs_dir.join(format!("{}.toml", atomic.graph_name()));

    let structure_ok = if graph_file.exists() {
        let result = primalspring::deploy::validate_structure(&graph_file);
        serde_json::to_value(&result).ok()
    } else {
        None
    };

    let composition = validate_composition_ctx(atomic);
    success_response(
        serde_json::json!({
            "atomic": atomic_str,
            "graph": atomic.graph_name(),
            "graph_exists": graph_file.exists(),
            "graph_path": graph_file.display().to_string(),
            "graph_validation": structure_ok,
            "composition": serde_json::to_value(&composition).unwrap_or_default(),
        }),
        id,
    )
}

pub fn handle_bonding_test(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let bond_str = params["bond_type"].as_str().unwrap_or("Covalent");
    let bond = match bond_str {
        "Covalent" => primalspring::bonding::BondType::Covalent,
        "Metallic" => primalspring::bonding::BondType::Metallic,
        "Ionic" => primalspring::bonding::BondType::Ionic,
        "Weak" => primalspring::bonding::BondType::Weak,
        "OrganoMetalSalt" => primalspring::bonding::BondType::OrganoMetalSalt,
        _ => {
            return error_response(
                error_codes::INVALID_PARAMS,
                &format!("Unknown bond type: {bond_str}"),
                id,
            );
        }
    };

    let capabilities = AtomicType::FullNucleus.required_capabilities();
    let discovered = discover_capabilities_for(capabilities);
    let gates = discovered.iter().filter(|d| d.socket.is_some()).count();

    success_response(
        serde_json::json!({
            "bond_type": bond_str,
            "description": bond.description(),
            "gates_discovered": gates,
            "total_capabilities": capabilities.len(),
            "status": if gates >= 2 { "ready" } else { "insufficient_capabilities" },
        }),
        id,
    )
}

#[allow(
    deprecated,
    reason = "handlers expose deprecated coordination RPCs for backward compatibility"
)]
pub fn handle_composition_health_by_capability(atomic: AtomicType, id: u64) -> JsonRpcResponse {
    let result = primalspring::coordination::validate_composition_by_capability(atomic);
    match serde_json::to_value(result) {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

/// Tower + Squirrel overlay health — Tower capabilities plus AI bridge.
#[allow(
    deprecated,
    reason = "handlers expose deprecated coordination RPCs for backward compatibility"
)]
pub fn handle_tower_squirrel_health(id: u64) -> JsonRpcResponse {
    let tower = primalspring::coordination::validate_composition_by_capability(AtomicType::Tower);
    let ai_disc = primalspring::ipc::discover::discover_by_capability("ai");
    let ai_ok = ai_disc.socket.as_ref().is_some_and(|sock| {
        primalspring::ipc::client::PrimalClient::connect(sock, primalspring::primal_names::SQUIRREL)
            .is_ok_and(|mut c| c.health_check().unwrap_or(false))
    });
    let all_healthy = tower.all_healthy && ai_ok;
    let combined = serde_json::json!({
        "tower": tower,
        "squirrel_healthy": ai_ok,
        "all_healthy": all_healthy,
    });
    success_response(combined, id)
}

#[allow(
    deprecated,
    reason = "handlers expose deprecated coordination RPCs for backward compatibility"
)]
pub fn handle_validate_composition_by_capability(
    params: &serde_json::Value,
    id: u64,
) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("Tower");
    let Some(atomic) = parse_atomic_type(atomic_str) else {
        return error_response(
            error_codes::INVALID_PARAMS,
            &format!("Unknown atomic type: {atomic_str}"),
            id,
        );
    };

    let result = primalspring::coordination::validate_composition_by_capability(atomic);
    match serde_json::to_value(result) {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

pub fn handle_probe_capability(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let capability = params["capability"].as_str().unwrap_or("security");

    use primalspring::composition::{CompositionContext, capability_to_primal};

    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let resolved = capability_to_primal(capability);
    let has = ctx.has_capability(capability);
    let health_ok = if has {
        ctx.health_check(capability).unwrap_or(false)
    } else {
        false
    };

    success_response(
        serde_json::json!({
            "capability": capability,
            "resolved_primal": resolved,
            "socket_found": has,
            "source": "CompositionContext",
            "health_ok": health_ok,
            "capabilities": ctx.available_capabilities(),
            "latency_us": 0,
        }),
        id,
    )
}

pub fn handle_nucleus_lifecycle(
    action: &str,
    params: &serde_json::Value,
    id: u64,
) -> JsonRpcResponse {
    let atomic_str = params["atomic"].as_str().unwrap_or("FullNucleus");
    let atomic = parse_atomic_type(atomic_str).unwrap_or(AtomicType::FullNucleus);

    let graphs_dir = resolve_graphs_dir();
    let graph_file = graphs_dir.join(format!("{}.toml", atomic.graph_name()));

    success_response(
        serde_json::json!({
            "action": action,
            "atomic": atomic_str,
            "graph": atomic.graph_name(),
            "graph_exists": graph_file.exists(),
            "required_capabilities": atomic.required_capabilities(),
            "status": if graph_file.exists() { "graph_ready" } else { "graph_missing" },
            "note": format!("nucleus.{action} queued — biomeOS orchestrates actual deployment via deploy graph"),
        }),
        id,
    )
}

pub fn handle_bonding_propose(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let proposal: Result<primalspring::bonding::ionic::IonicProposal, _> =
        serde_json::from_value(params.clone());
    match proposal {
        Ok(p) => {
            let errors = p.validate();
            if errors.is_empty() {
                success_response(
                    serde_json::json!({
                        "status": "validated",
                        "proposer": p.proposer_identity,
                        "capabilities_requested": p.requested_capabilities,
                        "duration_secs": p.duration_secs,
                        "note": "proposal validated — ionic negotiation runtime pending BearDog crypto signatures",
                    }),
                    id,
                )
            } else {
                error_response(
                    error_codes::INVALID_PARAMS,
                    &format!("proposal validation failed: {}", errors.join("; ")),
                    id,
                )
            }
        }
        Err(e) => error_response(
            error_codes::INVALID_PARAMS,
            &format!("invalid proposal: {e}"),
            id,
        ),
    }
}

pub fn handle_bonding_status(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let contract_id = params["contract_id"].as_str().unwrap_or("");
    if contract_id.is_empty() {
        return error_response(
            error_codes::INVALID_PARAMS,
            "missing required 'contract_id' parameter",
            id,
        );
    }
    success_response(
        serde_json::json!({
            "contract_id": contract_id,
            "state": "not_found",
            "capabilities": [],
            "bond_type": null,
        }),
        id,
    )
}

pub fn handle_graph_validate(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let Some(path_str) = params["path"].as_str() else {
        return error_response(
            error_codes::INVALID_PARAMS,
            "missing required 'path' parameter",
            id,
        );
    };
    let live = params["live"].as_bool().unwrap_or(false);
    let path = std::path::Path::new(path_str);
    let result = if live {
        serde_json::to_value(primalspring::deploy::validate_live(path))
    } else {
        serde_json::to_value(primalspring::deploy::validate_structure(path))
    };
    match result {
        Ok(val) => success_response(val, id),
        Err(e) => error_response(
            error_codes::INTERNAL_ERROR,
            &format!("serialization: {e}"),
            id,
        ),
    }
}

pub fn handle_graph_waves(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let Some(path_str) = params["path"].as_str() else {
        return error_response(
            error_codes::INVALID_PARAMS,
            "missing required 'path' parameter",
            id,
        );
    };
    let path = std::path::Path::new(path_str);
    let graph = match primalspring::deploy::load_graph(path) {
        Ok(g) => g,
        Err(e) => return error_response(error_codes::INTERNAL_ERROR, &format!("{e}"), id),
    };
    match primalspring::deploy::topological_waves(&graph) {
        Ok(waves) => success_response(
            serde_json::json!({
                "graph": graph.graph.name,
                "waves": waves,
                "wave_count": waves.len(),
            }),
            id,
        ),
        Err(e) => error_response(error_codes::INTERNAL_ERROR, &format!("{e}"), id),
    }
}

pub fn handle_graph_capabilities(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let Some(path_str) = params["path"].as_str() else {
        return error_response(
            error_codes::INVALID_PARAMS,
            "missing required 'path' parameter",
            id,
        );
    };
    let path = std::path::Path::new(path_str);
    let graph = match primalspring::deploy::load_graph(path) {
        Ok(g) => g,
        Err(e) => return error_response(error_codes::INTERNAL_ERROR, &format!("{e}"), id),
    };
    let caps = primalspring::deploy::graph_required_capabilities(&graph);
    success_response(
        serde_json::json!({
            "graph": graph.graph.name,
            "required_capabilities": caps,
            "count": caps.len(),
        }),
        id,
    )
}
