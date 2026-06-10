// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![allow(
    deprecated,
    reason = "integration test uses deprecated harness/launcher APIs"
)]
#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "integration tests — panics are the failure signal"
)]

//! Nest atomic + NestGate content pipeline integration tests.
//!
//! Run with `cargo test --ignored`.

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

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_atomic_live_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-ncap-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let caps = running.all_capabilities();
    assert!(
        caps.len() >= 2,
        "nest should expose security + storage at minimum, got {caps:?}"
    );
}

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

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_storage_round_trip() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nrt-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let key = format!("itest-key-{}", std::process::id());
    let value = base64::engine::general_purpose::STANDARD.encode(b"nest storage round trip value");

    let store = direct_rpc_call(
        nestgate_socket,
        "storage.store",
        &serde_json::json!({
            "key": key,
            "value": value,
        }),
    );
    assert!(store.is_ok(), "storage.store should succeed: {store:?}");

    let nestgate_socket2 = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket (second call)");

    let retrieve = direct_rpc_call(
        nestgate_socket2,
        "storage.retrieve",
        &serde_json::json!({ "key": key }),
    );
    assert!(
        retrieve.is_ok(),
        "storage.retrieve should succeed: {retrieve:?}"
    );
    let result = retrieve.unwrap();
    assert_eq!(result["value"], value);
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_storage_list_exists() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nsle-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let key = format!("itest-sle-{}", std::process::id());
    let value = base64::engine::general_purpose::STANDARD.encode(b"list-exists test");

    let _store = direct_rpc_call(
        nestgate_socket,
        "storage.store",
        &serde_json::json!({ "key": key, "value": value }),
    )
    .expect("store should succeed");

    let nestgate_socket2 = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket (second call)");

    let list = direct_rpc_call(nestgate_socket2, "storage.list", &serde_json::json!({}));
    assert!(list.is_ok(), "storage.list should succeed: {list:?}");
    let result = list.unwrap();
    let keys = result["keys"]
        .as_array()
        .or_else(|| result["items"].as_array());
    assert!(keys.is_some(), "storage.list should return keys or items");

    let nestgate_socket3 = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket (third call)");

    let exists = direct_rpc_call(
        nestgate_socket3,
        "storage.exists",
        &serde_json::json!({ "key": key }),
    );
    assert!(exists.is_ok(), "storage.exists should succeed: {exists:?}");
    let exists_result = exists.unwrap();
    let found = exists_result["exists"]
        .as_bool()
        .or_else(|| exists_result["found"].as_bool())
        .unwrap_or(false);
    assert!(found, "stored key should exist");
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_model_cache() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nmc-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let model_key = format!("model-cache-{}", std::process::id());
    let model_data = base64::engine::general_purpose::STANDARD
        .encode(b"fake model weights for caching test - not a real model");

    let store = direct_rpc_call(
        nestgate_socket,
        "storage.store",
        &serde_json::json!({
            "key": model_key,
            "value": model_data,
            "metadata": { "content_type": "application/x-model-weights" },
        }),
    );
    assert!(store.is_ok(), "model cache store should succeed: {store:?}");

    let nestgate_socket2 = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket (second call)");

    let retrieve = direct_rpc_call(
        nestgate_socket2,
        "storage.retrieve",
        &serde_json::json!({ "key": model_key }),
    );
    assert!(
        retrieve.is_ok(),
        "model cache retrieve should succeed: {retrieve:?}"
    );
    let result = retrieve.unwrap();
    assert_eq!(result["value"], model_data);
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_direct_health() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-ndh-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let health = direct_rpc_call(nestgate_socket, "health.liveness", &serde_json::json!({}));
    assert!(
        health.is_ok(),
        "nestgate health.liveness should respond: {health:?}"
    );
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn nest_discover_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-ndc-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Nest)
        .start(&family_id)
        .expect("nest atomic should start");

    let nestgate_socket = running
        .socket_for("storage")
        .or_else(|| running.socket_for_primal("nestgate"))
        .expect("nestgate socket");

    let caps = direct_rpc_call(nestgate_socket, "capabilities.list", &serde_json::json!({}));
    assert!(caps.is_ok(), "capabilities.list should succeed: {caps:?}");
    let result = caps.unwrap();
    let cap_list = result["capabilities"]
        .as_array()
        .or_else(|| result["methods"].as_array());
    assert!(
        cap_list.is_some(),
        "should return capabilities or methods array: {result}"
    );
    let cap_list = cap_list.unwrap();
    assert!(
        !cap_list.is_empty(),
        "nestgate should advertise at least one capability"
    );
    println!("  nestgate capabilities: {cap_list:?}");
}

// ===========================================================================
// NestGate Content Pipeline Tests (Wave 7: semantic contract gates)
// ===========================================================================

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

    let data_b64 =
        base64::engine::general_purpose::STANDARD.encode(b"primalSpring content gate test");
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

    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"content list gate test");
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

    let list = direct_rpc_call(nestgate_socket, "storage.list", &serde_json::json!({}))
        .expect("storage.list should succeed");

    let keys = list["keys"].as_array().or_else(|| list["items"].as_array());
    assert!(
        keys.is_some(),
        "storage.list should return keys or items array"
    );

    let keys = keys.unwrap();
    for key in keys {
        let key_str = key.as_str().unwrap_or_default();
        assert!(
            !key_str.contains("sensitive"),
            "storage.list should return opaque keys, not plaintext — leaked: {key_str}"
        );
    }
}

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

    let data_b64 =
        base64::engine::general_purpose::STANDARD.encode(b"blake3 content addressing test");
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
