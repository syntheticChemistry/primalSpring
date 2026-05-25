// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Coordination API surface — exercises primalSpring's own
//! coordination and neural_api methods (Wave 47 method coverage push).

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

fn phase_live_coordination(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:coordination",
            "orchestration not available — skipping live coordination checks",
        );
        return;
    }

    match ctx.call(
        "orchestration",
        "coordination.neural_api_status",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_total = resp.get("total_methods").is_some();
            v.check_bool(
                "live:neural_api_status",
                has_total,
                &format!("coordination.neural_api_status → total_methods present: {has_total}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:neural_api_status", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "live:neural_api_status",
                false,
                &format!("coordination.neural_api_status error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "coordination.probe_capability",
        serde_json::json!({ "capability": "security" }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:probe_capability",
                resp.is_object(),
                "coordination.probe_capability(security) responded",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:probe_capability", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "live:probe_capability",
                false,
                &format!("coordination.probe_capability error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "coordination.probe_primal",
        serde_json::json!({ "primal": "beardog" }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:probe_primal",
                resp.is_object(),
                "coordination.probe_primal(beardog) responded",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:probe_primal", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "live:probe_primal",
                false,
                &format!("coordination.probe_primal error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "coordination.validate_composition_by_capability",
        serde_json::json!({ "atomic": "Tower" }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:validate_by_capability",
                resp.is_object(),
                "coordination.validate_composition_by_capability(Tower) responded",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:validate_by_capability", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "live:validate_by_capability",
                false,
                &format!("error: {e}"),
            );
        }
    }

    match ctx.call(
        "orchestration",
        "coordination.bonding_test",
        serde_json::json!({ "bond_type": "covalent" }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:bonding_test",
                resp.is_object(),
                "coordination.bonding_test(covalent) responded",
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("live:bonding_test", &format!("not reachable: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "live:bonding_test",
                false,
                &format!("coordination.bonding_test error: {e}"),
            );
        }
    }
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
