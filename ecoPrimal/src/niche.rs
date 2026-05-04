// SPDX-License-Identifier: AGPL-3.0-or-later

//! Niche deployment self-knowledge for primalSpring.
//!
//! A Spring is a niche validation domain. primalSpring's domain is
//! coordination itself: atomic composition, graph execution, emergent
//! systems, bonding, and cross-spring interaction patterns.
//!
//! This module holds the niche's self-knowledge:
//! - Capability table (what the niche exposes via biomeOS)
//! - Semantic mappings (coordination domain methods)
//! - Operation dependencies (parallelization hints for Pathway Learner)
//! - Cost estimates (scheduling hints for biomeOS)
//! - Registration logic (how the niche advertises itself)
//!
//! # Absorption Pattern
//!
//! Other springs can absorb primalSpring's coordination validation by
//! composing against this capability table. biomeOS uses the semantic
//! mappings and cost estimates for graph scheduling.
//!
//! # Evolution
//!
//! The `primalspring_primal` binary exposes these capabilities via a
//! JSON-RPC server. The final form is graph-only deployment where
//! biomeOS orchestrates the niche directly from deploy graphs.

use std::path::Path;

use tracing::{info, warn};

use crate::ipc::client::PrimalClient;
use crate::ipc::discover::discover_primal;

/// Niche identity — delegates to the canonical [`crate::PRIMAL_NAME`].
pub const NICHE_NAME: &str = crate::PRIMAL_NAME;

/// Default registration target — discovered at runtime, not hardcoded.
/// Override via `BIOMEOS_PRIMAL` env var for non-standard deployments.
const REGISTRATION_TARGET: &str = crate::primal_names::BIOMEOS;

/// Capabilities this binary **locally serves** via `dispatch_request`.
///
/// These are the methods `primalspring_primal` actually handles — callers
/// get a real response, not a routing bounce. When registering with biomeOS,
/// these are claimed as "served here."
pub const LOCAL_CAPABILITIES: &[&str] = &[
    // ── Coordination (core domain) ──
    "coordination.validate_composition",
    "coordination.validate_composition_by_capability",
    "coordination.probe_primal",
    "coordination.probe_capability",
    "coordination.discovery_sweep",
    "coordination.deploy_atomic",
    "coordination.bonding_test",
    "coordination.neural_api_status",
    // ── Composition health ──
    "composition.nucleus_health",
    "composition.tower_health",
    "composition.node_health",
    "composition.nest_health",
    "composition.tower_squirrel_health",
    // ── Lifecycle management ──
    "nucleus.start",
    "nucleus.stop",
    "lifecycle.status",
    // ── Health probes ──
    "health.check",
    "health.liveness",
    "health.readiness",
    // ── Capability advertisement ──
    "capabilities.list",
    // ── Graph coordination ──
    "graph.validate",
    "graph.list",
    "graph.waves",
    "graph.capabilities",
    // ── MCP tool discovery ──
    "mcp.tools.list",
    // ── Ionic bond negotiation (Track 4) ──
    "bonding.propose",
    "bonding.status",
];

