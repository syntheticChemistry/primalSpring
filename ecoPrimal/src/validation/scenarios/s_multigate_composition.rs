// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Multi-Gate Composition — validates deploy graph composition
//! across a heterogeneous gate topology.
//!
//! Validates that the 5-gate sovereign mesh deploy graph correctly:
//! 1. Declares all mesh gates with valid addresses and roles
//! 2. Assigns primals based on gate role (compute gates get GPU primals)
//! 3. Routes dispatch correctly (gpu → ironGate, nest → sporeGate)
//! 4. Maintains BTSP enforcement across all gates
//! 5. Live: BiomeOS health on local gate

use crate::composition::CompositionContext;
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DEPLOY_GRAPH: &str =
    include_str!("../../../../graphs/multi_node/five_gate_sovereign_mesh.toml");

/// Multi-gate composition scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "multigate-composition",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave124_multigate",
        provenance_date: "2026-06-23",
        description: "Multi-gate BiomeOS composition — 5-gate deploy graph, role dispatch, BTSP enforcement",
    },
    run,
};

/// Run all multi-gate composition phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Deploy graph schema");
    phase_schema(v);

    v.section("Phase 2: Gate role assignments");
    phase_roles(v);

    v.section("Phase 3: Dispatch routing");
    phase_dispatch(v);

    v.section("Phase 4: Topology cross-check");
    phase_topology_crosscheck(v);

    v.section("Phase 5: Live BiomeOS health");
    phase_live(v, ctx);
}

fn phase_schema(v: &mut ValidationResult) {
    let parsed: Result<toml::Value, _> = DEPLOY_GRAPH.parse();
    let Ok(graph) = parsed else {
        v.check_bool("schema:parses", false, "five_gate_sovereign_mesh.toml parse failed");
        return;
    };
    v.check_bool("schema:parses", true, "deploy graph valid TOML");

    let meta = graph.get("metadata").and_then(|m| m.as_table());
    v.check_bool("schema:has_metadata", meta.is_some(), "metadata section present");

    if let Some(meta) = meta {
        let name = meta.get("name").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "schema:has_name",
            !name.is_empty(),
            &format!("graph name: \"{name}\""),
        );

        let resolve = meta.get("resolve").and_then(toml::Value::as_bool).unwrap_or(false);
        v.check_bool("schema:resolve_true", resolve, "resolve = true (fragment-first)");
    }

    let topo = graph.get("topology").and_then(|t| t.as_table());
    v.check_bool("schema:has_topology", topo.is_some(), "topology section present");

    if let Some(topo) = topo {
        let gates = topo
            .get("gates")
            .and_then(|g| g.as_array())
            .map_or(0, Vec::len);
        v.check_bool(
            "schema:five_gates",
            gates >= 5,
            &format!("{gates} gates in topology (expect >= 5)"),
        );

        let mesh = topo.get("mesh").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "schema:mesh_wireguard",
            mesh == "wireguard",
            &format!("mesh type: \"{mesh}\""),
        );
    }

    let validation = graph.get("validation").and_then(|v| v.as_table());
    if let Some(val) = validation {
        let btsp = val.get("btsp_enforced").and_then(toml::Value::as_bool).unwrap_or(false);
        v.check_bool("schema:btsp_enforced", btsp, "BTSP enforced in graph");

        let method_gate = val
            .get("method_gate_enforced")
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        v.check_bool(
            "schema:method_gate_enforced",
            method_gate,
            "MethodGate enforced in graph",
        );
    }
}

fn phase_roles(v: &mut ValidationResult) {
    let parsed: toml::Value = match DEPLOY_GRAPH.parse() {
        Ok(p) => p,
        Err(_) => return,
    };

    let Some(gates) = parsed.get("gates").and_then(|g| g.as_table()) else {
        v.check_bool("roles:gates_section", false, "no [gates] section");
        return;
    };

    let expected_roles = [
        ("golgi", "relay"),
        ("sporeGate", "build_authority"),
        ("eastGate", "orchestration"),
        ("flockGate", "tower"),
        ("ironGate", "compute"),
    ];

    for (gate_name, expected_role) in expected_roles {
        let gate = gates.get(gate_name);
        let role = gate
            .and_then(|g| g.get("role"))
            .and_then(|r| r.as_str())
            .unwrap_or("");
        v.check_bool(
            &format!("roles:{gate_name}"),
            role == expected_role,
            &format!("{gate_name} role = \"{role}\" (expected \"{expected_role}\")"),
        );
    }

    let iron = gates.get("ironGate");
    let iron_primals = iron
        .and_then(|g| g.get("primals"))
        .and_then(|p| p.as_array())
        .map_or(0, Vec::len);
    v.check_bool(
        "roles:irongate_primals",
        iron_primals >= 10,
        &format!("ironGate has {iron_primals} primals assigned"),
    );

    let iron_gpu = iron
        .and_then(|g| g.get("gpu_target"))
        .and_then(|t| t.as_str())
        .unwrap_or("");
    v.check_bool(
        "roles:irongate_gpu_target",
        iron_gpu.contains("gnu"),
        &format!("ironGate gpu_target: \"{iron_gpu}\""),
    );
}

