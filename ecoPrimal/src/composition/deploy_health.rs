// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Fleet deployment health aggregation — Deployment Signaling Phase 2.
//!
//! Consumes `deploy.result` gossip events emitted by biomeOS (Phase 1) and
//! aggregates fleet-wide deployment health. This closes the feedback loop:
//! gates signal deployment outcomes via gossip, not AARs.
//!
//! Phase 1 (biomeOS): emits `deploy.result` gossip after `composition.orchestrate`
//! Phase 2 (primalSpring): aggregates fleet deployment health (this module)
//! Phase 3 (cellMembrane): sovereignty validation → gossip
//! Phase 4 (sporeGate): topology-aware cascade decisions

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// A single deployment result event from a gate's biomeOS.
///
/// Emitted via `gossip.inject` with type `deploy.result` after biomeOS
/// completes a `composition.orchestrate` cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployResult {
    /// Gate name that emitted the event.
    pub gate: String,
    /// Composition kind deployed (e.g. "tower", "nucleus").
    pub composition: String,
    /// Whether the deployment succeeded.
    pub success: bool,
    /// Number of primals that started successfully.
    pub primals_alive: u16,
    /// Number of primals expected.
    pub primals_expected: u16,
    /// Duration of the deployment in milliseconds.
    pub deploy_ms: u64,
    /// Unix timestamp when the deployment completed.
    pub timestamp: u64,
    /// Error message if deployment failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Aggregated deployment health for a single gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateDeployHealth {
    /// Gate name.
    pub gate: String,
    /// Most recent deployment result.
    pub latest: DeployResult,
    /// Total successful deployments seen.
    pub success_count: u64,
    /// Total failed deployments seen.
    pub failure_count: u64,
    /// Rolling average deploy time in milliseconds.
    pub avg_deploy_ms: u64,
    /// Time since last successful deploy (seconds).
    pub staleness_secs: u64,
}

/// Fleet-wide deployment health summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetDeployHealth {
    /// Per-gate deployment health.
    pub gates: HashMap<String, GateDeployHealth>,
    /// Total gates reporting.
    pub gates_reporting: usize,
    /// Gates with healthy deployments (recent + successful).
    pub gates_healthy: usize,
    /// Gates with stale deployments (no report in threshold window).
    pub gates_stale: usize,
    /// Gates with failed latest deployment.
    pub gates_failed: usize,
    /// Timestamp of this summary.
    pub summary_at: u64,
}

/// Staleness threshold — gates not reporting within this window are "stale".
const STALE_THRESHOLD: Duration = Duration::from_secs(3600);

impl FleetDeployHealth {
    /// Create an empty fleet health summary.
    #[must_use]
    pub fn new() -> Self {
        Self {
            gates: HashMap::new(),
            gates_reporting: 0,
            gates_healthy: 0,
            gates_stale: 0,
            gates_failed: 0,
            summary_at: now_unix(),
        }
    }

    /// Ingest a `deploy.result` gossip event into the fleet summary.
    pub fn ingest(&mut self, result: DeployResult) {
        let gate_name = result.gate.clone();

        let health = self.gates.entry(gate_name).or_insert_with(|| {
            GateDeployHealth {
                gate: result.gate.clone(),
                latest: result.clone(),
                success_count: 0,
                failure_count: 0,
                avg_deploy_ms: result.deploy_ms,
                staleness_secs: 0,
            }
        });

        if result.success {
            health.success_count += 1;
        } else {
            health.failure_count += 1;
        }

        let total = health.success_count + health.failure_count;
        health.avg_deploy_ms =
            (health.avg_deploy_ms * (total - 1) + result.deploy_ms) / total;
        health.latest = result;

        self.recompute();
    }

    /// Recompute fleet-wide counters from per-gate state.
    pub fn recompute(&mut self) {
        let now = now_unix();
        self.summary_at = now;
        self.gates_reporting = self.gates.len();
        self.gates_healthy = 0;
        self.gates_stale = 0;
        self.gates_failed = 0;

        for health in self.gates.values_mut() {
            let age = now.saturating_sub(health.latest.timestamp);
            health.staleness_secs = age;

            if age > STALE_THRESHOLD.as_secs() {
                self.gates_stale += 1;
            } else if !health.latest.success {
                self.gates_failed += 1;
            } else {
                self.gates_healthy += 1;
            }
        }
    }

    /// Check if the fleet is fully healthy (all reporting gates have recent success).
    #[must_use]
    pub fn is_fleet_healthy(&self) -> bool {
        self.gates_reporting > 0
            && self.gates_failed == 0
            && self.gates_stale == 0
    }

    /// Fleet health as a fraction (0.0–1.0).
    #[must_use]
    pub fn health_ratio(&self) -> f64 {
        if self.gates_reporting == 0 {
            return 0.0;
        }
        self.gates_healthy as f64 / self.gates_reporting as f64
    }
}

impl Default for FleetDeployHealth {
    fn default() -> Self {
        Self::new()
    }
}

/// Query swarmVine for `deploy.result` gossip events and build fleet health.
///
/// Connects to swarmVine via standard socket discovery, queries gossip entries
/// with topic `deploy.result`, deserializes the payloads, and ingests them
/// into a `FleetDeployHealth` summary.
pub fn query_fleet_health() -> FleetDeployHealth {
    let mut fleet = FleetDeployHealth::new();

    let entries = query_deploy_result_entries();
    for entry in entries {
        fleet.ingest(entry);
    }

    fleet.recompute();
    fleet
}

