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

//! Node atomic integration tests. Run with `cargo test --ignored`.

mod integration;

use integration::direct_rpc_call;

// ===========================================================================
// Node Atomic Tests — Tower + ToadStool (Gates 11-13: compute composition)
// ===========================================================================

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
