// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Tier 2 Science API — live wire contract validation.
//!
//! Validates that the Tier 2 Science API methods documented in
//! `docs/LIVE_SCIENCE_API.md` are reachable and return well-formed
//! responses. This is the exemplar pattern for springs wiring Tier 2.
//!
//! Phases:
//! 1. `toadstool.validate` — workload pre-flight contract
//! 2. `toadstool.list_workloads` — workload registry enumeration
//! 3. `barracuda.precision.route` — precision tier routing
//! 4. `biomeos.spring_status` — spring health enumeration

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tier2-science-api",
        track: Track::Transport,
        tier: Tier::Live,
        provenance_crate: "live_science_api",
        provenance_date: "2026-05-14",
        description: "Tier 2 Science API wire contract (toadstool.validate, precision.route, spring_status)",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: toadstool.validate");
    phase_toadstool_validate(v, ctx);

    v.section("Phase 2: toadstool.list_workloads");
    phase_toadstool_list(v, ctx);

    v.section("Phase 3: barracuda.precision.route");
    phase_precision_route(v, ctx);

    v.section("Phase 4: biomeos.spring_status");
    phase_spring_status(v, ctx);
}

fn phase_toadstool_validate(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("compute") {
        v.check_skip("toadstool_validate", "compute capability not present");
        return;
    }

    let params = serde_json::json!({
        "workload_path": "/dev/null",
        "dry_run": true,
    });
    match ctx.call("compute", "toadstool.validate", params) {
        Ok(resp) => {
            let has_valid = resp.get("valid").is_some();
            v.check_bool(
                "toadstool_validate_shape",
                has_valid,
                "toadstool.validate returns {valid: ...} shape",
            );
        }
        Err(e) => {
            let is_method_missing = format!("{e}").contains("-32601");
            if is_method_missing {
                v.check_skip(
                    "toadstool_validate_shape",
                    "toadstool.validate not implemented on this build",
                );
            } else {
                v.check_bool("toadstool_validate_shape", false, &format!("call failed: {e}"));
            }
        }
    }
}

fn phase_toadstool_list(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("compute") {
        v.check_skip("toadstool_list_workloads", "compute capability not present");
        return;
    }

    let params = serde_json::json!({ "filter": "all" });
    match ctx.call("compute", "toadstool.list_workloads", params) {
        Ok(resp) => {
            let has_workloads = resp.get("workloads").is_some() || resp.get("total").is_some();
            v.check_bool(
                "toadstool_list_workloads_shape",
                has_workloads,
                "toadstool.list_workloads returns enumerable shape",
            );
        }
        Err(e) => {
            let is_method_missing = format!("{e}").contains("-32601");
            if is_method_missing {
                v.check_skip(
                    "toadstool_list_workloads_shape",
                    "toadstool.list_workloads not implemented on this build",
                );
            } else {
                v.check_bool(
                    "toadstool_list_workloads_shape",
                    false,
                    &format!("call failed: {e}"),
                );
            }
        }
    }
}

fn phase_precision_route(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("tensor") {
        v.check_skip("precision_route", "tensor capability not present");
        return;
    }

    let params = serde_json::json!({
        "operation": "stats.mean",
        "input_range": "normal",
        "tolerance": "1e-9",
    });
    match ctx.call("tensor", "barracuda.precision.route", params) {
        Ok(resp) => {
            let has_strategy = resp.get("strategy").is_some() || resp.get("precision_tier").is_some();
            v.check_bool(
                "precision_route_shape",
                has_strategy,
                "barracuda.precision.route returns strategy shape",
            );
        }
        Err(e) => {
            let is_method_missing = format!("{e}").contains("-32601");
            if is_method_missing {
                v.check_skip(
                    "precision_route_shape",
                    "barracuda.precision.route not implemented on this build",
                );
            } else {
                v.check_bool("precision_route_shape", false, &format!("call failed: {e}"));
            }
        }
    }
}

fn phase_spring_status(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip("spring_status", "orchestration capability not present");
        return;
    }

    match ctx.call("orchestration", "biomeos.spring_status", serde_json::json!({})) {
        Ok(resp) => {
            let is_object = resp.is_object();
            v.check_bool(
                "spring_status_shape",
                is_object,
                "biomeos.spring_status returns object shape",
            );
        }
        Err(e) => {
            let is_method_missing = format!("{e}").contains("-32601");
            if is_method_missing {
                v.check_skip(
                    "spring_status_shape",
                    "biomeos.spring_status not implemented on this build",
                );
            } else {
                v.check_bool("spring_status_shape", false, &format!("call failed: {e}"));
            }
        }
    }
}
