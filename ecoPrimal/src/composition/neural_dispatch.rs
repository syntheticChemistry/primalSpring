// SPDX-License-Identifier: AGPL-3.0-or-later

//! Neural dispatch — resolve and invoke methods through the Neural API.
//!
//! This module connects the `NeuralRoutingTable` (which models the full method
//! surface) to the `NeuralBridge` (which speaks to biomeOS). It provides:
//!
//! - **`NeuralDispatcher`**: a high-level dispatch surface that resolves
//!   methods to their owning primal, selects the appropriate transport tier,
//!   and forwards through `capability.call` or `signal.dispatch`.
//!
//! - **Tier-aware dispatch**: Tower methods go through Tower relay, Nest
//!   methods through Nest signal graphs, etc. Standalone methods dispatch
//!   directly.
//!
//! - **Composition dispatch**: named patterns like "rootpulse_commit" can be
//!   executed as a single `graph.execute` call, composing multiple primals.
//!
//! - **Metrics**: each dispatch records latency and outcome for future
//!   adaptive routing (Layer 4 of the Neural API evolution model).

use std::time::Instant;

use super::neural_routing::{
    canonical_routing_table, CompositionPattern, CompositionTier, NeuralRoutingTable, RouteEntry,
    TierComposition,
};
use crate::ipc::error::IpcError;
use crate::ipc::neural_bridge::{BridgeOutcome, NeuralBridge};

/// Outcome of a single neural dispatch.
#[derive(Debug, Clone)]
pub struct DispatchOutcome {
    /// The method that was dispatched.
    pub method: String,
    /// Which primal handled it.
    pub owner: String,
    /// The composition tier used.
    pub tier: CompositionTier,
    /// How the dispatch was routed.
    pub route_path: RoutePath,
    /// Result value from the primal (on success).
    pub result: Result<serde_json::Value, String>,
    /// Dispatch latency in milliseconds.
    pub latency_ms: u64,
}

/// How a method was routed to its handler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutePath {
    /// Routed through `capability.call` semantic dispatch.
    CapabilityCall,
    /// Routed through `signal.dispatch` (graph-backed signal tier).
    SignalDispatch,
    /// Routed through `graph.execute` (composition pattern).
    GraphExecute,
    /// Method not found in routing table.
    Unresolved,
    /// NeuralBridge not available (biomeOS not running).
    Offline,
}

/// Metrics collected per dispatch — the raw data for adaptive routing.
#[derive(Debug, Clone)]
pub struct DispatchMetric {
    /// JSON-RPC method or pattern name.
    pub method: String,
    /// Primal that handled the dispatch.
    pub owner: String,
    /// Composition tier.
    pub tier: CompositionTier,
    /// Wall-clock latency in milliseconds.
    pub latency_ms: u64,
    /// Whether the dispatch succeeded.
    pub success: bool,
    /// How the method was routed.
    pub route_path: RoutePath,
    /// Unix epoch milliseconds when dispatch occurred.
    pub timestamp_epoch_ms: u64,
}

/// High-level neural dispatch surface.
///
/// Wraps the routing table and optional NeuralBridge to provide method
/// dispatch across the full capability method surface. When biomeOS is running,
/// dispatches go through `capability.call`. When offline, returns structured
/// errors that downstream can handle gracefully.
pub struct NeuralDispatcher {
    table: NeuralRoutingTable,
    bridge: Option<NeuralBridge>,
    metrics: Vec<DispatchMetric>,
}

impl NeuralDispatcher {
    /// Create a dispatcher from the canonical routing table.
    /// Attempts to discover biomeOS for live dispatch.
    #[must_use]
    pub fn discover() -> Self {
        Self {
            table: canonical_routing_table(),
            bridge: NeuralBridge::discover(),
            metrics: Vec::new(),
        }
    }

    /// Create a dispatcher with an explicit routing table (for testing).
    #[must_use]
    pub fn with_table(table: NeuralRoutingTable) -> Self {
        Self {
            table,
            bridge: NeuralBridge::discover(),
            metrics: Vec::new(),
        }
    }

    /// Whether biomeOS is reachable for live dispatch.
    #[must_use]
    pub fn is_online(&self) -> bool {
        self.bridge.is_some()
    }

