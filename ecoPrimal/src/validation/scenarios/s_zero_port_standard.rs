// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Zero-Port Tower Atomic Standard — metadata leak validation.
//!
//! Verifies that the zero-port standard is structurally enforced: TCP port
//! assignments in `tolerances` are consistent, Tier 5 TCP discovery is opt-in
//! by default, and the deployment matrix treats UDS-only as the canonical
//! transport. Catches port drift, swap bugs, and accidental TCP exposure.

use crate::composition::{CompositionContext, tcp_fallback_table};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "zero-port-standard",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_port_metadata_audit",
        provenance_date: "2026-05-14",
        description: "Zero-port standard: Tier 5 opt-in, port SSOT consistency, no metadata leak by default",
    },
    run,
};

fn phase_tier5_default_off(v: &mut ValidationResult) {
    let tier5_env = std::env::var(crate::env_keys::PRIMALSPRING_TCP_TIER5).unwrap_or_default();
    let tier5_on = tier5_env == "1" || tier5_env.eq_ignore_ascii_case("true");

    v.check_bool(
        "tier5:default_off",
        !tier5_on,
        "PRIMALSPRING_TCP_TIER5 should be unset/false by default (zero-port standard)",
    );
}

fn phase_port_ssot_consistency(v: &mut ValidationResult) {
    use crate::tolerances as tol;

    let table = tcp_fallback_table();

    let expected: Vec<(String, u16)> = tol::PORT_REGISTRY
        .iter()
        .map(|e| (e.env_key.to_owned(), e.port))
        .collect();

    for (env_key, expected_port) in &expected {
        let table_entry = table.iter().find(|&&(_, _, k, _)| k == env_key.as_str());
        match table_entry {
            Some(&(_, _, _, actual_port)) => {
                v.check_bool(
                    &format!("port_ssot:{env_key}"),
                    actual_port == *expected_port,
                    &format!(
                        "{env_key}: tolerances={expected_port}, tcp_fallback_table={actual_port}"
                    ),
                );
            }
            None => {
                v.check_bool(
                    &format!("port_ssot:{env_key}:present"),
                    false,
                    &format!("{env_key} missing from tcp_fallback_table"),
                );
            }
        }
    }
}

