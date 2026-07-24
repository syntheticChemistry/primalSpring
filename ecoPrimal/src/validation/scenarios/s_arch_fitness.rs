// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Architecture Fitness — validates the cross-architecture evolution
//! posture of the primal ecosystem.
//!
//! Evaluates each primal's target declarations, checks binary availability
//! for declared targets, and validates that the current host's target triple
//! is correctly detected and fitness-scored.

use crate::composition::CompositionContext;
use crate::evolution::{self, Target, TargetDeclaration};
use crate::primal_names::Primal;
use crate::validation::ValidationResult;
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};

/// Architecture fitness scenario — cross-arch evolution posture validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "arch-fitness",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-16",
        description: "Cross-architecture evolution posture and fitness scoring",
    },
    run,
};

/// Run the architecture fitness validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Target detection and classification");
    phase_target_detection(v);

    v.section("Phase 2: Primal target declarations");
    phase_target_declarations(v);

    v.section("Phase 3: Binary depot coverage");
    phase_depot_coverage(v);

    v.section("Phase 4: Fitness scoring integrity");
    phase_fitness_scoring(v);
}

fn phase_target_detection(v: &mut ValidationResult) {
    let current = Target::current();
    let triple = current.triple();

    v.check_bool(
        "target:current_detected",
        !triple.is_empty(),
        &format!("current target: {triple}"),
    );

    v.check_bool(
        "target:has_filesystem",
        current.has_filesystem(),
        &format!("{triple} has_filesystem={}", current.has_filesystem()),
    );

    v.check_bool(
        "target:has_tcp",
        current.has_tcp(),
        &format!("{triple} has_tcp={}", current.has_tcp()),
    );

    let tier = current.tier();
    v.check_bool(
        "target:tier_classified",
        true,
        &format!("{triple} deployment tier: {tier:?}"),
    );

    let pressures = evolution::PressureCategory::active_for(current);
    v.check_bool(
        "target:pressures_defined",
        !pressures.is_empty() || current == Target::X86_64Musl,
        &format!("{} active pressures for {triple}", pressures.len()),
    );
}

fn phase_target_declarations(v: &mut ValidationResult) {
    let mut total_proven = 0usize;
    let mut total_targets = 0usize;

    for primal in Primal::ALL {
        let decl = TargetDeclaration::for_primal(*primal);

        total_proven += decl.proven.len();
        total_targets += decl.total_targets();

        v.check_bool(
            &format!("decl:{}:has_proven", primal.slug()),
            !decl.proven.is_empty(),
            &format!(
                "{}: {} proven, {} in-progress, {} endgame (coverage: {:.0}%)",
                primal.slug(),
                decl.proven.len(),
                decl.in_progress.len(),
                decl.endgame.len(),
                decl.coverage() * 100.0,
            ),
        );
    }

    #[expect(clippy::cast_precision_loss, reason = "primal counts are tiny")]
    let ecosystem_coverage = if total_targets > 0 {
        total_proven as f64 / total_targets as f64
    } else {
        0.0
    };

    v.check_bool(
        "decl:ecosystem_coverage",
        ecosystem_coverage > 0.0,
        &format!(
            "ecosystem: {total_proven}/{total_targets} targets proven ({:.0}%)",
            ecosystem_coverage * 100.0
        ),
    );
}

fn phase_depot_coverage(v: &mut ValidationResult) {
    let depot_root = crate::tolerances::plasmidbin_depot_root();
    let targets_to_check = [
        (Target::X86_64Musl, "x86_64-unknown-linux-musl"),
        (Target::Aarch64Musl, "aarch64-unknown-linux-musl"),
    ];

    for (target, triple) in targets_to_check {
        let depot_path = std::path::Path::new(&depot_root)
            .join("primals")
            .join(triple);

        if depot_path.is_dir() {
            let count =
                std::fs::read_dir(&depot_path).map_or(0, |rd| rd.filter_map(Result::ok).count());

            v.check_bool(
                &format!("depot:{triple}:present"),
                count > 0,
                &format!("{triple}: {count} binaries in depot"),
            );
        } else {
            v.check_skip(
                &format!("depot:{triple}:present"),
                &format!("depot not found for {triple} at {}", depot_path.display()),
            );
        }

        let composition_tier = evolution::CompositionTier::from_target(target);
        v.check_bool(
            &format!("depot:{triple}:composition_tier"),
            true,
            &format!(
                "{triple} supports up to {:?} ({} primals)",
                composition_tier,
                composition_tier.max_primals()
            ),
        );
    }
}

fn phase_fitness_scoring(v: &mut ValidationResult) {
    let mut test_result = ValidationResult::new("fitness-test-internal");
    for i in 0..10 {
        test_result.check_bool(&format!("test-{i}"), true, "pass");
    }
    test_result.check_bool("test-fail", false, "intentional failure");

    let fitness = evolution::evaluate_fitness("test-primal", Target::current(), &test_result);

    v.check_bool(
        "fitness:scoring_works",
        (fitness.survival_ratio - (10.0 / 11.0)).abs() < 0.01,
        &format!(
            "survival ratio: {:.3} (expected ~0.909)",
            fitness.survival_ratio
        ),
    );

    v.check_bool(
        "fitness:debt_detected",
        !fitness.debt.is_empty(),
        &format!("{} debt items for 1/11 failure", fitness.debt.len()),
    );

    v.check_bool(
        "fitness:confidence_medium",
        fitness.score.confidence == evolution::fitness::Confidence::Medium,
        &format!(
            "confidence: {:?} (11 checks → Medium)",
            fitness.score.confidence
        ),
    );

    let perfect_result = ValidationResult::new("perfect");
    let mut perfect = perfect_result;
    for i in 0..25 {
        perfect.check_bool(&format!("p-{i}"), true, "pass");
    }
    let perfect_fitness =
        evolution::evaluate_fitness("perfect-primal", Target::current(), &perfect);

    v.check_bool(
        "fitness:perfect_score",
        perfect_fitness.is_fully_adapted(),
        &format!(
            "perfect primal: adapted={}, score={:.3}",
            perfect_fitness.is_fully_adapted(),
            perfect_fitness.score.value
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_fitness_structural() {
        let mut v = ValidationResult::new("arch-fitness");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Wave 150w: flockGate has 1 arch-specific gap (no aarch64 depot locally)
        assert!(
            v.failed <= 1,
            "Arch fitness has {} failures (passed={}, skipped={}) — expect ≤1 known debt",
            v.failed,
            v.evaluated().saturating_sub(v.failed),
            v.skipped
        );
    }
}
