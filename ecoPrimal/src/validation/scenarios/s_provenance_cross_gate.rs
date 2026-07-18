// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Provenance Cross-Gate Verification — validates the Nest
//! provenance pipeline over the WAN mesh.
//!
//! `RhizoCrypt` DAG → `LoamSpine` ledger → `SweetGrass` attribution witness must
//! be verifiable from a remote gate. Phase 1 (Structural): provenance trio
//! model. Phase 2 (Structural): cross-gate model. Phase 3 (Live): remote probes.

use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::coordination::AtomicType;
use crate::evolution::gate::{GateMatrix, GateStatus, mesh_address};
use crate::primal_names;
use crate::tolerances::{SCENARIO_TCP_PROBE_TIMEOUT_MS, ports::port_entry_for};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const NEST_REFERENCE_GATES: &[&str] = &["sporeGate", "eastGate"];

const PROVENANCE_QUARTET: &[&str] = &[
    primal_names::NESTGATE,
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
];

const PROVENANCE_PIPELINE: &[(&str, &str)] = &[
    (primal_names::RHIZOCRYPT, "DAG"),
    (primal_names::LOAMSPINE, "Ledger"),
    (primal_names::SWEETGRASS, "Witness"),
];

const PROVENANCE_TRIO: &[&str] = &[
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
];

/// Provenance cross-gate verification over the WAN mesh.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "provenance-cross-gate",
        track: Track::Sovereignty,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-20",
        description: "Nest provenance pipeline verifiable from remote gates over WAN mesh",
    },
    run,
};

/// Run provenance cross-gate validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — provenance trio model");
    phase_provenance_model(v);

    v.section("Phase 2: Structural — cross-gate provenance model");
    phase_cross_gate_model(v);

    v.section("Phase 3: Live — remote provenance service probes");
    phase_live_probes(v);
}

fn phase_provenance_model(v: &mut ValidationResult) {
    let slugs = AtomicType::Nest.required_primal_slugs();

    for slug in PROVENANCE_QUARTET {
        v.check_bool(
            &format!("struct:nest:requires:{slug}"),
            slugs.contains(slug),
            &format!("Nest atomic requires {slug}"),
        );
    }

    v.check_bool(
        "struct:nest:provenance_quartet",
        PROVENANCE_QUARTET
            .iter()
            .filter(|slug| slugs.contains(slug))
            .count()
            == PROVENANCE_QUARTET.len(),
        &format!(
            "Nest provenance quartet present in required_primal_slugs ({})",
            slugs.join(", ")
        ),
    );

    v.check_bool(
        "struct:pipeline:three_stages",
        PROVENANCE_PIPELINE.len() == 3,
        "provenance pipeline: DAG → Ledger → Witness (3 stages)",
    );

    for (slug, stage) in PROVENANCE_PIPELINE {
        v.check_bool(
            &format!("struct:pipeline:{slug}"),
            slugs.contains(slug),
            &format!("{stage} stage ({slug}) in Nest composition"),
        );
    }

    let table = canonical_routing_table();
    for slug in PROVENANCE_TRIO {
        let methods = table.methods_for_primal(slug);
        v.check_bool(
            &format!("struct:routing:{slug}"),
            !methods.is_empty(),
            &format!(
                "{slug}: {} methods in canonical routing table",
                methods.len()
            ),
        );
    }
}

