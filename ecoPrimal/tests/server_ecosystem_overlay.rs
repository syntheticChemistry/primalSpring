// SPDX-License-Identifier: AGPL-3.0-or-later
#![expect(clippy::unwrap_used, clippy::expect_used, reason = "integration tests — panics are the failure signal")]

//! Graph-driven overlay composition tests. Run with `cargo test --ignored`.

#[expect(
    dead_code,
    reason = "shared helpers — each test file uses a different subset"
)]
mod integration;

use std::path::PathBuf;

// ===========================================================================
// Graph-Driven Overlay Composition Tests
// ===========================================================================
//
// These tests validate the graph-driven overlay model: tier-independent
// primals (Squirrel, petalTongue) composed with any base tier via deploy
// graphs, rather than fixed AtomicType enum gating.

// ---------------------------------------------------------------------------
// Gate 17: Tower + AI Overlay (beardog + songbird + squirrel via graph)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_tower_ai_spawn_order() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/tower_ai.toml");
    let family_id = format!("itest-tai-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("tower + AI overlay should start");

    assert!(
        running.primal_count() >= 3,
        "Tower AI = beardog + songbird + squirrel (got {})",
        running.primal_count()
    );

    let health = running.health_check_all();
    let live_count = health.iter().filter(|(_, live)| *live).count();
    assert!(live_count >= 2, "at least base tier should be live");

    for (name, live) in &health {
        println!("  {name}: live={live}");
    }
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_tower_ai_capability_routing() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/tower_ai.toml");
    let family_id = format!("itest-taicap-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("tower + AI overlay should start");

    assert!(
        running.socket_for("security").is_some(),
        "base tier security capability should resolve"
    );
    assert!(
        running.socket_for("discovery").is_some(),
        "base tier discovery capability should resolve"
    );
    assert!(
        running.socket_for("ai").is_some(),
        "overlay AI capability should resolve via graph"
    );

    let all_caps = running.all_capabilities();
    assert!(all_caps.contains(&"security".to_owned()));
    assert!(all_caps.contains(&"discovery".to_owned()));
    assert!(all_caps.contains(&"ai".to_owned()));

    let overlay = running.overlay_primals();
    assert!(
        overlay.contains(&"squirrel".to_owned()),
        "squirrel should be an overlay primal"
    );
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_tower_ai_validation() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/tower_ai.toml");
    let family_id = format!("itest-taival-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("tower + AI overlay should start");

    let mut v = ValidationResult::new("tower_ai_overlay");
    running.validate(&mut v);
    assert!(
        v.passed >= 2,
        "at least base tier checks should pass (got {})",
        v.passed
    );
    println!(
        "  passed={} failed={} skipped={}",
        v.passed, v.failed, v.skipped
    );
}

// ---------------------------------------------------------------------------
// Gate 18: Nest + Visualization Overlay (nest + petaltongue via graph)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_nest_viz_spawn_order() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/nest_viz.toml");
    let family_id = format!("itest-nviz-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Nest, &graph)
        .start(&family_id)
        .expect("nest + viz overlay should start");

    assert!(
        running.primal_count() >= 4,
        "Nest Viz = beardog + songbird + nestgate + petaltongue (got {})",
        running.primal_count()
    );

    let all_caps = running.all_capabilities();
    assert!(all_caps.contains(&"storage".to_owned()));
    assert!(all_caps.contains(&"visualization".to_owned()));

    let overlay = running.overlay_primals();
    assert!(
        overlay.contains(&"petaltongue".to_owned()),
        "petaltongue should be an overlay primal"
    );
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_nest_viz_validation() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/nest_viz.toml");
    let family_id = format!("itest-nvval-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Nest, &graph)
        .start(&family_id)
        .expect("nest + viz overlay should start");

    let mut v = ValidationResult::new("nest_viz_overlay");
    running.validate(&mut v);
    assert!(v.passed >= 2, "at least base tier checks should pass");
}

// ---------------------------------------------------------------------------
// Gate 19: Node + AI Overlay (node + squirrel via graph)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_node_ai_spawn_order() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/node_ai.toml");
    let family_id = format!("itest-nai-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Node, &graph)
        .start(&family_id)
        .expect("node + AI overlay should start");

    assert!(
        running.primal_count() >= 4,
        "Node AI = beardog + songbird + toadstool + squirrel (got {})",
        running.primal_count()
    );

    let all_caps = running.all_capabilities();
    assert!(all_caps.contains(&"compute".to_owned()));
    assert!(all_caps.contains(&"ai".to_owned()));

    let overlay = running.overlay_primals();
    assert!(
        overlay.contains(&"squirrel".to_owned()),
        "squirrel should be an overlay primal"
    );
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn overlay_node_ai_validation() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/node_ai.toml");
    let family_id = format!("itest-naival-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Node, &graph)
        .start(&family_id)
        .expect("node + AI overlay should start");

    let mut v = ValidationResult::new("node_ai_overlay");
    running.validate(&mut v);
    assert!(v.passed >= 2, "at least base tier checks should pass");
}

// ---------------------------------------------------------------------------
// Graph merge/compose tests (unit-level, no binaries needed)
// ---------------------------------------------------------------------------

#[test]
fn overlay_graph_structural_validation() {
    use primalspring::deploy::{load_graph, topological_waves, validate_structure};

    let graphs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    for name in &[
        "nucleus_complete.toml",
        "provenance_overlay.toml",
        "spring_byob_template.toml",
    ] {
        let path = graphs_dir.join(name);
        let v = validate_structure(&path);
        assert!(v.parsed, "{name} should parse");
        assert!(v.issues.is_empty(), "{name} issues: {:?}", v.issues);

        let graph = load_graph(&path).unwrap();
        let waves = topological_waves(&graph).unwrap();
        assert!(waves.len() >= 2, "{name} should have >= 2 waves");
    }
}

#[test]
fn overlay_graph_spawn_filter() {
    use primalspring::deploy::{graph_spawnable_primals, load_graph};

    let graphs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");

    let nucleus = load_graph(&graphs_dir.join("nucleus_complete.toml")).unwrap();
    let spawnable = graph_spawnable_primals(&nucleus);
    assert!(spawnable.contains(&"beardog".to_owned()));
    assert!(spawnable.contains(&"songbird".to_owned()));
}

#[test]
fn overlay_graph_capability_map() {
    use primalspring::deploy::{graph_capability_map, load_graph};

    let graphs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");

    let nucleus = load_graph(&graphs_dir.join("nucleus_complete.toml")).unwrap();
    let caps = graph_capability_map(&nucleus);
    assert!(caps.get("security").unwrap() == "beardog");
    assert!(caps.get("discovery").unwrap() == "songbird");
}

#[test]
fn overlay_graph_merge_base_plus_ai() {
    use primalspring::deploy::{load_graph, merge_graphs, topological_waves};

    let graphs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let base = load_graph(&graphs_dir.join("nucleus_complete.toml")).unwrap();
    let overlay = load_graph(&graphs_dir.join("provenance_overlay.toml")).unwrap();

    let merged = merge_graphs(&base, &overlay);
    assert!(merged.graph.name.contains('+'));

    let waves = topological_waves(&merged).unwrap();
    assert!(!waves.is_empty());

    let all_names: Vec<String> = waves.into_iter().flatten().collect();
    assert!(all_names.contains(&"beardog".to_owned()));
    assert!(all_names.contains(&"songbird".to_owned()));
}

// ---------------------------------------------------------------------------
// Gate 21: Squirrel Cross-Primal Discovery (full overlay)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn squirrel_discovers_sibling_primals() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/full.toml");
    let family_id = format!("itest-sqdsc-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("full overlay should start");

    assert!(
        running.primal_count() >= 3,
        "full overlay should have 3+ primals (got {})",
        running.primal_count()
    );

    if let Some(mut client) = running.client_for("ai") {
        let resp = client
            .call("capability.discover", serde_json::json!({}))
            .expect("capability.discover call should not fail at IPC level");
        assert!(
            resp.is_success(),
            "Squirrel capability.discover should succeed: {:?}",
            resp.error
        );
    }
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn squirrel_tool_list_aggregates() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/full.toml");
    let family_id = format!("itest-sqtl-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("full overlay should start");

    if let Some(mut client) = running.client_for("ai") {
        let resp = client
            .call("tool.list", serde_json::json!({}))
            .expect("tool.list call should not fail at IPC level");
        assert!(
            resp.is_success(),
            "Squirrel tool.list should succeed: {:?}",
            resp.error
        );
    }
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn squirrel_context_create() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/full.toml");
    let family_id = format!("itest-sqctx-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("full overlay should start");

    if let Some(mut client) = running.client_for("ai") {
        let resp = client
            .call(
                "context.create",
                serde_json::json!({
                    "name": "itest-context",
                    "description": "integration test context"
                }),
            )
            .expect("context.create call should not fail at IPC level");
        assert!(
            resp.is_success(),
            "Squirrel context.create should succeed: {:?}",
            resp.error
        );
    }
}

#[test]
#[ignore = "requires plasmidBin binaries + API key (run with --ignored)"]
fn squirrel_ai_query_routes_via_songbird() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    if std::env::var("ANTHROPIC_API_KEY").is_err() && std::env::var("OPENAI_API_KEY").is_err() {
        eprintln!("skipping: no API key set");
        return;
    }

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/profiles/full.toml");
    let family_id = format!("itest-sqai-{}", std::process::id());
    let running = AtomicHarness::with_graph(AtomicType::Tower, &graph)
        .start(&family_id)
        .expect("full overlay should start");

    if let Some(mut client) = running.client_for("ai") {
        let resp = client
            .call(
                "ai.query",
                serde_json::json!({
                    "prompt": "What is 2+2? Reply with just the number.",
                    "max_tokens": 16
                }),
            )
            .expect("ai.query call should not fail at IPC level");
        assert!(
            resp.is_success(),
            "Squirrel ai.query should succeed: {:?}",
            resp.error
        );
    }
}
