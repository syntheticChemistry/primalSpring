// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Covalent Mesh — Wave 49 LAN mesh validation.
//!
//! Validates cross-gate connectivity via Songbird TCP federation:
//!
//! 1. Structural: federation port, deploy graphs for multi-gate
//! 2. `discovery.peers` via Songbird TCP :7700 — sees remote gates
//! 3. Cross-gate `capability.call` — transparent routing through mesh
//!
//! Tier::Both — structural checks pass without primals, live checks
//! gracefully skip when federation is unavailable.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "covalent-mesh",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave49_covalent_mesh",
        provenance_date: "2026-05-25",
        description:
            "Covalent mesh — discovery.peers, cross-gate capability.call via Songbird TCP federation",
    },
    run,
};

/// Run all covalent mesh validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — federation prerequisites");
    phase_structural(v);

    v.section("Phase 2: Discovery — Songbird TCP federation peers");
    phase_discovery_peers(v, ctx);

    v.section("Phase 3: Cross-gate — capability.call via mesh dispatch");
    phase_cross_gate_dispatch(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "registry:discovery_peers",
        REGISTRY_TOML.contains("discovery.peers"),
        "discovery.peers registered in capability_registry.toml",
    );
    v.check_bool(
        "registry:capability_call",
        REGISTRY_TOML.contains("capability.call"),
        "capability.call registered (mesh dispatch entrypoint)",
    );
    v.check_bool(
        "registry:route_register",
        REGISTRY_TOML.contains("route.register"),
        "route.register registered (federation route sharing)",
    );

    let covalent_graph = include_str!("../../../../graphs/multi_node/basement_hpc_covalent.toml");
    let parsed: Result<toml::Value, _> = toml::from_str(covalent_graph);
    v.check_bool(
        "graph:covalent_parses",
        parsed.is_ok(),
        "basement_hpc_covalent.toml valid TOML",
    );
}

fn phase_discovery_peers(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("discovery") {
        v.check_skip(
            "live:discovery_peers",
            "discovery capability not in composition context",
        );
        v.check_skip(
            "live:peer_count",
            "discovery capability not in composition context",
        );
        return;
    }

    match ctx.call(
        "discovery",
        "discovery.peers",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:discovery_peers",
                true,
                &format!("discovery.peers responded: {resp}"),
            );
            let peer_count = resp
                .get("peers")
                .and_then(serde_json::Value::as_array)
                .map_or(0, Vec::len);
            v.check_bool(
                "live:peer_count",
                peer_count > 0,
                &format!("discovery.peers found {peer_count} remote peer(s)"),
            );
            if peer_count == 0 {
                v.check_skip(
                    "live:peer_gate_ids",
                    "no peers — federation port may not be enabled",
                );
            } else if let Some(peers) = resp.get("peers").and_then(serde_json::Value::as_array) {
                let gate_ids: Vec<&str> = peers
                    .iter()
                    .filter_map(|p| p.get("gate").and_then(serde_json::Value::as_str))
                    .collect();
                v.check_bool(
                    "live:peer_gate_ids",
                    !gate_ids.is_empty(),
                    &format!("peer gates: {gate_ids:?}"),
                );
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "live:discovery_peers",
                &format!("Songbird not reachable: {e}"),
            );
            v.check_skip("live:peer_count", "Songbird not reachable");
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("not found") || msg.contains("-32601") {
                v.check_skip(
                    "live:discovery_peers",
                    &format!("discovery.peers not implemented: {e}"),
                );
                v.check_skip("live:peer_count", "discovery.peers not available");
            } else {
                v.check_bool(
                    "live:discovery_peers",
                    false,
                    &format!("discovery.peers error: {e}"),
                );
                v.check_skip("live:peer_count", &format!("error: {e}"));
            }
        }
    }
}

fn phase_cross_gate_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:mesh_capability_call",
            "orchestration capability not in context",
        );
        return;
    }

    for target_gate in &["ironGate", "southGate", "biomeGate"] {
        let check_id = format!("live:mesh_call_{target_gate}");
        match ctx.call(
            "orchestration",
            "capability.call",
            serde_json::json!({
                "capability": "security",
                "operation": "health.liveness",
                "args": {},
                "gate": target_gate,
            }),
        ) {
            Ok(resp) => {
                v.check_bool(
                    &check_id,
                    true,
                    &format!("capability.call to {target_gate} succeeded: {resp}"),
                );
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(
                    &check_id,
                    &format!("{target_gate} not reachable (mesh not active): {e}"),
                );
            }
            Err(e) => {
                let msg = format!("{e}");
                let expected_skip = msg.contains("not found")
                    || msg.contains("unknown gate")
                    || msg.contains("no route")
                    || msg.contains("not available")
                    || msg.contains("-32601");
                if expected_skip {
                    v.check_skip(
                        &check_id,
                        &format!("{target_gate} mesh not available: {e}"),
                    );
                } else {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!("{target_gate} unexpected error: {e}"),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn covalent_mesh_no_panic() {
        let mut v = ValidationResult::new("covalent-mesh");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn registry_has_discovery_peers() {
        assert!(
            REGISTRY_TOML.contains("discovery.peers"),
            "discovery.peers must be in capability_registry.toml"
        );
    }
}
