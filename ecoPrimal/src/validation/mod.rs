// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation harness for primalSpring experiments.
//!
//! Follows the ecosystem-wide `ValidationResult::check()` pattern used by
//! all springs. primalSpring validates coordination correctness rather than
//! numerical accuracy, so tolerances are typically boolean or latency-based.
//!
//! Adds `check_skip` for honest scaffolding: checks that need live primals
//! are recorded as skipped rather than faked as passed.
//!
//! # JSON Output
//!
//! Set `PRIMALSPRING_JSON=1` or pass `--json` for machine-readable output.
//! This is used by CI pipelines and primalSpring server aggregation.
//!
//! # Pluggable Output
//!
//! The [`ValidationSink`] trait allows experiments to redirect check output
//! (e.g. to a test harness capture buffer instead of stdout). Default is
//! [`StdoutSink`].

pub mod dependency;
pub mod helpers;
pub mod live_mesh;
pub mod numeric;
pub mod scenarios;
pub mod shadow;
pub mod sink;

use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use sink::default_sink;
pub use sink::{NdjsonSink, NullSink, StdoutSink, TracingSink, ValidationSink};

/// Outcome of a single validation check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckOutcome {
    /// Check passed.
    Pass,
    /// Check failed.
    Fail,
    /// Check skipped (needs live primals or other prerequisites).
    Skip,
}

/// Structured provenance metadata for validation traceability.
///
/// Absorbed from ludoSpring V14 `with_provenance()` and groundSpring's
/// provenance schema. Links every validation result to its source
/// experiment, baseline date, and optional description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Where the baseline came from (e.g. `"exp001_tower_atomic"`).
    pub source: String,
    /// When the baseline was established (e.g. `"2026-03-18"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_date: Option<String>,
    /// Free-form description of what this validation proves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Aggregated validation result for one experiment.
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Experiment name (e.g. `"primalSpring Exp001 — Tower Atomic"`).
    pub experiment: String,
    /// Number of checks that passed.
    pub passed: u32,
    /// Number of checks that failed.
    pub failed: u32,
    /// Number of checks that were skipped.
    pub skipped: u32,
    /// Individual check results in execution order.
    pub checks: Vec<CheckResult>,
    /// Structured provenance metadata for traceability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
    /// Output sink (defaults to stdout, not serialized).
    #[serde(skip, default = "default_sink")]
    sink: Arc<dyn ValidationSink>,
}

/// Result of a single named check within an experiment.
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckResult {
    /// Check identifier (e.g. `"health_beardog"`).
    pub name: String,
    /// Whether this check passed, failed, or was skipped.
    pub outcome: CheckOutcome,
    /// Human-readable detail (e.g. `"500µs (max: 50000µs)"`).
    pub detail: String,
}

impl CheckResult {
    /// Whether this check passed.
    #[must_use]
    pub const fn passed(&self) -> bool {
        matches!(self.outcome, CheckOutcome::Pass)
    }
}

impl ValidationResult {
    /// Create a new empty validation result for the given experiment name.
    ///
    /// Uses [`StdoutSink`] by default. Call [`with_sink`](Self::with_sink) to override.
    #[must_use]
    pub fn new(experiment: &str) -> Self {
        Self {
            experiment: experiment.to_owned(),
            passed: 0,
            failed: 0,
            skipped: 0,
            checks: Vec::new(),
            provenance: None,
            sink: Arc::new(StdoutSink),
        }
    }

    /// Replace the output sink (builder-style).
    #[must_use]
    pub fn with_sink(mut self, sink: Arc<dyn ValidationSink>) -> Self {
        self.sink = sink;
        self
    }

    /// Attach structured provenance metadata to this validation result.
    ///
    /// Returns `self` for builder-style chaining.
    #[must_use]
    pub fn with_provenance(mut self, source: &str, baseline_date: &str) -> Self {
        self.provenance = Some(Provenance {
            source: source.to_owned(),
            baseline_date: Some(baseline_date.to_owned()),
            description: None,
        });
        self
    }

    /// Attach full provenance metadata including a description.
    #[must_use]
    pub fn with_provenance_full(
        mut self,
        source: &str,
        baseline_date: &str,
        description: &str,
    ) -> Self {
        self.provenance = Some(Provenance {
            source: source.to_owned(),
            baseline_date: Some(baseline_date.to_owned()),
            description: Some(description.to_owned()),
        });
        self
    }

