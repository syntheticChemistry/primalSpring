// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dispatch metrics and telemetry — the raw data for adaptive routing.
//!
//! Collected per dispatch and serializable to JSON-lines for persistence
//! and training data collection (Layer 4/5 of the Neural API evolution model).
//!
//! Uses `Arc<str>` for method/owner strings to avoid per-dispatch heap
//! allocations. The routing table already interns these as `Arc<str>` in
//! `RouteEntry`, so cloning into metrics is a refcount bump.

use std::sync::Arc;

use super::RoutePath;
use crate::composition::neural_routing::CompositionTier;

/// Metrics collected per dispatch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DispatchMetric {
    /// JSON-RPC method or pattern name.
    pub method: Arc<str>,
    /// Primal that handled the dispatch.
    pub owner: Arc<str>,
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

/// Per-primal dispatch statistics.
#[derive(Debug, Clone)]
pub struct PrimalDispatchSummary {
    /// Primal identifier.
    pub primal: Arc<str>,
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
    #[expect(
        clippy::cast_precision_loss,
        reason = "metrics calculation — sub-unit precision not needed"
    )]
    pub fn avg_latency_ms(&self) -> f64 {
        if self.total_dispatches == 0 {
            return 0.0;
        }
        self.total_latency_ms as f64 / self.total_dispatches as f64
    }

    /// Success rate (0.0–1.0) for dispatches to this primal.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "metrics calculation — sub-unit precision not needed"
    )]
    pub fn success_rate(&self) -> f64 {
        if self.total_dispatches == 0 {
            return 0.0;
        }
        self.successes as f64 / self.total_dispatches as f64
    }
}

/// Metrics analysis methods for [`super::NeuralDispatcher`].
impl super::NeuralDispatcher {
    /// All collected dispatch metrics (for adaptive routing analysis).
    #[must_use]
    pub fn metrics(&self) -> &[DispatchMetric] {
        &self.metrics
    }

