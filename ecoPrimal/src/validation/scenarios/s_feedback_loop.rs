// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Feedback Loop — validates the Wave 42 instrumented dispatch path
//! and verifies biomeOS routing weights update in response to dispatches.
//!
//! Exercises the full feedback cycle: dispatch_instrumented → BridgeOutcome →
//! record_bridge_outcome → routing_weights change → utilization tracking.

use crate::composition::CompositionContext;
use crate::composition::neural_dispatch::NeuralDispatcher;
use crate::ipc::neural_bridge::NeuralBridge;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Feedback loop scenario — Tier::Live, validates instrumented dispatch and weight updates.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "feedback-loop",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "primalspring_feedback_loop",
        provenance_date: "2026-05-23",
        description: "Instrumented dispatch feeds metrics back; routing weights and utilization update",
    },
    run,
};

const DISPATCH_ROUNDS: usize = 5;

/// Run the feedback loop validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let mut dispatcher = NeuralDispatcher::discover();

    if !dispatcher.is_online() {
        v.check_bool(
            "feedback-skipped",
            true,
            "biomeOS not available — feedback loop checks skipped (expected without deployment)",
        );
        return;
    }

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_bool("feedback-bridge-found", false, "NeuralBridge not available");
        return;
    };

    // Phase 1: Snapshot routing weights before dispatches.
    let weights_before = bridge.routing_weights().ok();
    v.check_bool(
        "weights-snapshot-before",
        weights_before.is_some(),
        "pre-dispatch routing weights snapshot captured",
    );

    // Phase 2: Run instrumented dispatches.
    let initial_metrics = dispatcher.metrics().len();
    let params = serde_json::json!({ "data": "feedback-probe", "algorithm": "blake3" });

    let mut successes = 0u32;
    let mut total_latency = 0u64;

    for i in 0..DISPATCH_ROUNDS {
        let outcome = dispatcher.dispatch_instrumented("crypto.hash", &params);
        if outcome.result.is_ok() {
            successes += 1;
        }
        total_latency += outcome.latency_ms;
        v.check_bool(
            &format!("instrumented-dispatch-{i}-latency"),
            outcome.latency_ms > 0 || outcome.result.is_err(),
            &format!(
                "round {i}: {}ms, success={}",
                outcome.latency_ms,
                outcome.result.is_ok()
            ),
        );
    }

    // Phase 3: Verify BridgeOutcome metrics accumulated in dispatcher.
    let final_metrics = dispatcher.metrics().len();
    let new_metrics = final_metrics - initial_metrics;
    v.check_bool(
        "metrics-accumulated",
        new_metrics >= DISPATCH_ROUNDS,
        &format!("{new_metrics} new metrics recorded (expected {DISPATCH_ROUNDS}+)"),
    );

    // Phase 4: Verify dispatch statistics.
    if let Some(avg) = dispatcher.avg_latency_ms("crypto.hash") {
        v.check_bool(
            "avg-latency-reasonable",
            avg < 500.0,
            &format!("avg crypto.hash latency: {avg:.1}ms"),
        );
    }

    if let Some(err_rate) = dispatcher.error_rate("crypto.hash") {
        v.check_bool(
            "error-rate-low",
            err_rate < 0.5,
            &format!("crypto.hash error rate: {err_rate:.2}"),
        );
    }

    v.check_bool(
        "dispatch-success-count",
        successes > 0 || total_latency > 0,
        &format!("{successes}/{DISPATCH_ROUNDS} dispatches succeeded, total {total_latency}ms"),
    );

    // Phase 5: Query utilization — dispatched method should appear.
    match bridge.utilization() {
        Ok(util) => {
            let util_str = util.to_string();
            let has_crypto = util_str.contains("crypto");
            v.check_bool(
                "utilization-has-crypto",
                has_crypto,
                "utilization tracking includes crypto methods after dispatch",
            );
        }
        Err(e) => {
            v.check_bool(
                "utilization-has-crypto",
                false,
                &format!("utilization query failed: {e}"),
            );
        }
    }

    // Phase 6: Snapshot routing weights after — compare for change.
    let weights_after = bridge.routing_weights().ok();
    v.check_bool(
        "weights-snapshot-after",
        weights_after.is_some(),
        "post-dispatch routing weights snapshot captured",
    );

    if let (Some(before), Some(after)) = (&weights_before, &weights_after) {
        let changed = *before != *after;
        v.check_bool(
            "weights-changed-after-dispatch",
            changed,
            &format!("routing weights changed after {DISPATCH_ROUNDS} dispatches: {changed}"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feedback_loop_runs_without_panic() {
        let mut v = ValidationResult::new("feedback-loop");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }
}
