// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
//! - **Composition dispatch**: named patterns like "`rootpulse_commit`" can be
//!   executed as a single `graph.execute` call, composing multiple primals.
//!
//! - **Metrics**: each dispatch records latency and outcome for future
//!   adaptive routing (Layer 4 of the Neural API evolution model).

pub mod metrics;
pub mod perceptron;
pub mod pipeline;

use std::sync::Arc;
use std::time::Instant;

use super::neural_routing::{
    CompositionPattern, CompositionTier, NeuralRoutingTable, RouteEntry, TierComposition,
    canonical_routing_table,
};
use crate::ipc::error::IpcError;
use crate::ipc::neural_bridge::{BridgeOutcome, NeuralBridge};

pub use metrics::{DispatchMetric, PrimalDispatchSummary};

/// Typed errors for neural dispatch operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DispatchError {
    /// Method not found in the routing table.
    #[error("method {0} not in routing table")]
    MethodNotFound(String),
    /// Named composition pattern not found.
    #[error("pattern {0} not found")]
    PatternNotFound(String),
    /// biomeOS Neural API is not reachable.
    #[error("biomeOS not available")]
    BridgeOffline,
    /// IPC-level error during dispatch — wraps the original typed error
    /// via `Arc` since `IpcError` is not `Clone` (contains `io::Error`).
    #[error("dispatch ipc: {0}")]
    Ipc(Arc<IpcError>),
    /// Graph execution failed (wraps IPC error from biomeOS graph deploy).
    #[error("graph dispatch failed: {0}")]
    GraphFailed(Arc<IpcError>),
}

/// Outcome of a single neural dispatch.
///
/// Uses `Arc<str>` for method/owner to avoid per-dispatch allocations —
/// these are cloned from the routing table's interned `RouteEntry` strings.
#[derive(Debug, Clone)]
pub struct DispatchOutcome {
    /// The method that was dispatched.
    pub method: Arc<str>,
    /// Which primal handled it.
    pub owner: Arc<str>,
    /// The composition tier used.
    pub tier: CompositionTier,
    /// How the dispatch was routed.
    pub route_path: RoutePath,
    /// Result value from the primal (on success).
    pub result: Result<serde_json::Value, DispatchError>,
    /// Dispatch latency in milliseconds.
    pub latency_ms: u64,
}

/// How a method was routed to its handler.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RoutePath {
    /// Routed through `capability.call` semantic dispatch.
    CapabilityCall,
    /// Routed through `signal.dispatch` (graph-backed signal tier).
    CompositionDispatch,
    /// Routed through `graph.execute` (composition pattern).
    GraphExecute,
    /// Method not found in routing table.
    Unresolved,
    /// `NeuralBridge` not available (biomeOS not running).
    Offline,
}

/// High-level neural dispatch surface.
///
/// Wraps the routing table and optional `NeuralBridge` to provide method
/// dispatch across the full capability method surface. When biomeOS is running,
/// dispatches go through `capability.call`. When offline, returns structured
/// errors that downstream can handle gracefully.
pub struct NeuralDispatcher {
    table: NeuralRoutingTable,
    bridge: Option<NeuralBridge>,
    pub(crate) metrics: Vec<DispatchMetric>,
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
    pub const fn is_online(&self) -> bool {
        self.bridge.is_some()
    }

