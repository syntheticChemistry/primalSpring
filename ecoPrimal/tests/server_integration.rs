// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration test for the `primalspring_primal` JSON-RPC 2.0 server.
//!
//! Spawns the server binary on a temporary Unix socket, sends real
//! JSON-RPC requests, and validates responses. No mocks — this is a
//! full-stack IPC round-trip.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
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
