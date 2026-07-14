// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Neural API Lifecycle — validates neural API lifecycle methods and
//! biomeOS ownership of lifecycle operations.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Neural API lifecycle scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "neural-api-lifecycle",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "wave138a_neural_api_lifecycle",
        provenance_date: "2026-07-14",
        description: "Neural API lifecycle — lifecycle.start/stop and biomeOS ownership",
    },
    run,
};

/// Run neural API lifecycle validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Lifecycle methods in registry");

    let lifecycle_methods = ["lifecycle.start", "lifecycle.stop", "lifecycle.status"];
    for method in lifecycle_methods {
        v.check_bool(
            &format!("lifecycle:{}", method.replace("lifecycle.", "")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    let has_resurrect = REGISTRY_TOML.contains("lifecycle.resurrect")
        || REGISTRY_TOML.contains("composition.reload")
        || REGISTRY_TOML.contains("nucleus.start");
    v.check_bool(
        "lifecycle:resurrect_semantics",
        has_resurrect,
        "lifecycle.resurrect, composition.reload, or nucleus.start provides resurrection path",
    );

    v.section("Phase 2: Neural API lifecycle surface");

    let neural_methods = [
        "neural_api.routing_weights",
        "neural_api.plan_tier",
        "biomeos.lifecycle.status",
    ];
    for method in neural_methods {
        v.check_bool(
            &format!("neural:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    v.section("Phase 3: biomeOS owns lifecycle methods");

    let table = canonical_routing_table();
    for method in ["lifecycle.start", "lifecycle.stop", "lifecycle.register"] {
        if let Some(entry) = table.route(method) {
            v.check_bool(
                &format!("owner:{}", method.replace('.', "_")),
                &*entry.owner == primal_names::BIOMEOS,
                &format!("{method} → {} (expected biomeOS)", entry.owner),
            );
        }
    }

    v.section("Phase 4: Live — lifecycle status probe");
    if ctx.has_capability("orchestration") {
        match ctx.call("orchestration", "lifecycle.status", serde_json::json!({})) {
            Ok(resp) => v.check_bool(
                "live:lifecycle_status",
                resp.is_object() || resp.is_string() || resp.is_null(),
                "lifecycle.status responds via orchestration",
            ),
            Err(e) if e.is_skippable() => {
                v.check_skip("live:lifecycle_status", &format!("{e}"));
            }
            Err(e) => v.check_bool("live:lifecycle_status", false, &format!("{e}")),
        }
    } else {
        v.check_skip("live:lifecycle_status", "orchestration not discovered");
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
