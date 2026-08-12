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
}
