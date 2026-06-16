// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation runner — executes scenario suites against a live or structural NUCLEUS.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
use primalspring::evolution::{self, Target};
use primalspring::validation::scenarios::{Tier, build_registry};
use primalspring::validation::ValidationResult;

/// Run validation scenarios against a live NUCLEUS.
///
/// Discovers composition via standard IPC, then runs either a specific
/// scenario or the default suite for the active composition type.
pub fn run_validation(atomic: AtomicType, scenario_id: Option<&str>, structural_only: bool) {
    println!();
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!("\x1b[36m  NUCLEUS Validation\x1b[0m");
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!();
    println!("  Composition: {atomic:?}");
    println!(
        "  Mode:        {}",
        if structural_only {
            "structural only (Tier::Rust)"
        } else {
            "full (structural + live)"
        }
    );
    println!();

    let registry = build_registry();
    let mut ctx = CompositionContext::discover();

    let scenarios: Vec<&_> = scenario_id.map_or_else(
        || {
            if structural_only {
                registry.filter_by_tier(Tier::Rust).collect()
            } else {
                registry.all().iter().collect()
            }
        },
        |id| {
            let Some(s) = registry.all().iter().find(|s| s.meta.id == id) else {
                eprintln!("error: scenario '{id}' not found in registry");
                eprintln!(
                    "  available: {}",
                    registry
                        .all()
                        .iter()
                        .map(|s| s.meta.id)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                std::process::exit(1);
            };
            vec![s]
        },
    );

    let total = scenarios.len();
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped_count = 0usize;

    for (i, scenario) in scenarios.iter().enumerate() {
        let mut v = ValidationResult::new(scenario.meta.id);
        println!("  [{}/{}] {} ...", i + 1, total, scenario.meta.id);
        (scenario.run)(&mut v, &mut ctx);

        if v.failed == 0 {
            passed += 1;
            println!(
                "    \x1b[32mPASS\x1b[0m ({} checks, {} skipped)",
                v.evaluated(),
                v.skipped
            );
        } else {
            failed += 1;
            println!(
                "    \x1b[31mFAIL\x1b[0m ({} failures, {} passed, {} skipped)",
                v.failed,
                v.evaluated().saturating_sub(v.failed),
                v.skipped
            );
        }
        if v.evaluated() == 0 && v.skipped > 0 {
            skipped_count += 1;
        }
    }

    println!();
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    if failed == 0 {
        println!(
            "  \x1b[32mALL PASS\x1b[0m — {passed}/{total} scenarios green ({skipped_count} fully skipped)"
        );
    } else {
        println!(
            "  \x1b[31m{failed} FAILED\x1b[0m — {passed} passed, {failed} failed, {skipped_count} skipped"
        );
    }
    println!("\x1b[36m══════════════════════════════════════════════\x1b[0m");
    println!();

    print_fitness_summary(passed, failed, total);

    if failed > 0 {
        std::process::exit(1);
    }
}

/// Print a fitness report summarizing the current target's evolution posture.
fn print_fitness_summary(passed: usize, _failed: usize, total: usize) {
    let target = Target::current();
    let pressures = evolution::PressureCategory::active_for(target);

    println!("  \x1b[36mFitness\x1b[0m: {target} | {} pressures active", pressures.len());

    #[expect(clippy::cast_precision_loss, reason = "scenario counts are tiny")]
    let score = if total > 0 {
        passed as f64 / total as f64
    } else {
        0.0
    };
    let bar_len = 20;
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "bar rendering")]
    let filled = (score * bar_len as f64) as usize;
    let empty = bar_len - filled;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

    let color = if score >= 0.95 {
        "\x1b[32m"
    } else if score >= 0.7 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };
    println!("  {color}{bar}\x1b[0m {:.0}% ({passed}/{total} scenarios)", score * 100.0);
    println!();
}
