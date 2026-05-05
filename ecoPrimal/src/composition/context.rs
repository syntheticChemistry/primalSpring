// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`CompositionContext`] — capability-keyed client management for NUCLEUS compositions.
//!
//! # Discovery Escalation Hierarchy
//!
//! Primals are organisms. Compositions are ecosystems. Some have a full Tower
//! Atomic with Songbird routing everything. Some have three primals on a
//! Raspberry Pi. Some run in containers with only TCP. The system doesn't ask
//! why — it watches with curiosity and uses whatever's available.
//!
//! The [`CompositionContext::discover`] constructor implements the full
//! escalation in tier order:
//!
//! | Tier | Mechanism | Scope |
//! |------|-----------|-------|
//! | 1 | Songbird routing (`ipc.resolve`) | Full NUCLEUS, cross-gate, transport-agnostic |
//! | 2 | biomeOS Neural API (`capability.discover`) | Local orchestration |
//! | 3 | UDS filesystem convention (`primal-family.sock`) | Local machine |
//! | 4 | Socket registry / primal manifests | Self-registered primals |
//! | 5 | TCP probing on well-known ports | Containers, scripts, no UDS |
//!
//! Every tier is a valid deployment model. No tier is deprecated. Partial
//! constructors ([`CompositionContext::from_live_discovery`] = tiers 2-4,
//! [`CompositionContext::from_live_discovery_with_fallback`] = tiers 2-5)
//! remain valid for callers that know their deployment context.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use crate::ipc::IpcError;
use crate::ipc::client::PrimalClient;

use super::btsp::{tcp_fallback_table, upgrade_btsp_clients};
use super::routing::{capability_to_primal, ALL_CAPS};

/// A capability-keyed set of IPC clients for a running primal composition.
///
/// Abstracts socket discovery and client lifecycle so springs interact with
/// capabilities ("tensor", "shader", "security") rather than primal names
/// or socket paths. Tracks per-capability BTSP authentication state so
/// guidestone can report security posture per atomic tier.
#[derive(Debug)]
pub struct CompositionContext {
    clients: HashMap<String, PrimalClient>,
    btsp_state: BTreeMap<String, bool>,
}

impl CompositionContext {
    /// Build a context from a running harness composition.
    ///
    /// Connects to each capability provider in the [`crate::harness::RunningAtomic`]
    /// and stores the clients keyed by capability name.
    #[must_use]
    pub fn from_running(running: &crate::harness::RunningAtomic) -> Self {
        let mut clients = HashMap::new();
        for cap in running.all_capabilities() {
            if let Some(client) = running.client_for(&cap) {
                clients.insert(cap, client);
            }
        }
        let btsp_state = clients
            .iter()
            .map(|(cap, c)| (cap.clone(), c.is_btsp_authenticated()))
            .collect();
        Self {
            clients,
            btsp_state,
        }
    }

