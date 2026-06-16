// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation runner — executes scenario suites against a live or structural NUCLEUS.

use primalspring::composition::CompositionContext;
use primalspring::coordination::AtomicType;
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

    if failed > 0 {
        std::process::exit(1);
    }
}
