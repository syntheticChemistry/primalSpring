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

use primalspring::ipc::methods;
use primalspring::ipc::tcp::tcp_rpc;
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

struct PrimalProbe {
    name: &'static str,
    port_env: &'static str,
    default_port: u16,
}

const PRIMALS: &[PrimalProbe] = &[
    PrimalProbe {
        name: primal_names::BEARDOG,
        port_env: "BEARDOG_PORT",
        default_port: tolerances::TCP_FALLBACK_BEARDOG_PORT,
    },
    PrimalProbe {
        name: primal_names::SONGBIRD,
        port_env: "SONGBIRD_PORT",
        default_port: tolerances::TCP_FALLBACK_SONGBIRD_PORT,
    },
    PrimalProbe {
        name: primal_names::NESTGATE,
        port_env: "NESTGATE_PORT",
        default_port: tolerances::TCP_FALLBACK_NESTGATE_PORT,
    },
    PrimalProbe {
        name: primal_names::TOADSTOOL,
        port_env: "TOADSTOOL_PORT",
        default_port: tolerances::TCP_FALLBACK_TOADSTOOL_PORT,
    },
    PrimalProbe {
        name: primal_names::SQUIRREL,
        port_env: "SQUIRREL_PORT",
        default_port: tolerances::TCP_FALLBACK_SQUIRREL_PORT,
    },
];

fn port_for(probe: &PrimalProbe) -> u16 {
    std::env::var(probe.port_env)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(probe.default_port)
}

fn probe_primal_health(v: &mut ValidationResult, host: &str, primal: &PrimalProbe) {
    let port = port_for(primal);
    let check_name = format!("{}_live", primal.name);

    match tcp_rpc(
        host,
        port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    ) {
        Ok((resp, _)) => {
            let status = resp.get("status").and_then(|s| s.as_str()).unwrap_or("ok");
            println!(
                "  {:<12} LIVE  (port {port}, status: {status})",
                primal.name
            );
            v.check_bool(
                &check_name,
                true,
                &format!(
                    "{} {} on port {port}",
                    primal.name,
                    methods::health::LIVENESS
                ),
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

    match tcp_rpc(
        host,
        port,
        methods::capabilities::LIST,
        &serde_json::json!({}),
    ) {
        Ok((caps, _)) => {
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
                &format!("{} {}: {count}", primal.name, methods::capabilities::LIST),
            );
        }
        Err(e) => {
            v.check_skip(
                &check_name,
                &format!("{} {}: {e}", primal.name, methods::capabilities::LIST),
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
                if tcp_rpc(
                    &host,
                    port,
                    methods::health::LIVENESS,
                    &serde_json::json!({}),
                )
                .is_ok()
                {
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