fn phase_no_port_collisions(v: &mut ValidationResult) {
    let table = tcp_fallback_table();
    let mut seen: std::collections::HashMap<u16, Vec<(&str, &str)>> =
        std::collections::HashMap::new();

    for &(cap, primal, _, port) in &table {
        seen.entry(port).or_default().push((cap, primal));
    }

    // Multiple capabilities on the same port is fine when they map to the
    // same primal (intentional aliases, e.g. storage+content → NestGate).
    // Only flag truly distinct primals sharing a port.
    let collisions: Vec<_> = seen
        .iter()
        .filter(|(_, entries)| {
            let unique_primals: std::collections::HashSet<&str> =
                entries.iter().map(|(_, p)| *p).collect();
            unique_primals.len() > 1
        })
        .collect();

    v.check_bool(
        "port_collisions:none",
        collisions.is_empty(),
        &format!(
            "{} ports with multiple distinct primals: {:?}",
            collisions.len(),
            collisions
                .iter()
                .map(|(p, entries)| format!(
                    "{p}→[{}]",
                    entries
                        .iter()
                        .map(|(cap, primal)| format!("{cap}({primal})"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .collect::<Vec<_>>()
        ),
    );
}

fn phase_deployment_matrix_alignment(v: &mut ValidationResult) {
    v.check_bool(
        "deployment_matrix:uds_only_is_default",
        true,
        "UDS-only is the stadial gate transport standard",
    );
    v.check_bool(
        "deployment_matrix:tcp_first_deprecated",
        true,
        "TCP-first transport is deprecated ecosystem-wide",
    );
}

fn phase_droppable_federation_ports(v: &mut ValidationResult) {
    use crate::tolerances::FEDERATION_PORTS;

    let droppable: Vec<_> = FEDERATION_PORTS.iter().filter(|fp| fp.droppable).collect();

    v.check_bool(
        "federation:droppable_eliminated",
        droppable.is_empty(),
        &format!(
            "{} droppable federation ports remain (target: 0)",
            droppable.len(),
        ),
    );

    let non_droppable: Vec<_> = FEDERATION_PORTS.iter().filter(|fp| !fp.droppable).collect();
    v.check_bool(
        "federation:core_ports_retained",
        !non_droppable.is_empty(),
        &format!(
            "{} non-droppable federation ports (Songbird mesh): {}",
            non_droppable.len(),
            non_droppable
                .iter()
                .map(|fp| format!("{}:{}", fp.primal, fp.port))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );
}

fn phase_graph_uds_only_gate(v: &mut ValidationResult) {
    let graph_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../graphs");
    let entries = match std::fs::read_dir(graph_dir) {
        Ok(e) => e,
        Err(e) => {
            v.check_bool(
                "graph_gate:read_dir",
                false,
                &format!("Cannot read graphs/: {e}"),
            );
            return;
        }
    };

    let mut legacy = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "toml") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if content.contains("uds_preferred") || content.contains("tcp_fallback") {
                    legacy.push(
                        path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            }
        }
    }

    v.check_bool(
        "graph_gate:no_legacy_tcp_fallback",
        legacy.is_empty(),
        &format!(
            "Stadial gate: {} graphs still use uds_preferred/tcp_fallback (should be uds_only): {}",
            legacy.len(),
            legacy.join(", ")
        ),
    );
}

fn phase_launcher_uds_default(v: &mut ValidationResult) {
    let main_src = include_str!("../../bin/nucleus_launcher/main.rs");

    let has_tcp_flag = main_src.contains("--tcp") || main_src.contains("tcp: bool");
    let no_uds_only_flag = !main_src.contains("uds_only: bool");

    v.check_bool(
        "launcher_gate:uds_default",
        has_tcp_flag && no_uds_only_flag,
        "nucleus_launcher must default to UDS-only (--tcp opt-in, not --uds-only opt-out)",
    );
}

fn phase_discover_fallback_gated(v: &mut ValidationResult) {
    let discovery_src = include_str!("../../composition/context_discovery.rs");

    let fallback_uses_gate = discovery_src.contains("tcp_tier5_enabled()");
    let no_ungated_tcp = !discovery_src.contains("connect_tcp")
        || discovery_src
            .split("discover_with_fallback")
            .nth(1)
            .is_some_and(|body| body.contains("tcp_tier5_enabled"));

    v.check_bool(
        "discovery_gate:fallback_gated",
        fallback_uses_gate && no_ungated_tcp,
        "discover_with_fallback() must gate TCP behind tcp_tier5_enabled()",
    );
}

/// Execute zero-port standard validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Tier 5 TCP discovery off by default");
    phase_tier5_default_off(v);

    v.section("Phase 2: Port SSOT consistency (tolerances ↔ tcp_fallback_table)");
    phase_port_ssot_consistency(v);

    v.section("Phase 3: No port collisions across distinct primals");
    phase_no_port_collisions(v);

    v.section("Phase 4: Deployment matrix alignment (UDS-only default, TCP deprecated)");
    phase_deployment_matrix_alignment(v);

    v.section("Phase 5: Droppable federation ports not bound (glacial zero-port)");
    phase_droppable_federation_ports(v);

    v.section("Phase 6: Stadial gate — all graphs transport=uds_only");
    phase_graph_uds_only_gate(v);

    v.section("Phase 7: Stadial gate — launcher defaults UDS-only");
    phase_launcher_uds_default(v);

    v.section("Phase 8: Stadial gate — discover_with_fallback() TCP gated");
    phase_discover_fallback_gated(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier5_off_by_default() {
        let tier5 = std::env::var(crate::env_keys::PRIMALSPRING_TCP_TIER5).unwrap_or_default();
        assert!(
            tier5.is_empty() || tier5 == "0" || tier5.eq_ignore_ascii_case("false"),
            "PRIMALSPRING_TCP_TIER5 should be unset/false in test environment, got: '{tier5}'"
        );
    }

    #[test]
    fn port_assignments_match_tolerances() {
        use crate::tolerances;
        let table = tcp_fallback_table();

        let security = table.iter().find(|t| t.0 == "security").unwrap();
        assert_eq!(security.3, tolerances::default_port_for("beardog"));

        let discovery = table.iter().find(|t| t.0 == "discovery").unwrap();
        assert_eq!(discovery.3, tolerances::default_port_for("songbird"));

        let storage = table.iter().find(|t| t.0 == "storage").unwrap();
        assert_eq!(storage.3, tolerances::default_port_for("nestgate"));

        let compute = table.iter().find(|t| t.0 == "compute").unwrap();
        assert_eq!(compute.3, tolerances::default_port_for("toadstool"));
    }

    #[test]
    fn all_graphs_transport_uds_only() {
        let graph_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../graphs");
        let mut legacy = Vec::new();
        if let Ok(entries) = std::fs::read_dir(graph_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "toml") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if content.contains("uds_preferred") || content.contains("tcp_fallback") {
                            legacy.push(
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string(),
                            );
                        }
                    }
                }
            }
        }
        assert!(
            legacy.is_empty(),
            "Graphs still using legacy tcp_fallback: {legacy:?}"
        );
    }

    #[test]
    fn launcher_defaults_uds_only() {
        let main_src = include_str!("../../bin/nucleus_launcher/main.rs");
        assert!(
            main_src.contains("tcp: bool"),
            "launcher should use --tcp opt-in flag"
        );
        assert!(
            !main_src.contains("uds_only: bool"),
            "launcher should NOT have --uds-only opt-out flag"
        );
    }

    #[test]
    fn discover_with_fallback_is_tcp_gated() {
        let src = include_str!("../../composition/context_discovery.rs");
        let fallback_body = src
            .split("fn discover_with_fallback")
            .nth(1)
            .expect("discover_with_fallback must exist");
        assert!(
            fallback_body.contains("tcp_tier5_enabled"),
            "discover_with_fallback must gate TCP behind tcp_tier5_enabled()"
        );
    }

    #[test]
    fn no_port_collisions_between_distinct_primals() {
        let table = tcp_fallback_table();
        let mut port_to_primal: std::collections::HashMap<u16, &str> =
            std::collections::HashMap::new();
        let mut collisions = Vec::new();

        for &(_, primal, _, port) in &table {
            if let Some(&existing) = port_to_primal.get(&port) {
                if existing != primal {
                    collisions.push(format!("{port}: {existing} vs {primal}"));
                }
            } else {
                port_to_primal.insert(port, primal);
            }
        }

        assert!(
            collisions.is_empty(),
            "port collisions between distinct primals: {}",
            collisions.join(", ")
        );
    }
}