/// Ecosystem capabilities that primalSpring **coordinates but routes to
/// other primals** via biomeOS Neural API semantic routing.
///
/// These are registered as coordination metadata so biomeOS and springs
/// know primalSpring understands these domains. Callers reaching
/// primalSpring directly for these methods get `METHOD_NOT_FOUND` — they
/// should go through `capability.call` or the Neural API.
///
/// Each entry names the canonical provider per `capability_registry.toml`.
pub const ROUTED_CAPABILITIES: &[(&str, &str)] = {
    use crate::primal_names as pn;
    &[
        // ── Lifecycle (biomeOS) ──
        ("lifecycle.start", pn::BIOMEOS),
        ("lifecycle.stop", pn::BIOMEOS),
        // ── Graph execution (biomeOS) ──
        ("graph.deploy", pn::BIOMEOS),
        ("graph.status", pn::BIOMEOS),
        ("graph.rollback", pn::BIOMEOS),
        // ── AI (Squirrel) ──
        ("ai.query", pn::SQUIRREL),
        ("ai.health", pn::SQUIRREL),
        // ── Discovery mesh (Songbird) ──
        ("discovery.announce", pn::SONGBIRD),
        ("discovery.find_primals", pn::SONGBIRD),
        // ── Network (Songbird) ──
        ("network.stun", pn::SONGBIRD),
        ("network.nat_type", pn::SONGBIRD),
        ("network.birdsong.beacon", pn::SONGBIRD),
        ("network.birdsong.decrypt", pn::SONGBIRD),
        ("network.onion.start", pn::SONGBIRD),
        ("network.onion.status", pn::SONGBIRD),
        ("network.tor.status", pn::SONGBIRD),
        ("network.federation.peers", pn::SONGBIRD),
        // ── Federation (biomeOS v2.78+) ──
        ("federation.configure", pn::BIOMEOS),
        ("federation.join", pn::BIOMEOS),
        ("federation.health_check", pn::BIOMEOS),
        // ── Topology (biomeOS) ──
        ("topology.get", pn::BIOMEOS),
        ("topology.proprioception", pn::BIOMEOS),
        // ── Visualization (petalTongue) ──
        ("visualization.render", pn::PETALTONGUE),
        ("visualization.render.scene", pn::PETALTONGUE),
        ("visualization.render.stream", pn::PETALTONGUE),
        ("visualization.render.dashboard", pn::PETALTONGUE),
        ("visualization.render.grammar", pn::PETALTONGUE),
        ("interaction.subscribe", pn::PETALTONGUE),
        ("interaction.poll", pn::PETALTONGUE),
        // ── Crypto (BearDog) ──
        ("crypto.sign_ed25519", pn::BEARDOG),
        ("crypto.verify_ed25519", pn::BEARDOG),
        // ── Provenance DAG (rhizoCrypt) ──
        ("dag.session.create", pn::RHIZOCRYPT),
        ("dag.session.get", pn::RHIZOCRYPT),
        ("dag.session.list", pn::RHIZOCRYPT),
        ("dag.event.append", pn::RHIZOCRYPT),
        ("dag.vertex.get", pn::RHIZOCRYPT),
        ("dag.frontier.get", pn::RHIZOCRYPT),
        ("dag.merkle.root", pn::RHIZOCRYPT),
        ("dag.merkle.proof", pn::RHIZOCRYPT),
        ("dag.merkle.verify", pn::RHIZOCRYPT),
        ("dag.slice.checkout", pn::RHIZOCRYPT),
        ("dag.dehydration.trigger", pn::RHIZOCRYPT),
        // ── Ledger (loamSpine) ──
        ("spine.create", pn::LOAMSPINE),
        ("spine.get", pn::LOAMSPINE),
        ("spine.seal", pn::LOAMSPINE),
        ("entry.append", pn::LOAMSPINE),
        ("entry.get", pn::LOAMSPINE),
        ("certificate.mint", pn::LOAMSPINE),
        ("session.commit", pn::LOAMSPINE),
        // ── Attribution (sweetGrass) ──
        ("braid.create", pn::SWEETGRASS),
        ("braid.get", pn::SWEETGRASS),
        ("braid.commit", pn::SWEETGRASS),
        ("anchoring.anchor", pn::SWEETGRASS),
        ("anchoring.verify", pn::SWEETGRASS),
        ("provenance.graph", pn::SWEETGRASS),
        ("provenance.export_provo", pn::SWEETGRASS),
        ("attribution.chain", pn::SWEETGRASS),
    ]
};

/// All capabilities (local + routed method names) as a flat slice.
///
/// Backward-compatible accessor for code that needs the combined list.
/// Prefer [`LOCAL_CAPABILITIES`] when you need to know what this binary
/// actually serves.
#[must_use]
pub fn all_capabilities() -> Vec<&'static str> {
    let mut all = LOCAL_CAPABILITIES.to_vec();
    all.extend(ROUTED_CAPABILITIES.iter().map(|(method, _)| *method));
    all
}

