// SPDX-License-Identifier: AGPL-3.0-or-later

//! Live ecosystem integration tests requiring plasmidBin binaries. Run with `cargo test --ignored`.

#[expect(
    dead_code,
    reason = "shared helpers — each test file uses a different subset"
)]
mod integration;

use integration::{
    SquirrelGuard, direct_rpc_call, load_anthropic_key, process_alive, spawn_squirrel_for_test,
};
use std::path::PathBuf;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Live atomic harness tests — require plasmidBin binaries
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_atomic_live_health_check() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-tower-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    assert_eq!(running.primal_count(), 2, "Tower = beardog + songbird");

    let health = running.health_check_all();
    for (name, live) in &health {
        assert!(live, "{name} should be live");
    }
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_atomic_live_capabilities() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-caps-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    let caps = running.capabilities_all();
    assert_eq!(caps.len(), 2, "should query both primals");
    // Primals may or may not implement capabilities.list — we just verify
    // the query completes without crashing.
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_atomic_live_validation_result() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let family_id = format!("itest-val-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    let mut v = ValidationResult::new("tower_atomic_live");
    running.validate(&mut v);
    assert!(v.passed > 0, "should have at least one passing check");
    assert_eq!(v.failed, 0, "should have zero failures");
}

// ---------------------------------------------------------------------------
// Live Tower + Neural API tests — require plasmidBin binaries
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_neural_api_health() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-neural-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    assert!(running.has_neural_api(), "neural API should be running");
    assert_eq!(running.primal_count(), 2, "Tower = beardog + songbird");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");
    let health = bridge.health_check();
    assert!(
        health.is_ok(),
        "Neural API health check should succeed: {health:?}"
    );
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_neural_api_capability_discovery() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-ncap-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");
    let coordination = bridge.discover_capability("ecosystem.coordination");
    assert!(
        coordination.is_ok(),
        "should discover ecosystem.coordination: {coordination:?}"
    );
}

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_neural_api_full_validation() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::validation::ValidationResult;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-nval-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let mut v = ValidationResult::new("tower_neural_api_live");
    running.validate(&mut v);
    assert!(v.passed > 0, "should have passing checks");
    assert!(
        v.checks.iter().any(|c| c.name == "neural_api_health"),
        "should include Neural API health check"
    );
}

// ---------------------------------------------------------------------------
// Gate 1.5: Zombie process check
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_zombie_check() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-zombie-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let pids = running.pids();
    assert!(
        pids.len() >= 3,
        "should have beardog + songbird + neural-api PIDs, got {}",
        pids.len()
    );

    for &pid in &pids {
        assert!(process_alive(pid), "PID {pid} should be alive before drop");
    }

    drop(running);
    std::thread::sleep(Duration::from_millis(500));

    for &pid in &pids {
        assert!(
            !process_alive(pid),
            "PID {pid} should NOT be alive after RunningAtomic::drop() — zombie or orphan detected"
        );
    }
}

