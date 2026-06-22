// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Capability Propagation — validates that capabilities announced
//! on one gate become visible on other gates via Songbird mesh routing.
//!
//! The sovereign mesh uses a push/gossip model (`mesh.capabilities_announce`) with
//! query surfaces (`mesh.capabilities_query`) so remote gates can resolve providers.
//! This scenario validates:
//!
//! 1. Structural: Songbird mesh propagation methods are in the routing table
//! 2. Propagation model: `MeshTopology` register → `reachable_capabilities()`
//! 3. Convergence: capabilities from all 4 gates unify (no orphans, no duplicates)
//! 4. Live probe: when mesh is available, query peer capabilities vs expected

use crate::composition::CompositionContext;
use crate::composition::mesh::MeshTopology;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::live_mesh::LiveMeshConfig;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::collections::BTreeSet;

const MESH_ANNOUNCE: &str = "mesh.capabilities_announce";
const MESH_QUERY: &str = "mesh.capabilities_query";

/// Four-gate mesh capability model (golgi, sporeGate, eastGate, flockGate).
type GateCaps = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
);

const FOUR_GATE_MESH: &[GateCaps] = &[
    (
        "golgi",
        &[primal_names::SONGBIRD, primal_names::NESTGATE],
        &["relay", "depot", "forgejo"],
    ),
    (
        "sporeGate",
        &[primal_names::NESTGATE, primal_names::RHIZOCRYPT],
        &["build", "nest", "provenance"],
    ),
    (
        "eastGate",
        &[primal_names::SQUIRREL, primal_names::BIOMEOS],
        &["ai", "orchestration", "spring"],
    ),
    (
        "flockGate",
        &[
            primal_names::BEARDOG,
            primal_names::SONGBIRD,
            primal_names::SKUNKBAT,
        ],
        &["tower", "security", "discovery"],
    ),
];

/// Mesh capability propagation validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-capability-propagation",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-22",
        description: "Mesh capability propagation — announce/query routing, topology convergence, live peer caps",
    },
    run,
};

/// Run mesh capability propagation validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — mesh propagation routing");
    phase_structural(v);

    v.section("Phase 2: Propagation model — MeshTopology register/query");
    phase_propagation_model(v);

    v.section("Phase 3: Convergence — four-gate unified capability view");
    phase_convergence(v);

    v.section("Phase 4: Live probe — mesh peer capability list");
    phase_live_probe(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let announce = table.route(MESH_ANNOUNCE);
    v.check_bool(
        "routing:mesh_capabilities_announce",
        announce.is_some(),
        &format!(
            "{MESH_ANNOUNCE} → owner={}",
            announce.map_or("missing", |e| e.owner.as_ref())
        ),
    );

    let query = table.route(MESH_QUERY);
    v.check_bool(
        "routing:mesh_capabilities_query",
        query.is_some(),
        &format!(
            "{MESH_QUERY} → owner={}",
            query.map_or("missing", |e| e.owner.as_ref())
        ),
    );

    if let Some(entry) = announce {
        v.check_bool(
            "routing:announce_owned_by_songbird",
            entry.owner.as_ref() == primal_names::SONGBIRD,
            &format!("{MESH_ANNOUNCE} owner: {}", entry.owner),
        );
    }

    if let Some(entry) = query {
        v.check_bool(
            "routing:query_owned_by_songbird",
            entry.owner.as_ref() == primal_names::SONGBIRD,
            &format!("{MESH_QUERY} owner: {}", entry.owner),
        );
    }

    let mesh_methods = table.methods_in_domain("mesh");
    v.check_bool(
        "routing:mesh_domain_populated",
        mesh_methods.len() >= 10,
        &format!("mesh domain: {} methods", mesh_methods.len()),
    );
}

fn phase_propagation_model(v: &mut ValidationResult) {
    let mut mesh = build_four_gate_topology();
    mesh.set_local_gate("eastGate");
    mesh.mark_healthy("eastGate", true);

    let (gate_id, _, caps) = FOUR_GATE_MESH[2];
    let cap_list: Vec<&str> = caps.to_vec();
    mesh.register_gate(
        gate_id,
        None::<String>,
        std::iter::empty::<&str>(),
        cap_list.clone(),
    );
    mesh.mark_healthy(gate_id, true);

    let reachable = mesh.reachable_capabilities();
    v.check_bool(
        "propagation:registered_caps_visible",
        caps.iter().all(|c| reachable.contains(c)),
        &format!("registered on {gate_id}: {caps:?} → reachable {reachable:?}"),
    );

    v.check_minimum("propagation:reachable_count", reachable.len(), caps.len());
}

fn phase_convergence(v: &mut ValidationResult) {
    let mesh = build_four_gate_topology();

    let expected: BTreeSet<&str> = FOUR_GATE_MESH
        .iter()
        .flat_map(|(_, _, caps)| caps.iter().copied())
        .collect();
    let reachable = mesh.reachable_capabilities();

    v.check_count(
        "convergence:gate_count",
        mesh.gate_count(),
        FOUR_GATE_MESH.len(),
    );
    v.check_count(
        "convergence:healthy_gates",
        mesh.healthy_gate_count(),
        FOUR_GATE_MESH.len(),
    );
    v.check_count(
        "convergence:unified_capability_count",
        reachable.len(),
        expected.len(),
    );

    let orphans = mesh.unreachable_capabilities();
    v.check_bool(
        "convergence:no_orphans",
        orphans.is_empty(),
        &format!("orphan capabilities (unhealthy/unreachable): {orphans:?}"),
    );

    let raw_total: usize = FOUR_GATE_MESH.iter().map(|(_, _, caps)| caps.len()).sum();
    v.check_bool(
        "convergence:no_duplicates",
        raw_total == expected.len(),
        &format!(
            "raw {raw_total} vs unique {} capability entries",
            expected.len()
        ),
    );

    for cap in &expected {
        v.check_bool(
            &format!("convergence:cap:{cap}"),
            reachable.contains(cap),
            &format!("{cap} reachable across 4-gate mesh"),
        );
    }
}

