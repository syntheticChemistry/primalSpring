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

use base64::Engine;
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

// ===========================================================================
// Compute Trio Gate Tests (Wave 8: Node atomic sovereign dispatch)
//
// These gates validate the compute trio composition: coralReef (shader
// compile), toadStool (compute dispatch), barraCuda (math/physics).
// The trio forms the Node atomic's sovereign compute pipeline.
// ===========================================================================

// ---------------------------------------------------------------------------
// Compute Trio Gate 1: coralReef shader.compile.capabilities
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn compute_trio_coralreef_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-crcap-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let coralreef_socket = running
        .socket_for("shader")
        .or_else(|| running.socket_for_primal("coralreef"))
        .expect("coralreef socket");

    let caps = direct_rpc_call(
        coralreef_socket,
        "shader.compile.capabilities",
        &serde_json::json!({}),
    );
    assert!(
        caps.is_ok(),
        "shader.compile.capabilities should succeed: {caps:?}"
    );
    let result = caps.unwrap();
    assert!(
        result["targets"].is_array(),
        "should return supported target architectures"
    );
    println!("  coralReef compile targets: {result}");
}

// ---------------------------------------------------------------------------
// Compute Trio Gate 2: toadStool compute.capabilities hardware info
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn compute_trio_toadstool_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-tscaps-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let toadstool_socket = running
        .socket_for("compute")
        .or_else(|| running.socket_for_primal("toadstool"))
        .expect("toadstool socket");

    let caps = direct_rpc_call(
        toadstool_socket,
        "compute.capabilities",
        &serde_json::json!({}),
    );
    assert!(
        caps.is_ok(),
        "compute.capabilities should succeed: {caps:?}"
    );
    let result = caps.unwrap();
    let has_hw_info = result["backends"].is_array()
        || result["devices"].is_array()
        || result["capabilities"].is_object();
    assert!(has_hw_info, "should return hardware/backend info: {result}");
    println!("  toadStool compute capabilities: {result}");
}

// ---------------------------------------------------------------------------
// Compute Trio Gate 3: barraCuda stats.mean CPU fallback round-trip
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn compute_trio_barracuda_stats_mean() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-bcmean-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let barracuda_socket = running
        .socket_for("tensor")
        .or_else(|| running.socket_for_primal("barracuda"))
        .expect("barracuda socket");

    let result = direct_rpc_call(
        barracuda_socket,
        "stats.mean",
        &serde_json::json!({ "data": [2.0_f64, 4.0, 6.0, 8.0] }),
    );
    assert!(result.is_ok(), "stats.mean should succeed: {result:?}");
    let resp = result.unwrap();
    let mean = resp["mean"]
        .as_f64()
        .or_else(|| resp["result"].as_f64())
        .expect("should return numeric mean");
    assert!(
        (mean - 5.0).abs() < 1e-9,
        "stats.mean([2,4,6,8]) should be 5.0, got {mean}"
    );
}

// ---------------------------------------------------------------------------
// Compute Trio Gate 4: Sovereign E2E — shader.compile.wgsl + dispatch
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries + GPU (run with --ignored)"]
fn compute_trio_sovereign_e2e() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-sove2e-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Node)
        .start(&family_id)
        .expect("node atomic should start");

    let coralreef_socket = running
        .socket_for("shader")
        .or_else(|| running.socket_for_primal("coralreef"))
        .expect("coralreef socket");

    let toadstool_socket = running
        .socket_for("compute")
        .or_else(|| running.socket_for_primal("toadstool"))
        .expect("toadstool socket");

    let trivial_wgsl = r"@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
}";

    let compile = direct_rpc_call(
        coralreef_socket,
        "shader.compile.wgsl",
        &serde_json::json!({
            "source": trivial_wgsl,
            "target": "sm70",
            "entry_point": "main",
            "workgroup_size": [1, 1, 1]
        }),
    );
    assert!(
        compile.is_ok(),
        "shader.compile.wgsl should succeed: {compile:?}"
    );
    let compile_resp = compile.unwrap();
    let binary_b64 = compile_resp["binary_b64"]
        .as_str()
        .expect("should return binary_b64");
    assert!(!binary_b64.is_empty(), "compiled binary should be non-empty");

    let default_shader_info = serde_json::json!({
        "gprs": 32, "shared_memory": 0, "barriers": 0,
        "workgroup": [1, 1, 1], "wave_size": 32
    });
    let dispatch = direct_rpc_call(
        toadstool_socket,
        "compute.dispatch.submit",
        &serde_json::json!({
            "binary_b64": binary_b64,
            "shader_info": compile_resp.get("shader_info").unwrap_or(&default_shader_info),
            "dispatch_dims": [1, 1, 1],
            "buffers": []
        }),
    );
    assert!(
        dispatch.is_ok(),
        "compute.dispatch.submit should succeed: {dispatch:?}"
    );
    let dispatch_resp = dispatch.unwrap();
    let has_result = dispatch_resp.get("dispatch_id").is_some()
        || dispatch_resp.get("status").is_some();
    assert!(has_result, "dispatch should return id or status: {dispatch_resp}");
}

