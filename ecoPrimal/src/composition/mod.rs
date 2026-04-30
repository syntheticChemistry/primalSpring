// SPDX-License-Identifier: AGPL-3.0-or-later

//! Composition parity validation — the bridge between coordination and math.
//!
//! Springs validate domain science through a pipeline:
//!
//! 1. **Python baseline** — peer-reviewed, reproducible
//! 2. **Rust port** — matches Python within documented tolerance
//! 3. **Primal composition** — matches Rust via IPC
//!
//! This module provides [`CompositionContext`] as a single entry point for
//! step 3. Springs call math through the composition layer and compare
//! results against their local Rust baselines without understanding primal
//! internals, socket paths, or JSON-RPC response schemas.
//!
//! # Example
//!
//! ```rust,no_run
//! use primalspring::composition::CompositionContext;
//! use primalspring::validation::ValidationResult;
//! use primalspring::tolerances;
//!
//! let mut ctx = CompositionContext::from_live_discovery();
//! let mut v = ValidationResult::new("hotSpring Composition Parity");
//!
//! // stats.mean: param key is "data", result key is "result"
//! primalspring::composition::validate_parity(
//!     &mut ctx, &mut v,
//!     "sample_mean",
//!     "tensor",           // capability — resolves to barraCuda
//!     "stats.mean",
//!     serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
//!     "result",
//!     3.0,
//!     tolerances::CPU_GPU_PARITY_TOL,
//! );
//! ```

use std::collections::{BTreeMap, HashMap};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use crate::ipc::IpcError;
use crate::ipc::client::PrimalClient;
use crate::validation::ValidationResult;

/// All NUCLEUS capabilities that primalSpring discovers and authenticates.
///
/// Single source of truth for `from_live_discovery`, `PROACTIVE_CAPS` in
/// `upgrade_btsp_clients`, and the TCP fallback table. Capabilities that
/// are aliases for the same primal socket (e.g. `dag` and `provenance`
/// both → rhizoCrypt) appear once each so guidestone reports per-capability.
const ALL_CAPS: &[&str] = &[
    "security",
    "discovery",
    "compute",
    "tensor",
    "shader",
    "storage",
    "ai",
    "dag",
    "commit",
    "provenance",
    "visualization",
    "ledger",
    "attribution",
];