/// Operation dependency hints for biomeOS Pathway Learner parallelization.
///
/// Maps each capability to the data it needs to be available before execution.
#[must_use]
pub fn operation_dependencies() -> serde_json::Value {
    serde_json::json!({
        "coordination.validate_composition": ["atomic_type"],
        "coordination.probe_primal": ["primal_name"],
        "coordination.discovery_sweep": ["atomic_type"],
        "coordination.deploy_atomic": ["atomic_type", "graph_path"],
        "coordination.bonding_test": ["bond_type", "family_id"],
        "composition.nucleus_health": [],
        "graph.validate": ["graph_path"],
        "graph.list": [],
        "coordination.neural_api_status": [],
    })
}

/// Cost estimates for biomeOS scheduling.
///
/// Coordination operations are IPC-bound (socket round-trips), not
/// compute-bound. Costs reflect expected latency for live primal probing.
/// Numeric values are sourced from [`crate::tolerances`] named constants.
#[must_use]
pub fn cost_estimates() -> serde_json::Value {
    use crate::tolerances;
    serde_json::json!({
        "coordination.validate_composition": {
            "latency_ms": tolerances::COST_VALIDATE_COMPOSITION_MS,
            "cpu": "low",
            "memory_bytes": tolerances::COST_VALIDATE_COMPOSITION_BYTES,
            "note": "probes N primals sequentially"
        },
        "coordination.probe_primal": {
            "latency_ms": tolerances::COST_PROBE_PRIMAL_MS,
            "cpu": "low",
            "memory_bytes": tolerances::COST_PROBE_PRIMAL_BYTES,
            "note": "single socket round-trip"
        },
        "coordination.discovery_sweep": {
            "latency_ms": tolerances::COST_DISCOVERY_SWEEP_MS,
            "cpu": "low",
            "memory_bytes": tolerances::COST_DISCOVERY_SWEEP_BYTES,
            "note": "filesystem probes + env vars"
        },
        "composition.nucleus_health": {
            "latency_ms": tolerances::COST_NUCLEUS_HEALTH_MS,
            "cpu": "low",
            "memory_bytes": tolerances::COST_NUCLEUS_HEALTH_BYTES,
            "note": "probes all NUCLEUS primals"
        },
        "graph.validate": {
            "latency_ms": tolerances::COST_GRAPH_VALIDATE_MS,
            "cpu": "low",
            "memory_bytes": tolerances::COST_GRAPH_VALIDATE_BYTES,
            "note": "TOML parse + structure check"
        },
        "health.check": {
            "latency_ms": tolerances::COST_HEALTH_CHECK_MS,
            "cpu": "low",
            "memory_bytes": tolerances::COST_HEALTH_CHECK_BYTES,
        },
    })
}

/// Semantic mappings for coordination capability domain routing.
///
/// Maps short operation names to fully qualified capability methods.
/// biomeOS uses these to route `capability.call` requests.
#[must_use]
pub fn coordination_semantic_mappings() -> serde_json::Value {
    serde_json::json!({
        "validate_composition":               "coordination.validate_composition",
        "validate_composition_by_capability":  "coordination.validate_composition_by_capability",
        "probe_primal":                        "coordination.probe_primal",
        "probe_capability":                    "coordination.probe_capability",
        "discovery_sweep":                     "coordination.discovery_sweep",
        "deploy_atomic":                       "coordination.deploy_atomic",
        "bonding_test":                        "coordination.bonding_test",
        "nucleus_health":                      "composition.nucleus_health",
        "tower_health":                        "composition.tower_health",
        "node_health":                         "composition.node_health",
        "nest_health":                         "composition.nest_health",
        "neural_api_status":                   "coordination.neural_api_status",
        "graph_validate":                      "graph.validate",
        "graph_list":                          "graph.list",
    })
}

