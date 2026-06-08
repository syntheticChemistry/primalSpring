// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! BTSP escalation and TCP fallback for composition clients.

use std::collections::{BTreeMap, HashMap};

use crate::ipc::client::PrimalClient;

use super::routing::{ALL_CAPS, BTSP_EXTRA_CAPS, CapabilityDomain, capability_to_primal};

/// Prefer family-scoped sockets over capability aliases for BTSP handshakes.
///
/// Capability aliases (e.g. `shader.sock`) may point to symlinks without
/// active BTSP listeners. Family-scoped sockets (`coralreef-default.sock`)
/// are the canonical endpoint where BTSP listeners are bound.
pub fn resolve_btsp_socket(discovered: &std::path::Path, primal: &str) -> std::path::PathBuf {
    let name = discovered
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let family = crate::env_keys::resolve_family_id();
    let family_marker = format!("{primal}-{family}");

    if name.contains(&family_marker) {
        return discovered.to_path_buf();
    }

    let family_path = crate::ipc::discover::socket_path(primal);
    if family_path.exists() {
        tracing::debug!(
            %primal,
            alias = %discovered.display(),
            family = %family_path.display(),
            "BTSP: preferring family-scoped socket over alias"
        );
        return family_path;
    }

    discovered.to_path_buf()
}

