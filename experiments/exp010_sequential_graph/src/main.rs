// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp010: Sequential Graph — validates sequential coordination pattern
//! with a live Tower Atomic composition (beardog -> songbird).
//!
//! Phase 1: CoordinationPattern constant validation
//! Phase 2: Graph structural validation (tower_atomic_bootstrap.toml)
//! Phase 3: Live sequential composition via AtomicHarness
//! Phase 4: Sequential ordering verification (beardog before songbird)

use std::path::{Path, PathBuf};

use primalspring::coordination::AtomicType;
use primalspring::deploy::{load_graph, topological_waves, validate_structure};
use primalspring::graphs::CoordinationPattern;
use primalspring::harness::AtomicHarness;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp010 — Sequential Graph");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp010: Sequential (tower_atomic_bootstrap.toml)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    println!("\n=== Phase 1: Pattern Constants ===\n");

    let desc = CoordinationPattern::Sequential.description();
    v.check_bool(
        "sequential_description_non_empty",
        !desc.is_empty(),
        &format!("CoordinationPattern::Sequential.description() is non-empty: {desc}"),
    );
    let expected = "Nodes in dependency order (A -> B -> C)";
    v.check_bool(
        "sequential_description_matches",
        desc == expected,
        &format!("sequential pattern matches expected: {expected}"),
    );

    println!("\n=== Phase 2: Graph Structural Validation ===\n");

    let graph_path = graphs_dir().join("tower_atomic_bootstrap.toml");
    let result = validate_structure(&graph_path);
    v.check_bool(
        "graph_parses",
        result.parsed,
        "tower_atomic_bootstrap.toml parses",
    );
    v.check_bool(
        "graph_clean",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if result.parsed {
        let graph = load_graph(&graph_path).unwrap();
        let waves = topological_waves(&graph).unwrap();
        v.check_minimum("topological_waves", waves.len(), 2);
        println!(
            "  {} nodes, {} waves (sequential ordering verified)",
            result.node_count,
            waves.len()
        );

        v.check_bool(
            "beardog_before_songbird",
            waves.len() >= 2
                && waves[0].contains(&"beardog".to_owned())
                && waves.iter().skip(1).any(|w| w.contains(&"songbird".to_owned())),
            "beardog is in wave 0, songbird in a later wave",
        );
    }

    println!("\n=== Phase 3: Live Sequential Composition ===\n");

    let family_id = format!("exp010-{}", std::process::id());
    match AtomicHarness::with_graph(AtomicType::Tower, &graph_path).start(&family_id) {
        Ok(running) => {
            v.check_bool("composition_started", true, "sequential composition started");
            v.check_minimum("primal_count", running.primal_count(), 2);

            running.validate(&mut v);

            println!("\n=== Phase 4: Sequential Ordering Verification ===\n");

            let health = running.health_check_all();
            let all_live = health.iter().all(|(_, live)| *live);
            v.check_bool(
                "all_primals_live",
                all_live,
                &format!("all primals healthy: {health:?}"),
            );

            let caps = running.capabilities_all();
            let beardog_caps = caps.iter().find(|(n, _)| n == "beardog");
            let songbird_caps = caps.iter().find(|(n, _)| n == "songbird");
            v.check_bool(
                "beardog_has_capabilities",
                beardog_caps.map_or(false, |(_, c)| !c.is_empty()),
                "beardog reports capabilities",
            );
            v.check_bool(
                "songbird_has_capabilities",
                songbird_caps.map_or(false, |(_, c)| !c.is_empty()),
                "songbird reports capabilities",
            );
        }
        Err(e) => {
            println!("  composition start failed: {e}");
            v.check_skip(
                "composition_started",
                &format!("sequential composition could not start: {e}"),
            );
        }
    }

    println!("\n{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    v.finish();
    std::process::exit(v.exit_code());
}
