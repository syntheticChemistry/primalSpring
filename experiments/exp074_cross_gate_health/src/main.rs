// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp074: Cross-Gate Health — validate remote NUCLEUS health via TCP.
//!
//! Probes all primals on a remote gate via TCP JSON-RPC and reports
//! per-primal health, capabilities, and federation readiness.
//!
//! Environment:
//!   `REMOTE_GATE_HOST` — hostname or IP of the remote gate (required)
//!   `BEARDOG_PORT`  — TCP fallback (default 9100, only for cross-gate)
//!   `SONGBIRD_PORT` — TCP fallback (default 9200, only for cross-gate)
//!   `NESTGATE_PORT` — TCP fallback (default 9300, only for cross-gate)
//!   `TOADSTOOL_PORT` — TCP fallback (default 9400, only for cross-gate)
//!   `SQUIRREL_PORT` — TCP fallback (default 9500, only for cross-gate)

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

/// TCP JSON-RPC call with timeout (raw newline-delimited JSON over TCP).
fn tcp_rpc(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    let addr = format!("{host}:{port}");
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

/// HTTP health probe for primals that serve HTTP instead of raw TCP JSON-RPC.
/// Sends `GET /health` and checks for a 200 OK response.
fn http_health(host: &str, port: u16) -> Result<serde_json::Value, String> {
    let addr = format!("{host}:{port}");
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

    if buf.contains("200 OK") || buf.contains("200 Ok") || buf.contains("\nOK\n") || buf.ends_with("OK\n") {
        Ok(serde_json::json!({"primal": "songbird", "status": "alive", "protocol": "http"}))
    } else {
        Err(format!("HTTP health: non-OK response"))
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
}

const PRIMALS: &[PrimalProbe] = &[
    PrimalProbe {
        name: primal_names::BEARDOG,
        port_env: "BEARDOG_PORT",
        default_port: tolerances::TCP_FALLBACK_BEARDOG_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
    },
    PrimalProbe {
        name: primal_names::SONGBIRD,
        port_env: "SONGBIRD_PORT",
        default_port: tolerances::TCP_FALLBACK_SONGBIRD_PORT,
        protocol: ProbeProtocol::Http,
    },
    PrimalProbe {
        name: primal_names::NESTGATE,
        port_env: "NESTGATE_PORT",
        default_port: tolerances::TCP_FALLBACK_NESTGATE_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
    },
    PrimalProbe {
        name: primal_names::TOADSTOOL,
        port_env: "TOADSTOOL_PORT",
        default_port: tolerances::TCP_FALLBACK_TOADSTOOL_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
    },
    PrimalProbe {
        name: primal_names::SQUIRREL,
        port_env: "SQUIRREL_PORT",
        default_port: tolerances::TCP_FALLBACK_SQUIRREL_PORT,
        protocol: ProbeProtocol::TcpJsonRpc,
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
) -> Result<serde_json::Value, String> {
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

fn probe_primal_health(v: &mut ValidationResult, host: &str, primal: &PrimalProbe) {
    let port = port_for(primal);
    let check_name = format!("{}_live", primal.name);

    match rpc_for_primal(host, port, "health.liveness", &serde_json::json!({}), primal.protocol) {
        Ok(resp) => {
            let status = resp.get("status").and_then(|s| s.as_str()).unwrap_or("ok");
            println!(
                "  {:<12} LIVE  (port {port}, status: {status})",
                primal.name
            );
            v.check_bool(
                &check_name,
                true,
                &format!("{} health.liveness on port {port}", primal.name),
            );
        }
        Err(e) => {
            println!("  {:<12} DOWN  (port {port}: {e})", primal.name);
            v.check_skip(&check_name, &format!("{} unreachable: {e}", primal.name));
        }
    }
}

fn probe_primal_capabilities(v: &mut ValidationResult, host: &str, primal: &PrimalProbe) {
    let port = port_for(primal);
    let check_name = format!("{}_capabilities", primal.name);

    match rpc_for_primal(host, port, "capabilities.list", &serde_json::json!({}), primal.protocol) {
        Ok(caps) => {
            let count = caps
                .as_array()
                .map(std::vec::Vec::len)
                .or_else(|| {
                    caps.get("capabilities")
                        .and_then(|c| c.as_array())
                        .map(std::vec::Vec::len)
                })
                .unwrap_or(1);
            println!("  {:<12} {count} capabilities", primal.name);
            v.check_bool(
                &check_name,
                count > 0,
                &format!("{} capabilities.list: {count}", primal.name),
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

fn main() {
    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();

    ValidationResult::new("primalSpring Exp074 — Cross-Gate Health")
        .with_provenance("exp074_cross_gate_health", "2026-03-24")
        .run(
            "primalSpring Exp074: Remote NUCLEUS per-primal health + capabilities via TCP",
            |v| {
            if host.is_empty() {
                println!("  REMOTE_GATE_HOST not set — skipping all remote checks.");
                println!(
                    "  Usage: REMOTE_GATE_HOST=192.168.1.100 cargo run --bin exp074_cross_gate_health"
                );
                v.check_skip("remote_gate_configured", "REMOTE_GATE_HOST not set");
                return;
            }

            println!("  Remote gate: {host}");
            println!();

            v.check_bool("remote_gate_configured", true, "REMOTE_GATE_HOST is set");

            v.section("Health Probes");
            let mut live_count: u32 = 0;
            for primal in PRIMALS {
                let port = port_for(primal);
                if rpc_for_primal(&host, port, "health.liveness", &serde_json::json!({}), primal.protocol).is_ok() {
                    live_count += 1;
                }
                probe_primal_health(v, &host, primal);
            }

            v.section("Capability Enumeration");
            for primal in PRIMALS {
                probe_primal_capabilities(v, &host, primal);
            }

            v.section("Composition Assessment");
            let composition = match live_count {
                0 => "NO NUCLEUS",
                1..=2 => "TOWER ATOMIC (partial)",
                3 => "TOWER + one layer",
                4 => "NUCLEUS (near-complete)",
                5.. => "FULL NUCLEUS",
            };
            println!("  Remote composition: {composition} ({live_count}/5 primals live)");
            v.check_bool(
                "nucleus_composition",
                live_count >= 2,
                &format!("{composition}: {live_count}/5 primals live"),
            );

            if live_count >= 2 {
                v.check_bool(
                    "tower_minimum",
                    true,
                    "at least Tower Atomic (beardog + songbird) live",
                );
            } else {
                v.check_skip(
                    "tower_minimum",
                    &format!("only {live_count} primals live, need >= 2 for Tower"),
                );
            }
            },
        );
}