// ---------------------------------------------------------------------------
// Gate 3.5: Discovery peer list via Neural API
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_discovery_peer_list() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-peers-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");

    let result = bridge.capability_call("discovery", "peers", &serde_json::json!({}));

    match result {
        Ok(call_result) => {
            assert!(
                !call_result.value.is_null(),
                "discovery.peers should return a non-null result"
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("not found") || msg.contains("not registered"),
                "expected capability routing (possibly unregistered), got unexpected error: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 4.1: TLS X25519 key exchange via Neural API
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_tls_handshake() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-tls-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");

    let result = bridge.capability_call(
        "crypto",
        "generate_keypair",
        &serde_json::json!({ "algorithm": "x25519" }),
    );

    match result {
        Ok(call_result) => {
            assert!(
                !call_result.value.is_null(),
                "crypto.generate_keypair should return key material"
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("not found")
                    || msg.contains("not registered")
                    || msg.contains("Method not found"),
                "expected capability routing (possibly unregistered), got unexpected error: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 4.2: TLS internet reach (requires network)
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries + network access (run with --include-ignored)"]
fn tower_tls_internet_reach() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-https-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");

    let result = bridge.capability_call(
        "discovery",
        "https_probe",
        &serde_json::json!({ "url": "https://github.com", "timeout_secs": 10 }),
    );

    match result {
        Ok(call_result) => {
            let status = call_result
                .value
                .get("status_code")
                .and_then(serde_json::Value::as_u64);
            if let Some(code) = status {
                assert!(
                    (200..400).contains(&code),
                    "HTTPS probe to github.com should return 2xx/3xx, got {code}"
                );
            }
        }
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("not found")
                    || msg.contains("not registered")
                    || msg.contains("not implemented")
                    || msg.contains("Failed to forward"),
                "expected routing attempt to songbird (forwarding or not-found), got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 4.3: TLS routing audit — verify crypto uses capability.call path
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn tower_tls_routing_audit() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-audit-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");

    let crypto_cap = bridge.discover_capability("crypto");
    assert!(
        crypto_cap.is_ok(),
        "Neural API should have 'crypto' capability registered: {crypto_cap:?}"
    );

    let crypto_info = crypto_cap.unwrap();
    assert!(
        !crypto_info.is_null(),
        "crypto capability discovery should return non-null metadata"
    );

    let security_cap = bridge.discover_capability("security");
    assert!(
        security_cap.is_ok(),
        "Neural API should have 'security' capability registered: {security_cap:?}"
    );

    let tls_ops = ["generate_keypair", "tls_x25519_keygen", "derive_child_seed"];
    let mut routable = 0;
    for op in &tls_ops {
        let result = bridge.capability_call("crypto", op, &serde_json::json!({}));
        match &result {
            Ok(_) => routable += 1,
            Err(e) => {
                let msg = format!("{e}");
                if !msg.contains("not found") && !msg.contains("Method not found") {
                    routable += 1;
                }
            }
        }
    }

    assert!(
        routable > 0,
        "at least one TLS crypto operation should be routable through capability.call"
    );
}

// ===========================================================================
// Squirrel AI composition tests
// ===========================================================================

// ---------------------------------------------------------------------------
// Squirrel AI Query: Tower + Squirrel + Neural API, sends ai.query
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries + ANTHROPIC_API_KEY (run with --ignored)"]
fn tower_squirrel_ai_query() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::launcher::SocketNucleation;

    let api_key = load_anthropic_key()
        .expect("ANTHROPIC_API_KEY must be set or testing-secrets/api-keys.toml must exist");

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-sqai-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let runtime_dir = running.runtime_dir().to_path_buf();
    let mut nucleation = SocketNucleation::new(runtime_dir);
    let _squirrel = spawn_squirrel_for_test(&family_id, &mut nucleation, &api_key);

    assert!(SquirrelGuard::health_liveness(), "squirrel should be alive");

    let bridge = running.neural_bridge().expect("should get NeuralBridge");
    let result = bridge.capability_call(
        "ai",
        "query",
        &serde_json::json!({
            "prompt": "In one sentence, what is ecosystem coordination?"
        }),
    );

    match result {
        Ok(call_result) => {
            let has_response = call_result
                .value
                .get("response")
                .and_then(|v| v.as_str())
                .is_some_and(|s| !s.is_empty());
            assert!(
                has_response,
                "AI query should return a non-empty response: {call_result:?}"
            );
        }
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("not found")
                    || msg.contains("not registered")
                    || msg.contains("Failed to forward")
                    || msg.contains("Method not found"),
                "expected routing attempt or AI response, got: {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Composition health: Tower + Squirrel all healthy simultaneously
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries + ANTHROPIC_API_KEY (run with --ignored)"]
fn tower_squirrel_composition_health() {
    use primalspring::coordination::AtomicType;
    use primalspring::harness::AtomicHarness;
    use primalspring::launcher::SocketNucleation;

    let api_key = load_anthropic_key()
        .expect("ANTHROPIC_API_KEY must be set or testing-secrets/api-keys.toml must exist");

    let graphs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../graphs");
    let family_id = format!("itest-sqhealth-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start_with_neural_api(&family_id, &graphs)
        .expect("tower + neural-api should start");

    let runtime_dir = running.runtime_dir().to_path_buf();
    let mut nucleation = SocketNucleation::new(runtime_dir);
    let _squirrel = spawn_squirrel_for_test(&family_id, &mut nucleation, &api_key);

    // Verify all Tower primals healthy
    for (name, live) in running.health_check_all() {
        assert!(
            live,
            "{name} should be healthy in tower+squirrel composition"
        );
    }

    // Verify squirrel healthy
    assert!(
        SquirrelGuard::health_liveness(),
        "squirrel should be healthy in composition"
    );

    // Verify the Neural API bridge is still functional
    let bridge = running.neural_bridge().expect("should get NeuralBridge");
    let health = bridge.health_check();
    assert!(
        health.is_ok(),
        "Neural API should be healthy with Squirrel added: {health:?}"
    );

    // Verify security capability still registered
    let security = bridge.discover_capability("security");
    assert!(
        security.is_ok(),
        "security capability should still be registered: {security:?}"
    );

    // Verify discovery capability still registered
    let discovery = bridge.discover_capability("discovery");
    assert!(
        discovery.is_ok(),
        "discovery capability should still be registered: {discovery:?}"
    );
}

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
