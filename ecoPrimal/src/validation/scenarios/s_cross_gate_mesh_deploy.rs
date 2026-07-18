// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cross Gate Mesh Deploy — validates cross-gate mesh deployment routing
//! and topology support for inter-gate deploy paths.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::evolution::gate::{CytoplasmZone, all_mesh_gates, mesh_address};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Cross gate mesh deploy scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-gate-mesh-deploy",
        track: Track::BiomeosDeploy,
        tier: Tier::Both,
        provenance_crate: "wave138a_cross_gate_mesh_deploy",
        provenance_date: "2026-07-14",
        description: "Cross-gate mesh deploy — graph.deploy routing and cross-zone mesh topology",
    },
    run,
};

/// Run cross gate mesh deploy validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Deploy methods route correctly");

    let deploy_methods = [
        "graph.deploy",
        "graph.execute",
        "coordination.deploy_atomic",
    ];
    let table = canonical_routing_table();

    for method in deploy_methods {
        let present = REGISTRY_TOML.contains(method);
        v.check_bool(
            &format!("deploy:{}", method.replace('.', "_")),
            present,
            &format!("{method} registered"),
        );
        if let Some(entry) = table.route(method) {
            v.check_bool(
                &format!("routing:{}", method.replace('.', "_")),
                &*entry.owner == primal_names::BIOMEOS || &*entry.owner == "primalspring",
                &format!("{method} → {} (biomeOS or primalSpring)", entry.owner),
            );
        }
    }

    v.section("Phase 2: Cross-gate mesh routing topology");

    let backbone_addr = mesh_address("eastGate");
    let wan_addr = mesh_address("flockGate");
    v.check_bool(
        "topo:backbone_gate_addressable",
        backbone_addr.is_some(),
        &format!("eastGate (Backbone) mesh address: {backbone_addr:?}"),
    );
    v.check_bool(
        "topo:wan_gate_addressable",
        wan_addr.is_some(),
        &format!("flockGate (WAN) mesh address: {wan_addr:?}"),
    );

    let cross_zone = CytoplasmZone::for_gate("eastGate") != CytoplasmZone::for_gate("flockGate");
    v.check_bool(
        "topo:cross_zone_routing",
        cross_zone,
        "eastGate (Backbone) and flockGate (WAN) are cross-zone deploy targets",
    );

    let peered = all_mesh_gates()
        .iter()
        .filter(|g| !g.address.is_empty())
        .count();
    v.check_bool(
        "topo:multi_gate_mesh",
        peered >= 4,
        &format!("{peered} meshed gates support cross-gate deploy routing"),
    );

    v.section("Phase 3: Cross-gate sync compositions");

    v.check_bool(
        "deploy:sync_resolve_crossgate",
        REGISTRY_TOML.contains("sync.resolve.crossgate"),
        "sync.resolve.crossgate composition registered for cross-gate deploy",
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
