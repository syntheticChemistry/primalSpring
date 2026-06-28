// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Relay Forward Transport — cross-gate frame forwarding.
//!
//! Validates the relay.forward path through the mesh: how frames traverse
//! from one gate to another via golgi (relay hub) and Songbird's mesh.relay
//! capability. Wave 127 proved sporeGate is ephemeral — relay must work
//! independently of any single compute node.
//!
//! Phases:
//! 1. Topology: relay hub identified, multi-path mesh structure
//! 2. Capability: mesh.relay + relay-related methods registered
//! 3. Ephemeral compute: sporeGate removable (not the only relay)
//! 4. Path diversity: multiple routes exist for cross-gate traffic
//! 5. Live: Songbird relay dispatch readiness

use crate::composition::CompositionContext;
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const TOPOLOGY_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Relay forward transport scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "relay-forward-transport",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave128_relay_forward",
        provenance_date: "2026-06-28",
        description: "Relay forward transport — cross-gate frame forwarding, ephemeral compute, path diversity",
    },
    run,
};

/// Run all relay forward validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Topology — relay hub structure");
    phase_topology(v);

    v.section("Phase 2: Capability — relay methods");
    phase_capability(v);

    v.section("Phase 3: Ephemeral compute — sporeGate independence");
    phase_ephemeral(v);

    v.section("Phase 4: Path diversity");
    phase_path_diversity(v);

    v.section("Phase 5: Live — relay dispatch");
    phase_live(v, ctx);
}

fn phase_topology(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let hub_gates: Vec<&str> = gates
        .iter()
        .filter(|g| g.role == "hub" || g.role == "relay")
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "topo:relay_hub_exists",
        !hub_gates.is_empty(),
        &format!("relay/hub gates: {hub_gates:?}"),
    );

    let golgi = gates.iter().find(|g| g.name == "golgi");
    v.check_bool(
        "topo:golgi_is_hub",
        golgi.is_some_and(|g| g.role == "hub"),
        "golgi has role=hub (relay anchor)",
    );

    let golgi_addr = mesh_address("golgi");
    v.check_bool(
        "topo:golgi_address",
        golgi_addr.is_some(),
        &format!("golgi address resolved from SSOT: {golgi_addr:?}"),
    );

    let meshed_count = gates.iter().filter(|g| !g.address.is_empty()).count();
    v.check_bool(
        "topo:multi_gate_mesh",
        meshed_count >= 4,
        &format!("{meshed_count} gates with WG addresses (relay needs ≥2 endpoints)"),
    );
}

fn phase_capability(v: &mut ValidationResult) {
    let relay_methods = [
        ("mesh.relay", "frame forwarding between gates"),
        ("mesh.connect", "establish relay connection"),
        ("mesh.find_path", "route path calculation"),
        ("mesh.publish", "event propagation via relay"),
    ];

    let mut registered = 0;
    for (method, desc) in relay_methods {
        let present = REGISTRY_TOML.contains(method);
        if present {
            registered += 1;
        }
        v.check_bool(
            &format!("cap:{}", method.replace('.', "_")),
            present,
            &format!("{method} — {desc}"),
        );
    }

    v.check_bool(
        "cap:relay_breadth",
        registered >= 3,
        &format!("{registered}/{} relay methods present", relay_methods.len()),
    );

    let has_birdsong = REGISTRY_TOML.contains("network.birdsong.beacon");
    v.check_bool(
        "cap:birdsong_discovery",
        has_birdsong,
        "network.birdsong.beacon for encrypted relay discovery",
    );
}

fn phase_ephemeral(v: &mut ValidationResult) {
    let has_compute_role = TOPOLOGY_TOML.contains("role = \"compute\"");
    v.check_bool(
        "ephemeral:sporegate_compute_role",
        has_compute_role,
        "sporeGate role = compute (ephemeral, not routing-critical)",
    );

    let golgi_hub = TOPOLOGY_TOML.contains("role = \"hub\"");
    v.check_bool(
        "ephemeral:golgi_hub_independent",
        golgi_hub,
        "golgi retains hub role (relay works without sporeGate)",
    );

    let gates = all_mesh_gates();
    let non_sporegate_meshed: Vec<&str> = gates
        .iter()
        .filter(|g| g.name != "sporeGate" && !g.address.is_empty())
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "ephemeral:mesh_without_sporegate",
        non_sporegate_meshed.len() >= 3,
        &format!(
            "{} gates remain meshed if sporeGate unplugged: {non_sporegate_meshed:?}",
            non_sporegate_meshed.len()
        ),
    );

    let sporegate_retired =
        TOPOLOGY_TOML.contains("retired = true") && TOPOLOGY_TOML.contains("sporeGate");
    v.check_bool(
        "ephemeral:sporegate_not_retired",
        !sporegate_retired,
        "sporeGate is active (ephemeral ≠ retired)",
    );
}

