// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Bridge to biomeOS Neural API mode — zero compile-time coupling.
//!
//! biomeOS is the ecosystem's **substrate primal**: it orchestrates
//! deployments, routes capabilities, and coordinates primals. The Neural
//! API is one of its `UniBin` modes (`biomeos neural-api`), providing
//! graph execution and capability routing over JSON-RPC 2.0.
//!
//! This module provides a thin wrapper around [`PrimalClient`] for
//! communicating with biomeOS's neural-api mode. No special client
//! library is needed — biomeOS is just another primal speaking JSON-RPC
//! 2.0 over a Unix socket.
//!
//! Socket discovery uses the standard tiered strategy:
//! 1. `$NEURAL_API_SOCKET` env override
//! 2. `$XDG_RUNTIME_DIR/biomeos/neural-api-{family}.sock`
//! 3. `/tmp/biomeos/neural-api-{family}.sock`
//!
//! # Ecosystem Compliance
//!
//! Per `PRIMAL_IPC_PROTOCOL.md` v3.1 and `STANDARDS_AND_EXPECTATIONS.md`:
//! primals never import other primals as Rust crates. All cross-primal
//! communication is JSON-RPC 2.0 at runtime.

use std::path::{Path, PathBuf};

use super::client::PrimalClient;
use super::error::IpcError;

/// Result of a `capability.call` invocation via the Neural API.
#[derive(Debug, Clone)]
pub struct CapabilityCallResult {
    /// The JSON value returned by the capability provider.
    pub value: serde_json::Value,
}

/// Outcome of a bridge round-trip — captures latency and success for
/// feeding back into the `NeuralDispatcher` metrics and ultimately
/// into biomeOS's adaptive routing weights.
#[derive(Debug, Clone)]
pub struct BridgeOutcome {
    /// Capability domain dispatched.
    pub capability: String,
    /// Operation within the domain.
    pub operation: String,
    /// Wall-clock latency of the round-trip (ms).
    pub latency_ms: u64,
    /// Whether the call succeeded.
    pub success: bool,
    /// Unix epoch milliseconds when dispatch occurred.
    pub timestamp_epoch_ms: u64,
}

/// Bridge to biomeOS's neural-api mode (graph orchestration + capability routing).
///
/// biomeOS is the substrate primal — the ecosystem's composition and
/// deployment orchestrator. Its `neural-api` mode exposes graph execution
/// and capability routing. This struct wraps [`PrimalClient`] with the
/// neural-api method names and socket discovery conventions. All
/// communication is JSON-RPC 2.0 over a Unix domain socket — no
/// compile-time dependency on biomeOS.
#[derive(Debug)]
pub struct NeuralBridge {
    socket_path: PathBuf,
    client: std::cell::RefCell<Option<PrimalClient>>,
}

impl NeuralBridge {
    /// Discover the biomeOS neural-api socket using standard tiered lookup.
    ///
    /// Returns `None` if no socket is found (biomeOS not running).
    #[must_use]
    pub fn discover() -> Option<Self> {
        Self::discover_with(None, None)
    }

    /// Discover the biomeOS neural-api with optional explicit socket path and family ID.
    ///
    /// When `socket_hint` is `Some`, that path is tried first. Otherwise
    /// the standard `{NEURAL_API_SOCKET}` → XDG → `/tmp` tiers are walked.
    #[must_use]
    pub fn discover_with(socket_hint: Option<&str>, family_hint: Option<&str>) -> Option<Self> {
        use super::discover::socket_is_alive;

        if let Some(hint) = socket_hint {
            let path = PathBuf::from(hint);
            if socket_is_alive(&path) {
                return Some(Self::with_path(path));
            }
        }

        if let Ok(explicit) = std::env::var(crate::env_keys::NEURAL_API_SOCKET) {
            let path = PathBuf::from(&explicit);
            if socket_is_alive(&path) {
                return Some(Self::with_path(path));
            }
        }

        let family = family_hint.map_or_else(crate::env_keys::resolve_family_id, String::from);

        let candidates = [
            format!("neural-api-{family}.sock"),
            format!("biomeos-{family}.sock"),
        ];

        if let Ok(xdg) = std::env::var(crate::env_keys::XDG_RUNTIME_DIR) {
            let base = PathBuf::from(xdg).join(crate::primal_names::BIOMEOS);
            for name in &candidates {
                let path = base.join(name);
                if socket_is_alive(&path) {
                    return Some(Self::with_path(path));
                }
            }
        }

        for name in &candidates {
            let path = std::env::temp_dir()
                .join(crate::primal_names::BIOMEOS)
                .join(name);
            if socket_is_alive(&path) {
                return Some(Self::with_path(path));
            }
        }

        None
    }