/// Probe a cleartext client for `btsp.capabilities` to check whether the
/// primal advertises BTSP server support before attempting a handshake.
///
/// Returns `true` if the primal responds with a capabilities object that
/// includes a truthy `server` field, or if the method returns any non-error
/// result (indicating BTSP awareness). Returns `false` if the call fails
/// or the primal does not support the method (-32601).
fn supports_btsp(client: &mut PrimalClient) -> bool {
    match client.call("btsp.capabilities", serde_json::json!({})) {
        Ok(resp) => resp
            .result
            .as_ref()
            .and_then(|v| v.get("server"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or_else(|| resp.is_success()),
        Err(_) => false,
    }
}

/// Escalate discovered clients to BTSP.
///
/// For each capability, probes `btsp.capabilities` on the cleartext client
/// first. If the primal does not advertise BTSP server support, the
/// handshake is skipped (fixing the healthSpring mixed-deployment issue
/// where unconditional negotiation broke peers without BTSP listeners).
///
/// Returns a `BTreeMap<capability, btsp_authenticated>` for guidestone reporting.
pub fn upgrade_btsp_clients(clients: &mut HashMap<CapabilityDomain, PrimalClient>) -> BTreeMap<CapabilityDomain, bool> {
    let mut state: BTreeMap<CapabilityDomain, bool> =
        clients.keys().map(|cap| (cap.clone(), false)).collect();

    let Some(beacon) = crate::ipc::btsp_handshake::mito_beacon_from_env() else {
        return state;
    };
    let seed = beacon.key_bytes().to_vec();

    let proactive: Vec<&str> = ALL_CAPS
        .iter()
        .chain(BTSP_EXTRA_CAPS.iter())
        .copied()
        .collect();

    let all_caps: Vec<CapabilityDomain> = clients.keys().cloned().collect();

    for cap in &all_caps {
        let cap_str = cap.as_str();
        let primal = capability_to_primal(cap_str);
        let result = crate::ipc::discover::discover_by_capability(cap_str);
        if let Some(discovered_path) = result.socket {
            let path = resolve_btsp_socket(&discovered_path, primal);

            let btsp_supported = clients
                .get_mut(cap_str)
                .is_some_and(supports_btsp);

            if !btsp_supported {
                tracing::debug!(cap = cap_str, primal, "btsp.capabilities: not supported, skipping BTSP upgrade");
                continue;
            }

            if proactive.contains(&cap_str) {
                match PrimalClient::connect_btsp(&path, primal, &seed) {
                    Ok(btsp_client) => {
                        tracing::info!(cap = cap_str, primal, "BTSP authenticated (proactive)");
                        clients.insert(cap.clone(), btsp_client);
                        state.insert(cap.clone(), true);
                    }
                    Err(e) => {
                        tracing::debug!(
                            cap = cap_str,
                            primal,
                            ?e,
                            "BTSP upgrade failed, re-establishing cleartext"
                        );
                        if let Ok(fresh) = PrimalClient::connect(&discovered_path, primal) {
                            clients.insert(cap.clone(), fresh);
                        }
                    }
                }
            } else {
                let rejected = clients.get_mut(cap_str).is_some_and(|c| {
                    matches!(
                        c.call("health.liveness", serde_json::json!({})),
                        Err(e) if e.is_connection_error() || e.is_protocol_error()
                    )
                });

                if rejected {
                    match PrimalClient::connect_btsp(&path, primal, &seed) {
                        Ok(btsp_client) => {
                            tracing::info!(cap = cap_str, primal, "BTSP authenticated (reactive)");
                            clients.insert(cap.clone(), btsp_client);
                            state.insert(cap.clone(), true);
                        }
                        Err(e) => {
                            tracing::debug!(
                                cap = cap_str,
                                primal,
                                ?e,
                                "reactive BTSP failed, reconnecting cleartext"
                            );
                            if let Ok(fresh) = PrimalClient::connect(&path, primal) {
                                clients.insert(cap.clone(), fresh);
                            }
                        }
                    }
                }
            }
        }
    }

    for &cap in ALL_CAPS.iter() {
        if clients.contains_key(cap) {
            continue;
        }
        let result = crate::ipc::discover::discover_by_capability(cap);
        if let Some(discovered_path) = result.socket {
            let primal = capability_to_primal(cap);
            let path = resolve_btsp_socket(&discovered_path, primal);
            match PrimalClient::connect_btsp(&path, primal, &seed) {
                Ok(btsp_client) => {
                    tracing::info!(
                        cap,
                        primal,
                        "BTSP authenticated (BTSP-first, no cleartext client)"
                    );
                    clients.insert(CapabilityDomain::from(cap), btsp_client);
                    state.insert(CapabilityDomain::from(cap), true);
                }
                Err(e) => {
                    tracing::debug!(cap, primal, ?e, "BTSP-first connection failed");
                }
            }
        }
    }

    state
}

/// Capability → (primal slug, env var, default port) for tier 5 TCP probing.
///
/// Derived from `ALL_CAPS` + `capability_to_primal()` + `PORT_REGISTRY` —
/// no hand-maintained mapping needed. Each capability in `ALL_CAPS` is
/// resolved to its owning primal via the TOML-derived routing table, then
/// the port registry provides the env key and default port.
#[must_use]
pub fn tcp_fallback_table() -> Vec<(&'static str, &'static str, &'static str, u16)> {
    use crate::tolerances;

    ALL_CAPS
        .iter()
        .filter_map(|&cap| {
            let slug = capability_to_primal(cap);
            tolerances::port_entry_for(slug)
                .map(|e| (cap, slug, e.env_key, e.port))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::routing::ALL_CAPS;

    #[test]
    fn tcp_fallback_covers_all_caps() {
        let table = tcp_fallback_table();
        let tcp_caps: Vec<&str> = table.iter().map(|&(cap, _, _, _)| cap).collect();
        for &cap in ALL_CAPS.iter() {
            assert!(
                tcp_caps.contains(&cap),
                "ALL_CAPS entry '{cap}' missing from tcp_fallback_table — \
                 Tier 5 TCP discovery will silently skip this capability"
            );
        }
    }

    #[test]
    fn tcp_fallback_no_duplicates() {
        let table = tcp_fallback_table();
        let mut seen = std::collections::HashSet::new();
        for &(cap, _, _, _) in &table {
            assert!(
                seen.insert(cap),
                "duplicate capability '{cap}' in tcp_fallback_table"
            );
        }
    }

    #[test]
    fn tcp_fallback_resolves_to_known_primals() {
        let table = tcp_fallback_table();
        for &(cap, primal, _, _) in &table {
            assert!(
                !primal.is_empty(),
                "tcp_fallback_table entry '{cap}' has empty primal"
            );
            assert!(
                super::super::routing::capability_to_primal(cap) == primal,
                "tcp_fallback_table primal for '{cap}' ({primal}) doesn't match \
                 capability_to_primal ({})",
                super::super::routing::capability_to_primal(cap)
            );
        }
    }
}
