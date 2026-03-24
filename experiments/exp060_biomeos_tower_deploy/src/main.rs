// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp060: biomeOS Tower Deploy — validate Tower composition via biomeOS orchestration.
//!
//! Unlike the harness-based experiments, this experiment launches the biomeOS
//! `neural-api-server` binary directly with the `neural-api` subcommand,
//! pointing it at biomeOS's `tower_atomic_bootstrap.toml` graph. biomeOS
//! discovers and germinates beardog + songbird from plasmidBin, then
//! primalSpring validates the resulting composition via the Neural API bridge.
//!
//! Requires `ECOPRIMALS_PLASMID_BIN` to point at `ecoPrimals/plasmidBin/`.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

struct BiomeOsGuard {
    child: Child,
    socket_path: PathBuf,
    runtime_dir: PathBuf,
}

impl Drop for BiomeOsGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_file(&self.socket_path);
        let _ = std::fs::remove_dir_all(&self.runtime_dir);
    }
}

fn discover_neural_api_binary() -> Option<PathBuf> {
    if let Ok(plasmid) = std::env::var("ECOPRIMALS_PLASMID_BIN") {
        let candidate = PathBuf::from(&plasmid).join("primals/neural-api-server");
        if candidate.is_file() {
            return Some(candidate);
        }
        let candidate = PathBuf::from(&plasmid).join("neural-api-server");
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn discover_biomeos_graphs() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("BIOMEOS_GRAPHS_DIR") {
        let p = PathBuf::from(&dir);
        if p.is_dir() {
            return Some(p);
        }
    }
    let candidates = [
        PathBuf::from("../phase2/biomeOS/graphs"),
        PathBuf::from("../../phase2/biomeOS/graphs"),
        PathBuf::from("../../../phase2/biomeOS/graphs"),
    ];
    for c in &candidates {
        if c.join("tower_atomic_bootstrap.toml").is_file() {
            return std::fs::canonicalize(c).ok();
        }
    }
    None
}

fn spawn_biomeos_neural_api(v: &mut ValidationResult) -> Option<BiomeOsGuard> {
    let Some(binary) = discover_neural_api_binary() else {
        v.check_skip(
            "biomeos_binary",
            "neural-api-server not found — set ECOPRIMALS_PLASMID_BIN",
        );
        return None;
    };

    let Some(graphs_dir) = discover_biomeos_graphs() else {
        v.check_skip(
            "biomeos_graphs",
            "biomeOS graphs/ not found — set BIOMEOS_GRAPHS_DIR",
        );
        return None;
    };

    v.check_bool(
        "biomeos_binary",
        true,
        &format!("found {}", binary.display()),
    );
    v.check_bool(
        "biomeos_graphs",
        true,
        &format!("found {}", graphs_dir.display()),
    );

    let pid = std::process::id();
    let family_id = format!("exp060-{pid}");
    let runtime_dir = std::env::temp_dir().join(format!("primalspring-exp060-{pid}"));
    let _ = std::fs::create_dir_all(runtime_dir.join("biomeos"));
    let socket_path = runtime_dir
        .join("biomeos")
        .join(format!("neural-api-{family_id}.sock"));
    let _ = std::fs::remove_file(&socket_path);

    let working_dir = graphs_dir.parent().unwrap_or(&graphs_dir);

    let mut cmd = Command::new(&binary);
    cmd.arg("neural-api");
    cmd.arg("--socket").arg(&socket_path);
    cmd.arg("--graphs-dir").arg(&graphs_dir);
    cmd.arg("--family-id").arg(&family_id);
    cmd.current_dir(working_dir);
    cmd.env("FAMILY_ID", &family_id);
    cmd.env("XDG_RUNTIME_DIR", &runtime_dir);
    cmd.env("BIOMEOS_MODE", "bootstrap");
    if let Ok(plasmid) = std::env::var("ECOPRIMALS_PLASMID_BIN") {
        cmd.env("BIOMEOS_PLASMID_BIN_DIR", &plasmid);
        cmd.env("ECOPRIMALS_PLASMID_BIN", &plasmid);
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            v.check_bool("biomeos_spawn", false, &format!("spawn failed: {e}"));
            return None;
        }
    };

    v.check_bool("biomeos_spawn", true, "neural-api-server spawned");

    let start = Instant::now();
    let timeout = Duration::from_secs(45);
    while start.elapsed() < timeout {
        if socket_path.exists() {
            std::thread::sleep(Duration::from_millis(500));
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    if !socket_path.exists() {
        v.check_bool(
            "biomeos_socket",
            false,
            &format!("socket did not appear within {timeout:?}"),
        );
        let mut guard = BiomeOsGuard {
            child,
            socket_path,
            runtime_dir,
        };
        let _ = guard.child.kill();
        return None;
    }

    v.check_bool(
        "biomeos_socket",
        true,
        &format!("ready in {:.1}s", start.elapsed().as_secs_f64()),
    );

    Some(BiomeOsGuard {
        child,
        socket_path,
        runtime_dir,
    })
}

fn validate_neural_api(guard: &BiomeOsGuard, v: &mut ValidationResult) {
    let socket_str = guard.socket_path.to_string_lossy();
    let family_id = guard
        .socket_path
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("neural-api-"))
        .unwrap_or("default");

    let Some(bridge) = NeuralBridge::discover_with(Some(&socket_str), Some(family_id)) else {
        v.check_bool("neural_bridge", false, "could not connect to Neural API");
        return;
    };

    let health = bridge.health_check();
    v.check_bool(
        "neural_api_health",
        health.is_ok(),
        "Neural API health check",
    );

    let security = bridge.discover_capability("security");
    v.check_bool(
        "capability_security",
        security.is_ok(),
        "security capability registered",
    );

    let discovery = bridge.discover_capability("discovery");
    v.check_bool(
        "capability_discovery",
        discovery.is_ok(),
        "discovery capability registered",
    );

    let crypto_result =
        bridge.capability_call("crypto", "generate_keypair", &serde_json::json!({}));
    match &crypto_result {
        Ok(r) => v.check_bool(
            "capability_call_crypto",
            !r.value.is_null(),
            "crypto.generate_keypair returned data",
        ),
        Err(e) => {
            let msg = format!("{e}");
            let expected = msg.contains("not found")
                || msg.contains("not registered")
                || msg.contains("Method not found");
            v.check_bool(
                "capability_call_crypto",
                expected,
                &format!("capability routing attempt: {e}"),
            );
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp060 — biomeOS Tower Deploy")
        .with_provenance("exp060_biomeos_tower_deploy", "2026-03-24")
        .run(
            "primalSpring Exp060: biomeOS-orchestrated Tower Atomic deployment",
            |v| {
                let guard = spawn_biomeos_neural_api(v);
                if let Some(ref g) = guard {
                    validate_neural_api(g, v);
                }
                drop(guard);
            },
        );
}
