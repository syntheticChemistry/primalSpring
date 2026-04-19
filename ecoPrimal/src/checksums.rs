// SPDX-License-Identifier: AGPL-3.0-or-later

//! CHECKSUMS generation and verification for guideStone Property 3 (Self-Verifying).
//!
//! Every guideStone must detect tampering in its own artifacts. This module
//! provides a standard way to generate and verify a `CHECKSUMS` manifest
//! file that covers all validation-critical files for a spring.
//!
//! The manifest format is one line per file:
//!
//! ```text
//! <blake3-hex>  <relative-path>
//! ```
//!
//! Springs generate this at build/release time and verify at runtime:
//!
//! ```rust,no_run
//! use primalspring::checksums;
//! use primalspring::validation::ValidationResult;
//!
//! let mut v = ValidationResult::new("self-verify");
//! checksums::verify_manifest(&mut v, "validation/CHECKSUMS");
//! ```

use std::io::BufRead;
use std::path::Path;

use crate::validation::ValidationResult;

/// Generate a BLAKE3 hex digest for a file's contents.
///
/// Returns `None` if the file cannot be read.
#[must_use]
pub fn blake3_file(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let hash = blake3::hash(&data);
    Some(hash.to_hex().to_string())
}

/// Generate a CHECKSUMS manifest for all files matching the given paths.
///
/// Each path is hashed with BLAKE3 and written as `<hex>  <path>`.
/// Paths that cannot be read are skipped with a warning to stderr.
///
/// Returns the manifest content as a string.
#[must_use]
pub fn generate_manifest(root: &Path, relative_paths: &[&str]) -> String {
    let mut lines = Vec::new();
    for &rel in relative_paths {
        let full = root.join(rel);
        match blake3_file(&full) {
            Some(hex) => lines.push(format!("{hex}  {rel}")),
            None => eprintln!("[checksums] warning: cannot read {rel}, skipping"),
        }
    }
    lines.join("\n")
}

/// Parse a CHECKSUMS manifest into (hash, path) pairs.
///
/// Returns an empty vec if the file cannot be read or parsed.
#[must_use]
pub fn parse_manifest(manifest_path: &Path) -> Vec<(String, String)> {
    let Ok(file) = std::fs::File::open(manifest_path) else {
        return Vec::new();
    };
    let reader = std::io::BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else { continue };
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((hash, path)) = line.split_once("  ") {
            entries.push((hash.to_owned(), path.to_owned()));
        }
    }
    entries
}

/// Verify a CHECKSUMS manifest against files on disk.
///
/// For each entry in the manifest, hashes the file and compares. Records
/// results on the [`ValidationResult`]:
/// - **PASS** if hash matches
/// - **FAIL** if hash does not match (tamper detected)
/// - **SKIP** if the file cannot be read
///
/// Returns `true` if the manifest was found and all entries verified.
pub fn verify_manifest(v: &mut ValidationResult, manifest_path: &str) -> bool {
    let manifest = Path::new(manifest_path);
    if !manifest.exists() {
        v.check_skip(
            "p3:checksums_manifest",
            &format!("{manifest_path} not found — generate with checksums::generate_manifest()"),
        );
        return false;
    }

    let entries = parse_manifest(manifest);
    if entries.is_empty() {
        v.check_bool(
            "p3:checksums_manifest",
            false,
            "manifest is empty or unparseable",
        );
        return false;
    }

    v.check_bool(
        "p3:checksums_manifest",
        true,
        &format!("{} entries in {manifest_path}", entries.len()),
    );

    let root = manifest
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("."));

    let mut all_ok = true;
    for (expected_hash, rel_path) in &entries {
        let check_name = format!("p3:checksum:{rel_path}");
        let full_path = root.join(rel_path);
        if let Some(actual_hash) = blake3_file(&full_path) {
            let ok = actual_hash == *expected_hash;
            if ok {
                v.check_bool(&check_name, true, "hash matches");
            } else {
                v.check_bool(
                    &check_name,
                    false,
                    &format!(
                        "TAMPER: expected {}, got {}",
                        &expected_hash[..expected_hash.len().min(16)],
                        &actual_hash[..actual_hash.len().min(16)]
                    ),
                );
                all_ok = false;
            }
        } else {
            v.check_skip(&check_name, &format!("cannot read {rel_path}"));
            all_ok = false;
        }
    }
    all_ok
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::{NullSink, ValidationResult};
    use std::sync::Arc;

    fn null_result(name: &str) -> ValidationResult {
        ValidationResult::new(name).with_sink(Arc::new(NullSink))
    }

    #[test]
    fn blake3_file_on_existing() {
        let path = Path::new("Cargo.toml");
        let hash = blake3_file(path);
        assert!(hash.is_some(), "should hash Cargo.toml");
        assert_eq!(hash.unwrap().len(), 64, "BLAKE3 hex is 64 chars");
    }

    #[test]
    fn blake3_file_on_missing() {
        let path = Path::new("nonexistent_file_xyz.toml");
        assert!(blake3_file(path).is_none());
    }

    #[test]
    fn generate_manifest_produces_lines() {
        let manifest = generate_manifest(Path::new("."), &["Cargo.toml"]);
        assert!(manifest.contains("Cargo.toml"));
        let parts: Vec<&str> = manifest.split("  ").collect();
        assert_eq!(parts.len(), 2, "format: <hash>  <path>");
        assert_eq!(parts[0].len(), 64, "BLAKE3 hex length");
    }

    #[test]
    fn verify_manifest_skips_on_missing() {
        let mut v = null_result("test");
        let ok = verify_manifest(&mut v, "nonexistent/CHECKSUMS");
        assert!(!ok);
        assert_eq!(v.skipped, 1);
    }

    #[test]
    fn parse_manifest_handles_comments_and_blanks() {
        let dir = std::env::temp_dir().join("primalspring_checksum_test");
        let _ = std::fs::create_dir_all(&dir);
        let manifest_path = dir.join("CHECKSUMS");
        std::fs::write(
            &manifest_path,
            "# comment\n\nabc123  some/file.rs\ndef456  other/file.rs\n",
        )
        .unwrap();
        let entries = parse_manifest(&manifest_path);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].0, "abc123");
        assert_eq!(entries[0].1, "some/file.rs");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
