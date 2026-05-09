// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stub primal harness — minimal JSON-RPC responders for contract testing.
//!
//! Provides lightweight in-process servers that speak the wire contract
//! of real primals (method names, response shapes, error codes) without
//! requiring `plasmidBin` binaries. Stubs run on Unix sockets and are
//! discoverable via the standard XDG convention.
//!
//! Supported stubs:
//! - **BearDog** (security): `health.liveness`, `identity.get`, `capabilities.list`,
//!   `auth.mode`, `auth.check`, `auth.issue_ionic`, `auth.verify_ionic`, `crypto.hash`
//! - **Songbird** (discovery): `health.liveness`, `identity.get`, `capabilities.list`,
//!   `ipc.resolve`, `ipc.register`
//!
//! Feature-gated behind `stub-primals` so CI can opt in without affecting
//! the default build.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static STUB_COUNTER: AtomicU32 = AtomicU32::new(0);

/// A running stub primal. Stops the listener thread on drop.
pub struct StubPrimal {
    pub name: String,
    pub socket_path: PathBuf,
    _handle: thread::JoinHandle<()>,
    _runtime_dir: PathBuf,
}

impl StubPrimal {
    fn socket_dir() -> PathBuf {
        let id = STUB_COUNTER.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let dir = std::env::temp_dir()
            .join(format!("primalspring-stub-{pid}-{id}"))
            .join("biomeos");
        std::fs::create_dir_all(&dir).expect("create stub socket dir");
        dir
    }
}

impl Drop for StubPrimal {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
        if let Some(parent) = self.socket_path.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
    }
}

type MethodHandler = Box<dyn Fn(&serde_json::Value) -> serde_json::Value + Send + Sync>;

/// Builder for stub primals.
pub struct StubBuilder {
    name: String,
    handlers: HashMap<String, MethodHandler>,
}

impl StubBuilder {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            handlers: HashMap::new(),
        }
    }

    fn method(
        mut self,
        name: &str,
        handler: impl Fn(&serde_json::Value) -> serde_json::Value + Send + Sync + 'static,
    ) -> Self {
        self.handlers.insert(name.to_owned(), Box::new(handler));
        self
    }

    /// Start the stub, returning the running handle.
    fn start(self) -> StubPrimal {
        let dir = StubPrimal::socket_dir();
        let runtime_dir = dir.parent().expect("parent").to_owned();
        let socket_path = dir.join(format!("{}-default.sock", self.name));
        let _ = std::fs::remove_file(&socket_path);

        let listener = UnixListener::bind(&socket_path).expect("bind stub socket");
        listener
            .set_nonblocking(false)
            .expect("set blocking on stub listener");

        let handlers = std::sync::Arc::new(self.handlers);
        let sock = socket_path.clone();

        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { break };
                let handlers = std::sync::Arc::clone(&handlers);
                thread::spawn(move || {
                    let _ = handle_stub_connection(&stream, &handlers);
                });
            }
        });

        // Wait for socket to be connectable.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while std::time::Instant::now() < deadline {
            if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        StubPrimal {
            name: self.name,
            socket_path,
            _handle: handle,
            _runtime_dir: runtime_dir,
        }
    }
}

fn handle_stub_connection(
    stream: &std::os::unix::net::UnixStream,
    handlers: &HashMap<String, MethodHandler>,
) -> std::io::Result<()> {
    let mut writer = stream;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let parsed: serde_json::Value =
            serde_json::from_str(line.trim()).unwrap_or(serde_json::json!(null));
        let method = parsed["method"].as_str().unwrap_or("");
        let params = parsed
            .get("params")
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let id = parsed["id"].as_u64().unwrap_or(0);

        let response = if let Some(handler) = handlers.get(method) {
            let result = handler(&params);
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": id,
            })
        } else {
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32601, "message": format!("method not found: {method}") },
                "id": id,
            })
        };

        let resp_str = serde_json::to_string(&response).unwrap_or_default();
        writer.write_all(resp_str.as_bytes())?;
        writer.write_all(b"\n")?;
        line.clear();
    }
    Ok(())
}

