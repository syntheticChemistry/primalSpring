// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Fitness scoring — quantifies how well a primal survives selection pressure.

use super::target::Target;
use super::{EvolutionDebt, Severity};

/// Per-primal fitness result on a specific target.
///
/// Contains both the scalar score and the detailed debt inventory.
/// Used to build the cross-architecture survival matrix.
#[derive(Debug, Clone)]
pub struct ArchFitness {
    /// Which primal was evaluated.
    pub primal: &'static str,
    /// Which target the evaluation ran on.
    pub target: Target,
    /// Total validation checks executed.
    pub checks_total: u32,
    /// Checks that passed.
    pub checks_passed: u32,
    /// Checks that were skipped (target not available).
    pub checks_skipped: u32,
    /// Ratio of passed / total (0.0 = dead, 1.0 = fully adapted).
    pub survival_ratio: f64,
    /// Identified evolution debt items.
    pub debt: Vec<EvolutionDebt>,
    /// Aggregate fitness score.
    pub score: FitnessScore,
}

impl ArchFitness {
    /// Whether this primal is fully adapted to the target (no debt).
    #[must_use]
    pub fn is_fully_adapted(&self) -> bool {
        self.debt.is_empty() && self.survival_ratio >= 1.0
    }

    /// Whether this primal is blocked from deploying on this target.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        self.debt
            .iter()
            .any(|d| d.severity == Severity::BlocksDeployment)
    }

    /// Whether this primal has any unverified targets.
    #[must_use]
    pub fn has_unverified(&self) -> bool {
        self.debt.iter().any(|d| d.severity == Severity::Unverified)
    }

    /// Count of blocking debt items.
    #[must_use]
    pub fn blocking_count(&self) -> usize {
        self.debt
            .iter()
            .filter(|d| d.severity == Severity::BlocksDeployment)
            .count()
    }
}

/// Scalar fitness score with breakdown.
///
/// The score represents how "fit" a primal is for a target:
/// - 1.0 = fully adapted (all checks pass, no debt)
/// - 0.0 = completely incompatible (all checks fail)
///
/// The score is NOT a simple pass ratio — it's weighted by the number
/// of checks (more checks = higher confidence), with a penalty for
/// skipped checks (unvalidated = lower confidence).
#[derive(Debug, Clone)]
pub struct FitnessScore {
    /// Primary scalar: 0.0 (dead) to 1.0 (fully adapted).
    pub value: f64,
    /// Confidence: how many checks contributed to this score.
    /// Low check count = low confidence even if score is 1.0.
    pub confidence: Confidence,
    /// Per-dimension breakdown (for future multi-axis evolution).
    pub dimensions: Vec<(String, f64)>,
}

/// Confidence level in a fitness score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    /// Fewer than 5 checks — score is unreliable.
    Low,
    /// 5-20 checks — score is indicative.
    Medium,
    /// More than 20 checks — score is reliable.
    High,
}

impl FitnessScore {
    /// Compute a fitness score from survival ratio and check count.
    #[must_use]
    #[expect(
        clippy::missing_const_for_fn,
        reason = "Vec::new() is not const-stable"
    )]
    pub fn from_survival(survival_ratio: f64, checks_total: u32) -> Self {
        let confidence = Self::confidence_for(checks_total);
        Self {
            value: survival_ratio,
            confidence,
            dimensions: Vec::new(),
        }
    }

    /// Determine confidence level from check count.
    #[must_use]
    pub const fn confidence_for(checks_total: u32) -> Confidence {
        match checks_total {
            0..=4 => Confidence::Low,
            5..=20 => Confidence::Medium,
            _ => Confidence::High,
        }
    }

    /// A score of zero (not validated at all).
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            value: 0.0,
            confidence: Confidence::Low,
            dimensions: Vec::new(),
        }
    }

    /// Whether this score indicates full adaptation.
    #[must_use]
    pub fn is_perfect(&self) -> bool {
        (self.value - 1.0).abs() < f64::EPSILON && self.confidence == Confidence::High
    }
}
