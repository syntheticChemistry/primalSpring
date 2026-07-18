// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Gate Expansion Readiness — validates infrastructure for
//! enrolling new gates (northGate, westGate) into the ecosystem.
//!
//! Phase 1 (Structural): Verify `GateSpecialization` model covers all planned
//! gate roles, bonding policies are consistent, and composition graphs exist
//! for both `FullNucleus` (13/13) and Nest Atomic (7/7) deployments.
//!
//! Phase 2 (Structural): Verify `x86_64` binary depot has required primals
//! for each target composition and deployment infrastructure exists.
//!
//! Phase 3 (Live): Probe existing NUCLEUS health to confirm the seed gate
//! (eastGate) can serve as mesh anchor for new gate enrollment.

use crate::bonding::{BondType, GateSpecialization};
use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Gate expansion readiness scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gate-expansion-readiness",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave111_gate_expansion",
        provenance_date: "2026-06-11",
        description: "Gate expansion readiness — northGate (13/13) + westGate (7/7 Nest) enrollment prerequisites",
    },
    run,
};

/// Execute all gate expansion readiness phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — GateSpecialization model");
    phase_specialization_model(v);

    v.section("Phase 2: Structural — composition graphs + depot inventory");
    phase_depot_readiness(v);

    v.section("Phase 3: Live — seed gate mesh anchor health");
    phase_seed_gate_health(v, ctx);
}

fn phase_specialization_model(v: &mut ValidationResult) {
    let north = GateSpecialization::ComputeHeavy;
    let west = GateSpecialization::ColdStorage;
    let east = GateSpecialization::FullNucleus;

    v.check_bool(
        "model:northgate_exports",
        !north.natural_exports().is_empty(),
        &format!(
            "northGate (ComputeHeavy) exports: {:?}",
            north.natural_exports()
        ),
    );

    v.check_bool(
        "model:westgate_exports",
        !west.natural_exports().is_empty(),
        &format!(
            "westGate (ColdStorage) exports: {:?}",
            west.natural_exports()
        ),
    );

    v.check_bool(
        "model:northgate_imports_storage",
        north
            .natural_imports()
            .iter()
            .any(|i| i.starts_with("storage")),
        "northGate (ComputeHeavy) imports storage.* from bonded peers",
    );

    v.check_bool(
        "model:westgate_imports_compute",
        west.natural_imports()
            .iter()
            .any(|i| i.starts_with("compute")),
        "westGate (ColdStorage) imports compute.* from bonded peers",
    );

    v.check_bool(
        "model:complementary_bond",
        {
            let n_exports = north.natural_exports();
            let w_imports = west.natural_imports();
            w_imports.iter().any(|imp| {
                n_exports
                    .iter()
                    .any(|exp| crate::bonding::exports_satisfy(exp, imp))
            })
        },
        "northGate exports satisfy westGate imports (complementary bond)",
    );

    v.check_bool(
        "model:intra_family_covalent",
        north.default_intra_family_bond() == BondType::Covalent
            && west.default_intra_family_bond() == BondType::Covalent
            && east.default_intra_family_bond() == BondType::Covalent,
        "all non-relay gates default to Covalent intra-family bond",
    );

    let nucleus_primals = AtomicType::FullNucleus.required_capabilities().len();
    let nest_primals = AtomicType::Nest.required_capabilities().len();
    v.check_bool(
        "model:nucleus_covers_13",
        nucleus_primals >= 13,
        &format!("FullNucleus composition requires {nucleus_primals} capabilities (≥13)"),
    );
    v.check_bool(
        "model:nest_covers_7",
        nest_primals >= 7,
        &format!("Nest Atomic composition requires {nest_primals} capabilities (≥7)"),
    );
}

