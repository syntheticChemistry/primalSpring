//! Exp118: Neural API Graph Execution — validates the biomeOS graph executor
//! lifecycle through the live Neural API socket.
//!
//! Validates:
//!   1. riboCipher dual-lane wire protocol (0xEC + tier byte + NDJSON)
//!   2. primal.announce → dynamic capability registration
//!   3. capability.call → semantic forwarding to registered primals
//!   4. graph.execute → async graph orchestration with execution_id
//!   5. graph.status → per-execution timing and node-level state
//!   6. Repeated execution → trace accumulation (structural PathwayLearner input)
//!
//! This is primalSpring's validation anchor for Neural API Phase 1-3 convergence.
//! Exercises the "graph of graphs" substrate that primals self-compose through.

use primalspring::validation::ValidationResult;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

const NEURAL_SOCKET_ENV: &str = "NEURAL_API_SOCKET";
const DEFAULT_NEURAL_SOCKET: &str = "/run/user/1000/biomeos/biomeos-neural.sock";
const RIBOCIPHER_SIGNAL: &[u8] = &[0xEC, 0x00];

fn neural_socket_path() -> String {
    std::env::var(NEURAL_SOCKET_ENV).unwrap_or_else(|_| DEFAULT_NEURAL_SOCKET.to_string())
}