    #[expect(clippy::missing_const_for_fn, reason = "RefCell::new is not const-stable")]
    fn with_path(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            client: std::cell::RefCell::new(None),
        }
    }

    /// Get or lazily connect a reusable client. Reconnects on stale connections.
    fn extract(resp: super::protocol::JsonRpcResponse) -> Result<serde_json::Value, IpcError> {
        if let Some(err) = resp.error {
            Err(IpcError::from(err))
        } else {
            Ok(resp.result.unwrap_or(serde_json::Value::Null))
        }
    }

    fn rpc(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value, IpcError> {
        let mut slot = self.client.borrow_mut();

        if slot.is_none() {
            *slot = Some(PrimalClient::connect(&self.socket_path, "neural-api")?);
        }

        let Some(client) = slot.as_mut() else {
            return Err(IpcError::ProtocolError {
                detail: "client slot empty after insert".into(),
            });
        };

        match client.call(method, params.clone()) {
            Ok(resp) => Self::extract(resp),
            Err(e) => {
                *slot = None;
                let mut fresh = PrimalClient::connect(&self.socket_path, "neural-api")
                    .map_err(|_| e)?;
                let resp = fresh.call(method, params)?;
                *slot = Some(fresh);
                Self::extract(resp)
            }
        }
    }

    /// The resolved socket path.
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Check whether the biomeOS neural-api is healthy.
    ///
    /// Uses the liveness fallback chain (`health.liveness` → `health.check`
    /// → `health` → `neural-api.health`) since the Neural API may not
    /// implement every health method. Falls back to `graph.list` as a
    /// last-resort liveness probe — if the API can list graphs, it is alive.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the socket is unreachable or no probe succeeds.
    pub fn health_check(&self) -> Result<bool, IpcError> {
        for method in ["health.liveness", "health.check", "health", "graph.list"] {
            match self.rpc(method, serde_json::Value::Null) {
                Ok(_) => return Ok(true),
                Err(_) if method == "graph.list" => return Ok(false),
                Err(_) => {}
            }
        }
        Ok(false)
    }

    /// Discover what capabilities are registered for a capability name.
    ///
    /// Calls `capability.discover` with the `capability` parameter.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn discover_capability(&self, capability: &str) -> Result<serde_json::Value, IpcError> {
        self.rpc("capability.discover", serde_json::json!({ "capability": capability }))
    }

    /// Discover what capabilities are registered for a domain.
    ///
    /// Calls `capability.discover` with the `domain` parameter — biomeOS v2.78+
    /// accepts both `capability` and `domain` for cross-transport compatibility.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn discover_domain(&self, domain: &str) -> Result<serde_json::Value, IpcError> {
        self.rpc("capability.discover", serde_json::json!({ "domain": domain }))
    }

    /// Deploy a graph via the biomeOS graph executor.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn graph_deploy(&self, graph: &serde_json::Value) -> Result<serde_json::Value, IpcError> {
        self.rpc("graph.execute", graph.clone())
    }

    /// Query graph execution status.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn graph_status(&self, graph_id: &str) -> Result<serde_json::Value, IpcError> {
        self.rpc("graph.status", serde_json::json!({ "graph_id": graph_id }))
    }

    /// Roll back a deployed graph (reverse topological lifecycle.stop + capability.unregister).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn graph_rollback(&self, graph_id: &str) -> Result<serde_json::Value, IpcError> {
        self.rpc("graph.rollback", serde_json::json!({ "graph_id": graph_id }))
    }

    /// Trigger a topology rescan so biomeOS re-discovers late-registering primals.
    ///
    /// Available since biomeOS v2.81. Older versions return "Method not found".
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn topology_rescan(&self) -> Result<serde_json::Value, IpcError> {
        self.rpc("topology.rescan", serde_json::Value::Null)
    }

    /// Invoke a semantic capability via the biomeOS capability router.
    ///
    /// biomeOS resolves `{capability}.{operation}` to the correct provider
    /// primal, translates the method, and returns the result.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn capability_call(
        &self,
        capability: &str,
        operation: &str,
        args: &serde_json::Value,
    ) -> Result<CapabilityCallResult, IpcError> {
        let params = serde_json::json!({
            "capability": capability,
            "operation": operation,
            "args": args,
        });
        let value = self.rpc("capability.call", params)?;
        Ok(CapabilityCallResult { value })
    }

    // ── Instrumented dispatch (feedback loop) ─────────────────────────
    //
    // These methods wrap core calls with timing + success/failure
    // recording, producing `BridgeOutcome` values that the
    // `NeuralDispatcher` can ingest as metrics.

    /// Invoke `capability.call` and record the round-trip outcome.
    ///
    /// Returns both the call result and a `BridgeOutcome` capturing
    /// latency and success. The caller (typically `NeuralDispatcher`)
    /// feeds the outcome into its metrics for adaptive routing analysis.
    ///
    /// # Errors
    ///
    /// Returns `(Err, BridgeOutcome)` on transport or application failure.
    /// The outcome is always produced regardless of success.
    pub fn capability_call_instrumented(
        &self,
        capability: &str,
        operation: &str,
        args: &serde_json::Value,
    ) -> (Result<CapabilityCallResult, IpcError>, BridgeOutcome) {
        let start = std::time::Instant::now();
        let result = self.capability_call(capability, operation, args);
        let latency_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let success = result.is_ok();
        let outcome = BridgeOutcome {
            capability: capability.to_owned(),
            operation: operation.to_owned(),
            latency_ms,
            success,
            timestamp_epoch_ms: u64::try_from(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
            )
            .unwrap_or(u64::MAX),
        };
        (result, outcome)
    }

    // ── Observatory methods (Layer 4+) ──────────────────────────────
    //
    // primalSpring's domain IS primal coordination. These methods consume
    // biomeOS's runtime routing intelligence so primalSpring can study,
    // validate, and push evolution upstream — the same pattern other
    // springs use for their domain science.

    /// Query the adaptive routing weights from biomeOS (v3.67+).
    ///
    /// Returns the full weight table snapshot and summary statistics.
    /// primalSpring uses this to study routing patterns and validate
    /// that adaptive dispatch is converging correctly.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure or if biomeOS < v3.67.
    pub fn routing_weights(&self) -> Result<serde_json::Value, IpcError> {
        self.rpc("neural_api.routing_weights", serde_json::Value::Null)
    }

    /// Explain the routing decision for a method (v3.67+).
    ///
    /// Returns which providers exist, what translations apply, how each
    /// candidate scores, and which would be selected. This is primalSpring's
    /// primary tool for studying routing intelligence.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure or if biomeOS < v3.67.
    pub fn route_explain(&self, method: &str) -> Result<serde_json::Value, IpcError> {
        self.rpc("neural_api.route_explain", serde_json::json!({ "method": method }))
    }

    /// Query composition patterns from biomeOS (v3.67+).
    ///
    /// Returns all registered composition patterns — the named method
    /// sequences that form emergent systems. primalSpring validates
    /// pattern consistency against its own graph analysis.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure or if biomeOS < v3.67.
    pub fn composition_patterns(&self) -> Result<serde_json::Value, IpcError> {
        self.rpc("neural_api.composition_patterns", serde_json::Value::Null)
    }

    /// Query a tier composition plan from biomeOS (v3.67+).
    ///
    /// Returns which primals, domains, and patterns are needed to deploy
    /// a specific composition tier (tower, node, nest, etc.).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure or if biomeOS < v3.67.
    pub fn plan_tier(&self, tier: &str) -> Result<serde_json::Value, IpcError> {
        self.rpc("neural_api.plan_tier", serde_json::json!({ "tier": tier }))
    }

    /// Query capability utilization tracking from biomeOS (v3.69+).
    ///
    /// Returns hot/cold method analysis — call counts, last-called
    /// timestamps, and summary statistics. primalSpring uses this to
    /// verify the feedback loop is accumulating operational data.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure or if biomeOS < v3.69.
    pub fn utilization(&self) -> Result<serde_json::Value, IpcError> {
        self.rpc("neural_api.utilization", serde_json::Value::Null)
    }

    /// Query routing weight health diagnostics from biomeOS (v3.70+).
    ///
    /// Returns convergence diagnostics: healthy flag, persistence status,
    /// convergence stats (converging vs cold providers), and open circuit
    /// breaker details. primalSpring uses this to validate that adaptive
    /// routing is converging and that persistent weights survived restarts.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport failure or if biomeOS < v3.70.
    pub fn weight_health(&self) -> Result<serde_json::Value, IpcError> {
        self.rpc("neural_api.weight_health", serde_json::Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_returns_none_when_no_biomeos() {
        assert!(NeuralBridge::discover().is_none());
    }

    #[test]
    fn discover_with_bad_hint_falls_through() {
        let bridge = NeuralBridge::discover_with(Some("/nonexistent/socket.sock"), None);
        assert!(bridge.is_none());
    }

    #[test]
    fn discover_with_none_hints_returns_none_when_no_sockets() {
        assert!(NeuralBridge::discover_with(None, Some("test-family")).is_none());
    }

    #[test]
    fn socket_path_accessor() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/tmp/test.sock"));
        assert_eq!(bridge.socket_path(), Path::new("/tmp/test.sock"));
    }

    #[test]
    fn health_check_returns_false_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(!bridge.health_check().unwrap_or(false));
    }

    #[test]
    fn discover_capability_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.discover_capability("crypto").is_err());
    }

    #[test]
    fn discover_domain_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.discover_domain("crypto").is_err());
    }

    #[test]
    fn capability_call_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        let args = serde_json::json!({});
        assert!(bridge.capability_call("crypto", "sign", &args).is_err());
    }

    #[test]
    fn capability_call_instrumented_records_outcome() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        let args = serde_json::json!({});
        let (result, outcome) = bridge.capability_call_instrumented("crypto", "sign", &args);
        assert!(result.is_err());
        assert!(!outcome.success);
        assert_eq!(outcome.capability, "crypto");
        assert_eq!(outcome.operation, "sign");
        assert!(outcome.latency_ms < 1000);
        assert!(outcome.timestamp_epoch_ms > 0);
    }

    #[test]
    fn graph_deploy_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        let graph = serde_json::json!({"nodes": []});
        assert!(bridge.graph_deploy(&graph).is_err());
    }

    #[test]
    fn graph_status_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.graph_status("test-graph-123").is_err());
    }

    #[test]
    fn graph_rollback_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.graph_rollback("test-graph-123").is_err());
    }

    #[test]
    fn topology_rescan_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.topology_rescan().is_err());
    }

    #[test]
    fn utilization_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.utilization().is_err());
    }

    #[test]
    fn weight_health_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge::with_path(PathBuf::from("/nonexistent/neural-api.sock"));
        assert!(bridge.weight_health().is_err());
    }
}
