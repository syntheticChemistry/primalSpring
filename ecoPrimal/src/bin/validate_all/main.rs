// SPDX-License-Identifier: AGPL-3.0-or-later

//! Meta-validator that runs all primalSpring experiment binaries in sequence.
//!
//! Follows the hotSpring/neuralSpring `validate_all` pattern: discover
//! experiment binaries at build time, run each one, collect pass/fail,
//! and exit 0 only if every experiment passes.

use std::process::Command;
use std::time::Instant;

use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;

/// Experiment binaries in execution order (tracks 1-8).
const EXPERIMENTS: &[&str] = &[
    "primalspring-exp001",
    "primalspring-exp002",
    "primalspring-exp003",
    "primalspring-exp004",
    "primalspring-exp005",
    "primalspring-exp006",
    "primalspring-exp010",
    "primalspring-exp011",
    "primalspring-exp012",
    "primalspring-exp013",
    "primalspring-exp014",
    "primalspring-exp015",
    "primalspring-exp020",
    "primalspring-exp021",
    "primalspring-exp022",
    "primalspring-exp023",
    "primalspring-exp024",
    "primalspring-exp025",
    "primalspring-exp030",
    "primalspring-exp031",
    "primalspring-exp032",
    "primalspring-exp033",
    "primalspring-exp034",
    "primalspring-exp040",
    "primalspring-exp041",
    "primalspring-exp042",
    "primalspring-exp043",
    "primalspring-exp044",
    "primalspring-exp050",
    "primalspring-exp051",
    "primalspring-exp052",
    "primalspring-exp053",
    "primalspring-exp054",
    "primalspring-exp055",
    "primalspring-exp056",
    "primalspring-exp057",
    "primalspring-exp058",
    "primalspring-exp059",
    "primalspring-exp060",
    "primalspring-exp061",
    "primalspring-exp062",
    "primalspring-exp063",
    "primalspring-exp064",
    "primalspring-exp065",
    "primalspring-exp066",
    "primalspring-exp067",
    "primalspring-exp068",
    "primalspring-exp069",
    "primalspring-exp070",
    "primalspring-exp071",
    "primalspring-exp072",
];

fn main() {
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!(
        "primalSpring validate_all — running {} experiments",
        EXPERIMENTS.len()
    );
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut failures: Vec<&str> = Vec::new();
    let wall_start = Instant::now();

    for &name in EXPERIMENTS {
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
                failures.push(name);
            }
            Err(e) => {
                println!("  [FAIL] {name} (spawn error: {e})");
                failed += 1;
                failures.push(name);
            }
        }
    }

    let wall_elapsed = wall_start.elapsed();
    println!("\n{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!(
        "validate_all: {passed}/{} passed, {failed} failed ({wall_elapsed:.1?})",
        EXPERIMENTS.len()
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
