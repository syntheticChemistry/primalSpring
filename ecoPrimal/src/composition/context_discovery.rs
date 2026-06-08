// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery escalation for [`super::context::CompositionContext`].
//!
//! Extracted from `context.rs` to keep each module under the 800-line threshold.
//! All five discovery tiers live here; the `CompositionContext` constructors
//! call into these functions and consume the results.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use crate::ipc::client::PrimalClient;

use super::btsp::{tcp_fallback_table, upgrade_btsp_clients};
use super::context::DiscoveryPath;
use super::routing::{ALL_CAPS, CapabilityDomain, capability_to_primal};

/// Intermediate result from a discovery pass — consumed by `CompositionContext` constructors.
pub(super) struct DiscoveryResult {
    pub clients: HashMap<CapabilityDomain, PrimalClient>,
    pub btsp_state: BTreeMap<CapabilityDomain, bool>,
    pub discovery_paths: BTreeMap<CapabilityDomain, DiscoveryPath>,
}

/// Returns `true` when Tier 5 TCP port probing is explicitly enabled.
///
/// The zero-port Tower Atomic standard treats TCP port exposure as metadata
/// leakage. Tier 5 is off by default; set `PRIMALSPRING_TCP_TIER5=1` for
/// containers, Android, or deployments without Unix domain sockets.
///
/// In release builds, TCP Tier 5 is unconditionally disabled — the env var
/// is ignored. This enforces the glacial zero-port standard at compile time.
pub(super) fn tcp_tier5_enabled() -> bool {
    #[cfg(debug_assertions)]
    {
        std::env::var(crate::env_keys::PRIMALSPRING_TCP_TIER5)
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }
    #[cfg(not(debug_assertions))]
    {
        false
    }
}

/// Full 5-tier discovery escalation (Songbird → Neural API → UDS → registry → TCP).
pub(super) fn discover_full() -> DiscoveryResult {
    let mut clients = HashMap::new();
    let mut discovery_paths = BTreeMap::new();

    // Tier 1: Songbird routing
    if let Ok(songbird) = crate::ipc::client::connect_by_capability("discovery") {
        clients.insert(CapabilityDomain::from("discovery"), songbird);
        discovery_paths.insert(CapabilityDomain::from("discovery"), DiscoveryPath::Songbird);

        let caps_to_resolve: Vec<&str> = ALL_CAPS
            .iter()
            .copied()
            .filter(|&c| c != "discovery")
            .collect();

        for cap in caps_to_resolve {
            let primal = capability_to_primal(cap);
            let resolve_result = clients
                .get_mut("discovery")
                .and_then(|sb| {
                    sb.call("ipc.resolve", serde_json::json!({"primal_id": primal}))
                        .ok()
                })
                .and_then(|resp| resp.result);

            if let Some(result) = resolve_result {
                let socket_path = result
                    .get("socket")
                    .or_else(|| result.get("native_endpoint"))
                    .or_else(|| result.get("endpoint"))
                    .and_then(serde_json::Value::as_str)
                    .map(PathBuf::from);

                if let Some(path) = socket_path {
                    if let Ok(client) = PrimalClient::connect(&path, primal) {
                        tracing::debug!(cap, primal, tier = 1, "discovered via Songbird");
                        clients.insert(CapabilityDomain::from(cap), client);
                        discovery_paths.insert(CapabilityDomain::from(cap), DiscoveryPath::Songbird);
                    }
                }
            }
        }
    }

    // Tiers 2-4: Neural API, UDS convention, registry
    discover_tiers_2_4(&mut clients, &mut discovery_paths);

    // Tier 5: TCP probing (opt-in)
    if tcp_tier5_enabled() {
        discover_tier5_tcp(&mut clients, &mut discovery_paths);
    }

    // BTSP escalation
    let btsp_state = upgrade_btsp_clients(&mut clients);
    DiscoveryResult { clients, btsp_state, discovery_paths }
}

/// Tiers 2-4 only (Neural API, UDS convention, socket registry).
pub(super) fn discover_live() -> DiscoveryResult {
    let mut clients = HashMap::new();
    let mut discovery_paths = BTreeMap::new();
    discover_tiers_2_4(&mut clients, &mut discovery_paths);
    let btsp_state: BTreeMap<CapabilityDomain, bool> = clients
        .iter()
        .map(|(cap, c)| (cap.clone(), c.is_btsp_authenticated()))
        .collect();
    DiscoveryResult { clients, btsp_state, discovery_paths }
}