fn phase_dispatch(v: &mut ValidationResult) {
    let parsed: toml::Value = match DEPLOY_GRAPH.parse() {
        Ok(p) => p,
        Err(_) => return,
    };

    let Some(dispatch) = parsed.get("dispatch").and_then(|d| d.as_table()) else {
        v.check_bool("dispatch:section", false, "no [dispatch] section");
        return;
    };

    let gpu_gate = dispatch
        .get("gpu_workloads")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "dispatch:gpu_to_irongate",
        gpu_gate == "ironGate",
        &format!("gpu_workloads → \"{gpu_gate}\""),
    );

    let build_gate = dispatch
        .get("build_artifacts")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "dispatch:build_to_sporegate",
        build_gate == "sporeGate",
        &format!("build_artifacts → \"{build_gate}\""),
    );

    let routing = dispatch.get("routing").and_then(|r| r.as_table());
    if let Some(routing) = routing {
        let tensor_route = routing
            .get("tensor.*")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "dispatch:tensor_route",
            tensor_route == "ironGate",
            &format!("tensor.* → \"{tensor_route}\""),
        );

        let nest_route = routing
            .get("nest.*")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "dispatch:nest_route",
            nest_route == "sporeGate",
            &format!("nest.* → \"{nest_route}\""),
        );

        v.check_bool(
            "dispatch:routing_depth",
            routing.len() >= 4,
            &format!("{} dispatch routes defined", routing.len()),
        );
    }
}

fn phase_topology_crosscheck(v: &mut ValidationResult) {
    let mesh_gates = all_mesh_gates();

    let parsed: toml::Value = match DEPLOY_GRAPH.parse() {
        Ok(p) => p,
        Err(_) => return,
    };

    let graph_gates: Vec<String> = parsed
        .get("topology")
        .and_then(|t| t.get("gates"))
        .and_then(|g| g.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    for gate_name in &graph_gates {
        let in_mesh = mesh_gates.iter().any(|g| g.name == *gate_name);
        v.check_bool(
            &format!("topo:{gate_name}_in_mesh"),
            in_mesh,
            &format!("{gate_name} from deploy graph exists in mesh_topology.toml"),
        );

        let addr = mesh_address(gate_name);
        v.check_bool(
            &format!("topo:{gate_name}_addressable"),
            addr.is_some(),
            &format!("{gate_name} mesh address: {addr:?}"),
        );
    }

    v.check_bool(
        "topo:graph_mesh_alignment",
        graph_gates.len() <= mesh_gates.len(),
        &format!(
            "deploy graph gates ({}) <= mesh topology gates ({})",
            graph_gates.len(),
            mesh_gates.len()
        ),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let biome_cap = primal_names::BIOMEOS;
    match ctx.client_for(biome_cap) {
        Some(client) => {
            let resp = client.call("health.liveness", serde_json::json!({}));
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:biomeos_health",
                        r.is_success(),
                        "BiomeOS responding to health.liveness",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:biomeos_health", &format!("biomeOS: {e}"));
                }
                Err(e) => {
                    v.check_bool("live:biomeos_health", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:biomeos_health", "no biomeOS client available");
        }
    }

    match ctx.client_for(biome_cap) {
        Some(client) => {
            let resp = client.call("composition.status", serde_json::json!({}));
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:composition_status",
                        r.is_success(),
                        "BiomeOS composition.status responds",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:composition_status", &format!("{e}"));
                }
                Err(e) => {
                    v.check_bool("live:composition_status", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:composition_status", "no biomeOS client");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn multigate_composition_runs() {
        let mut v = ValidationResult::new("multigate-composition");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 20, "expected ≥20 checks, got {total}");
    }

    #[test]
    fn deploy_graph_parses() {
        let parsed: toml::Value = DEPLOY_GRAPH.parse().unwrap();
        assert!(parsed.get("metadata").is_some());
        assert!(parsed.get("topology").is_some());
        assert!(parsed.get("gates").is_some());
        assert!(parsed.get("dispatch").is_some());
    }

    #[test]
    fn five_gates_declared() {
        let parsed: toml::Value = DEPLOY_GRAPH.parse().unwrap();
        let gates = parsed
            .get("topology")
            .unwrap()
            .get("gates")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(gates.len(), 5);
    }

    #[test]
    fn irongate_is_compute_role() {
        let parsed: toml::Value = DEPLOY_GRAPH.parse().unwrap();
        let iron = parsed.get("gates").unwrap().get("ironGate").unwrap();
        assert_eq!(iron.get("role").unwrap().as_str().unwrap(), "compute");
    }

    #[test]
    fn dispatch_routes_gpu_to_irongate() {
        let parsed: toml::Value = DEPLOY_GRAPH.parse().unwrap();
        let dispatch = parsed.get("dispatch").unwrap();
        assert_eq!(
            dispatch.get("gpu_workloads").unwrap().as_str().unwrap(),
            "ironGate"
        );
    }

    #[test]
    fn all_gates_in_mesh_topology() {
        let mesh_gates = all_mesh_gates();
        let gate_names = ["golgi", "sporeGate", "eastGate", "flockGate", "ironGate"];
        for name in gate_names {
            assert!(
                mesh_gates.iter().any(|g| g.name == name),
                "{name} should be in mesh_topology.toml"
            );
        }
    }

    #[test]
    fn btsp_enforced_in_graph() {
        let parsed: toml::Value = DEPLOY_GRAPH.parse().unwrap();
        let val = parsed.get("validation").unwrap();
        assert!(val.get("btsp_enforced").unwrap().as_bool().unwrap());
    }

    #[test]
    fn tensor_routes_to_irongate() {
        let parsed: toml::Value = DEPLOY_GRAPH.parse().unwrap();
        let routing = parsed
            .get("dispatch")
            .unwrap()
            .get("routing")
            .unwrap()
            .as_table()
            .unwrap();
        assert_eq!(routing.get("tensor.*").unwrap().as_str().unwrap(), "ironGate");
        assert_eq!(routing.get("ml.*").unwrap().as_str().unwrap(), "ironGate");
    }
}