fn neural_rpc(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let path = neural_socket_path();
    let socket_path = Path::new(&path);

    if !socket_path.exists() {
        return Err(format!("Neural API socket not found: {path}"));
    }

    let mut stream = UnixStream::connect(socket_path)
        .map_err(|e| format!("Connect failed: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .ok();

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let msg = serde_json::to_string(&request).map_err(|e| format!("Serialize: {e}"))?;

    stream
        .write_all(RIBOCIPHER_SIGNAL)
        .map_err(|e| format!("Write signal: {e}"))?;
    stream
        .write_all(msg.as_bytes())
        .map_err(|e| format!("Write msg: {e}"))?;
    stream
        .write_all(b"\n")
        .map_err(|e| format!("Write newline: {e}"))?;
    stream.flush().map_err(|e| format!("Flush: {e}"))?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .map_err(|e| format!("Read response: {e}"))?;

    if line.is_empty() {
        return Err("Empty response from Neural API".to_string());
    }

    let response: serde_json::Value =
        serde_json::from_str(line.trim()).map_err(|e| format!("Parse response: {e}"))?;

    if let Some(error) = response.get("error") {
        return Err(format!(
            "RPC error: {}",
            error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown")
        ));
    }

    response
        .get("result")
        .cloned()
        .ok_or_else(|| "Missing 'result' field".to_string())
}

fn phase_connectivity(v: &mut ValidationResult) {
    let path = neural_socket_path();
    let exists = Path::new(&path).exists();
    v.check_bool(
        "neural_api_socket_exists",
        exists,
        &format!("Neural API socket present at {path}"),
    );

    if !exists {
        return;
    }

    match neural_rpc("health.check", &serde_json::json!({})) {
        Ok(result) => {
            let status = result
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("unknown");
            v.check_bool(
                "neural_api_health",
                status == "alive",
                &format!("Neural API health: {status}"),
            );

            let mode = result
                .get("mode")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown");
            v.check_bool(
                "neural_api_mode",
                mode == "Bootstrap" || mode == "Coordinated",
                &format!("Neural API mode: {mode} (Bootstrap or Coordinated)"),
            );
        }
        Err(e) => {
            v.check_bool("neural_api_health", false, &format!("health.check: {e}"));
        }
    }
}

fn phase_announce(v: &mut ValidationResult) {
    let announcements = vec![
        serde_json::json!({
            "primal": "beardog",
            "socket": "/run/user/1000/biomeos/beardog-default.sock",
            "capabilities": ["security", "crypto", "trust"],
            "methods": ["health.check", "crypto.sign_ed25519", "crypto.verify_ed25519"],
            "signal_tiers": ["tower"],
            "version": "0.9.0"
        }),
        serde_json::json!({
            "primal": "sweetgrass",
            "socket": "/run/user/1000/biomeos/sweetgrass.sock",
            "capabilities": ["attribution", "braid", "convergence", "provenance"],
            "methods": ["health.check", "braid.create", "braid.list", "convergence.check"],
            "signal_tiers": ["nest"],
            "version": "0.8.0"
        }),
    ];

    let mut total_caps = 0u32;
    for announcement in &announcements {
        let primal = announcement["primal"].as_str().unwrap_or("?");
        match neural_rpc("primal.announce", announcement) {
            Ok(result) => {
                let caps = result
                    .get("capabilities_registered")
                    .and_then(|c| c.as_u64())
                    .unwrap_or(0);
                total_caps += caps as u32;
                v.check_bool(
                    &format!("announce_{primal}"),
                    caps > 0,
                    &format!("{primal}: {caps} capabilities registered"),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("announce_{primal}"),
                    false,
                    &format!("primal.announce({primal}): {e}"),
                );
            }
        }
    }

    v.check_bool(
        "total_capabilities_registered",
        total_caps >= 5,
        &format!("{total_caps} total capabilities from announcements"),
    );
}

fn phase_capability_routing(v: &mut ValidationResult) {
    let call_params = serde_json::json!({
        "capability": "crypto",
        "operation": "health.check",
        "args": {}
    });

    match neural_rpc("capability.call", &call_params) {
        Ok(result) => {
            let primal = result
                .get("primal")
                .and_then(|p| p.as_str())
                .unwrap_or("unknown");
            v.check_bool(
                "capability_call_routes",
                primal == "beardog",
                &format!("capability.call(crypto) → {primal}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "capability_call_routes",
                false,
                &format!("capability.call: {e}"),
            );
        }
    }

    match neural_rpc("capability.discover", &serde_json::json!({"capability": "crypto"})) {
        Ok(result) => {
            let has_provider = result.get("providers").is_some()
                || result.get("primals").is_some()
                || result.get("primal").is_some()
                || result.get("socket").is_some()
                || result.get("primary_endpoint").is_some();
            v.check_bool(
                "capability_discover",
                has_provider,
                "capability.discover(crypto) returns provider info",
            );
        }
        Err(e) => {
            v.check_bool(
                "capability_discover",
                false,
                &format!("capability.discover: {e}"),
            );
        }
    }
}

fn phase_graph_execution(v: &mut ValidationResult) {
    match neural_rpc("graph.list", &serde_json::json!({})) {
        Ok(result) => {
            let count = if let Some(arr) = result.as_array() {
                arr.len()
            } else {
                0
            };
            v.check_bool(
                "graph_list_populated",
                count >= 10,
                &format!("graph.list: {count} graphs loaded"),
            );
        }
        Err(e) => {
            v.check_bool("graph_list_populated", false, &format!("graph.list: {e}"));
        }
    }

    let graph_id = "tower_health";
    match neural_rpc("graph.execute", &serde_json::json!({"graph_id": graph_id})) {
        Ok(result) => {
            let eid = result
                .get("execution_id")
                .and_then(|e| e.as_str())
                .unwrap_or("");
            v.check_bool(
                "graph_execute_returns_id",
                !eid.is_empty(),
                &format!("graph.execute({graph_id}) → execution_id={eid}"),
            );

            std::thread::sleep(Duration::from_millis(500));

            match neural_rpc("graph.status", &serde_json::json!({"execution_id": eid})) {
                Ok(status) => {
                    let state = status
                        .get("state")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown");
                    let duration_ms = status
                        .get("duration_ms")
                        .and_then(|d| d.as_u64())
                        .unwrap_or(0);
                    let has_timing = duration_ms > 0 || state == "completed" || state == "failed";
                    v.check_bool(
                        "graph_status_has_timing",
                        has_timing,
                        &format!(
                            "graph.status: state={state}, duration_ms={duration_ms}"
                        ),
                    );

                    let has_node_info = status.get("completed_nodes").is_some()
                        || status.get("failed_nodes").is_some();
                    v.check_bool(
                        "graph_status_has_node_info",
                        has_node_info,
                        "graph.status reports per-node execution info",
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "graph_status_has_timing",
                        false,
                        &format!("graph.status: {e}"),
                    );
                }
            }
        }
        Err(e) => {
            v.check_bool(
                "graph_execute_returns_id",
                false,
                &format!("graph.execute: {e}"),
            );
        }
    }
}

fn phase_repeated_execution(v: &mut ValidationResult) {
    let graph_id = "tower_health";
    let n = 10;
    let start = Instant::now();
    let mut exec_ids = Vec::new();

    for _ in 0..n {
        match neural_rpc("graph.execute", &serde_json::json!({"graph_id": graph_id})) {
            Ok(result) => {
                if let Some(eid) = result.get("execution_id").and_then(|e| e.as_str()) {
                    exec_ids.push(eid.to_string());
                }
            }
            Err(_) => {}
        }
    }

    let elapsed = start.elapsed();
    v.check_bool(
        "repeated_execute_throughput",
        exec_ids.len() >= n / 2,
        &format!(
            "{}/{n} executions in {:.1}ms ({:.1}ms/exec)",
            exec_ids.len(),
            elapsed.as_secs_f64() * 1000.0,
            elapsed.as_secs_f64() * 1000.0 / n as f64
        ),
    );

    std::thread::sleep(Duration::from_secs(1));

    let mut durations = Vec::new();
    for eid in &exec_ids {
        if let Ok(status) = neural_rpc("graph.status", &serde_json::json!({"execution_id": eid})) {
            if let Some(d) = status.get("duration_ms").and_then(|d| d.as_u64()) {
                durations.push(d);
            }
        }
    }

    v.check_bool(
        "repeated_execution_timing_collected",
        durations.len() >= n / 2,
        &format!(
            "{}/{} executions have timing data (mean={:.1}ms)",
            durations.len(),
            exec_ids.len(),
            if durations.is_empty() {
                0.0
            } else {
                durations.iter().sum::<u64>() as f64 / durations.len() as f64
            }
        ),
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp118 — Neural API Graph Execution")
        .with_provenance("exp118_neural_api_graph_execution", "2026-08-08")
        .run(
            "Neural API lifecycle: riboCipher → announce → route → execute → trace",
            |v| {
                v.section("Phase 1: Neural API connectivity (riboCipher dual-lane)");
                phase_connectivity(v);

                v.section("Phase 2: Primal self-announcement (graph of self)");
                phase_announce(v);

                v.section("Phase 3: Capability routing (semantic forwarding)");
                phase_capability_routing(v);

                v.section("Phase 4: Graph execution lifecycle");
                phase_graph_execution(v);

                v.section("Phase 5: Repeated execution (trace accumulation)");
                phase_repeated_execution(v);
            },
        );
}
