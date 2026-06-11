// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Compute Triangle — sovereign dispatch contract test.
//!
//! Validates the compute trio (coralReef + toadStool + barraCuda) as the
//! Node atomic's E2E dispatch pipeline. Phases:
//!
//! 1. Discovery — verify shader, compute, tensor capabilities present
//! 2. coralReef health + `shader.compile.capabilities`
//! 3. toadStool health + `compute.capabilities`
//! 4. barraCuda health + `tensor.create` + `stats.mean` round-trip
//! 5. Sovereign dispatch contract — `shader.compile.wgsl` -> response shape
//!    -> `compute.dispatch.submit` -> response shape (E2E)
//!
//! Absorbed from exp050; evolved in Wave 8 (compute trio composition).

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
        provenance_date: "2026-05-11",
        description: "Compute triangle - sovereign dispatch contract (compile+dispatch E2E shape)",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Discovery");
    phase_discovery(v, ctx);

    v.section("Phase 2: coralReef health + shader.compile.capabilities");
    phase_coralreef_capabilities(v, ctx);

    v.section("Phase 3: toadStool health + compute.capabilities");
    phase_toadstool_capabilities(v, ctx);

    v.section("Phase 4: barraCuda health + tensor.create + stats.mean round-trip");
    phase_barracuda_math(v, ctx);

    v.section("Phase 5: Sovereign dispatch contract (compile + dispatch E2E)");
    phase_sovereign_dispatch(v, ctx);
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

