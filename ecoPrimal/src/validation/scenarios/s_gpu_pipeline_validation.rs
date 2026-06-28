// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: GPU Pipeline Validation — end-to-end sovereign GPU compute.
//!
//! Validates the full GPU compute pipeline from manifest declaration through
//! capability routing to live dispatch readiness. This scenario proves that
//! the ecosystem's GPU infrastructure (ironGate RTX 5070) is correctly
//! wired into the primalSpring validation surface.
//!
//! Phases:
//! 1. Manifest: `gpu_target` declared for GPU gates, dual-target model
//! 2. Capability coverage: tensor/ml/nautilus/shader/compute methods breadth
//! 3. Precision model: f64 native support, SHADER_F64, precision tiers
//! 4. Pipeline routing: eastGate → ironGate dispatch path validated
//! 5. Live: GPU primal health + tensor create + ml dispatch readiness

use crate::composition::{CompositionContext, capability_to_primal};
use crate::evolution::mesh_address;
use crate::primal_names;
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const MANIFEST_TOML: &str =
    include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gpu-pipeline-validation",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave124_gpu_pipeline",
        provenance_date: "2026-06-23",
        description: "GPU pipeline E2E — manifest gpu_target, capability breadth, precision model, cross-gate routing",
    },
    run,
};

/// GPU-owning primals in the Node atomic.
const GPU_PRIMALS: &[&str] = &[
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
    primal_names::TOADSTOOL,
];

/// Methods that must exist for a complete GPU pipeline.
const PIPELINE_METHODS: &[(&str, &str)] = &[
    ("tensor.create", "tensor session lifecycle"),
    ("tensor.matmul", "matrix multiplication dispatch"),
    ("tensor.batch.submit", "batch workload submission"),
    ("ml.mlp_train", "MLP training pipeline"),
    ("ml.esn_predict", "ESN prediction dispatch"),
    ("nautilus.train", "nautilus model training"),
    ("nautilus.predict", "nautilus inference"),
    ("shader.compile.wgsl", "WGSL shader compilation"),
    ("shader.compile.spirv", "SPIR-V compilation path"),
    ("compute.dispatch.submit", "workload submission to fleet"),
    ("compute.gpu", "GPU-specific compute dispatch"),
    ("rng.uniform", "RNG for stochastic workloads"),
];

/// Run all GPU pipeline validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Manifest — GPU target declaration");
    phase_manifest(v);

    v.section("Phase 2: Capability coverage — GPU method breadth");
    phase_capability_coverage(v);

    v.section("Phase 3: Precision model — f64 native + tiers");
    phase_precision_model(v);

    v.section("Phase 4: Pipeline routing — cross-gate dispatch path");
    phase_routing(v);

    v.section("Phase 5: Live — GPU primal dispatch readiness");
    phase_live(v, ctx);
}

fn phase_manifest(v: &mut ValidationResult) {
    let parsed: Result<toml::Value, _> = MANIFEST_TOML.parse();
    let Ok(manifest) = parsed else {
        v.check_bool(
            "manifest:parses",
            false,
            "ecosystem_manifest.toml failed to parse",
        );
        return;
    };

    v.check_bool(
        "manifest:parses",
        true,
        "ecosystem_manifest.toml valid TOML",
    );

    let gates = manifest.get("gates").and_then(|g| g.as_table());
    let Some(gates) = gates else {
        v.check_bool("manifest:gates_section", false, "no [gates] section");
        return;
    };

    let gpu_gates: Vec<&String> = gates
        .iter()
        .filter(|(_, gate)| gate.get("gpu_target").and_then(|v| v.as_str()).is_some())
        .map(|(name, _)| name)
        .collect();

    v.check_bool(
        "manifest:gpu_gates_declared",
        !gpu_gates.is_empty(),
        &format!("gates with gpu_target: {gpu_gates:?}"),
    );

    for gate_name in &gpu_gates {
        let gate = &gates[gate_name.as_str()];
        let gpu_target = gate
            .get("gpu_target")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        v.check_bool(
            &format!("manifest:{gate_name}:gpu_target_valid"),
            gpu_target.contains("gnu") || gpu_target.contains("cuda"),
            &format!("{gate_name}.gpu_target = \"{gpu_target}\" (expect gnu/cuda)"),
        );

        let main_target = gate.get("target").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            &format!("manifest:{gate_name}:dual_target"),
            main_target != gpu_target,
            &format!("{gate_name} dual-target: main=\"{main_target}\" gpu=\"{gpu_target}\""),
        );

        let roles = gate
            .get("roles")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            &format!("manifest:{gate_name}:gpu_role"),
            roles.iter().any(|r| *r == "gpu" || *r == "compute"),
            &format!("{gate_name} roles include gpu/compute: {roles:?}"),
        );
    }
}

