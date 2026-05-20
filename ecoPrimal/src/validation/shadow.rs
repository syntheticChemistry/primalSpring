// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shadow comparison — run two code paths and compare results.
//!
//! Extracted from songbird Wave 213's `shadow_comparator::compare_paths` pattern
//! (TURN vs cloudflared parallel execution). Generalized for any A/B comparison
//! where you want to run a primary and shadow path, collect latency + correctness
//! metrics, and record structured results on a `ValidationResult`.
//!
//! # Usage
//!
//! ```rust,no_run
//! use primalspring::validation::shadow::{ShadowComparison, ShadowResult};
//!
//! let result = ShadowComparison::run("tls_path", || {
//!     // primary: sovereign TLS termination
//!     Ok("response_hash_abc".to_string())
//! }, || {
//!     // shadow: cloudflare proxy
//!     Ok("response_hash_abc".to_string())
//! });
//!
//! assert!(result.outcomes_match);
//! ```

use std::time::Instant;

/// Result of a shadow comparison between two execution paths.
#[derive(Debug, Clone)]
pub struct ShadowResult {
    /// Descriptive label for this comparison (e.g. `"tls_termination"`).
    pub label: String,
    /// Primary path latency in microseconds.
    pub primary_latency_us: u64,
    /// Shadow path latency in microseconds.
    pub shadow_latency_us: u64,
    /// Whether both paths produced identical output.
    pub outcomes_match: bool,
    /// Whether the primary path succeeded.
    pub primary_ok: bool,
    /// Whether the shadow path succeeded.
    pub shadow_ok: bool,
    /// Primary path output (if it succeeded).
    pub primary_value: Option<String>,
    /// Shadow path output (if it succeeded).
    pub shadow_value: Option<String>,
}

impl ShadowResult {
    /// Latency ratio: shadow / primary. Values > 1.0 mean shadow is slower.
    #[must_use]
    pub fn latency_ratio(&self) -> f64 {
        if self.primary_latency_us == 0 {
            return f64::INFINITY;
        }
        self.shadow_latency_us as f64 / self.primary_latency_us as f64
    }

    /// Record this comparison onto a `ValidationResult` with structured checks.
    pub fn record_on(&self, v: &mut super::ValidationResult) {
        v.check_bool(
            &format!("shadow_{}_primary_ok", self.label),
            self.primary_ok,
            &format!("{} primary path succeeded", self.label),
        );
        v.check_bool(
            &format!("shadow_{}_shadow_ok", self.label),
            self.shadow_ok,
            &format!("{} shadow path succeeded", self.label),
        );
        v.check_bool(
            &format!("shadow_{}_match", self.label),
            self.outcomes_match,
            &format!("{} primary and shadow outcomes match", self.label),
        );
    }
}

/// Shadow comparison runner.
///
/// Executes two closures (primary + shadow), captures timing and result,
/// compares outputs by string equality.
pub struct ShadowComparison;

impl ShadowComparison {
    /// Run a shadow comparison. Both paths return `Result<String, E>` where
    /// the string is a comparable output (hash, status, etc).
    pub fn run<F1, F2, E1, E2>(
        label: &str,
        primary: F1,
        shadow: F2,
    ) -> ShadowResult
    where
        F1: FnOnce() -> Result<String, E1>,
        F2: FnOnce() -> Result<String, E2>,
        E1: std::fmt::Debug,
        E2: std::fmt::Debug,
    {
        let start_primary = Instant::now();
        let primary_result = primary();
        let primary_latency_us = crate::cast::micros_u64(start_primary.elapsed());

        let start_shadow = Instant::now();
        let shadow_result = shadow();
        let shadow_latency_us = crate::cast::micros_u64(start_shadow.elapsed());

        let primary_ok = primary_result.is_ok();
        let shadow_ok = shadow_result.is_ok();
        let primary_value = primary_result.ok();
        let shadow_value = shadow_result.ok();

        let outcomes_match = match (&primary_value, &shadow_value) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };

        ShadowResult {
            label: label.to_owned(),
            primary_latency_us,
            shadow_latency_us,
            outcomes_match,
            primary_ok,
            shadow_ok,
            primary_value,
            shadow_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_paths_report_match() {
        let result = ShadowComparison::run(
            "test_equal",
            || Ok::<_, &str>("hello".to_owned()),
            || Ok::<_, &str>("hello".to_owned()),
        );
        assert!(result.outcomes_match);
        assert!(result.primary_ok);
        assert!(result.shadow_ok);
    }

    #[test]
    fn divergent_paths_report_mismatch() {
        let result = ShadowComparison::run(
            "test_diverge",
            || Ok::<_, &str>("aaa".to_owned()),
            || Ok::<_, &str>("bbb".to_owned()),
        );
        assert!(!result.outcomes_match);
        assert!(result.primary_ok);
        assert!(result.shadow_ok);
    }

    #[test]
    fn primary_failure_is_captured() {
        let result = ShadowComparison::run(
            "test_primary_fail",
            || Err::<String, _>("primary broke"),
            || Ok::<_, &str>("shadow ok".to_owned()),
        );
        assert!(!result.primary_ok);
        assert!(result.shadow_ok);
        assert!(!result.outcomes_match);
    }

    #[test]
    fn both_fail_counts_as_match() {
        let result = ShadowComparison::run(
            "test_both_fail",
            || Err::<String, _>("nope"),
            || Err::<String, _>("also nope"),
        );
        assert!(!result.primary_ok);
        assert!(!result.shadow_ok);
        assert!(result.outcomes_match);
    }

    #[test]
    fn latency_ratio_computed() {
        let result = ShadowResult {
            label: "test".to_owned(),
            primary_latency_us: 100,
            shadow_latency_us: 200,
            outcomes_match: true,
            primary_ok: true,
            shadow_ok: true,
            primary_value: None,
            shadow_value: None,
        };
        assert!((result.latency_ratio() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn record_on_validation_result() {
        use crate::validation::{NullSink, ValidationResult};
        use std::sync::Arc;

        let mut v = ValidationResult::new("shadow_test").with_sink(Arc::new(NullSink));
        let result = ShadowComparison::run(
            "rec",
            || Ok::<_, &str>("ok".to_owned()),
            || Ok::<_, &str>("ok".to_owned()),
        );
        result.record_on(&mut v);
        assert_eq!(v.passed, 3);
        assert_eq!(v.failed, 0);
    }
}
