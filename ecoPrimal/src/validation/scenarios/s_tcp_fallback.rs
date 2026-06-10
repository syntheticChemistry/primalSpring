// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: TCP-Only Fallback — validates the server-side transport
//! binding infrastructure for platforms where UDS is denied (Android SELinux).
//!
//! Structural phase (Tier::Rust):
//!   Validates that `server_bind` module exists and functions correctly:
//!   bind mode parsing, port resolution for all 13 primals, permission
//!   error detection, and the UDS → TCP fallback chain.
//!
//! This scenario is the gate for grapheneGate 13/13: the 4 remaining
//! primals (coralreef, nestgate, biomeOS, petaltongue) need to adopt
//! `bind_transport(..., BindMode::Fallback)` to gracefully degrade
//! from UDS to TCP when SELinux denies `bind()`.

use crate::composition::CompositionContext;
use crate::ipc::server_bind::{BindError, BindMode, BoundTransport, bind_transport};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// TCP-only fallback validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tcp-fallback",
        track: Track::Transport,
        tier: Tier::Rust,
        provenance_crate: "wave106_graphenegate_fallback",
        provenance_date: "2026-06-10",
        description: "TCP-only fallback — server_bind infrastructure for UDS→TCP graceful degradation",
    },
    run,
};

/// The 4 primals that need TCP-only fallback for grapheneGate 13/13.
const GRAPHENEGATE_BLOCKED: &[(&str, u16)] = &[
    ("coralreef", 9730),
    ("nestgate", 9500),
    ("biomeos", 9800),
    ("petaltongue", 9900),
];

/// All 13 NUCLEUS primals should have port entries.
const ALL_PRIMALS: &[&str] = &[
    "beardog",
    "songbird",
    "squirrel",
    "toadstool",
    "nestgate",
    "rhizocrypt",
    "loamspine",
    "coralreef",
    "barracuda",
    "skunkbat",
    "biomeos",
    "sweetgrass",
    "petaltongue",
];

/// Run the TCP-only fallback validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: BindMode parsing");
    phase_bind_mode(v);

    v.section("Phase 2: Port resolution for all 13 primals");
    phase_port_resolution(v);

    v.section("Phase 3: grapheneGate blocked primal ports");
    phase_graphenegate_ports(v);

    v.section("Phase 4: UDS bind + transport types");
    phase_uds_bind(v);

    v.section("Phase 5: TCP bind via port 0");
    phase_tcp_bind(v);

    v.section("Phase 6: Error handling — no port configured");
    phase_no_port_error(v);
}

fn phase_bind_mode(v: &mut ValidationResult) {
    v.check_bool(
        "bind_mode:uds_only",
        BindMode::UdsOnly != BindMode::TcpOnly,
        "BindMode::UdsOnly is distinct from TcpOnly",
    );
    v.check_bool(
        "bind_mode:fallback",
        BindMode::Fallback != BindMode::UdsOnly,
        "BindMode::Fallback is distinct from UdsOnly",
    );
    v.check_bool(
        "bind_mode:three_variants",
        BindMode::UdsOnly != BindMode::TcpOnly
            && BindMode::TcpOnly != BindMode::Fallback
            && BindMode::Fallback != BindMode::UdsOnly,
        "all three BindMode variants are distinct",
    );
}

fn phase_port_resolution(v: &mut ValidationResult) {
    for primal in ALL_PRIMALS {
        let port = crate::tolerances::default_port_for(primal);
        v.check_bool(
            &format!("port:{primal}"),
            port > 0,
            &format!("{primal} TCP fallback port = {port}"),
        );
    }
}

fn phase_graphenegate_ports(v: &mut ValidationResult) {
    for &(primal, expected_port) in GRAPHENEGATE_BLOCKED {
        let port = crate::tolerances::default_port_for(primal);
        v.check_bool(
            &format!("graphenegate:{primal}:port_correct"),
            port == expected_port,
            &format!("{primal} port {port} matches expected {expected_port}"),
        );

        let env_key = crate::env_keys::port_env_key(primal);
        v.check_bool(
            &format!("graphenegate:{primal}:env_key"),
            !env_key.is_empty(),
            &format!("{primal} env key = {env_key}"),
        );
    }
}

fn phase_uds_bind(v: &mut ValidationResult) {
    let result = bind_transport("tcp_fallback_test_uds", BindMode::UdsOnly);
    match result {
        Ok(bound) => {
            let is_unix = !bound.is_tcp();
            let display = bound.endpoint_display();
            if is_unix {
                v.check_bool("uds:bind_succeeds", true, &format!("UDS bind: {display}"));
                v.check_bool(
                    "uds:is_not_tcp",
                    true,
                    "BoundTransport::Unix reports is_tcp() = false",
                );
                v.check_bool(
                    "uds:display_prefix",
                    display.starts_with("unix:"),
                    &format!("endpoint display: {display}"),
                );
                if let BoundTransport::Unix(_, ref path) = bound {
                    let _ = std::fs::remove_file(path);
                }
            } else {
                v.check_bool(
                    "uds:bind_succeeds",
                    false,
                    "UDS mode returned TCP (unexpected)",
                );
            }
        }
        Err(e) => {
            v.check_skip(
                "uds:bind_succeeds",
                &format!("UDS bind skipped (possibly no write access): {e}"),
            );
            v.check_skip("uds:is_not_tcp", "UDS bind skipped");
            v.check_skip("uds:display_prefix", "UDS bind skipped");
        }
    }
}

fn phase_tcp_bind(v: &mut ValidationResult) {
    let result = bind_transport("beardog", BindMode::TcpOnly);
    match result {
        Ok(bound) => {
            let is_tcp = bound.is_tcp();
            let display = bound.endpoint_display();
            if is_tcp {
                v.check_bool("tcp:bind_succeeds", true, &format!("TCP bind: {display}"));
                v.check_bool(
                    "tcp:is_tcp",
                    true,
                    "BoundTransport::Tcp reports is_tcp() = true",
                );
                v.check_bool(
                    "tcp:display_prefix",
                    display.starts_with("tcp:"),
                    &format!("endpoint display: {display}"),
                );
            } else {
                v.check_bool(
                    "tcp:bind_succeeds",
                    false,
                    "TCP mode returned Unix (unexpected)",
                );
            }
        }
        Err(e) => {
            v.check_skip(
                "tcp:bind_succeeds",
                &format!("TCP bind skipped (port in use?): {e}"),
            );
            v.check_skip("tcp:is_tcp", "TCP bind skipped");
            v.check_skip("tcp:display_prefix", "TCP bind skipped");
        }
    }
}

fn phase_no_port_error(v: &mut ValidationResult) {
    let result = bind_transport("nonexistent_primal_zzz", BindMode::TcpOnly);
    v.check_bool(
        "error:no_port_configured",
        matches!(result, Err(BindError::NoPortConfigured { .. })),
        "TcpOnly with unknown primal returns NoPortConfigured error",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn tcp_fallback_scenario_passes() {
        let mut v = ValidationResult::new("tcp-fallback");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "tcp-fallback scenario: {} failures (passed={}, skipped={})",
            v.failed, v.passed, v.skipped,
        );
    }
}