    /// Access the underlying routing table.
    #[must_use]
    pub const fn routing_table(&self) -> &NeuralRoutingTable {
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
    /// 2. If `NeuralBridge` available, dispatch via `capability.call`
    /// 3. Record metrics for adaptive routing
    pub fn dispatch(&mut self, method: &str, params: &serde_json::Value) -> DispatchOutcome {
        let start = Instant::now();

        let Some(entry) = self.table.route(method).cloned() else {
            let m: Arc<str> = Arc::from(method);
            return DispatchOutcome {
                method: m,
                owner: Arc::from("unknown"),
                tier: CompositionTier::Standalone,
                route_path: RoutePath::Unresolved,
                result: Err(DispatchError::MethodNotFound(method.to_owned())),
                latency_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            };
        };

        let (result, route_path) = self.bridge.as_ref().map_or(
            (Err(DispatchError::BridgeOffline), RoutePath::Offline),
            |bridge| dispatch_through_bridge(bridge, method, &entry, params),
        );

        let latency_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let success = result.is_ok();
        let method_arc = Arc::clone(&entry.method);
        let owner_arc = Arc::clone(&entry.owner);

        self.metrics.push(DispatchMetric {
            method: Arc::clone(&method_arc),
            owner: Arc::clone(&owner_arc),
            tier: entry.tier,
            latency_ms,
            success,
            route_path: route_path.clone(),
            timestamp_epoch_ms: epoch_ms(),
        });

        DispatchOutcome {
            method: method_arc,
            owner: owner_arc,
            tier: entry.tier,
            route_path,
            result,
            latency_ms,
        }
    }

    /// Dispatch a composition pattern by name.
    ///
    /// If the pattern is registered, constructs a graph execution request
    /// and dispatches via `graph.execute`.
    pub fn dispatch_pattern(
        &mut self,
        pattern_name: &str,
        params: &serde_json::Value,
    ) -> DispatchOutcome {
        let start = Instant::now();

        let pattern = match self
            .table
            .patterns()
            .iter()
            .find(|p| &*p.name == pattern_name)
        {
            Some(p) => p.clone(),
            None => {
                return DispatchOutcome {
                    method: Arc::from(pattern_name),
                    owner: Arc::from("unknown"),
                    tier: CompositionTier::Standalone,
                    route_path: RoutePath::Unresolved,
                    result: Err(DispatchError::PatternNotFound(pattern_name.to_owned())),
                    latency_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
                };
            }
        };

        let graph_request = build_graph_request(&pattern, params);
        let result = self
            .bridge
            .as_ref()
            .map_or(Err(DispatchError::BridgeOffline), |bridge| {
                bridge
                    .graph_deploy(&graph_request)
                    .map_err(|e| DispatchError::GraphFailed(Arc::new(e)))
            });

        let latency_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let success = result.is_ok();
        let method_arc = Arc::clone(&pattern.name);
        let owner_arc: Arc<str> = Arc::from(join_arc_strs(&pattern.primals, "+"));

        self.metrics.push(DispatchMetric {
            method: Arc::clone(&method_arc),
            owner: Arc::clone(&owner_arc),
            tier: pattern.tier,
            latency_ms,
            success,
            route_path: RoutePath::GraphExecute,
            timestamp_epoch_ms: epoch_ms(),
        });

        DispatchOutcome {
            method: method_arc,
            owner: owner_arc,
            tier: pattern.tier,
            route_path: RoutePath::GraphExecute,
            result,
            latency_ms,
        }
    }

    /// Ingest a `BridgeOutcome` from an external `capability_call_instrumented`
    /// round-trip.
    pub fn record_bridge_outcome(&mut self, outcome: &BridgeOutcome) {
        let method_str = format!("{}.{}", outcome.capability, outcome.operation);
        let (method_arc, owner_arc, tier) = self.table.route(&method_str).map_or_else(
            || {
                (
                    Arc::from(method_str.as_str()),
                    Arc::from("unknown"),
                    CompositionTier::Standalone,
                )
            },
            |entry| {
                (
                    Arc::clone(&entry.method),
                    Arc::clone(&entry.owner),
                    entry.tier,
                )
            },
        );

        self.metrics.push(DispatchMetric {
            method: method_arc,
            owner: owner_arc,
            tier,
            latency_ms: outcome.latency_ms,
            success: outcome.success,
            route_path: RoutePath::CapabilityCall,
            timestamp_epoch_ms: outcome.timestamp_epoch_ms,
        });
    }

    /// Dispatch a method using the instrumented bridge path, recording the
    /// round-trip outcome into metrics automatically.
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
                    method: Arc::from(method),
                    owner: Arc::from("unknown"),
                    tier: CompositionTier::Standalone,
                    route_path: RoutePath::Unresolved,
                    result: Err(DispatchError::MethodNotFound(method.to_owned())),
                    latency_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
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
                    .map_err(|e: IpcError| DispatchError::Ipc(Arc::new(e)));
                (result, RoutePath::CapabilityCall)
            }
            None => (Err(DispatchError::BridgeOffline), RoutePath::Offline),
        };

        let latency_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        DispatchOutcome {
            method: Arc::clone(&entry.method),
            owner: Arc::clone(&entry.owner),
            tier: entry.tier,
            route_path,
            result,
            latency_ms,
        }
    }
}

