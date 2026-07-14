// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Composition Subtypes — validates Tower, Node, Nest, and Meta tiers
//! are registered with representative composition graphs and scenarios.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

const TIER_SCENARIOS: &[(&str, &str)] = &[
    ("tower", "tower-atomic"),
    ("node", "node-atomic"),
    ("nest", "nest-atomic"),
    ("meta", "meta-tier-compositions"),
];

/// Composition subtypes scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "composition-subtypes",
        track: Track::AtomicComposition,
        tier: Tier::Rust,
        provenance_crate: "wave138a_composition_subtypes",
        provenance_date: "2026-07-14",
        description: "Composition subtypes — Tower/Node/Nest/Meta tiers registered with representative scenarios",
    },
    run,
};

/// Run composition subtypes validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Composition tier registry sections");

    for tier in ["tower", "node", "nest", "meta"] {
        let section = format!("[compositions.{tier}]");
        v.check_bool(
            &format!("registry:{tier}_section"),
            REGISTRY_TOML.contains(&section),
            &format!("{section} declared in capability_registry.toml"),
        );
    }

    v.section("Phase 2: Representative composition graphs per tier");

    let tier_graphs = [
        ("tower", "tower_publish.toml"),
        ("node", "node_compute.toml"),
        ("nest", "nest_store.toml"),
        ("meta", "meta_deploy.toml"),
    ];
    for (tier, graph) in tier_graphs {
        v.check_bool(
            &format!("graph:{tier}_representative"),
            REGISTRY_TOML.contains(graph),
            &format!("{tier} tier references {graph}"),
        );
    }

    v.section("Phase 3: Representative scenario coverage");

    let registry = super::build_registry();
    for (tier, scenario_id) in TIER_SCENARIOS {
        let found = registry.all().iter().any(|s| s.meta.id == *scenario_id);
        v.check_bool(
            &format!("scenario:{tier}_representative"),
            found,
            &format!("{tier} tier has representative scenario '{scenario_id}'"),
        );
    }

    v.section("Phase 4: Tier primal rosters");

    let tier_primals = [
        ("tower", "skunkbat"),
        ("node", "toadstool"),
        ("nest", "nestgate"),
        ("meta", "biomeos"),
    ];
    for (tier, expected_primal) in tier_primals {
        let section_marker = format!("[compositions.{tier}]");
        let section_start = REGISTRY_TOML.find(&section_marker);
        let has_primal = section_start.is_some_and(|start| {
            let slice = &REGISTRY_TOML[start..];
            slice.contains(expected_primal)
        });
        v.check_bool(
            &format!("tier:{tier}_primals"),
            has_primal,
            &format!("{tier} tier roster includes {expected_primal}"),
        );
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
