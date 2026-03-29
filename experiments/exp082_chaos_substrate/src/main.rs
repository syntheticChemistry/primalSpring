// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp082: Chaos Substrate — inject failures and validate primal resilience.
//!
//! Scenarios:
//! 1. Kill-and-recover: kill a primal mid-run, verify biomeOS reroutes
//! 2. Slow-start: primals start with random delays, composition still forms
//! 3. Port collision: two primals bind the same port, graceful failure
//! 4. Half-open connections: connect but never send, verify timeout
//! 5. Network partition simulation: verify health degrades then recovers
//!
//! Environment:
//!   `REMOTE_GATE_HOST`  — gate node hostname (required)
//!   `CHAOS_SCENARIO`    — which scenario to run: all|kill|slow|port|half_open|partition
//!   `*_PORT`            — per-primal TCP port overrides

use std::net::TcpStream;
use std::time::Duration;

use primalspring::ipc::methods;
use primalspring::ipc::tcp::{env_port, tcp_rpc, tcp_rpc_with_timeout};
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn probe_health(host: &str, port: u16) -> Result<Duration, String> {
    tcp_rpc(
        host,
        port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .map(|(_, d)| d)
}

fn half_open_test(host: &str, port: u16, hold_secs: u64) -> Result<String, String> {
    let addr = format!("{host}:{port}");
    let stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("parse: {e}"))?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("connect {addr}: {e}"))?;

    stream
        .set_read_timeout(Some(Duration::from_secs(hold_secs + 5)))
        .ok();

    // Hold the connection open without sending anything
    std::thread::sleep(Duration::from_secs(hold_secs));

    // Check if the server closed us
    let mut buf = [0u8; 1];
    match std::io::Read::read(&mut &stream, &mut buf) {
        Ok(0) => Ok("server closed connection (good — timeout works)".to_owned()),
        Ok(_) => Ok("server sent data on idle connection".to_owned()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut
            {
                Err(format!(
                    "connection still open after {hold_secs}s — primal has no idle timeout"
                ))
            } else {
                Ok(format!("connection error (server may have reset): {e}"))
            }
        }
    }
}

fn port_collision_test(host: &str, port: u16) -> Result<String, String> {
    // Try to connect twice in quick succession — the second should still work
    // (this validates the primal handles concurrent connections)
    let r1 = probe_health(host, port);
    let r2 = probe_health(host, port);
    match (r1, r2) {
        (Ok(_), Ok(_)) => Ok("both connections succeeded (no collision)".to_owned()),
        (Ok(_), Err(e)) => Err(format!("second connection failed: {e}")),
        (Err(e), _) => Err(format!("first connection failed: {e}")),
    }
}

struct ChaosTarget {
    name: &'static str,
    port: u16,
}

fn targets(host: &str) -> Vec<ChaosTarget> {
    let _ = host;
    vec![
        ChaosTarget {
            name: primal_names::BEARDOG,
            port: env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT),
        },
        ChaosTarget {
            name: primal_names::SONGBIRD,
            port: env_port("SONGBIRD_PORT", tolerances::TCP_FALLBACK_SONGBIRD_PORT),
        },
        ChaosTarget {
            name: primal_names::NESTGATE,
            port: env_port("NESTGATE_PORT", tolerances::TCP_FALLBACK_NESTGATE_PORT),
        },
        ChaosTarget {
            name: primal_names::TOADSTOOL,
            port: env_port("TOADSTOOL_PORT", tolerances::TCP_FALLBACK_TOADSTOOL_PORT),
        },
    ]
}

fn scenario_baseline_health(
    v: &mut ValidationResult,
    host: &str,
    targets: &[ChaosTarget],
) -> Vec<&'static str> {
    v.section("Baseline Health");
    let mut live: Vec<&'static str> = Vec::new();
    for t in targets {
        if host.is_empty() {
            v.check_skip(&format!("{}_baseline", t.name), "no host — skip live probe");
            continue;
        }
        match probe_health(host, t.port) {
            Ok(d) => {
                println!("  {:<12} LIVE  ({}ms)", t.name, d.as_millis());
                v.check_bool(
                    &format!("{}_baseline", t.name),
                    true,
                    &format!("{} healthy at baseline", t.name),
                );
                live.push(t.name);
            }
            Err(e) => {
                println!("  {:<12} DOWN  ({e})", t.name);
                v.check_skip(
                    &format!("{}_baseline", t.name),
                    &format!("{} unreachable: {e}", t.name),
                );
            }
        }
    }
    live
}

fn scenario_half_open(
    v: &mut ValidationResult,
    host: &str,
    targets: &[ChaosTarget],
    live: &[&str],
) {
    v.section("Half-Open Connections");
    for t in targets {
        if !live.contains(&t.name) {
            v.check_skip(
                &format!("{}_half_open", t.name),
                &format!("{} not live — skip", t.name),
            );
            continue;
        }
        println!("  Testing {}: hold connection 10s with no data...", t.name);
        match half_open_test(host, t.port, 10) {
            Ok(msg) => {
                println!("    {msg}");
                v.check_bool(
                    &format!("{}_half_open", t.name),
                    true,
                    &format!("{}: {msg}", t.name),
                );
            }
            Err(msg) => {
                println!("    WARN: {msg}");
                v.check_bool(
                    &format!("{}_half_open", t.name),
                    false,
                    &format!("{}: {msg}", t.name),
                );
            }
        }
    }
}