fn phase_live_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mesh_cfg = LiveMeshConfig::from_env();
    if !mesh_cfg.is_connectable() {
        v.check_skip(
            "live:mesh_available",
            &format!("no remote gates configured ({})", mesh_cfg.summary()),
        );
        v.check_skip("live:capability_list_matches", "mesh not connectable");
        return;
    }

    let readiness = mesh_cfg.check_readiness();
    let any_songbird = readiness.iter().any(|g| g.songbird_responding);
    v.check_bool(
        "live:mesh_songbird_responding",
        any_songbird,
        &format!(
            "{} / {} remote gates with Songbird responding",
            readiness.iter().filter(|g| g.songbird_responding).count(),
            readiness.len()
        ),
    );

    if !any_songbird {
        v.check_skip("live:capability_list_matches", "no Songbird endpoints");
        return;
    }

    let expected: BTreeSet<&str> = FOUR_GATE_MESH
        .iter()
        .flat_map(|(_, _, caps)| caps.iter().copied())
        .collect();

    match ctx.call("discovery", "discovery.peers", serde_json::json!({})) {
        Ok(resp) => {
            let live_caps = collect_peer_capabilities(&resp);
            if live_caps.is_empty() {
                v.check_skip(
                    "live:capability_list_matches",
                    "discovery.peers returned no capability advertisements",
                );
                return;
            }

            let overlap = expected
                .iter()
                .filter(|c| live_caps.iter().any(|lc| lc == *c))
                .count();
            v.check_minimum("live:discovered_capability_count", live_caps.len(), 1);
            v.check_bool(
                "live:capability_list_matches",
                overlap >= 1,
                &format!(
                    "live mesh caps ({live_caps:?}) overlap {overlap}/{} expected model caps",
                    expected.len()
                ),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:capability_list_matches",
                &format!("discovery.peers not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:capability_list_matches",
                false,
                &format!("discovery.peers error: {e}"),
            );
        }
    }
}

/// Build a fully healthy four-gate topology from the canonical mesh model.
#[must_use]
pub fn build_four_gate_topology() -> MeshTopology {
    let mut mesh = MeshTopology::new();
    for (gate_id, primals, caps) in FOUR_GATE_MESH {
        mesh.register_gate(
            *gate_id,
            None::<String>,
            primals.iter().copied().map(str::to_owned),
            caps.iter().copied().map(str::to_owned),
        );
        mesh.mark_healthy(gate_id, true);
    }
    mesh
}

fn collect_peer_capabilities(resp: &serde_json::Value) -> BTreeSet<String> {
    let mut caps = BTreeSet::new();
    let Some(peers) = resp.get("peers").and_then(serde_json::Value::as_array) else {
        return caps;
    };
    for peer in peers {
        if let Some(arr) = peer
            .get("capabilities")
            .and_then(serde_json::Value::as_array)
        {
            for cap in arr {
                if let Some(s) = cap.as_str() {
                    caps.insert(s.to_owned());
                }
            }
        }
    }
    caps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_capability_propagation_structural() {
        let table = canonical_routing_table();
        assert!(
            table.route(MESH_ANNOUNCE).is_some(),
            "{MESH_ANNOUNCE} must be in canonical routing table"
        );
        assert!(
            table.route(MESH_QUERY).is_some(),
            "{MESH_QUERY} must be in canonical routing table"
        );
        let announce = table.route(MESH_ANNOUNCE).unwrap();
        assert_eq!(
            announce.owner.as_ref(),
            primal_names::SONGBIRD,
            "{MESH_ANNOUNCE} must route to songbird"
        );
    }

    #[test]
    fn mesh_capability_propagation_model() {
        let mut mesh = MeshTopology::new();
        mesh.set_local_gate("eastGate");
        mesh.register_gate(
            "eastGate",
            None::<String>,
            [primal_names::SQUIRREL],
            ["ai", "orchestration"],
        );
        mesh.mark_healthy("eastGate", true);

        let reachable = mesh.reachable_capabilities();
        assert!(reachable.contains("ai"));
        assert!(reachable.contains("orchestration"));
    }

    #[test]
    fn mesh_capability_propagation_convergence() {
        let mesh = build_four_gate_topology();
        let expected: BTreeSet<&str> = FOUR_GATE_MESH
            .iter()
            .flat_map(|(_, _, caps)| caps.iter().copied())
            .collect();

        assert_eq!(mesh.gate_count(), 4);
        assert_eq!(mesh.healthy_gate_count(), 4);
        assert!(mesh.unreachable_capabilities().is_empty());
        assert_eq!(mesh.reachable_capabilities().len(), expected.len());
    }

    #[test]
    fn mesh_capability_cross_gate_resolution() {
        let mut mesh = build_four_gate_topology();
        mesh.set_local_gate("eastGate");

        let security = mesh.resolve_capability("security").unwrap();
        assert_eq!(security.gate_id, "flockGate");

        let build = mesh.resolve_capability("build").unwrap();
        assert_eq!(build.gate_id, "sporeGate");

        let local = mesh.resolve_capability("orchestration").unwrap();
        assert_eq!(local.gate_id, "eastGate");
    }

    #[test]
    fn mesh_capability_propagation_run_no_panic() {
        let mut v = ValidationResult::new("mesh-capability-propagation");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() >= 4,
            "mesh-capability-propagation should evaluate structural checks"
        );
    }
}
