// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Auto Distribution — validates automatic mesh distribution and sync
//! semantics across gates.

use crate::composition::CompositionContext;
use crate::evolution::all_mesh_gates;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Mesh auto distribution scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-auto-distribution",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave138a_mesh_auto_distribution",
        provenance_date: "2026-07-14",
        description: "Mesh auto distribution — mesh sync/mirror methods and multi-gate auto-distribute topology",
    },
    run,
};

/// Run mesh auto distribution validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Distribution methods in registry");

    let has_distribute = REGISTRY_TOML.contains("mesh.distribute")
        || REGISTRY_TOML.contains("mesh.sync")
        || REGISTRY_TOML.contains("mesh.mirror")
        || REGISTRY_TOML.contains("mesh.auto_discover");
    v.check_bool(
        "mesh:distribute_or_sync",
        has_distribute,
        "mesh.distribute, mesh.sync, mesh.mirror, or mesh.auto_discover registered",
    );

    v.check_bool(
        "mesh:auto_discover",
        REGISTRY_TOML.contains("mesh.auto_discover"),
        "mesh.auto_discover registered for automatic peer discovery",
    );
    v.check_bool(
        "mesh:capabilities_announce",
        REGISTRY_TOML.contains("mesh.capabilities_announce"),
        "mesh.capabilities_announce registered for capability propagation",
    );

    v.section("Phase 2: Auto-distribute topology semantics");

    let has_auto_semantics = MESH_TOML.contains("mesh.auto_discover")
        || MESH_TOML.contains("auto_reconnect")
        || REGISTRY_TOML.contains("sync.diverge")
        || REGISTRY_TOML.contains("ecosystem.pull");
    v.check_bool(
        "topo:auto_distribute_semantics",
        has_auto_semantics || all_mesh_gates().len() >= 4,
        "mesh topology supports multi-gate auto-distribution (≥4 gates or sync compositions)",
    );

    let peered = all_mesh_gates()
        .iter()
        .filter(|g| !g.address.is_empty())
        .count();
    v.check_bool(
        "topo:multi_gate_peered",
        peered >= 3,
        &format!("{peered} meshed gates for automatic distribution"),
    );

    v.section("Phase 3: Live — mesh client availability");
    if let Some(client) = ctx.client_for("mesh") {
        let resp = client.call("health.liveness", serde_json::json!({}));
        match resp {
            Ok(r) => v.check_bool(
                "live:mesh_liveness",
                r.is_success(),
                "Songbird mesh transport live for auto-distribution",
            ),
            Err(e) if e.is_skippable() => {
                v.check_skip("live:mesh_liveness", &format!("{e}"));
            }
            Err(e) => v.check_bool("live:mesh_liveness", false, &format!("{e}")),
        }
    } else {
        v.check_skip("live:mesh_liveness", "no mesh client discovered");
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
