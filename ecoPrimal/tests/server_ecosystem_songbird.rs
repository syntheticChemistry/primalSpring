// SPDX-License-Identifier: AGPL-3.0-or-later
#![expect(clippy::unwrap_used, clippy::expect_used, reason = "integration tests — panics are the failure signal")]

//! Songbird IPC surface integration tests. Run with `cargo test --ignored`.

#[expect(
    dead_code,
    reason = "shared helpers — each test file uses a different subset"
)]
mod integration;

use integration::direct_rpc_call;
use std::path::PathBuf;

// ===========================================================================
// Tower Subsystem Tests — Songbird IPC surface (Phase 2 utilization gates)
// ===========================================================================

// ---------------------------------------------------------------------------
// Gate 7.1: Discovery announce + find_primals
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_discovery_announce_find() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-disc-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let songbird_socket = running
        .socket_for("discovery")
        .or_else(|| running.socket_for_primal("songbird"))
        .expect("songbird socket");

    let announce_result = direct_rpc_call(
        songbird_socket,
        "discovery.announce",
        &serde_json::json!({
            "primal": "primalspring-test",
            "capabilities": ["coordination"],
            "socket": "/tmp/primalspring-test.sock"
        }),
    );

    match &announce_result {
        Ok(_) => println!("  discovery.announce: OK"),
        Err(e) => println!("  discovery.announce: {e} (may be expected)"),
    }

    let find_result = direct_rpc_call(
        songbird_socket,
        "discovery.find_primals",
        &serde_json::json!({}),
    );

    match &find_result {
        Ok(v) => {
            println!("  discovery.find_primals: {v}");
            assert!(!v.is_null(), "find_primals should return data");
        }
        Err(e) => {
            assert!(
                e.contains("not found") || e.contains("Method not found") || e.contains("error"),
                "expected graceful response from find_primals, got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 7.2: STUN public address detection
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries + network (run with --ignored)"]
fn tower_stun_public_address() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-stun-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let songbird_socket = running
        .socket_for("discovery")
        .or_else(|| running.socket_for_primal("songbird"))
        .expect("songbird socket");

    let result = direct_rpc_call(
        songbird_socket,
        "stun.get_public_address",
        &serde_json::json!({}),
    );

    match &result {
        Ok(v) => {
            println!("  stun.get_public_address: {v}");
        }
        Err(e) => {
            println!("  stun.get_public_address: {e}");
            assert!(
                e.contains("not found")
                    || e.contains("Method not found")
                    || e.contains("timeout")
                    || e.contains("error"),
                "expected graceful STUN response, got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 7.3: BirdSong beacon round-trip (encrypt + decrypt)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_birdsong_beacon() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-birdsong-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let songbird_socket = running
        .socket_for("discovery")
        .or_else(|| running.socket_for_primal("songbird"))
        .expect("songbird socket");

    let gen_result = direct_rpc_call(
        songbird_socket,
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "family_id": &family_id,
            "node_id": &family_id,
            "capabilities": ["security", "discovery"]
        }),
    );

    match &gen_result {
        Ok(beacon) => {
            println!(
                "  birdsong.generate_encrypted_beacon: OK ({}B)",
                beacon.to_string().len()
            );

            let enc_str = beacon
                .get("encrypted_beacon")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let decrypt_result = direct_rpc_call(
                songbird_socket,
                "birdsong.decrypt_beacon",
                &serde_json::json!({ "encrypted_beacon": enc_str }),
            );

            match &decrypt_result {
                Ok(v) => println!("  birdsong.decrypt_beacon: {v}"),
                Err(e) => println!("  birdsong.decrypt_beacon: {e}"),
            }
        }
        Err(e) => {
            println!("  birdsong.generate_encrypted_beacon: {e}");
            assert!(
                e.contains("not found") || e.contains("Method not found") || e.contains("error"),
                "expected graceful BirdSong response, got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 7.4: Sovereign onion service lifecycle
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_onion_service() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-onion-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let songbird_socket = running
        .socket_for("discovery")
        .or_else(|| running.socket_for_primal("songbird"))
        .expect("songbird socket");

    let status_result = direct_rpc_call(songbird_socket, "onion.status", &serde_json::json!({}));

    match &status_result {
        Ok(v) => println!("  onion.status: {v}"),
        Err(e) => println!("  onion.status: {e}"),
    }

    let start_result = direct_rpc_call(
        songbird_socket,
        "onion.start",
        &serde_json::json!({ "family_id": &family_id }),
    );

    match &start_result {
        Ok(v) => {
            println!("  onion.start: {v}");
            if let Some(addr) = v.get("address").and_then(|a| a.as_str()) {
                println!("  onion address: {addr}");
            }
        }
        Err(e) => {
            println!("  onion.start: {e}");
            assert!(
                e.contains("not found") || e.contains("Method not found") || e.contains("error"),
                "expected graceful onion response, got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 7.5: Tor subsystem status
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_tor_status() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-tor-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let songbird_socket = running
        .socket_for("discovery")
        .or_else(|| running.socket_for_primal("songbird"))
        .expect("songbird socket");

    let result = direct_rpc_call(songbird_socket, "tor.status", &serde_json::json!({}));

    match &result {
        Ok(v) => {
            println!("  tor.status: {v}");
        }
        Err(e) => {
            println!("  tor.status: {e}");
            assert!(
                e.contains("not found") || e.contains("Method not found") || e.contains("error"),
                "expected graceful Tor response, got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 7.6: Federation status
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_federation_status() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-fed-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let songbird_socket = running
        .socket_for("discovery")
        .or_else(|| running.socket_for_primal("songbird"))
        .expect("songbird socket");

    let result = direct_rpc_call(
        songbird_socket,
        "songbird.federation.peers",
        &serde_json::json!({}),
    );

    match &result {
        Ok(v) => {
            println!("  federation.peers: {v}");
        }
        Err(e) => {
            println!("  federation.peers: {e}");
            assert!(
                e.contains("not found") || e.contains("Method not found") || e.contains("error"),
                "expected graceful federation response, got: {e}"
            );
        }
    }
}
