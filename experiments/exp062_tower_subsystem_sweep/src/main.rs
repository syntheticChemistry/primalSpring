// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp062: Tower Subsystem Sweep — probe every songbird JSON-RPC method.
//!
//! Spawns a Tower atomic via the harness and systematically calls every
//! known songbird subsystem method (Tor, STUN, BirdSong, Onion,
//! Federation, Discovery). Reports each subsystem as UP / DEGRADED / DOWN
//! with latency. Like exp051 (socket discovery sweep) but for RPC methods.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

use primalspring::coordination::AtomicType;
use primalspring::harness::AtomicHarness;
use primalspring::validation::ValidationResult;

struct ProbeResult {
    method: &'static str,
    subsystem: &'static str,
    status: ProbeStatus,
    latency: Duration,
    detail: String,
}

enum ProbeStatus {
    Up,
    Degraded,
    Down,
}

impl ProbeStatus {
    const fn label(&self) -> &'static str {
        match self {
            Self::Up => "UP",
            Self::Degraded => "DEGRADED",
            Self::Down => "DOWN",
        }
    }
}

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

fn probe(
    socket: &Path,
    method: &'static str,
    subsystem: &'static str,
    params: &serde_json::Value,
) -> ProbeResult {
    let start = Instant::now();
    let result = rpc_call(socket, method, params);
    let latency = start.elapsed();

    match result {
        Ok(v) => ProbeResult {
            method,
            subsystem,
            status: ProbeStatus::Up,
            latency,
            detail: v.to_string().chars().take(120).collect(),
        },
        Err(e) if e.contains("Method not found") => ProbeResult {
            method,
            subsystem,
            status: ProbeStatus::Degraded,
            latency,
            detail: "method not registered".to_owned(),
        },
        Err(e) => ProbeResult {
            method,
            subsystem,
            status: ProbeStatus::Down,
            latency,
            detail: e.chars().take(120).collect(),
        },
    }
}

const PROBES: &[(&str, &str, &str)] = &[
    ("health.liveness", "core", "{}"),
    ("capabilities.list", "core", "{}"),
    ("capability.list", "core", "{}"),
    ("primal.capabilities", "core", "{}"),
    ("discovery.find_primals", "discovery", "{}"),
    ("stun.get_public_address", "stun", "{}"),
    ("stun.detect_nat_type", "stun", "{}"),
    (
        "birdsong.generate_encrypted_beacon",
        "birdsong",
        r#"{"family_id":"sweep-test","node_id":"sweep-test","capabilities":["security"]}"#,
    ),
    ("onion.status", "onion", "{}"),
    ("onion.start", "onion", r#"{"family_id":"sweep-test"}"#),
    ("tor.status", "tor", "{}"),
    ("tor.connect", "tor", r#"{"address":"example.com:443"}"#),
    ("songbird.federation.peers", "federation", "{}"),
    ("songbird.federation.status", "federation", "{}"),
];

fn main() {
    let graphs_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../graphs");
    let family_id = format!("exp062-sweep-{}", std::process::id());

    ValidationResult::run_experiment(
        "primalSpring Exp062 — Tower Subsystem Sweep",
        "primalSpring Exp062: Comprehensive songbird subsystem capability probe",
        |v| {
            let running = match AtomicHarness::new(AtomicType::Tower)
                .start_with_neural_api(&family_id, &graphs_dir)
            {
                Ok(r) => r,
                Err(e) => {
                    v.check_bool("harness_start", false, &format!("failed to start: {e}"));
                    v.finish();
                    std::process::exit(v.exit_code());
                }
            };

            let Some(songbird_socket) = running
                .socket_for("discovery")
                .or_else(|| running.socket_for_primal("songbird"))
            else {
                v.check_bool("songbird_socket", false, "songbird socket not found");
                v.finish();
                std::process::exit(v.exit_code());
            };

            let mut results = Vec::new();
            for &(method, subsystem, params_str) in PROBES {
                let params: serde_json::Value = match serde_json::from_str(params_str) {
                    Ok(p) => p,
                    Err(e) => {
                        v.check_bool(
                            "probe_params_json",
                            false,
                            &format!("invalid JSON for {method}: {e}"),
                        );
                        v.finish();
                        std::process::exit(v.exit_code());
                    }
                };
                results.push(probe(songbird_socket, method, subsystem, &params));
            }

            let up_count = results
                .iter()
                .filter(|r| matches!(r.status, ProbeStatus::Up))
                .count();
            let degraded = results
                .iter()
                .filter(|r| matches!(r.status, ProbeStatus::Degraded))
                .count();
            let down = results
                .iter()
                .filter(|r| matches!(r.status, ProbeStatus::Down))
                .count();

            println!("\n  ╔══════════════════════════════════════════════════════════════╗");
            println!(
                "  ║  Tower Subsystem Sweep — {up_count} UP / {degraded} DEGRADED / {down} DOWN  ║"
            );
            println!("  ╚══════════════════════════════════════════════════════════════╝\n");

            for r in &results {
                println!(
                    "  [{:>8}] {:>35} ({:>5}ms) {}",
                    r.status.label(),
                    r.method,
                    r.latency.as_millis(),
                    r.detail.chars().take(60).collect::<String>()
                );
            }

            v.check_bool(
                "tower_subsystem_sweep_ran",
                !results.is_empty(),
                "sweep probed all known subsystem methods",
            );

            v.check_bool(
                "core_health_up",
                results
                    .iter()
                    .any(|r| r.method == "health.liveness" && matches!(r.status, ProbeStatus::Up)),
                "health.liveness should respond",
            );

            v.check_count("total_probes", results.len(), PROBES.len());

            let subsystems: std::collections::HashSet<&str> =
                results.iter().map(|r| r.subsystem).collect();
            println!("\n  Subsystems probed: {}", subsystems.len());
            for ss in &subsystems {
                let ss_up = results
                    .iter()
                    .filter(|r| r.subsystem == *ss && matches!(r.status, ProbeStatus::Up))
                    .count();
                let ss_total = results.iter().filter(|r| r.subsystem == *ss).count();
                println!("    {ss}: {ss_up}/{ss_total} methods UP");
            }
        },
    );
}
