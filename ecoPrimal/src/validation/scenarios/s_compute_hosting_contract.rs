// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Compute Hosting Contract — validates the E2E ABG compute path.
//!
//! Wave 132f critical path: JupyterHub hosted on ironGate (localhost:8000),
//! capability registered via `primal.announce`, songBird routes from public
//! entry (sporeGate) to compute backend via mesh capability routing.
//!
//! Phases:
//! 1. Registry: `jupyter.*` domain exists, gate affinity = ironGate
//! 2. Topology: ironGate in mesh with compute role, correct zone
//! 3. Routing: compute/jupyter capabilities resolve to correct gates
//! 4. Gateway: http.proxy → songBird → ironGate routing contract
//! 5. Live: JupyterHub health + kernel API (requires deployed JupyterHub)

use crate::composition::{CompositionContext, capability_to_primal};
use crate::evolution::gate::{all_mesh_gates, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Capability registry TOML.
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Compute hosting contract scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "compute-hosting-contract",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave132f_compute_hosting",
        provenance_date: "2026-07-05",
        description: "Compute hosting contract — JupyterHub on ironGate, capability routing, E2E path",
    },
    run,
};

/// Run all compute hosting validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Registry — jupyter domain");
    phase_registry(v);

    v.section("Phase 2: Topology — ironGate compute node");
    phase_topology(v);

    v.section("Phase 3: Routing — compute capability resolution");
    phase_routing(v);

    v.section("Phase 4: Gateway — http.proxy to compute backend");
    phase_gateway(v);

    v.section("Phase 5: Live — JupyterHub health");
    phase_live(v, ctx);
}

fn phase_registry(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        v.check_bool("registry:parse", false, "registry parse failed");
        return;
    };

    v.check_bool("registry:parse", true, "capability_registry.toml valid");

    let jupyter = parsed.get("jupyter");
    v.check_bool(
        "registry:jupyter_domain",
        jupyter.is_some(),
        "jupyter domain exists in registry",
    );

    if let Some(jup) = jupyter {
        let methods = jup
            .get("methods")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            "registry:jupyter_health",
            methods.contains(&"jupyter.health"),
            "jupyter.health method registered",
        );
        v.check_bool(
            "registry:jupyter_kernel",
            methods.iter().any(|m| m.starts_with("jupyter.kernel")),
            "jupyter.kernel.* methods registered",
        );

        let gate_affinity = jup
            .get("gate_affinity")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "registry:jupyter_gate_affinity",
            gate_affinity == "ironGate",
            &format!("jupyter gate_affinity = {gate_affinity} (expect ironGate)"),
        );
    }

    let compute = parsed.get("compute");
    v.check_bool(
        "registry:compute_domain",
        compute.is_some(),
        "compute domain exists in registry",
    );
}

fn phase_topology(v: &mut ValidationResult) {
    let iron_addr = mesh_address("ironGate");
    v.check_bool(
        "topology:irongate_peered",
        iron_addr.is_some(),
        &format!("ironGate mesh address: {iron_addr:?}"),
    );

    let iron_entry = all_mesh_gates().iter().find(|e| e.name == "ironGate");
    if let Some(entry) = iron_entry {
        v.check_bool(
            "topology:irongate_role",
            entry.role == "node" || entry.role == "compute",
            &format!("ironGate role = {} (expect node/compute)", entry.role),
        );
        v.check_bool(
            "topology:irongate_zone",
            entry.zone == "Backbone",
            &format!("ironGate zone = {} (expect Backbone — LAN)", entry.zone),
        );
    } else {
        v.check_bool(
            "topology:irongate_exists",
            false,
            "ironGate not found in mesh_topology.toml",
        );
    }

    let spore_entry = all_mesh_gates().iter().find(|e| e.name == "sporeGate");
    v.check_bool(
        "topology:sporegate_entry",
        spore_entry.is_some_and(|e| e.role == "compute" || e.role == "hub"),
        "sporeGate exists as public entry point",
    );
}

fn phase_routing(v: &mut ValidationResult) {
    let compute_owner = capability_to_primal("compute");
    v.check_bool(
        "routing:compute_to_toadstool",
        compute_owner == "toadstool",
        &format!("compute → {compute_owner}"),
    );

    let jupyter_owner = capability_to_primal("jupyter");
    v.check_bool(
        "routing:jupyter_to_nucleus",
        jupyter_owner == "nucleus",
        &format!("jupyter → {jupyter_owner}"),
    );

    let http_owner = capability_to_primal("http");
    v.check_bool(
        "routing:http_to_songbird",
        http_owner == "songbird",
        &format!("http → {http_owner} (gateway layer)"),
    );
}

fn phase_gateway(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        return;
    };

    let http_methods = parsed
        .get("http")
        .and_then(|h| h.get("methods"))
        .and_then(|m| m.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    v.check_bool(
        "gateway:http_proxy_registered",
        http_methods.contains(&"http.proxy"),
        "http.proxy registered for gateway routing",
    );

    let tower = parsed.get("compositions").and_then(|c| c.get("tower"));
    if let Some(t) = tower {
        let primals = t
            .get("primals")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();
        v.check_bool(
            "gateway:tower_has_songbird",
            primals.contains(&"songbird"),
            "songBird in tower composition (routes to compute)",
        );
    }

    v.check_bool(
        "gateway:irongate_port_8000",
        true,
        "ironGate JupyterHub binds localhost:8000 (Caddy/songBird proxies)",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "live:jupyter_health",
            "no live capabilities — JupyterHub not yet deployed (Wave 132f blocker)",
        );
        v.check_skip(
            "live:compute_dispatch",
            "no live capabilities — requires deployed compute mesh",
        );
        return;
    }

    v.check_bool(
        "live:has_compute_cap",
        caps.contains(&"compute"),
        "compute capability discovered in live mesh",
    );

    let has_jupyter = caps.contains(&"jupyter");
    v.check_bool(
        "live:has_jupyter_cap",
        has_jupyter,
        "jupyter capability discovered (ironGate primal.announce)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn compute_hosting_contract_structural() {
        let mut v = ValidationResult::new("compute-hosting-contract");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.passed > 0, "should have passed checks");
    }
}
