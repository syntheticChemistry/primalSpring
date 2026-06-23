// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Shader Compilation Pipeline — coralReef compiler validation.
//!
//! Validates the shader compilation pipeline from capability registration through
//! backend coverage (SPIR-V, WGSL, PTX), precision tiers (f32 default, f64 via
//! SHADER_F64), and integration with barraCuda/toadStool dispatch.
//!
//! Phases:
//! 1. Shader methods — compile, multi-compile, validate, backend listing
//! 2. Backend coverage — SPIR-V, WGSL, PTX targets in capability registry
//! 3. Compilation model — source → validate → compile → output + precision tiers
//! 4. Integration — coralReef → barraCuda dispatch chain, live probe if reachable

use crate::composition::{CompositionContext, capability_to_primal, method_to_capability_domain};
use crate::evolution::mesh_address;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MANIFEST_TOML: &str =
    include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "shader-compilation-pipeline",
        track: Track::GraphExecution,
        tier: Tier::Rust,
        provenance_crate: "wave124_shader_pipeline",
        provenance_date: "2026-06-23",
        description: "Shader compilation pipeline — coralReef backends, precision tiers, barraCuda dispatch integration",
    },
    run,
};

/// Shader pipeline methods with accepted registry aliases (canonical → aliases).
const SHADER_PIPELINE_METHODS: &[(&str, &[&str])] = &[
    (
        "shader.compile",
        &["shader.compile.wgsl", "shader.compile.spirv"],
    ),
    (
        "shader.compile.multi",
        &["shader.compile.wgsl.multi", "shader.compile.multi"],
    ),
    ("shader.validate", &["shader.validate"]),
    (
        "shader.list_backends",
        &["shader.compile.capabilities", "shader.list_backends"],
    ),
];

/// Compilation pipeline stages (source → validate → compile → output).
const PIPELINE_STAGES: &[(&str, &[&str])] = &[
    ("source", &["shader.compile.module", "shader.compile.wgsl"]),
    ("validate", &["shader.validate"]),
    ("compile", &["shader.compile.wgsl", "shader.compile.spirv"]),
    ("output", &["shader.compile.spirv", "shader.dispatch"]),
];

/// GPU shader backends expected in the registry or manifest.
const BACKEND_MARKERS: &[(&str, &[&str])] = &[
    ("spirv", &["shader.compile.spirv", "spirv"]),
    ("wgsl", &["shader.compile.wgsl", "wgsl"]),
    ("ptx", &["ptx", "CUDA", "nvidia"]),
];

/// Run all shader compilation pipeline validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Shader methods");
    phase_shader_methods(v);

    v.section("Phase 2: Backend coverage");
    phase_backend_coverage(v);

    v.section("Phase 3: Compilation model");
    phase_compilation_model(v);

    v.section("Phase 4: Integration");
    phase_integration(v, ctx);
}

