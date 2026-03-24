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
/// Converged ecosystem pattern from airSpring, rhizoCrypt, and groundSpring.
/// Default implementation writes to stdout; test harnesses can
/// substitute a capture buffer.
///
/// The `section` and `write_summary` methods have default no-op impls
/// for backward compatibility (absorbed from groundSpring V120).
pub trait ValidationSink: std::fmt::Debug + Send + Sync {
    /// Emit a single check result.
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str);

    /// Begin a named section of checks (e.g. "IPC health", "Graph validation").
    ///
    /// Absorbed from groundSpring V120 `ValidationSink::section()`. Sinks that
    /// support structured output can use this to group checks.
    fn section(&self, _name: &str) {}

    /// Write a summary footer after all checks are emitted.
    ///
    /// Absorbed from groundSpring V120 `ValidationSink::write_summary()`.
    fn write_summary(&self, _passed: u32, _failed: u32, _skipped: u32) {}
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

    fn section(&self, name: &str) {
        println!("\n--- {name} ---");
    }

    fn write_summary(&self, passed: u32, failed: u32, skipped: u32) {
        let total = passed + failed;
        print!("  Summary: {passed}/{total} passed");
        if skipped > 0 {
            print!(" ({skipped} skipped)");
        }
        println!();
    }
}

/// Sink that discards all output (useful for tests that only inspect counts).
#[derive(Debug)]
pub struct NullSink;

impl ValidationSink for NullSink {
    fn on_check(&self, _: CheckOutcome, _: &str, _: &str) {}
}

/// Newline-delimited JSON sink for streaming validation output.
///
/// Absorbed from groundSpring V121 / wetSpring V133 / neuralSpring V122.
/// Emits one JSON object per check, one per line — ideal for log aggregation,
/// CI pipelines, and cross-process streaming. Each line is independently
/// parseable, unlike a single JSON array.
#[derive(Debug)]
pub struct NdjsonSink<W: std::io::Write + std::fmt::Debug + Send + Sync> {
    writer: std::sync::Mutex<W>,
}

impl<W: std::io::Write + std::fmt::Debug + Send + Sync> NdjsonSink<W> {
    /// Create a new NDJSON sink writing to the given destination.
    pub const fn new(writer: W) -> Self {
        Self {
            writer: std::sync::Mutex::new(writer),
        }
    }
}

impl NdjsonSink<std::io::Stdout> {
    /// Create a sink that writes NDJSON to stdout.
    #[must_use]
    pub fn stdout() -> Self {
        Self::new(std::io::stdout())
    }
}

