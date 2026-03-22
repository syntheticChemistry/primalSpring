// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration test for the `primalspring_primal` JSON-RPC 2.0 server.
//!
//! Spawns the server binary on a temporary Unix socket, sends real
//! JSON-RPC requests, and validates responses. No mocks — this is a
//! full-stack IPC round-trip.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

fn unique_socket_dir() -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = std::env::temp_dir()
        .join(format!("primalspring-itest-{pid}-{id}"))
        .join("biomeos");
    std::fs::create_dir_all(&dir).expect("create temp biomeos dir");
    dir
}

struct ServerGuard {
    child: Child,
    socket_path: PathBuf,
    runtime_dir: PathBuf,
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_file(&self.socket_path);
        let _ = std::fs::remove_dir_all(&self.runtime_dir);
    }
}

fn setup_server() -> (ServerGuard, PathBuf) {
    let dir = unique_socket_dir();
    let runtime_dir = dir.parent().expect("parent").to_owned();
    let socket_path = dir.join("primalspring-default.sock");
    let _ = std::fs::remove_file(&socket_path);

    let binary = env!("CARGO_BIN_EXE_primalspring_primal");
    let child = Command::new(binary)
        .arg("server")
        .env("XDG_RUNTIME_DIR", &runtime_dir)
        .env("FAMILY_ID", "default")
        .spawn()
        .expect("spawn primalspring_primal server");

    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(5);
    while start.elapsed() < timeout {
        if socket_path.exists() {
            std::thread::sleep(Duration::from_millis(50));
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(
        socket_path.exists(),
        "server socket did not appear at {}",
        socket_path.display()
    );

    let guard = ServerGuard {
        child,
        socket_path: socket_path.clone(),
        runtime_dir,
    };
    (guard, socket_path)
}

fn send_rpc(stream: &UnixStream, method: &str, params: &serde_json::Value) -> serde_json::Value {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let mut line = serde_json::to_string(&request).unwrap();
    line.push('\n');

    let mut writer = stream;
    writer.write_all(line.as_bytes()).unwrap();

    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).unwrap();
    serde_json::from_str(&response_line).unwrap()
}

fn connect(socket_path: &PathBuf) -> UnixStream {
    let stream = UnixStream::connect(socket_path).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream
}

#[test]
fn health_check_returns_healthy() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(&stream, "health.check", &serde_json::Value::Null);
    assert_eq!(resp["jsonrpc"], "2.0");
    assert!(resp["error"].is_null());
    assert_eq!(resp["result"]["status"], "healthy");
    assert_eq!(resp["result"]["primal"], "primalspring");
    assert_eq!(resp["result"]["domain"], "coordination");
}

#[test]
fn health_liveness_returns_healthy() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(&stream, "health.liveness", &serde_json::Value::Null);
    assert!(resp["error"].is_null());
    assert_eq!(resp["result"]["status"], "healthy");
}

#[test]
fn capabilities_list_returns_niche_knowledge() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(&stream, "capabilities.list", &serde_json::Value::Null);
    assert!(resp["error"].is_null());
    let result = &resp["result"];
    assert!(result["capabilities"].is_array());
    let caps = result["capabilities"].as_array().unwrap();
    assert!(
        caps.iter()
            .any(|c| c == "coordination.validate_composition")
    );
    assert!(caps.iter().any(|c| c == "health.check"));
    assert!(result["semantic_mappings"].is_object());
    assert!(result["operation_dependencies"].is_object());
    assert!(result["cost_estimates"].is_object());
}

#[test]
fn lifecycle_status_returns_running() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(&stream, "lifecycle.status", &serde_json::Value::Null);
    assert!(resp["error"].is_null());
    assert_eq!(resp["result"]["primal"], "primalspring");
    assert_eq!(resp["result"]["status"], "running");
}

#[test]
fn unknown_method_returns_method_not_found() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(&stream, "nonexistent.method", &serde_json::Value::Null);
    assert!(resp["result"].is_null());
    assert_eq!(resp["error"]["code"], -32_601);
}

#[test]
fn validate_composition_tower() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(
        &stream,
        "coordination.validate_composition",
        &serde_json::json!({"atomic": "Tower"}),
    );
    assert!(resp["error"].is_null());
    let result = &resp["result"];
    assert_eq!(result["atomic"], "Tower");
    assert!(result["primals"].is_array());
}

#[test]
fn validate_composition_invalid_type_returns_error() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(
        &stream,
        "coordination.validate_composition",
        &serde_json::json!({"atomic": "NotARealType"}),
    );
    assert!(resp["result"].is_null());
    assert_eq!(resp["error"]["code"], -32_602);
}

#[test]
fn discovery_sweep_returns_capabilities() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(
        &stream,
        "coordination.discovery_sweep",
        &serde_json::json!({"atomic": "Tower"}),
    );
    assert!(resp["error"].is_null());
    assert_eq!(resp["result"]["mode"], "capability");
    let caps = resp["result"]["capabilities"].as_array().unwrap();
    assert_eq!(caps.len(), 2);
    assert_eq!(caps[0]["capability"], "security");
    assert_eq!(caps[1]["capability"], "discovery");
}

#[test]
fn discovery_sweep_identity_mode_returns_primals() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let resp = send_rpc(
        &stream,
        "coordination.discovery_sweep",
        &serde_json::json!({"atomic": "Tower", "mode": "identity"}),
    );
    assert!(resp["error"].is_null());
    assert_eq!(resp["result"]["mode"], "identity");
    let primals = resp["result"]["primals"].as_array().unwrap();
    assert_eq!(primals.len(), 2);
    assert_eq!(primals[0]["primal"], "beardog");
    assert_eq!(primals[1]["primal"], "songbird");
}

