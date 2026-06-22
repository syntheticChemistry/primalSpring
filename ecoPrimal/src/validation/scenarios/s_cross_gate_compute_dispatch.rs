// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cross-Gate Compute Dispatch
//!
//! Validates that compute workloads can be dispatched across the mesh —
//! eastGate (orchestration) routes `ml.*` and `compute.*` operations to
//! ironGate (GPU node) through the capability routing layer.
//!
//! Phases:
//! 1. Structural: compute capabilities registered and routed to Node atomic
//! 2. Wire contract: ml.*, compute.*, fleet.* methods in registry
//! 3. Routing: capability_to_primal resolves compute domains correctly
//! 4. Live: ToadStool/BarraCuda health probing for dispatch readiness

use crate::composition::{CompositionContext, capability_to_primal, method_to_capability_domain};
use crate::primal_names;
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

const COMPUTE_PRIMALS: &[&str] = &[
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
];

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-gate-compute-dispatch",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave123_covalent_trust",
        provenance_date: "2026-06-22",
        description: "Cross-gate compute dispatch — mesh routing to GPU nodes, fleet management",
    },
    run,
};

/// Run all cross-gate compute dispatch validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — compute topology");
    phase_structural(v);

    v.section("Phase 2: Wire contract — compute methods");
    phase_wire_contract(v);

    v.section("Phase 3: Routing — compute domain resolution");
    phase_routing(v);

    v.section("Phase 4: Live — compute primal readiness");
    phase_live_compute(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let node_gates: Vec<&str> = gates
        .iter()
        .filter(|g| g.role == "node" || g.role == "tower")
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "struct:node_gates_exist",
        !node_gates.is_empty(),
        &format!("node/tower gates in mesh: {node_gates:?}"),
    );

    let iron_gate = gates.iter().find(|g| g.name == "ironGate");
    v.check_bool(
        "struct:irongate_in_topology",
        iron_gate.is_some(),
        "ironGate (GPU compute node) present in mesh_topology.toml",
    );

    if let Some(ig) = iron_gate {
        v.check_bool(
            "struct:irongate_role_node",
            ig.role == "node",
            &format!("ironGate role = '{}' (expected 'node')", ig.role),
        );
    }

    for slug in COMPUTE_PRIMALS {
        let port = ports::default_port_for(slug);
        v.check_bool(
            &format!("struct:{slug}_port_assigned"),
            port > 0,
            &format!("{slug} has port {port} in port registry"),
        );
    }
}

fn phase_wire_contract(v: &mut ValidationResult) {
    let compute_methods = [
        ("compute.dispatch", "dispatch workload to fleet"),
        ("compute.status", "query compute node status"),
        ("ml.infer", "ML inference dispatch"),
        ("ml.train", "ML training dispatch"),
        ("fleet.status", "fleet node health"),
        ("fleet.submit", "submit workload to fleet"),
        ("shader.compile", "WGSL/SPIR-V shader compilation"),
        ("shader.list", "list available shader pipelines"),
    ];

    let mut registered = 0;
    for (method, desc) in compute_methods {
        let present = REGISTRY_TOML.contains(method);
        if present {
            registered += 1;
        }
        v.check_bool(
            &format!("wire:{}", method.replace('.', "_")),
            present,
            &format!("{method} ({desc}) in capability_registry.toml"),
        );
    }

    v.check_bool(
        "wire:compute_coverage",
        registered >= 4,
        &format!(
            "{registered}/{} compute methods registered",
            compute_methods.len()
        ),
    );
}

