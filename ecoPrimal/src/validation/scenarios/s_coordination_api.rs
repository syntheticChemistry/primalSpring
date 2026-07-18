// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Coordination API surface — exercises primalSpring's own
//! coordination and `neural_api` methods (Wave 47 method coverage push).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "coordination-api",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "wave47_method_coverage",
        provenance_date: "2026-05-24",
        description: "Coordination API surface: probe, validate, neural_api status, bonding_test",
    },
    run,
};

const COORDINATION_METHODS: &[&str] = &[
    "coordination.bonding_test",
    "coordination.neural_api_status",
    "coordination.probe_capability",
    "coordination.probe_primal",
    "coordination.status",
    "coordination.validate_composition_by_capability",
];

const NEURAL_API_METHODS: &[&str] = &[
    "neural_api.composition_patterns",
    "neural_api.plan_tier",
    "neural_api.route_explain",
    "neural_api.routing_weights",
    "neural_api.utilization",
    "neural_api.weight_health",
    "primal.capabilities",
];

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — method registry presence");
    phase_structural(v);

    v.section("Phase 2: Live coordination dispatch");
    phase_live_coordination(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let registry_src = include_str!("../../../../config/capability_registry.toml");

    for method in COORDINATION_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in capability_registry.toml"),
        );
    }

    for method in NEURAL_API_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in capability_registry.toml"),
        );
    }
}

fn probe_coordination_method(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    check_id: &str,
    method: &str,
    params: serde_json::Value,
    validate: fn(&serde_json::Value) -> (bool, String),
) {
    match ctx.call("orchestration", method, params) {
        Ok(resp) => {
            let (ok, msg) = validate(&resp);
            v.check_bool(check_id, ok, &msg);
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(check_id, &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(check_id, false, &format!("{method} error: {e}"));
        }
    }
}

fn phase_live_coordination(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:coordination",
            "orchestration not available — skipping live coordination checks",
        );
        return;
    }

    probe_coordination_method(
        v,
        ctx,
        "live:neural_api_status",
        "coordination.neural_api_status",
        serde_json::json!({}),
        |resp| {
            let has_total = resp.get("total_methods").is_some();
            (
                has_total,
                format!("coordination.neural_api_status → total_methods present: {has_total}"),
            )
        },
    );

    probe_coordination_method(
        v,
        ctx,
        "live:probe_capability",
        "coordination.probe_capability",
        serde_json::json!({ "capability": "security" }),
        |resp| {
            (
                resp.is_object(),
                "coordination.probe_capability(security) responded".into(),
            )
        },
    );

    probe_coordination_method(
        v,
        ctx,
        "live:probe_primal",
        "coordination.probe_primal",
        serde_json::json!({ "primal": "beardog" }),
        |resp| {
            (
                resp.is_object(),
                "coordination.probe_primal(beardog) responded".into(),
            )
        },
    );

    probe_coordination_method(
        v,
        ctx,
        "live:validate_by_capability",
        "coordination.validate_composition_by_capability",
        serde_json::json!({ "atomic": "Tower" }),
        |resp| {
            (
                resp.is_object(),
                "coordination.validate_composition_by_capability(Tower) responded".into(),
            )
        },
    );

    probe_coordination_method(
        v,
        ctx,
        "live:bonding_test",
        "coordination.bonding_test",
        serde_json::json!({ "bond_type": "covalent" }),
        |resp| {
            (
                resp.is_object(),
                "coordination.bonding_test(covalent) responded".into(),
            )
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordination_api_pass() {
        let mut v = ValidationResult::new("coordination-api");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "coordination-api had {} failures", v.failed);
    }
}