/// Tiers 2-5 (Neural API + UDS + registry + gated TCP fallback).
///
/// TCP Tier 5 is only attempted when `tcp_tier5_enabled()` returns true
/// (debug builds + `PRIMALSPRING_TCP_TIER5=1`). In Tower Atomic posture,
/// Songbird handles cross-gate routing — direct TCP is standalone debris.
pub(super) fn discover_with_fallback() -> DiscoveryResult {
    let mut clients = HashMap::new();
    let mut discovery_paths = BTreeMap::new();

    let cap_to_primal = tcp_fallback_table();

    for &(cap, _primal, _port_env, _default_port) in &cap_to_primal {
        if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
            clients.insert(CapabilityDomain::from(cap), client);
            discovery_paths.insert(CapabilityDomain::from(cap), DiscoveryPath::LocalDiscovery);
        }
    }

    if tcp_tier5_enabled() {
        let host = std::env::var(crate::env_keys::PRIMALSPRING_HOST)
            .unwrap_or_else(|_| crate::tolerances::DEFAULT_HOST.to_owned());
        for &(cap, primal, port_env, default_port) in &cap_to_primal {
            if clients.contains_key(cap) {
                continue;
            }
            let port: u16 = std::env::var(port_env)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(default_port);
            let addr = format!("{host}:{port}");
            if let Ok(client) = PrimalClient::connect_tcp(&addr, primal) {
                clients.insert(CapabilityDomain::from(cap), client);
                discovery_paths.insert(CapabilityDomain::from(cap), DiscoveryPath::TcpFallback);
            }
        }
    }

    let btsp_state = upgrade_btsp_clients(&mut clients);
    DiscoveryResult { clients, btsp_state, discovery_paths }
}

/// Re-discover capabilities, keeping existing live connections.
pub(super) fn rediscover_clients(
    clients: &mut HashMap<CapabilityDomain, PrimalClient>,
    discovery_paths: &mut BTreeMap<CapabilityDomain, DiscoveryPath>,
) -> BTreeMap<CapabilityDomain, bool> {
    for &cap in ALL_CAPS.iter() {
        if clients.contains_key(cap) {
            if let Some(client) = clients.get_mut(cap) {
                if client.health_check().unwrap_or(false) {
                    continue;
                }
            }
        }
        if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
            tracing::info!(cap, "rediscovered capability after topology change");
            clients.insert(CapabilityDomain::from(cap), client);
            discovery_paths.insert(CapabilityDomain::from(cap), DiscoveryPath::LocalDiscovery);
        }
    }
    upgrade_btsp_clients(clients)
}

/// Tiers 2-4: connect by capability for each ALL_CAPS entry.
fn discover_tiers_2_4(
    clients: &mut HashMap<CapabilityDomain, PrimalClient>,
    discovery_paths: &mut BTreeMap<CapabilityDomain, DiscoveryPath>,
) {
    for &cap in ALL_CAPS.iter() {
        if clients.contains_key(cap) {
            continue;
        }
        if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
            tracing::debug!(cap, tier = "2-4", "discovered via UDS/Neural API");
            clients.insert(CapabilityDomain::from(cap), client);
            discovery_paths.insert(CapabilityDomain::from(cap), DiscoveryPath::LocalDiscovery);
        }
    }
}

/// Tier 5: TCP port probing on well-known ports.
fn discover_tier5_tcp(
    clients: &mut HashMap<CapabilityDomain, PrimalClient>,
    discovery_paths: &mut BTreeMap<CapabilityDomain, DiscoveryPath>,
) {
    let host = std::env::var(crate::env_keys::PRIMALSPRING_HOST)
        .unwrap_or_else(|_| crate::tolerances::DEFAULT_HOST.to_owned());
    for &(cap, primal, port_env, default_port) in &tcp_fallback_table() {
        if clients.contains_key(cap) {
            continue;
        }
        let port: u16 = std::env::var(port_env)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_port);
        let addr = format!("{host}:{port}");
        if let Ok(client) = PrimalClient::connect_tcp(&addr, primal) {
            tracing::debug!(cap, primal, %addr, tier = 5, "discovered via TCP");
            clients.insert(CapabilityDomain::from(cap), client);
            discovery_paths.insert(CapabilityDomain::from(cap), DiscoveryPath::TcpFallback);
        }
    }
}
