// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Neural Dispatch Live — validates `NeuralDispatcher.dispatch()` routes
//! through biomeOS to real primals via `capability.call`.
//!
//! Requires plasmidBin-deployed primals and biomeOS neural-api running.

use crate::composition::CompositionContext;
use crate::composition::neural_dispatch::{NeuralDispatcher, RoutePath};
use crate::composition::neural_routing::CompositionTier;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Neural dispatch live scenario — `Tier::Live`, validates dispatch through biomeOS.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "neural-dispatch-live",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "primalspring_neural_dispatch",
        provenance_date: "2026-05-23",
        description: "NeuralDispatcher routes capability.call through biomeOS to live primals",
    },
    run,
};

/// Run the neural dispatch live validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let mut dispatcher = NeuralDispatcher::discover();

    // Phase 1: biomeOS reachability.
    let online = dispatcher.is_online();
    v.check_bool(
        "neural-api-online",
        online,
        &format!("biomeOS neural-api reachable: {online}"),
    );

    if !online {
        v.check_bool(
            "neural-dispatch-skipped",
            true,
            "biomeOS not available — live dispatch checks skipped (expected without deployment)",
        );
        return;
    }

    // Phase 2: Dispatch crypto.hash → bearDog.
    let params = serde_json::json!({ "data": "hello", "algorithm": "blake3" });
    let outcome = dispatcher.dispatch("crypto.hash", &params);
    v.check_bool(
        "dispatch-crypto-hash-owner",
        &*outcome.owner == "beardog",
        &format!("crypto.hash routed to: {}", outcome.owner),
    );
    v.check_bool(
        "dispatch-crypto-hash-tier",
        outcome.tier == CompositionTier::Tower,
        &format!("crypto.hash tier: {:?}", outcome.tier),
    );
    v.check_bool(
        "dispatch-crypto-hash-route",
        outcome.route_path == RoutePath::CapabilityCall,
        &format!("crypto.hash route: {:?}", outcome.route_path),
    );
    v.check_bool(
        "dispatch-crypto-hash-latency",
        outcome.latency_ms < crate::tolerances::SCENARIO_DISPATCH_LATENCY_MAX_MS,
        &format!(
            "crypto.hash latency: {}ms (< {}ms)",
            outcome.latency_ms,
            crate::tolerances::SCENARIO_DISPATCH_LATENCY_MAX_MS,
        ),
    );

    // Phase 3: Dispatch storage.store → nestgate.
    let store_params =
        serde_json::json!({ "key": "validation-probe", "value": "neural-dispatch-live" });
    let store_outcome = dispatcher.dispatch("storage.store", &store_params);
    v.check_bool(
        "dispatch-storage-owner",
        &*store_outcome.owner == "nestgate",
        &format!("storage.store routed to: {}", store_outcome.owner),
    );
    v.check_bool(
        "dispatch-storage-tier",
        store_outcome.tier == CompositionTier::Nest,
        &format!("storage.store tier: {:?}", store_outcome.tier),
    );

    // Phase 4: Dispatch unknown method → graceful error.
    let unknown_outcome = dispatcher.dispatch("nonexistent.method", &serde_json::json!({}));
    v.check_bool(
        "dispatch-unknown-unresolved",
        unknown_outcome.route_path == RoutePath::Unresolved,
        &format!("unknown method route: {:?}", unknown_outcome.route_path),
    );
    v.check_bool(
        "dispatch-unknown-error",
        unknown_outcome.result.is_err(),
        "unknown method returns error",
    );

    // Phase 5: Metrics accumulated.
    let metrics = dispatcher.metrics();
    v.check_bool(
        "dispatch-metrics-accumulated",
        metrics.len() >= 2,
        &format!("{} dispatch metrics recorded", metrics.len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neural_dispatch_live_runs_without_panic() {
        let mut v = ValidationResult::new("neural-dispatch-live");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Live tier: without biomeOS, the scenario should skip gracefully.
        // With biomeOS, all checks should pass.
    }
}