/// Query swarmVine `gossip.query` for deploy.result entries.
///
/// Returns deserialized `DeployResult` events from the gossip mesh.
/// Returns an empty vec on connection failure or parse errors (graceful).
fn query_deploy_result_entries() -> Vec<DeployResult> {
    use crate::ipc::client::connect_primal;

    let mut client = match connect_primal("swarmvine") {
        Ok(c) => c,
        Err(e) => {
            tracing::debug!("swarmVine not reachable for fleet health query: {e}");
            return Vec::new();
        }
    };

    let params = serde_json::json!({
        "topic": "deploy.result",
        "key_prefix": "deploy.result:",
    });

    let response = match client.call("gossip.query", params) {
        Ok(r) => r,
        Err(e) => {
            tracing::debug!("gossip.query failed: {e}");
            return Vec::new();
        }
    };

    let Some(result) = response.result else {
        return Vec::new();
    };

    extract_deploy_results(&result)
}

/// Extract `DeployResult` payloads from a gossip.query response.
///
/// Handles two response shapes:
/// - `{"entries": [{"value": <DeployResult>}, ...]}`
/// - `[{"value": <DeployResult>}, ...]`
fn extract_deploy_results(value: &serde_json::Value) -> Vec<DeployResult> {
    let entries = value
        .get("entries")
        .and_then(|e| e.as_array())
        .or_else(|| value.as_array());

    let Some(arr) = entries else {
        if let Ok(single) = serde_json::from_value::<DeployResult>(value.clone()) {
            return vec![single];
        }
        return Vec::new();
    };

    arr.iter()
        .filter_map(|entry| {
            let payload = entry.get("value").unwrap_or(entry);
            // Try parsing string payloads (gossip may store as JSON string)
            if let Some(s) = payload.as_str() {
                serde_json::from_str::<DeployResult>(s).ok()
            } else {
                serde_json::from_value::<DeployResult>(payload.clone()).ok()
            }
        })
        .collect()
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(gate: &str, success: bool, deploy_ms: u64) -> DeployResult {
        DeployResult {
            gate: gate.to_owned(),
            composition: "nucleus".to_owned(),
            success,
            primals_alive: if success { 14 } else { 8 },
            primals_expected: 14,
            deploy_ms,
            timestamp: now_unix(),
            error: if success { None } else { Some("spawn failure".to_owned()) },
        }
    }

    #[test]
    fn empty_fleet_not_healthy() {
        let fleet = FleetDeployHealth::new();
        assert!(!fleet.is_fleet_healthy());
        assert!((fleet.health_ratio() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn single_success_is_healthy() {
        let mut fleet = FleetDeployHealth::new();
        fleet.ingest(make_result("eastGate", true, 450));
        assert!(fleet.is_fleet_healthy());
        assert_eq!(fleet.gates_reporting, 1);
        assert_eq!(fleet.gates_healthy, 1);
        assert!((fleet.health_ratio() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn failure_marks_unhealthy() {
        let mut fleet = FleetDeployHealth::new();
        fleet.ingest(make_result("eastGate", true, 450));
        fleet.ingest(make_result("ironGate", false, 120));
        assert!(!fleet.is_fleet_healthy());
        assert_eq!(fleet.gates_failed, 1);
        assert_eq!(fleet.gates_healthy, 1);
    }

    #[test]
    fn staleness_detection() {
        let mut fleet = FleetDeployHealth::new();
        let mut old_result = make_result("westGate", true, 300);
        old_result.timestamp = now_unix() - STALE_THRESHOLD.as_secs() - 100;
        fleet.ingest(old_result);
        assert!(!fleet.is_fleet_healthy());
        assert_eq!(fleet.gates_stale, 1);
    }

    #[test]
    fn average_deploy_time() {
        let mut fleet = FleetDeployHealth::new();
        fleet.ingest(make_result("eastGate", true, 400));
        fleet.ingest(make_result("eastGate", true, 600));
        let health = fleet.gates.get("eastGate").unwrap();
        assert_eq!(health.avg_deploy_ms, 500);
        assert_eq!(health.success_count, 2);
    }

    #[test]
    fn multi_gate_fleet() {
        let mut fleet = FleetDeployHealth::new();
        fleet.ingest(make_result("eastGate", true, 450));
        fleet.ingest(make_result("ironGate", true, 380));
        fleet.ingest(make_result("westGate", true, 520));
        fleet.ingest(make_result("sporeGate", true, 290));
        assert!(fleet.is_fleet_healthy());
        assert_eq!(fleet.gates_reporting, 4);
        assert_eq!(fleet.gates_healthy, 4);
    }

    #[test]
    fn extract_from_entries_array() {
        let ts = now_unix();
        let value = serde_json::json!({
            "entries": [
                {"value": {"gate":"eastGate","composition":"nucleus","success":true,
                            "primals_alive":14,"primals_expected":14,"deploy_ms":450,"timestamp":ts}},
                {"value": {"gate":"ironGate","composition":"tower","success":true,
                            "primals_alive":4,"primals_expected":4,"deploy_ms":120,"timestamp":ts}},
            ]
        });
        let results = extract_deploy_results(&value);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].gate, "eastGate");
        assert_eq!(results[1].gate, "ironGate");
    }

    #[test]
    fn extract_from_string_payloads() {
        let ts = now_unix();
        let inner = serde_json::json!({"gate":"westGate","composition":"nest","success":false,
                                       "primals_alive":6,"primals_expected":8,"deploy_ms":900,
                                       "timestamp":ts,"error":"spawn failure"});
        let value = serde_json::json!({
            "entries": [{"value": inner.to_string()}]
        });
        let results = extract_deploy_results(&value);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].gate, "westGate");
        assert!(!results[0].success);
    }

    #[test]
    fn extract_empty_on_garbage() {
        let value = serde_json::json!({"unrelated": "data"});
        let results = extract_deploy_results(&value);
        assert!(results.is_empty());
    }
}