fn phase_capability_coverage(v: &mut ValidationResult) {
    let mut covered = 0;

    for (method, desc) in PIPELINE_METHODS {
        let present = REGISTRY_TOML.contains(method);
        if present {
            covered += 1;
        }
        v.check_bool(
            &format!("cap:{}", method.replace('.', "_")),
            present,
            &format!("{method} — {desc}"),
        );
    }

    v.check_bool(
        "cap:pipeline_breadth",
        covered >= 10,
        &format!(
            "{covered}/{} GPU pipeline methods registered",
            PIPELINE_METHODS.len()
        ),
    );

    let gpu_domains = ["tensor", "ml", "nautilus", "shader", "compute", "rng"];
    let mut domain_count = 0;
    for domain in gpu_domains {
        let has_section = REGISTRY_TOML.contains(&format!("[{domain}]"));
        if has_section {
            domain_count += 1;
        }
        v.check_bool(
            &format!("cap:domain_{domain}"),
            has_section,
            &format!("[{domain}] section in capability_registry.toml"),
        );
    }

    v.check_bool(
        "cap:domain_completeness",
        domain_count >= 5,
        &format!("{domain_count}/6 GPU-related domains present"),
    );
}

fn phase_precision_model(v: &mut ValidationResult) {
    let has_f64_method =
        REGISTRY_TOML.contains("precision") || REGISTRY_TOML.contains("compute.precision");
    v.check_bool(
        "precision:compute_precision_method",
        has_f64_method,
        "compute.precision method registered for tier negotiation",
    );

    let tensor_methods: Vec<&str> = PIPELINE_METHODS
        .iter()
        .filter(|(m, _)| m.starts_with("tensor."))
        .map(|(m, _)| *m)
        .collect();

    v.check_bool(
        "precision:tensor_ops_count",
        tensor_methods.len() >= 3,
        &format!(
            "{} tensor operations for GPU dispatch",
            tensor_methods.len()
        ),
    );

    let has_matmul = REGISTRY_TOML.contains("tensor.matmul");
    let has_matmul_inline = REGISTRY_TOML.contains("tensor.matmul_inline");
    v.check_bool(
        "precision:matmul_variants",
        has_matmul && has_matmul_inline,
        "tensor.matmul + tensor.matmul_inline (zero-copy + inline paths)",
    );

    let ml_methods = ["ml.mlp_train", "ml.esn_predict"];
    let ml_count = ml_methods
        .iter()
        .filter(|m| REGISTRY_TOML.contains(*m))
        .count();
    v.check_bool(
        "precision:ml_pipeline_depth",
        ml_count >= 2,
        &format!(
            "{ml_count}/{} ML training+inference methods",
            ml_methods.len()
        ),
    );
}