fn phase_depot_readiness(v: &mut ValidationResult) {
    let depot_root =
        ecoprimals_root().map(|r| r.join("infra/plasmidBin/primals/x86_64-unknown-linux-musl"));

    let Some(depot_path) = depot_root else {
        v.check_skip(
            "depot:x86_64_exists",
            "ecoPrimals workspace root not found (ECOPRIMALS_ROOT / CARGO_MANIFEST_DIR)",
        );
        return;
    };

    let depot_exists = depot_path.is_dir();
    v.check_bool(
        "depot:x86_64_exists",
        depot_exists,
        &format!(
            "x86_64-unknown-linux-musl depot exists at {}",
            depot_path.display()
        ),
    );

    if !depot_exists {
        v.check_skip("depot:binary_count", "x86_64 depot directory not found");
        return;
    }

    let all_primals: Vec<&str> = primal_names::Primal::ALL.iter().map(|p| p.slug()).collect();
    let mut present = 0usize;
    let mut missing = Vec::new();

    for primal in &all_primals {
        let binary_path = depot_path.join(primal);
        if binary_path.exists() {
            present += 1;
        } else {
            missing.push(*primal);
        }
    }

    v.check_bool(
        "depot:binary_count",
        present == all_primals.len(),
        &format!(
            "{present}/{} x86_64 binaries present{}",
            all_primals.len(),
            if missing.is_empty() {
                String::new()
            } else {
                format!(" (missing: {})", missing.join(", "))
            }
        ),
    );

    let checksums_path = depot_path.join("../checksums.toml");
    let checksums_exist = checksums_path.exists();
    v.check_bool(
        "depot:checksums_exist",
        checksums_exist,
        "x86_64 checksums.toml exists for depot integrity verification",
    );

    let provenance_path = depot_path.join("../provenance.toml");
    let provenance_exists = provenance_path.exists();
    v.check_bool(
        "depot:provenance_exists",
        provenance_exists,
        "provenance.toml exists for build traceability",
    );

    let nucleus_graph =
        ecoprimals_root().map(|r| r.join("springs/primalSpring/graphs/nucleus_complete.toml"));
    v.check_bool(
        "depot:nucleus_graph_exists",
        nucleus_graph.as_ref().is_some_and(|p| p.exists()),
        "nucleus_complete.toml deploy graph exists (northGate target)",
    );

    let nest_graph =
        ecoprimals_root().map(|r| r.join("springs/primalSpring/graphs/nest_atomic.toml"));
    v.check_bool(
        "depot:nest_graph_exists",
        nest_graph.as_ref().is_some_and(|p| p.exists()),
        "nest_atomic.toml deploy graph exists (westGate target)",
    );
}

fn phase_seed_gate_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip(
            "seed:discovery_available",
            "discovery (songBird) not available — cannot validate mesh anchor",
        );
        return;
    }

    v.check_bool(
        "seed:discovery_available",
        true,
        "songBird discovery capability available (mesh anchor for gate enrollment)",
    );

    let mesh_status = ctx.call("discovery", "federation.status", serde_json::json!({}));
    match mesh_status {
        Ok(resp) => {
            let enabled = resp
                .get("enabled")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "seed:federation_enabled",
                enabled,
                &format!("songBird federation enabled: {enabled}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "seed:federation_enabled",
                false,
                &format!("federation.status call failed: {e} (may need TCP federation port)"),
            );
        }
    }

    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "seed:biomeos_orchestration",
            "biomeOS orchestration not available",
        );
        return;
    }

    let health_result = ctx.call("orchestration", "health.liveness", serde_json::json!({}));
    v.check_bool(
        "seed:biomeos_orchestration",
        health_result.is_ok(),
        &format!(
            "biomeOS orchestration alive (required for gate enrollment): {}",
            match &health_result {
                Ok(_) => "OK".to_owned(),
                Err(e) => format!("{e}"),
            }
        ),
    );

    let btsp_state = ctx.btsp_state().clone();
    let authenticated_count = btsp_state.values().filter(|&&a| a).count();
    v.check_bool(
        "seed:btsp_active",
        authenticated_count >= 1,
        &format!(
            "seed gate has BTSP-authenticated channels ({authenticated_count}) — new gates can trust this anchor"
        ),
    );
}

fn ecoprimals_root() -> Option<std::path::PathBuf> {
    if let Ok(root) = std::env::var("ECOPRIMALS_ROOT") {
        return Some(std::path::PathBuf::from(root));
    }
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest_dir.join("../../..");
    if candidate.join("infra").is_dir() {
        Some(candidate.canonicalize().unwrap_or(candidate))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn gate_expansion_readiness_structural() {
        let mut v = ValidationResult::new("gate-expansion-readiness");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed > 0,
            "gate-expansion-readiness must produce checks ({} passed)",
            v.passed
        );
    }
}
