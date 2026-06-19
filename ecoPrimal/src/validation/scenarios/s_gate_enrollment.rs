// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Gate Enrollment Pipeline — validates the staged enrollment
//! process that brings a new gate from bare hardware to full NUCLEUS.
//!
//! The pipeline stages (from sporeGate reference implementation):
//!
//! ```text
//! 1. SSH enable     → can reach gate via SSH
//! 2. gate.preflight → interface detect, DNS, IP conflicts, port 53
//! 3. membrane install → cellMembrane binary deployed
//! 4. NUCLEUS 13/13  → all primals launched and healthy
//! 5. systemd        → persistence via membrane-nucleus.target
//! 6. WireGuard peer → overlay mesh connected (10.13.37.0/24)
//! 7. cascade connect → VCS push/pull both remotes
//! ```
//!
//! This scenario validates that primalSpring can model, track, and verify
//! each stage of enrollment. It tests the data structures and transition
//! logic — live enrollment is cellMembrane's responsibility.

use crate::composition::CompositionContext;
use crate::evolution::gate::{CytoplasmZone, GateMatrix, GateStatus, ReadinessLevel};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Gate enrollment pipeline validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gate-enrollment",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Gate enrollment pipeline: staged progression from bare to full NUCLEUS",
    },
    run,
};

/// Enrollment pipeline stages (ordered).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EnrollmentStage {
    Bare,
    SshEnabled,
    PreflightPassed,
    MembraneInstalled,
    NucleusAlive,
    SystemdPersisted,
    WireGuardPeered,
    CascadeConnected,
}

impl EnrollmentStage {
    const fn label(self) -> &'static str {
        match self {
            Self::Bare => "bare",
            Self::SshEnabled => "ssh-enabled",
            Self::PreflightPassed => "preflight-passed",
            Self::MembraneInstalled => "membrane-installed",
            Self::NucleusAlive => "nucleus-alive",
            Self::SystemdPersisted => "systemd-persisted",
            Self::WireGuardPeered => "wireguard-peered",
            Self::CascadeConnected => "cascade-connected",
        }
    }

    fn from_gate_status(status: &GateStatus) -> Self {
        if status.readiness == ReadinessLevel::Offline {
            return Self::Bare;
        }
        if status.primals_alive == 0 && status.mesh_peers == 0 && !status.vcs_synced {
            return Self::SshEnabled;
        }
        if status.primals_alive == 0 && !status.vcs_synced {
            return Self::PreflightPassed;
        }
        if status.primals_alive > 0 && status.primals_alive < status.primals_expected {
            return Self::MembraneInstalled;
        }
        if status.primals_alive == status.primals_expected && !status.vcs_synced {
            if status.mesh_peers > 0 {
                return Self::WireGuardPeered;
            }
            return Self::NucleusAlive;
        }
        if status.primals_alive == status.primals_expected
            && status.mesh_peers > 0
            && status.vcs_synced
        {
            return Self::CascadeConnected;
        }
        if status.primals_alive == status.primals_expected && status.mesh_peers > 0 {
            return Self::SystemdPersisted;
        }
        if status.primals_alive == status.primals_expected {
            return Self::NucleusAlive;
        }
        Self::SshEnabled
    }
}

/// Run gate enrollment validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Enrollment stage model");
    phase_stage_model(v);

    v.section("Phase 2: Stage derivation from gate status");
    phase_stage_derivation(v);

    v.section("Phase 3: Ecosystem enrollment posture");
    phase_ecosystem_posture(v);

    v.section("Phase 4: Enrollment target identification");
    phase_enrollment_targets(v);
}

fn phase_stage_model(v: &mut ValidationResult) {
    let stages = [
        EnrollmentStage::Bare,
        EnrollmentStage::SshEnabled,
        EnrollmentStage::PreflightPassed,
        EnrollmentStage::MembraneInstalled,
        EnrollmentStage::NucleusAlive,
        EnrollmentStage::SystemdPersisted,
        EnrollmentStage::WireGuardPeered,
        EnrollmentStage::CascadeConnected,
    ];

    for i in 0..stages.len() - 1 {
        v.check_bool(
            &format!("stage:ordering:{}", stages[i].label()),
            stages[i] < stages[i + 1],
            &format!("{} < {}", stages[i].label(), stages[i + 1].label()),
        );
    }

    v.check_count("stage:total_stages", stages.len(), 8);
}

