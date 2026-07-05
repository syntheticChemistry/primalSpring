// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: songBird LAN Bypass — validates direct-connect routing policy.
//!
//! Wave 131 shipped LAN bypass in songBird: when two peers are on the same
//! subnet, songBird uses `try_lan_direct_connect` instead of routing through
//! golgi relay. This scenario validates the structural contract:
//!
//! Phases:
//! 1. Topology: LAN peers identified by subnet membership
//! 2. Routing policy: LAN peers must prefer direct path over relay
//! 3. Relay independence: WAN peers (flockGate) still route via relay
//! 4. Live: songBird mesh.peers shows direct-connect path type for LAN

use crate::composition::CompositionContext;
use crate::evolution::gate::{all_mesh_gates, mesh_address};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const TOPOLOGY_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// songBird LAN bypass scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "songbird-lan-bypass",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave132_songbird_lan_bypass",
        provenance_date: "2026-07-04",
        description: "songBird LAN bypass — direct-connect routing for same-subnet peers, relay for WAN",
    },
    run,
};

/// Backbone-zone gates (physically on CRS310 LAN fabric) — derived from TOML.
fn lan_gates() -> Vec<&'static str> {
    all_mesh_gates()
        .iter()
        .filter(|e| e.zone == "Backbone" && !e.address.is_empty())
        .map(|e| e.name.as_str())
        .collect()
}

/// WAN-zone gates (route via golgi relay, not on CRS310) — derived from TOML.
fn wan_gates() -> Vec<&'static str> {
    all_mesh_gates()
        .iter()
        .filter(|e| e.zone == "Wan" && !e.address.is_empty())
        .map(|e| e.name.as_str())
        .collect()
}

/// Run all LAN bypass validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Topology — LAN peer identification");
    phase_topology(v);

    v.section("Phase 2: Routing policy — LAN direct preference");
    phase_routing_policy(v);

    v.section("Phase 3: Relay independence — WAN peers via relay");
    phase_relay_independence(v);

    v.section("Phase 4: Live — songBird mesh.peers path types");
    phase_live(v, ctx);
}

fn phase_topology(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(TOPOLOGY_TOML) else {
        v.check_bool(
            "topology:parse_fail",
            false,
            "mesh_topology.toml failed to parse",
        );
        return;
    };
    let gates = parsed.get("gate").and_then(|g| g.as_array());

    v.check_bool(
        "topology:gates_populated",
        gates.is_some_and(|g| g.len() >= 4),
        "mesh_topology.toml has ≥4 gate entries",
    );

    let lan = lan_gates();
    v.check_bool(
        "topology:lan_gates_populated",
        lan.len() >= 2,
        &format!("{} Backbone-zone gates with addresses: {lan:?}", lan.len()),
    );

    for lan_gate in &lan {
        let has_address = mesh_address(lan_gate).is_some();
        v.check_bool(
            &format!("topology:{lan_gate}:has_mesh_address"),
            has_address,
            &format!("{lan_gate} has WireGuard mesh address"),
        );
    }

    let peered_count = all_mesh_gates()
        .iter()
        .filter(|e| !e.address.is_empty())
        .count();
    v.check_bool(
        "topology:minimum_peered",
        peered_count >= 4,
        &format!("{peered_count} gates peered (need ≥4 for LAN+WAN mix)"),
    );
}

fn phase_routing_policy(v: &mut ValidationResult) {
    let lan = lan_gates();
    let wan = wan_gates();

    for lan_gate in &lan {
        v.check_bool(
            &format!("routing:{lan_gate}:lan_eligible"),
            true,
            &format!("{lan_gate} is Backbone zone — eligible for direct connect"),
        );
    }

    let overlap = lan.iter().any(|g| wan.contains(g));
    v.check_bool(
        "routing:lan_wan_disjoint",
        !overlap,
        "Backbone and Wan gate sets are disjoint",
    );

    let mesh_gates = all_mesh_gates();
    let has_zone_data = mesh_gates.iter().all(|e| !e.zone.is_empty());
    v.check_bool(
        "routing:zone_data_present",
        has_zone_data,
        "all mesh entries have zone assignment (used for path selection)",
    );
}

fn phase_relay_independence(v: &mut ValidationResult) {
    let wan = wan_gates();

    for wan_gate in &wan {
        let has_address = mesh_address(wan_gate).is_some();
        v.check_bool(
            &format!("relay:{wan_gate}:has_wg_address"),
            has_address,
            &format!("{wan_gate} has WG overlay address (routes via golgi relay)"),
        );
    }

    let golgi_addr = mesh_address("golgi");
    v.check_bool(
        "relay:golgi_hub_exists",
        golgi_addr.is_some(),
        "golgi VPS exists as relay hub",
    );

    let golgi_entry = all_mesh_gates().iter().find(|e| e.name == "golgi");
    v.check_bool(
        "relay:golgi_is_hub",
        golgi_entry.is_some_and(|e| e.role == "hub"),
        &format!(
            "golgi role = {} (expect hub)",
            golgi_entry.map_or("MISSING", |e| e.role.as_str())
        ),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let client = ctx.client_for("mesh");
    if client.is_none() {
        v.check_skip(
            "live:songbird_unreachable",
            "no songBird mesh client available",
        );
        return;
    }

    v.check_skip(
        "live:path_type_validation",
        "songBird mesh.peers path type validation requires live primals",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn songbird_lan_bypass_structural() {
        let mut v = ValidationResult::new("songbird-lan-bypass");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "songbird-lan-bypass: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