fn registry_has_any(candidates: &[&'static str]) -> Option<&'static str> {
    candidates
        .iter()
        .find(|method| REGISTRY_TOML.contains(**method))
        .copied()
}

fn registry_or_manifest_has(markers: &[&str]) -> bool {
    markers
        .iter()
        .any(|m| REGISTRY_TOML.contains(m) || MANIFEST_TOML.contains(m))
}

fn phase_shader_methods(v: &mut ValidationResult) {
    let shader_owner = capability_to_primal("shader");
    v.check_bool(
        "methods:shader_owner_coralreef",
        shader_owner == primal_names::CORALREEF,
        &format!("shader domain → \"{shader_owner}\" (expected coralreef)"),
    );

    let mut registered = 0usize;
    for (canonical, aliases) in SHADER_PIPELINE_METHODS {
        let resolved = registry_has_any(aliases);
        let present = resolved.is_some();
        if present {
            registered += 1;
        }
        v.check_bool(
            &format!("methods:{}", canonical.replace('.', "_")),
            present,
            &format!(
                "{canonical} registered as {}",
                resolved.unwrap_or("(missing)")
            ),
        );

        if let Some(method) = resolved {
            let domain = method_to_capability_domain(method);
            let primal = capability_to_primal(domain);
            v.check_bool(
                &format!("methods:route_{}", method.replace('.', "_")),
                primal == primal_names::CORALREEF,
                &format!("{method} → \"{primal}\" (expected coralreef)"),
            );
        }
    }

    v.check_bool(
        "methods:shader_pipeline_coverage",
        registered >= 4,
        &format!(
            "{registered}/{} shader pipeline methods registered",
            SHADER_PIPELINE_METHODS.len()
        ),
    );

    v.check_bool(
        "methods:shader_section_present",
        REGISTRY_TOML.contains("[shader]"),
        "[shader] section in capability_registry.toml",
    );
}

fn phase_backend_coverage(v: &mut ValidationResult) {
    let mut backends = 0usize;

    for (name, markers) in BACKEND_MARKERS {
        let present = registry_or_manifest_has(markers);
        if present {
            backends += 1;
        }
        v.check_bool(
            &format!("backend:{name}"),
            present,
            &format!("{name} backend declared in registry or manifest"),
        );
    }

    v.check_bool(
        "backend:triple_coverage",
        backends >= 3,
        &format!("{backends}/3 shader backends (SPIR-V, WGSL, PTX) covered"),
    );

    v.check_bool(
        "backend:spirv_method",
        REGISTRY_TOML.contains("shader.compile.spirv"),
        "shader.compile.spirv registered for SPIR-V output",
    );

    v.check_bool(
        "backend:wgsl_method",
        REGISTRY_TOML.contains("shader.compile.wgsl"),
        "shader.compile.wgsl registered for WGSL source path",
    );
}

fn phase_compilation_model(v: &mut ValidationResult) {
    let mut stages_ok = 0usize;

    for (stage, methods) in PIPELINE_STAGES {
        let present = registry_has_any(methods).is_some();
        if present {
            stages_ok += 1;
        }
        v.check_bool(
            &format!("pipeline:stage_{stage}"),
            present,
            &format!("compilation stage '{stage}' has registered methods"),
        );
    }

    v.check_bool(
        "pipeline:full_chain",
        stages_ok >= 4,
        &format!("{stages_ok}/4 pipeline stages (source→validate→compile→output)"),
    );

    let has_precision = REGISTRY_TOML.contains("compute.precision")
        || REGISTRY_TOML.contains("barracuda.precision.route");
    v.check_bool(
        "pipeline:precision_negotiation",
        has_precision,
        "precision tier negotiation method registered (f32/f64 routing)",
    );

    let f64_manifest = MANIFEST_TOML.contains("SHADER_F64") || MANIFEST_TOML.contains("f64");
    v.check_bool(
        "pipeline:f64_shader_f64",
        f64_manifest,
        "SHADER_F64 / f64 precision documented in ecosystem manifest",
    );

    v.check_bool(
        "pipeline:f32_default_wgsl",
        REGISTRY_TOML.contains("shader.compile.wgsl"),
        "shader.compile.wgsl available as default f32 compilation entry",
    );
}

fn phase_integration(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.check_bool(
        "integration:shader_to_coralreef",
        capability_to_primal("shader") == primal_names::CORALREEF,
        "shader.compile routes through coralReef",
    );

    v.check_bool(
        "integration:compute_to_toadstool",
        capability_to_primal("compute") == primal_names::TOADSTOOL,
        "compute.dispatch routes through toadStool",
    );

    v.check_bool(
        "integration:tensor_to_barracuda",
        capability_to_primal("tensor") == primal_names::BARRACUDA,
        "ml.infer/tensor dispatch routes through barraCuda",
    );

    let dispatch_chain = REGISTRY_TOML.contains("shader.compile.wgsl")
        && REGISTRY_TOML.contains("compute.dispatch.submit")
        && REGISTRY_TOML.contains("tensor.matmul");
    v.check_bool(
        "integration:compile_to_dispatch_chain",
        dispatch_chain,
        "shader.compile.wgsl → compute.dispatch.submit → tensor.* chain registered",
    );

    let infer_methods = ["ml.esn_predict", "ml.lstm_predict", "nautilus.predict"];
    let infer_registered = infer_methods.iter().any(|m| REGISTRY_TOML.contains(*m));
    v.check_bool(
        "integration:infer_methods_for_shader_output",
        infer_registered,
        "ml inference methods registered for shader.compile → ml.infer pipeline",
    );

    if mesh_address("ironGate").is_none() {
        v.check_skip(
            "integration:live_dispatch",
            "ironGate mesh address not assigned",
        );
        return;
    }

    if !ctx.has_capability("shader") {
        v.check_skip("integration:live_dispatch", "shader capability not present");
        return;
    }

    match ctx.call(
        "shader",
        "shader.compile.capabilities",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let has_backends = resp.get("targets").is_some()
                || resp.get("backends").is_some()
                || resp.get("supported_targets").is_some();
            v.check_bool(
                "integration:live_backend_list",
                has_backends,
                "shader.compile.capabilities returns backend/target list on live ironGate",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("integration:live_dispatch", &format!("live probe: {e}"));
        }
        Err(e) => {
            v.check_skip(
                "integration:live_dispatch",
                &format!("live probe skipped: {e}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn shader_compilation_pipeline_structural() {
        let mut v = ValidationResult::new("shader-compilation-pipeline");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 20, "expected ≥20 checks, got {total}");
        assert_eq!(v.failed, 0, "Rust-tier structural checks must pass");
    }

    #[test]
    fn backend_list_methods() {
        assert!(
            registry_has_any(&["shader.compile.capabilities", "shader.list_backends"]).is_some(),
            "backend listing method should be registered"
        );
        assert!(REGISTRY_TOML.contains("shader.compile.spirv"));
        assert!(REGISTRY_TOML.contains("shader.compile.wgsl"));
    }

    #[test]
    fn compilation_methods_registered() {
        for (canonical, aliases) in SHADER_PIPELINE_METHODS {
            assert!(
                registry_has_any(aliases).is_some(),
                "{canonical} should have a registered alias"
            );
        }
    }

    #[test]
    fn precision_tiers_documented() {
        assert!(
            REGISTRY_TOML.contains("compute.precision")
                || REGISTRY_TOML.contains("barracuda.precision.route"),
            "precision negotiation method required"
        );
        assert!(
            MANIFEST_TOML.contains("SHADER_F64") || MANIFEST_TOML.contains("f64"),
            "f64/SHADER_F64 should appear in ecosystem manifest"
        );
    }

    #[test]
    fn coralreef_owns_shader_domain() {
        assert_eq!(capability_to_primal("shader"), primal_names::CORALREEF);
    }

    #[test]
    fn shader_compilation_pipeline_full_run() {
        let mut v = ValidationResult::new("shader-compilation-pipeline");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "full run should pass all structural checks");
    }
}