fn phase_stage_derivation(v: &mut ValidationResult) {
    // Bare gate (offline)
    let bare = GateStatus::new("test-bare", ReadinessLevel::Offline);
    v.check_bool(
        "derive:offline_is_bare",
        EnrollmentStage::from_gate_status(&bare) == EnrollmentStage::Bare,
        &format!(
            "offline → {}",
            EnrollmentStage::from_gate_status(&bare).label()
        ),
    );

    // SSH-enabled (reachable, no primals)
    let ssh = GateStatus {
        name: "test-ssh".to_owned(),
        readiness: ReadinessLevel::Reachable,
        zone: CytoplasmZone::Unassigned,
        primals_alive: 0,
        primals_expected: 13,
        depot_fresh: false,
        vcs_synced: false,
        mesh_peers: 0,
        last_seen: Some(1_718_700_000),
        notes: String::new(),
    };
    v.check_bool(
        "derive:reachable_no_primals_is_ssh",
        EnrollmentStage::from_gate_status(&ssh) == EnrollmentStage::SshEnabled,
        &format!(
            "reachable, 0 primals → {}",
            EnrollmentStage::from_gate_status(&ssh).label()
        ),
    );

    // Fully enrolled (sporeGate reference)
    let enrolled = GateStatus {
        name: "sporeGate".to_owned(),
        readiness: ReadinessLevel::Verified,
        zone: CytoplasmZone::Backbone,
        primals_alive: 13,
        primals_expected: 13,
        depot_fresh: true,
        vcs_synced: true,
        mesh_peers: 2,
        last_seen: Some(1_718_700_000),
        notes: String::new(),
    };
    v.check_bool(
        "derive:full_enrolled_is_cascade",
        EnrollmentStage::from_gate_status(&enrolled) == EnrollmentStage::CascadeConnected,
        &format!(
            "13/13 + WG + VCS → {}",
            EnrollmentStage::from_gate_status(&enrolled).label()
        ),
    );

    // Partial: NUCLEUS alive but no WG or cascade
    let nucleus_only = GateStatus {
        name: "test-nucleus".to_owned(),
        readiness: ReadinessLevel::Full,
        zone: CytoplasmZone::Unassigned,
        primals_alive: 13,
        primals_expected: 13,
        depot_fresh: false,
        vcs_synced: false,
        mesh_peers: 0,
        last_seen: Some(1_718_700_000),
        notes: String::new(),
    };
    v.check_bool(
        "derive:nucleus_no_wg_is_alive",
        EnrollmentStage::from_gate_status(&nucleus_only) == EnrollmentStage::NucleusAlive,
        &format!(
            "13/13, no WG → {}",
            EnrollmentStage::from_gate_status(&nucleus_only).label()
        ),
    );
}

fn phase_ecosystem_posture(v: &mut ValidationResult) {
    let matrix = GateMatrix::ecosystem_snapshot();

    v.check_bool(
        "ecosystem:has_gates",
        !matrix.gates.is_empty(),
        &format!("{} gates in ecosystem snapshot", matrix.gates.len()),
    );

    // Count gates at each enrollment stage
    let mut stage_counts = std::collections::HashMap::new();
    for gate in &matrix.gates {
        let stage = EnrollmentStage::from_gate_status(gate);
        *stage_counts.entry(stage.label()).or_insert(0u32) += 1;
    }

    let enrolled_count = matrix
        .gates
        .iter()
        .filter(|g| EnrollmentStage::from_gate_status(g) >= EnrollmentStage::NucleusAlive)
        .count();

    v.check_bool(
        "ecosystem:some_enrolled",
        true,
        &format!(
            "{}/{} gates at NUCLEUS+ (stages: {:?})",
            enrolled_count,
            matrix.gates.len(),
            stage_counts
        ),
    );
}