    /// Access the underlying routing table.
    #[must_use]
    pub fn routing_table(&self) -> &NeuralRoutingTable {
        &self.table
    }

    /// Resolve a method to its route entry without dispatching.
    #[must_use]
    pub fn resolve(&self, method: &str) -> Option<&RouteEntry> {
        self.table.route(method)
    }

    /// Plan a composition — determine which primals and methods are needed
    /// for a given tier deployment.
    #[must_use]
    pub fn plan_tier(&self, tier: CompositionTier) -> TierComposition {
        self.table.tier_composition(tier)
    }

    /// Dispatch a single method through the Neural API.
    ///
    /// Resolution order:
    /// 1. Look up method in routing table → get owner, domain, tier
    /// 2. If NeuralBridge available, dispatch via `capability.call`
    /// 3. Record metrics for adaptive routing
    pub fn dispatch(
        &mut self,
        method: &str,
        params: &serde_json::Value,
    ) -> DispatchOutcome {
        let start = Instant::now();

        let entry = match self.table.route(method) {
            Some(e) => e.clone(),
            None => {
                return DispatchOutcome {
                    method: method.to_owned(),
                    owner: "unknown".to_owned(),
                    tier: CompositionTier::Standalone,
                    route_path: RoutePath::Unresolved,
                    result: Err(format!("method {method} not in routing table")),
                    latency_ms: start.elapsed().as_millis() as u64,
                };
            }
        };

        let (result, route_path) = match &self.bridge {
            Some(bridge) => dispatch_through_bridge(bridge, method, &entry, params),
            None => (Err("biomeOS not available".to_owned()), RoutePath::Offline),
        };

        let latency_ms = start.elapsed().as_millis() as u64;
        let success = result.is_ok();

        self.metrics.push(DispatchMetric {
            method: method.to_owned(),
            owner: entry.owner.clone(),
            tier: entry.tier,
            latency_ms,
            success,
            route_path: route_path.clone(),
            timestamp_epoch_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });

        DispatchOutcome {
            method: method.to_owned(),
            owner: entry.owner,
            tier: entry.tier,
            route_path,
            result,
            latency_ms,
        }
    }

    /// Dispatch a composition pattern by name.
    ///
    /// If the pattern is registered, constructs a graph execution request
    /// and dispatches via `graph.execute`. Each method in the pattern is
    /// treated as a graph node.
    pub fn dispatch_pattern(
        &mut self,
        pattern_name: &str,
        params: &serde_json::Value,
    ) -> DispatchOutcome {
        let start = Instant::now();

        let pattern = match self.table.patterns().iter().find(|p| p.name == pattern_name) {
            Some(p) => p.clone(),
            None => {
                return DispatchOutcome {
                    method: pattern_name.to_owned(),
                    owner: "unknown".to_owned(),
                    tier: CompositionTier::Standalone,
                    route_path: RoutePath::Unresolved,
                    result: Err(format!("pattern {pattern_name} not found")),
                    latency_ms: start.elapsed().as_millis() as u64,
                };
            }
        };

        let graph_request = build_graph_request(&pattern, params);
        let result = match &self.bridge {
            Some(bridge) => bridge
                .graph_deploy(&graph_request)
                .map_err(|e| format!("graph dispatch failed: {e}")),
            None => Err("biomeOS not available".to_owned()),
        };

        let latency_ms = start.elapsed().as_millis() as u64;
        let success = result.is_ok();

        self.metrics.push(DispatchMetric {
            method: pattern_name.to_owned(),
            owner: pattern.primals.join("+"),
            tier: pattern.tier,
            latency_ms,
            success,
            route_path: RoutePath::GraphExecute,
            timestamp_epoch_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });

        DispatchOutcome {
            method: pattern_name.to_owned(),
            owner: pattern.primals.join("+"),
            tier: pattern.tier,
            route_path: RoutePath::GraphExecute,
            result,
            latency_ms,
        }
    }

