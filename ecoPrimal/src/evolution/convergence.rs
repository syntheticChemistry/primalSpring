// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Convergence monitoring — continuous ecosystem health reconciliation.
//!
//! Where `GateMatrix` tracks point-in-time readiness, the convergence monitor
//! tracks *drift* over time. It detects when gates diverge from the expected
//! ecosystem state and produces actionable reconciliation signals.
//!
//! # Model
//!
//! ```text
//! Convergence = f(VCS parity, depot freshness, primal liveness, mesh connectivity)
//! ```
//!
//! Each dimension produces a `DriftSignal` when it detects deviation from the
//! expected state. The monitor aggregates signals into an `EcosystemConvergence`
//! score that overwatch uses to prioritize attention.

use super::gate::{GateMatrix, GateStatus, ReadinessLevel};

/// A signal that some dimension of ecosystem convergence has drifted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftSignal {
    /// Which gate is drifting.
    pub gate: String,
    /// What dimension drifted (vcs, depot, liveness, mesh).
    pub dimension: DriftDimension,
    /// Severity: how far from convergence.
    pub severity: DriftSeverity,
    /// Human-readable description.
    pub detail: String,
}

/// Dimensions of ecosystem convergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriftDimension {
    /// VCS remote parity (Forgejo ↔ GitHub ↔ local).
    Vcs,
    /// Depot binary freshness (HEAD alignment).
    Depot,
    /// Primal liveness (alive/expected ratio).
    Liveness,
    /// Mesh peer connectivity.
    Mesh,
}

impl DriftDimension {
    /// Short label for display.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Vcs => "vcs",
            Self::Depot => "depot",
            Self::Liveness => "liveness",
            Self::Mesh => "mesh",
        }
    }
}

/// How severe is the drift?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DriftSeverity {
    /// Within tolerance — informational only.
    Nominal,
    /// Drifting — attention recommended within hours.
    Warning,
    /// Diverged — reconciliation needed now.
    Critical,
}

/// Aggregate ecosystem convergence state.
#[derive(Debug, Clone)]
pub struct EcosystemConvergence {
    /// Convergence score (0.0 = fully diverged, 1.0 = fully converged).
    pub score: f64,
    /// Active drift signals across all gates.
    pub signals: Vec<DriftSignal>,
    /// Number of gates at full convergence.
    pub gates_converged: u32,
    /// Total gates monitored.
    pub gates_total: u32,
}

impl EcosystemConvergence {
    /// Compute convergence from a gate matrix.
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "gate count is always < 256"
    )]
    pub fn from_matrix(matrix: &GateMatrix) -> Self {
        let mut signals = Vec::new();
        let mut converged = 0u32;
        let total = matrix.gates.len() as u32;

        for gate in &matrix.gates {
            let gate_signals = check_gate_convergence(gate);
            if gate_signals.is_empty() {
                converged += 1;
            }
            signals.extend(gate_signals);
        }

        let score = if total == 0 {
            1.0
        } else {
            f64::from(converged) / f64::from(total)
        };

        Self {
            score,
            signals,
            gates_converged: converged,
            gates_total: total,
        }
    }

    /// Whether the ecosystem is fully converged (no drift signals above Nominal).
    #[must_use]
    pub fn is_converged(&self) -> bool {
        self.signals
            .iter()
            .all(|s| s.severity == DriftSeverity::Nominal)
    }

    /// Critical signals requiring immediate attention.
    #[must_use]
    pub fn critical_signals(&self) -> Vec<&DriftSignal> {
        self.signals
            .iter()
            .filter(|s| s.severity == DriftSeverity::Critical)
            .collect()
    }

    /// Human-readable one-line summary.
    #[must_use]
    pub fn summary(&self) -> String {
        let critical = self
            .signals
            .iter()
            .filter(|s| s.severity == DriftSeverity::Critical)
            .count();
        let warnings = self
            .signals
            .iter()
            .filter(|s| s.severity == DriftSeverity::Warning)
            .count();

        if critical > 0 {
            format!(
                "DIVERGED — {}/{} converged, {} critical, {} warnings",
                self.gates_converged, self.gates_total, critical, warnings
            )
        } else if warnings > 0 {
            format!(
                "DRIFTING — {}/{} converged, {} warnings",
                self.gates_converged, self.gates_total, warnings
            )
        } else {
            format!(
                "CONVERGED — {}/{} gates green",
                self.gates_converged, self.gates_total
            )
        }
    }
}

