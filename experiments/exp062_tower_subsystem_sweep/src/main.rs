// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp062: Tower Subsystem Sweep — probe songbird JSON-RPC methods via composition discovery.

use std::time::{Duration, Instant};

use primalspring::composition::CompositionContext;
use primalspring::ipc::IpcError;
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

fn probe(
    ctx: &mut CompositionContext,
    method: &'static str,
    subsystem: &'static str,
    params: serde_json::Value,
) -> ProbeResult {
    let start = Instant::now();
    let result = ctx.call("discovery", method, params);
    let latency = start.elapsed();

    match result {
        Ok(v) => ProbeResult {
            method,
            subsystem,
            status: ProbeStatus::Up,
            latency,
            detail: v.to_string().chars().take(120).collect(),
        },
        Err(e) if e.is_method_not_found() || degraded_message(&e) => ProbeResult {
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
            detail: format!("{e}").chars().take(120).collect(),
        },
    }
}

fn degraded_message(e: &IpcError) -> bool {
    format!("{e}").contains("Method not found")
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
    ValidationResult::new("primalSpring Exp062 — Tower Subsystem Sweep")
        .with_provenance("exp062_tower_subsystem_sweep", "2026-05-09")
        .run(
            "primalSpring Exp062: Comprehensive songbird subsystem capability probe",
            |v| {
                v.section("Phase 1: Discovery capability sweep");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();

                if !ctx.has_capability("discovery") {
                    v.check_skip(
                        "tower_subsystem_sweep_ran",
                        "discovery capability not connected",
                    );
                    return;
                }

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
                            return;
                        }
                    };
                    results.push(probe(&mut ctx, method, subsystem, params));
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
                    results.iter().any(|r| {
                        r.method == "health.liveness" && matches!(r.status, ProbeStatus::Up)
                    }),
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
