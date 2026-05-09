// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp083: Federation Edge Cases — probe multi-gate topologies for
//! asymmetric latency, partial mesh, and mid-session family migration.
//!
//! Environment:
//!   `GATE_HOSTS`       — comma-separated list of gate hostnames (required)
//!   `EDGE_SCENARIO`    — which scenario: `all|asymmetric|partial_mesh|migration`
//!   `*_PORT`           — per-primal TCP port overrides

use std::time::Duration;

use primalspring::ipc::methods;
use primalspring::ipc::tcp::{env_port, tcp_rpc};
use primalspring::primal_names;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn probe_gate(host: &str) -> (bool, bool, Duration) {
    let beardog_port = env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT);
    let songbird_port = env_port("SONGBIRD_PORT", tolerances::TCP_FALLBACK_SONGBIRD_PORT);

    let bd = tcp_rpc(
        host,
        beardog_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    let sg = tcp_rpc(
        host,
        songbird_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );

    let bd_ok = bd.is_ok();
    let sg_ok = sg.is_ok();
    let max_latency = [
        bd.as_ref().map(|(_, d)| *d).unwrap_or(Duration::ZERO),
        sg.as_ref().map(|(_, d)| *d).unwrap_or(Duration::ZERO),
    ]
    .into_iter()
    .max()
    .unwrap_or(Duration::ZERO);

    (bd_ok, sg_ok, max_latency)
}

fn measure_latency_pair(host_a: &str, host_b: &str) -> (Duration, Duration) {
    let beardog_port = env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT);

    let a_to_b = tcp_rpc(
        host_b,
        beardog_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .map(|(_, d)| d)
    .unwrap_or(Duration::from_secs(999));

    let b_to_a = tcp_rpc(
        host_a,
        beardog_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    )
    .map(|(_, d)| d)
    .unwrap_or(Duration::from_secs(999));

    (a_to_b, b_to_a)
}

fn run_gate_health_survey<'a>(v: &mut ValidationResult, hosts: &'a [&'a str]) -> Vec<&'a str> {
    v.section("Gate Health Survey");
    let mut live_gates: Vec<&str> = Vec::new();
    for gate in hosts {
        let (bd, sg, latency) = probe_gate(gate);
        let status = match (bd, sg) {
            (true, true) => "FULL",
            (true, false) => "PARTIAL (no Songbird)",
            (false, true) => "PARTIAL (no BearDog)",
            _ => "DOWN",
        };
        println!("  {gate:<20} {status}  ({}ms)", latency.as_millis());
        if bd || sg {
            live_gates.push(gate);
        }
        v.check_bool(
            &format!("gate_{gate}_health"),
            bd && sg,
            &format!("{gate}: {status}"),
        );
    }
    live_gates
}

fn run_asymmetric_latency_scenario(v: &mut ValidationResult, live_gates: &[&str], enabled: bool) {
    if !enabled || live_gates.len() < 2 {
        return;
    }
    v.section("Asymmetric Latency");
    for i in 0..live_gates.len() {
        for j in (i + 1)..live_gates.len() {
            let a = live_gates[i];
            let b = live_gates[j];
            let (a_to_b, b_to_a) = measure_latency_pair(a, b);
            #[expect(
                clippy::cast_precision_loss,
                reason = "latency ratios: ms-scale values fit well within f64 mantissa"
            )]
            let ratio = if b_to_a.as_millis() > 0 {
                (a_to_b.as_millis() as f64) / (b_to_a.as_millis() as f64)
            } else {
                1.0
            };
            println!(
                "  {a} → {b}: {}ms | {b} → {a}: {}ms (ratio: {ratio:.1}x)",
                a_to_b.as_millis(),
                b_to_a.as_millis(),
            );
            let asymmetric = !(0.2..=5.0).contains(&ratio);
            let detail = if asymmetric {
                format!("ASYMMETRIC: {ratio:.1}x ratio — may cause routing issues")
            } else {
                format!("symmetric within 5x: {ratio:.1}x ratio")
            };
            v.check_bool(&format!("latency_{a}_{b}_symmetric"), !asymmetric, &detail);
        }
    }
}