/// Register this niche's capabilities with biomeOS.
///
/// Discovers biomeOS at runtime, then sends `lifecycle.register` followed
/// by `capability.register` for each domain and individual capability.
/// Degrades gracefully if biomeOS is unreachable — coordination must not
/// depend on registration success.
pub fn register_with_target(our_socket: &Path) {
    let target = std::env::var(crate::env_keys::BIOMEOS_PRIMAL).unwrap_or_else(|_| REGISTRATION_TARGET.to_owned());
    let discovery = discover_primal(&target);
    let Some(biomeos_path) = discovery.socket else {
        info!(target: "niche", primal = %target, "registration target not discovered — deferred");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(&biomeos_path, &target) else {
        warn!(target: "niche", primal = %target, "cannot connect to registration target — skipping");
        return;
    };

    let sock_str = our_socket.to_string_lossy().to_string();

    let reg_result = client.call(
        "lifecycle.register",
        serde_json::json!({
            "name": NICHE_NAME,
            "socket_path": &sock_str,
            "pid": std::process::id(),
            "domain": crate::PRIMAL_DOMAIN,
            "version": env!("CARGO_PKG_VERSION"),
        }),
    );

    if reg_result.is_ok() {
        info!(target: "biomeos", "registered with lifecycle manager");
    } else {
        warn!(target: "biomeos", "lifecycle.register failed (non-fatal)");
    }

    let domains: &[(&str, serde_json::Value)] = &[
        ("coordination", coordination_semantic_mappings()),
        (
            "composition",
            serde_json::json!({
                "nucleus_health": "composition.nucleus_health",
                "tower_health":   "composition.tower_health",
                "node_health":    "composition.node_health",
                "nest_health":    "composition.nest_health",
            }),
        ),
        (
            "graph",
            serde_json::json!({
                "validate": "graph.validate",
                "list":     "graph.list",
            }),
        ),
    ];

    for (domain, mappings) in domains {
        let mut payload = serde_json::json!({
            "primal": NICHE_NAME,
            "capability": domain,
            "socket": &sock_str,
            "semantic_mappings": mappings,
        });
        if *domain == "coordination" {
            payload["operation_dependencies"] = operation_dependencies();
            payload["cost_estimates"] = cost_estimates();
        }
        let _ = client.call("capability.register", payload);
    }

    let mut registered = 0u32;
    for cap in LOCAL_CAPABILITIES {
        if client
            .call(
                "capability.register",
                serde_json::json!({
                    "primal": NICHE_NAME,
                    "capability": cap,
                    "socket": &sock_str,
                    "served_locally": true,
                }),
            )
            .is_ok()
        {
            registered += 1;
        } else {
            warn!(target: "biomeos", capability = cap, "capability.register failed (non-fatal)");
        }
    }

    for (cap, provider) in ROUTED_CAPABILITIES {
        let _ = client.call(
            "capability.register",
            serde_json::json!({
                "primal": NICHE_NAME,
                "capability": cap,
                "socket": &sock_str,
                "served_locally": false,
                "canonical_provider": provider,
            }),
        );
    }

    let total = LOCAL_CAPABILITIES.len() + ROUTED_CAPABILITIES.len();
    info!(
        target: "biomeos",
        registered,
        local = LOCAL_CAPABILITIES.len(),
        routed = ROUTED_CAPABILITIES.len(),
        total,
        domains = domains.len(),
        "capabilities + domains registered",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_capabilities_are_not_empty() {
        assert!(!LOCAL_CAPABILITIES.is_empty());
    }

    #[test]
    fn routed_capabilities_are_not_empty() {
        assert!(!ROUTED_CAPABILITIES.is_empty());
    }

    #[test]
    fn all_capabilities_follow_semantic_naming() {
        let all = all_capabilities();
        for cap in &all {
            assert!(
                cap.contains('.'),
                "capability '{cap}' should follow domain.operation format"
            );
        }
    }

    #[test]
    fn no_duplicate_capabilities() {
        let mut seen = std::collections::HashSet::new();
        let all = all_capabilities();
        for cap in &all {
            assert!(seen.insert(cap), "duplicate capability: {cap}");
        }
    }

    #[test]
    fn local_and_routed_are_disjoint() {
        for (routed_method, _) in ROUTED_CAPABILITIES {
            assert!(
                !LOCAL_CAPABILITIES.contains(routed_method),
                "'{routed_method}' is in both LOCAL and ROUTED — pick one"
            );
        }
    }

    #[test]
    fn routed_capabilities_have_providers() {
        for (method, provider) in ROUTED_CAPABILITIES {
            assert!(
                !provider.is_empty(),
                "routed capability '{method}' has empty provider"
            );
        }
    }

    #[test]
    fn operation_dependencies_is_object() {
        let deps = operation_dependencies();
        assert!(deps.is_object());
    }

    #[test]
    fn cost_estimates_is_object() {
        let costs = cost_estimates();
        assert!(costs.is_object());
    }

    #[test]
    fn semantic_mappings_cover_coordination_capabilities() {
        let mappings = coordination_semantic_mappings();
        let map = mappings.as_object().unwrap();
        let coord_caps: Vec<&&str> = LOCAL_CAPABILITIES
            .iter()
            .filter(|c| c.starts_with("coordination."))
            .collect();
        for cap in &coord_caps {
            assert!(
                map.values().any(|v| v.as_str() == Some(*cap)),
                "coordination capability '{cap}' should appear in semantic mappings"
            );
        }
    }

    #[test]
    fn niche_name_matches_convention() {
        assert_eq!(NICHE_NAME, crate::PRIMAL_NAME);
        assert!(NICHE_NAME.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn capabilities_match_registry_toml() {
        let toml_str = include_str!("../../config/capability_registry.toml");
        let parsed: toml::Value = toml::from_str(toml_str).unwrap();
        let caps_in_toml: Vec<&str> = parsed["capabilities"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|c| c.get("method")?.as_str())
            .collect();

        let all = all_capabilities();
        for code_cap in &all {
            assert!(
                caps_in_toml.contains(code_cap),
                "capability '{code_cap}' is in niche but missing from \
                 config/capability_registry.toml"
            );
        }
        for toml_cap in &caps_in_toml {
            assert!(
                all.contains(toml_cap),
                "capability '{toml_cap}' is in capability_registry.toml but missing from \
                 niche capabilities"
            );
        }
    }

    #[test]
    fn cost_estimates_have_latency() {
        let costs = cost_estimates();
        let map = costs.as_object().unwrap();
        for (key, val) in map {
            assert!(
                val.get("latency_ms").is_some(),
                "cost estimate for '{key}' missing latency_ms"
            );
        }
    }

    #[test]
    fn register_with_target_graceful_when_biomeos_unreachable() {
        let sock = std::path::Path::new("/tmp/primalspring-niche-test-nonexistent.sock");
        register_with_target(sock);
    }

    #[test]
    fn cost_estimates_have_memory() {
        let costs = cost_estimates();
        let map = costs.as_object().unwrap();
        for (key, val) in map {
            assert!(
                val.get("memory_bytes").is_some(),
                "cost estimate for '{key}' missing memory_bytes"
            );
        }
    }

    #[test]
    fn cost_estimates_cover_core_operations() {
        let costs = cost_estimates();
        let map = costs.as_object().unwrap();
        let expected_ops = [
            "coordination.validate_composition",
            "coordination.probe_primal",
            "coordination.discovery_sweep",
            "composition.nucleus_health",
            "graph.validate",
            "health.check",
        ];
        for op in expected_ops {
            assert!(map.contains_key(op), "missing cost estimate for '{op}'");
        }
    }

    #[test]
    fn operation_dependencies_cover_core_operations() {
        let deps = operation_dependencies();
        let map = deps.as_object().unwrap();
        assert!(map.contains_key("coordination.validate_composition"));
        assert!(map.contains_key("coordination.probe_primal"));
        assert!(map.contains_key("graph.validate"));
    }

    #[test]
    fn semantic_mappings_values_exist_in_capabilities() {
        let mappings = coordination_semantic_mappings();
        let map = mappings.as_object().unwrap();
        let all = all_capabilities();
        for (key, val) in map {
            let method = val.as_str().unwrap();
            assert!(
                all.contains(&method),
                "semantic mapping '{key}' → '{method}' not in capabilities"
            );
        }
    }

    #[test]
    fn registration_target_is_biomeos() {
        assert_eq!(REGISTRATION_TARGET, "biomeos");
    }
}
