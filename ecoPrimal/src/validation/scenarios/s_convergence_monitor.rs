// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Convergence Monitor — validates the ecosystem convergence
//! detection infrastructure and drift signal generation.
//!
//! Tests that the convergence monitor correctly identifies:
//! - Fully converged ecosystems (all green)
//! - VCS drift (remotes out of sync)
//! - Depot staleness (binaries behind HEAD)
//! - Liveness degradation (primals down)
//! - Mesh isolation (no peers)

use crate::composition::CompositionContext;
use crate::evolution::convergence::{DriftDimension, DriftSeverity, EcosystemConvergence};
use crate::evolution::gate::{GateMatrix, GateStatus, ReadinessLevel};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Convergence monitor validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "convergence-monitor",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-17",
        description: "Convergence monitoring: drift detection, signal generation, reconciliation",
    },
    run,
};

/// Run convergence monitoring validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Full convergence detection");
    phase_full_convergence(v);

    v.section("Phase 2: Drift signal generation");
    phase_drift_signals(v);

    v.section("Phase 3: Severity escalation");
    phase_severity_escalation(v);

    v.section("Phase 4: Real gate matrix assessment");
    phase_real_assessment(v);
}

fn phase_full_convergence(v: &mut ValidationResult) {
    let matrix = GateMatrix {
        gates: vec![
            healthy_gate("eastGate"),
            healthy_gate("sporeGate"),
            healthy_gate("golgi"),
        ],
    };
    let conv = EcosystemConvergence::from_matrix(&matrix);

    v.check_bool(
        "convergence:full:is_converged",
        conv.is_converged(),
        &format!("3 healthy gates → converged (score={:.2})", conv.score),
    );
    v.check_bool(
        "convergence:full:no_critical",
        conv.critical_signals().is_empty(),
        "no critical signals in healthy ecosystem",
    );
    v.check_count("convergence:full:gates_converged", conv.gates_converged as usize, 3);
}

fn phase_drift_signals(v: &mut ValidationResult) {
    let mut vcs_drifted = healthy_gate("golgi");
    vcs_drifted.vcs_synced = false;

    let mut depot_stale = healthy_gate("pepti");
    depot_stale.depot_fresh = false;

    let matrix = GateMatrix {
        gates: vec![healthy_gate("eastGate"), vcs_drifted, depot_stale],
    };
    let conv = EcosystemConvergence::from_matrix(&matrix);

    v.check_bool(
        "drift:vcs:detected",
        conv.signals.iter().any(|s| s.dimension == DriftDimension::Vcs && s.gate == "golgi"),
        "VCS drift detected on golgi",
    );
    v.check_bool(
        "drift:depot:detected",
        conv.signals.iter().any(|s| s.dimension == DriftDimension::Depot && s.gate == "pepti"),
        "depot staleness detected on pepti",
    );
    v.check_bool(
        "drift:not_full_convergence",
        !conv.is_converged(),
        &format!("diverged ecosystem (score={:.2})", conv.score),
    );
    v.check_bool(
        "drift:one_gate_converged",
        conv.gates_converged == 1,
        "only eastGate converged",
    );
}

fn phase_severity_escalation(v: &mut ValidationResult) {
    // Critical: <50% liveness
    let mut critical = healthy_gate("fieldGate");
    critical.primals_alive = 3;
    critical.readiness = ReadinessLevel::Partial;

    // Warning: <100% liveness
    let mut warning = healthy_gate("northGate");
    warning.primals_alive = 10;

    let matrix = GateMatrix {
        gates: vec![critical, warning],
    };
    let conv = EcosystemConvergence::from_matrix(&matrix);

    let critical_signals = conv.critical_signals();
    v.check_bool(
        "severity:critical_liveness",
        critical_signals.iter().any(|s| s.severity == DriftSeverity::Critical),
        &format!("{} critical signal(s) detected", critical_signals.len()),
    );

    let warnings: Vec<_> = conv.signals.iter().filter(|s| s.severity == DriftSeverity::Warning).collect();
    v.check_bool(
        "severity:warning_liveness",
        warnings.iter().any(|s| s.dimension == DriftDimension::Liveness),
        "warning-level liveness drift for partial degradation",
    );

    // Verify severity ordering: Critical > Warning > Nominal
    v.check_bool(
        "severity:ordering",
        DriftSeverity::Critical > DriftSeverity::Warning
            && DriftSeverity::Warning > DriftSeverity::Nominal,
        "severity ordering: Critical > Warning > Nominal",
    );
}

