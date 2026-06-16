// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Release validation gate — pure Rust replacement for `validate_release.sh`.
//!
//! Runs the quality pipeline (fmt, clippy, tests, docs) and verifies the
//! plasmidBin depot. Cargo invocations are intentionally `Command` calls
//! since cargo is an external tool.

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process::Command;

use primalspring::primal_names;
use primalspring::validation::ValidationResult;

const MIN_TESTS: u32 = 750;
const MIN_COVERAGE: u32 = 70;

/// Full roster including orchestrator — derived from canonical `Primal::ALL`.
fn expected_primals() -> Vec<&'static str> {
    primal_names::Primal::ALL.iter().map(|p| p.slug()).collect()
}

pub struct ReleaseArgs {
    pub skip_coverage: bool,
    pub skip_nucleus: bool,
    pub json: bool,
}

pub fn run(args: &ReleaseArgs) {
    let mut v = ValidationResult::new("primalSpring Release Validation Gate");

    check_fmt(&mut v);
    check_clippy(&mut v);
    check_deny(&mut v);
    check_tests(&mut v);
    if !args.skip_coverage {
        check_coverage(&mut v);
    }
    check_docs(&mut v);
    check_plasmidbin(&mut v);
    if !args.skip_nucleus {
        check_nucleus(&mut v);
    }

    if args.json {
        if let Ok(j) = v.to_json() {
            println!("{j}");
        }
    } else {
        v.finish();
    }
    std::process::exit(v.exit_code());
}

fn check_fmt(v: &mut ValidationResult) {
    v.section("cargo fmt --check");
    let ok = Command::new("cargo")
        .args(["fmt", "--all", "--check"])
        .output()
        .is_ok_and(|o| o.status.success());
    v.check_bool("fmt-clean", ok, "formatting clean");
}

fn check_clippy(v: &mut ValidationResult) {
    v.section("cargo clippy --workspace");
    let ok = Command::new("cargo")
        .args(["clippy", "--workspace", "--", "-D", "warnings"])
        .output()
        .is_ok_and(|o| o.status.success());
    v.check_bool("clippy-clean", ok, "clippy clean");
}

fn check_deny(v: &mut ValidationResult) {
    v.section("cargo deny check");
    let has_deny = Command::new("cargo-deny")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success());
    if !has_deny {
        v.check_bool("deny-skip", true, "cargo-deny not installed — skipped");
        return;
    }
    let ok = Command::new("cargo")
        .args(["deny", "check"])
        .output()
        .is_ok_and(|o| o.status.success());
    v.check_bool("deny-clean", ok, "dependency audit clean");
}

fn check_tests(v: &mut ValidationResult) {
    v.section("cargo test --workspace");
    let output = Command::new("cargo").args(["test", "--workspace"]).output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let stderr = String::from_utf8_lossy(&o.stderr);
            let combined = format!("{stdout}\n{stderr}");

            let has_failures = combined.contains("FAILED");
            v.check_bool(
                "tests-pass",
                !has_failures && o.status.success(),
                "all tests passed",
            );

            let test_count: u32 = combined
                .lines()
                .filter(|l| l.contains("test result:"))
                .filter_map(|l| {
                    l.split_whitespace()
                        .nth(3)
                        .and_then(|n| n.parse::<u32>().ok())
                })
                .sum();
            v.check_bool(
                "test-count",
                test_count >= MIN_TESTS,
                &format!("test count: {test_count} (floor: {MIN_TESTS})"),
            );
        }
        Err(e) => {
            v.check_bool(
                "tests-run",
                false,
                &format!("cargo test failed to spawn: {e}"),
            );
        }
    }
}

