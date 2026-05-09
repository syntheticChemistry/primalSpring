// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Compute Triangle — absorbed from exp050.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const COMPUTE_TRIANGLE: &[&str] = &["shader", "compute", "tensor"];

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "compute-triangle",
        track: Track::Transport,
        tier: Tier::Live,
        provenance_crate: "exp050_compute_triangle",
        provenance_date: "2026-05-09",
        description: "Compute triangle — shader, compute, tensor capability discovery",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 2: Health + Capabilities");
    phase_health_capabilities(v, ctx);

    v.section("Phase 3: Pipeline");
    phase_pipeline(v);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in COMPUTE_TRIANGLE {
        if ctx.has_capability(cap) {
            v.check_bool(
                &format!("has_{cap}"),
                true,
                &format!("{cap} capability discoverable"),
            );
        } else {
            v.check_skip(
                &format!("has_{cap}"),
                &format!("{cap} not present in context"),
            );
        }
    }
}

fn phase_health_capabilities(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    for cap in COMPUTE_TRIANGLE {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("health_liveness_{cap}"),
                &format!("{cap} not present in context"),
            );
            continue;
        }
        match ctx.health_check(cap) {
            Ok(true) => v.check_bool(
                &format!("health_liveness_{cap}"),
                true,
                &format!("{cap} health.liveness ok"),
            ),
            Ok(false) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("{cap} not live"),
            ),
            Err(e) if e.is_connection_error() => {
                v.check_skip(&format!("health_liveness_{cap}"), &format!("{cap}: {e}"));
            }
            Err(e) => v.check_bool(
                &format!("health_liveness_{cap}"),
                false,
                &format!("error: {e}"),
            ),
        }
    }
}

fn phase_pipeline(v: &mut ValidationResult) {
    v.check_skip(
        "compile_dispatch_pipeline",
        "actual compile+dispatch pipeline needs live primals",
    );
}