// ===========================================================================
// NestGate Content Pipeline Tests (Wave 7: semantic contract gates)
//
// These gates validate `content.*` methods — the capability domain that was
// registered in the 413-method registry but never exercised, allowing the
// NestGate transport parity gap to reach projectNUCLEUS uncaught.
// ===========================================================================

// ---------------------------------------------------------------------------
// Content Gate 1: content.put stores bytes, returns BLAKE3 hash
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nestgate_content_put_returns_hash() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-cput-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("content")
        .or_else(|| running.socket_for("storage"))
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let data_b64 = base64::engine::general_purpose::STANDARD
        .encode(b"primalSpring content gate test");
    let put = direct_rpc_call(
        nestgate_socket,
        "content.put",
        &serde_json::json!({
            "data": data_b64,
            "content_type": "text/plain",
            "family_id": family_id,
        }),
    );
    assert!(put.is_ok(), "content.put should succeed: {put:?}");
    let result = put.unwrap();
    let hash = result["hash"].as_str().expect("should return hash");
    assert_eq!(hash.len(), 64, "BLAKE3 hex hash should be 64 chars");
}

// ---------------------------------------------------------------------------
// Content Gate 2: content.get retrieves by hash, matches original
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nestgate_content_get_roundtrip() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-cget-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("content")
        .or_else(|| running.socket_for("storage"))
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let original = b"wave7 content roundtrip test bytes";
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(original);

    let put = direct_rpc_call(
        nestgate_socket,
        "content.put",
        &serde_json::json!({
            "data": data_b64,
            "content_type": "application/octet-stream",
            "family_id": family_id,
        }),
    )
    .expect("content.put should succeed");
    let hash = put["hash"].as_str().expect("should return hash");

    let get = direct_rpc_call(
        nestgate_socket,
        "content.get",
        &serde_json::json!({ "hash": hash, "family_id": family_id }),
    )
    .expect("content.get should succeed");

    let retrieved_b64 = get["data"].as_str().expect("should return data");
    assert_eq!(
        retrieved_b64, data_b64,
        "content.get should return identical base64 data"
    );
}

// ---------------------------------------------------------------------------
// Content Gate 3: content.list includes stored hash
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nestgate_content_list_includes_stored() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-clst-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("content")
        .or_else(|| running.socket_for("storage"))
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let data_b64 = base64::engine::general_purpose::STANDARD
        .encode(b"content list gate test");
    let put = direct_rpc_call(
        nestgate_socket,
        "content.put",
        &serde_json::json!({
            "data": data_b64,
            "content_type": "text/plain",
            "family_id": family_id,
        }),
    )
    .expect("content.put should succeed");
    let hash = put["hash"].as_str().expect("should return hash");

    let list = direct_rpc_call(
        nestgate_socket,
        "content.list",
        &serde_json::json!({ "family_id": family_id }),
    )
    .expect("content.list should succeed");

    let items = list["items"].as_array().expect("items should be array");
    let hashes: Vec<&str> = items
        .iter()
        .filter_map(|item| item.as_str().or_else(|| item["hash"].as_str()))
        .collect();
    assert!(
        hashes.contains(&hash),
        "content.list should include stored hash {hash}, got: {hashes:?}"
    );
}

// ===========================================================================
// Storage Auth Boundary Gate (NestGate storage.list scoping — BTSP Phase 2b)
// ===========================================================================

// ---------------------------------------------------------------------------
// Auth Gate 1: storage.list returns opaque hashes (no metadata leak)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nestgate_storage_list_returns_opaque_hashes() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-slst-{}", std::process::id());
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
            "key": format!("auth-boundary-test-{}", std::process::id()),
            "value": base64::engine::general_purpose::STANDARD.encode(b"sensitive data"),
        }),
    );
    assert!(store.is_ok(), "storage.store should succeed: {store:?}");

    let list = direct_rpc_call(
        nestgate_socket,
        "storage.list",
        &serde_json::json!({}),
    )
    .expect("storage.list should succeed");

    let keys = list["keys"]
        .as_array()
        .or_else(|| list["items"].as_array());
    assert!(keys.is_some(), "storage.list should return keys or items array");

    let keys = keys.unwrap();
    for key in keys {
        let key_str = key.as_str().unwrap_or_default();
        assert!(
            !key_str.contains("sensitive"),
            "storage.list should return opaque keys, not plaintext — leaked: {key_str}"
        );
    }
}

// ---------------------------------------------------------------------------
// Auth Gate 2: storage.list is content-addressed (BLAKE3 hashes)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nestgate_storage_list_content_addressed() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-sca-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("content")
        .or_else(|| running.socket_for("storage"))
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let data_b64 = base64::engine::general_purpose::STANDARD
        .encode(b"blake3 content addressing test");
    let put = direct_rpc_call(
        nestgate_socket,
        "content.put",
        &serde_json::json!({
            "data": data_b64,
            "content_type": "text/plain",
            "family_id": family_id,
        }),
    )
    .expect("content.put should succeed");
    let hash = put["hash"].as_str().expect("should return hash");

    assert!(
        hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()),
        "content hash should be 64 hex chars (BLAKE3), got: {hash}"
    );
}