fn phase_real_assessment(v: &mut ValidationResult) {
    // Build a matrix approximating our actual ecosystem from the Wave 115 FRAGO
    let matrix = wave115_matrix();
    let conv = EcosystemConvergence::from_matrix(&matrix);

    v.check_bool(
        "real:summary_generated",
        !conv.summary().is_empty(),
        &format!("summary: {}", conv.summary()),
    );

    v.check_bool(
        "real:score_valid",
        (0.0..=1.0).contains(&conv.score),
        &format!("convergence score: {:.2}", conv.score),
    );

    // eastGate + sporeGate + golgi should be converged (per FRAGO snapshot)
    let live_converged = matrix
        .gates
        .iter()
        .filter(|g| g.readiness >= ReadinessLevel::Full && g.vcs_synced && g.depot_fresh)
        .count();
    v.check_bool(
        "real:live_gates_healthy",
        live_converged >= 2,
        &format!("{live_converged} gates at full convergence"),
    );
}

/// Approximate Wave 115 ecosystem state.
fn wave115_matrix() -> GateMatrix {
    GateMatrix {
        gates: vec![
            GateStatus {
                name: "eastGate".to_owned(),
                readiness: ReadinessLevel::Verified,
                primals_alive: 0,
                primals_expected: 0,
                depot_fresh: true,
                vcs_synced: true,
                mesh_peers: 1,
                last_seen: Some(1_718_650_000),
                notes: "validation node + overwatch".to_owned(),
            },
            GateStatus {
                name: "sporeGate".to_owned(),
                readiness: ReadinessLevel::Full,
                primals_alive: 13,
                primals_expected: 13,
                depot_fresh: true,
                vcs_synced: true,
                mesh_peers: 1,
                last_seen: Some(1_718_650_000),
                notes: "13/13 systemd persisted".to_owned(),
            },
            GateStatus {
                name: "golgi".to_owned(),
                readiness: ReadinessLevel::Full,
                primals_alive: 13,
                primals_expected: 13,
                depot_fresh: true,
                vcs_synced: true,
                mesh_peers: 2,
                last_seen: Some(1_718_650_000),
                notes: "relay + Forgejo".to_owned(),
            },
            GateStatus {
                name: "pepti".to_owned(),
                readiness: ReadinessLevel::Partial,
                primals_alive: 0,
                primals_expected: 13,
                depot_fresh: true,
                vcs_synced: true,
                mesh_peers: 1,
                last_seen: Some(1_718_600_000),
                notes: "build authority only".to_owned(),
            },
            GateStatus {
                name: "northGate".to_owned(),
                readiness: ReadinessLevel::Reachable,
                primals_alive: 0,
                primals_expected: 13,
                depot_fresh: false,
                vcs_synced: false,
                mesh_peers: 0,
                last_seen: None,
                notes: "NUCLEUS deploy pending".to_owned(),
            },
            GateStatus {
                name: "fieldGate".to_owned(),
                readiness: ReadinessLevel::Offline,
                primals_alive: 0,
                primals_expected: 0,
                depot_fresh: false,
                vcs_synced: false,
                mesh_peers: 0,
                last_seen: None,
                notes: "OFFLINE — dead CMOS".to_owned(),
            },
        ],
    }
}

fn healthy_gate(name: &str) -> GateStatus {
    GateStatus {
        name: name.to_owned(),
        readiness: ReadinessLevel::Full,
        primals_alive: 13,
        primals_expected: 13,
        depot_fresh: true,
        vcs_synced: true,
        mesh_peers: 2,
        last_seen: Some(1_718_650_000),
        notes: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_monitor_structural() {
        let mut v = ValidationResult::new("convergence-monitor");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "convergence monitor: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