fn dispatch_through_bridge(
    bridge: &NeuralBridge,
    method: &str,
    _entry: &RouteEntry,
    params: &serde_json::Value,
) -> (Result<serde_json::Value, DispatchError>, RoutePath) {
    let (domain, operation) = match method.split_once('.') {
        Some((d, o)) => (d, o),
        None => (method, ""),
    };

    let result = bridge
        .capability_call(domain, operation, params)
        .map(|r| r.value)
        .map_err(|e: IpcError| DispatchError::Ipc(Arc::new(e)));

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
        "graph_id": &*pattern.name,
        "coordination": "sequential",
        "nodes": nodes,
        "params": params,
    })
}

fn join_arc_strs(parts: &[Arc<str>], sep: &str) -> String {
    let strs: Vec<&str> = parts.iter().map(|s| &**s).collect();
    strs.join(sep)
}

fn epoch_ms() -> u64 {
    u64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )
    .unwrap_or(u64::MAX)
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
        assert_eq!(&*entry.owner, "beardog");
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
        assert_eq!(&*outcome.owner, "beardog");
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
        assert!(report["total_methods"].as_u64().unwrap() >= 460);
        assert!(report["patterns_registered"].as_u64().unwrap() >= 3);
    }

    #[test]
    fn plan_tower_tier() {
        let d = test_dispatcher();
        let comp = d.plan_tier(CompositionTier::Tower);
        assert!(comp.primals.iter().any(|s| &**s == "beardog"));
        assert!(comp.primals.iter().any(|s| &**s == "songbird"));
        assert!(comp.method_count >= 50);
    }

    #[test]
    fn plan_nest_tier() {
        let d = test_dispatcher();
        let comp = d.plan_tier(CompositionTier::Nest);
        assert!(comp.primals.iter().any(|s| &**s == "nestgate"));
        assert!(comp.primals.iter().any(|s| &**s == "loamspine"));
    }

    #[test]
    fn primal_summary_after_dispatches() {
        let mut d = test_dispatcher();
        d.dispatch("crypto.hash", &serde_json::json!({}));
        d.dispatch("crypto.sign", &serde_json::json!({}));
        d.dispatch("storage.store", &serde_json::json!({}));
        let summary = d.primal_summary();
        let key: Arc<str> = Arc::from("beardog");
        assert!(summary.contains_key(&key));
        assert_eq!(summary[&key].total_dispatches, 2);
    }

    #[test]
    fn record_bridge_outcome_creates_metric() {
        let mut d = test_dispatcher();
        let outcome = BridgeOutcome {
            capability: "crypto".into(),
            operation: "hash".into(),
            latency_ms: 42,
            success: true,
            timestamp_epoch_ms: 1_700_000_000_000,
        };
        d.record_bridge_outcome(&outcome);
        assert_eq!(d.metrics().len(), 1);
        assert_eq!(&*d.metrics()[0].method, "crypto.hash");
        assert_eq!(&*d.metrics()[0].owner, "beardog");
        assert_eq!(d.metrics()[0].latency_ms, 42);
        assert!(d.metrics()[0].success);
    }

    #[test]
    fn record_bridge_outcome_unknown_method() {
        let mut d = test_dispatcher();
        let outcome = BridgeOutcome {
            capability: "nonexistent".into(),
            operation: "thing".into(),
            latency_ms: 5,
            success: false,
            timestamp_epoch_ms: 1_700_000_000_000,
        };
        d.record_bridge_outcome(&outcome);
        assert_eq!(&*d.metrics()[0].owner, "unknown");
        assert_eq!(d.metrics()[0].tier, CompositionTier::Standalone);
    }

    #[test]
    fn dispatch_instrumented_offline() {
        let mut d = test_dispatcher();
        let outcome = d.dispatch_instrumented("crypto.hash", &serde_json::json!({}));
        assert_eq!(outcome.route_path, RoutePath::Offline);
        assert!(outcome.result.is_err());
        assert_eq!(&*outcome.owner, "beardog");
    }

    #[test]
    fn build_graph_request_structure() {
        let pattern = CompositionPattern {
            name: "test_pattern".into(),
            methods: vec!["crypto.sign".into(), "dag.append".into()],
            primals: vec!["beardog".into(), "rhizocrypt".into()],
            tier: CompositionTier::Nest,
        };
        let req = build_graph_request(&pattern, &serde_json::json!({"key": "val"}));
        assert_eq!(req["graph_id"], "test_pattern");
        assert_eq!(req["nodes"].as_array().unwrap().len(), 2);
        let step1 = &req["nodes"][1];
        assert_eq!(step1["depends_on"][0], "step_0");
    }
}