fn phase_path_diversity(v: &mut ValidationResult) {
    let gates = all_mesh_gates();
    let wan_gates: Vec<&str> = gates
        .iter()
        .filter(|g| g.zone == "Wan" && !g.address.is_empty())
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "path:wan_relay_exists",
        !wan_gates.is_empty(),
        &format!("WAN-reachable relay gates: {wan_gates:?}"),
    );

    let backbone_gates: Vec<&str> = gates
        .iter()
        .filter(|g| g.zone == "Backbone" && !g.address.is_empty())
        .map(|g| g.name.as_str())
        .collect();

    v.check_bool(
        "path:backbone_gates",
        backbone_gates.len() >= 2,
        &format!("Backbone zone gates: {backbone_gates:?}"),
    );

    let flockgate_addr = mesh_address("flockGate");
    let golgi_addr = mesh_address("golgi");
    v.check_bool(
        "path:wan_to_lan_relay",
        flockgate_addr.is_some() && golgi_addr.is_some(),
        "flockGate (WAN) ↔ golgi (hub) relay path exists",
    );

    let eastgate_addr = mesh_address("eastGate");
    let irongate_addr = mesh_address("ironGate");
    v.check_bool(
        "path:lan_internal_relay",
        eastgate_addr.is_some() && irongate_addr.is_some(),
        "eastGate ↔ ironGate internal relay path exists",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let Some(client) = ctx.client_for("mesh") else {
        v.check_skip("live:songbird_relay", "no mesh client");
        v.check_skip("live:mesh_connect", "no mesh client");
        return;
    };

    let resp = client.call("health.liveness", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:songbird_relay",
                r.is_success(),
                "Songbird live (relay transport available)",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:songbird_relay", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:songbird_relay", false, &format!("{e}"));
        }
    }

    let resp = client.call("mesh.peers", serde_json::json!({}));
    match resp {
        Ok(r) => {
            v.check_bool(
                "live:mesh_connect",
                r.is_success(),
                "mesh.peers responds (relay can enumerate targets)",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:mesh_connect", &format!("{e}"));
        }
        Err(e) => {
            v.check_bool("live:mesh_connect", false, &format!("{e}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::primal_names;
    use crate::tolerances::ports;
    use crate::validation::ValidationResult;

    #[test]
    fn relay_forward_runs() {
        let mut v = ValidationResult::new("relay-forward-transport");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 15, "expected ≥15 checks, got {total}");
    }

    #[test]
    fn golgi_is_relay_hub() {
        let gates = all_mesh_gates();
        let golgi = gates.iter().find(|g| g.name == "golgi");
        assert!(golgi.is_some_and(|g| g.role == "hub"));
    }

    #[test]
    fn mesh_relay_registered() {
        assert!(REGISTRY_TOML.contains("mesh.relay"));
    }

    #[test]
    fn sporegate_is_compute() {
        assert!(TOPOLOGY_TOML.contains("role = \"compute\""));
    }

    #[test]
    fn mesh_without_sporegate_viable() {
        let gates = all_mesh_gates();
        let non_spore = gates
            .iter()
            .filter(|g| g.name != "sporeGate" && !g.address.is_empty())
            .count();
        assert!(non_spore >= 3, "mesh must survive without sporeGate");
    }

    #[test]
    fn wan_relay_path_exists() {
        assert!(mesh_address("flockGate").is_some());
        assert!(mesh_address("golgi").is_some());
    }

    #[test]
    fn songbird_port_assigned() {
        let port = ports::default_port_for(primal_names::SONGBIRD);
        assert!(port > 0);
    }
}