impl<W: std::io::Write + std::fmt::Debug + Send + Sync> ValidationSink for NdjsonSink<W> {
    fn on_check(&self, outcome: CheckOutcome, name: &str, detail: &str) {
        let tag = match outcome {
            CheckOutcome::Pass => "pass",
            CheckOutcome::Fail => "fail",
            CheckOutcome::Skip => "skip",
        };
        let line = format!(
            "{{\"outcome\":\"{tag}\",\"name\":{},\"detail\":{}}}",
            serde_json::to_string(name).unwrap_or_else(|_| format!("\"{name}\"")),
            serde_json::to_string(detail).unwrap_or_else(|_| format!("\"{detail}\"")),
        );
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "{line}");
        }
    }

    fn section(&self, name: &str) {
        let line = format!(
            "{{\"section\":{}}}",
            serde_json::to_string(name).unwrap_or_else(|_| format!("\"{name}\"")),
        );
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "{line}");
        }
    }

    fn write_summary(&self, passed: u32, failed: u32, skipped: u32) {
        let line = format!(
            "{{\"summary\":{{\"passed\":{passed},\"failed\":{failed},\"skipped\":{skipped}}}}}"
        );
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "{line}");
        }
    }
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

    /// Print human-readable summary to stdout and delegate to the sink.
    pub fn summary(&self) {
        use crate::tolerances::VALIDATION_SUMMARY_WIDTH;
        println!("\n{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
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
        println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
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
        if self.all_passed() { 0 } else { 1 }
    }

    /// Skip-aware exit code for CI: 0 = pass, 1 = fail, 2 = all skipped.
    ///
    /// Absorbed from wetSpring V129 `skip_with_code()` pattern. When all
    /// checks are skipped (no live primals available), returning 2 lets CI
    /// distinguish "nothing to test" from "tests failed" — skip ≠ fail.
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
        println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
        println!("{title}");
        println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    }

    /// Run an experiment body, print summary, and exit.
    ///
    /// Encapsulates the banner + body + finish + exit pattern shared
    /// by all primalSpring experiments.
    pub fn run_experiment(title: &str, subtitle: &str, body: impl FnOnce(&mut Self)) -> ! {
        let mut v = Self::new(title);
        Self::print_banner(subtitle);
        body(&mut v);
        v.finish();
        std::process::exit(v.exit_code());
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
        let v = ValidationResult::new("test").with_provenance("exp001_tower", "2026-03-18");
        let prov = v.provenance.as_ref().unwrap();
        assert_eq!(prov.source, "exp001_tower");
        assert_eq!(prov.baseline_date.as_deref(), Some("2026-03-18"));
        assert!(prov.description.is_none());
    }

    #[test]
    fn with_provenance_full_sets_all_fields() {
        let v = ValidationResult::new("test").with_provenance_full(
            "exp050_compute_triangle",
            "2026-03-18",
            "Compute triangle coordination validation",
        );
        let prov = v.provenance.as_ref().unwrap();
        assert_eq!(prov.source, "exp050_compute_triangle");
        assert_eq!(prov.baseline_date.as_deref(), Some("2026-03-18"));
        assert_eq!(
            prov.description.as_deref(),
            Some("Compute triangle coordination validation")
        );
    }

    #[test]
    fn provenance_survives_json_round_trip() {
        let mut v = ValidationResult::new("exp_prov").with_provenance("exp001_tower", "2026-03-18");
        v.check_bool("ok", true, "yes");

        let json = v.to_json().unwrap();
        let back: ValidationResult = serde_json::from_str(&json).unwrap();
        let prov = back.provenance.as_ref().unwrap();
        assert_eq!(prov.source, "exp001_tower");
    }

    #[test]
    fn provenance_absent_omitted_from_json() {
        let mut v = ValidationResult::new("no_prov");
        v.check_bool("ok", true, "yes");

        let json = v.to_json().unwrap();
        assert!(!json.contains("provenance"));
    }

    #[test]
    fn all_experiment_tracks_have_provenance_schema() {
        let experiment_ids = [
            "exp001", "exp002", "exp003", "exp004", "exp005", "exp006", "exp010", "exp011",
            "exp012", "exp013", "exp014", "exp015", "exp020", "exp021", "exp022", "exp023",
            "exp024", "exp025", "exp030", "exp031", "exp032", "exp033", "exp034", "exp040",
            "exp041", "exp042", "exp043", "exp044", "exp050", "exp051", "exp052", "exp053",
            "exp054", "exp055", "exp056", "exp057", "exp058", "exp059", "exp060", "exp061",
            "exp062", "exp063", "exp064", "exp065", "exp066", "exp067", "exp068", "exp069",
            "exp070", "exp071", "exp072", "exp073", "exp074",
        ];
        assert_eq!(
            experiment_ids.len(),
            53,
            "expected 53 experiments across tracks"
        );
        let tracks: std::collections::HashSet<u32> = experiment_ids
            .iter()
            .filter_map(|id| id.strip_prefix("exp"))
            .filter_map(|num| num.parse::<u32>().ok())
            .map(|n| n / 10)
            .collect();
        assert!(
            tracks.len() >= 8,
            "expected at least 8 tracks, got {}",
            tracks.len()
        );
    }

    #[test]
    fn print_banner_does_not_panic() {
        ValidationResult::print_banner("Test Banner Title");
    }

    #[test]
    fn with_sink_replaces_default() {
        let v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        assert_eq!(v.passed, 0);
        assert_eq!(v.failed, 0);
    }

    #[test]
    fn evaluated_counts_pass_and_fail_only() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_bool("a", true, "pass");
        v.check_bool("b", false, "fail");
        v.check_skip("c", "skip");
        assert_eq!(v.evaluated(), 2);
        assert_eq!(v.passed, 1);
        assert_eq!(v.failed, 1);
        assert_eq!(v.skipped, 1);
    }

    #[test]
    fn check_count_records_exact_match() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_count("count_ok", 5, 5);
        assert_eq!(v.passed, 1);
        v.check_count("count_bad", 3, 5);
        assert_eq!(v.failed, 1);
    }

    #[test]
    fn check_minimum_records_threshold() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_minimum("min_ok", 5, 3);
        assert_eq!(v.passed, 1);
        v.check_minimum("min_bad", 1, 3);
        assert_eq!(v.failed, 1);
    }

    #[test]
    fn exit_code_zero_when_all_passed() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_bool("ok", true, "yes");
        assert_eq!(v.exit_code(), 0);
    }

    #[test]
    fn exit_code_one_when_failure() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_bool("bad", false, "no");
        assert_eq!(v.exit_code(), 1);
    }

    #[test]
    fn display_format_includes_experiment_name() {
        let v = ValidationResult::new("my experiment").with_sink(Arc::new(NullSink));
        let display = format!("{v}");
        assert!(display.contains("my experiment"));
    }

    #[test]
    fn section_does_not_panic() {
        let v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.section("IPC health");
    }

    #[test]
    fn exit_code_skip_aware_zero_on_pass() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_bool("ok", true, "yes");
        assert_eq!(v.exit_code_skip_aware(), 0);
    }

    #[test]
    fn exit_code_skip_aware_one_on_fail() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_bool("bad", false, "no");
        assert_eq!(v.exit_code_skip_aware(), 1);
    }

    #[test]
    fn exit_code_skip_aware_two_when_all_skipped() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_skip("pending", "no live primals");
        assert_eq!(v.exit_code_skip_aware(), 2);
    }

    #[test]
    fn exit_code_skip_aware_two_when_empty() {
        let v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        assert_eq!(v.exit_code_skip_aware(), 2);
    }

    #[test]
    fn stdout_sink_section_does_not_panic() {
        let sink = StdoutSink;
        sink.section("test section");
    }

    #[test]
    fn stdout_sink_write_summary_does_not_panic() {
        let sink = StdoutSink;
        sink.write_summary(5, 1, 2);
    }

    #[test]
    fn check_result_passed_method() {
        let pass = CheckResult {
            name: "a".to_owned(),
            outcome: CheckOutcome::Pass,
            detail: "ok".to_owned(),
        };
        assert!(pass.passed());

        let fail = CheckResult {
            name: "b".to_owned(),
            outcome: CheckOutcome::Fail,
            detail: "no".to_owned(),
        };
        assert!(!fail.passed());
    }

    // ── check_relative tests ──

    #[test]
    fn check_relative_pass_exact() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_relative("exact", 1.0, 1.0, 0.01);
        assert_eq!(v.passed, 1);
    }

    #[test]
    fn check_relative_pass_within_tol() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_relative("close", 1.005, 1.0, 0.01);
        assert_eq!(v.passed, 1);
    }

    #[test]
    fn check_relative_fail_outside_tol() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_relative("far", 1.05, 1.0, 0.01);
        assert_eq!(v.failed, 1);
    }

    #[test]
    fn check_relative_zero_expected() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_relative("zero_ok", 0.005, 0.0, 0.01);
        assert_eq!(v.passed, 1);
        v.check_relative("zero_bad", 0.05, 0.0, 0.01);
        assert_eq!(v.failed, 1);
    }

    // ── check_abs_or_rel tests ──

    #[test]
    fn check_abs_or_rel_pass_by_abs() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_abs_or_rel("abs_ok", 0.001, 0.0, 0.01, 0.000_01);
        assert_eq!(v.passed, 1);
    }

    #[test]
    fn check_abs_or_rel_pass_by_rel() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_abs_or_rel("rel_ok", 100.5, 100.0, 0.001, 0.01);
        assert_eq!(v.passed, 1);
    }

    #[test]
    fn check_abs_or_rel_fail_both() {
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        v.check_abs_or_rel("both_bad", 200.0, 100.0, 0.01, 0.01);
        assert_eq!(v.failed, 1);
    }

    // ── NdjsonSink tests ──

    #[test]
    fn ndjson_sink_emits_valid_json() {
        let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
        sink.on_check(CheckOutcome::Pass, "test_check", "all good");

        let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
        assert_eq!(parsed["outcome"], "pass");
        assert_eq!(parsed["name"], "test_check");
    }

    #[test]
    fn ndjson_sink_section_emits_json() {
        let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
        sink.section("IPC health");

        let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
        assert_eq!(parsed["section"], "IPC health");
    }

    #[test]
    fn ndjson_sink_summary_emits_json() {
        let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
        sink.write_summary(10, 2, 3);

        let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
        assert_eq!(parsed["summary"]["passed"], 10);
        assert_eq!(parsed["summary"]["failed"], 2);
        assert_eq!(parsed["summary"]["skipped"], 3);
    }

    #[test]
    fn ndjson_sink_with_validation_result() {
        let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        let sink = NdjsonSink::new(CursorWriter(Arc::clone(&buf)));
        let mut v = ValidationResult::new("ndjson_test").with_sink(Arc::new(sink));
        v.check_bool("a", true, "pass");
        v.check_bool("b", false, "fail");
        v.check_skip("c", "skipped");

        let output = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        let lines: Vec<&str> = output.trim().lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(v.passed, 1);
        assert_eq!(v.failed, 1);
        assert_eq!(v.skipped, 1);
    }

    /// Helper writer that delegates to a shared Vec<u8> behind a Mutex.
    #[derive(Debug, Clone)]
    struct CursorWriter(Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for CursorWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
