// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cross Membrane Data Flow — validates cross-membrane data flow control
//! via membrane-owned methods with ingress/egress semantics.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Cross membrane data flow scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-membrane-data-flow",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave138a_cross_membrane_data_flow",
        provenance_date: "2026-07-14",
        description: "Cross-membrane data flow — membrane methods and ingress/egress semantics",
    },
    run,
};

/// Run cross membrane data flow validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Membrane-owned methods");

    let membrane_methods = [
        "impulse.post",
        "impulse.read",
        "potential.sense",
        "temporal.sync",
        "git.push",
    ];
    let mut membrane_count = 0;
    for method in membrane_methods {
        let present = REGISTRY_TOML.contains(method);
        if present {
            membrane_count += 1;
        }
        v.check_bool(
            &format!("membrane:{}", method.replace('.', "_")),
            present,
            &format!("{method} registered (membrane domain)"),
        );
    }

    v.check_bool(
        "membrane:method_breadth",
        membrane_count >= 4,
        &format!(
            "{membrane_count}/{} membrane coordination methods present",
            membrane_methods.len()
        ),
    );

    let table = canonical_routing_table();
    if let Some(entry) = table.route("impulse.post") {
        v.check_bool(
            "membrane:impulse_owner",
            &*entry.owner == "membrane",
            &format!("impulse.post → {} (expected membrane)", entry.owner),
        );
    }

    v.section("Phase 2: Ingress/egress data flow semantics");

    let ingress_methods = ["content.put", "storage.store", "content.push", "git.push"];
    let egress_methods = [
        "content.get",
        "storage.fetch",
        "content.fetch_heads",
        "impulse.read",
    ];

    let ingress_ok = ingress_methods
        .iter()
        .filter(|m| REGISTRY_TOML.contains(**m))
        .count();
    let egress_ok = egress_methods
        .iter()
        .filter(|m| REGISTRY_TOML.contains(**m))
        .count();

    v.check_bool(
        "flow:ingress_methods",
        ingress_ok >= 3,
        &format!(
            "{ingress_ok}/{} ingress methods registered",
            ingress_methods.len()
        ),
    );
    v.check_bool(
        "flow:egress_methods",
        egress_ok >= 3,
        &format!(
            "{egress_ok}/{} egress methods registered",
            egress_methods.len()
        ),
    );
    v.check_bool(
        "flow:bidirectional",
        ingress_ok >= 2 && egress_ok >= 2,
        "cross-membrane flow has both ingress and egress semantics",
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