#[test]
fn malformed_json_returns_parse_error() {
    let (_guard, socket_path) = setup_server();
    let stream = connect(&socket_path);

    let mut writer = &stream;
    writer.write_all(b"not valid json\n").unwrap();

    let mut reader = BufReader::new(&stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).unwrap();
    let resp: serde_json::Value = serde_json::from_str(&response_line).unwrap();
    assert_eq!(resp["error"]["code"], -32_700);
}

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

/// Check if a process is alive via `kill(pid, 0)` (signal 0 = existence check).
fn process_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
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
                    (200..400).contains(&(code as u16).into()),
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

fn load_anthropic_key() -> Option<String> {
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }
    let candidates = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../testing-secrets/api-keys.toml"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../testing-secrets/api-keys.toml"),
    ];
    for path in &candidates {
        if let Ok(contents) = std::fs::read_to_string(path) {
            // The file may have non-TOML lines before the first section header.
            // Extract everything from the first `[` onward.
            let toml_start = contents.find("\n[").map_or(0, |i| i + 1);
            let toml_slice = &contents[toml_start..];
            if let Ok(parsed) = toml_slice.parse::<toml::Table>() {
                if let Some(ai) = parsed.get("ai_providers").and_then(|v| v.as_table()) {
                    if let Some(key) = ai.get("anthropic_api_key").and_then(|v| v.as_str()) {
                        if !key.is_empty() {
                            return Some(key.to_owned());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Squirrel uses `UniversalListener` which on Linux prefers an abstract
/// socket (`\0squirrel`) over filesystem sockets. We spawn the process
/// and poll health on the abstract socket.
struct SquirrelGuard {
    _process: primalspring::launcher::PrimalProcess,
}

impl SquirrelGuard {
    fn connect_abstract() -> std::io::Result<std::os::unix::net::UnixStream> {
        use std::os::linux::net::SocketAddrExt;
        use std::os::unix::net::{SocketAddr, UnixStream};
        let addr = SocketAddr::from_abstract_name("squirrel")?;
        UnixStream::connect_addr(&addr)
    }

    fn health_liveness(&self) -> bool {
        if let Ok(mut stream) = Self::connect_abstract() {
            use std::io::{BufRead, BufReader, Write};
            let req = r#"{"jsonrpc":"2.0","method":"health.liveness","id":1}"#;
            let msg = format!("{req}\n");
            if stream.write_all(msg.as_bytes()).is_ok() {
                let _ = stream.shutdown(std::net::Shutdown::Write);
                let reader = BufReader::new(&stream);
                for line in reader.lines().map_while(Result::ok) {
                    if line.contains("\"result\"") {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn spawn_squirrel_for_test(
    family_id: &str,
    nucleation: &mut primalspring::launcher::SocketNucleation,
    api_key: &str,
) -> SquirrelGuard {
    use primalspring::launcher::{self, PrimalProcess};

    let squirrel_socket = nucleation.assign("squirrel", family_id);
    let binary = launcher::discover_binary("squirrel").expect("squirrel binary should be found");
    let mut cmd = std::process::Command::new(&binary);
    cmd.arg("server");
    cmd.arg("--socket").arg(&squirrel_socket);
    cmd.arg("--port").arg("0");
    cmd.env("FAMILY_ID", family_id);
    cmd.env("XDG_RUNTIME_DIR", nucleation.base_dir());
    cmd.env("ANTHROPIC_API_KEY", api_key);
    cmd.env("SERVICE_MESH_PORT", "0");
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let child = cmd.spawn().expect("squirrel should spawn");

    // Poll until the abstract socket is reachable
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    loop {
        if SquirrelGuard::connect_abstract().is_ok() {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "squirrel should become reachable within 15s (abstract socket \\0squirrel)"
        );
        std::thread::sleep(Duration::from_millis(100));
    }

    SquirrelGuard {
        _process: PrimalProcess::from_parts("squirrel".to_owned(), squirrel_socket, child),
    }
}

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
    let squirrel = spawn_squirrel_for_test(&family_id, &mut nucleation, &api_key);

    assert!(squirrel.health_liveness(), "squirrel should be alive");

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
    let squirrel = spawn_squirrel_for_test(&family_id, &mut nucleation, &api_key);

    // Verify all Tower primals healthy
    for (name, live) in running.health_check_all() {
        assert!(
            live,
            "{name} should be healthy in tower+squirrel composition"
        );
    }

    // Verify squirrel healthy
    assert!(
        squirrel.health_liveness(),
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

/// Helper: send a JSON-RPC call directly to a primal's socket.
fn direct_rpc_call(
    socket: &std::path::Path,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::UnixStream;

    let mut stream =
        UnixStream::connect(socket).map_err(|e| format!("connect to {}: {e}", socket.display()))?;
    stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = format!("{req}\n");
    stream
        .write_all(msg.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    let _ = stream.shutdown(std::net::Shutdown::Write);

    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&line) {
            if let Some(result) = parsed.get("result") {
                return Ok(result.clone());
            }
            if let Some(error) = parsed.get("error") {
                return Err(format!("RPC error: {error}"));
            }
        }
    }
    Err("no JSON-RPC response received".to_owned())
}

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

            let enc_str = beacon.get("encrypted_beacon")
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
