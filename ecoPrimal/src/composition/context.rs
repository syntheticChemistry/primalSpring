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
//!
//! # Degradation Behavior (per lithoSpore R1, May 17 2026)
//!
//! When a primal is unreachable, `CompositionContext` degrades gracefully:
//!
//! | Capability | Unreachable Behavior | Consumer Impact |
//! |------------|----------------------|-----------------|
//! | `dag` (rhizoCrypt) | `call` returns `Err` | Tier 3 provenance unavailable; Tier 2 science still runs |
//! | `spine` (loamSpine) | `call` returns `Err` | No ledger entry; DAG session valid but unbacked |
//! | `braid` (sweetGrass) | `call` returns `Err` | No attribution braid; DAG + spine are partial provenance |
//! | `visualization` (petalTongue) | `call` returns `Err` | No rendered figures; data still valid |
//! | `discovery` (songBird) | `discover()` returns `None` | Falls to lower discovery tier or standalone mode |
//! | `orchestration` (biomeOS) | `announce()` returns `Err` | Self-registration skipped; CLI still functional |
//! | `crypto` (bearDog) | `call` returns `Err` | Optional signing skipped |
//! | `compute` (toadStool) | `call` returns `Err` | Local compute only; no accelerated dispatch |
//!
//! **Invariant**: No `CompositionContext` method panics on unreachable primals.
//! All RPC calls return `Result` — callers decide whether to skip, retry, or
//! abort. The `has_capability` method provides pre-call reachability checks.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use crate::ipc::IpcError;
use crate::ipc::client::PrimalClient;

use super::btsp::{tcp_fallback_table, upgrade_btsp_clients};
use super::routing::{ALL_CAPS, capability_to_primal};

/// Returns `true` when Tier 5 TCP port probing is explicitly enabled.
///
/// The zero-port Tower Atomic standard treats TCP port exposure as metadata
/// leakage. Tier 5 is off by default; set `PRIMALSPRING_TCP_TIER5=1` for
/// containers, Android, or deployments without Unix domain sockets.
///
/// In release builds, TCP Tier 5 is unconditionally disabled — the env var
/// is ignored. This enforces the glacial zero-port standard at compile time.
fn tcp_tier5_enabled() -> bool {
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

/// How a capability was discovered — mirrors lithoSpore's `DiscoveryPath`.
///
/// Tracks provenance of each capability's connection so downstream can
/// report discovery health and detect deployment topology changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryPath {
    /// Resolved via Songbird routing (`ipc.resolve`)
    Songbird,
    /// Discovered via biomeOS Neural API or UDS filesystem convention
    LocalDiscovery,
    /// Connected via TCP port probing (tier 5)
    TcpFallback,
    /// Inherited from a `RunningAtomic` harness
    Harness,
    /// Directly injected via `from_clients`
    Injected,
}

/// A capability-keyed set of IPC clients for a running primal composition.
///
/// Abstracts socket discovery and client lifecycle so springs interact with
/// capabilities ("tensor", "shader", "security") rather than primal names
/// or socket paths. Tracks per-capability BTSP authentication state and
/// discovery path so guidestone can report security posture and topology.
///
/// Supports optional bearer token threading for JH-11 token federation:
/// when a bearer token is set, `call_authenticated` injects it as
/// `_bearer_token` in JSON-RPC params for scope-checked dispatch.
#[derive(Debug)]
pub struct CompositionContext {
    pub(super) clients: HashMap<String, PrimalClient>,
    btsp_state: BTreeMap<String, bool>,
    discovery_paths: BTreeMap<String, DiscoveryPath>,
    pub(super) bearer_token: Option<String>,
    /// Gate identity for multi-gate mesh awareness.
    gate_id: Option<String>,
    /// Optional mesh topology for cross-gate capability resolution.
    mesh: Option<super::mesh::MeshTopology>,
}

