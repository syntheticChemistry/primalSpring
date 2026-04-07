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
//!   `MATRIX_CELL`       — cell ID from `deployment_matrix.toml` (for reporting)
//!   `PRIMAL_TRANSPORT`  — "tcp" or "uds" (default: detect)
//!   `DEPLOY_ARCH`       — "`x86_64`" or "aarch64" (for reporting)
//!   `*_PORT`            — per-primal TCP port overrides

use std::time::Duration;

use primalspring::ipc::methods;
use primalspring::ipc::tcp::{env_port, tcp_rpc};
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

struct PrimalProbe {
    name: &'static str,
    port_env: &'static str,
    default_port: u16,
    required_for_tcp: bool,
}

const ALL_PRIMALS: &[PrimalProbe] = &[
    PrimalProbe {
        name: primal_names::BEARDOG,
        port_env: "BEARDOG_PORT",
        default_port: tolerances::TCP_FALLBACK_BEARDOG_PORT,
        required_for_tcp: true,
    },
    PrimalProbe {
        name: primal_names::SONGBIRD,
        port_env: "SONGBIRD_PORT",
        default_port: tolerances::TCP_FALLBACK_SONGBIRD_PORT,
        required_for_tcp: true,
    },
    PrimalProbe {
        name: primal_names::NESTGATE,
        port_env: "NESTGATE_PORT",
        default_port: tolerances::TCP_FALLBACK_NESTGATE_PORT,
        required_for_tcp: false,
    },
    PrimalProbe {
        name: primal_names::TOADSTOOL,
        port_env: "TOADSTOOL_PORT",
        default_port: tolerances::TCP_FALLBACK_TOADSTOOL_PORT,
        required_for_tcp: false,
    },
    PrimalProbe {
        name: primal_names::SQUIRREL,
        port_env: "SQUIRREL_PORT",
        default_port: tolerances::TCP_FALLBACK_SQUIRREL_PORT,
        required_for_tcp: false,
    },
];

fn port_for(probe: &PrimalProbe) -> u16 {
    env_port(probe.port_env, probe.default_port)
}

fn phase_tcp_connectivity(
    v: &mut ValidationResult,
    host: &str,
    tcp_mode: bool,
) -> (Vec<&'static str>, Vec<(&'static str, Duration)>) {
    v.section("TCP Connectivity");
    let mut live_primals: Vec<&'static str> = Vec::new();
    let mut response_times: Vec<(&'static str, Duration)> = Vec::new();

    for primal in ALL_PRIMALS {
        let port = port_for(primal);
        let check_name = format!("{}_health", primal.name);

        match tcp_rpc(
            host,
            port,
            methods::health::LIVENESS,
            &serde_json::json!({}),
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
                    v.check_skip(&check_name, &format!("{} unreachable: {e}", primal.name));
                }
            }
        }
    }

    (live_primals, response_times)
}

fn phase_tcp_transport_compliance(v: &mut ValidationResult, tcp_mode: bool, live_primals: &[&str]) {
    if tcp_mode {
        v.section("TCP Transport Compliance");
        let tower_tcp = live_primals.contains(&primal_names::BEARDOG)
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
            println!("  Fix: BearDog needs --listen TCP-first, biomeOS needs --port TCP-only");
        }
    }
}

fn phase_latency_profile(v: &mut ValidationResult, response_times: &[(&str, Duration)]) {
    v.section("Latency Profile");
    if !response_times.is_empty() {
        let max_latency = response_times
            .iter()
            .map(|(_, d)| d.as_millis())
            .max()
            .unwrap_or(0);
        let count = response_times.len() as u128;
        let avg_latency = response_times
            .iter()
            .map(|(_, d)| d.as_millis())
            .sum::<u128>()
            / count;

        println!("  Avg response: {avg_latency}ms");
        println!("  Max response: {max_latency}ms");

        v.check_bool(
            "latency_acceptable",
            max_latency < 5000,
            &format!("max latency {max_latency}ms < 5000ms threshold"),
        );

        for (name, latency) in response_times {
            if latency.as_millis() > 2000 {
                println!(
                    "  WARNING: {name} response {}ms exceeds 2s soft limit",
                    latency.as_millis()
                );
            }
        }
    }
}

fn phase_capability_enumeration(
    v: &mut ValidationResult,
    host: &str,
    live_primals: &[&str],
) -> usize {
    v.section("Capabilities");
    let mut total_capabilities: usize = 0;
    for primal in ALL_PRIMALS {
        if !live_primals.contains(&primal.name) {
            continue;
        }
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
                    &format!("{} {}: {e}", primal.name, methods::capabilities::LIST),
                );
            }
        }
    }
    total_capabilities
}

fn phase_composition_assessment(
    v: &mut ValidationResult,
    cell: &str,
    arch: &str,
    transport: &str,
    live: usize,
    total_capabilities: usize,
) {
    v.section("Composition Assessment");
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

                let (live_primals, response_times) = phase_tcp_connectivity(v, &host, tcp_mode);

                phase_tcp_transport_compliance(v, tcp_mode, &live_primals);

                phase_latency_profile(v, &response_times);

                let total_capabilities = phase_capability_enumeration(v, &host, &live_primals);

                phase_composition_assessment(
                    v,
                    cell.as_str(),
                    arch.as_str(),
                    transport.as_str(),
                    live_primals.len(),
                    total_capabilities,
                );
            },
        );
}
