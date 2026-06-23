// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: GPU Dispatch Cross-Gate — eastGate → ironGate workload routing.
//!
//! Validates cross-gate GPU workload dispatch: ironGate as the sole GPU gate,
//! dual-target manifest model (musl general + gnu GPU primals), method routing
//! through toadStool/barraCuda/coralReef, and live `compute.health` probing.
//!
//! Phases:
//! 1. Dispatch topology — ironGate GPU gate, mesh route, fleet GPU dispatch
//! 2. Dual-target validation — musl/gnu targets, GPU primal association
//! 3. Method routing — compute/ml/shader dispatch paths + GPU-tagged methods
//! 4. Live probe — `compute.health` when ironGate mesh is reachable

use crate::composition::{CompositionContext, capability_to_primal, method_to_capability_domain};
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
        id: "gpu-dispatch-cross-gate",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave124_gpu_dispatch",
        provenance_date: "2026-06-23",
        description: "Cross-gate GPU dispatch — ironGate topology, dual-target model, compute trio routing",
    },
    run,
};

/// GPU node primals deployed on ironGate.
const GPU_NODE_PRIMALS: &[&str] = &[
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
];

/// Dispatch methods with accepted registry aliases (primary first).
const DISPATCH_METHODS: &[(&str, &[&str], &str, &str)] = &[
    (
        "compute.dispatch_gpu",
        &["compute.gpu", "compute.dispatch.submit"],
        "compute",
        primal_names::TOADSTOOL,
    ),
    (
        "ml.train",
        &["ml.mlp_train", "ml.lstm_train", "nautilus.train"],
        "ml",
        primal_names::BARRACUDA,
    ),
    (
        "ml.infer",
        &["ml.esn_predict", "ml.lstm_predict", "nautilus.predict"],
        "ml",
        primal_names::BARRACUDA,
    ),
    (
        "shader.compile",
        &[
            "shader.compile.wgsl",
            "shader.compile.spirv",
            "shader.compile.module",
        ],
        "shader",
        primal_names::CORALREEF,
    ),
];

/// Methods whose names or domains indicate GPU requirement in the registry.
const GPU_TAGGED_METHODS: &[&str] = &[
    "compute.gpu",
    "compute.dispatch.submit",
    "tensor.matmul",
    "shader.compile.spirv",
];

/// Run all cross-gate GPU dispatch validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Dispatch topology");
    phase_dispatch_topology(v);

    v.section("Phase 2: Dual-target validation");
    phase_dual_target(v);

    v.section("Phase 3: Method routing");
    phase_method_routing(v);

    v.section("Phase 4: Live probe");
    phase_live_probe(v, ctx);
}