    /// Ingest a `BridgeOutcome` from an external `capability_call_instrumented`
    /// round-trip. This feeds the observatory's metric collection from any
    /// code path that uses the NeuralBridge directly (not just `dispatch()`).
    pub fn record_bridge_outcome(&mut self, outcome: &BridgeOutcome) {
        let method = format!("{}.{}", outcome.capability, outcome.operation);
        let owner = self
            .table
            .route(&method)
            .map_or_else(|| "unknown".to_owned(), |e| e.owner.clone());
        let tier = self
            .table
            .route(&method)
            .map_or(CompositionTier::Standalone, |e| e.tier);

        self.metrics.push(DispatchMetric {
            method,
            owner,
            tier,
            latency_ms: outcome.latency_ms,
            success: outcome.success,
            route_path: RoutePath::CapabilityCall,
            timestamp_epoch_ms: outcome.timestamp_epoch_ms,
        });
    }

    /// Dispatch a method using the instrumented bridge path, recording the
    /// round-trip outcome into metrics automatically.
    ///
    /// Equivalent to `dispatch()` but uses `capability_call_instrumented`
    /// for precise bridge-level timing.
    pub fn dispatch_instrumented(
        &mut self,
        method: &str,
        params: &serde_json::Value,
    ) -> DispatchOutcome {
        let start = Instant::now();

        let entry = match self.table.route(method) {
            Some(e) => e.clone(),
            None => {
                return DispatchOutcome {
                    method: method.to_owned(),
                    owner: "unknown".to_owned(),
                    tier: CompositionTier::Standalone,
                    route_path: RoutePath::Unresolved,
                    result: Err(format!("method {method} not in routing table")),
                    latency_ms: start.elapsed().as_millis() as u64,
                };
            }
        };

        let (domain, operation) = method.split_once('.').unwrap_or((method, ""));

        let (result, route_path) = match &self.bridge {
            Some(bridge) => {
                let (call_result, outcome) =
                    bridge.capability_call_instrumented(domain, operation, params);
                self.record_bridge_outcome(&outcome);
                let result = call_result
                    .map(|r| r.value)
                    .map_err(|e: IpcError| format!("{e}"));
                (result, RoutePath::CapabilityCall)
            }
            None => (Err("biomeOS not available".to_owned()), RoutePath::Offline),
        };

        let latency_ms = start.elapsed().as_millis() as u64;

        DispatchOutcome {
            method: method.to_owned(),
            owner: entry.owner,
            tier: entry.tier,
            route_path,
            result,
            latency_ms,
        }
    }

    /// All collected dispatch metrics (for adaptive routing analysis).
    #[must_use]
    pub fn metrics(&self) -> &[DispatchMetric] {
        &self.metrics
    }

    /// Average latency for a specific method across all dispatches.
    #[must_use]
    pub fn avg_latency_ms(&self, method: &str) -> Option<f64> {
        let relevant: Vec<_> = self
            .metrics
            .iter()
            .filter(|m| m.method == method && m.success)
            .collect();
        if relevant.is_empty() {
            return None;
        }
        let total: u64 = relevant.iter().map(|m| m.latency_ms).sum();
        Some(total as f64 / relevant.len() as f64)
    }

    /// Error rate for a specific method (0.0–1.0).
    #[must_use]
    pub fn error_rate(&self, method: &str) -> Option<f64> {
        let relevant: Vec<_> = self
            .metrics
            .iter()
            .filter(|m| m.method == method)
            .collect();
        if relevant.is_empty() {
            return None;
        }
        let failures = relevant.iter().filter(|m| !m.success).count();
        Some(failures as f64 / relevant.len() as f64)
    }

    /// Per-primal dispatch summary.
    #[must_use]
    pub fn primal_summary(&self) -> std::collections::HashMap<String, PrimalDispatchSummary> {
        let mut summaries: std::collections::HashMap<String, PrimalDispatchSummary> =
            std::collections::HashMap::new();
        for m in &self.metrics {
            let entry = summaries.entry(m.owner.clone()).or_insert_with(|| {
                PrimalDispatchSummary {
                    primal: m.owner.clone(),
                    total_dispatches: 0,
                    successes: 0,
                    total_latency_ms: 0,
                }
            });
            entry.total_dispatches += 1;
            if m.success {
                entry.successes += 1;
            }
            entry.total_latency_ms += m.latency_ms;
        }
        summaries
    }