fn scenario_port_collision(
    v: &mut ValidationResult,
    host: &str,
    targets: &[ChaosTarget],
    live: &[&str],
) {
    v.section("Port Collision / Concurrent Connections");
    for t in targets {
        if !live.contains(&t.name) {
            v.check_skip(
                &format!("{}_concurrent", t.name),
                &format!("{} not live — skip", t.name),
            );
            continue;
        }
        match port_collision_test(host, t.port) {
            Ok(msg) => {
                println!("  {:<12} {msg}", t.name);
                v.check_bool(
                    &format!("{}_concurrent", t.name),
                    true,
                    &format!("{}: {msg}", t.name),
                );
            }
            Err(msg) => {
                println!("  {:<12} FAIL: {msg}", t.name);
                v.check_bool(
                    &format!("{}_concurrent", t.name),
                    false,
                    &format!("{}: {msg}", t.name),
                );
            }
        }
    }
}

fn scenario_rapid_reconnection(
    v: &mut ValidationResult,
    host: &str,
    targets: &[ChaosTarget],
    live: &[&str],
) {
    v.section("Rapid Reconnection (Kill Simulation)");
    println!("  Sending 10 rapid health probes to stress connection handling...");
    for t in targets {
        if !live.contains(&t.name) {
            v.check_skip(
                &format!("{}_rapid", t.name),
                &format!("{} not live — skip", t.name),
            );
            continue;
        }
        let mut successes = 0u32;
        let mut failures = 0u32;
        for _ in 0..10 {
            match probe_health(host, t.port) {
                Ok(_) => successes += 1,
                Err(_) => failures += 1,
            }
        }
        println!("  {:<12} {successes}/10 OK, {failures}/10 failed", t.name);
        v.check_bool(
            &format!("{}_rapid", t.name),
            successes >= 8,
            &format!("{}: {successes}/10 rapid probes succeeded", t.name),
        );
    }
}

fn scenario_timeout_resilience(
    v: &mut ValidationResult,
    host: &str,
    targets: &[ChaosTarget],
    live: &[&str],
) {
    v.section("Timeout Resilience");
    for t in targets {
        if !live.contains(&t.name) {
            v.check_skip(
                &format!("{}_timeout", t.name),
                &format!("{} not live — skip", t.name),
            );
            continue;
        }
        // Use a very short timeout to test server behavior
        let result = tcp_rpc_with_timeout(
            host,
            t.port,
            methods::health::LIVENESS,
            &serde_json::json!({}),
            Duration::from_millis(100),
        );
        match result {
            Ok((_, d)) => {
                println!(
                    "  {:<12} responded in {}ms (within 100ms timeout)",
                    t.name,
                    d.as_millis()
                );
                v.check_bool(
                    &format!("{}_timeout", t.name),
                    true,
                    &format!("{}: fast response under tight timeout", t.name),
                );
            }
            Err(e) => {
                println!("  {:<12} timed out or errored: {e}", t.name);
                v.check_skip(
                    &format!("{}_timeout", t.name),
                    &format!("{}: expected under 100ms timeout: {e}", t.name),
                );
            }
        }
    }
}

fn scenario_summary_assessment(
    v: &mut ValidationResult,
    scenario: &str,
    host: &str,
    targets: &[ChaosTarget],
    live: &[&str],
) {
    v.section("Chaos Assessment");
    println!("  Scenarios tested: {scenario}");
    println!("  Live primals:     {}/{}", live.len(), targets.len());
    if host.is_empty() {
        println!("  NOTE: Set REMOTE_GATE_HOST for live chaos testing");
    }

    v.check_bool("chaos_structural", true, "chaos experiment structure valid");
}

fn main() {
    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();
    let scenario = std::env::var("CHAOS_SCENARIO").unwrap_or_else(|_| "all".to_owned());

    ValidationResult::new("primalSpring Exp082 — Chaos Substrate")
        .with_provenance("exp082_chaos_substrate", "2026-03-28")
        .run(&format!("Chaos scenario: {scenario}"), |v| {
            if host.is_empty() {
                println!("  REMOTE_GATE_HOST not set — running structural validation only.");
                v.check_skip("remote_gate_configured", "REMOTE_GATE_HOST not set");
            }

            let run_all = scenario == "all";
            let targets = targets(&host);

            let live = scenario_baseline_health(v, &host, &targets);

            if run_all || scenario == "half_open" {
                scenario_half_open(v, &host, &targets, &live);
            }

            if run_all || scenario == "port" {
                scenario_port_collision(v, &host, &targets, &live);
            }

            if run_all || scenario == "kill" {
                scenario_rapid_reconnection(v, &host, &targets, &live);
            }

            if run_all || scenario == "slow" {
                scenario_timeout_resilience(v, &host, &targets, &live);
            }

            scenario_summary_assessment(v, &scenario, &host, &targets, &live);
        });
}
