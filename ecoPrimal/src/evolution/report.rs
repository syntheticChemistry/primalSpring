// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Evolution reports — serializable fitness summaries for cross-gate comparison.
//!
//! When `primalSpring` runs on different gates (eastGate x86, grapheneGate arm),
//! each produces a `TargetReport`. Comparing reports across gates proves
//! silicon-atheism: same scenarios, same results, regardless of hardware.

use serde::{Deserialize, Serialize};

use super::fitness::Confidence;
use super::pressure::PressureCategory;
use super::target::Target;
use super::Severity;

/// A serializable fitness report for a single target evaluation.
///
/// Produced by running the evolution scenarios on a specific gate.
/// Can be stored as JSON and compared across gates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetReport {
    /// Gate identity (hostname or gate name).
    pub gate: String,
    /// Target triple.
    pub target_triple: String,
    /// When this report was generated (ISO 8601).
    pub timestamp: String,
    /// Total scenarios evaluated.
    pub scenarios_run: u32,
    /// Per-primal fitness entries.
    pub primals: Vec<PrimalFitnessEntry>,
    /// Aggregate ecosystem score.
    pub ecosystem_score: f64,
    /// Active pressures on this target.
    pub active_pressures: Vec<String>,
    /// Summary of evolution debt.
    pub debt_summary: DebtSummary,
}

/// Per-primal fitness entry in a report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalFitnessEntry {
    /// Primal slug.
    pub slug: String,
    /// Fitness score (0.0-1.0).
    pub score: f64,
    /// Confidence level.
    pub confidence: String,
    /// Number of checks run.
    pub checks_total: u32,
    /// Number passed.
    pub checks_passed: u32,
    /// Whether this primal is blocked on this target.
    pub blocked: bool,
    /// Debt items for this primal.
    pub debt_count: usize,
}

/// Aggregate debt summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtSummary {
    /// Total blocking items across all primals.
    pub blocking: usize,
    /// Total degraded items.
    pub degraded: usize,
    /// Total unverified items.
    pub unverified: usize,
}

impl TargetReport {
    /// Build a report from a set of fitness evaluations.
    #[must_use]
    #[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation, reason = "primal/scenario counts are tiny")]
    pub fn build(gate: &str, fitness_results: &[super::ArchFitness]) -> Self {
        let target = fitness_results
            .first()
            .map_or_else(Target::current, |first| first.target);

        let primals: Vec<PrimalFitnessEntry> = fitness_results
            .iter()
            .map(|f| PrimalFitnessEntry {
                slug: f.primal.to_owned(),
                score: f.score.value,
                confidence: confidence_label(f.score.confidence),
                checks_total: f.checks_total,
                checks_passed: f.checks_passed,
                blocked: f.is_blocked(),
                debt_count: f.debt.len(),
            })
            .collect();

        let ecosystem_score = if primals.is_empty() {
            0.0
        } else {
            primals.iter().map(|p| p.score).sum::<f64>() / primals.len() as f64
        };

        let active_pressures = PressureCategory::active_for(target)
            .into_iter()
            .map(|p| p.label().to_owned())
            .collect();

        let mut blocking = 0;
        let mut degraded = 0;
        let mut unverified = 0;
        for f in fitness_results {
            for d in &f.debt {
                match d.severity {
                    Severity::BlocksDeployment => blocking += 1,
                    Severity::Degraded => degraded += 1,
                    Severity::Unverified => unverified += 1,
                }
            }
        }

        Self {
            gate: gate.to_owned(),
            target_triple: target.triple().to_owned(),
            timestamp: now_iso8601(),
            scenarios_run: fitness_results.len() as u32,
            primals,
            ecosystem_score,
            active_pressures,
            debt_summary: DebtSummary {
                blocking,
                degraded,
                unverified,
            },
        }
    }

    /// Serialize to pretty JSON.
    #[must_use]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_owned())
    }

    /// Compare this report against another gate's report.
    /// Returns divergences where the same primal has different fitness status.
    #[must_use]
    pub fn divergences_from(&self, other: &Self) -> Vec<Divergence> {
        let mut divs = Vec::new();

        for mine in &self.primals {
            if let Some(theirs) = other.primals.iter().find(|p| p.slug == mine.slug) {
                let score_diff = (mine.score - theirs.score).abs();
                if score_diff > 0.01 || mine.blocked != theirs.blocked {
                    divs.push(Divergence {
                        primal: mine.slug.clone(),
                        gate_a: self.gate.clone(),
                        gate_b: other.gate.clone(),
                        score_a: mine.score,
                        score_b: theirs.score,
                        blocked_a: mine.blocked,
                        blocked_b: theirs.blocked,
                    });
                }
            }
        }

        divs
    }
}

/// A divergence between two gates for the same primal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divergence {
    /// Primal that diverged.
    pub primal: String,
    /// First gate.
    pub gate_a: String,
    /// Second gate.
    pub gate_b: String,
    /// Score on gate A.
    pub score_a: f64,
    /// Score on gate B.
    pub score_b: f64,
    /// Blocked on gate A?
    pub blocked_a: bool,
    /// Blocked on gate B?
    pub blocked_b: bool,
}

impl Divergence {
    /// Human-readable summary.
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "{}: {} ({:.0}%{}) vs {} ({:.0}%{})",
            self.primal,
            self.gate_a,
            self.score_a * 100.0,
            if self.blocked_a { " BLOCKED" } else { "" },
            self.gate_b,
            self.score_b * 100.0,
            if self.blocked_b { " BLOCKED" } else { "" },
        )
    }
}

fn confidence_label(c: Confidence) -> String {
    match c {
        Confidence::Low => "low".to_owned(),
        Confidence::Medium => "medium".to_owned(),
        Confidence::High => "high".to_owned(),
    }
}

fn now_iso8601() -> String {
    let since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = since_epoch.as_secs();
    let days = secs / 86400;
    let years = 1970 + days / 365;
    format!("{years}-xx-xxT00:00:00Z (epoch: {secs})")
}

/// Compare two reports and produce a parity verdict.
#[must_use]
pub fn cross_gate_parity(report_a: &TargetReport, report_b: &TargetReport) -> ParityVerdict {
    let divergences = report_a.divergences_from(report_b);
    let score_diff = (report_a.ecosystem_score - report_b.ecosystem_score).abs();

    if divergences.is_empty() && score_diff < 0.01 {
        ParityVerdict::Equivalent
    } else if divergences.iter().any(|d| d.blocked_a != d.blocked_b) {
        ParityVerdict::TargetDependent(divergences)
    } else {
        ParityVerdict::MinorDrift(divergences)
    }
}

/// Result of comparing two gates.
#[derive(Debug, Clone)]
pub enum ParityVerdict {
    /// Same fitness across gates — silicon-atheism proven for these primals.
    Equivalent,
    /// Minor score differences (within tolerance, no blocking divergence).
    MinorDrift(Vec<Divergence>),
    /// Significant divergence: primals are blocked on one target but not another.
    TargetDependent(Vec<Divergence>),
}

impl ParityVerdict {
    /// Whether this is a passing verdict (equivalent or minor drift).
    #[must_use]
    pub const fn is_pass(&self) -> bool {
        matches!(self, Self::Equivalent | Self::MinorDrift(_))
    }
}
