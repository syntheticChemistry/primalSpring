// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Topology Visualization — validates mesh topology has sufficient gate
//! data for meaningful visualization.

use crate::composition::CompositionContext;
use crate::evolution::{all_mesh_gates, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Topology visualization scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "topology-visualization",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "wave138a_topology_visualization",
        provenance_date: "2026-07-14",
        description: "Topology visualization — meshed gates with resolvable mesh addresses",
    },
    run,
};

/// Run topology visualization validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Gate count for visualization");

    let gates = all_mesh_gates();
    let peered: Vec<_> = gates.iter().filter(|g| !g.address.is_empty()).collect();

    v.check_bool(
        "viz:minimum_gates",
        peered.len() >= 4,
        &format!("{} peered gates (need ≥4 for meaningful visualization)", peered.len()),
    );

    v.check_bool(
        "viz:mesh_subnet_declared",
        MESH_TOML.contains("subnet = \"10.13.37.0/24\""),
        "mesh_topology.toml declares 10.13.37.0/24 subnet for viz layout",
    );

    v.section("Phase 2: Per-gate mesh_address resolution");

    let mut resolved = 0;
    for gate in &peered {
        let addr = mesh_address(&gate.name);
        let ok = addr.is_some_and(|a| !a.is_empty());
        if ok {
            resolved += 1;
        }
        v.check_bool(
            &format!("viz:{}:mesh_address", gate.name.to_lowercase()),
            ok,
            &format!(
                "mesh_address(\"{}\") = {:?}",
                gate.name,
                addr.unwrap_or("MISSING")
            ),
        );
    }

    v.check_bool(
        "viz:all_addresses_resolved",
        resolved == peered.len(),
        &format!("{resolved}/{} peered gates have mesh_address lookups", peered.len()),
    );

    v.section("Phase 3: Role diversity for viz layers");

    let roles: std::collections::HashSet<_> = peered.iter().map(|g| g.role.as_str()).collect();
    v.check_bool(
        "viz:role_diversity",
        roles.len() >= 2,
        &format!("{} distinct gate roles for visualization layers: {roles:?}", roles.len()),
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