    /// Generate a routing status report as JSON — suitable for
    /// `coordination.neural_api_status` responses.
    #[must_use]
    pub fn status_report(&self) -> serde_json::Value {
        let summary = self.table.tier_summary();
        serde_json::json!({
            "online": self.is_online(),
            "total_methods": self.table.method_count(),
            "total_domains": self.table.domain_count(),
            "total_primals": self.table.primal_count(),
            "tier_distribution": summary,
            "patterns_registered": self.table.patterns().len(),
            "dispatches_recorded": self.metrics.len(),
        })
    }
}

/// Per-primal dispatch statistics.
#[derive(Debug, Clone)]
pub struct PrimalDispatchSummary {
    /// Primal identifier.
    pub primal: String,
    /// Total dispatch attempts to this primal.
    pub total_dispatches: u64,
    /// Successful dispatches.
    pub successes: u64,
    /// Cumulative latency across all dispatches (ms).
    pub total_latency_ms: u64,
}

impl PrimalDispatchSummary {
    /// Average latency per dispatch to this primal.
    #[must_use]
    pub fn avg_latency_ms(&self) -> f64 {
        if self.total_dispatches == 0 {
            return 0.0;
        }
        self.total_latency_ms as f64 / self.total_dispatches as f64
    }

    /// Success rate (0.0–1.0) for dispatches to this primal.
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        if self.total_dispatches == 0 {
            return 0.0;
        }
        self.successes as f64 / self.total_dispatches as f64
    }
}

fn dispatch_through_bridge(
    bridge: &NeuralBridge,
    method: &str,
    _entry: &RouteEntry,
    params: &serde_json::Value,
) -> (Result<serde_json::Value, String>, RoutePath) {
    let (domain, operation) = match method.split_once('.') {
        Some((d, o)) => (d, o),
        None => (method, ""),
    };

    let result = bridge
        .capability_call(domain, operation, params)
        .map(|r| r.value)
        .map_err(|e: IpcError| format!("{e}"));

    (result, RoutePath::CapabilityCall)
}