fn phase_cross_gate_model(v: &mut ValidationResult) {
    let matrix = GateMatrix::enumerate();

    for gate_name in NEST_REFERENCE_GATES {
        let found = matrix.gates.iter().any(|g| g.name == *gate_name);
        v.check_bool(
            &format!("struct:gate:tracked:{gate_name}"),
            found,
            &format!("{gate_name}: tracked in gate matrix"),
        );
    }

    let nest_capable: Vec<&GateStatus> = matrix
        .gates
        .iter()
        .filter(|g| gate_has_nest_capability(g))
        .collect();

    v.check_bool(
        "struct:nest_gates:at_least_two",
        nest_capable.len() >= 2,
        &format!(
            "{} Nest-capable gates (reference: sporeGate, eastGate): {}",
            nest_capable.len(),
            nest_capable
                .iter()
                .map(|g| g.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );

    let mesh_nest_gates: Vec<&GateStatus> = nest_capable
        .iter()
        .copied()
        .filter(|g| mesh_address(&g.name).is_some())
        .collect();

    v.check_bool(
        "struct:mesh:nest_gate_addresses",
        mesh_nest_gates.len() >= 2,
        &format!(
            "{} Nest-capable gates with WireGuard mesh addresses: {}",
            mesh_nest_gates.len(),
            mesh_nest_gates
                .iter()
                .map(|g| {
                    mesh_address(&g.name)
                        .map_or_else(|| g.name.clone(), |ip| format!("{}={ip}", g.name))
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );
}

fn gate_has_nest_capability(gate: &GateStatus) -> bool {
    if gate.primals_expected >= 13 {
        return NEST_REFERENCE_GATES.contains(&gate.name.as_str())
            || gate.primals_alive >= 7
            || mesh_address(&gate.name).is_some();
    }
    gate.primals_alive >= 7
}

fn phase_live_probes(v: &mut ValidationResult) {
    if !std::path::Path::new("/sys/class/net/wg0").exists() {
        v.check_skip(
            "live:wg0",
            "wg0 not present — skipping cross-gate provenance probes",
        );
        return;
    }

    let matrix = GateMatrix::enumerate();
    let nest_gates: Vec<&GateStatus> = matrix
        .gates
        .iter()
        .filter(|g| gate_has_nest_capability(g) && mesh_address(&g.name).is_some())
        .collect();

    if nest_gates.is_empty() {
        v.check_skip(
            "live:provenance:gates_reachable",
            "no Nest-capable gates with mesh addresses configured",
        );
        return;
    }

    let timeout = Duration::from_millis(SCENARIO_TCP_PROBE_TIMEOUT_MS);
    let local_gate = detect_local_gate();
    let mut gates_with_provenance = 0u32;

    for gate in &nest_gates {
        if gate.name.eq_ignore_ascii_case(&local_gate) {
            continue;
        }

        let Some(mesh_ip) = mesh_address(&gate.name) else {
            v.check_skip(
                &format!("live:ping:{}", gate.name),
                &format!("{}: no mesh address", gate.name),
            );
            continue;
        };

        let ping_ok = icmp_reachable(mesh_ip);
        v.check_bool(
            &format!("live:ping:{}", gate.name),
            ping_ok,
            &format!(
                "ping {mesh_ip} ({}): {}",
                gate.name,
                if ping_ok { "REACHABLE" } else { "UNREACHABLE" }
            ),
        );

        if !ping_ok {
            for slug in PROVENANCE_TRIO {
                v.check_skip(
                    &format!("live:tcp:{}:{}", gate.name, slug),
                    &format!("{} unreachable — skipping {slug} TCP probe", gate.name),
                );
            }
            continue;
        }

        let mut trio_reachable = 0u32;
        for slug in PROVENANCE_TRIO {
            let port = port_entry_for(slug).map_or(0, |e| e.port);
            if port == 0 {
                v.check_skip(
                    &format!("live:tcp:{}:{}", gate.name, slug),
                    &format!("no port registered for {slug}"),
                );
                continue;
            }

            let reachable = probe_tcp(mesh_ip, port, timeout);
            if reachable {
                trio_reachable += 1;
            }
            v.check_bool(
                &format!("live:tcp:{}:{}", gate.name, slug),
                reachable,
                &format!(
                    "{slug} @ {mesh_ip}:{port}: {}",
                    if reachable {
                        "REACHABLE"
                    } else {
                        "UNREACHABLE"
                    }
                ),
            );
        }

        #[expect(
            clippy::cast_possible_truncation,
            reason = "provenance trio has 3 primals"
        )]
        let expected = PROVENANCE_TRIO.len() as u32;
        if trio_reachable == expected {
            gates_with_provenance += 1;
        }
    }

    v.check_bool(
        "live:provenance:gates_reachable",
        gates_with_provenance > 0,
        &format!(
            "{gates_with_provenance} remote gate(s) with all provenance trio TCP services reachable"
        ),
    );
}

fn detect_local_gate() -> String {
    if let Ok(name) = std::env::var("GATE_NAME") {
        return name;
    }
    std::process::Command::new("hostname")
        .output()
        .ok()
        .map_or_else(
            || "unknown".to_owned(),
            |o| String::from_utf8_lossy(&o.stdout).trim().to_owned(),
        )
}

fn icmp_reachable(ip: &str) -> bool {
    std::process::Command::new("ping")
        .args(["-c", "1", "-W", "2", ip])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

fn probe_tcp(host: &str, port: u16, timeout: Duration) -> bool {
    let Ok(ip) = host.parse::<Ipv4Addr>() else {
        return false;
    };
    let addr = SocketAddr::new(ip.into(), port);
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_cross_gate_structural() {
        let mut v = ValidationResult::new("provenance-cross-gate");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "provenance-cross-gate should evaluate at least one check"
        );
    }
}
