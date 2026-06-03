// SPDX-License-Identifier: AGPL-3.0-or-later

//! Niche deployment self-knowledge for primalSpring.
//!
//! Uses the typed [`Primal`] enum for routed capabilities — the compiler
//! enforces that every routing target is a real primal.
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
use crate::primal_names::Primal;

/// Niche identity — delegates to the canonical [`crate::PRIMAL_NAME`].
pub const NICHE_NAME: &str = crate::PRIMAL_NAME;

/// Default registration target slug (fallback when capability discovery fails).
/// Override via `BIOMEOS_PRIMAL` env var for non-standard deployments.
const REGISTRATION_TARGET_FALLBACK: &str = crate::primal_names::BIOMEOS;

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
    "health.drain",
    "health.liveness",
    "health.readiness",
    "health.version",
    // ── Capability advertisement ──
    "capabilities.list",
    "capability.list",
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
pub const ROUTED_CAPABILITIES: &[(&str, Primal)] = {
    use Primal::{BiomeOS, Squirrel, Songbird, PetalTongue, BearDog, RhizoCrypt, LoamSpine, SweetGrass};
    &[
        // ── Lifecycle (biomeOS) ──
        ("lifecycle.start", BiomeOS),
        ("lifecycle.stop", BiomeOS),
        // ── Graph execution (biomeOS) ──
        ("graph.deploy", BiomeOS),
        ("graph.status", BiomeOS),
        ("graph.rollback", BiomeOS),
        // ── AI (Squirrel) ──
        ("ai.query", Squirrel),
        ("ai.health", Squirrel),
        // ── Discovery mesh (Songbird) ──
        ("discovery.announce", Songbird),
        ("discovery.find_primals", Songbird),
        // ── Network (Songbird) ──
        ("network.stun", Songbird),
        ("network.nat_type", Songbird),
        ("network.birdsong.beacon", Songbird),
        ("network.birdsong.decrypt", Songbird),
        ("network.onion.start", Songbird),
        ("network.onion.status", Songbird),
        ("network.tor.status", Songbird),
        ("network.federation.peers", Songbird),
        // ── Federation (biomeOS v2.78+) ──
        ("federation.configure", BiomeOS),
        ("federation.join", BiomeOS),
        ("federation.health_check", BiomeOS),
        // ── Topology (biomeOS) ──
        ("topology.get", BiomeOS),
        ("topology.proprioception", BiomeOS),
        // ── Visualization (petalTongue) ──
        ("visualization.render", PetalTongue),
        ("visualization.render.scene", PetalTongue),
        ("visualization.render.stream", PetalTongue),
        ("visualization.render.dashboard", PetalTongue),
        ("visualization.render.grammar", PetalTongue),
        ("interaction.subscribe", PetalTongue),
        ("interaction.poll", PetalTongue),
        // ── Crypto (BearDog) ──
        ("crypto.sign_ed25519", BearDog),
        ("crypto.verify_ed25519", BearDog),
        // ── Provenance DAG (rhizoCrypt) ──
        ("dag.session.create", RhizoCrypt),
        ("dag.session.get", RhizoCrypt),
        ("dag.session.list", RhizoCrypt),
        ("dag.event.append", RhizoCrypt),
        ("dag.vertex.get", RhizoCrypt),
        ("dag.frontier.get", RhizoCrypt),
        ("dag.merkle.root", RhizoCrypt),
        ("dag.merkle.proof", RhizoCrypt),
        ("dag.merkle.verify", RhizoCrypt),
        ("dag.slice.checkout", RhizoCrypt),
        ("dag.dehydration.trigger", RhizoCrypt),
        // ── Ledger (loamSpine) ──
        ("spine.create", LoamSpine),
        ("spine.get", LoamSpine),
        ("spine.seal", LoamSpine),
        ("entry.append", LoamSpine),
        ("entry.get", LoamSpine),
        ("certificate.mint", LoamSpine),
        ("session.commit", LoamSpine),
        // ── Attribution (sweetGrass) ──
        ("braid.create", SweetGrass),
        ("braid.get", SweetGrass),
        ("braid.commit", SweetGrass),
        ("anchoring.anchor", SweetGrass),
        ("anchoring.verify", SweetGrass),
        ("provenance.graph", SweetGrass),
        ("provenance.export_provo", SweetGrass),
        ("attribution.chain", SweetGrass),
        // ── impulsePotential (membrane-shadow → primal graduation) ──
        // Shadow provider: membrane. Routes through BiomeOS for graph composition
        // when primals are available; membrane-shadow fallback for direct fs/git.
        ("impulse.post", BiomeOS),
        ("impulse.ack", BiomeOS),
        ("impulse.archive", BiomeOS),
        ("potential.sense", BiomeOS),
        ("potential.check", BiomeOS),
        // ── Context braids (membrane-shadow → sweetGrass graduation) ──
        ("context.weave", BiomeOS),
        ("context.sense", BiomeOS),
        ("context.clear", BiomeOS),
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

/// Register this niche's capabilities with biomeOS via `announce_or_register`.
///
/// Tries `primal.announce` (biomeOS v3.57+) first — single-call atomic
/// registration. Falls back to the legacy 3-call pattern
/// (`lifecycle.register` then `capability.register` per domain and per
/// capability) when announce is unavailable. This is the canonical
/// backward-compatible approach endorsed by the ecosystem
/// (`wateringHole/SIGNAL_ADOPTION_STANDARD.md`).
///
/// Degrades gracefully if biomeOS is unreachable — coordination must not
/// depend on registration success.
pub fn register_with_target(our_socket: &Path) {
    let orchestration = crate::ipc::discover::discover_by_capability("orchestration");
    let (target_path, target_label) = if let Some(socket) = orchestration.socket {
        (socket, "orchestration".to_owned())
    } else {
        let fallback = std::env::var(crate::env_keys::BIOMEOS_PRIMAL)
            .unwrap_or_else(|_| REGISTRATION_TARGET_FALLBACK.to_owned());
        let disc = discover_primal(&fallback);
        let Some(s) = disc.socket else {
            info!(target: "niche", "registration target not discovered — deferred");
            return;
        };
        (s, fallback)
    };

    let Ok(mut client) = PrimalClient::connect(&target_path, &target_label) else {
        warn!(target: "niche", target = %target_label, "cannot connect to registration target — skipping");
        return;
    };

    let sock_str = our_socket.to_string_lossy().to_string();

    // ── Try primal.announce first (biomeOS v3.57+) ──
    if try_announce(&mut client, &sock_str) {
        return;
    }

    // ── Fallback: legacy 3-call registration ──
    legacy_register(&mut client, &sock_str);
}

/// Atomic announce via `primal.announce` (Wave 17+ pattern).
///
/// Returns `true` if announce succeeded, `false` to trigger legacy fallback.
fn try_announce(client: &mut PrimalClient, sock_str: &str) -> bool {
    let all_methods: Vec<&str> = all_capabilities();
    let announce_params = serde_json::json!({
        "primal_id": NICHE_NAME,
        "transport": sock_str,
        "methods": all_methods,
        "lifecycle": {
            "state": "running",
            "pid": std::process::id(),
            "version": env!("CARGO_PKG_VERSION"),
        },
        "domain": crate::PRIMAL_DOMAIN,
        "composition_tiers": ["tower", "node", "nest", "nucleus", "meta"],
    });

    match client.call("primal.announce", announce_params) {
        Ok(resp) if resp.is_success() => {
            info!(
                target: "biomeos",
                methods = all_methods.len(),
                "primal.announce succeeded (atomic registration)"
            );
            true
        }
        Ok(_) => {
            info!(target: "biomeos", "primal.announce returned error — falling back to legacy");
            false
        }
        Err(_) => {
            info!(target: "biomeos", "primal.announce unavailable — falling back to legacy");
            false
        }
    }
}

/// Legacy 3-call registration (pre-v3.57 biomeOS).
fn legacy_register(client: &mut PrimalClient, sock_str: &str) {
    let reg_result = client.call(
        "lifecycle.register",
        serde_json::json!({
            "name": NICHE_NAME,
            "socket_path": sock_str,
            "pid": std::process::id(),
            "domain": crate::PRIMAL_DOMAIN,
            "version": env!("CARGO_PKG_VERSION"),
        }),
    );

    if reg_result.is_ok() {
        info!(target: "biomeos", "registered with lifecycle manager (legacy)");
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
            "socket": sock_str,
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
                    "socket": sock_str,
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
                "socket": sock_str,
                "served_locally": false,
                "canonical_provider": provider.slug(),
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
        "capabilities + domains registered (legacy)",
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
    fn routed_capabilities_have_valid_providers() {
        for (method, provider) in ROUTED_CAPABILITIES {
            assert!(
                !provider.slug().is_empty(),
                "routed capability '{method}' has empty provider slug"
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

        let table = parsed.as_table().expect("registry should be a TOML table");
        let mut caps_in_toml: Vec<&str> = Vec::new();
        for (_domain, section) in table {
            if let Some(methods) = section.get("methods").and_then(|m| m.as_array()) {
                for m in methods {
                    if let Some(s) = m.as_str() {
                        caps_in_toml.push(s);
                    }
                }
            }
        }

        let all = all_capabilities();
        for code_cap in &all {
            assert!(
                caps_in_toml.contains(code_cap),
                "capability '{code_cap}' is in niche but missing from \
                 config/capability_registry.toml"
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
    fn registration_target_fallback_is_biomeos() {
        assert_eq!(REGISTRATION_TARGET_FALLBACK, "biomeos");
    }
}
