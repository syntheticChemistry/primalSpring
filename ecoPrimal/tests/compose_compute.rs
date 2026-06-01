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

//! Compute trio integration tests. Run with `cargo test --ignored`.

#[expect(
    dead_code,
    reason = "shared helpers — each test file uses a different subset"
)]
mod integration;

use integration::direct_rpc_call;

// ===========================================================================
// Compute Trio Gate Tests (Wave 8: Node atomic sovereign dispatch)
//
// coralReef (shader compile) + toadStool (dispatch) + barraCuda (math/physics)
// ===========================================================================

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
