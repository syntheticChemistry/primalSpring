// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Cross Spring Data Flow — absorbed from exp040.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cross-spring-data-flow",
        track: Track::CrossSpring,
        tier: Tier::Live,
        provenance_crate: "exp040_cross_spring_data_flow",
        provenance_date: "2026-05-09",
        description: "Cross spring data flow — ecology routing via composition context",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 2: Routing (skips)");
    phase_routing_skips(v, ctx);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_context_non_empty",
        !caps.is_empty(),
        &format!("capabilities: {}", caps.join(", ")),
    );

    for cap in ["visualization", "ai"] {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} in context"),
        );
    }
}

fn phase_routing_skips(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if ctx.has_capability("orchestration") {
        match ctx.health_check("orchestration") {
            Ok(true) => {
                v.check_bool(
                    "orchestration_reachable",
                    true,
                    "orchestration health.liveness ok",
                );
                v.check_skip(
                    "cross_spring_data_flow",
                    "end-to-end flow requires airSpring + wetSpring + neuralSpring registered",
                );
            }
            Ok(false) => {
                v.check_bool("orchestration_reachable", false, "orchestration not live");
                v.check_skip(
                    "cross_spring_data_flow",
                    "needs live spring primals for capability routing",
                );
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip("orchestration_reachable", &format!("{e}"));
                v.check_skip(
                    "cross_spring_data_flow",
                    "needs live spring primals for capability routing",
                );
            }
            Err(e) => {
                v.check_bool("orchestration_reachable", false, &format!("error: {e}"));
                v.check_skip(
                    "cross_spring_data_flow",
                    "needs live spring primals for capability routing",
                );
            }
        }
    } else {
        v.check_skip(
            "orchestration_reachable",
            "orchestration capability not in context",
        );
        v.check_skip(
            "cross_spring_data_flow",
            "needs live spring primals for capability routing",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_spring_data_flow_no_panic() {
        let mut v = ValidationResult::new("cross-spring-data-flow");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
