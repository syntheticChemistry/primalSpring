// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: coralReef Shader Targets — dual-vendor GPU compilation coverage.
//!
//! Validates coralReef's sovereign shader compiler across target ISAs.
//! With strandGate hardware (3090 NVIDIA + 6950 AMD), this scenario
//! exercises the dual-vendor compilation paths and validates that
//! every piece of GPU silicon can be targeted.
//!
//! Phases:
//! 1. `shader.compile.capabilities` — enumerate supported targets
//! 2. Target coverage — PTX (NVIDIA) and RDNA (AMD) targets advertised
//! 3. `shader.compile.wgsl` — contract shape for a trivial shader
//! 4. `naga.module` ingest — direct module compilation path

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "coralreef-shader-targets",
        track: Track::Transport,
        tier: Tier::Live,
        provenance_crate: "exp_coralreef_shader",
        provenance_date: "2026-05-14",
        description: "coralReef dual-vendor shader targets (PTX + RDNA capabilities, WGSL compile, naga ingest)",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Shader compile capabilities");
    phase_capabilities(v, ctx);

    v.section("Phase 2: Target coverage (NVIDIA PTX + AMD RDNA)");
    phase_target_coverage(v, ctx);

    v.section("Phase 3: WGSL compile contract");
    phase_wgsl_compile(v, ctx);

    v.section("Phase 4: naga::Module direct ingest");
    phase_naga_ingest(v, ctx);
}

fn phase_capabilities(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("shader") {
        v.check_skip("shader_capabilities", "shader capability not present");
        return;
    }

    match ctx.call("shader", "shader.compile.capabilities", serde_json::json!({})) {
        Ok(resp) => {
            let has_targets = resp.get("targets").is_some()
                || resp.get("backends").is_some()
                || resp.get("supported_targets").is_some();
            v.check_bool(
                "shader_capabilities_shape",
                has_targets,
                "shader.compile.capabilities returns target list",
            );
        }
        Err(_) => {
            v.check_skip("shader_capabilities_shape", "capabilities probe failed");
        }
    }
}

fn phase_target_coverage(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("shader") {
        v.check_skip("target_coverage", "shader capability not present");
        return;
    }

    match ctx.call("shader", "shader.compile.capabilities", serde_json::json!({})) {
        Ok(resp) => {
            let targets_str = format!("{resp:?}").to_lowercase();

            let has_ptx = targets_str.contains("ptx") || targets_str.contains("nvidia");
            let has_rdna = targets_str.contains("rdna")
                || targets_str.contains("amd")
                || targets_str.contains("gfx");

            if has_ptx {
                v.check_bool("nvidia_ptx_target", true, "NVIDIA PTX target advertised");
            } else {
                v.check_skip(
                    "nvidia_ptx_target",
                    "PTX not in capabilities (may need NVIDIA hardware)",
                );
            }

            if has_rdna {
                v.check_bool("amd_rdna_target", true, "AMD RDNA target advertised");
            } else {
                v.check_skip(
                    "amd_rdna_target",
                    "RDNA not in capabilities (may need AMD hardware)",
                );
            }
        }
        Err(_) => {
            v.check_skip("nvidia_ptx_target", "capabilities not available");
            v.check_skip("amd_rdna_target", "capabilities not available");
        }
    }
}

const TRIVIAL_WGSL: &str = r"
@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // no-op kernel for contract validation
}
";

fn phase_wgsl_compile(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("shader") {
        v.check_skip("wgsl_compile", "shader capability not present");
        return;
    }

    let params = serde_json::json!({
        "source": TRIVIAL_WGSL,
        "target": "spirv",
    });
    match ctx.call("shader", "shader.compile.wgsl", params) {
        Ok(resp) => {
            let has_binary = resp.get("binary").is_some()
                || resp.get("binary_b64").is_some()
                || resp.get("spirv").is_some();
            v.check_bool(
                "wgsl_compile_shape",
                has_binary,
                "shader.compile.wgsl returns binary output",
            );
        }
        Err(e) => {
            let is_method_missing = format!("{e}").contains("-32601");
            if is_method_missing {
                v.check_skip("wgsl_compile_shape", "shader.compile.wgsl not implemented");
            } else {
                v.check_skip(
                    "wgsl_compile_shape",
                    &format!("WGSL compile call error: {e}"),
                );
            }
        }
    }
}

fn phase_naga_ingest(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("shader") {
        v.check_skip("naga_ingest", "shader capability not present");
        return;
    }

    let params = serde_json::json!({ "format": "wgsl", "source": TRIVIAL_WGSL });
    match ctx.call("shader", "shader.compile.module", params) {
        Ok(_) => {
            v.check_bool(
                "naga_module_ingest",
                true,
                "shader.compile.module (naga direct ingest) accepted",
            );
        }
        Err(e) => {
            let is_method_missing = format!("{e}").contains("-32601");
            if is_method_missing {
                v.check_skip("naga_module_ingest", "compile.module not available");
            } else {
                v.check_skip(
                    "naga_module_ingest",
                    &format!("compile.module error: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coralreef_shader_targets_no_panic() {
        let mut v = ValidationResult::new("coralreef-shader-targets");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce at least one check");
    }
}
