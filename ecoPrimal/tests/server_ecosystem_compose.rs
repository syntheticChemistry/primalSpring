// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    deprecated,
    reason = "integration test uses deprecated harness/launcher APIs"
)]
#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "integration tests — panics are the failure signal"
)]

//! Nest and node atomic integration tests. Run with `cargo test --ignored`.

#[expect(
    dead_code,
    reason = "shared helpers — each test file uses a different subset"
)]
mod integration;

use integration::direct_rpc_call;

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
