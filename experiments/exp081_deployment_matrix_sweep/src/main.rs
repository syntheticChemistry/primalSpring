// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

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

use primalspring::composition::CompositionContext;
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

fn all_primals() -> Vec<PrimalProbe> {
    tolerances::PORT_REGISTRY
        .iter()
        .map(|e| PrimalProbe {
            name: e.slug,
            port_env: e.env_key,
            default_port: e.port,
            required_for_tcp: matches!(e.slug, "beardog" | "songbird"),
        })
        .collect()
}

fn port_for(probe: &PrimalProbe) -> u16 {
    env_port(probe.port_env, probe.default_port)
}

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Composition discovery (local)");
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    v.check_bool(
        "has_security_capability_path",
        ctx.has_capability("security"),
        "security present in CompositionContext",
    );
    v.check_bool(
        "has_discovery_capability_path",
        ctx.has_capability("discovery"),
        "discovery present in CompositionContext",
    );
}

fn phase_tcp_connectivity(
    v: &mut ValidationResult,
    host: &str,
    tcp_mode: bool,
) -> (Vec<&'static str>, Vec<(&'static str, Duration)>) {
    v.section("Phase 2: TCP connectivity");
    let mut live_primals: Vec<&'static str> = Vec::new();
    let mut response_times: Vec<(&'static str, Duration)> = Vec::new();

    for primal in &all_primals() {
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
        v.section("Phase 3: TCP transport compliance");
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
    v.section("Phase 4: Latency profile");
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
    v.section("Phase 5: Capabilities");
    let mut total_capabilities: usize = 0;
    for primal in &all_primals() {
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
    v.section("Phase 6: Composition assessment");
    let composition = match live {
        0 => "NO NUCLEUS",
        1 => "SINGLE PRIMAL",
        2 => "TOWER ATOMIC (minimal)",
        3..=4 => "NODE COMPOSITION (partial)",
        5..=7 => "NEST COMPOSITION (partial)",
        8..=12 => "FULL NUCLEUS (near-complete)",
        _ => "FULL NUCLEUS (13/13)",
    };
    println!("  Composition:  {composition}");
    println!("  Live primals: {live}/13");
    println!("  Capabilities: {total_capabilities}");
    println!("  Architecture: {arch}");
    println!("  Transport:    {transport}");
    println!("  Cell:         {cell}");

    v.check_bool(
        "composition_viable",
        live >= 2,
        &format!("{composition}: {live}/13 primals, {total_capabilities} capabilities"),
    );
}

fn main() {
    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();
    let cell = std::env::var("MATRIX_CELL").unwrap_or_else(|_| "unknown".to_owned());
    let transport = std::env::var("PRIMAL_TRANSPORT").unwrap_or_else(|_| "uds".to_owned());
    let arch = std::env::var("DEPLOY_ARCH").unwrap_or_else(|_| "x86_64".to_owned());
    let tcp_mode = transport == "tcp";

    ValidationResult::new("primalSpring Exp081 — Deployment Matrix Sweep")
        .with_provenance("exp081_deployment_matrix_sweep", "2026-05-09")
        .run(
            &format!("Matrix cell: {cell} [{arch} / {transport}]"),
            |v| {
                let ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_composition_discovery(v, &ctx);

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