    /// Average latency for a specific method across all dispatches.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "metrics calculation — sub-unit precision not needed"
    )]
    pub fn avg_latency_ms(&self, method: &str) -> Option<f64> {
        let relevant: Vec<_> = self
            .metrics
            .iter()
            .filter(|m| &*m.method == method && m.success)
            .collect();
        if relevant.is_empty() {
            return None;
        }
        let total: u64 = relevant.iter().map(|m| m.latency_ms).sum();
        Some(total as f64 / relevant.len() as f64)
    }

    /// Error rate for a specific method (0.0–1.0).
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "metrics calculation — sub-unit precision not needed"
    )]
    pub fn error_rate(&self, method: &str) -> Option<f64> {
        let relevant: Vec<_> = self
            .metrics
            .iter()
            .filter(|m| &*m.method == method)
            .collect();
        if relevant.is_empty() {
            return None;
        }
        let failures = relevant.iter().filter(|m| !m.success).count();
        Some(failures as f64 / relevant.len() as f64)
    }

    /// Per-primal dispatch summary.
    #[must_use]
    pub fn primal_summary(&self) -> std::collections::HashMap<Arc<str>, PrimalDispatchSummary> {
        let mut summaries: std::collections::HashMap<Arc<str>, PrimalDispatchSummary> =
            std::collections::HashMap::new();
        for m in &self.metrics {
            let entry =
                summaries
                    .entry(Arc::clone(&m.owner))
                    .or_insert_with(|| PrimalDispatchSummary {
                        primal: Arc::clone(&m.owner),
                        total_dispatches: 0,
                        successes: 0,
                        total_latency_ms: 0,
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

    /// Flush accumulated metrics to a JSON-lines file for persistent telemetry.
    ///
    /// Appends each `DispatchMetric` as a single JSON line, enabling offline
    /// analysis and training data collection for Layer 4/5 routing evolution.
    ///
    /// Returns the number of metrics written. Clears the in-memory buffer on
    /// success so the same metrics are never written twice.
    ///
    /// # Errors
    ///
    /// Returns `std::io::Error` if the file cannot be opened or written to.
    pub fn flush_metrics_to_file(&mut self, path: &std::path::Path) -> std::io::Result<usize> {
        use std::io::Write;

        if self.metrics.is_empty() {
            return Ok(0);
        }

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        let mut writer = std::io::BufWriter::new(file);

        let count = self.metrics.len();
        for metric in self.metrics.drain(..) {
            if let Ok(line) = serde_json::to_string(&metric) {
                writeln!(writer, "{line}")?;
            }
        }

        writer.flush()?;
        tracing::debug!(
            path = %path.display(),
            metrics_flushed = count,
            "dispatch telemetry persisted"
        );
        Ok(count)
    }

    /// Default telemetry file path (under the socket dir).
    #[must_use]
    pub fn default_telemetry_path() -> std::path::PathBuf {
        let socket_dir = crate::ipc::discover::resolve_socket_dir();
        std::path::PathBuf::from(socket_dir).join("dispatch_telemetry.jsonl")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_metric(method: &str, owner: &str, latency: u64, success: bool) -> DispatchMetric {
        DispatchMetric {
            method: Arc::from(method),
            owner: Arc::from(owner),
            tier: CompositionTier::Tower,
            latency_ms: latency,
            success,
            route_path: RoutePath::CapabilityCall,
            timestamp_epoch_ms: 1_000_000,
        }
    }

    #[test]
    fn primal_dispatch_summary_avg_latency() {
        let summary = PrimalDispatchSummary {
            primal: Arc::from("beardog"),
            total_dispatches: 4,
            successes: 3,
            total_latency_ms: 400,
        };
        assert!((summary.avg_latency_ms() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn primal_dispatch_summary_zero_dispatches() {
        let summary = PrimalDispatchSummary {
            primal: Arc::from("beardog"),
            total_dispatches: 0,
            successes: 0,
            total_latency_ms: 0,
        };
        assert!((summary.avg_latency_ms() - 0.0).abs() < f64::EPSILON);
        assert!((summary.success_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn primal_dispatch_summary_success_rate() {
        let summary = PrimalDispatchSummary {
            primal: Arc::from("beardog"),
            total_dispatches: 10,
            successes: 7,
            total_latency_ms: 1000,
        };
        assert!((summary.success_rate() - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn dispatch_metric_serializes() {
        let metric = make_metric("health.check", "beardog", 42, true);
        let json = serde_json::to_string(&metric).unwrap();
        assert!(json.contains("health.check"));
        assert!(json.contains("beardog"));
    }

    #[test]
    fn dispatch_metric_round_trip() {
        let metric = make_metric("compute.submit", "toadstool", 100, false);
        let json = serde_json::to_string(&metric).unwrap();
        let back: DispatchMetric = serde_json::from_str(&json).unwrap();
        assert_eq!(&*back.method, "compute.submit");
        assert_eq!(&*back.owner, "toadstool");
        assert!(!back.success);
    }

    #[test]
    fn neural_dispatcher_metrics_start_empty() {
        let dispatcher = super::super::NeuralDispatcher::discover();
        assert!(dispatcher.metrics().is_empty());
    }

    #[test]
    fn neural_dispatcher_avg_latency_none_when_empty() {
        let dispatcher = super::super::NeuralDispatcher::discover();
        assert!(dispatcher.avg_latency_ms("nonexistent").is_none());
    }

    #[test]
    fn neural_dispatcher_error_rate_none_when_empty() {
        let dispatcher = super::super::NeuralDispatcher::discover();
        assert!(dispatcher.error_rate("nonexistent").is_none());
    }

    #[test]
    fn neural_dispatcher_primal_summary_empty() {
        let dispatcher = super::super::NeuralDispatcher::discover();
        assert!(dispatcher.primal_summary().is_empty());
    }

    #[test]
    fn neural_dispatcher_status_report_has_fields() {
        let dispatcher = super::super::NeuralDispatcher::discover();
        let report = dispatcher.status_report();
        assert!(report.get("total_methods").is_some());
        assert!(report.get("dispatches_recorded").is_some());
    }
}