    /// Begin a named section of checks.
    ///
    /// Delegates to the sink's `section()` method for structured output.
    /// Absorbed from groundSpring V120 pattern.
    pub fn section(&self, name: &str) {
        self.sink.section(name);
    }

    /// Record a boolean pass/fail check.
    pub fn check_bool(&mut self, name: &str, condition: bool, detail: &str) {
        let outcome = if condition {
            self.passed += 1;
            CheckOutcome::Pass
        } else {
            self.failed += 1;
            CheckOutcome::Fail
        };
        self.sink.on_check(outcome, name, detail);
        self.checks.push(CheckResult {
            name: name.to_owned(),
            outcome,
            detail: detail.to_owned(),
        });
    }

    /// Record a check that cannot be evaluated yet (needs live primals).
    ///
    /// Skipped checks do not count as pass or fail. They are honest
    /// markers of incomplete validation — never fake a pass.
    pub fn check_skip(&mut self, name: &str, reason: &str) {
        self.skipped += 1;
        self.sink.on_check(CheckOutcome::Skip, name, reason);
        self.checks.push(CheckResult {
            name: name.to_owned(),
            outcome: CheckOutcome::Skip,
            detail: reason.to_owned(),
        });
    }

    /// Check that a latency measurement is within an acceptable bound.
    pub fn check_latency(&mut self, name: &str, actual_us: u64, max_us: u64) {
        let ok = actual_us <= max_us;
        let detail = format!("{actual_us}\u{03bc}s (max: {max_us}\u{03bc}s)");
        self.check_bool(name, ok, &detail);
    }

    /// Check that an exact count matches.
    pub fn check_count(&mut self, name: &str, actual: usize, expected: usize) {
        let ok = actual == expected;
        let detail = format!("got {actual}, expected {expected}");
        self.check_bool(name, ok, &detail);
    }

    /// Check that a count meets a minimum threshold.
    pub fn check_minimum(&mut self, name: &str, actual: usize, minimum: usize) {
        let ok = actual >= minimum;
        let detail = format!("got {actual}, minimum {minimum}");
        self.check_bool(name, ok, &detail);
    }

    /// Check that a floating-point value is within a relative tolerance.
    ///
    /// Absorbed from groundSpring V120 / wetSpring V133 / healthSpring V42.
    /// Passes when `|actual - expected| / |expected| <= rel_tol`, or when
    /// both values are zero.
    pub fn check_relative(&mut self, name: &str, actual: f64, expected: f64, rel_tol: f64) {
        let ok = if expected == 0.0 {
            actual.abs() <= rel_tol
        } else {
            ((actual - expected) / expected).abs() <= rel_tol
        };
        let detail = format!("got {actual}, expected {expected} (rel_tol {rel_tol})");
        self.check_bool(name, ok, &detail);
    }

    /// Check a floating-point value against either absolute OR relative tolerance.
    ///
    /// Absorbed from groundSpring V120 / healthSpring V42. Passes if the value
    /// is within `abs_tol` of expected, OR within `rel_tol` fraction of expected.
    /// This avoids false negatives near zero (where relative tolerance explodes)
    /// while still catching large relative errors.
    pub fn check_abs_or_rel(
        &mut self,
        name: &str,
        actual: f64,
        expected: f64,
        abs_tol: f64,
        rel_tol: f64,
    ) {
        let abs_ok = (actual - expected).abs() <= abs_tol;
        let rel_ok = if expected == 0.0 {
            actual.abs() <= rel_tol
        } else {
            ((actual - expected) / expected).abs() <= rel_tol
        };
        let ok = abs_ok || rel_ok;
        let detail =
            format!("got {actual}, expected {expected} (abs_tol {abs_tol}, rel_tol {rel_tol})");
        self.check_bool(name, ok, &detail);
    }

    /// Conditionally run a check or skip based on a prerequisite.
    ///
    /// If `prerequisite` is `Some`, runs the check with the value.
    /// If `None`, records a skip with the given reason.
    pub fn check_or_skip<T, F>(
        &mut self,
        name: &str,
        prerequisite: Option<T>,
        skip_reason: &str,
        check: F,
    ) where
        F: FnOnce(T, &mut Self),
    {
        match prerequisite {
            Some(val) => check(val, self),
            None => self.check_skip(name, skip_reason),
        }
    }