fn registry_has_any(candidates: &[&'static str]) -> Option<&'static str> {
    candidates
        .iter()
        .find(|method| REGISTRY_TOML.contains(**method))
        .copied()
}

fn phase_dispatch_topology(v: &mut ValidationResult) {
    let parsed: Result<toml::Value, _> = MANIFEST_TOML.parse();
    let Ok(manifest) = parsed else {
        v.check_bool(
            "topology:manifest_parses",
            false,
            "ecosystem_manifest.toml failed to parse",
        );
        return;
    };

    v.check_bool(
        "topology:manifest_parses",
        true,
        "ecosystem_manifest.toml valid TOML",
    );

    let gates = manifest.get("gates").and_then(|g| g.as_table());
    let Some(gates) = gates else {
        v.check_bool("topology:gates_section", false, "no [gates] section");
        return;
    };

    let gpu_gates: Vec<&String> = gates
        .iter()
        .filter(|(_, gate)| gate.get("gpu_target").and_then(|v| v.as_str()).is_some())
        .map(|(name, _)| name)
        .collect();

    v.check_bool(
        "topology:irongate_only_gpu_gate",
        gpu_gates == ["ironGate"],
        &format!("gates with gpu_target (expect only ironGate): {gpu_gates:?}"),
    );

    let iron_addr = mesh_address("ironGate");
    v.check_bool(
        "topology:irongate_mesh_route",
        iron_addr == Some("10.13.37.7"),
        &format!("ironGate mesh route: {iron_addr:?} (expect 10.13.37.7)"),
    );

    let east_addr = mesh_address("eastGate");
    v.check_bool(
        "topology:eastgate_orchestrator",
        east_addr.is_some(),
        &format!("eastGate orchestrator mesh address: {east_addr:?}"),
    );

    let fleet_gpu = registry_has_any(&["fleet.dispatch_gpu", "fleet.submit"]);
    v.check_bool(
        "topology:fleet_gpu_dispatch",
        fleet_gpu.is_some(),
        &format!(
            "toadStool fleet GPU dispatch registered: {}",
            fleet_gpu.unwrap_or("(missing)")
        ),
    );

    let fleet_owner = capability_to_primal("fleet");
    v.check_bool(
        "topology:fleet_routes_toadstool",
        fleet_owner == primal_names::TOADSTOOL,
        &format!("fleet → \"{fleet_owner}\" (expected toadstool)"),
    );
}

fn phase_dual_target(v: &mut ValidationResult) {
    let Ok(parsed) = MANIFEST_TOML.parse::<toml::Value>() else {
        v.check_bool("dual:manifest_parses", false, "manifest parse failed");
        return;
    };

    let Some(gates) = parsed.get("gates").and_then(|g| g.as_table()) else {
        v.check_bool("dual:gates_section", false, "no [gates] section");
        return;
    };

    let Some(iron) = gates.get("ironGate") else {
        v.check_bool(
            "dual:irongate_present",
            false,
            "ironGate missing from manifest",
        );
        return;
    };

    let main_target = iron.get("target").and_then(|v| v.as_str()).unwrap_or("");
    let gpu_target = iron
        .get("gpu_target")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    v.check_bool(
        "dual:musl_general_target",
        main_target.contains("musl"),
        &format!("ironGate.target = \"{main_target}\" (expect musl general build)"),
    );

    v.check_bool(
        "dual:gnu_gpu_target",
        gpu_target.contains("gnu"),
        &format!("ironGate.gpu_target = \"{gpu_target}\" (expect gnu GPU build)"),
    );

    v.check_bool(
        "dual:targets_differ",
        main_target != gpu_target,
        &format!("dual-target: main=\"{main_target}\" gpu=\"{gpu_target}\""),
    );

    let repos = iron
        .get("repos")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    for (repo, slug) in [
        ("barraCuda", primal_names::BARRACUDA),
        ("coralReef", primal_names::CORALREEF),
    ] {
        let in_gate = repos.contains(&repo);
        v.check_bool(
            &format!("dual:{slug}_on_irongate"),
            in_gate,
            &format!("{repo} listed in ironGate repos (gnu GPU node association)"),
        );

        let port = ports::default_port_for(slug);
        v.check_bool(
            &format!("dual:{slug}_port"),
            port > 0,
            &format!("{slug} port = {port}"),
        );
    }
}

fn phase_method_routing(v: &mut ValidationResult) {
    let mut routed = 0usize;

    for (canonical, aliases, domain, expected_primal) in DISPATCH_METHODS {
        let registered = registry_has_any(aliases);
        let present = registered.is_some();
        if present {
            routed += 1;
        }
        v.check_bool(
            &format!("route:{}", canonical.replace('.', "_")),
            present,
            &format!(
                "{canonical} registered as {}",
                registered.unwrap_or("(missing)")
            ),
        );

        let domain_primal = capability_to_primal(domain);
        v.check_bool(
            &format!("route:{domain}_domain"),
            domain_primal == *expected_primal,
            &format!("{domain} → \"{domain_primal}\" (expected {expected_primal})"),
        );

        if let Some(method) = registered {
            let resolved_domain = method_to_capability_domain(method);
            let method_primal = capability_to_primal(resolved_domain);
            v.check_bool(
                &format!("route:method_{}", method.replace('.', "_")),
                method_primal == *expected_primal,
                &format!("{method} → domain '{resolved_domain}' → \"{method_primal}\""),
            );
        }
    }

    v.check_bool(
        "route:dispatch_method_coverage",
        routed >= 4,
        &format!(
            "{routed}/{} dispatch methods registered",
            DISPATCH_METHODS.len()
        ),
    );

    let mut gpu_tagged = 0usize;
    for method in GPU_TAGGED_METHODS {
        if REGISTRY_TOML.contains(method) {
            gpu_tagged += 1;
        }
        v.check_bool(
            &format!("route:gpu_tagged_{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered (requires_gpu family)"),
        );
    }

    v.check_bool(
        "route:gpu_tagged_methods",
        gpu_tagged >= 3,
        &format!(
            "{gpu_tagged}/{} GPU-tagged methods in registry",
            GPU_TAGGED_METHODS.len()
        ),
    );

    for slug in GPU_NODE_PRIMALS {
        let port = ports::default_port_for(slug);
        v.check_bool(
            &format!("route:{slug}_port"),
            port > 0,
            &format!("{slug} dispatch port = {port}"),
        );
    }
}

fn phase_live_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if mesh_address("ironGate").is_none() {
        v.check_skip(
            "live:irongate_reachable",
            "ironGate mesh address not assigned",
        );
        return;
    }

    match ctx.client_for("compute") {
        Some(client) => {
            let resp = client.call("compute.health", serde_json::json!({}));
            match resp {
                Ok(r) => {
                    v.check_bool(
                        "live:compute_health",
                        r.is_success(),
                        "compute.health probe on ironGate dispatch path",
                    );
                }
                Err(e) if e.is_skippable() => {
                    v.check_skip("live:compute_health", &format!("compute.health: {e}"));
                }
                Err(e) => {
                    v.check_bool("live:compute_health", false, &format!("{e}"));
                }
            }
        }
        None => {
            v.check_skip(
                "live:compute_health",
                "no compute client (ironGate offline)",
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
    fn gpu_dispatch_cross_gate_structural() {
        let mut v = ValidationResult::new("gpu-dispatch-cross-gate");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 20, "expected ≥20 checks, got {total}");
    }

    #[test]
    fn dispatch_methods_registered() {
        for (canonical, aliases, _, _) in DISPATCH_METHODS {
            assert!(
                registry_has_any(aliases).is_some(),
                "{canonical} should have a registered alias in capability_registry.toml"
            );
        }
    }

    #[test]
    fn dual_target_model() {
        let parsed: toml::Value = MANIFEST_TOML.parse().unwrap();
        let iron = parsed.get("gates").and_then(|g| g.get("ironGate")).unwrap();
        let main = iron.get("target").and_then(|v| v.as_str()).unwrap();
        let gpu = iron.get("gpu_target").and_then(|v| v.as_str()).unwrap();
        assert!(main.contains("musl"));
        assert!(gpu.contains("gnu"));
        assert_ne!(main, gpu);
    }

    #[test]
    fn gpu_gate_identification() {
        let parsed: toml::Value = MANIFEST_TOML.parse().unwrap();
        let gates = parsed.get("gates").unwrap().as_table().unwrap();
        let gpu_gates: Vec<&str> = gates
            .iter()
            .filter(|(_, gate)| gate.get("gpu_target").is_some())
            .map(|(name, _)| name.as_str())
            .collect();
        assert_eq!(gpu_gates, vec!["ironGate"]);
    }

    #[test]
    fn compute_trio_routing() {
        assert_eq!(capability_to_primal("compute"), primal_names::TOADSTOOL);
        assert_eq!(capability_to_primal("ml"), primal_names::BARRACUDA);
        assert_eq!(capability_to_primal("shader"), primal_names::CORALREEF);
    }

    #[test]
    fn irongate_mesh_address() {
        assert_eq!(mesh_address("ironGate"), Some("10.13.37.7"));
    }

    #[test]
    fn gpu_dispatch_cross_gate_full_run() {
        let mut v = ValidationResult::new("gpu-dispatch-cross-gate");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "structural phases should pass: {v:?}");
    }
}