fn build_graph_request(
    pattern: &CompositionPattern,
    params: &serde_json::Value,
) -> serde_json::Value {
    let nodes: Vec<serde_json::Value> = pattern
        .methods
        .iter()
        .enumerate()
        .map(|(i, method)| {
            let (domain, operation) = method.split_once('.').unwrap_or((method, ""));
            serde_json::json!({
                "id": format!("step_{i}"),
                "capability": domain,
                "operation": operation,
                "order": i,
                "depends_on": if i > 0 { vec![format!("step_{}", i - 1)] } else { vec![] },
            })
        })
        .collect();

    serde_json::json!({
        "graph_id": pattern.name,
        "coordination": "sequential",
        "nodes": nodes,
        "params": params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dispatcher() -> NeuralDispatcher {
        NeuralDispatcher {
            table: canonical_routing_table(),
            bridge: None,
            metrics: Vec::new(),
        }
    }

    #[test]
    fn resolve_crypto_hash() {
        let d = test_dispatcher();
        let entry = d.resolve("crypto.hash").expect("should resolve");
        assert_eq!(entry.owner, "beardog");
        assert_eq!(entry.tier, CompositionTier::Tower);
    }

    #[test]
    fn resolve_unknown_returns_none() {
        let d = test_dispatcher();
        assert!(d.resolve("nonexistent.method").is_none());
    }

    #[test]
    fn dispatch_offline_returns_error() {
        let mut d = test_dispatcher();
        let outcome = d.dispatch("crypto.hash", &serde_json::json!({}));
        assert_eq!(outcome.route_path, RoutePath::Offline);
        assert!(outcome.result.is_err());
        assert_eq!(outcome.owner, "beardog");
    }

    #[test]
    fn dispatch_unresolved_method() {
        let mut d = test_dispatcher();
        let outcome = d.dispatch("nonexistent.method", &serde_json::json!({}));
        assert_eq!(outcome.route_path, RoutePath::Unresolved);
        assert!(outcome.result.is_err());
    }

    #[test]
    fn dispatch_pattern_offline() {
        let mut d = test_dispatcher();
        let outcome = d.dispatch_pattern("rootpulse_commit", &serde_json::json!({}));
        assert_eq!(outcome.route_path, RoutePath::GraphExecute);
        assert!(outcome.result.is_err());
        assert!(outcome.owner.contains("beardog"));
    }

    #[test]
    fn dispatch_pattern_unknown() {
        let mut d = test_dispatcher();
        let outcome = d.dispatch_pattern("nonexistent_pattern", &serde_json::json!({}));
        assert_eq!(outcome.route_path, RoutePath::Unresolved);
    }

    #[test]
    fn metrics_recorded_after_dispatch() {
        let mut d = test_dispatcher();
        d.dispatch("crypto.hash", &serde_json::json!({}));
        d.dispatch("storage.store", &serde_json::json!({}));
        assert_eq!(d.metrics().len(), 2);
    }

    #[test]
    fn status_report_structure() {
        let d = test_dispatcher();
        let report = d.status_report();
        assert!(!report["online"].as_bool().unwrap());
        assert!(report["total_methods"].as_u64().unwrap() >= 450);
        assert!(report["patterns_registered"].as_u64().unwrap() >= 3);
    }

    #[test]
    fn plan_tower_tier() {
        let d = test_dispatcher();
        let comp = d.plan_tier(CompositionTier::Tower);
        assert!(comp.primals.contains(&"beardog".to_owned()));
        assert!(comp.primals.contains(&"songbird".to_owned()));
        assert!(comp.method_count >= 50);
    }

    #[test]
    fn plan_nest_tier() {
        let d = test_dispatcher();
        let comp = d.plan_tier(CompositionTier::Nest);
        assert!(comp.primals.contains(&"nestgate".to_owned()));
        assert!(comp.primals.contains(&"loamspine".to_owned()));
    }

    #[test]
    fn primal_summary_after_dispatches() {
        let mut d = test_dispatcher();
        d.dispatch("crypto.hash", &serde_json::json!({}));
        d.dispatch("crypto.sign", &serde_json::json!({}));
        d.dispatch("storage.store", &serde_json::json!({}));
        let summary = d.primal_summary();
        assert!(summary.contains_key("beardog"));
        assert_eq!(summary["beardog"].total_dispatches, 2);
    }

    #[test]
    fn record_bridge_outcome_creates_metric() {
        let mut d = test_dispatcher();
        let outcome = BridgeOutcome {
            capability: "crypto".to_owned(),
            operation: "hash".to_owned(),
            latency_ms: 42,
            success: true,
            timestamp_epoch_ms: 1_700_000_000_000,
        };
        d.record_bridge_outcome(&outcome);
        assert_eq!(d.metrics().len(), 1);
        assert_eq!(d.metrics()[0].method, "crypto.hash");
        assert_eq!(d.metrics()[0].owner, "beardog");
        assert_eq!(d.metrics()[0].latency_ms, 42);
        assert!(d.metrics()[0].success);
    }

    #[test]
    fn record_bridge_outcome_unknown_method() {
        let mut d = test_dispatcher();
        let outcome = BridgeOutcome {
            capability: "nonexistent".to_owned(),
            operation: "thing".to_owned(),
            latency_ms: 5,
            success: false,
            timestamp_epoch_ms: 1_700_000_000_000,
        };
        d.record_bridge_outcome(&outcome);
        assert_eq!(d.metrics()[0].owner, "unknown");
        assert_eq!(d.metrics()[0].tier, CompositionTier::Standalone);
    }

    #[test]
    fn dispatch_instrumented_offline() {
        let mut d = test_dispatcher();
        let outcome = d.dispatch_instrumented("crypto.hash", &serde_json::json!({}));
        assert_eq!(outcome.route_path, RoutePath::Offline);
        assert!(outcome.result.is_err());
        assert_eq!(outcome.owner, "beardog");
    }

    #[test]
    fn build_graph_request_structure() {
        let pattern = CompositionPattern {
            name: "test_pattern".to_owned(),
            methods: vec!["crypto.sign".to_owned(), "dag.append".to_owned()],
            primals: vec!["beardog".to_owned(), "rhizocrypt".to_owned()],
            tier: CompositionTier::Nest,
        };
        let req = build_graph_request(&pattern, &serde_json::json!({"key": "val"}));
        assert_eq!(req["graph_id"], "test_pattern");
        assert_eq!(req["nodes"].as_array().unwrap().len(), 2);
        let step1 = &req["nodes"][1];
        assert_eq!(step1["depends_on"][0], "step_0");
    }
}
