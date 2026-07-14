// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Federation Readiness — validates federation methods and
//! WireGuard mesh WAN peer configuration.

use crate::composition::CompositionContext;
use crate::evolution::gate::{CytoplasmZone, all_mesh_gates};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Mesh federation readiness scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-federation-readiness",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave138a_mesh_federation_readiness",
        provenance_date: "2026-07-14",
        description: "Mesh federation readiness — federation methods and WireGuard WAN peers",
    },
    run,
};

/// Run mesh federation readiness validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Federation methods in registry");

    let federation_methods = [
        "federation.configure",
        "federation.join",
        "federation.health",
        "songbird.federation.peers",
    ];
    for method in federation_methods {
        v.check_bool(
            &format!("federation:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    v.section("Phase 2: WireGuard mesh WAN peers");

    v.check_bool(
        "wg:subnet_declared",
        MESH_TOML.contains("subnet = \"10.13.37.0/24\""),
        "mesh_topology.toml declares WireGuard subnet",
    );
    v.check_bool(
        "wg:interface_declared",
        MESH_TOML.contains("interface = \"wg0\""),
        "mesh_topology.toml declares wg0 interface",
    );

    let wan_peers: Vec<_> = all_mesh_gates()
        .iter()
        .filter(|g| {
            CytoplasmZone::for_gate(&g.name) == CytoplasmZone::Wan && !g.address.is_empty()
        })
        .collect();

    v.check_bool(
        "wg:wan_peers",
        !wan_peers.is_empty(),
        &format!(
            "{} WAN WireGuard peers: {:?}",
            wan_peers.len(),
            wan_peers.iter().map(|g| (&g.name, &g.address)).collect::<Vec<_>>()
        ),
    );

    v.section("Phase 3: Live — federation health probe");
    if let Some(client) = ctx.client_for("orchestration") {
        let resp = client.call("federation.health", serde_json::json!({}));
        match resp {
            Ok(r) => v.check_bool(
                "live:federation_health",
                r.is_success(),
                "federation.health responds",
            ),
            Err(e) if e.is_skippable() => {
                v.check_skip("live:federation_health", &format!("{e}"));
            }
            Err(e) => v.check_bool("live:federation_health", false, &format!("{e}")),
        }
    } else {
        v.check_skip("live:federation_health", "orchestration not discovered");
    }
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