impl CompositionContext {
    /// Build a context from a running harness composition.
    ///
    /// Connects to each capability provider in the [`crate::harness::RunningAtomic`]
    /// and stores the clients keyed by capability name.
    #[must_use]
    #[deprecated(
        since = "0.9.25",
        note = "use CompositionContext::from_live_discovery_with_fallback() against deployed ecoBins instead"
    )]
    #[expect(
        deprecated,
        reason = "from_running bridges deprecated RunningAtomic to CompositionContext"
    )]
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
        let discovery_paths = clients
            .keys()
            .map(|cap| (cap.clone(), DiscoveryPath::Harness))
            .collect();
        Self {
            clients,
            btsp_state,
            discovery_paths,
            bearer_token: None,
            gate_id: None,
            mesh: None,
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
    /// 3. **Tier 5** (TCP probing, opt-in) — requires `PRIMALSPRING_TCP_TIER5=1`.
    ///    Well-known TCP ports are a metadata leak in the zero-port Tower
    ///    Atomic standard. Only enabled for containers and legacy deployments.
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
        let mut discovery_paths = BTreeMap::new();

        // ── Tier 1: Songbird routing ──
        if let Ok(songbird) = crate::ipc::client::connect_by_capability("discovery") {
            clients.insert("discovery".to_owned(), songbird);
            discovery_paths.insert("discovery".to_owned(), DiscoveryPath::Songbird);

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
                            clients.insert(cap.to_owned(), client);
                            discovery_paths.insert(cap.to_owned(), DiscoveryPath::Songbird);
                        }
                    }
                }
            }
        }

        // ── Tiers 2-4: Neural API, UDS convention, registry ──
        for &cap in ALL_CAPS {
            if clients.contains_key(cap) {
                continue;
            }
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                tracing::debug!(cap, tier = "2-4", "discovered via UDS/Neural API");
                clients.insert(cap.to_owned(), client);
                discovery_paths.insert(cap.to_owned(), DiscoveryPath::LocalDiscovery);
            }
        }

        // ── Tier 5: TCP probing (opt-in) ──
        if tcp_tier5_enabled() {
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
                    clients.insert(cap.to_owned(), client);
                    discovery_paths.insert(cap.to_owned(), DiscoveryPath::TcpFallback);
                }
            }
        }

        // ── BTSP escalation ──
        let btsp_state = upgrade_btsp_clients(&mut clients);
        Self {
            clients,
            btsp_state,
            discovery_paths,
            bearer_token: None,
            gate_id: None,
            mesh: None,
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
        let mut discovery_paths = BTreeMap::new();
        for &cap in ALL_CAPS {
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                clients.insert(cap.to_owned(), client);
                discovery_paths.insert(cap.to_owned(), DiscoveryPath::LocalDiscovery);
            }
        }
        let btsp_state = clients
            .iter()
            .map(|(cap, c)| (cap.clone(), c.is_btsp_authenticated()))
            .collect();
        Self {
            clients,
            btsp_state,
            discovery_paths,
            bearer_token: None,
            gate_id: None,
            mesh: None,
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

        let host = std::env::var(crate::env_keys::PRIMALSPRING_HOST)
            .unwrap_or_else(|_| crate::tolerances::DEFAULT_HOST.to_owned());

        let mut clients = HashMap::new();
        let mut discovery_paths = BTreeMap::new();
        for &(cap, primal, port_env, default_port) in &cap_to_primal {
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                clients.insert(cap.to_owned(), client);
                discovery_paths.insert(cap.to_owned(), DiscoveryPath::LocalDiscovery);
                continue;
            }
            let port: u16 = std::env::var(port_env)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(default_port);
            let addr = format!("{host}:{port}");
            if let Ok(client) = PrimalClient::connect_tcp(&addr, primal) {
                clients.insert(cap.to_owned(), client);
                discovery_paths.insert(cap.to_owned(), DiscoveryPath::TcpFallback);
            }
        }

        let btsp_state = upgrade_btsp_clients(&mut clients);
        Self {
            clients,
            btsp_state,
            discovery_paths,
            bearer_token: None,
            gate_id: None,
            mesh: None,
        }
    }

    /// Build from an explicit set of capability-to-client mappings.
    #[must_use]
    pub fn from_clients(clients: HashMap<String, PrimalClient>) -> Self {
        let btsp_state = clients
            .iter()
            .map(|(cap, c)| (cap.clone(), c.is_btsp_authenticated()))
            .collect();
        let discovery_paths = clients
            .keys()
            .map(|cap| (cap.clone(), DiscoveryPath::Injected))
            .collect();
        Self {
            clients,
            btsp_state,
            discovery_paths,
            bearer_token: None,
            gate_id: None,
            mesh: None,
        }
    }

    // ── Bearer token management (JH-11 preparation) ────────────────────

    /// Set a bearer token that will be threaded through authenticated calls.
    ///
    /// Once set, [`call_authenticated`](Self::call_authenticated) will inject
    /// `_bearer_token` into JSON-RPC params for scope-checked dispatch on
    /// the receiving primal's MethodGate.
    pub fn set_bearer_token(&mut self, token: impl Into<String>) {
        self.bearer_token = Some(token.into());
    }

    /// Clear any previously set bearer token.
    pub fn clear_bearer_token(&mut self) {
        self.bearer_token = None;
    }

    /// Whether a bearer token is currently set for authenticated calls.
    #[must_use]
    pub const fn has_bearer_token(&self) -> bool {
        self.bearer_token.is_some()
    }

    // ── Gate identity (multi-gate mesh awareness) ───────────────────────

    /// Set the gate identity for this composition context.
    ///
    /// Enables mesh-aware validation — the context knows which gate it
    /// belongs to, enabling cross-gate capability routing analysis.
    pub fn set_gate_id(&mut self, gate_id: impl Into<String>) {
        self.gate_id = Some(gate_id.into());
    }

    /// The gate identity, if set.
    #[must_use]
    pub fn gate_id(&self) -> Option<&str> {
        self.gate_id.as_deref()
    }

    // ── Mesh topology (cross-gate resolution) ───────────────────────────

    /// Attach a mesh topology for cross-gate capability resolution.
    ///
    /// When set, `resolve_cross_gate` can determine which remote gate hosts
    /// a capability not available locally, enabling transparent routing.
    pub fn set_mesh(&mut self, topology: super::mesh::MeshTopology) {
        self.mesh = Some(topology);
    }

    /// Access the mesh topology, if attached.
    #[must_use]
    pub const fn mesh(&self) -> Option<&super::mesh::MeshTopology> {
        self.mesh.as_ref()
    }

    /// Resolve a capability to its hosting gate via mesh topology.
    ///
    /// Returns `None` if no mesh is attached or the capability is not reachable.
    #[must_use]
    pub fn resolve_cross_gate(&self, capability: &str) -> Option<&str> {
        self.mesh
            .as_ref()
            .and_then(|m| m.resolve_capability(capability))
            .map(|node| node.gate_id.as_str())
    }

    // ── Atomic signals ──────────────────────────────────────────────────

    /// Atomic tier names recognized by [`signal`](Self::signal).
    const COMPOSITION_TIERS: &[&str] = &["tower", "node", "nest", "nucleus", "meta", "ecosystem", "rootpulse"];

    /// Send an atomic signal to a composition tier.
    ///
    /// Signals are compound operations addressed to atomic tiers (Tower, Node,
    /// Nest, NUCLEUS) rather than individual capability domains. When biomeOS
    /// Neural API is available, the signal is dispatched as a graph execution
    /// via `capability.call` with the tier as capability and the signal name
    /// as the operation. When the Neural API is unavailable, the signal falls
    /// back to the `orchestration` domain.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use primalspring::composition::CompositionContext;
    /// # fn example(ctx: &mut CompositionContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = serde_json::json!({"data": "hello"});
    /// let result = ctx.composition("tower", "publish", &params)?;
    /// // biomeOS decomposes: bearDog.sign → songbird.announce → skunkBat.audit
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the orchestration capability is absent or the
    /// signal dispatch fails.
    pub fn composition(
        &mut self,
        tier: &str,
        signal_name: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        if !self.clients.contains_key("orchestration") {
            return Err(IpcError::SocketNotFound {
                primal: format!("orchestration (for signal {tier}.{signal_name})"),
            });
        }

        // Prefer signal.dispatch (direct graph execution) over capability.call
        // (which requires biomeOS v3.56+ signal-tier interception).
        let dispatch_params = serde_json::json!({
            "signal": format!("{tier}.{signal_name}"),
            "params": params,
        });
        self.call("orchestration", "signal.dispatch", dispatch_params).map_or_else(|_| {
            // Fallback to capability.call for older biomeOS versions
            // where signal.dispatch is not yet available.
            let cap_params = serde_json::json!({
                "capability": tier,
                "operation": signal_name,
                "args": params,
            });
            self.call("orchestration", "capability.call", cap_params)
        }, Ok)
    }

    /// Whether the given tier name is a recognized atomic signal tier.
    #[must_use]
    pub fn is_composition_tier(tier: &str) -> bool {
        Self::COMPOSITION_TIERS.contains(&tier)
    }

    /// Dispatch an atomic signal by its unified identifier.
    ///
    /// Takes a `signal_id` in `"tier.name"` form (matching `composition_tools.toml`
    /// identifiers like `"nest.store"`, `"tower.publish"`, `"node.compute"`)
    /// and delegates to [`signal`](Self::signal).
    ///
    /// This is the **primary consumption API** for springs adopting the Neural
    /// API semantic collapse pattern. Instead of calling individual primal
    /// methods (`content.put`, `dag.event.append`, `spine.seal`, `braid.create`),
    /// springs call `dispatch("nest.store", params)` and biomeOS executes the
    /// full provenance graph.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use primalspring::composition::CompositionContext;
    /// # fn example(ctx: &mut CompositionContext) -> Result<(), Box<dyn std::error::Error>> {
    /// // Before (flat method surface — 4 calls, spring manages sequencing):
    /// // ctx.call("content", "content.put", data)?;
    /// // ctx.call("dag", "dag.event.append", event)?;
    /// // ctx.call("spine", "spine.seal", vertex)?;
    /// // ctx.call("braid", "braid.create", contributors)?;
    ///
    /// // After (semantic collapse — 1 call, biomeOS manages the graph):
    /// let params = serde_json::json!({
    ///     "content": "experiment data",
    ///     "author": "wetSpring:ltee-b7",
    /// });
    /// let result = ctx.dispatch("nest.store", &params)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the signal identifier is malformed (no `.`
    /// separator), the tier is unrecognized, or dispatch fails.
    pub fn dispatch(
        &mut self,
        signal_id: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        let (tier, name) = signal_id.split_once('.').ok_or_else(|| IpcError::ProtocolError {
            detail: format!(
                "signal identifier must be 'tier.name' (e.g. 'nest.store'), got: {signal_id:?}"
            ),
        })?;

        if !Self::is_composition_tier(tier) {
            return Err(IpcError::ProtocolError {
                detail: format!(
                    "unrecognized signal tier {tier:?} in {signal_id:?} — valid tiers: {:?}",
                    Self::COMPOSITION_TIERS,
                ),
            });
        }

        self.composition(tier, name, params)
    }

    /// Ask squirrel to plan a multi-signal workflow from user intent.
    ///
    /// Sends the intent to the `ai` capability via `ai.query` with
    /// the signal tool schema context, expecting squirrel to return
    /// an ordered list of `(tier, signal_name, params)` tuples.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `ai` capability is absent or the
    /// query fails.
    pub fn composition_plan(
        &mut self,
        intent: &str,
        context: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        let plan_params = serde_json::json!({
            "prompt": intent,
            "context": context,
            "mode": "composition_plan",
            "tool_schema": "config/composition_tools.toml",
        });
        self.call("ai", "ai.query", plan_params)
    }

    /// Execute a signal plan — an ordered sequence of atomic signals.
    ///
    /// Takes a plan (as returned by [`composition_plan`](Self::composition_plan) or
    /// constructed manually) and dispatches each step sequentially through
    /// [`signal`](Self::signal). Collects results into a JSON array.
    ///
    /// The plan should be a JSON array of objects, each with `tier`,
    /// `signal`, and `params` fields.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on the first signal that fails. Completed
    /// steps are included in the partial results.
    pub fn execute_plan(
        &mut self,
        plan: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        let steps = plan.as_array().ok_or_else(|| IpcError::ProtocolError {
            detail: "signal plan must be a JSON array".to_owned(),
        })?;

        let mut results = Vec::with_capacity(steps.len());

        for (i, step) in steps.iter().enumerate() {
            let tier = step
                .get("tier")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| IpcError::ProtocolError {
                    detail: format!("plan step {i} missing 'tier' field"),
                })?;
            let signal_name = step
                .get("signal")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| IpcError::ProtocolError {
                    detail: format!("plan step {i} missing 'signal' field"),
                })?;
            let params = step
                .get("params")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({}));

            let result = self.composition(tier, signal_name, &params)?;
            results.push(serde_json::json!({
                "step": i,
                "tier": tier,
                "signal": signal_name,
                "result": result,
            }));
        }

        Ok(serde_json::Value::Array(results))
    }

    // ── Registration ──────────────────────────────────────────────────

    /// Announce a spring/primal to biomeOS using the modern `primal.announce`
    /// protocol (biomeOS v3.57+).
    ///
    /// Replaces the legacy 3-call registration pattern:
    /// ```text
    /// method.register   + capability.register   + lifecycle.register
    /// ```
    /// with a single atomic announcement that registers lifecycle state,
    /// capabilities, method translations, and signal-tier membership in one
    /// RPC call.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use primalspring::composition::CompositionContext;
    /// # fn example(ctx: &mut CompositionContext) -> Result<(), Box<dyn std::error::Error>> {
    /// // Before (3 separate calls):
    /// // rpc::send(biomeos, "method.register", json!({"primal": "airspring", ...}));
    /// // rpc::send(biomeos, "capability.register", json!({...}));
    /// // rpc::send(biomeos, "lifecycle.register", json!({...}));
    ///
    /// // After (single announce):
    /// ctx.announce(
    ///     "airspring",
    ///     &["ag.measure", "ag.calibrate", "ag.predict"],
    ///     std::path::Path::new("/run/ecoprimals/airspring-family.sock"),
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `orchestration` capability is absent or
    /// biomeOS rejects the announcement. Falls back to `method.register` if
    /// `primal.announce` is not available (pre-v3.57 biomeOS).
    pub fn announce(
        &mut self,
        primal_id: &str,
        methods: &[&str],
        socket: &std::path::Path,
    ) -> Result<serde_json::Value, IpcError> {
        let announce_params = serde_json::json!({
            "primal_id": primal_id,
            "transport": socket.to_string_lossy(),
            "methods": methods,
            "lifecycle": { "state": "running" },
        });

        self.call("orchestration", "primal.announce", announce_params).map_or_else(|_| {
            let register_params = serde_json::json!({
                "primal": primal_id,
                "transport": socket.to_string_lossy(),
                "methods": methods,
            });
            self.call("orchestration", "method.register", register_params)
        }, Ok)
    }

    // ── Composition lifecycle ───────────────────────────────────────────

    /// Request a composition reload via biomeOS Neural API.
    ///
    /// Sends `composition.reload` to the `orchestration` capability,
    /// then re-discovers all capabilities and refreshes BTSP state.
    /// This validates the hot-reload contract (JH-3).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the orchestration capability is absent,
    /// the reload call fails, or re-discovery finds fewer capabilities.
    pub fn reload(&mut self) -> Result<serde_json::Value, IpcError> {
        let result = self.call("orchestration", "composition.reload", serde_json::json!({}))?;
        self.rediscover();
        Ok(result)
    }

    /// Re-discover all capabilities, reconnecting to any that changed
    /// sockets after a topology event (primal restart, graph hot-reload).
    ///
    /// Preserves existing live connections and adds newly discoverable
    /// capabilities. Updates BTSP state for all clients.
    pub fn rediscover(&mut self) {
        for &cap in ALL_CAPS {
            if self.clients.contains_key(cap) {
                if let Some(client) = self.clients.get_mut(cap) {
                    if client.health_check().unwrap_or(false) {
                        continue;
                    }
                }
            }
            if let Ok(client) = crate::ipc::client::connect_by_capability(cap) {
                tracing::info!(cap, "rediscovered capability after topology change");
                self.clients.insert(cap.to_owned(), client);
                self.discovery_paths
                    .insert(cap.to_owned(), DiscoveryPath::LocalDiscovery);
            }
        }
        self.btsp_state = upgrade_btsp_clients(&mut self.clients);
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

    /// How a specific capability was discovered.
    ///
    /// Returns `None` if the capability is not in this context.
    #[must_use]
    pub fn discovery_path(&self, capability: &str) -> Option<DiscoveryPath> {
        self.discovery_paths.get(capability).copied()
    }

    /// Full discovery path map (capability -> mechanism).
    ///
    /// Useful for telemetry, guidestone reporting, and liveSpore-style
    /// provenance journals (mirrors lithoSpore's DiscoveryPath pattern).
    #[must_use]
    pub const fn discovery_paths(&self) -> &BTreeMap<String, DiscoveryPath> {
        &self.discovery_paths
    }

}
