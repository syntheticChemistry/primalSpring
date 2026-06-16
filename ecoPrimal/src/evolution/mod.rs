// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Evolution playground — cross-architecture fitness testing infrastructure.
//!
//! This module implements the silicon-atheist principle: any primal behavior
//! that depends on platform assumptions is evolution debt. The fitness function
//! evaluates how well a primal (or composition) survives across different
//! deployment targets and constraint surfaces.
//!
//! # Architecture
//!
//! ```text
//! Target (where)  ×  Pressure (what stress)  →  Debt (what broke)
//!     ↓                                              ↓
//! ArchFitness (per-primal survival matrix)    FitnessScore (scalar)
//! ```
//!
//! # Integration
//!
//! The evolution module consumes [`ValidationResult`](crate::validation::ValidationResult)
//! outputs from standard scenarios and scores them against declared target support.
//! It does NOT replace the boolean pass/fail semantics of validation — it adds
//! a fitness dimension on top.

pub mod fitness;
pub mod pressure;
pub mod target;

pub use fitness::{ArchFitness, FitnessScore};
pub use pressure::PressureCategory;
pub use target::{CompositionTier, DeploymentTier, Target};

use crate::primal_names::Primal;
use crate::validation::ValidationResult;

/// Evolution debt — a specific failure on a specific target for a specific primal.
///
/// Debt items are actionable: they identify what broke, where, and how severe
/// the failure is (blocks deployment vs. degraded operation).
#[derive(Debug, Clone)]
pub struct EvolutionDebt {
    /// The primal carrying this debt.
    pub primal: &'static str,
    /// The deployment target where the failure occurred.
    pub target: Target,
    /// The category of selection pressure that exposed the debt.
    pub pressure: PressureCategory,
    /// Human-readable description of the failure.
    pub failure: String,
    /// Whether this blocks deployment or merely degrades operation.
    pub severity: Severity,
}

/// How severe is the evolution debt?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Primal cannot run on this target at all (crash, EACCES, EROFS).
    BlocksDeployment,
    /// Primal runs but with reduced capability (no UDS, degraded latency).
    Degraded,
    /// Primal runs but hasn't been validated on this target.
    Unverified,
}

impl Severity {
    /// Short label for display.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::BlocksDeployment => "BLOCKS",
            Self::Degraded => "DEGRADED",
            Self::Unverified => "UNVERIFIED",
        }
    }
}

/// Evaluate evolution fitness for a primal based on validation results.
///
/// This is the main entry point: given a set of validation results from
/// running scenarios on a specific target, compute the fitness score and
/// identify any evolution debt.
#[must_use]
pub fn evaluate_fitness(
    primal: &'static str,
    target: Target,
    result: &ValidationResult,
) -> ArchFitness {
    let checks_total = result.evaluated();
    let checks_passed = checks_total.saturating_sub(result.failed);
    let checks_skipped = result.skipped;

    let survival_ratio = if checks_total > 0 {
        f64::from(checks_passed) / f64::from(checks_total)
    } else {
        0.0
    };

    let mut debt = Vec::new();

    if result.failed > 0 {
        let severity = if survival_ratio < 0.5 {
            Severity::BlocksDeployment
        } else {
            Severity::Degraded
        };

        debt.push(EvolutionDebt {
            primal,
            target,
            pressure: PressureCategory::Unknown,
            failure: format!(
                "{} of {} checks failed on {:?}",
                result.failed, checks_total, target
            ),
            severity,
        });
    }

    if checks_total == 0 && checks_skipped > 0 {
        debt.push(EvolutionDebt {
            primal,
            target,
            pressure: PressureCategory::Unknown,
            failure: format!("all {checks_skipped} checks skipped — target not validated"),
            severity: Severity::Unverified,
        });
    }

    ArchFitness {
        primal,
        target,
        checks_total,
        checks_passed,
        checks_skipped,
        survival_ratio,
        debt,
        score: FitnessScore::from_survival(survival_ratio, checks_total),
    }
}

/// Evaluate a full primal roster against a target.
#[must_use]
pub fn evaluate_roster(
    target: Target,
    results: &[(&'static str, &ValidationResult)],
) -> Vec<ArchFitness> {
    results
        .iter()
        .map(|(primal, result)| evaluate_fitness(primal, target, result))
        .collect()
}

/// The declared target support matrix for a primal.
///
/// Each primal declares which targets it supports (proven), which it aims
/// for (target), and which are endgame goals. This is the "genome" that
/// selection pressure tests against.
#[derive(Debug, Clone)]
pub struct TargetDeclaration {
    /// Primal identity.
    pub primal: Primal,
    /// Targets where the primal is proven to work (validated green).
    pub proven: Vec<Target>,
    /// Targets the primal is actively working toward.
    pub in_progress: Vec<Target>,
    /// Endgame targets (Wave 130+).
    pub endgame: Vec<Target>,
}

impl TargetDeclaration {
    /// Build the canonical target matrix for a primal based on known ecosystem state.
    #[must_use]
    pub fn for_primal(primal: Primal) -> Self {
        let x86 = Target::X86_64Musl;
        let arm = Target::Aarch64Musl;

        // All primals proven on x86_64 (fieldGate 13/13 ALIVE)
        let proven = vec![x86];

        // aarch64: beardog alive on grapheneGate, others have binaries
        // but are blocked by SELinux fixes or not yet live-validated
        let in_progress = vec![arm];

        Self {
            primal,
            proven,
            in_progress,
            endgame: vec![Target::Riscv64Musl, Target::Wasm32Wasi],
        }
    }

    /// Total targets this primal aims to support (proven + in_progress + endgame).
    #[must_use]
    #[expect(clippy::missing_const_for_fn, reason = "Vec::len() is not const-stable")]
    pub fn total_targets(&self) -> usize {
        self.proven.len() + self.in_progress.len() + self.endgame.len()
    }

    /// Coverage ratio: proven / total declared.
    #[must_use]
    #[expect(clippy::cast_precision_loss, reason = "target counts are tiny (<10)")]
    pub fn coverage(&self) -> f64 {
        let total = self.total_targets();
        if total == 0 {
            return 0.0;
        }
        self.proven.len() as f64 / total as f64
    }
}
