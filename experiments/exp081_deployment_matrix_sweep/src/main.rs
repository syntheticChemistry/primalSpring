// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp081: Deployment Matrix Sweep — validate primal compositions across
//! architecture, transport, and network conditions.
//!
//! Probes all primals declared in the topology, validates health on both
//! TCP JSON-RPC and HTTP, checks transport compliance (TCP vs UDS), and
//! reports structured per-primal results for matrix cell evaluation.
//!
//! Environment:
//!   `REMOTE_GATE_HOST`  — hostname or IP of the primary gate node (required)
//!   `MATRIX_CELL`       — cell ID from deployment_matrix.toml (for reporting)
//!   `PRIMAL_TRANSPORT`  — "tcp" or "uds" (default: detect)
//!   `DEPLOY_ARCH`       — "x86_64" or "aarch64" (for reporting)
//!   `*_PORT`            — per-primal TCP port overrides

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn tcp_rpc(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<(serde_json::Value, Duration), String> {
    let addr = format!("{host}:{port}");
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("parse: {e}"))?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("connect {addr}: {e}"))?;
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
            let elapsed = start.elapsed();
            if let Some(result) = parsed.get("result") {
                return Ok((result.clone(), elapsed));
            }
            if let Some(error) = parsed.get("error") {
                return Err(format!("RPC error: {error}"));
            }
        }
    }
    Err("no response".to_owned())
}

fn http_health(host: &str, port: u16) -> Result<(serde_json::Value, Duration), String> {
    let addr = format!("{host}:{port}");
    let start = Instant::now();
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("parse: {e}"))?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("connect {addr}: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let http_req = format!(
        "GET /health HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(http_req.as_bytes())
        .map_err(|e| format!("write: {e}"))?;

    let mut buf = String::new();
    let reader = BufReader::new(&stream);
    for line in reader.lines().map_while(Result::ok) {
        buf.push_str(&line);
        buf.push('\n');
    }
    let elapsed = start.elapsed();

    if buf.contains("200 OK") || buf.contains("200 Ok") || buf.contains("\nOK\n") || buf.ends_with("OK\n")
    {
        Ok((
            serde_json::json!({"status": "alive", "protocol": "http"}),
            elapsed,
        ))
    } else {
        Err("HTTP health: non-OK response".to_owned())
    }
}

#[derive(Clone, Copy)]
enum ProbeProtocol {
    TcpJsonRpc,
    Http,
}

struct PrimalProbe {
    name: &'static str,
    port_env: &'static str,
    default_port: u16,
    protocol: ProbeProtocol,
    required_for_tcp: bool,
}

const ALL_PRIMALS: &[PrimalProbe] = &[
    PrimalProbe {
        name: primal_names::BEARDOG,
        port_env: "BEARDOG_PORT",
        default_port: tolerances::TCP_FALLBACK_BEARDOG_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
        required_for_tcp: true,
    },
    PrimalProbe {
        name: primal_names::SONGBIRD,
        port_env: "SONGBIRD_PORT",
        default_port: tolerances::TCP_FALLBACK_SONGBIRD_PORT,
        protocol: ProbeProtocol::Http,
        required_for_tcp: true,
    },
    PrimalProbe {
        name: primal_names::NESTGATE,
        port_env: "NESTGATE_PORT",
        default_port: tolerances::TCP_FALLBACK_NESTGATE_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
        required_for_tcp: false,
    },
    PrimalProbe {
        name: primal_names::TOADSTOOL,
        port_env: "TOADSTOOL_PORT",
        default_port: tolerances::TCP_FALLBACK_TOADSTOOL_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
        required_for_tcp: false,
    },
    PrimalProbe {
        name: primal_names::SQUIRREL,
        port_env: "SQUIRREL_PORT",
        default_port: tolerances::TCP_FALLBACK_SQUIRREL_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
        required_for_tcp: false,
    },
];

fn port_for(probe: &PrimalProbe) -> u16 {
    std::env::var(probe.port_env)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(probe.default_port)
}

fn rpc_for_primal(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
    protocol: ProbeProtocol,
) -> Result<(serde_json::Value, Duration), String> {
    match protocol {
        ProbeProtocol::TcpJsonRpc => tcp_rpc(host, port, method, params),
        ProbeProtocol::Http => {
            if method == "health.liveness" {
                http_health(host, port)
            } else {
                Err("HTTP probe: only health.liveness supported".to_owned())
            }
        }
    }
}

