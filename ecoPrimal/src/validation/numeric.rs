// SPDX-License-Identifier: AGPL-3.0-or-later

//! Numeric validation bridge — adapter for science springs migrating
//! from prokaryotic validation binaries to eukaryotic scenario registries.
//!
//! Science springs (wetSpring, hotSpring, healthSpring, groundSpring) often
//! validate numerical accuracy against Python/reference baselines using f64
//! comparisons, tolerance windows, and count checks. This module provides
//! [`NumericValidator`] as the standard adapter: accumulate numeric checks,
//! then [`bridge_into`](NumericValidator::bridge_into) a
//! [`ValidationResult`](super::ValidationResult) for scenario registry dispatch.
//!
//! # Migration Pattern
//!
//! ```rust,no_run
//! use primalspring::validation::numeric::NumericValidator;
//! use primalspring::validation::ValidationResult;
//!
//! fn legacy_validation(v: &mut NumericValidator) {
//!     v.check_f64("pi", std::f64::consts::PI, 3.14159, 1e-5);
//!     v.check_count("items", 42, 42);
//! }
//!
//! fn run_as_scenario(result: &mut ValidationResult) {
//!     let mut v = NumericValidator::new("my_experiment");
//!     legacy_validation(&mut v);
//!     v.bridge_into(result);
//! }
//! ```

use super::ValidationResult;

/// A numeric check result, accumulated before bridging.
#[derive(Debug, Clone)]
struct Check {
    name: String,
    passed: bool,
    detail: String,
}

/// Numeric validation accumulator for science spring experiments.
///
/// Collects f64/count/boolean checks, then bridges them into a
/// [`ValidationResult`] for eukaryotic scenario dispatch.
pub struct NumericValidator {
    name: String,
    checks: Vec<Check>,
    passed: u32,
    failed: u32,
}

impl NumericValidator {
    /// Create a new validator with the given experiment name.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            checks: Vec::new(),
            passed: 0,
            failed: 0,
        }
    }

    /// Check that two f64 values are within absolute tolerance.
    pub fn check_f64(&mut self, name: &str, actual: f64, expected: f64, tolerance: f64) {
        let diff = (actual - expected).abs();
        let ok = diff <= tolerance;
        self.record(name, ok, format!("actual={actual}, expected={expected}, diff={diff}, tol={tolerance}"));
    }

    /// Check that two f64 values are within relative tolerance.
    pub fn check_f64_rel(&mut self, name: &str, actual: f64, expected: f64, rel_tolerance: f64) {
        let diff = (actual - expected).abs();
        let denom = expected.abs().max(f64::EPSILON);
        let rel = diff / denom;
        let ok = rel <= rel_tolerance;
        self.record(name, ok, format!("actual={actual}, expected={expected}, rel_diff={rel:.2e}, tol={rel_tolerance}"));
    }

    /// Check that a count matches exactly.
    pub fn check_count(&mut self, name: &str, actual: usize, expected: usize) {
        let ok = actual == expected;
        self.record(name, ok, format!("actual={actual}, expected={expected}"));
    }

    /// Check that a count meets a minimum threshold.
    pub fn check_minimum(&mut self, name: &str, actual: usize, minimum: usize) {
        let ok = actual >= minimum;
        self.record(name, ok, format!("actual={actual}, minimum={minimum}"));
    }

    /// Record a boolean check.
    pub fn check_bool(&mut self, name: &str, condition: bool, detail: &str) {
        self.record(name, condition, detail.to_owned());
    }

    /// Total number of checks recorded.
    #[must_use]
    pub fn total(&self) -> u32 {
        self.passed + self.failed
    }

    /// Number of passing checks.
    #[must_use]
    pub fn passed(&self) -> u32 {
        self.passed
    }

    /// Number of failing checks.
    #[must_use]
    pub fn failed(&self) -> u32 {
        self.failed
    }

    /// Bridge all accumulated checks into a [`ValidationResult`].
    ///
    /// Each individual check becomes a `check_bool` entry in the result,
    /// preserving per-check granularity for CI triage.
    pub fn bridge_into(self, result: &mut ValidationResult) {
        result.section(&format!("NumericBridge: {}", self.name));
        for check in &self.checks {
            result.check_bool(&check.name, check.passed, &check.detail);
        }
    }

    /// Bridge as a single summary check (coarse — loses per-check granularity).
    ///
    /// Use [`bridge_into`](Self::bridge_into) for detailed bridging.
    pub fn bridge_into_summary(self, result: &mut ValidationResult) {
        result.check_bool(
            &format!("{} ({}/{})", self.name, self.passed, self.total()),
            self.passed == self.total() && self.total() > 0,
            &format!("passed={}, total={}, provenance=numeric-bridge", self.passed, self.total()),
        );
    }

    fn record(&mut self, name: &str, ok: bool, detail: String) {
        if ok {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
        self.checks.push(Check {
            name: name.to_owned(),
            passed: ok,
            detail,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_within_tolerance() {
        let mut v = NumericValidator::new("test");
        v.check_f64("pi", std::f64::consts::PI, 3.14159, 1e-4);
        assert_eq!(v.passed(), 1);
        assert_eq!(v.failed(), 0);
    }

    #[test]
    fn f64_outside_tolerance() {
        let mut v = NumericValidator::new("test");
        v.check_f64("pi", std::f64::consts::PI, 3.0, 0.01);
        assert_eq!(v.passed(), 0);
        assert_eq!(v.failed(), 1);
    }

    #[test]
    fn f64_relative_tolerance() {
        let mut v = NumericValidator::new("test");
        v.check_f64_rel("approx", 100.5, 100.0, 0.01);
        assert_eq!(v.passed(), 1);
        v.check_f64_rel("too_far", 110.0, 100.0, 0.01);
        assert_eq!(v.failed(), 1);
    }

    #[test]
    fn count_exact_match() {
        let mut v = NumericValidator::new("test");
        v.check_count("items", 42, 42);
        assert_eq!(v.passed(), 1);
        v.check_count("wrong", 41, 42);
        assert_eq!(v.failed(), 1);
    }

    #[test]
    fn bridge_preserves_granularity() {
        let mut v = NumericValidator::new("experiment");
        v.check_f64("a", 1.0, 1.0, 0.001);
        v.check_f64("b", 2.0, 3.0, 0.001);
        v.check_count("c", 5, 5);

        let mut result = ValidationResult::new("bridge-test");
        v.bridge_into(&mut result);
        assert_eq!(result.passed, 2);
        assert_eq!(result.failed, 1);
    }

    #[test]
    fn bridge_summary_coalesces() {
        let mut v = NumericValidator::new("experiment");
        v.check_f64("a", 1.0, 1.0, 0.001);
        v.check_count("b", 5, 5);

        let mut result = ValidationResult::new("summary-test");
        v.bridge_into_summary(&mut result);
        assert_eq!(result.passed, 1);
        assert_eq!(result.failed, 0);
    }
}