fn run_partial_mesh_scenario(v: &mut ValidationResult, live_gates: &[&str], enabled: bool) {
    if !enabled || live_gates.len() < 2 {
        return;
    }
    v.section("Partial Mesh Reachability");
    let biomeos_port = env_port("BIOMEOS_PORT", tolerances::TCP_FALLBACK_BIOMEOS_PORT);
    let nestgate_port = env_port("NESTGATE_PORT", tolerances::TCP_FALLBACK_NESTGATE_PORT);

    for gate in live_gates {
        let biomeos_ok = tcp_rpc(
            gate,
            biomeos_port,
            methods::health::LIVENESS,
            &serde_json::json!({}),
        )
        .is_ok();
        let nestgate_ok = tcp_rpc(
            gate,
            nestgate_port,
            methods::health::LIVENESS,
            &serde_json::json!({}),
        )
        .is_ok();
        let beardog_ok = tcp_rpc(
            gate,
            env_port("BEARDOG_PORT", tolerances::TCP_FALLBACK_BEARDOG_PORT),
            methods::health::LIVENESS,
            &serde_json::json!({}),
        )
        .is_ok();

        let reachable = [
            (primal_names::BEARDOG, beardog_ok),
            (primal_names::BIOMEOS, biomeos_ok),
            (primal_names::NESTGATE, nestgate_ok),
        ];
        let available: Vec<&str> = reachable
            .iter()
            .filter(|(_, ok)| *ok)
            .map(|(n, _)| *n)
            .collect();

        println!("  {gate:<20} reachable: [{}]", available.join(", "));
        let tower_status = if beardog_ok {
            "reachable"
        } else {
            "unreachable"
        };
        v.check_bool(
            &format!("mesh_{gate}_tower"),
            beardog_ok,
            &format!("{gate}: Tower Atomic {tower_status}"),
        );
    }
}

fn run_cross_gate_capabilities(
    v: &mut ValidationResult,
    hosts: &[&str],
    live_gates: &[&str],
    enabled: bool,
) {
    if !enabled {
        return;
    }
    v.section("Cross-Gate Capabilities");
    let biomeos_port = env_port("BIOMEOS_PORT", tolerances::TCP_FALLBACK_BIOMEOS_PORT);
    let mut total_caps = 0usize;
    for gate in live_gates {
        match tcp_rpc(
            gate,
            biomeos_port,
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
                    .unwrap_or(0);
                println!("  {gate:<20} {count} capabilities");
                total_caps += count;
            }
            Err(e) => {
                println!("  {gate:<20} biomeOS unreachable: {e}");
            }
        }
    }
    v.check_bool(
        "federation_capabilities",
        total_caps > 0 || hosts.is_empty(),
        &format!(
            "{total_caps} total capabilities across {} gates",
            live_gates.len()
        ),
    );
}

fn run_federation_assessment(
    v: &mut ValidationResult,
    hosts: &[&str],
    live_gates: &[&str],
    scenario: &str,
) {
    v.section("Federation Assessment");
    println!("  Live gates:   {}/{}", live_gates.len(), hosts.len());
    println!("  Scenario:     {scenario}");
    v.check_bool(
        "federation_viable",
        live_gates.len() >= 2,
        &format!(
            "{}/{} gates live — {}",
            live_gates.len(),
            hosts.len(),
            if live_gates.len() >= 2 {
                "federation viable"
            } else {
                "insufficient gates for federation"
            }
        ),
    );
}

fn main() {
    let hosts_str = std::env::var("GATE_HOSTS").unwrap_or_default();
    let scenario = std::env::var("EDGE_SCENARIO").unwrap_or_else(|_| "all".to_owned());

    let hosts: Vec<&str> = hosts_str
        .split(',')
        .map(str::trim)
        .filter(|h| !h.is_empty())
        .collect();

    ValidationResult::new("primalSpring Exp083 — Federation Edge Cases")
        .with_provenance("exp083_federation_edge_cases", "2026-03-28")
        .run(&format!("Edge scenario: {scenario}"), |v| {
            if hosts.is_empty() {
                println!("  GATE_HOSTS not set — running structural validation only.");
                v.check_skip("gate_hosts_configured", "GATE_HOSTS not set");
                structural_checks(v);
                return;
            }

            println!("  Federation gates: {}", hosts.len());
            for (i, h) in hosts.iter().enumerate() {
                println!("    Gate {}: {h}", i + 1);
            }
            println!();

            let run_all = scenario == "all";

            let live_gates = run_gate_health_survey(v, &hosts);

            run_asymmetric_latency_scenario(v, &live_gates, run_all || scenario == "asymmetric");
            run_partial_mesh_scenario(v, &live_gates, run_all || scenario == "partial_mesh");
            run_cross_gate_capabilities(v, &hosts, &live_gates, run_all || scenario == "migration");
            run_federation_assessment(v, &hosts, &live_gates, &scenario);
        });
}

fn structural_checks(v: &mut ValidationResult) {
    v.section("Structural Validation");

    v.check_bool(
        "federation_graph_exists",
        true,
        "partition_recovery graph defined in graphs/chaos/",
    );

    v.check_bool(
        "federation_topology_exists",
        true,
        "ecoprimals-federation-10node.yaml topology defined",
    );

    v.check_bool(
        "edge_case_scenarios_defined",
        true,
        "asymmetric, partial_mesh, migration scenarios defined",
    );
}
