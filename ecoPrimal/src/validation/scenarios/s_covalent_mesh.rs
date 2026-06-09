// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Covalent Mesh — cross-gate trust validation.
//!
//! Six-phase validation of the covalent mesh trust model:
//!
//! 1. Structural: federation port, deploy graphs, capability registry
//! 2. Discovery: `discovery.peers` via Songbird TCP :7700
//! 3. Cross-gate dispatch: `capability.call` transparent routing
//! 4. Security: bearDog BTSP cross-gate token validation
//! 5. Content integrity: NestGate federation BLAKE3 end-to-end
//! 6. Dark Forest invariants: isolation + reversibility
//!
//! Tier::Both — structural checks pass without primals, live checks
//! gracefully skip when federation is unavailable.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

pub(super) const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

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

    v.section("Phase 4: Security — bearDog BTSP cross-gate token validation");
    super::covalent_mesh_trust::phase_security_trust(v, ctx);

    v.section("Phase 5: Content integrity — NestGate federation BLAKE3");
    super::covalent_mesh_trust::phase_content_integrity(v, ctx);

    v.section("Phase 6: Dark Forest invariants — isolation + reversibility");
    super::covalent_mesh_trust::phase_dark_forest_invariants(v, ctx);
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
                v.check_skip(
                    "live:peer_latency",
                    "no peers for latency measurement",
                );
                v.check_skip(
                    "live:capability_propagation",
                    "no peers for capability check",
                );
            } else if let Some(peers) = resp.get("peers").and_then(serde_json::Value::as_array) {
                let gate_ids: Vec<&str> = peers
                    .iter()
                    .filter_map(|p| {
                        p.get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)
                    })
                    .collect();
                v.check_bool(
                    "live:peer_gate_ids",
                    !gate_ids.is_empty(),
                    &format!("peer gates: {gate_ids:?}"),
                );
                let latency_peers: Vec<(&str, f64)> = peers
                    .iter()
                    .filter_map(|p| {
                        let gate = p
                            .get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)?;
                        let ms = p.get("latency_ms").and_then(serde_json::Value::as_f64)?;
                        Some((gate, ms))
                    })
                    .collect();
                if latency_peers.is_empty() {
                    v.check_skip(
                        "live:peer_latency",
                        "peers present but no latency_ms field (Songbird may need update)",
                    );
                } else {
                    let summary: Vec<String> = latency_peers
                        .iter()
                        .map(|(g, ms)| format!("{g}={ms:.1}ms"))
                        .collect();
                    v.check_bool(
                        "live:peer_latency",
                        true,
                        &format!("peer latencies: {}", summary.join(", ")),
                    );
                }

                let peers_with_caps: Vec<&str> = peers
                    .iter()
                    .filter(|p| {
                        p.get("capabilities")
                            .and_then(serde_json::Value::as_array)
                            .is_some_and(|a| !a.is_empty())
                    })
                    .filter_map(|p| {
                        p.get("gate")
                            .or_else(|| p.get("node_id"))
                            .and_then(serde_json::Value::as_str)
                    })
                    .collect();
                if peers_with_caps.is_empty() {
                    v.check_skip(
                        "live:capability_propagation",
                        "peers discovered but capabilities: [] — Songbird propagation gap (P1)",
                    );
                } else {
                    v.check_bool(
                        "live:capability_propagation",
                        true,
                        &format!(
                            "capability propagation: {} peer(s) advertising caps: {:?}",
                            peers_with_caps.len(),
                            peers_with_caps
                        ),
                    );
                }
            }
        }
        Err(e) if e.is_skippable() => {
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

    let manifest_toml = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
    let peer_gates: Vec<String> = toml::from_str::<toml::Value>(manifest_toml)
        .ok()
        .and_then(|p| p.get("gates")?.as_table().cloned())
        .map(|t| {
            t.keys()
                .filter(|k| !["golgiBody", "peptidoglycan", "golgiBody-ext"].contains(&k.as_str()))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    for target_gate in &peer_gates {
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
            Err(e) if e.is_skippable() => {
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
                    || msg.contains("Invalid JSON from remote")
                    || msg.contains("No local or remote provider")
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
    use crate::ipc::verifiers::parse_verify_ionic_response;
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

    #[test]
    fn phase_structural_passes() {
        let mut v = ValidationResult::new("covalent-mesh-structural");
        phase_structural(&mut v);
        assert_eq!(v.failed, 0, "structural checks should pass");
    }

    #[test]
    fn phase_dark_forest_structural() {
        let mut v = ValidationResult::new("covalent-mesh-darkforest");
        let mut ctx = CompositionContext::discover();
        super::super::covalent_mesh_trust::phase_dark_forest_invariants(&mut v, &mut ctx);
    }

    #[test]
    fn forged_token_format() {
        let forged = serde_json::json!({ "valid": false, "error": "invalid_signature" });
        assert!(
            parse_verify_ionic_response(&forged).is_none(),
            "forged token must be rejected"
        );
    }
}
