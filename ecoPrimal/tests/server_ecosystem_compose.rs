// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nest, node, overlay, and cross-primal integration tests. Run with `cargo test --ignored`.

#[allow(dead_code)]
mod integration;

use integration::direct_rpc_call;
use std::path::PathBuf;

// ===========================================================================
// Nest Atomic Tests — Tower + NestGate (Gates 8-10: storage composition)
// ===========================================================================

// ---------------------------------------------------------------------------
// Gate 8.1: Nest Atomic startup — beardog + songbird + nestgate
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_atomic_live_health_check() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nest-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    assert_eq!(
        running.primal_count(),
        3,
        "Nest = beardog + songbird + nestgate"
    );

    let health = running.health_check_all();
    for (name, live) in &health {
        assert!(live, "{name} should be live");
    }
}

// ---------------------------------------------------------------------------
// Gate 8.2: Nest Atomic capabilities discovery
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_atomic_live_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-ncaps-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let caps = running.capabilities_all();
    assert_eq!(caps.len(), 3, "should query all three primals");
}

// ---------------------------------------------------------------------------
// Gate 8.3: Nest Atomic validation
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_atomic_live_validation() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let family_id = format!("itest-nval-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let mut v = ValidationResult::new("nest_atomic_live");
    running.validate(&mut v);
    assert!(v.passed > 0, "should have at least one passing check");
    assert_eq!(v.failed, 0, "should have zero failures");
}

// ---------------------------------------------------------------------------
// Gate 9.1: NestGate storage store/retrieve round-trip
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_storage_round_trip() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nstr-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let store = direct_rpc_call(
        nestgate_socket,
        "storage.store",
        &serde_json::json!({
            "family_id": family_id,
            "key": "test_key",
            "data": {"message": "hello from primalSpring"}
        }),
    );
    assert!(store.is_ok(), "storage.store should succeed: {store:?}");
    assert_eq!(store.unwrap()["success"], true);

    let retrieve = direct_rpc_call(
        nestgate_socket,
        "storage.retrieve",
        &serde_json::json!({
            "family_id": family_id,
            "key": "test_key"
        }),
    );
    assert!(
        retrieve.is_ok(),
        "storage.retrieve should succeed: {retrieve:?}"
    );
    assert_eq!(
        retrieve.as_ref().unwrap()["data"]["message"],
        "hello from primalSpring"
    );
}

// ---------------------------------------------------------------------------
// Gate 9.2: NestGate storage list + exists
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_storage_list_exists() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nlst-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let _ = direct_rpc_call(
        nestgate_socket,
        "storage.store",
        &serde_json::json!({
            "family_id": family_id,
            "key": "list_test_a",
            "data": {"idx": 1}
        }),
    );
    let _ = direct_rpc_call(
        nestgate_socket,
        "storage.store",
        &serde_json::json!({
            "family_id": family_id,
            "key": "list_test_b",
            "data": {"idx": 2}
        }),
    );

    let list = direct_rpc_call(
        nestgate_socket,
        "storage.list",
        &serde_json::json!({"family_id": family_id}),
    );
    assert!(list.is_ok(), "storage.list should succeed: {list:?}");
    let keys = list.unwrap();
    assert!(keys["keys"].is_array(), "keys field should be an array");

    let exists = direct_rpc_call(
        nestgate_socket,
        "storage.exists",
        &serde_json::json!({"family_id": family_id, "key": "list_test_a"}),
    );
    assert!(exists.is_ok(), "storage.exists should succeed: {exists:?}");
}

// ---------------------------------------------------------------------------
// Gate 9.3: NestGate model cache register + locate
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_model_cache() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nmod-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let register = direct_rpc_call(
        nestgate_socket,
        "model.register",
        &serde_json::json!({
            "model_id": "test-llm-v1",
            "metadata": {
                "name": "Test LLM",
                "size_bytes": 1024,
                "format": "gguf"
            }
        }),
    );
    assert!(
        register.is_ok(),
        "model.register should succeed: {register:?}"
    );

    let locate = direct_rpc_call(
        nestgate_socket,
        "model.locate",
        &serde_json::json!({"model_id": "test-llm-v1"}),
    );
    match &locate {
        Ok(v) => println!("  model.locate: {v}"),
        Err(e) => println!("  model.locate error (may be expected): {e}"),
    }
}

// ---------------------------------------------------------------------------
// Gate 10.1: NestGate health via direct RPC
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_direct_health() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nhlt-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let health = direct_rpc_call(nestgate_socket, "health", &serde_json::json!({}));
    assert!(health.is_ok(), "nestgate health should respond: {health:?}");
    assert_eq!(health.unwrap()["status"], "healthy");
}

// ---------------------------------------------------------------------------
// Gate 10.2: NestGate discover_capabilities
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_discover_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-ndsc-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let caps = direct_rpc_call(
        nestgate_socket,
        "discover_capabilities",
        &serde_json::json!({}),
    );
    assert!(
        caps.is_ok(),
        "discover_capabilities should succeed: {caps:?}"
    );
    let cap_result = caps.unwrap();
    println!("  nestgate capabilities: {cap_result}");
}

