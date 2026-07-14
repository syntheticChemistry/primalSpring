// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: WAN Dispatch Validation — validates WAN relay routing through Songbird.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::evolution::gate::{CytoplasmZone, all_mesh_gates};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// WAN dispatch validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "wan-dispatch-validation",
        track: Track::Transport,
        tier: Tier::Rust,
        provenance_crate: "wave138a_wan_dispatch_validation",
        provenance_date: "2026-07-14",
        description: "WAN dispatch validation — relay forwarding and Songbird WAN routing ownership",
    },
    run,
};

/// Run WAN dispatch validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Relay forwarding methods");

    let has_relay_forward = REGISTRY_TOML.contains("relay.forward")
        || REGISTRY_TOML.contains("relay.mediate")
        || REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "relay:forward_path",
        has_relay_forward,
        "relay.forward or relay.mediate or mesh.relay registered for frame forwarding",
    );

    let has_relay_route = REGISTRY_TOML.contains("relay.route")
        || REGISTRY_TOML.contains("mesh.find_path")
        || REGISTRY_TOML.contains("capability.route");
    v.check_bool(
        "relay:route_path",
        has_relay_route,
        "relay.route or mesh.find_path registered for WAN route calculation",
    );

    v.check_bool(
        "relay:mesh_relay_present",
        REGISTRY_TOML.contains("mesh.relay"),
        "mesh.relay registered (Songbird WAN frame transport)",
    );

    v.section("Phase 2: Songbird owns WAN dispatch surface");

    let table = canonical_routing_table();
    let mesh_owner = table
        .route("mesh.relay")
        .map_or("", |e| e.owner.as_ref());
    v.check_bool(
        "owner:mesh_relay_songbird",
        mesh_owner == primal_names::SONGBIRD,
        &format!("mesh.relay owner: {mesh_owner} (expected songBird)"),
    );

    let wan_methods = ["network.federation.peers", "songbird.federation.peers"];
    let songbird_network = wan_methods.iter().any(|method| {
        table
            .route(method)
            .is_some_and(|e| &*e.owner == primal_names::SONGBIRD)
    });
    v.check_bool(
        "owner:wan_network_songbird",
        songbird_network,
        "network.federation.peers or songbird.federation.peers routes to songBird",
    );

    v.section("Phase 3: WAN gate endpoints in topology");

    let wan_gates: Vec<_> = all_mesh_gates()
        .iter()
        .filter(|g| CytoplasmZone::for_gate(&g.name) == CytoplasmZone::Wan)
        .collect();
    v.check_bool(
        "topo:wan_gates_declared",
        !wan_gates.is_empty(),
        &format!(
            "{} WAN-zone gates in mesh topology",
            wan_gates.len()
        ),
    );

    let wan_meshed = wan_gates.iter().filter(|g| !g.address.is_empty()).count();
    v.check_bool(
        "topo:wan_meshed",
        wan_meshed >= 1,
        &format!("{wan_meshed} WAN gates with mesh addresses for dispatch"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
