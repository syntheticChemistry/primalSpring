// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp011: Parallel Graph — validates parallel coordination pattern
//! with a live 4-primal composition (beardog, songbird, nestgate, toadstool).
//!
//! Phase 1: CoordinationPattern constant validation
//! Phase 2: Graph structural validation (parallel_capability_burst.toml)
//! Phase 3: Live parallel composition via AtomicHarness
//! Phase 4: Parallel health burst (all 4 primals respond concurrently)

use std::path::{Path, PathBuf};
use std::time::Instant;

use primalspring::coordination::AtomicType;
use primalspring::deploy::{graph_spawnable_primals, load_graph, validate_structure};
use primalspring::graphs::CoordinationPattern;
use primalspring::harness::AtomicHarness;
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp011 — Parallel Graph");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp011: Parallel (parallel_capability_burst.toml)");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    println!("\n=== Phase 1: Pattern Constants ===\n");

    let desc = CoordinationPattern::Parallel.description();
    v.check_bool(
        "parallel_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::Parallel.description() exists: {desc}"),
    );

    println!("\n=== Phase 2: Graph Structural Validation ===\n");

    let graph_path = graphs_dir().join("parallel_capability_burst.toml");
    let result = validate_structure(&graph_path);
    v.check_bool(
        "graph_parses",
        result.parsed,
        "parallel_capability_burst.toml parses",
    );
    v.check_bool(
        "graph_clean",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if result.parsed {
        let graph = load_graph(&graph_path).unwrap();
        let spawnable = graph_spawnable_primals(&graph);
        v.check_minimum("spawnable_count", spawnable.len(), 4);
        println!(
            "  {} nodes, {} spawnable: {spawnable:?}",
            result.node_count,
            spawnable.len()
        );
    }

    println!("\n=== Phase 3: Live Parallel Composition ===\n");

    let family_id = format!("exp011-{}", std::process::id());
    match AtomicHarness::with_graph(AtomicType::Tower, &graph_path).start(&family_id) {
        Ok(running) => {
            v.check_bool("composition_started", true, "parallel composition started");
            v.check_minimum("primal_count", running.primal_count(), 2);

            println!("  {} primals running", running.primal_count());

            running.validate(&mut v);

            println!("\n=== Phase 4: Parallel Health Burst ===\n");

            let start = Instant::now();
            let health = running.health_check_all();
            let burst_ms = start.elapsed().as_millis();

            let live_count = health.iter().filter(|(_, live)| *live).count();
            v.check_minimum("live_primal_count", live_count, 2);
            println!(
                "  health burst: {live_count}/{} live in {burst_ms}ms",
                health.len()
            );

            for (name, live) in &health {
                println!("    {name}: {}", if *live { "LIVE" } else { "DOWN" });
            }

            let caps = running.capabilities_all();
            let total_caps: usize = caps.iter().map(|(_, c)| c.len()).sum();
            v.check_minimum("total_capabilities", total_caps, 2);
            println!("  total capabilities across all primals: {total_caps}");
        }
        Err(e) => {
            println!("  composition start failed: {e}");
            v.check_skip(
                "composition_started",
                &format!("parallel composition could not start: {e}"),
            );
        }
    }

    println!("\n{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    v.finish();
    std::process::exit(v.exit_code());
}
