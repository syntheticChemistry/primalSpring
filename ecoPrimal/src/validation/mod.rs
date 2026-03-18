// SPDX-License-Identifier: AGPL-3.0-or-later

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

pub mod or_exit;

pub use or_exit::OrExit;

use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Pluggable output destination for validation check results.
///
/// Converged ecosystem pattern from airSpring and rhizoCrypt.
/// Default implementation writes to stdout; test harnesses can
/// substitute a capture buffer.
pub trait ValidationSink: std::fmt::Debug + Send + Sync {
    /// Emit a single check result.
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str);
}

/// Default sink that writes check results to stdout.
#[derive(Debug)]
pub struct StdoutSink;

impl ValidationSink for StdoutSink {
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str) {
        let tag = match outcome {
            CheckOutcome::Pass => "PASS",
            CheckOutcome::Fail => "FAIL",
            CheckOutcome::Skip => "SKIP",
        };
        println!("  [{tag}] {name}: {detail}");
    }
}

/// Sink that discards all output (useful for tests that only inspect counts).
#[derive(Debug)]
pub struct NullSink;

impl ValidationSink for NullSink {
    fn on_check(&self, _: CheckOutcome, _: &str, _: &str) {}
}

fn default_sink() -> Arc<dyn ValidationSink> {
    Arc::new(StdoutSink)
}

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
    /// Optional provenance chain for the Provenance Trio integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
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

    /// Attach provenance metadata to this validation result.
    ///
    /// Returns `self` for builder-style chaining.
    #[must_use]
    pub fn with_provenance(mut self, provenance: &str) -> Self {
        self.provenance = Some(provenance.to_owned());
        self
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

    /// Print human-readable summary to stdout.
    pub fn summary(&self) {
        println!("\n{}", "=".repeat(72));
        println!(
            "{}: {}/{} checks passed{}",
            self.experiment,
            self.passed,
            self.evaluated(),
            if self.skipped > 0 {
                format!(" ({} skipped)", self.skipped)
            } else {
                String::new()
            }
        );
        if self.all_passed() {
            println!("Result: ALL PASS");
        } else {
            println!("Result: {} FAILURES", self.failed);
        }
        println!("{}", "=".repeat(72));
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
        if std::env::var("PRIMALSPRING_JSON").is_ok() {
            if let Ok(json) = self.to_json() {
                println!("{json}");
            } else {
                self.summary();
            }
        } else {
            self.summary();
        }
    }

    /// Process exit code: 0 if all passed, 1 otherwise.
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        if self.all_passed() {
            0
        } else {
            1
        }
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
mod tests {
    use super::*;

    #[test]
    fn new_result_has_zero_counts() {
        let v = ValidationResult::new("test");
        assert_eq!(v.passed, 0);
        assert_eq!(v.failed, 0);
        assert_eq!(v.skipped, 0);
        assert!(v.checks.is_empty());
    }

    #[test]
    fn check_bool_pass_increments_passed() {
        let mut v = ValidationResult::new("test");
        v.check_bool("ok", true, "detail");
        assert_eq!(v.passed, 1);
        assert_eq!(v.failed, 0);
        assert!(v.all_passed());
    }

    #[test]
    fn check_bool_fail_increments_failed() {
        let mut v = ValidationResult::new("test");
        v.check_bool("bad", false, "detail");
        assert_eq!(v.passed, 0);
        assert_eq!(v.failed, 1);
        assert!(!v.all_passed());
    }

    #[test]
    fn check_skip_increments_skipped() {
        let mut v = ValidationResult::new("test");
        v.check_skip("pending", "needs live primals");
        assert_eq!(v.skipped, 1);
        assert_eq!(v.passed, 0);
        assert_eq!(v.failed, 0);
        assert!(!v.all_passed());
    }

    #[test]
    fn all_passed_requires_at_least_one_pass() {
        let v = ValidationResult::new("test");
        assert!(!v.all_passed());
    }

    #[test]
    fn check_latency_pass() {
        let mut v = ValidationResult::new("test");
        v.check_latency("fast", 100, 50_000);
        assert!(v.all_passed());
    }

    #[test]
    fn check_latency_fail() {
        let mut v = ValidationResult::new("test");
        v.check_latency("slow", 100_000, 50_000);
        assert!(!v.all_passed());
    }

    #[test]
    fn check_count_pass() {
        let mut v = ValidationResult::new("test");
        v.check_count("exact", 5, 5);
        assert!(v.all_passed());
    }

    #[test]
    fn check_count_fail() {
        let mut v = ValidationResult::new("test");
        v.check_count("wrong", 3, 5);
        assert!(!v.all_passed());
    }

    #[test]
    fn check_minimum_pass() {
        let mut v = ValidationResult::new("test");
        v.check_minimum("enough", 10, 5);
        assert!(v.all_passed());
    }

    #[test]
    fn check_minimum_fail() {
        let mut v = ValidationResult::new("test");
        v.check_minimum("low", 2, 5);
        assert!(!v.all_passed());
    }

    #[test]
    fn evaluated_excludes_skips() {
        let mut v = ValidationResult::new("test");
        v.check_bool("ok", true, "yes");
        v.check_skip("pending", "later");
        v.check_bool("bad", false, "no");
        assert_eq!(v.evaluated(), 2);
        assert_eq!(v.skipped, 1);
    }

    #[test]
    fn display_format() {
        let mut v = ValidationResult::new("exp001");
        v.check_bool("ok", true, "yes");
        let display = format!("{v}");
        assert!(display.contains("exp001"));
        assert!(display.contains("1/1 passed"));
    }

    #[test]
    fn display_format_with_skips() {
        let mut v = ValidationResult::new("exp001");
        v.check_bool("ok", true, "yes");
        v.check_skip("pending", "later");
        let display = format!("{v}");
        assert!(display.contains("1 skipped"));
    }

    #[test]
    fn to_json_round_trip() {
        let mut v = ValidationResult::new("exp_json");
        v.check_bool("ok", true, "detail");
        v.check_skip("pending", "later");

        let json = v.to_json().unwrap();
        let back: ValidationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.experiment, "exp_json");
        assert_eq!(back.passed, 1);
        assert_eq!(back.skipped, 1);
        assert_eq!(back.checks.len(), 2);
    }

    #[test]
    fn exit_code_zero_on_pass() {
        let mut v = ValidationResult::new("test");
        v.check_bool("ok", true, "yes");
        assert_eq!(v.exit_code(), 0);
    }

    #[test]
    fn exit_code_one_on_fail() {
        let mut v = ValidationResult::new("test");
        v.check_bool("bad", false, "no");
        assert_eq!(v.exit_code(), 1);
    }

    #[test]
    fn check_or_skip_runs_check_when_some() {
        let mut v = ValidationResult::new("test");
        v.check_or_skip("found", Some(42u64), "not available", |val, v| {
            v.check_bool("value", val == 42, "expected 42");
        });
        assert_eq!(v.passed, 1);
        assert_eq!(v.skipped, 0);
    }

    #[test]
    fn check_or_skip_skips_when_none() {
        let mut v = ValidationResult::new("test");
        v.check_or_skip::<u64, _>("missing", None, "not available", |_, _| {
            panic!("should not be called");
        });
        assert_eq!(v.passed, 0);
        assert_eq!(v.skipped, 1);
    }

    #[test]
    fn provenance_none_by_default() {
        let v = ValidationResult::new("test");
        assert!(v.provenance.is_none());
    }

    #[test]
    fn with_provenance_sets_field() {
        let v = ValidationResult::new("test").with_provenance("rhizocrypt:sha256:abc123");
        assert_eq!(v.provenance.as_deref(), Some("rhizocrypt:sha256:abc123"));
    }

    #[test]
    fn provenance_survives_json_round_trip() {
        let mut v = ValidationResult::new("exp_prov").with_provenance("chain:abc");
        v.check_bool("ok", true, "yes");

        let json = v.to_json().unwrap();
        let back: ValidationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.provenance.as_deref(), Some("chain:abc"));
    }

    #[test]
    fn provenance_absent_omitted_from_json() {
        let mut v = ValidationResult::new("no_prov");
        v.check_bool("ok", true, "yes");

        let json = v.to_json().unwrap();
        assert!(!json.contains("provenance"));
    }
}
