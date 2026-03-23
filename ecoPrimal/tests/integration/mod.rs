// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared integration test infrastructure for the `primalspring_primal` server.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn unique_socket_dir() -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = std::env::temp_dir()
        .join(format!("primalspring-itest-{pid}-{id}"))
        .join("biomeos");
    std::fs::create_dir_all(&dir).expect("create temp biomeos dir");
    dir
}

pub struct ServerGuard {
    pub child: Child,
    pub socket_path: PathBuf,
    pub runtime_dir: PathBuf,
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_file(&self.socket_path);
        let _ = std::fs::remove_dir_all(&self.runtime_dir);
    }
}

pub fn setup_server() -> (ServerGuard, PathBuf) {
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

pub fn send_rpc(
    stream: &UnixStream,
    method: &str,
    params: &serde_json::Value,
) -> serde_json::Value {
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

pub fn connect(socket_path: &PathBuf) -> UnixStream {
    let stream = UnixStream::connect(socket_path).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream
}

/// Check if a process is alive via `kill(pid, 0)` (signal 0 = existence check).
pub fn process_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

pub fn load_anthropic_key() -> Option<String> {
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
pub struct SquirrelGuard {
    _process: primalspring::launcher::PrimalProcess,
}

impl SquirrelGuard {
    fn connect_abstract() -> std::io::Result<std::os::unix::net::UnixStream> {
        use std::os::linux::net::SocketAddrExt;
        use std::os::unix::net::{SocketAddr, UnixStream};
        let addr = SocketAddr::from_abstract_name("squirrel")?;
        UnixStream::connect_addr(&addr)
    }

    pub fn health_liveness() -> bool {
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

pub fn spawn_squirrel_for_test(
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

/// Helper: send a JSON-RPC call directly to a primal's socket.
pub fn direct_rpc_call(
    socket: &Path,
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