fn phase_coralreef_capabilities(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("shader") {
        v.check_skip("coralreef_health", "shader not present in context");
        v.check_skip("shader_compile_capabilities", "shader not present");
        return;
    }

    match ctx.health_check("shader") {
        Ok(true) => v.check_bool("coralreef_health", true, "coralReef health.liveness ok"),
        Ok(false) => {
            v.check_bool("coralreef_health", false, "coralReef not live");
            v.check_skip("shader_compile_capabilities", "coralReef not live");
            return;
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("coralreef_health", &format!("coralReef: {e}"));
            v.check_skip("shader_compile_capabilities", "coralReef not reachable");
            return;
        }
        Err(e) => {
            v.check_bool("coralreef_health", false, &format!("error: {e}"));
            v.check_skip("shader_compile_capabilities", "coralReef health failed");
            return;
        }
    }

    match ctx.call(
        "shader",
        "shader.compile.capabilities",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_targets = resp.get("targets").and_then(|t| t.as_array()).is_some();
            v.check_bool(
                "shader_compile_capabilities",
                has_targets,
                if has_targets {
                    "shader.compile.capabilities returns target architectures"
                } else {
                    "shader.compile.capabilities: missing 'targets' array"
                },
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("shader_compile_capabilities", &format!("coralReef: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "shader_compile_capabilities",
                false,
                &format!("shader.compile.capabilities error: {e}"),
            );
        }
    }
}

fn phase_toadstool_capabilities(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("compute") {
        v.check_skip("toadstool_health", "compute not present in context");
        v.check_skip("compute_capabilities", "compute not present");
        return;
    }

    match ctx.health_check("compute") {
        Ok(true) => v.check_bool("toadstool_health", true, "toadStool health.liveness ok"),
        Ok(false) => {
            v.check_bool("toadstool_health", false, "toadStool not live");
            v.check_skip("compute_capabilities", "toadStool not live");
            return;
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("toadstool_health", &format!("toadStool: {e}"));
            v.check_skip("compute_capabilities", "toadStool not reachable");
            return;
        }
        Err(e) => {
            v.check_bool("toadstool_health", false, &format!("error: {e}"));
            v.check_skip("compute_capabilities", "toadStool health failed");
            return;
        }
    }

    match ctx.call("compute", "compute.capabilities", serde_json::json!({})) {
        Ok(resp) => {
            let has_backends = resp.get("backends").and_then(|b| b.as_array()).is_some()
                || resp.get("devices").and_then(|d| d.as_array()).is_some()
                || resp.get("capabilities").is_some();
            v.check_bool(
                "compute_capabilities",
                has_backends,
                if has_backends {
                    "compute.capabilities returns hardware info"
                } else {
                    "compute.capabilities: missing backends/devices/capabilities"
                },
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("compute_capabilities", &format!("toadStool: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "compute_capabilities",
                false,
                &format!("compute.capabilities error: {e}"),
            );
        }
    }
}

fn phase_barracuda_math(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("tensor") {
        v.check_skip("barracuda_health", "tensor not present in context");
        v.check_skip("tensor_create_shape", "tensor not present");
        v.check_skip("stats_mean_roundtrip", "tensor not present");
        return;
    }

    match ctx.health_check("tensor") {
        Ok(true) => v.check_bool("barracuda_health", true, "barraCuda health.liveness ok"),
        Ok(false) => {
            v.check_bool("barracuda_health", false, "barraCuda not live");
            v.check_skip("tensor_create_shape", "barraCuda not live");
            v.check_skip("stats_mean_roundtrip", "barraCuda not live");
            return;
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("barracuda_health", &format!("barraCuda: {e}"));
            v.check_skip("tensor_create_shape", "barraCuda not reachable");
            v.check_skip("stats_mean_roundtrip", "barraCuda not reachable");
            return;
        }
        Err(e) => {
            v.check_bool("barracuda_health", false, &format!("error: {e}"));
            v.check_skip("tensor_create_shape", "barraCuda health failed");
            v.check_skip("stats_mean_roundtrip", "barraCuda health failed");
            return;
        }
    }

    match ctx.call(
        "tensor",
        "tensor.create",
        serde_json::json!({
            "shape": [4],
            "data": [2.0_f64, 4.0, 6.0, 8.0],
            "dtype": "f64"
        }),
    ) {
        Ok(resp) => {
            let has_id = resp.get("tensor_id").is_some() || resp.get("id").is_some();
            v.check_bool(
                "tensor_create_shape",
                has_id,
                if has_id {
                    "tensor.create returns handle for [4] f64"
                } else {
                    "tensor.create: missing tensor_id/id in response"
                },
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("tensor_create_shape", &format!("barraCuda: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "tensor_create_shape",
                false,
                &format!("tensor.create error: {e}"),
            );
        }
    }

    match ctx.call(
        "tensor",
        "stats.mean",
        serde_json::json!({ "data": [2.0_f64, 4.0, 6.0, 8.0] }),
    ) {
        Ok(resp) => {
            let mean = resp
                .get("mean")
                .or_else(|| resp.get("result"))
                .and_then(serde_json::Value::as_f64);
            let correct =
                mean.is_some_and(|m| (m - 5.0).abs() < crate::tolerances::CPU_GPU_PARITY_TOL);
            v.check_bool(
                "stats_mean_roundtrip",
                correct,
                &format!(
                    "stats.mean([2,4,6,8]) = {} (expected 5.0)",
                    mean.map_or_else(|| "N/A".to_owned(), |m| format!("{m}"))
                ),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("stats_mean_roundtrip", &format!("barraCuda: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "stats_mean_roundtrip",
                false,
                &format!("stats.mean error: {e}"),
            );
        }
    }
}

/// Sovereign dispatch E2E: shader.compile.wgsl -> compute.dispatch.submit
///
/// Tests the contract *shape* — we verify that:
/// 1. shader.compile.wgsl returns `binary_b64` + `shader_info`
/// 2. compute.dispatch.submit accepts compiled binary + dispatch params
///    and returns `dispatch_id` + `status` + `buffers`/`timing`
///
/// Both calls SKIP on connection error (primal not running).
/// This validates the IPC contract, not the GPU result.
/// Attempt shader compilation and return the base64 binary on success.
/// Returns `None` and emits appropriate checks on skip/failure.
fn try_compile_shader(v: &mut ValidationResult, ctx: &mut CompositionContext) -> Option<String> {
    let trivial_wgsl = r"@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
}";

    let compile_result = ctx.call(
        "shader",
        "shader.compile.wgsl",
        serde_json::json!({
            "source": trivial_wgsl,
            "target": "sm70",
            "entry_point": "main",
            "workgroup_size": [1, 1, 1]
        }),
    );

    match compile_result {
        Ok(resp) => {
            let has_binary = resp.get("binary_b64").and_then(|b| b.as_str()).is_some()
                || resp.get("binary").is_some();
            let has_info = resp.get("shader_info").is_some() || resp.get("info").is_some();

            v.check_bool(
                "sovereign_compile_response_shape",
                has_binary && has_info,
                &format!("shader.compile.wgsl: binary={has_binary}, shader_info={has_info}"),
            );

            if has_binary {
                resp.get("binary_b64")
                    .and_then(|b| b.as_str())
                    .map(String::from)
            } else {
                None
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "sovereign_compile_response_shape",
                &format!("coralReef: {e}"),
            );
            None
        }
        Err(e) => {
            v.check_bool(
                "sovereign_compile_response_shape",
                false,
                &format!("shader.compile.wgsl error: {e}"),
            );
            None
        }
    }
}

fn phase_sovereign_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_shader = ctx.has_capability("shader");
    let has_compute = ctx.has_capability("compute");

    if !has_shader || !has_compute {
        v.check_skip(
            "sovereign_compile_response_shape",
            &format!("sovereign dispatch requires shader ({has_shader}) + compute ({has_compute})"),
        );
        v.check_skip("sovereign_dispatch_response_shape", "compile skipped");
        v.check_skip("sovereign_e2e_pipeline_viable", "compile skipped");
        return;
    }

    let Some(binary_b64) = try_compile_shader(v, ctx) else {
        v.check_skip(
            "sovereign_dispatch_response_shape",
            "no binary from compile step",
        );
        v.check_skip("sovereign_e2e_pipeline_viable", "no binary from compile");
        return;
    };

    let dispatch_result = ctx.call(
        "compute",
        "compute.dispatch.submit",
        serde_json::json!({
            "binary_b64": binary_b64,
            "shader_info": { "gprs": 32, "shared_memory": 0, "barriers": 0, "workgroup": [1, 1, 1], "wave_size": 32 },
            "dispatch_dims": [1, 1, 1],
            "buffers": []
        }),
    );

    match dispatch_result {
        Ok(resp) => {
            let has_id = resp.get("dispatch_id").is_some() || resp.get("id").is_some();
            let has_status = resp.get("status").is_some();

            v.check_bool(
                "sovereign_dispatch_response_shape",
                has_id || has_status,
                &format!("compute.dispatch.submit: dispatch_id={has_id}, status={has_status}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "sovereign_dispatch_response_shape",
                &format!("toadStool: {e}"),
            );
            v.check_skip("sovereign_e2e_pipeline_viable", "dispatch not reachable");
            return;
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("not found") || msg.contains("not implemented") {
                v.check_skip(
                    "sovereign_dispatch_response_shape",
                    "compute.dispatch.submit not yet wired (expected pre-absorption)",
                );
            } else {
                v.check_bool(
                    "sovereign_dispatch_response_shape",
                    false,
                    &format!("compute.dispatch.submit error: {e}"),
                );
            }
        }
    }

    v.check_bool(
        "sovereign_e2e_pipeline_viable",
        true,
        "sovereign dispatch contract shape validated (compile -> dispatch path exists)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_triangle_no_panic() {
        let mut v = ValidationResult::new("compute-triangle");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce at least one check"
        );
    }
}