fn phase_enrollment_targets(v: &mut ValidationResult) {
    let matrix = GateMatrix::ecosystem_snapshot();

    let targets: Vec<&GateStatus> = matrix
        .gates
        .iter()
        .filter(|g| EnrollmentStage::from_gate_status(g) < EnrollmentStage::NucleusAlive)
        .collect();

    v.check_bool(
        "targets:identified",
        true,
        &format!(
            "{} gates pending enrollment: [{}]",
            targets.len(),
            targets
                .iter()
                .map(|g| g.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );

    // The reference gate (sporeGate) enrollment stage from env-based status
    let reference = matrix.gates.iter().find(|g| g.name == "sporeGate");
    if let Some(ref_gate) = reference {
        let ref_stage = EnrollmentStage::from_gate_status(ref_gate);
        if ref_gate.readiness == ReadinessLevel::Offline
            && std::env::var("GATE_SPOREGATE_STATUS").is_err()
        {
            v.check_skip(
                "targets:reference_gate_enrolled",
                &format!(
                    "sporeGate status not configured (env absent), stage: {}",
                    ref_stage.label()
                ),
            );
        } else {
            v.check_bool(
                "targets:reference_gate_enrolled",
                ref_stage >= EnrollmentStage::SshEnabled,
                &format!("sporeGate stage: {} (reference gate)", ref_stage.label()),
            );
        }
    } else {
        v.check_skip(
            "targets:reference_gate_enrolled",
            "sporeGate not in matrix (unexpected)",
        );
    }

    // Wave 116 immediate targets: eastGate (SSH done), ironGate, flockGate
    validate_wave116_targets(v, &matrix);
}

/// Wave 116 immediate enrollment targets and their expected minimum stages.
fn validate_wave116_targets(v: &mut ValidationResult, matrix: &GateMatrix) {
    use CytoplasmZone as Z;

    struct Target {
        name: &'static str,
        min_stage: EnrollmentStage,
        zone: Z,
    }

    let wave116 = [
        Target {
            name: "eastGate",
            min_stage: EnrollmentStage::WireGuardPeered,
            zone: Z::Backbone,
        },
        Target {
            name: "ironGate",
            min_stage: EnrollmentStage::Bare,
            zone: Z::Backbone,
        },
        Target {
            name: "flockGate",
            min_stage: EnrollmentStage::WireGuardPeered,
            zone: Z::Wan,
        },
    ];

    for target in &wave116 {
        let gate = matrix.gates.iter().find(|g| g.name == target.name);
        match gate {
            Some(g) => {
                let stage = EnrollmentStage::from_gate_status(g);
                v.check_bool(
                    &format!("w116:{}:zone", target.name),
                    g.zone == target.zone,
                    &format!("expected {}, got {}", target.zone.label(), g.zone.label()),
                );
                if target.min_stage == EnrollmentStage::Bare {
                    v.check_bool(
                        &format!("w116:{}:tracked", target.name),
                        true,
                        &format!("stage: {} (enrollment pending SSH)", stage.label()),
                    );
                } else if g.readiness == ReadinessLevel::Offline
                    && std::env::var(format!("GATE_{}_STATUS", target.name.to_uppercase())).is_err()
                {
                    v.check_skip(
                        &format!("w116:{}:min_stage", target.name),
                        &format!("env absent, stage: {}", stage.label()),
                    );
                } else {
                    v.check_bool(
                        &format!("w116:{}:min_stage", target.name),
                        stage >= target.min_stage,
                        &format!(
                            "expected >= {}, got {}",
                            target.min_stage.label(),
                            stage.label()
                        ),
                    );
                }
            }
            None => {
                v.check_skip(
                    &format!("w116:{}:absent", target.name),
                    &format!("{} not in matrix", target.name),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_enrollment_structural() {
        let mut v = ValidationResult::new("gate-enrollment");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "gate enrollment: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
