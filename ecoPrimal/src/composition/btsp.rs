// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP escalation and TCP fallback for composition clients.

use std::collections::{BTreeMap, HashMap};

use crate::ipc::client::PrimalClient;

use super::routing::{capability_to_primal, ALL_CAPS, BTSP_EXTRA_CAPS};

/// Prefer family-scoped sockets over capability aliases for BTSP handshakes.
///
/// Capability aliases (e.g. `shader.sock`) may point to symlinks without
/// active BTSP listeners. Family-scoped sockets (`coralreef-default.sock`)
/// are the canonical endpoint where BTSP listeners are bound.
pub fn resolve_btsp_socket(
    discovered: &std::path::Path,
    primal: &str,
) -> std::path::PathBuf {
    let name = discovered
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let family = std::env::var(crate::env_keys::FAMILY_ID).unwrap_or_else(|_| "default".to_owned());
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

/// Escalate discovered clients to BTSP.
///
/// Every capability in [`ALL_CAPS`] + [`BTSP_EXTRA_CAPS`] gets a proactive
/// handshake. On success the authenticated client replaces the cleartext one.
/// On failure the original cleartext client is re-established.
///
/// Returns a `BTreeMap<capability, btsp_authenticated>` for guidestone reporting.
pub fn upgrade_btsp_clients(
    clients: &mut HashMap<String, PrimalClient>,
) -> BTreeMap<String, bool> {
    let mut state: BTreeMap<String, bool> =
        clients.keys().map(|cap| (cap.clone(), false)).collect();

    #[expect(deprecated, reason = "backward-compat bridge")]
    let Some(seed) = crate::ipc::btsp_handshake::family_seed_from_env() else {
        return state;
    };

    let proactive: Vec<&str> = ALL_CAPS
        .iter()
        .chain(BTSP_EXTRA_CAPS.iter())
        .copied()
        .collect();

    let all_caps: Vec<String> = clients.keys().cloned().collect();

    for cap in &all_caps {
        let primal = capability_to_primal(cap);
        let result = crate::ipc::discover::discover_by_capability(cap);
        if let Some(discovered_path) = result.socket {
            let path = resolve_btsp_socket(&discovered_path, primal);
            if proactive.contains(&cap.as_str()) {
                match PrimalClient::connect_btsp(&path, primal, &seed) {
                    Ok(btsp_client) => {
                        tracing::info!(cap, primal, "BTSP authenticated (proactive)");
                        clients.insert(cap.clone(), btsp_client);
                        state.insert(cap.clone(), true);
                    }
                    Err(e) => {
                        tracing::debug!(cap, primal, ?e, "BTSP upgrade failed, re-establishing cleartext");
                        if let Ok(fresh) = PrimalClient::connect(&discovered_path, primal) {
                            clients.insert(cap.clone(), fresh);
                        }
                    }
                }
            } else {
                let rejected = clients.get_mut(cap.as_str()).is_some_and(|c| {
                    matches!(
                        c.call("health.liveness", serde_json::json!({})),
                        Err(e) if e.is_connection_error() || e.is_protocol_error()
                    )
                });

                if rejected {
                    match PrimalClient::connect_btsp(&path, primal, &seed) {
                        Ok(btsp_client) => {
                            tracing::info!(cap, primal, "BTSP authenticated (reactive)");
                            clients.insert(cap.clone(), btsp_client);
                            state.insert(cap.clone(), true);
                        }
                        Err(e) => {
                            tracing::debug!(
                                cap,
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

    for &cap in ALL_CAPS {
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
                    clients.insert(cap.to_owned(), btsp_client);
                    state.insert(cap.to_owned(), true);
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
/// Part of the discovery escalation hierarchy (tier 5). Centralized here
/// so the mapping is consistent across `discover`, `from_live_discovery_with_fallback`,
/// experiments, and docs. Capabilities that alias the same primal socket
/// (e.g. `dag` and `provenance` both → rhizoCrypt) appear separately so
/// the probe covers both capability names.
///
/// Port assignments confirmed against ironGate live deployment (2026-05-04).
#[must_use]
pub fn tcp_fallback_table() -> Vec<(&'static str, &'static str, &'static str, u16)> {
    use crate::env_keys as ek;
    use crate::primal_names as pn;
    use crate::tolerances as tol;

    vec![
        ("security", pn::BEARDOG, ek::BEARDOG_PORT, tol::TCP_FALLBACK_BEARDOG_PORT),
        ("discovery", pn::SONGBIRD, ek::SONGBIRD_PORT, tol::TCP_FALLBACK_SONGBIRD_PORT),
        ("storage", pn::NESTGATE, ek::NESTGATE_PORT, tol::TCP_FALLBACK_NESTGATE_PORT),
        ("compute", pn::TOADSTOOL, ek::TOADSTOOL_PORT, tol::TCP_FALLBACK_TOADSTOOL_PORT),
        ("tensor", pn::BARRACUDA, ek::BARRACUDA_PORT, tol::TCP_FALLBACK_BARRACUDA_PORT),
        ("shader", pn::CORALREEF, ek::CORALREEF_PORT, tol::TCP_FALLBACK_CORALREEF_PORT),
        ("ai", pn::SQUIRREL, ek::SQUIRREL_PORT, tol::TCP_FALLBACK_SQUIRREL_PORT),
        ("dag", pn::RHIZOCRYPT, ek::RHIZOCRYPT_PORT, tol::TCP_FALLBACK_RHIZOCRYPT_PORT),
        ("provenance", pn::RHIZOCRYPT, ek::RHIZOCRYPT_PORT, tol::TCP_FALLBACK_RHIZOCRYPT_PORT),
        ("ledger", pn::LOAMSPINE, ek::LOAMSPINE_PORT, tol::TCP_FALLBACK_LOAMSPINE_PORT),
        ("commit", pn::SWEETGRASS, ek::SWEETGRASS_PORT, tol::TCP_FALLBACK_SWEETGRASS_PORT),
        ("attribution", pn::SWEETGRASS, ek::SWEETGRASS_PORT, tol::TCP_FALLBACK_SWEETGRASS_PORT),
        ("visualization", pn::PETALTONGUE, ek::PETALTONGUE_PORT, tol::TCP_FALLBACK_PETALTONGUE_PORT),
        ("defense", pn::SKUNKBAT, ek::SKUNKBAT_PORT, tol::TCP_FALLBACK_SKUNKBAT_PORT),
    ]
}