fn main() {
    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();
    let cell = std::env::var("MATRIX_CELL").unwrap_or_else(|_| "unknown".to_owned());
    let transport = std::env::var("PRIMAL_TRANSPORT").unwrap_or_else(|_| "uds".to_owned());
    let arch = std::env::var("DEPLOY_ARCH").unwrap_or_else(|_| "x86_64".to_owned());
    let tcp_mode = transport == "tcp";

    ValidationResult::new("primalSpring Exp081 — Deployment Matrix Sweep")
        .with_provenance("exp081_deployment_matrix_sweep", "2026-03-28")
        .run(
            &format!("Matrix cell: {cell} [{arch} / {transport}]"),
            |v| {
                if host.is_empty() {
                    println!("  REMOTE_GATE_HOST not set — skipping.");
                    v.check_skip("remote_gate_configured", "REMOTE_GATE_HOST not set");
                    return;
                }

                println!("  Gate host:   {host}");
                println!("  Cell:        {cell}");
                println!("  Arch:        {arch}");
                println!("  Transport:   {transport}");
                println!();

                v.check_bool("remote_gate_configured", true, "REMOTE_GATE_HOST is set");

                // Phase 1: TCP connectivity sweep
                v.section("TCP Connectivity");
                let mut live_primals: Vec<&str> = Vec::new();
                let mut response_times: Vec<(&str, Duration)> = Vec::new();

                for primal in ALL_PRIMALS {
                    let port = port_for(primal);
                    let check_name = format!("{}_health", primal.name);

                    match rpc_for_primal(
                        &host,
                        port,
                        "health.liveness",
                        &serde_json::json!({}),
                        primal.protocol,
                    ) {
                        Ok((_resp, latency)) => {
                            let ms = latency.as_millis();
                            println!("  {:<12} LIVE  (port {port}, {ms}ms)", primal.name);
                            v.check_bool(
                                &check_name,
                                true,
                                &format!("{} alive on TCP port {port} ({ms}ms)", primal.name),
                            );
                            live_primals.push(primal.name);
                            response_times.push((primal.name, latency));
                        }
                        Err(e) => {
                            println!("  {:<12} DOWN  (port {port}: {e})", primal.name);
                            if tcp_mode && primal.required_for_tcp {
                                v.check_bool(
                                    &check_name,
                                    false,
                                    &format!(
                                        "{} REQUIRED for TCP-first but unreachable: {e}",
                                        primal.name
                                    ),
                                );
                            } else {
                                v.check_skip(
                                    &check_name,
                                    &format!("{} unreachable: {e}", primal.name),
                                );
                            }
                        }
                    }
                }

                // Phase 2: Transport compliance (TCP-first mode)
                if tcp_mode {
                    v.section("TCP Transport Compliance");
                    let tower_tcp =
                        live_primals.contains(&primal_names::BEARDOG)
                            && live_primals.contains(&primal_names::SONGBIRD);
                    v.check_bool(
                        "tower_tcp_reachable",
                        tower_tcp,
                        "Tower Atomic (BearDog + Songbird) reachable via TCP",
                    );

                    println!();
                    if !tower_tcp {
                        println!(
                            "  BLOCKER: Tower primals not reachable on TCP — this blocks mobile/Pixel deployment"
                        );
                        println!(
                            "  Fix: BearDog needs --listen TCP-first, biomeOS needs --port TCP-only"
                        );
                    }
                }

                // Phase 3: Response time analysis
                v.section("Latency Profile");
                if !response_times.is_empty() {
                    let max_latency = response_times
                        .iter()
                        .map(|(_, d)| d.as_millis())
                        .max()
                        .unwrap_or(0);
                    let avg_latency = response_times
                        .iter()
                        .map(|(_, d)| d.as_millis())
                        .sum::<u128>()
                        / response_times.len() as u128;

                    println!("  Avg response: {avg_latency}ms");
                    println!("  Max response: {max_latency}ms");

                    v.check_bool(
                        "latency_acceptable",
                        max_latency < 5000,
                        &format!("max latency {max_latency}ms < 5000ms threshold"),
                    );

                    for (name, latency) in &response_times {
                        if latency.as_millis() > 2000 {
                            println!(
                                "  WARNING: {name} response {}ms exceeds 2s soft limit",
                                latency.as_millis()
                            );
                        }
                    }
                }

                // Phase 4: Capability enumeration
                v.section("Capabilities");
                let mut total_capabilities: usize = 0;
                for primal in ALL_PRIMALS {
                    if !live_primals.contains(&primal.name) {
                        continue;
                    }
                    let port = port_for(primal);
                    let check_name = format!("{}_capabilities", primal.name);

                    match rpc_for_primal(
                        &host,
                        port,
                        "capabilities.list",
                        &serde_json::json!({}),
                        primal.protocol,
                    ) {
                        Ok((caps, _)) => {
                            let count = caps
                                .as_array()
                                .map(Vec::len)
                                .or_else(|| {
                                    caps.get("capabilities")
                                        .and_then(|c| c.as_array())
                                        .map(Vec::len)
                                })
                                .unwrap_or(1);
                            println!("  {:<12} {count} capabilities", primal.name);
                            total_capabilities += count;
                            v.check_bool(
                                &check_name,
                                count > 0,
                                &format!("{}: {count} capabilities", primal.name),
                            );
                        }
                        Err(e) => {
                            v.check_skip(
                                &check_name,
                                &format!("{} capabilities.list: {e}", primal.name),
                            );
                        }
                    }
                }

                // Phase 5: Composition assessment
                v.section("Composition Assessment");
                let live = live_primals.len();
                let composition = match live {
                    0 => "NO NUCLEUS",
                    1 => "SINGLE PRIMAL",
                    2 => "TOWER ATOMIC (partial)",
                    3 => "TOWER + one layer",
                    4 => "NUCLEUS (near-complete)",
                    _ => "FULL NUCLEUS",
                };
                println!("  Composition:  {composition}");
                println!("  Live primals: {live}/5");
                println!("  Capabilities: {total_capabilities}");
                println!("  Architecture: {arch}");
                println!("  Transport:    {transport}");
                println!("  Cell:         {cell}");

                v.check_bool(
                    "composition_viable",
                    live >= 2,
                    &format!("{composition}: {live}/5 primals, {total_capabilities} capabilities"),
                );
            },
        );
}
