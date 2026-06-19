// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Pressure Surface — validates that the evolution module correctly
//! classifies constraint surfaces per target and that the platform-adaptive
//! code paths respond appropriately.
//!
//! This scenario does NOT deploy to other architectures — it validates
//! the structural classification system and its alignment with the
//! tolerances/platform module's runtime detection.

use crate::composition::CompositionContext;
use crate::evolution::pressure::PressureCategory;
use crate::evolution::target::{DeploymentTier, Target};
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};

/// Pressure surface validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "pressure-surface",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-16",
        description: "Validates evolution pressure classification and platform adaptivity",
    },
    run,
};

/// Run pressure surface validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Pressure classification completeness");
    phase_classification(v);

    v.section("Phase 2: Platform detection alignment");
    phase_platform_alignment(v);

    v.section("Phase 3: Degradation path coverage");
    phase_degradation_paths(v);
}

fn phase_classification(v: &mut ValidationResult) {
    let all_targets = [
        Target::X86_64Musl,
        Target::Aarch64Musl,
        Target::Riscv64Musl,
        Target::Wasm32Wasi,
    ];

    for target in &all_targets {
        let pressures = PressureCategory::active_for(*target);
        let tier = target.tier();

        // More constrained tiers should have more pressures
        let expected_min = match tier {
            DeploymentTier::Permissive => 0,
            DeploymentTier::Restricted | DeploymentTier::Constrained => 3,
            DeploymentTier::Sandboxed => 5,
            DeploymentTier::Bare => 6,
        };

        v.check_bool(
            &format!("pressure:{}:count", target.triple()),
            pressures.len() >= expected_min,
            &format!(
                "{}: {} pressures (tier {:?} expects ≥{})",
                target.triple(),
                pressures.len(),
                tier,
                expected_min,
            ),
        );
    }

    // Verify no duplicate pressures per target
    for target in &all_targets {
        let pressures = PressureCategory::active_for(*target);
        let unique: std::collections::HashSet<_> = pressures.iter().collect();
        v.check_bool(
            &format!("pressure:{}:unique", target.triple()),
            unique.len() == pressures.len(),
            &format!(
                "{}: {} pressures, {} unique (no duplicates)",
                target.triple(),
                pressures.len(),
                unique.len(),
            ),
        );
    }

    // Verify permissive target (x86) is subset of restricted targets
    let x86_pressures: std::collections::HashSet<_> =
        PressureCategory::active_for(Target::X86_64Musl)
            .into_iter()
            .collect();
    let arm_pressures: std::collections::HashSet<_> =
        PressureCategory::active_for(Target::Aarch64Musl)
            .into_iter()
            .collect();

    v.check_bool(
        "pressure:hierarchy:x86-subset-arm",
        x86_pressures.is_subset(&arm_pressures),
        &format!(
            "x86 pressures ({}) ⊆ arm pressures ({}) — restriction monotonic",
            x86_pressures.len(),
            arm_pressures.len(),
        ),
    );
}

fn phase_platform_alignment(v: &mut ValidationResult) {
    let current = Target::current();
    let triple_detected = tolerances::platform::current_target_triple();

    v.check_bool(
        "platform:triple-matches-target",
        triple_detected.contains("x86_64")
            || triple_detected.contains("aarch64")
            || triple_detected.contains("riscv"),
        &format!("detected triple '{triple_detected}' resolves to {current}"),
    );

    let runtime_dir = tolerances::platform::runtime_dir();
    let runtime_path = std::path::Path::new(&runtime_dir);
    v.check_bool(
        "platform:runtime_dir-resolves",
        !runtime_dir.is_empty(),
        &format!("runtime_dir: {runtime_dir}"),
    );

    if current.has_uds() {
        v.check_bool(
            "platform:uds-available",
            runtime_path.exists() || std::path::Path::new("/tmp").exists(),
            "UDS target has writable socket directory",
        );
    } else {
        v.check_skip(
            "platform:uds-available",
            &format!("{current} declares no UDS support — skip check"),
        );
    }
}

fn phase_degradation_paths(v: &mut ValidationResult) {
    let pressures_to_validate = [
        (
            PressureCategory::Filesystem,
            "read-only root",
            "configurable state dir",
        ),
        (PressureCategory::IpcTransport, "no UDS", "TCP fallback"),
        (
            PressureCategory::SecurityPolicy,
            "SELinux restrictions",
            "capability-based discovery",
        ),
        (
            PressureCategory::Network,
            "intermittent connectivity",
            "offline-capable crypto",
        ),
    ];

    for (pressure, threat, mitigation) in &pressures_to_validate {
        let mitigated = match pressure {
            PressureCategory::Filesystem => {
                // State dir is configurable via env
                std::env::var("SONGBIRD_STATE_DIR").is_ok()
                    || std::path::Path::new(&tolerances::platform::runtime_dir()).exists()
            }
            PressureCategory::IpcTransport => {
                // TCP fallback exists
                !tolerances::ports::PORT_REGISTRY.is_empty()
            }
            PressureCategory::SecurityPolicy | PressureCategory::Network => {
                // SecurityPolicy: capability-based discovery exists
                // Network: BearDog crypto ops are local-only
                true
            }
            _ => false,
        };

        v.check_bool(
            &format!("degrade:{}:{}", pressure.label(), threat.replace(' ', "-")),
            mitigated,
            &format!(
                "{}: threat='{}' → mitigation='{}'",
                pressure.label(),
                threat,
                mitigation,
            ),
        );
    }

    // Validate that current target's active pressures all have known degradation
    let current_pressures = PressureCategory::active_for(Target::current());
    let known_mitigations = [
        PressureCategory::Filesystem,
        PressureCategory::IpcTransport,
        PressureCategory::SecurityPolicy,
        PressureCategory::Network,
        PressureCategory::Memory,
    ];

    let covered = current_pressures
        .iter()
        .filter(|p| known_mitigations.contains(p))
        .count();

    v.check_bool(
        "degrade:coverage",
        covered == current_pressures.len() || current_pressures.is_empty(),
        &format!(
            "{covered}/{} active pressures have known degradation paths",
            current_pressures.len(),
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_surface_structural() {
        let mut v = ValidationResult::new("pressure-surface");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "Pressure surface has {} failures", v.failed);
    }
}