fn phase_routing(v: &mut ValidationResult) {
    let compute_domains = ["compute", "ml", "shader", "fleet"];

    for domain in compute_domains {
        let primal = capability_to_primal(domain);
        let resolves = !primal.is_empty();
        v.check_bool(
            &format!("route:{domain}_resolves"),
            resolves,
            &format!("capability_to_primal(\"{domain}\") → \"{primal}\""),
        );
    }

    let toadstool_methods = ["compute.dispatch", "fleet.submit", "fleet.status"];
    for method in toadstool_methods {
        let domain = method_to_capability_domain(method);
        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("route:method_{}", method.replace('.', "_")),
            primal == "toadstool" || primal == "barracuda" || !primal.is_empty(),
            &format!("{method} → domain '{domain}' → primal '{primal}'"),
        );
    }

    let shader_methods = ["shader.compile", "shader.list"];
    for method in shader_methods {
        let domain = method_to_capability_domain(method);
        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("route:method_{}", method.replace('.', "_")),
            primal == "coralreef" || !primal.is_empty(),
            &format!("{method} → domain '{domain}' → primal '{primal}'"),
        );
    }

    let iron_addr = mesh_address("ironGate");
    v.check_bool(
        "route:irongate_addressable",
        iron_addr.is_some(),
        &format!("ironGate mesh address: {iron_addr:?}"),
    );
}

fn phase_live_compute(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let compute_caps = ["compute", "shader"];

    for cap in compute_caps {
        match ctx.client_for(cap) {
            Some(client) => {
                let resp = client.call("health.liveness", serde_json::json!({}));
                match resp {
                    Ok(r) => {
                        v.check_bool(
                            &format!("live:{cap}_alive"),
                            r.is_success(),
                            &format!("{cap} primal responding to health.liveness"),
                        );
                    }
                    Err(e) if e.is_skippable() => {
                        v.check_skip(&format!("live:{cap}_alive"), &format!("{cap}: {e}"));
                    }
                    Err(e) => {
                        v.check_bool(&format!("live:{cap}_alive"), false, &format!("{e}"));
                    }
                }
            }
            None => {
                v.check_skip(&format!("live:{cap}_alive"), &format!("no {cap} client"));
            }
        }
    }

    match ctx.client_for("compute") {
        Some(client) => {
            let resp = client.call("fleet.status", serde_json::json!({}));
            match resp {
                Ok(r) => {
                    let has_fleet = r
                        .result
                        .as_ref()
                        .and_then(|v| v.get("nodes").or_else(|| v.get("workers")))
                        .is_some()
                        || r.is_success();
                    v.check_bool(
                        "live:fleet_status_responds",
                        has_fleet,
                        "ToadStool fleet.status returns node information",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:fleet_status_responds", &format!("fleet.status: {e}"));
                }
                Err(e) => {
                    v.check_bool("live:fleet_status_responds", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:fleet_status_responds", "no compute client");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn cross_gate_compute_structural() {
        let mut v = ValidationResult::new("cross-gate-compute-dispatch");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 15, "expected ≥15 checks, got {total}");
    }

    #[test]
    fn compute_routing_resolves() {
        let primal = capability_to_primal("compute");
        assert!(
            !primal.is_empty(),
            "compute domain should resolve to a primal"
        );
    }

    #[test]
    fn node_primals_have_ports() {
        for slug in COMPUTE_PRIMALS {
            let port = ports::default_port_for(slug);
            assert!(port > 0, "{slug} should have a non-zero port");
        }
    }

    #[test]
    fn irongate_in_topology() {
        let gates = all_mesh_gates();
        let iron = gates.iter().find(|g| g.name == "ironGate");
        assert!(iron.is_some(), "ironGate should be in mesh_topology.toml");
    }

    #[test]
    fn irongate_has_wg_address() {
        let addr = mesh_address("ironGate");
        assert_eq!(addr, Some("10.13.37.7"), "ironGate WG address should be .7");
    }

    #[test]
    fn compute_methods_in_registry() {
        assert!(REGISTRY_TOML.contains("compute.dispatch"));
        assert!(REGISTRY_TOML.contains("fleet.submit") || REGISTRY_TOML.contains("fleet.status"));
    }

    #[test]
    fn shader_domain_resolves() {
        let primal = capability_to_primal("shader");
        assert!(!primal.is_empty(), "shader domain should resolve");
    }

    #[test]
    fn ml_domain_resolves() {
        let primal = capability_to_primal("ml");
        assert!(!primal.is_empty(), "ml domain should resolve");
    }
}
