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
const REGISTRATION_TARGET: &str = "biomeos";

/// All capabilities this niche exposes to biomeOS.
///
/// Source: `primalspring_deploy.toml` node 9 capabilities + server RPC methods.
pub const CAPABILITIES: &[&str] = &[
    // ── Coordination (core domain) ──
    "coordination.validate_composition",
    "coordination.probe_primal",
    "coordination.discovery_sweep",
    "coordination.deploy_atomic",
    "coordination.bonding_test",
    // ── Composition health ──
    "composition.nucleus_health",
    "composition.tower_health",
    "composition.node_health",
    "composition.nest_health",
    // ── Lifecycle management ──
    "nucleus.start",
    "nucleus.stop",
    "lifecycle.status",
    // ── Health probes (biomeOS orchestration) ──
    "health.check",
    "health.liveness",
    "health.readiness",
    // ── Capability advertisement ──
    "capabilities.list",
    // ── Graph coordination ──
    "graph.validate",
    "graph.list",
    // ── Neural API bridge ──
    "coordination.neural_api_status",
    // ── MCP tool discovery ──
    "mcp.tools.list",
];

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
        "validate_composition":  "coordination.validate_composition",
        "probe_primal":          "coordination.probe_primal",
        "discovery_sweep":       "coordination.discovery_sweep",
        "deploy_atomic":         "coordination.deploy_atomic",
        "bonding_test":          "coordination.bonding_test",
        "nucleus_health":        "composition.nucleus_health",
        "tower_health":          "composition.tower_health",
        "node_health":           "composition.node_health",
        "nest_health":           "composition.nest_health",
        "neural_api_status":     "coordination.neural_api_status",
        "graph_validate":        "graph.validate",
        "graph_list":            "graph.list",
    })
}

/// Register this niche's capabilities with biomeOS.
///
/// Discovers biomeOS at runtime, then sends `lifecycle.register` followed
/// by `capability.register` for each domain and individual capability.
/// Degrades gracefully if biomeOS is unreachable — coordination must not
/// depend on registration success.
pub fn register_with_target(our_socket: &Path) {
    let target = std::env::var("BIOMEOS_PRIMAL").unwrap_or_else(|_| REGISTRATION_TARGET.to_owned());
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
    for cap in CAPABILITIES {
        if client
            .call(
                "capability.register",
                serde_json::json!({
                    "primal": NICHE_NAME,
                    "capability": cap,
                    "socket": &sock_str,
                }),
            )
            .is_ok()
        {
            registered += 1;
        } else {
            warn!(target: "biomeos", capability = cap, "capability.register failed (non-fatal)");
        }
    }

    info!(
        target: "biomeos",
        registered,
        total = CAPABILITIES.len(),
        domains = domains.len(),
        "capabilities + domains registered",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_are_not_empty() {
        assert!(!CAPABILITIES.is_empty());
    }

    #[test]
    fn capabilities_follow_semantic_naming() {
        for cap in CAPABILITIES {
            assert!(
                cap.contains('.'),
                "capability '{cap}' should follow domain.operation format"
            );
        }
    }

    #[test]
    fn no_duplicate_capabilities() {
        let mut seen = std::collections::HashSet::new();
        for cap in CAPABILITIES {
            assert!(seen.insert(cap), "duplicate capability: {cap}");
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
        let coord_caps: Vec<&&str> = CAPABILITIES
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

        for code_cap in CAPABILITIES {
            assert!(
                caps_in_toml.contains(code_cap),
                "capability '{code_cap}' is in niche::CAPABILITIES but missing from \
                 config/capability_registry.toml"
            );
        }
        for toml_cap in &caps_in_toml {
            assert!(
                CAPABILITIES.contains(toml_cap),
                "capability '{toml_cap}' is in capability_registry.toml but missing from \
                 niche::CAPABILITIES"
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
}