fn check_coverage(v: &mut ValidationResult) {
    v.section("cargo llvm-cov");
    let has_cov = Command::new("cargo-llvm-cov")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success());
    if !has_cov {
        v.check_bool(
            "coverage-skip",
            true,
            "cargo-llvm-cov not installed — skipped",
        );
        return;
    }
    let output = Command::new("cargo")
        .args(["llvm-cov", "--workspace", "--json"])
        .output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let cov_pct = parse_line_coverage(&stdout).unwrap_or(0);
            v.check_bool(
                "line-coverage",
                cov_pct >= MIN_COVERAGE,
                &format!("line coverage: {cov_pct}% (floor: {MIN_COVERAGE}%)"),
            );
        }
        Err(e) => {
            v.check_bool("coverage-run", false, &format!("llvm-cov failed: {e}"));
        }
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "bounded by 100.0"
)]
fn parse_line_coverage(json: &str) -> Option<u32> {
    let count_marker = "\"lines\":{\"count\":";
    let pos = json.find(count_marker)?;
    let rest = &json[pos + count_marker.len()..];
    let count: f64 = rest.split(',').next()?.parse().ok()?;
    let covered_marker = "\"covered\":";
    let cpos = rest.find(covered_marker)?;
    let crest = &rest[cpos + covered_marker.len()..];
    let covered: f64 = crest.split([',', '}']).next()?.parse().ok()?;
    if count > 0.0 {
        Some((covered / count * 100.0) as u32)
    } else {
        Some(0)
    }
}

fn check_docs(v: &mut ValidationResult) {
    v.section("cargo doc --workspace --no-deps");
    let ok = Command::new("cargo")
        .args(["doc", "--workspace", "--no-deps"])
        .output()
        .is_ok_and(|o| o.status.success());
    v.check_bool("docs-clean", ok, "docs build clean");
}

fn check_plasmidbin(v: &mut ValidationResult) {
    v.section("plasmidBin health check");
    let depot_dir = resolve_depot_dir();
    if !depot_dir.is_dir() {
        v.check_bool(
            "plasmidbin-skip",
            true,
            &format!("depot not found at {} — skipped", depot_dir.display()),
        );
        return;
    }

    let primals = expected_primals();
    let mut all_ok = true;
    for &primal in &primals {
        let bin = depot_dir.join(primal);
        if !bin.is_file() {
            v.check_bool(
                &format!("depot-{primal}"),
                false,
                &format!("missing: {primal}"),
            );
            all_ok = false;
        }
    }
    if all_ok {
        v.check_bool(
            "depot-binaries",
            true,
            &format!("{} primals present in depot", primals.len()),
        );
    }

    let depot_root = depot_dir
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(&depot_dir);
    let checksums = depot_root.join("checksums.toml");
    v.check_bool(
        "checksums-present",
        checksums.is_file(),
        "checksums.toml present",
    );

    let provenance = depot_root.join("provenance.toml");
    v.check_bool(
        "provenance-present",
        provenance.is_file(),
        "provenance.toml present",
    );
}

fn check_nucleus(v: &mut ValidationResult) {
    v.section("NUCLEUS deployment gate");

    if std::env::var(primalspring::env_keys::ECOPRIMALS_ROOT).is_err()
        && std::env::var(primalspring::env_keys::ECOPRIMALS_PLASMID_BIN).is_err()
    {
        v.check_bool("nucleus-skip", true, "ECOPRIMALS_ROOT not set — skipped");
        return;
    }

    let binary = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("primalspring_unibin"));
    let output = Command::new(&binary)
        .args(["nucleus", "--skip-launch", "--json"])
        .output();
    match output {
        Ok(o) => {
            v.check_bool(
                "nucleus-gate",
                o.status.success(),
                if o.status.success() {
                    "NUCLEUS gate passed (pre-flight + live checks)"
                } else {
                    "NUCLEUS gate found issues — run `primalspring nucleus` for details"
                },
            );
        }
        Err(e) => {
            v.check_bool(
                "nucleus-gate",
                false,
                &format!("nucleus subcommand failed: {e}"),
            );
        }
    }
}

fn resolve_depot_dir() -> PathBuf {
    if let Ok(bin) = std::env::var(primalspring::env_keys::ECOPRIMALS_PLASMID_BIN) {
        return PathBuf::from(bin);
    }
    let triple = primalspring::tolerances::current_target_triple();
    PathBuf::from(primalspring::tolerances::plasmidbin_depot_root())
        .join("primals")
        .join(&triple)
}
