// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data dependency validation — verify file existence and BLAKE3 integrity
//! before dispatch.
//!
//! Extracted from toadStool S266's `validate_data_dependencies()` pattern
//! (workload TOML pre-dispatch staging). Generalized for any primal or spring
//! that needs to verify input artifacts before execution.
//!
//! # Usage
//!
//! ```rust,no_run
//! use primalspring::validation::dependency::{DependencySpec, validate_dependencies};
//!
//! let deps = vec![
//!     DependencySpec::required("data/genome.fasta", None),
//!     DependencySpec::required("data/reference.fa", Some("abc123...")),
//!     DependencySpec::optional("data/annotations.gff", None),
//! ];
//! let report = validate_dependencies(&deps);
//! assert_eq!(report.failed, 0);
//! ```

use std::path::Path;

/// A single data dependency to validate before dispatch.
#[derive(Debug, Clone)]
pub struct DependencySpec {
    /// Filesystem path (absolute or relative to base).
    pub path: String,
    /// Whether this dependency must exist for dispatch to proceed.
    pub required: bool,
    /// Expected BLAKE3 hex digest. If `Some`, the file's hash is verified.
    pub expected_blake3: Option<String>,
}

impl DependencySpec {
    /// Create a required dependency (dispatch fails if missing).
    #[must_use]
    pub fn required(path: &str, blake3: Option<&str>) -> Self {
        Self {
            path: path.to_owned(),
            required: true,
            expected_blake3: blake3.map(ToOwned::to_owned),
        }
    }

    /// Create an optional dependency (skipped if missing, logged as warning).
    #[must_use]
    pub fn optional(path: &str, blake3: Option<&str>) -> Self {
        Self {
            path: path.to_owned(),
            required: false,
            expected_blake3: blake3.map(ToOwned::to_owned),
        }
    }
}

/// Result of validating a set of dependencies.
#[derive(Debug, Clone, Default)]
pub struct DependencyReport {
    /// Number of dependencies that passed validation.
    pub passed: usize,
    /// Number of dependencies that failed (required missing, hash mismatch).
    pub failed: usize,
    /// Number of optional dependencies that were absent.
    pub skipped: usize,
    /// Human-readable messages for each dependency checked.
    pub messages: Vec<String>,
}

impl DependencyReport {
    /// Returns `true` when no required dependencies are missing or corrupt.
    #[must_use]
    pub const fn is_ok(&self) -> bool {
        self.failed == 0
    }

    /// Record this report onto a `ValidationResult`.
    pub fn record_on(&self, v: &mut super::ValidationResult, prefix: &str) {
        v.check_bool(
            &format!("{prefix}_deps_ok"),
            self.is_ok(),
            &format!("{prefix} all required dependencies present and verified"),
        );
        if self.failed > 0 {
            for msg in &self.messages {
                if msg.starts_with("FAIL") {
                    v.check_bool(&format!("{prefix}_dep_detail"), false, msg);
                }
            }
        }
    }
}

/// Validate a set of dependencies against the filesystem.
///
/// For each dependency:
/// - Check existence (FAIL for required, SKIP for optional)
/// - If `expected_blake3` is set and file exists, verify the hash
#[must_use]
pub fn validate_dependencies(deps: &[DependencySpec]) -> DependencyReport {
    validate_dependencies_at(deps, Path::new("."))
}

/// Validate dependencies relative to a base directory.
#[must_use]
pub fn validate_dependencies_at(deps: &[DependencySpec], base: &Path) -> DependencyReport {
    let mut report = DependencyReport::default();

    for dep in deps {
        let full_path = base.join(&dep.path);

        if !full_path.exists() {
            if dep.required {
                report.failed += 1;
                report
                    .messages
                    .push(format!("FAIL: required dependency missing: {}", dep.path));
            } else {
                report.skipped += 1;
                report
                    .messages
                    .push(format!("SKIP: optional dependency missing: {}", dep.path));
            }
            continue;
        }

        if let Some(ref expected) = dep.expected_blake3 {
            match hash_file_blake3(&full_path) {
                Ok(actual) if actual == *expected => {
                    report.passed += 1;
                    report
                        .messages
                        .push(format!("PASS: {} (BLAKE3 verified)", dep.path));
                }
                Ok(actual) => {
                    report.failed += 1;
                    report.messages.push(format!(
                        "FAIL: {} BLAKE3 mismatch: expected {}, got {}",
                        dep.path, expected, actual
                    ));
                }
                Err(e) => {
                    report.failed += 1;
                    report
                        .messages
                        .push(format!("FAIL: {} hash error: {e}", dep.path));
                }
            }
        } else {
            report.passed += 1;
            report.messages.push(format!("PASS: {} (exists)", dep.path));
        }
    }

    report
}