/// Check a single gate for convergence drift.
fn check_gate_convergence(gate: &GateStatus) -> Vec<DriftSignal> {
    let mut signals = Vec::new();

    // VCS parity
    if !gate.vcs_synced {
        signals.push(DriftSignal {
            gate: gate.name.clone(),
            dimension: DriftDimension::Vcs,
            severity: DriftSeverity::Warning,
            detail: format!("{}: VCS not synced", gate.name),
        });
    }

    // Depot freshness
    if !gate.depot_fresh {
        signals.push(DriftSignal {
            gate: gate.name.clone(),
            dimension: DriftDimension::Depot,
            severity: DriftSeverity::Warning,
            detail: format!("{}: depot stale", gate.name),
        });
    }

    // Liveness
    if gate.primals_expected > 0 {
        let ratio = f64::from(gate.primals_alive) / f64::from(gate.primals_expected);
        let severity = if ratio < 0.5 {
            DriftSeverity::Critical
        } else if ratio < 1.0 {
            DriftSeverity::Warning
        } else {
            DriftSeverity::Nominal
        };

        if severity > DriftSeverity::Nominal {
            signals.push(DriftSignal {
                gate: gate.name.clone(),
                dimension: DriftDimension::Liveness,
                severity,
                detail: format!(
                    "{}: {}/{} primals alive ({:.0}%)",
                    gate.name,
                    gate.primals_alive,
                    gate.primals_expected,
                    ratio * 100.0
                ),
            });
        }
    }

    // Mesh connectivity
    if gate.mesh_peers == 0 && gate.readiness >= ReadinessLevel::Partial {
        signals.push(DriftSignal {
            gate: gate.name.clone(),
            dimension: DriftDimension::Mesh,
            severity: DriftSeverity::Warning,
            detail: format!("{}: no mesh peers (isolated)", gate.name),
        });
    }

    signals
}

#[cfg(test)]
mod tests {
    use super::super::gate::CytoplasmZone;
    use super::*;

    fn healthy_gate(name: &str) -> GateStatus {
        GateStatus {
            name: name.to_owned(),
            readiness: ReadinessLevel::Full,
            zone: CytoplasmZone::for_gate(name),
            primals_alive: 13,
            primals_expected: 13,
            depot_fresh: true,
            vcs_synced: true,
            mesh_peers: 2,
            last_seen: Some(1_718_000_000),
            notes: String::new(),
        }
    }

    #[test]
    fn fully_converged_ecosystem() {
        let matrix = GateMatrix {
            gates: vec![healthy_gate("eastGate"), healthy_gate("sporeGate")],
        };
        let conv = EcosystemConvergence::from_matrix(&matrix);
        assert!(conv.is_converged());
        assert_eq!(conv.gates_converged, 2);
        assert!((conv.score - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn drifted_gate_detected() {
        let mut drifted = healthy_gate("golgi");
        drifted.vcs_synced = false;
        drifted.primals_alive = 10;

        let matrix = GateMatrix {
            gates: vec![healthy_gate("eastGate"), drifted],
        };
        let conv = EcosystemConvergence::from_matrix(&matrix);
        assert!(!conv.is_converged());
        assert_eq!(conv.gates_converged, 1);
        assert!(
            conv.signals
                .iter()
                .any(|s| s.dimension == DriftDimension::Vcs)
        );
        assert!(
            conv.signals
                .iter()
                .any(|s| s.dimension == DriftDimension::Liveness)
        );
    }

    #[test]
    fn critical_liveness() {
        let mut critical = healthy_gate("fieldGate");
        critical.primals_alive = 3;

        let matrix = GateMatrix {
            gates: vec![critical],
        };
        let conv = EcosystemConvergence::from_matrix(&matrix);
        let crits = conv.critical_signals();
        assert_eq!(crits.len(), 1);
        assert_eq!(crits[0].dimension, DriftDimension::Liveness);
    }

    #[test]
    fn empty_matrix_converged() {
        let matrix = GateMatrix { gates: vec![] };
        let conv = EcosystemConvergence::from_matrix(&matrix);
        assert!(conv.is_converged());
        assert!((conv.score - 1.0).abs() < f64::EPSILON);
    }
}
