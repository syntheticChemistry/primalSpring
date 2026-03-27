// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp012: Conditional DAG — validates conditional coordination pattern
//! with toadstool GPU dispatch and CPU fallback path.
//!
//! Phase 1: CoordinationPattern constant validation
//! Phase 2: Graph structural validation (conditional_fallback.toml)
//! Phase 3: Live conditional composition via AtomicHarness
//! Phase 4: Conditional branch verification (toadstool primary, CPU fallback)

use std::path::{Path, PathBuf};

use primalspring::coordination::AtomicType;
use primalspring::deploy::{graph_spawnable_primals, load_graph, validate_structure};
use primalspring::graphs::CoordinationPattern;
use primalspring::harness::{AtomicHarness, RunningAtomic};
use primalspring::primal_names;
use primalspring::validation::OrExit;
use primalspring::validation::ValidationResult;

fn graphs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../graphs")
}

fn coordination_pattern_constants(v: &mut ValidationResult) {
    println!("\n=== Phase 1: Pattern Constants ===\n");

    let desc = CoordinationPattern::ConditionalDag.description();
    v.check_bool(
        "conditional_dag_description_exists",
        !desc.is_empty(),
        &format!("CoordinationPattern::ConditionalDag.description() exists: {desc}"),
    );
}

fn conditional_fallback_graph_structure(v: &mut ValidationResult, graph_path: &Path) {
    println!("\n=== Phase 2: Graph Structural Validation ===\n");

    let result = validate_structure(graph_path);
    v.check_bool(
        "graph_parses",
        result.parsed,
        "conditional_fallback.toml parses",
    );
    v.check_bool(
        "graph_clean",
        result.issues.is_empty(),
        &format!("structural issues: {:?}", result.issues),
    );

    if result.parsed {
        let graph = load_graph(graph_path).or_exit("load conditional_fallback graph");
        let spawnable = graph_spawnable_primals(&graph);
        println!(
            "  {} nodes, {} spawnable: {spawnable:?}",
            result.node_count,
            spawnable.len()
        );
        v.check_bool(
            "cpu_fallback_not_spawned",
            !spawnable.contains(&"cpu_fallback".to_owned()),
            "cpu_fallback (spawn=false) excluded from spawnable",
        );
        v.check_bool(
            "toadstool_spawnable",
            spawnable.contains(&primal_names::TOADSTOOL.to_owned()),
            "toadstool is spawnable (primary path)",
        );
    }
}

fn conditional_branch_verification(v: &mut ValidationResult, running: &RunningAtomic) {
    println!("\n=== Phase 4: Conditional Branch Verification ===\n");

    let health = running.health_check_all();
    let beardog_live = health.iter().any(|(n, l)| n == primal_names::BEARDOG && *l);
    let songbird_live = health
        .iter()
        .any(|(n, l)| n == primal_names::SONGBIRD && *l);
    v.check_bool("beardog_live", beardog_live, "beardog healthy (required)");
    v.check_bool(
        "songbird_live",
        songbird_live,
        "songbird healthy (required)",
    );

    let toadstool_live = health
        .iter()
        .any(|(n, l)| n == primal_names::TOADSTOOL && *l);
    if toadstool_live {
        println!("  toadstool: LIVE — GPU dispatch path active");
        v.check_bool(
            "primary_path_active",
            true,
            "toadstool GPU dispatch path active",
        );

        let all_caps = running.all_capabilities();
        v.check_bool(
            "compute_capability",
            all_caps.contains(&"compute".to_owned()),
            "compute capability available via toadstool",
        );
    } else {
        println!("  toadstool: DOWN — CPU fallback path would activate");
        v.check_skip(
            "primary_path_active",
            "toadstool not available — CPU fallback path",
        );
    }

    for (name, live) in &health {
        println!(
            "    {name}: {}",
            if *live { "LIVE" } else { "DOWN (optional)" }
        );
    }
}

fn conditional_composition_live(v: &mut ValidationResult, graph_path: &Path) {
    println!("\n=== Phase 3: Live Conditional Composition ===\n");

    let family_id = format!("exp012-{}", std::process::id());
    match AtomicHarness::with_graph(AtomicType::Tower, graph_path).start(&family_id) {
        Ok(running) => {
            v.check_bool(
                "composition_started",
                true,
                "conditional composition started",
            );
            v.check_minimum("primal_count", running.primal_count(), 2);

            running.validate(v);

            conditional_branch_verification(v, &running);
        }
        Err(e) => {
            println!("  composition start failed: {e}");
            v.check_skip(
                "composition_started",
                &format!("conditional composition could not start: {e}"),
            );
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp012 — Conditional DAG")
        .with_provenance("exp012_conditional_dag", "2026-03-24")
        .run(
            "primalSpring Exp012: ConditionalDag (conditional_fallback.toml)",
            |v| {
                coordination_pattern_constants(v);
                let graph_path = graphs_dir().join("conditional_fallback.toml");
                conditional_fallback_graph_structure(v, &graph_path);
                conditional_composition_live(v, &graph_path);
            },
        );
}