fn hash_file_blake3(path: &Path) -> Result<String, std::io::Error> {
    let data = std::fs::read(path)?;
    let hash = blake3::hash(&data);
    Ok(hash.to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn required_missing_fails() {
        let deps = vec![DependencySpec::required("/nonexistent/path/xyzzy", None)];
        let report = validate_dependencies(&deps);
        assert_eq!(report.failed, 1);
        assert!(!report.is_ok());
    }

    #[test]
    fn optional_missing_skips() {
        let deps = vec![DependencySpec::optional("/nonexistent/path/xyzzy", None)];
        let report = validate_dependencies(&deps);
        assert_eq!(report.skipped, 1);
        assert_eq!(report.failed, 0);
        assert!(report.is_ok());
    }

    #[test]
    fn existing_file_passes() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, b"hello").unwrap();

        let deps = vec![DependencySpec::required(file_path.to_str().unwrap(), None)];
        let report = validate_dependencies(&deps);
        assert_eq!(report.passed, 1);
        assert!(report.is_ok());
    }

    #[test]
    fn blake3_match_passes() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("verified.bin");
        std::fs::write(&file_path, b"test data for hashing").unwrap();

        let expected = blake3::hash(b"test data for hashing").to_hex().to_string();
        let deps = vec![DependencySpec::required(
            file_path.to_str().unwrap(),
            Some(&expected),
        )];
        let report = validate_dependencies(&deps);
        assert_eq!(report.passed, 1);
        assert!(report.is_ok());
    }

    #[test]
    fn blake3_mismatch_fails() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("tampered.bin");
        std::fs::write(&file_path, b"actual content").unwrap();

        let deps = vec![DependencySpec::required(
            file_path.to_str().unwrap(),
            Some("0000000000000000000000000000000000000000000000000000000000000000"),
        )];
        let report = validate_dependencies(&deps);
        assert_eq!(report.failed, 1);
        assert!(!report.is_ok());
    }

    #[test]
    fn mixed_deps_report() {
        let dir = tempfile::tempdir().unwrap();
        let exists = dir.path().join("exists.txt");
        std::fs::write(&exists, b"present").unwrap();

        let deps = vec![
            DependencySpec::required(exists.to_str().unwrap(), None),
            DependencySpec::required("/nonexistent/required", None),
            DependencySpec::optional("/nonexistent/optional", None),
        ];
        let report = validate_dependencies(&deps);
        assert_eq!(report.passed, 1);
        assert_eq!(report.failed, 1);
        assert_eq!(report.skipped, 1);
        assert!(!report.is_ok());
    }

    #[test]
    fn record_on_validation_result() {
        use crate::validation::{NullSink, ValidationResult};
        use std::sync::Arc;

        let mut v = ValidationResult::new("dep_test").with_sink(Arc::new(NullSink));
        let report = DependencyReport {
            passed: 3,
            failed: 0,
            skipped: 1,
            messages: vec!["PASS: a".to_owned()],
        };
        report.record_on(&mut v, "workload");
        assert_eq!(v.passed, 1);
        assert_eq!(v.failed, 0);
    }

    #[test]
    fn validate_at_base_dir() {
        let dir = tempfile::tempdir().unwrap();
        let mut f = std::fs::File::create(dir.path().join("input.csv")).unwrap();
        f.write_all(b"col1,col2\n1,2\n").unwrap();

        let deps = vec![DependencySpec::required("input.csv", None)];
        let report = validate_dependencies_at(&deps, dir.path());
        assert_eq!(report.passed, 1);
        assert!(report.is_ok());
    }
}