/// Extended capability aliases for BTSP proactive escalation.
///
/// Includes names that map to the same primal sockets as [`ALL_CAPS`]
/// (e.g. `inference` → Squirrel, `spine`/`merkle` → loamSpine) to ensure
/// BTSP coverage even when a client was connected under an alias name.
const BTSP_EXTRA_CAPS: &[&str] = &["inference", "spine", "merkle", "braid"];

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

    /// Build a context by live-discovering all primals on the local system.
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

    /// Build a context by trying UDS discovery first, then falling back to
    /// TCP probing on well-known ports.
    ///
    /// TCP port resolution uses `{PRIMAL}_PORT` env vars (e.g. `BEARDOG_PORT=9100`)
    /// with sensible defaults from [`crate::tolerances`]. This makes composition
    /// experiments work both in UDS (local development) and TCP (containers,
    /// cross-arch, benchScale) deployments.
    #[must_use]
    pub fn from_live_discovery_with_fallback() -> Self {
        let cap_to_primal = tcp_fallback_table();

        let host = std::env::var("PRIMALSPRING_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

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
        let result = self.call(capability, method, params)?;
        result
            .get(key)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| IpcError::SerializationError {
                detail: format!("key '{key}' not found or not a number in {result}"),
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
        let params = match capability_to_primal(capability) {
            "loamspine" => serde_json::json!({"include_details": true}),
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
    /// Songbird's `ipc.resolve` expects `primal_id`, so this maps from
    /// capability → known primal name using the local atomic registry,
    /// then calls `ipc.resolve`.
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
    /// This is the typed equivalent of ludoSpring's
    /// `PetalTonguePushClient::push_scene` — routed through the
    /// composition layer with `IpcError` semantics and BTSP enforcement.
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
    /// Returns the raw interaction payload from petalTongue. Empty
    /// `events` array means no pending input.
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

/// Map a capability domain to its canonical primal provider.
///
/// This is the ecosystem's single source of truth for "which primal owns
/// which capability domain." Springs use this to route IPC calls without
/// hardcoding primal names.
///
/// ```
/// assert_eq!(primalspring::composition::capability_to_primal("tensor"), "barracuda");
/// assert_eq!(primalspring::composition::capability_to_primal("crypto"), "beardog");
/// assert_eq!(primalspring::composition::capability_to_primal("storage"), "nestgate");
/// ```
#[must_use]
pub fn capability_to_primal(capability: &str) -> &str {
    use crate::primal_names as pn;
    match capability {
        "security" | "crypto" => pn::BEARDOG,
        "discovery" | "network" => pn::SONGBIRD,
        "compute" => pn::TOADSTOOL,
        "tensor" | "math" => pn::BARRACUDA,
        "shader" => pn::CORALREEF,
        "storage" => pn::NESTGATE,
        "ai" | "inference" => pn::SQUIRREL,
        "dag" | "provenance" => pn::RHIZOCRYPT,
        "ledger" | "spine" | "merkle" => pn::LOAMSPINE,
        "commit" | "attribution" | "braid" => pn::SWEETGRASS,
        "visualization" => pn::PETALTONGUE,
        "orchestration" => pn::BIOMEOS,
        other => other,
    }
}

/// Map a JSON-RPC method name to the capability domain that owns it.
///
/// Given a method like `"tensor.matmul"` or `"stats.mean"`, returns the
/// capability domain string that [`CompositionContext`] uses for routing.
/// Springs use this to determine which `call()` domain to use for a given
/// method from their `validation_capabilities` manifest entry.
///
/// ```
/// assert_eq!(primalspring::composition::method_to_capability_domain("tensor.matmul"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("stats.mean"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("crypto.hash"), "security");
/// assert_eq!(primalspring::composition::method_to_capability_domain("storage.store"), "storage");
/// assert_eq!(primalspring::composition::method_to_capability_domain("compute.dispatch"), "compute");
/// assert_eq!(primalspring::composition::method_to_capability_domain("linalg.solve"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("spectral.fft"), "tensor");
/// ```
#[must_use]
pub fn method_to_capability_domain(method: &str) -> &str {
    let prefix = method.split('.').next().unwrap_or(method);
    match prefix {
        "crypto" | "health" | "identity" | "primal" => "security",
        "ipc" | "discovery" => "discovery",
        "compute" => "compute",
        "tensor" | "stats" | "math" | "noise" | "activation" | "rng" | "fhe" | "tolerances"
        | "validate" | "device" | "linalg" | "spectral" => "tensor",
        "shader" => "shader",
        "storage" => "storage",
        "inference" | "ai" | "squirrel" | "mcp" => "ai",
        "dag" => "dag",
        "spine" | "entry" | "certificate" => "ledger",
        "braid" | "anchoring" => "commit",
        "visualization" | "viz" | "proprioception" => "visualization",
        "graph" | "capability" | "lifecycle" | "coordination" => "orchestration",
        _ => prefix,
    }
}

/// Validate scalar parity between a local baseline and a primal composition.
///
/// This is the primary convenience function for springs. It:
/// 1. Calls `method` on the provider of `capability` via the composition
/// 2. Extracts a scalar `f64` from the result using `result_key`
/// 3. Compares against `expected` within `tolerance`
/// 4. Records the outcome on `v` (pass, fail, or skip if IPC unavailable)
///
/// Springs call this repeatedly for each math operation they want to validate:
///
/// ```rust,no_run
/// # use primalspring::composition::{CompositionContext, validate_parity};
/// # use primalspring::validation::ValidationResult;
/// # use primalspring::tolerances;
/// # let mut ctx = CompositionContext::from_live_discovery();
/// # let mut v = ValidationResult::new("test");
/// validate_parity(
///     &mut ctx, &mut v,
///     "sample_mean",
///     "tensor",              // capability — resolves to barraCuda
///     "stats.mean",
///     serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
///     "result",
///     3.0,
///     tolerances::CPU_GPU_PARITY_TOL,
/// );
/// ```
#[expect(
    clippy::too_many_arguments,
    reason = "domain-driven API: each parameter is semantically distinct"
)]
pub fn validate_parity(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
    result_key: &str,
    expected: f64,
    tolerance: f64,
) {
    match ctx.call_f64(capability, method, params, result_key) {
        Ok(actual) => {
            let diff = (actual - expected).abs();
            let ok = diff <= tolerance;
            let detail = format!(
                "composition={actual}, local={expected}, diff={diff:.2e}, tol={tolerance:.2e}"
            );
            v.check_bool(name, ok, &detail);
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(name, &format!("{capability} not available: {e}"));
        }
        Err(e) if e.is_transport_mismatch() => {
            v.check_skip(
                name,
                &format!("{capability} uses non-JSON-RPC transport: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(name, false, &format!("composition error: {e}"));
        }
    }
}

fn flatten_numeric_json_values(arr: &[serde_json::Value]) -> Vec<f64> {
    let mut out = Vec::new();
    for val in arr {
        if let Some(n) = val.as_f64() {
            out.push(n);
        } else if let Some(inner) = val.as_array() {
            out.extend(flatten_numeric_json_values(inner));
        }
    }
    out
}

/// Validate vector parity between a local baseline and a primal composition.
///
/// Like [`validate_parity`] but for multi-element results (tensors, arrays).
/// All elements must match within tolerance for the check to pass.
#[expect(
    clippy::too_many_arguments,
    reason = "domain-driven API: each parameter is semantically distinct"
)]
pub fn validate_parity_vec(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
    result_key: &str,
    expected: &[f64],
    tolerance: f64,
) {
    let result = match ctx.call(capability, method, params) {
        Ok(r) => r,
        Err(e) if e.is_connection_error() => {
            v.check_skip(name, &format!("{capability} not available: {e}"));
            return;
        }
        Err(e) if e.is_transport_mismatch() => {
            v.check_skip(
                name,
                &format!("{capability} uses non-JSON-RPC transport: {e}"),
            );
            return;
        }
        Err(e) => {
            v.check_bool(name, false, &format!("composition error: {e}"));
            return;
        }
    };

    let arr = result.get(result_key).and_then(serde_json::Value::as_array);

    let Some(arr) = arr else {
        v.check_bool(
            name,
            false,
            &format!("key '{result_key}' not found or not an array"),
        );
        return;
    };

    let actual = flatten_numeric_json_values(arr);
    if actual.len() != arr.len() && actual.is_empty() {
        v.check_bool(
            name,
            false,
            &format!(
                "{}/{} array elements are not numeric (null, string, or object) — \
                 check primal response schema",
                arr.len() - actual.len(),
                arr.len()
            ),
        );
        return;
    }
    if actual.len() != expected.len() {
        v.check_bool(
            name,
            false,
            &format!(
                "length mismatch: composition={}, local={}",
                actual.len(),
                expected.len()
            ),
        );
        return;
    }

    let max_diff = actual
        .iter()
        .zip(expected.iter())
        .map(|(a, e)| (a - e).abs())
        .fold(0.0_f64, f64::max);
    let ok = max_diff <= tolerance;
    let detail = format!(
        "len={}, max_diff={max_diff:.2e}, tol={tolerance:.2e}",
        actual.len()
    );
    v.check_bool(name, ok, &detail);
}

/// Call a primal method, recording PASS on success, SKIP on absent/protocol
/// errors, and FAIL on unexpected errors.
///
/// Returns `Some(result)` on success so callers can chain pipeline steps.
/// When a step returns `None`, downstream steps should also skip. This is
/// the standard pattern for multi-primal pipeline validation (e.g.,
/// hash → store → retrieve → verify).
///
/// Absorbed from ludoSpring V46 and healthSpring V56 — both independently
/// invented this pattern for cross-atomic pipeline validation.
pub fn call_or_skip(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    check_name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
) -> Option<serde_json::Value> {
    match ctx.call(capability, method, params) {
        Ok(result) => {
            v.check_bool(check_name, true, "ok");
            Some(result)
        }
        Err(e) if is_skip_error(&e) => {
            v.check_skip(check_name, &format!("{e}"));
            None
        }
        Err(e) => {
            v.check_bool(check_name, false, &format!("{e}"));
            None
        }
    }
}

/// Whether an IPC error should be treated as a graceful skip.
///
/// Covers absent primals (connection refused), protocol mismatches
/// (HTTP-on-UDS), and transport dialect differences (BTSP). Springs use
/// this to degrade gracefully — a skip means "reachable or expected
/// absent" and does not count as a failure.
#[must_use]
pub fn is_skip_error(e: &IpcError) -> bool {
    e.is_connection_error() || e.is_protocol_error() || e.is_transport_mismatch()
}

/// Escalate discovered clients to BTSP.
///
/// BTSP is the default for the entire NUCLEUS. Every capability in
/// [`ALL_CAPS`] + [`BTSP_EXTRA_CAPS`] gets a proactive handshake. On success the
/// authenticated client replaces the cleartext one. On failure the original
/// cleartext client is kept and the capability is marked as non-BTSP —
/// guidestone reports this as FAIL.
///
/// Capabilities not in the proactive set use a reactive fallback: probe
/// cleartext first, only escalate if the server rejects cleartext.
///
/// Primals that enforce BTSP may reject the initial cleartext connection
/// in `from_live_discovery_with_fallback`, leaving no client for that
/// capability. After upgrading existing clients, this function performs a
/// second pass over `ALL_CAPS` to BTSP-connect any capabilities that have
/// discoverable sockets but no client yet.
///
/// Returns a `BTreeMap<capability, btsp_authenticated>` so guidestone can
/// report per-atomic security posture without re-probing.
fn upgrade_btsp_clients(clients: &mut HashMap<String, PrimalClient>) -> BTreeMap<String, bool> {
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
        if let Some(path) = result.socket {
            if proactive.contains(&cap.as_str()) {
                match PrimalClient::connect_btsp(&path, primal, &seed) {
                    Ok(btsp_client) => {
                        tracing::info!(cap, primal, "BTSP authenticated (proactive)");
                        clients.insert(cap.clone(), btsp_client);
                        state.insert(cap.clone(), true);
                    }
                    Err(e) => {
                        tracing::debug!(cap, primal, ?e, "BTSP upgrade failed, keeping cleartext");
                    }
                }
            } else {
                // Reactive: probe cleartext first, only escalate if rejected.
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

    // Second pass: BTSP-first for capabilities with discoverable sockets but
    // no client (e.g. BTSP-enforcing primals that rejected cleartext connect).
    for &cap in ALL_CAPS {
        if clients.contains_key(cap) {
            continue;
        }
        let result = crate::ipc::discover::discover_by_capability(cap);
        if let Some(path) = result.socket {
            let primal = capability_to_primal(cap);
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

/// Capability → (primal slug, env var, default port) for TCP fallback.
///
/// Centralized in one place so the mapping is consistent across
/// `from_live_discovery_with_fallback`, experiments, and docs.
/// Default ports reference [`crate::tolerances`] constants where defined;
/// Node-tier additions (barraCuda, coralReef) and Nest-tier (rhizoCrypt,
/// loamSpine, sweetGrass) use sequential offsets from well-known bases.
#[must_use]
fn tcp_fallback_table() -> Vec<(&'static str, &'static str, &'static str, u16)> {
    use crate::primal_names as pn;
    use crate::tolerances as tol;

    vec![
        (
            "security",
            pn::BEARDOG,
            "BEARDOG_PORT",
            tol::TCP_FALLBACK_BEARDOG_PORT,
        ),
        (
            "discovery",
            pn::SONGBIRD,
            "SONGBIRD_PORT",
            tol::TCP_FALLBACK_SONGBIRD_PORT,
        ),
        (
            "storage",
            pn::NESTGATE,
            "NESTGATE_PORT",
            tol::TCP_FALLBACK_NESTGATE_PORT,
        ),
        (
            "compute",
            pn::TOADSTOOL,
            "TOADSTOOL_PORT",
            tol::TCP_FALLBACK_TOADSTOOL_PORT,
        ),
        (
            "tensor",
            pn::BARRACUDA,
            "BARRACUDA_PORT",
            tol::TCP_FALLBACK_TOADSTOOL_PORT + 1,
        ),
        (
            "shader",
            pn::CORALREEF,
            "CORALREEF_PORT",
            tol::TCP_FALLBACK_TOADSTOOL_PORT + 2,
        ),
        (
            "ai",
            pn::SQUIRREL,
            "SQUIRREL_PORT",
            tol::TCP_FALLBACK_SQUIRREL_PORT,
        ),
        (
            "dag",
            pn::RHIZOCRYPT,
            "RHIZOCRYPT_PORT",
            tol::TCP_FALLBACK_NESTGATE_PORT + 1,
        ),
        (
            "commit",
            pn::SWEETGRASS,
            "SWEETGRASS_PORT",
            tol::TCP_FALLBACK_NESTGATE_PORT + 3,
        ),
        (
            "provenance",
            pn::RHIZOCRYPT,
            "RHIZOCRYPT_PORT",
            tol::TCP_FALLBACK_NESTGATE_PORT + 1,
        ),
        (
            "visualization",
            pn::PETALTONGUE,
            "PETALTONGUE_PORT",
            tol::TCP_FALLBACK_PETALTONGUE_PORT,
        ),
        (
            "ledger",
            pn::LOAMSPINE,
            "LOAMSPINE_PORT",
            tol::TCP_FALLBACK_NESTGATE_PORT + 2,
        ),
        (
            "attribution",
            pn::SWEETGRASS,
            "SWEETGRASS_PORT",
            tol::TCP_FALLBACK_NESTGATE_PORT + 3,
        ),
    ]
}

/// Validate that a set of required capabilities are live in the composition.
///
/// This is the standard "preamble" for a primal proof binary: check that
/// the NUCLEUS primals your science needs are actually running before
/// attempting math parity checks. Each capability gets a health check;
/// missing capabilities are recorded as skipped.
///
/// Returns the count of capabilities that responded alive. Springs should
/// `exit(2)` when zero (no NUCLEUS deployed).
///
/// ```rust,no_run
/// # use primalspring::composition::{CompositionContext, validate_liveness};
/// # use primalspring::validation::ValidationResult;
/// # let mut ctx = CompositionContext::from_live_discovery();
/// # let mut v = ValidationResult::new("test");
/// let alive = validate_liveness(
///     &mut ctx, &mut v,
///     &["tensor", "security", "compute"],
/// );
/// if alive == 0 {
///     eprintln!("No NUCLEUS primals discovered.");
///     std::process::exit(2);
/// }
/// ```
pub fn validate_liveness(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    required_capabilities: &[&str],
) -> usize {
    let mut alive = 0;
    for &cap in required_capabilities {
        let primal = capability_to_primal(cap);
        let name = format!("{primal}.liveness");
        match ctx.health_check(cap) {
            Ok(true) => {
                v.check_bool(&name, true, &format!("{primal} alive via {cap}"));
                alive += 1;
            }
            Ok(false) => {
                v.check_bool(&name, false, &format!("{primal} responded but not alive"));
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(&name, &format!("{primal} not reachable: {e}"));
            }
            Err(e) if e.is_protocol_error() => {
                v.check_skip(
                    &name,
                    &format!("{primal} reachable but protocol mismatch (likely HTTP): {e}"),
                );
                alive += 1;
            }
            Err(e) => {
                v.check_bool(&name, false, &format!("{primal} health error: {e}"));
            }
        }
    }
    alive
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
