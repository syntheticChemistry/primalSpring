// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Outer Membrane Posture — validates public-facing drawbridge boundary
//! control and dark forest demarcation methods.

use crate::composition::{CompositionContext, capability_to_primal};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PORTS_TOML: &str = include_str!("../../../../config/ports.toml");

/// Outer membrane posture scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "outer-membrane-posture",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave138a_outer_membrane_posture",
        provenance_date: "2026-07-14",
        description: "Outer membrane posture — drawbridge boundary control and dark forest demarcation",
    },
    run,
};

/// Run outer membrane posture validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Drawbridge boundary control");

    let has_drawbridge =
        REGISTRY_TOML.contains("drawbridge") || REGISTRY_TOML.contains("http.proxy");
    v.check_bool(
        "drawbridge:method_registered",
        has_drawbridge,
        "drawbridge or http.proxy method registered for boundary crossing",
    );

    let http_owner = capability_to_primal("http");
    v.check_bool(
        "drawbridge:songbird_owner",
        http_owner == "songbird",
        &format!("http capability owner: {http_owner} (songBird drawbridge)"),
    );

    v.check_bool(
        "drawbridge:port_registered",
        PORTS_TOML.contains("[gateway.drawbridge]"),
        "ports.toml declares gateway.drawbridge crossing point",
    );

    v.section("Phase 2: Dark forest demarcation");

    let darkforest_methods = [
        "defense.audit",
        "defense.status",
        "security.authenticate",
        "threat.assess",
    ];
    let mut registered = 0;
    for method in darkforest_methods {
        if REGISTRY_TOML.contains(method) {
            registered += 1;
        }
        v.check_bool(
            &format!("darkforest:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered for dark forest demarcation"),
        );
    }

    v.check_bool(
        "darkforest:method_breadth",
        registered >= 3,
        &format!(
            "{registered}/{} dark forest demarcation methods present",
            darkforest_methods.len()
        ),
    );

    v.check_bool(
        "darkforest:mesh_internal_only",
        REGISTRY_TOML.contains("mesh.relay") && REGISTRY_TOML.contains("mesh.peers"),
        "mesh.relay + mesh.peers enforce internal-only dark forest routing",
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
