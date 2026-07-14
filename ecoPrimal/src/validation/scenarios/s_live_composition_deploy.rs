// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Live Composition Deploy — validates deployment and lifecycle contracts
//! for live composition rollout via biomeOS.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const META_DEPLOY_GRAPH: &str = include_str!("../../../../graphs/compositions/meta_deploy.toml");

/// Live composition deploy scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "live-composition-deploy",
        track: Track::BiomeosDeploy,
        tier: Tier::Both,
        provenance_crate: "wave138a_live_composition_deploy",
        provenance_date: "2026-07-14",
        description: "Live composition deploy — graph.deploy + lifecycle methods registered and routable",
    },
    run,
};

/// Run live composition deploy validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Deploy methods in capability registry");

    let deploy_methods = ["graph.deploy", "graph.execute", "coordination.deploy_atomic"];
    for method in deploy_methods {
        v.check_bool(
            &format!("registry:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered in capability_registry.toml"),
        );
    }

    v.check_bool(
        "registry:meta_deploy_composition",
        REGISTRY_TOML.contains("meta_deploy.toml"),
        "meta-tier deploy composition declared in registry",
    );

    v.section("Phase 2: Lifecycle methods in capability registry");

    let lifecycle_methods = [
        "lifecycle.register",
        "lifecycle.start",
        "lifecycle.stop",
        "lifecycle.status",
    ];
    for method in lifecycle_methods {
        v.check_bool(
            &format!("lifecycle:{}", method.replace("lifecycle.", "")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    v.section("Phase 3: Routing — deploy and lifecycle owners");

    let table = canonical_routing_table();
    if let Some(entry) = table.route("graph.deploy") {
        v.check_bool(
            "routing:graph_deploy_owner",
            &*entry.owner == primal_names::BIOMEOS,
            &format!("graph.deploy → {} (expected biomeOS)", entry.owner),
        );
    }
    if let Some(entry) = table.route("lifecycle.start") {
        v.check_bool(
            "routing:lifecycle_owner",
            &*entry.owner == primal_names::BIOMEOS,
            &format!("lifecycle.start → {} (expected biomeOS)", entry.owner),
        );
    }

    v.section("Phase 4: Deploy graph structure");

    v.check_bool(
        "graph:meta_deploy_parses",
        toml::from_str::<toml::Value>(META_DEPLOY_GRAPH).is_ok(),
        "meta_deploy.toml parses as valid TOML",
    );
    v.check_bool(
        "graph:meta_deploy_has_graph_deploy",
        META_DEPLOY_GRAPH.contains("graph.deploy"),
        "meta_deploy graph references graph.deploy capability",
    );

    v.section("Phase 5: Live — orchestration availability");
    if ctx.has_capability("orchestration") {
        v.check_bool(
            "live:orchestration_available",
            true,
            "biomeOS orchestration capability discovered for live deploy",
        );
    } else {
        v.check_skip(
            "live:orchestration_available",
            "orchestration not discovered — live deploy phase skipped",
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