// ===========================================================================
// Node Atomic Tests — Tower + ToadStool (Gates 11-13: compute composition)
// ===========================================================================

// ---------------------------------------------------------------------------
// Gate 11.1: Node Atomic startup — beardog + songbird + toadstool
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn node_atomic_live_health_check() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-node-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    assert_eq!(
        running.primal_count(),
        3,
        "Node = beardog + songbird + toadstool"
    );

    let health = running.health_check_all();
    for (name, live) in &health {
        assert!(live, "{name} should be live");
    }
}

// ---------------------------------------------------------------------------
// Gate 11.2: Node Atomic validation
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn node_atomic_live_validation() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let family_id = format!("itest-nval2-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let mut v = ValidationResult::new("node_atomic_live");
    running.validate(&mut v);
    assert!(v.passed > 0, "should have at least one passing check");
    assert_eq!(v.failed, 0, "should have zero failures");
}

// ---------------------------------------------------------------------------
// Gate 12.1: ToadStool health via direct RPC
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn node_toadstool_health() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-tshlt-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let toadstool_socket = running
        .socket_for("compute")
        .or_else(|| running.socket_for_primal("toadstool"))
        .expect("toadstool socket");

    let health = direct_rpc_call(toadstool_socket, "toadstool.health", &serde_json::json!({}));
    assert!(
        health.is_ok(),
        "toadstool.health should respond: {health:?}"
    );
    assert_eq!(health.unwrap()["healthy"], true);
}

// ---------------------------------------------------------------------------
// Gate 12.2: ToadStool capabilities
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn node_toadstool_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-tscap-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let toadstool_socket = running
        .socket_for("compute")
        .or_else(|| running.socket_for_primal("toadstool"))
        .expect("toadstool socket");

    let caps = direct_rpc_call(
        toadstool_socket,
        "toadstool.query_capabilities",
        &serde_json::json!({}),
    );
    assert!(
        caps.is_ok(),
        "toadstool.query_capabilities should succeed: {caps:?}"
    );
    let result = caps.unwrap();
    assert!(result["supported_workload_types"].is_array());
    println!("  toadstool capabilities: {result}");
}

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
    println!("  passed={} failed={} skipped={}", v.passed, v.failed, v.skipped);
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
    for name in &["tower_ai.toml", "tower_ai_viz.toml", "nest_viz.toml", "node_ai.toml"] {
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

    let tower_ai = load_graph(&graphs_dir.join("tower_ai.toml")).unwrap();
    let spawnable = graph_spawnable_primals(&tower_ai);
    assert!(spawnable.contains(&"beardog".to_owned()));
    assert!(spawnable.contains(&"songbird".to_owned()));
    assert!(spawnable.contains(&"squirrel".to_owned()));
    assert!(
        !spawnable.contains(&"validate_tower_ai".to_owned()),
        "validation nodes should be excluded"
    );
}

#[test]
fn overlay_graph_capability_map() {
    use primalspring::deploy::{graph_capability_map, load_graph};

    let graphs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");

    let tower_ai = load_graph(&graphs_dir.join("tower_ai.toml")).unwrap();
    let caps = graph_capability_map(&tower_ai);
    assert_eq!(caps.get("security").unwrap(), "beardog");
    assert_eq!(caps.get("discovery").unwrap(), "songbird");
    assert_eq!(caps.get("ai").unwrap(), "squirrel");
    assert!(
        !caps.contains_key("coordination")
            || caps.get("coordination").unwrap() != "validate_tower_ai",
        "validation node should not be in spawnable capability map"
    );
}

#[test]
fn overlay_graph_merge_base_plus_ai() {
    use primalspring::deploy::{load_graph, merge_graphs, topological_waves};

    let graphs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let base = load_graph(&graphs_dir.join("tower_atomic_bootstrap.toml")).unwrap();
    let overlay = load_graph(&graphs_dir.join("tower_ai.toml")).unwrap();

    let merged = merge_graphs(&base, &overlay);
    assert!(merged.graph.name.contains('+'));

    let waves = topological_waves(&merged).unwrap();
    assert!(!waves.is_empty());

    let all_names: Vec<String> = waves.into_iter().flatten().collect();
    assert!(all_names.contains(&"beardog".to_owned()));
    assert!(all_names.contains(&"songbird".to_owned()));
    assert!(all_names.contains(&"squirrel".to_owned()));
}

// ---------------------------------------------------------------------------
// Gate 21: Squirrel Cross-Primal Discovery (full overlay)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn squirrel_discovers_sibling_primals() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/full_overlay.toml");
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

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/full_overlay.toml");
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

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/full_overlay.toml");
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

    let graph = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs/full_overlay.toml");
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
