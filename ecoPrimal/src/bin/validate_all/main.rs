// SPDX-License-Identifier: AGPL-3.0-or-later

//! Meta-validator that runs all primalSpring experiment binaries in sequence.
//!
//! Auto-discovers experiment packages from `cargo metadata` rather than
//! maintaining a hardcoded list. Any workspace member whose package name
//! starts with `primalspring-exp` is treated as an experiment.

use std::process::Command;
use std::time::Instant;

use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;

/// Discover experiment package names from `cargo metadata`.
///
/// Falls back to an empty list if metadata cannot be read (e.g. outside
/// the workspace). Sorts lexicographically so execution order is stable.
fn discover_experiments() -> Vec<String> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps"])
        .output();

    let Ok(output) = output else {
        eprintln!("[validate_all] warning: cargo metadata failed, no experiments discovered");
        return Vec::new();
    };

    let Ok(meta) = serde_json::from_slice::<serde_json::Value>(&output.stdout) else {
        eprintln!("[validate_all] warning: failed to parse cargo metadata JSON");
        return Vec::new();
    };

    let mut experiments: Vec<String> = meta
        .get("packages")
        .and_then(|p| p.as_array())
        .into_iter()
        .flatten()
        .filter_map(|pkg| {
            let name = pkg.get("name")?.as_str()?;
            name.starts_with("primalspring-exp")
                .then(|| name.to_owned())
        })
        .collect();

    experiments.sort();
    experiments
}

fn main() {
    let experiments = discover_experiments();

    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!(
        "primalSpring validate_all — running {} experiments",
        experiments.len()
    );
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut failures: Vec<String> = Vec::new();
    let wall_start = Instant::now();

    for name in &experiments {
        let start = Instant::now();
        let result = Command::new("cargo")
            .args(["run", "--release", "-p", name])
            .env("PRIMALSPRING_JSON", "0")
            .status();

        let elapsed = start.elapsed();
        match result {
            Ok(status) if status.success() => {
                println!("  [PASS] {name} ({elapsed:.1?})");
                passed += 1;
            }
            Ok(status) => {
                let code = status.code().unwrap_or(-1);
                println!("  [FAIL] {name} (exit {code}, {elapsed:.1?})");
                failed += 1;
                failures.push(name.clone());
            }
            Err(e) => {
                println!("  [FAIL] {name} (spawn error: {e})");
                failed += 1;
                failures.push(name.clone());
            }
        }
    }

    let wall_elapsed = wall_start.elapsed();
    println!("\n{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!(
        "validate_all: {passed}/{} passed, {failed} failed ({wall_elapsed:.1?})",
        experiments.len()
    );

    if !failures.is_empty() {
        println!("\nFailed experiments:");
        for f in &failures {
            println!("  - {f}");
        }
    }

    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    std::process::exit(i32::from(failed > 0));
}
