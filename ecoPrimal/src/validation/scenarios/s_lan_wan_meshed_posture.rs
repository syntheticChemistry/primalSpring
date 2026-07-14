// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: LAN/WAN Meshed Posture — validates backbone and WAN zone representation
//! in the sovereign mesh topology.

use crate::composition::CompositionContext;
use crate::evolution::gate::{CytoplasmZone, all_mesh_gates};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// LAN/WAN meshed posture scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "lan-wan-meshed-posture",
        track: Track::Transport,
        tier: Tier::Rust,
        provenance_crate: "wave138a_lan_wan_meshed_posture",
        provenance_date: "2026-07-14",
        description: "LAN/WAN meshed posture — backbone + WAN zones with multi-gate backbone",
    },
    run,
};

/// Run LAN/WAN meshed posture validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Zone representation in mesh topology");

    let has_backbone = MESH_TOML.contains("zone = \"Backbone\"");
    let has_wan = MESH_TOML.contains("zone = \"Wan\"");
    v.check_bool(
        "zones:backbone_present",
        has_backbone,
        "mesh_topology.toml declares Backbone zone gates",
    );
    v.check_bool(
        "zones:wan_present",
        has_wan,
        "mesh_topology.toml declares Wan zone gates",
    );
    v.check_bool(
        "zones:distinct_lan_wan",
        has_backbone && has_wan,
        "at least two distinct zones (Backbone + WAN) in mesh topology",
    );

    v.section("Phase 2: Backbone gate density");

    let gates = all_mesh_gates();
    let backbone_gates: Vec<_> = gates
        .iter()
        .filter(|g| g.zone == "Backbone")
        .collect();
    v.check_bool(
        "backbone:gate_count",
        backbone_gates.len() >= 2,
        &format!(
            "{} Backbone zone gates (need ≥2): {:?}",
            backbone_gates.len(),
            backbone_gates.iter().map(|g| &g.name).collect::<Vec<_>>()
        ),
    );

    let meshed_backbone = backbone_gates.iter().filter(|g| !g.address.is_empty()).count();
    v.check_bool(
        "backbone:meshed_peers",
        meshed_backbone >= 2,
        &format!("{meshed_backbone} Backbone gates with mesh addresses"),
    );

    v.section("Phase 3: Cross-zone mesh reachability");

    let wan_meshed = gates
        .iter()
        .filter(|g| CytoplasmZone::for_gate(&g.name) == CytoplasmZone::Wan && !g.address.is_empty())
        .count();
    v.check_bool(
        "wan:meshed_gates",
        wan_meshed >= 1,
        &format!("{wan_meshed} WAN-zone gates peered in mesh overlay"),
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
