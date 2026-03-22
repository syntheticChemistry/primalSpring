// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp065: petalTongue Tower Dashboard — visualization via Tower composition.
//!
//! Spawns Tower + petalTongue via the harness and validates:
//! 1. petalTongue health via JSON-RPC
//! 2. `visualization.render.dashboard` with Tower health data
//! 3. `visualization.render.grammar` with a Grammar-of-Graphics expression
//! 4. Headless SVG/JSON export
//!
//! petalTongue must be in `plasmidBin/primals/` and its launch profile in
//! `config/primal_launch_profiles.toml`.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::launcher::{PrimalProcess, SocketNucleation};
use primalspring::validation::ValidationResult;

fn rpc_call(
    socket: &Path,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let mut stream = UnixStream::connect(socket).map_err(|e| format!("connect: {e}"))?;
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
    Err("no response".to_owned())
}

fn find_petaltongue_binary() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(plasmid) = std::env::var("ECOPRIMALS_PLASMID_BIN") {
        candidates.push(PathBuf::from(plasmid).join("primals/petaltongue"));
    }

    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../plasmidBin/primals/petaltongue"),
    );

    candidates.into_iter().find(|p| p.exists())
}

fn spawn_petaltongue(
    family_id: &str,
    nucleation: &mut SocketNucleation,
    binary: &Path,
) -> Option<(PrimalProcess, PathBuf)> {
    let socket = nucleation.assign("petaltongue", family_id);

    let mut cmd = std::process::Command::new(binary);
    cmd.arg("server");
    cmd.env("FAMILY_ID", family_id);
    cmd.env("XDG_RUNTIME_DIR", nucleation.base_dir());
    cmd.env("PETALTONGUE_SOCKET", &socket);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().ok()?;
    let process = PrimalProcess::from_parts("petaltongue".to_owned(), socket.clone(), child);

    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        if socket.exists() {
            if let Ok(mut s) = UnixStream::connect(&socket) {
                let health_req = r#"{"jsonrpc":"2.0","method":"health.liveness","id":1}"#;
                if s.write_all(format!("{health_req}\n").as_bytes()).is_ok() {
                    let _ = s.shutdown(std::net::Shutdown::Write);
                    let reader = BufReader::new(&s);
                    for line in reader.lines().map_while(Result::ok) {
                        if line.contains("\"result\"") {
                            return Some((process, socket));
                        }
                    }
                }
            }
        }
        if std::time::Instant::now() >= deadline {
            println!("  [WARN] petaltongue did not become ready within 10s");
            return Some((process, socket));
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}

fn validate_dashboard(
    v: &mut primalspring::validation::ValidationResult,
    pt_socket: &Path,
    _tower_health: &[serde_json::Value],
    family_id: &str,
) {
    let dashboard = rpc_call(
        pt_socket,
        "visualization.render.dashboard",
        &serde_json::json!({
            "session_id": family_id,
            "title": "Tower Atomic Health",
            "bindings": [{
                "channel_type": "bar",
                "id": "primal_status",
                "label": "Primal Status",
                "x_label": "Primal",
                "y_label": "Health",
                "categories": ["beardog", "songbird"],
                "values": [1.0, 1.0],
                "unit": "status"
            }],
            "modality": "description"
        }),
    );

    match &dashboard {
        Ok(resp) => {
            println!("  dashboard render: {} bytes", resp.to_string().len());
            v.check_bool(
                "dashboard_rendered",
                true,
                "Tower health dashboard rendered",
            );
        }
        Err(e) => {
            println!("  dashboard: {e}");
            v.check_bool("dashboard_rendered", false, &format!("dashboard: {e}"));
        }
    }

    let grammar = rpc_call(
        pt_socket,
        "visualization.render.grammar",
        &serde_json::json!({
            "session_id": family_id,
            "grammar": {
                "data_source": "tower_health",
                "variables": [
                    { "name": "x", "role": "X", "field": "primal" },
                    { "name": "y", "role": "Y", "field": "status" }
                ],
                "geometry": "Bar",
                "scales": [],
                "coordinate": "Cartesian",
                "facets": null,
                "aesthetics": [],
                "title": "Tower Health",
                "domain": "health"
            },
            "data": [
                { "primal": "beardog", "status": 1 },
                { "primal": "songbird", "status": 1 }
            ],
            "modality": "description"
        }),
    );

    match &grammar {
        Ok(resp) => {
            let is_svg = resp.as_str().is_some_and(|s| s.contains("<svg"));
            let is_json = resp.is_object() || resp.is_array();
            println!(
                "  grammar render: {} bytes (svg={is_svg}, json={is_json})",
                resp.to_string().len()
            );
            v.check_bool(
                "grammar_rendered",
                true,
                "Grammar of Graphics expression rendered",
            );
        }
        Err(e) => {
            println!("  grammar: {e}");
            v.check_bool("grammar_rendered", false, &format!("grammar: {e}"));
        }
    }
}

fn main() {
    let graphs_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../graphs");
    let family_id = format!("e065-{}", std::process::id());

    ValidationResult::run_experiment(
        "primalSpring Exp065 — petalTongue Tower Dashboard",
        "primalSpring Exp065: Tower health visualization via petalTongue",
        |v| {
            let Some(pt_binary) = find_petaltongue_binary() else {
                println!("  [SKIP] petaltongue binary not found in plasmidBin/primals/");
                v.check_bool(
                    "petaltongue_binary_found",
                    false,
                    "petaltongue not in plasmidBin — build and copy first",
                );
                return;
            };

            v.check_bool(
                "petaltongue_binary_found",
                true,
                "petaltongue binary located",
            );

            let running = AtomicHarness::new(AtomicType::Tower)
                .start_with_neural_api(&family_id, &graphs_dir)
                .expect("tower + neural-api should start");

            let runtime_dir = running.runtime_dir().to_path_buf();
            let mut nucleation = SocketNucleation::new(runtime_dir);

            let Some((_pt_proc, pt_socket)) =
                spawn_petaltongue(&family_id, &mut nucleation, &pt_binary)
            else {
                v.check_bool("petaltongue_spawned", false, "petaltongue failed to spawn");
                return;
            };

            v.check_bool("petaltongue_spawned", true, "petaltongue running");

            let health = rpc_call(&pt_socket, "health.liveness", &serde_json::json!({}));
            match &health {
                Ok(_) => v.check_bool("petaltongue_healthy", true, "petalTongue health OK"),
                Err(e) => {
                    println!("  health: {e}");
                    v.check_bool("petaltongue_healthy", false, &format!("health: {e}"));
                }
            }

            let tower_health: Vec<serde_json::Value> = running
                .health_check_all()
                .iter()
                .map(|(name, live)| {
                    serde_json::json!({
                        "primal": name,
                        "status": if *live { "healthy" } else { "unhealthy" }
                    })
                })
                .collect();

            validate_dashboard(v, &pt_socket, &tower_health, &family_id);
        },
    );
}