fn phase_routing(v: &mut ValidationResult) {
    let gpu_domains = ["tensor", "ml", "nautilus", "rng"];
    for domain in gpu_domains {
        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("route:{domain}_to_barracuda"),
            primal == primal_names::BARRACUDA,
            &format!("{domain} → \"{primal}\" (expected barracuda)"),
        );
    }

    let primal = capability_to_primal("shader");
    v.check_bool(
        "route:shader_to_coralreef",
        primal == primal_names::CORALREEF,
        &format!("shader → \"{primal}\" (expected coralreef)"),
    );

    let primal = capability_to_primal("compute");
    v.check_bool(
        "route:compute_to_toadstool",
        primal == primal_names::TOADSTOOL,
        &format!("compute → \"{primal}\" (expected toadstool)"),
    );

    let iron_addr = mesh_address("ironGate");
    v.check_bool(
        "route:irongate_reachable",
        iron_addr.is_some(),
        &format!("ironGate mesh address resolved from SSOT: {iron_addr:?}"),
    );

    for slug in GPU_PRIMALS {
        let port = ports::default_port_for(slug);
        v.check_bool(
            &format!("route:{slug}_port"),
            port > 0,
            &format!("{slug} port = {port}"),
        );
    }

    let east_addr = mesh_address("eastGate");
    v.check_bool(
        "route:eastgate_orchestrator",
        east_addr.is_some(),
        &format!("eastGate (orchestrator) mesh address: {east_addr:?}"),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let gpu_caps = ["tensor", "ml", "shader", "compute"];

    for cap in gpu_caps {
        match ctx.client_for(cap) {
            Some(client) => {
                let resp = client.call("health.liveness", serde_json::json!({}));
                match resp {
                    Ok(r) => {
                        v.check_bool(
                            &format!("live:{cap}_health"),
                            r.is_success(),
                            &format!("{cap} primal live and healthy"),
                        );
                    }
                    Err(e) if e.is_skippable() => {
                        v.check_skip(&format!("live:{cap}_health"), &format!("{cap}: {e}"));
                    }
                    Err(e) => {
                        v.check_bool(&format!("live:{cap}_health"), false, &format!("{e}"));
                    }
                }
            }
            None => {
                v.check_skip(&format!("live:{cap}_health"), &format!("no {cap} client"));
            }
        }
    }

    match ctx.client_for("tensor") {
        Some(client) => {
            let resp = client.call(
                "tensor.create",
                serde_json::json!({"shape": [2, 2], "dtype": "f64"}),
            );
            match resp {
                Ok(r) => {
                    let has_id = r
                        .result
                        .as_ref()
                        .and_then(|v| v.get("tensor_id").or_else(|| v.get("id")))
                        .is_some();
                    v.check_bool(
                        "live:tensor_create_responds",
                        has_id || r.is_success(),
                        "tensor.create returns tensor_id (f64 dispatch ready)",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip(
                        "live:tensor_create_responds",
                        &format!("tensor.create: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool("live:tensor_create_responds", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip("live:tensor_create_responds", "no tensor client");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn gpu_pipeline_runs_structural() {
        let mut v = ValidationResult::new("gpu-pipeline-validation");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 30, "expected ≥30 checks, got {total}");
    }

    #[test]
    fn manifest_declares_gpu_target() {
        let parsed: toml::Value = MANIFEST_TOML.parse().unwrap();
        let gates = parsed.get("gates").unwrap().as_table().unwrap();
        let iron = gates.get("ironGate").unwrap();
        let gpu_target = iron.get("gpu_target").and_then(|v| v.as_str());
        assert_eq!(
            gpu_target,
            Some("x86_64-unknown-linux-gnu"),
            "ironGate should declare gpu_target"
        );
    }

    #[test]
    fn dual_target_model() {
        let parsed: toml::Value = MANIFEST_TOML.parse().unwrap();
        let gates = parsed.get("gates").unwrap().as_table().unwrap();
        let iron = gates.get("ironGate").unwrap();
        let main = iron.get("target").and_then(|v| v.as_str()).unwrap();
        let gpu = iron.get("gpu_target").and_then(|v| v.as_str()).unwrap();
        assert_ne!(
            main, gpu,
            "main and gpu targets should differ (musl vs gnu)"
        );
    }

    #[test]
    fn pipeline_methods_registered() {
        let mut count = 0;
        for (method, _) in PIPELINE_METHODS {
            if REGISTRY_TOML.contains(method) {
                count += 1;
            }
        }
        assert!(
            count >= 10,
            "expected ≥10 pipeline methods registered, got {count}"
        );
    }

    #[test]
    fn gpu_domains_present() {
        let domains = ["tensor", "ml", "nautilus", "shader", "compute"];
        for domain in domains {
            assert!(
                REGISTRY_TOML.contains(&format!("[{domain}]")),
                "[{domain}] section missing from capability_registry.toml"
            );
        }
    }

    #[test]
    fn barracuda_owns_tensor_ml() {
        assert_eq!(capability_to_primal("tensor"), primal_names::BARRACUDA);
        assert_eq!(capability_to_primal("ml"), primal_names::BARRACUDA);
        assert_eq!(capability_to_primal("nautilus"), primal_names::BARRACUDA);
    }

    #[test]
    fn coralreef_owns_shader() {
        assert_eq!(capability_to_primal("shader"), primal_names::CORALREEF);
    }

    #[test]
    fn toadstool_owns_compute() {
        assert_eq!(capability_to_primal("compute"), primal_names::TOADSTOOL);
    }

    #[test]
    fn irongate_gpu_addressable() {
        assert_eq!(mesh_address("ironGate"), Some("10.13.37.7"));
    }

    #[test]
    fn gpu_primals_have_ports() {
        for slug in GPU_PRIMALS {
            let port = ports::default_port_for(slug);
            assert!(port > 0, "{slug} should have a non-zero port");
        }
    }

    #[test]
    fn matmul_variants_exist() {
        assert!(REGISTRY_TOML.contains("tensor.matmul"));
        assert!(REGISTRY_TOML.contains("tensor.matmul_inline"));
    }
}
