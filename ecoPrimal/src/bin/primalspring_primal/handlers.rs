// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use primalspring::coordination::{AtomicType, validate_composition_ctx};
use primalspring::ipc::discover::discover_capabilities_for;
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
        tracing::warn!(
            "identity mode removed — redirecting to capability mode for runtime discovery"
        );
    }

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

pub fn handle_probe_primal(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    use primalspring::composition::{CompositionContext, capability_to_primal};

    let capability = params["capability"]
        .as_str()
        .or_else(|| params["primal"].as_str())
        .unwrap_or("security");

    let start = std::time::Instant::now();
    let mut ctx = CompositionContext::discover();
    let resolved = capability_to_primal(capability);
    let has = ctx.has_capability(capability);
    let health_ok = if has {
        ctx.health_check(capability).unwrap_or(false)
    } else {
        false
    };
    let latency = primalspring::cast::micros_u64(start.elapsed());
    success_response(
        serde_json::json!({
            "capability": capability,
            "resolved_primal": resolved,
            "socket_found": has,
            "health_ok": health_ok,
            "capabilities": ctx.available_capabilities(),
            "latency_us": latency,
        }),
        id,
    )
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

pub fn handle_composition_health_by_capability(atomic: AtomicType, id: u64) -> JsonRpcResponse {
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

/// Tower + Squirrel overlay health — Tower capabilities plus AI bridge.
pub fn handle_tower_squirrel_health(id: u64) -> JsonRpcResponse {
    use primalspring::composition::CompositionContext;

    let tower = validate_composition_ctx(AtomicType::Tower);
    let mut ctx = CompositionContext::discover();
    let ai_ok = ctx.has_capability("ai") && ctx.health_check("ai").unwrap_or(false);
    let all_healthy = tower.all_healthy && ai_ok;
    let combined = serde_json::json!({
        "tower": tower,
        "squirrel_healthy": ai_ok,
        "all_healthy": all_healthy,
    });
    success_response(combined, id)
}

pub fn handle_validate_composition_by_capability(
    params: &serde_json::Value,
    id: u64,
) -> JsonRpcResponse {
    handle_validate_composition(params, id)
}

pub fn handle_probe_capability(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    use primalspring::composition::{CompositionContext, capability_to_primal};

    let capability = params["capability"].as_str().unwrap_or("security");

    let start = std::time::Instant::now();
    let mut ctx = CompositionContext::discover();
    let resolved = capability_to_primal(capability);
    let has = ctx.has_capability(capability);
    let health_ok = if has {
        ctx.health_check(capability).unwrap_or(false)
    } else {
        false
    };
    let latency = primalspring::cast::micros_u64(start.elapsed());

    success_response(
        serde_json::json!({
            "capability": capability,
            "resolved_primal": resolved,
            "socket_found": has,
            "source": "CompositionContext",
            "health_ok": health_ok,
            "capabilities": ctx.available_capabilities(),
            "latency_us": latency,
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
                let mut registry = ionic_registry();
                match registry.propose(p.clone()) {
                    Ok(contract_id) => success_response(
                        serde_json::json!({
                            "status": "proposed",
                            "contract_id": contract_id,
                            "proposer": p.proposer_identity,
                            "capabilities_requested": p.requested_capabilities,
                            "duration_secs": p.duration_secs,
                            "note": "proposal registered — use bonding.accept with bearDog crypto.ionic_bond.verify_proposal for E2E",
                        }),
                        id,
                    ),
                    Err(e) => error_response(
                        error_codes::INTERNAL_ERROR,
                        &format!("registry error: {e}"),
                        id,
                    ),
                }
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

pub fn handle_bonding_accept(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let contract_id = match params["contract_id"].as_str() {
        Some(id_str) if !id_str.is_empty() => id_str,
        _ => {
            return error_response(
                error_codes::INVALID_PARAMS,
                "missing required 'contract_id' parameter",
                id,
            )
        }
    };
    let constraints: primalspring::bonding::BondingConstraint =
        serde_json::from_value(params["constraints"].clone()).unwrap_or_default();

    let mut registry = ionic_registry();
    match registry.accept(contract_id, constraints) {
        Ok(response) => success_response(
            serde_json::to_value(response).unwrap_or_default(),
            id,
        ),
        Err(e) => error_response(
            error_codes::INVALID_PARAMS,
            &format!("accept failed: {e}"),
            id,
        ),
    }
}

pub fn handle_bonding_terminate(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let contract_id = match params["contract_id"].as_str() {
        Some(id_str) if !id_str.is_empty() => id_str,
        _ => {
            return error_response(
                error_codes::INVALID_PARAMS,
                "missing required 'contract_id' parameter",
                id,
            )
        }
    };
    let reason = match params["reason"].as_str().unwrap_or("complete") {
        "policy_violation" => primalspring::bonding::ionic::TerminationReason::PolicyViolation,
        "mutual" => primalspring::bonding::ionic::TerminationReason::MutualAgreement,
        "expired" => primalspring::bonding::ionic::TerminationReason::Expired,
        _ => primalspring::bonding::ionic::TerminationReason::Complete,
    };
    let request = primalspring::bonding::ionic::TerminationRequest {
        contract_id: contract_id.to_owned(),
        reason,
    };

    let mut registry = ionic_registry();
    match registry.terminate(&request) {
        Ok(seal) => success_response(
            serde_json::to_value(seal).unwrap_or_default(),
            id,
        ),
        Err(e) => error_response(
            error_codes::INVALID_PARAMS,
            &format!("terminate failed: {e}"),
            id,
        ),
    }
}

pub fn handle_bonding_modify_scope(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let modification: Result<primalspring::bonding::ionic::ScopeModification, _> =
        serde_json::from_value(params.clone());
    match modification {
        Ok(m) => {
            let mut registry = ionic_registry();
            match registry.modify_scope(&m) {
                Ok(contract) => success_response(
                    serde_json::json!({
                        "contract_id": contract.contract_id,
                        "state": format!("{:?}", contract.state),
                        "capabilities": contract.negotiated_constraints.capability_allow,
                    }),
                    id,
                ),
                Err(e) => error_response(
                    error_codes::INVALID_PARAMS,
                    &format!("modify_scope failed: {e}"),
                    id,
                ),
            }
        }
        Err(e) => error_response(
            error_codes::INVALID_PARAMS,
            &format!("invalid modification: {e}"),
            id,
        ),
    }
}

pub fn handle_bonding_status(params: &serde_json::Value, id: u64) -> JsonRpcResponse {
    let contract_id = match params["contract_id"].as_str() {
        Some(id_str) if !id_str.is_empty() => id_str,
        _ => {
            return error_response(
                error_codes::INVALID_PARAMS,
                "missing required 'contract_id' parameter",
                id,
            )
        }
    };
    let registry = ionic_registry();
    registry.get(contract_id).map_or_else(
        || {
            success_response(
                serde_json::json!({
                    "contract_id": contract_id,
                    "state": "not_found",
                    "capabilities": [],
                    "bond_type": null,
                }),
                id,
            )
        },
        |contract| {
            success_response(
                serde_json::json!({
                    "contract_id": contract.contract_id,
                    "state": format!("{:?}", contract.state),
                    "capabilities": contract.negotiated_constraints.capability_allow,
                    "bond_type": "ionic",
                    "usage": {
                        "total_calls": contract.usage.total_calls,
                        "total_bytes": contract.usage.total_bytes,
                        "distinct_methods": contract.usage.distinct_methods,
                    },
                }),
                id,
            )
        },
    )
}

fn ionic_registry() -> primalspring::bonding::ionic_runtime::IonicContractRegistry {
    primalspring::bonding::ionic_runtime::IonicContractRegistry::new()
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