    /// Validate composition parity: call a primal via IPC, extract a scalar
    /// result, and compare against a local baseline within tolerance.
    ///
    /// If the IPC call fails (primal not running), records a `check_skip`
    /// rather than a failure — honest about what couldn't be tested.
    ///
    /// The `extractor` closure pulls a numeric value from the JSON-RPC
    /// result payload, keeping this method schema-agnostic. It receives
    /// the `result` field of the response (the `serde_json::Value`).
    #[expect(
        clippy::too_many_arguments,
        reason = "domain-driven API: each parameter is semantically distinct"
    )]
    pub fn check_composition_parity(
        &mut self,
        name: &str,
        client: &mut crate::ipc::client::PrimalClient,
        method: &str,
        params: serde_json::Value,
        extractor: impl FnOnce(&serde_json::Value) -> Option<f64>,
        expected: f64,
        tolerance: f64,
    ) {
        let response = match client.call(method, params) {
            Ok(r) => r,
            Err(e) => {
                self.check_skip(name, &format!("IPC call failed: {e}"));
                return;
            }
        };

        let Some(ref result_val) = response.result else {
            let detail = response.error.as_ref().map_or_else(
                || "no result in response".to_owned(),
                |e| format!("RPC error: {}", e.message),
            );
            self.check_skip(name, &detail);
            return;
        };

        match extractor(result_val) {
            Some(actual) => {
                let diff = (actual - expected).abs();
                let ok = diff <= tolerance;
                let detail = format!(
                    "composition={actual}, local={expected}, diff={diff:.2e}, tol={tolerance:.2e}"
                );
                self.check_bool(name, ok, &detail);
            }
            None => {
                self.check_bool(
                    name,
                    false,
                    &format!("extractor returned None from result: {result_val}"),
                );
            }
        }
    }

    /// Validate composition parity for a vector of values.
    ///
    /// Calls a primal via IPC, extracts a `Vec<f64>` result, and compares
    /// element-wise against a local baseline within tolerance. All elements
    /// must match for the check to pass.
    #[expect(
        clippy::too_many_arguments,
        reason = "domain-driven API: each parameter is semantically distinct"
    )]
    pub fn check_composition_parity_vec(
        &mut self,
        name: &str,
        client: &mut crate::ipc::client::PrimalClient,
        method: &str,
        params: serde_json::Value,
        extractor: impl FnOnce(&serde_json::Value) -> Option<Vec<f64>>,
        expected: &[f64],
        tolerance: f64,
    ) {
        let response = match client.call(method, params) {
            Ok(r) => r,
            Err(e) => {
                self.check_skip(name, &format!("IPC call failed: {e}"));
                return;
            }
        };

        let Some(ref result_val) = response.result else {
            let detail = response.error.as_ref().map_or_else(
                || "no result in response".to_owned(),
                |e| format!("RPC error: {}", e.message),
            );
            self.check_skip(name, &detail);
            return;
        };

        match extractor(result_val) {
            Some(actual) => {
                if actual.len() != expected.len() {
                    self.check_bool(
                        name,
                        false,
                        &format!(
                            "length mismatch: composition={}, local={}",
                            actual.len(),
                            expected.len()
                        ),
                    );
                    return;
                }
                let max_diff = actual
                    .iter()
                    .zip(expected.iter())
                    .map(|(a, e)| (a - e).abs())
                    .fold(0.0_f64, f64::max);
                let ok = max_diff <= tolerance;
                let detail = format!(
                    "len={}, max_diff={max_diff:.2e}, tol={tolerance:.2e}",
                    actual.len()
                );
                self.check_bool(name, ok, &detail);
            }
            None => {
                self.check_bool(
                    name,
                    false,
                    &format!("extractor returned None from result: {result_val}"),
                );
            }
        }
    }

    /// All non-skipped checks passed and at least one check was evaluated.
    #[must_use]
    pub const fn all_passed(&self) -> bool {
        self.failed == 0 && self.passed > 0
    }

    /// Total checks evaluated (pass + fail, excluding skips).
    #[must_use]
    pub const fn evaluated(&self) -> u32 {
        self.passed + self.failed
    }

    /// Print human-readable summary to stdout and emit via tracing, then
    /// delegate to the sink.
    pub fn summary(&self) {
        use crate::tolerances::VALIDATION_SUMMARY_WIDTH;
        let banner = "=".repeat(VALIDATION_SUMMARY_WIDTH);
        let result_str = if self.all_passed() {
            "ALL PASS".to_owned()
        } else {
            format!("{} FAILURES", self.failed)
        };
        let skip_str = if self.skipped > 0 {
            format!(" ({} skipped)", self.skipped)
        } else {
            String::new()
        };

        println!("\n{banner}");
        println!(
            "{}: {}/{} checks passed{skip_str}",
            self.experiment,
            self.passed,
            self.evaluated(),
        );
        println!("Result: {result_str}");
        println!("{banner}");

        tracing::info!(
            experiment = %self.experiment,
            passed = self.passed,
            failed = self.failed,
            skipped = self.skipped,
            result = %result_str,
            "validation complete"
        );

        self.sink
            .write_summary(self.passed, self.failed, self.skipped);
    }

    /// Serialize to JSON string for machine-readable output.
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if serialization fails.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Print summary, choosing JSON or human format based on `PRIMALSPRING_JSON` env.
    pub fn finish(&self) {
        if std::env::var(crate::env_keys::PRIMALSPRING_JSON).is_ok() {
            if let Ok(json) = self.to_json() {
                println!("{json}");
                tracing::debug!(format = "json", "validation result emitted");
            } else {
                self.summary();
            }
        } else {
            self.summary();
        }
    }

    /// Process exit code: 0 if all passed (at least one pass), 1 otherwise.
    ///
    /// # Design note
    ///
    /// This is the default used by [`run`](Self::run). An experiment where
    /// every check is skipped (no live primals) exits 1 because `passed == 0`.
    /// This is intentional: a skip-only run is not a confirmed pass. Use
    /// [`exit_code_skip_aware`](Self::exit_code_skip_aware) when CI needs to
    /// distinguish "nothing to test" (exit 2) from "tests failed" (exit 1).
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        if self.all_passed() { 0 } else { 1 }
    }

    /// Skip-aware exit code for CI: 0 = pass, 1 = fail, 2 = all skipped.
    ///
    /// Absorbed from wetSpring V129 `skip_with_code()` pattern. When all
    /// checks are skipped (no live primals available), returning 2 lets CI
    /// distinguish "nothing to test" from "tests failed" — skip ≠ fail.
    ///
    /// Use this in CI pipelines or scripts that need to handle the
    /// "no primals available" case differently from actual failures.
    #[must_use]
    pub const fn exit_code_skip_aware(&self) -> i32 {
        if self.all_passed() {
            0
        } else if self.failed > 0 {
            1
        } else {
            2
        }
    }

    /// Print a standard experiment banner.
    ///
    /// Shared helper that replaces `println!("=".repeat(72))` boilerplate
    /// across all experiments.
    pub fn print_banner(title: &str) {
        use crate::tolerances::VALIDATION_SUMMARY_WIDTH;
        let banner = "=".repeat(VALIDATION_SUMMARY_WIDTH);
        println!("{banner}");
        println!("{title}");
        println!("{banner}");
        tracing::info!(experiment = title, "validation started");
    }

    /// Run an experiment body, print summary, and exit.
    ///
    /// Encapsulates the banner + body + finish + exit pattern shared
    /// by all primalSpring experiments.
    pub fn run_experiment(title: &str, subtitle: &str, body: impl FnOnce(&mut Self)) -> ! {
        Self::new(title).run(subtitle, body);
    }

    /// Builder-pattern entry point: run the experiment body, print summary, and exit.
    ///
    /// Consumes `self` so that builder methods like [`with_provenance`](Self::with_provenance)
    /// and [`with_sink`](Self::with_sink) can be chained before execution:
    ///
    /// ```rust,no_run
    /// # use primalspring::validation::ValidationResult;
    /// ValidationResult::new("Exp001 — Tower Atomic")
    ///     .with_provenance("exp001_tower_atomic", "2026-03-24")
    ///     .run("Tower Atomic (security + discovery)", |v| {
    ///         v.check_bool("alive", true, "yes");
    ///     });
    /// ```
    pub fn run(mut self, subtitle: &str, body: impl FnOnce(&mut Self)) -> ! {
        Self::print_banner(subtitle);
        body(&mut self);
        self.finish();
        std::process::exit(self.exit_code());
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}/{} passed",
            self.experiment,
            self.passed,
            self.evaluated()
        )?;
        if self.skipped > 0 {
            write!(f, " ({} skipped)", self.skipped)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
