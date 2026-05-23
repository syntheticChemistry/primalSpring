// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Deployment Matrix — absorbed from exp081.

use crate::composition::CompositionContext;
use crate::ipc::methods;
use crate::ipc::tcp::{env_port, tcp_rpc};
use crate::primal_names;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::time::Duration;

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "deployment-matrix",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "exp081_deployment_matrix_sweep",
        provenance_date: "2026-05-09",
        description: "Deployment matrix — composition discovery, TCP probes, latency, capabilities",
    },
    run,
};

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

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let host = std::env::var(crate::env_keys::REMOTE_GATE_HOST).unwrap_or_default();
    let cell = std::env::var(crate::env_keys::MATRIX_CELL).unwrap_or_else(|_| "unknown".to_owned());
    let transport = std::env::var(crate::env_keys::PRIMAL_TRANSPORT).unwrap_or_else(|_| "uds".to_owned());
    let arch = std::env::var(crate::env_keys::DEPLOY_ARCH).unwrap_or_else(|_| "x86_64".to_owned());
    let tcp_mode = transport == "tcp";

    phase_composition_discovery(v, ctx);

    if host.is_empty() {
        v.check_skip("remote_gate_configured", "REMOTE_GATE_HOST not set");
        return;
    }

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
                v.check_bool(
                    &check_name,
                    true,
                    &format!(
                        "{} alive on TCP port {port} ({}ms)",
                        primal.name,
                        latency.as_millis()
                    ),
                );
                live_primals.push(primal.name);
                response_times.push((primal.name, latency));
            }
            Err(e) => {
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

        v.check_bool(
            "latency_acceptable",
            max_latency < 5000,
            &format!("avg {avg_latency}ms, max {max_latency}ms < 5000ms threshold"),
        );
    }
}

fn phase_capability_enumeration(
    v: &mut ValidationResult,
    host: &str,
    live_primals: &[&str],
) -> usize {
    v.section("Phase 5: Capabilities");
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
        2 => "TOWER ATOMIC (partial)",
        3 => "TOWER + one layer",
        4 => "NUCLEUS (near-complete)",
        _ => "FULL NUCLEUS",
    };

    v.check_bool(
        "composition_viable",
        live >= 2,
        &format!("{composition}: {live}/5 primals, {total_capabilities} capabilities; arch={arch} transport={transport} cell={cell}"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deployment_matrix_no_panic() {
        let mut v = ValidationResult::new("deployment-matrix");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Live scenario: failures expected without primals/REMOTE_GATE_HOST.
        // This test validates no panics and that the scenario runs to completion.
        assert!(v.evaluated() > 0, "deployment matrix should produce at least one check");
    }
}
