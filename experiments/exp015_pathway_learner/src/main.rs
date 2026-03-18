// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp015: Pathway Learner — validates all five coordination patterns have descriptions (exp010–014 with metrics).

use primalspring::graphs::CoordinationPattern;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp015 — Pathway Learner");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp015: Pathway Learner (exp010–014 with metrics)");
    println!("{}", "=".repeat(72));

    let patterns = [
        CoordinationPattern::Sequential,
        CoordinationPattern::Parallel,
        CoordinationPattern::ConditionalDag,
        CoordinationPattern::Pipeline,
        CoordinationPattern::Continuous,
    ];
    for (i, p) in patterns.iter().enumerate() {
        let desc = p.description();
        v.check_bool(
            &format!("pattern_{i}_description_non_empty"),
            !desc.is_empty(),
            &format!("{p:?} has non-empty description: {desc}"),
        );
    }

    v.check_skip(
        "actual_pathway_learning",
        "actual pathway learning needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