    /// Discover a composition using the full escalation hierarchy.
    ///
    /// Tries every discovery tier in order:
    ///
    /// 1. **Songbird routing** — if the `discovery` capability is reachable,
    ///    asks Songbird to resolve every other capability via `ipc.resolve`.
    ///    This is the highest-fidelity path: transport-agnostic, cross-gate,
    ///    and Songbird-managed.
    /// 2. **Tiers 2-4** (Neural API, UDS, registry) — fills any gaps that
    ///    Songbird didn't cover, or covers everything if Tower isn't running.
    /// 3. **Tier 5** (TCP probing) — for capabilities still undiscovered,
    ///    probes well-known ports from [`crate::tolerances`].
    ///
    /// Finally, attempts BTSP escalation on all discovered clients.
    ///
    /// This is the recommended entry point. If you know your deployment
    /// context, use [`from_live_discovery`](Self::from_live_discovery)
    /// (tiers 2-4) or [`from_live_discovery_with_fallback`](Self::from_live_discovery_with_fallback)
    /// (tiers 2-5) directly.
    #[must_use]
    pub fn discover() -> Self {
        let mut clients = HashMap::new();

        // ── Tier 1: Songbird routing ──
        //
        // If we can reach the "discovery" capability (Songbird), ask it to
        // resolve every other capability. Songbird decides the transport.
        if let Ok(songbird) = crate::ipc::client::connect_by_capability("discovery") {
            clients.insert("discovery".to_owned(), songbird);

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
                        sb.call(
                            "ipc.resolve",
                            serde_json::json!({"primal_id": primal}),
                        )
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
                            clients.insert(cap.to_owned(), client);
                        }
                    }
                }
            }
        }

        // ── Tiers 2-4: Neural API, UDS convention, registry ──
        //
        // Fill any gaps that Songbird didn't cover (or everything if Tower
        // isn't running). This is the same path as from_live_discovery().
        for &cap in ALL_CAPS {
            if clients.contains_key(cap) {
                continue;
            }
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                tracing::debug!(cap, tier = "2-4", "discovered via UDS/Neural API");
                clients.insert(cap.to_owned(), client);
            }
        }

        // ── Tier 5: TCP probing ──
        //
        // For capabilities still undiscovered, probe well-known TCP ports.
        // Valid for containers, architectures without UDS, and standalone
        // compositions that choose not to run Tower.
        let host = std::env::var(crate::env_keys::PRIMALSPRING_HOST)
            .unwrap_or_else(|_| "127.0.0.1".to_owned());
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
                clients.insert(cap.to_owned(), client);
            }
        }

        // ── BTSP escalation ──
        let btsp_state = upgrade_btsp_clients(&mut clients);
        Self {
            clients,
            btsp_state,
        }
    }

    /// Build a context by live-discovering all primals on the local system
    /// (tiers 2-4 only: Neural API, UDS convention, socket registry).
    ///
    /// Uses the filesystem/socket discovery layer to find whatever primals
    /// are currently running. This is the entry point for springs that launch
    /// compositions externally (e.g. via `plasmidBin`) rather than through
    /// the test harness.
    #[must_use]
    pub fn from_live_discovery() -> Self {
        let mut clients = HashMap::new();
        for &cap in ALL_CAPS {
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                clients.insert(cap.to_owned(), client);
            }
        }
        let btsp_state = clients
            .iter()
            .map(|(cap, c)| (cap.clone(), c.is_btsp_authenticated()))
            .collect();
        Self {
            clients,
            btsp_state,
        }
    }

    /// Build a context using tiers 2-5 (Neural API, UDS, registry, TCP).
    ///
    /// Skips tier 1 (Songbird routing) — use [`discover`](Self::discover)
    /// for the full escalation. This constructor is appropriate when you
    /// know Tower is not available or want to avoid the Songbird probe.
    ///
    /// TCP port resolution uses `{PRIMAL}_PORT` env vars (e.g. `BEARDOG_PORT=9100`)
    /// with sensible defaults from [`crate::tolerances`]. This makes composition
    /// experiments work both in UDS (local development) and TCP (containers,
    /// cross-arch, benchScale) deployments.
    #[must_use]
    pub fn from_live_discovery_with_fallback() -> Self {
        let cap_to_primal = tcp_fallback_table();

        let host = std::env::var(crate::env_keys::PRIMALSPRING_HOST).unwrap_or_else(|_| "127.0.0.1".to_owned());

        let mut clients = HashMap::new();
        for &(cap, primal, port_env, default_port) in &cap_to_primal {
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                clients.insert(cap.to_owned(), client);
                continue;
            }
            let port: u16 = std::env::var(port_env)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(default_port);
            let addr = format!("{host}:{port}");
            if let Ok(client) = PrimalClient::connect_tcp(&addr, primal) {
                clients.insert(cap.to_owned(), client);
            }
        }

        let btsp_state = upgrade_btsp_clients(&mut clients);
        Self {
            clients,
            btsp_state,
        }
    }

    /// Build from an explicit set of capability-to-client mappings.
    #[must_use]
    pub fn from_clients(clients: HashMap<String, PrimalClient>) -> Self {
        let btsp_state = clients
            .iter()
            .map(|(cap, c)| (cap.clone(), c.is_btsp_authenticated()))
            .collect();
        Self {
            clients,
            btsp_state,
        }
    }

    /// Get a mutable reference to the client for a given capability.
    pub fn client_for(&mut self, capability: &str) -> Option<&mut PrimalClient> {
        self.clients.get_mut(capability)
    }

    /// Per-capability BTSP authentication state from the last escalation pass.
    ///
    /// Returns `Some(true)` if the capability was upgraded to BTSP,
    /// `Some(false)` if it remained cleartext, `None` if not discovered.
    #[must_use]
    pub fn btsp_authenticated(&self, capability: &str) -> Option<bool> {
        self.btsp_state.get(capability).copied()
    }

    /// Full BTSP state map (capability -> authenticated).
    #[must_use]
    pub const fn btsp_state(&self) -> &BTreeMap<String, bool> {
        &self.btsp_state
    }

    /// Call a method on the provider of `capability`.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client or the call fails.
    pub fn call(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        let client = self
            .clients
            .get_mut(capability)
            .ok_or_else(|| IpcError::SocketNotFound {
                primal: format!("capability:{capability}"),
            })?;
        let response = client.call(method, params)?;
        response.result.ok_or_else(|| IpcError::ProtocolError {
            detail: response
                .error
                .as_ref()
                .map_or_else(|| "no result".to_owned(), |e| e.message.clone()),
        })
    }

    /// Call a method and extract a single `f64` from the result by key.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client, the call fails,
    /// or the key is missing/not numeric.
    pub fn call_f64(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
        key: &str,
    ) -> Result<f64, IpcError> {
        self.call_f64_flex(capability, method, params, &[key])
    }

    /// Like [`call_f64`](Self::call_f64) but tries multiple candidate keys
    /// in order, returning the first that resolves to a number.
    ///
    /// Handles cross-primal response schema divergence (e.g. barraCuda
    /// returns `{"mean": N}` while the canonical key is `"result"`).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails or no key resolves to a number.
    pub fn call_f64_flex(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
        keys: &[&str],
    ) -> Result<f64, IpcError> {
        let result = self.call(capability, method, params)?;
        for key in keys {
            if let Some(v) = result.get(*key).and_then(serde_json::Value::as_f64) {
                return Ok(v);
            }
        }
        Err(IpcError::SerializationError {
            detail: format!(
                "none of keys {keys:?} found as number in {result}"
            ),
        })
    }

    /// Like [`call`](Self::call) but tries multiple candidate keys for an
    /// array result, returning the first that resolves.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails or no key resolves to an array.
    pub fn call_array_flex(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
        keys: &[&str],
    ) -> Result<(String, Vec<serde_json::Value>), IpcError> {
        let result = self.call(capability, method, params)?;
        for key in keys {
            if let Some(arr) = result.get(*key).and_then(serde_json::Value::as_array) {
                return Ok(((*key).to_owned(), arr.clone()));
            }
        }
        Err(IpcError::SerializationError {
            detail: format!(
                "none of keys {keys:?} found as array in {result}"
            ),
        })
    }

    /// All capability names that have live clients in this context.
    #[must_use]
    pub fn available_capabilities(&self) -> Vec<&str> {
        self.clients.keys().map(String::as_str).collect()
    }

    /// Whether this context has a live client for the given capability.
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.clients.contains_key(capability)
    }

    /// Normalized health check that handles all primal response formats.
    ///
    /// Primals return different health shapes: `{"alive":true}`,
    /// `{"status":"alive"}`, `{"status":"ok"}`. This method normalizes
    /// all of them into a single boolean.
    ///
    /// loamSpine requires `{"include_details": true}` — this method sends
    /// that param for the `ledger`/`spine`/`merkle` capabilities automatically.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client or the call fails.
    pub fn health_check(&mut self, capability: &str) -> Result<bool, IpcError> {
        use crate::primal_names;
        let params = match capability_to_primal(capability) {
            primal_names::LOAMSPINE => serde_json::json!({"include_details": true}),
            _ => serde_json::json!({}),
        };
        let result = self.call(capability, "health.liveness", params)?;
        Ok(result
            .get("alive")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
            || result
                .get("status")
                .and_then(|s| s.as_str())
                .is_some_and(|s| s == "ok" || s == "alive"))
    }

    /// Hash arbitrary bytes via the security primal (`crypto.hash`).
    ///
    /// `BearDog` expects the `data` param as base64-encoded bytes and returns
    /// the hash as base64. This method handles the encoding round-trip.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if security is unavailable or the call fails.
    pub fn hash_bytes(&mut self, data: &[u8], algorithm: &str) -> Result<String, IpcError> {
        let encoded = BASE64.encode(data);
        let result = self.call(
            "security",
            "crypto.hash",
            serde_json::json!({"data": encoded, "algorithm": algorithm}),
        )?;
        result
            .get("hash")
            .and_then(|h| h.as_str())
            .map(String::from)
            .ok_or_else(|| IpcError::SerializationError {
                detail: format!("'hash' key not found in {result}"),
            })
    }

    /// Resolve a capability to its provider primal name using Songbird.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if discovery is unavailable or resolution fails.
    pub fn resolve_capability(&mut self, capability: &str) -> Result<serde_json::Value, IpcError> {
        let primal_id = capability_to_primal(capability);
        self.call(
            "discovery",
            "ipc.resolve",
            serde_json::json!({"primal_id": primal_id}),
        )
    }

    // ── Visualization convenience (petalTongue live pipeline) ──────────

    /// Push a scene to petalTongue for live rendering.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `visualization` capability is absent or
    /// the render call fails.
    pub fn push_scene(
        &mut self,
        session_id: &str,
        title: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        self.call(
            "visualization",
            crate::ipc::methods::visualization::RENDER_SCENE,
            serde_json::json!({
                "session_id": session_id,
                "title": title,
                "domain": "game",
                "scene": data,
            }),
        )
    }

    /// Push a streaming metric update to petalTongue.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `visualization` capability is absent or
    /// the render call fails.
    pub fn push_stream(
        &mut self,
        session_id: &str,
        action: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        self.call(
            "visualization",
            crate::ipc::methods::visualization::RENDER_STREAM,
            serde_json::json!({
                "session_id": session_id,
                "action": action,
                "data": payload,
            }),
        )
    }

    /// Poll petalTongue for pending user interaction events.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `visualization` capability is absent or
    /// the poll call fails.
    pub fn poll_interactions(&mut self, session_id: &str) -> Result<serde_json::Value, IpcError> {
        self.call(
            "visualization",
            crate::ipc::methods::interaction::POLL,
            serde_json::json!({ "session_id": session_id }),
        )
    }
}
