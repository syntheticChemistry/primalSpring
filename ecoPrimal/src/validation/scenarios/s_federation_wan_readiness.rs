// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Federation WAN Readiness — validates federation port 7700 and
//! WAN-capable gates in the mesh topology.

use crate::composition::CompositionContext;
use crate::evolution::gate::{CytoplasmZone, all_mesh_gates, mesh_address};
use crate::tolerances::ports::FEDERATION_PORT;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PORTS_TOML: &str = include_str!("../../../../config/ports.toml");

/// Federation WAN readiness scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "federation-wan-readiness",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave138a_federation_wan_readiness",
        provenance_date: "2026-07-14",
        description: "Federation WAN readiness — port 7700 and WAN-capable mesh gates",
    },
    run,
};

/// Run federation WAN readiness validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Federation port configuration");

    v.check_bool(
        "federation:port_7700",
        FEDERATION_PORT == 7700,
        &format!("FEDERATION_PORT is {FEDERATION_PORT} (expected 7700)"),
    );
    v.check_bool(
        "federation:ports_toml",
        PORTS_TOML.contains("port = 7700"),
        "ports.toml declares federation port 7700",
    );
    v.check_bool(
        "federation:methods_registered",
        REGISTRY_TOML.contains("federation.join")
            && REGISTRY_TOML.contains("federation.health"),
        "federation.join and federation.health registered",
    );

    v.section("Phase 2: WAN-capable gates in topology");

    let wan_gates: Vec<_> = all_mesh_gates()
        .iter()
        .filter(|g| CytoplasmZone::for_gate(&g.name) == CytoplasmZone::Wan)
        .collect();

    v.check_bool(
        "wan:gates_identified",
        !wan_gates.is_empty(),
        &format!(
            "{} WAN-capable gates: {:?}",
            wan_gates.len(),
            wan_gates.iter().map(|g| &g.name).collect::<Vec<_>>()
        ),
    );

    let wan_meshed: Vec<_> = wan_gates
        .iter()
        .filter(|g| mesh_address(&g.name).is_some())
        .collect();
    v.check_bool(
        "wan:meshed_for_federation",
        !wan_meshed.is_empty(),
        &format!(
            "{} WAN gates with mesh addresses for federation: {:?}",
            wan_meshed.len(),
            wan_meshed.iter().map(|g| &g.name).collect::<Vec<_>>()
        ),
    );

    v.section("Phase 3: Songbird federation surface");

    v.check_bool(
        "federation:songbird_peers",
        REGISTRY_TOML.contains("songbird.federation.peers")
            || REGISTRY_TOML.contains("network.federation.peers"),
        "songBird federation peer methods registered for WAN readiness",
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