// ── Pre-built stub factories ────────────────────────────────────────────

/// Stub BearDog — security primal responding to the Tower contract.
pub fn stub_beardog() -> StubPrimal {
    StubBuilder::new("beardog")
        .method(
            "health.liveness",
            |_| serde_json::json!({ "alive": true, "primal": "beardog" }),
        )
        .method("identity.get", |_| {
            serde_json::json!({
                "primal_id": "beardog",
                "version": "0.0.0-stub",
                "domain": "security",
            })
        })
        .method("capabilities.list", |_| {
            serde_json::json!({
                "capabilities": [
                    "crypto.sign", "crypto.verify", "crypto.hash",
                    "auth.check", "auth.mode", "auth.issue_ionic",
                    "auth.verify_ionic", "health.liveness", "identity.get",
                ]
            })
        })
        .method("auth.mode", |_| serde_json::json!({ "mode": "permissive" }))
        .method("auth.check", |params| {
            let has_token = params.get("_bearer_token").is_some();
            serde_json::json!({
                "authenticated": has_token,
                "verified": has_token,
                "enforcement": "permissive",
            })
        })
        .method("auth.issue_ionic", |params| {
            let scope = params
                .get("scope")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("*");
            serde_json::json!({
                "token": format!("stub-ionic-{scope}"),
                "scope": scope,
                "expires_in": 300,
            })
        })
        .method("auth.verify_ionic", |params| {
            let token = params
                .get("token")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            let valid = token.starts_with("stub-ionic-");
            serde_json::json!({
                "valid": valid,
                "scopes": [if valid { token.trim_start_matches("stub-ionic-") } else { "" }],
                "subject": "stub-user",
                "expires_in": 300,
            })
        })
        .method("crypto.hash", |params| {
            let data = params
                .get("data")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            serde_json::json!({ "hash": format!("stub-hash-{}", data.len()) })
        })
        .start()
}

/// Stub Songbird — discovery primal responding to the Tower contract.
pub fn stub_songbird(registered: &[(&str, &Path)]) -> StubPrimal {
    let registry: HashMap<String, String> = registered
        .iter()
        .map(|(name, path)| ((*name).to_owned(), path.to_string_lossy().to_string()))
        .collect();
    let registry = std::sync::Arc::new(registry);

    let reg_resolve = std::sync::Arc::clone(&registry);
    let reg_caps = std::sync::Arc::clone(&registry);

    StubBuilder::new("songbird")
        .method(
            "health.liveness",
            |_| serde_json::json!({ "alive": true, "primal": "songbird" }),
        )
        .method("identity.get", |_| {
            serde_json::json!({
                "primal_id": "songbird",
                "version": "0.0.0-stub",
                "domain": "discovery",
            })
        })
        .method("capabilities.list", move |_| {
            let names: Vec<&str> = reg_caps.keys().map(String::as_str).collect();
            serde_json::json!({
                "capabilities": names,
            })
        })
        .method("ipc.resolve", move |params| {
            let primal_id = params
                .get("primal_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            if let Some(socket) = reg_resolve.get(primal_id) {
                serde_json::json!({
                    "primal_id": primal_id,
                    "socket": socket,
                    "transport": "unix",
                })
            } else {
                serde_json::json!({
                    "error": format!("primal not found: {primal_id}"),
                })
            }
        })
        .method("ipc.register", |params| {
            let primal_id = params
                .get("primal_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            serde_json::json!({ "registered": primal_id })
        })
        .start()
}

/// Start a minimal Tower stub (BearDog + Songbird) and return both
/// stubs with their socket paths.
pub fn stub_tower() -> (StubPrimal, StubPrimal) {
    let beardog = stub_beardog();
    let songbird = stub_songbird(&[("beardog", &beardog.socket_path)]);
    (beardog, songbird)
}
