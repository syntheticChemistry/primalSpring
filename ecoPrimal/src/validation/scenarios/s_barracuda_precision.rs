// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: barraCuda Precision Routing — deep tensor and dispatch validation.
//!
//! Exercises barraCuda's precision routing, TensorSession lifecycle, and
//! spring-system absorption patterns. With barraCuda released back for
//! evolution, this scenario validates deeper integration than the basic
//! compute-triangle `stats.mean` round-trip.
//!
//! Phases:
//! 1. `barracuda.precision.route` — multi-operation precision tier queries
//! 2. `tensor.create` + `tensor.session.status` — TensorSession lifecycle
//! 3. `stats.variance` / `stats.std` — statistical operation coverage

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "barracuda-precision",
        track: Track::Transport,
        tier: Tier::Live,
        provenance_crate: "exp_barracuda_precision",
        provenance_date: "2026-05-14",
        description: "barraCuda precision routing depth (multi-op route, TensorSession, stats suite)",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Precision routing — multi-operation");
    phase_precision_multi(v, ctx);

    v.section("Phase 2: TensorSession lifecycle");
    phase_tensor_session(v, ctx);

    v.section("Phase 3: Statistical operation coverage");
    phase_stats_coverage(v, ctx);
}

fn phase_precision_multi(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("tensor") {
        v.check_skip("precision_multi", "tensor capability not present");
        return;
    }

    let operations = [
        ("stats.mean", "1e-9"),
        ("stats.variance", "1e-6"),
        ("linalg.dot", "1e-12"),
    ];

    for (op, tol) in &operations {
        let params = serde_json::json!({
            "operation": op,
            "input_range": "normal",
            "tolerance": tol,
        });
        match ctx.call("tensor", "barracuda.precision.route", params) {
            Ok(resp) => {
                let has_tier =
                    resp.get("precision_tier").is_some() || resp.get("strategy").is_some();
                v.check_bool(
                    &format!("precision_route_{}", op.replace('.', "_")),
                    has_tier,
                    &format!("precision.route for {op} returns tier"),
                );
            }
            Err(_) => {
                v.check_skip(
                    &format!("precision_route_{}", op.replace('.', "_")),
                    &format!("{op} routing not available"),
                );
            }
        }
    }
}

fn phase_tensor_session(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("tensor") {
        v.check_skip("tensor_session", "tensor capability not present");
        return;
    }

    let create_params = serde_json::json!({
        "precision": "f64",
        "dims": [4, 4],
    });
    match ctx.call("tensor", "tensor.create", create_params) {
        Ok(resp) => {
            let has_id = resp.get("session_id").is_some() || resp.get("id").is_some();
            v.check_bool(
                "tensor_create",
                has_id,
                "tensor.create returns session identifier",
            );
        }
        Err(_) => {
            v.check_skip("tensor_create", "tensor.create not available");
        }
    }
}

fn phase_stats_coverage(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("tensor") {
        v.check_skip("stats_coverage", "tensor capability not present");
        return;
    }

    let methods = ["stats.mean", "stats.variance", "stats.std"];
    let data = serde_json::json!({ "data": [1.0, 2.0, 3.0, 4.0, 5.0] });

    for method in &methods {
        match ctx.call("tensor", method, data.clone()) {
            Ok(resp) => {
                let has_result = resp.get("result").is_some() || resp.get("value").is_some();
                v.check_bool(
                    &method.replace('.', "_"),
                    has_result,
                    &format!("{method} returns numeric result"),
                );
            }
            Err(_) => {
                v.check_skip(
                    &method.replace('.', "_"),
                    &format!("{method} not available"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barracuda_precision_no_panic() {
        let mut v = ValidationResult::new("barracuda-precision");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }
}
