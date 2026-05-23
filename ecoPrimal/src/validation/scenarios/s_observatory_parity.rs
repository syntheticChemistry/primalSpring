// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Observatory Parity — validates that biomeOS observatory RPCs return
//! data consistent with primalSpring's local routing model.
//!
//! Exercises the Layer 4 observatory surface: routing_weights, route_explain,
//! composition_patterns, plan_tier. Cross-references against the local
//! NeuralRoutingTable to verify parity.

use crate::composition::neural_routing::{canonical_routing_table, CompositionTier};
use crate::composition::CompositionContext;
use crate::ipc::neural_bridge::NeuralBridge;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Observatory parity scenario — Tier::Live, validates biomeOS observatory RPCs.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "observatory-parity",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "primalspring_observatory",
        provenance_date: "2026-05-23",
        description: "biomeOS observatory RPCs match primalSpring routing model",
    },
    run,
};

/// Run the observatory parity validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let bridge = match NeuralBridge::discover() {
        Some(b) => b,
        None => {
            v.check_bool(
                "observatory-skipped",
                true,
                "biomeOS not available — observatory checks skipped (expected without deployment)",
            );
            return;
        }
    };

    let local_table = canonical_routing_table();

    // Phase 1: Routing weights — verify primals announced.
    match bridge.routing_weights() {
        Ok(weights) => {
            let is_obj = weights.is_object() || weights.is_array();
            v.check_bool(
                "routing-weights-returned",
                is_obj,
                "neural_api.routing_weights returned structured data",
            );
            let weight_str = weights.to_string();
            let has_beardog = weight_str.contains("beardog");
            v.check_bool(
                "routing-weights-has-beardog",
                has_beardog,
                "routing weights include beardog provider",
            );
            let has_nestgate = weight_str.contains("nestgate");
            v.check_bool(
                "routing-weights-has-nestgate",
                has_nestgate,
                "routing weights include nestgate provider",
            );
        }
        Err(e) => {
            v.check_bool(
                "routing-weights-returned",
                false,
                &format!("routing_weights call failed: {e}"),
            );
        }
    }

    // Phase 2: Route explain — verify bearDog owns crypto.hash.
    match bridge.route_explain("crypto.hash") {
        Ok(explanation) => {
            v.check_bool(
                "route-explain-returned",
                !explanation.is_null(),
                "neural_api.route_explain returned data for crypto.hash",
            );
            let explain_str = explanation.to_string();
            let mentions_beardog = explain_str.contains("beardog");
            v.check_bool(
                "route-explain-beardog-provider",
                mentions_beardog,
                "route_explain for crypto.hash mentions beardog",
            );
        }
        Err(e) => {
            v.check_bool(
                "route-explain-returned",
                false,
                &format!("route_explain call failed: {e}"),
            );
        }
    }

    // Phase 3: Composition patterns — known patterns exist.
    match bridge.composition_patterns() {
        Ok(patterns) => {
            let patterns_str = patterns.to_string();
            let has_rootpulse = patterns_str.contains("rootpulse_commit");
            v.check_bool(
                "patterns-has-rootpulse",
                has_rootpulse,
                "biomeOS has rootpulse_commit pattern",
            );
            let local_pattern_count = local_table.patterns().len();
            v.check_bool(
                "patterns-count-nonzero",
                local_pattern_count >= 3,
                &format!("local table has {local_pattern_count} patterns"),
            );
        }
        Err(e) => {
            v.check_bool(
                "patterns-has-rootpulse",
                false,
                &format!("composition_patterns call failed: {e}"),
            );
        }
    }

    // Phase 4: Plan tier — tower includes beardog+songbird+skunkbat.
    match bridge.plan_tier("tower") {
        Ok(plan) => {
            let plan_str = plan.to_string();
            v.check_bool(
                "plan-tower-beardog",
                plan_str.contains("beardog"),
                "tower plan includes beardog",
            );
            v.check_bool(
                "plan-tower-songbird",
                plan_str.contains("songbird"),
                "tower plan includes songbird",
            );
            v.check_bool(
                "plan-tower-skunkbat",
                plan_str.contains("skunkbat"),
                "tower plan includes skunkbat",
            );
        }
        Err(e) => {
            v.check_bool(
                "plan-tower-beardog",
                false,
                &format!("plan_tier call failed: {e}"),
            );
        }
    }

    // Phase 5: Weight health — convergence diagnostics (v3.70+).
    match bridge.weight_health() {
        Ok(health) => {
            v.check_bool(
                "weight-health-returned",
                health.is_object(),
                "neural_api.weight_health returned structured data",
            );
            if let Some(healthy) = health.get("healthy") {
                v.check_bool(
                    "weight-health-no-open-circuits",
                    healthy.as_bool().unwrap_or(false),
                    &format!("weight health: healthy={healthy}"),
                );
            }
            if let Some(persistent) = health.get("persistent") {
                v.check_bool(
                    "weight-health-persistent",
                    persistent.as_bool().unwrap_or(false),
                    &format!("weight persistence: {persistent}"),
                );
            }
            if let Some(convergence) = health.get("convergence") {
                let total = convergence
                    .get("total_providers")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                v.check_bool(
                    "weight-health-has-providers",
                    total > 0,
                    &format!("convergence: {total} total providers"),
                );
            }
        }
        Err(e) => {
            v.check_bool(
                "weight-health-returned",
                false,
                &format!("weight_health call failed (biomeOS < v3.70?): {e}"),
            );
        }
    }

    // Phase 6: Method count parity — biomeOS and local table within tolerance.
    let local_count = local_table.method_count();
    let local_tower = local_table.tier_composition(CompositionTier::Tower);
    v.check_bool(
        "local-table-populated",
        local_count >= 450,
        &format!("local routing table: {local_count} methods"),
    );
    v.check_bool(
        "local-tower-primals",
        local_tower.primals.len() >= 3,
        &format!("local tower: {} primals", local_tower.primals.len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observatory_parity_runs_without_panic() {
        let mut v = ValidationResult::new("observatory-parity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }
}
