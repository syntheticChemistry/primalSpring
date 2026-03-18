// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tolerances for primalSpring validation.
//!
//! primalSpring validates coordination, not numerical accuracy. Tolerances
//! are expressed as latency bounds, count expectations, and boolean
//! conditions rather than floating-point epsilons.
//!
//! # Provenance
//!
//! These values are initial estimates pending calibration against real
//! NUCLEUS deployment measurements with live primals. Each constant
//! documents its source and justification.

/// Maximum acceptable latency for a health check round-trip (microseconds).
///
/// Source: 50ms round-trip is generous for local Unix socket IPC.
/// Calibration: pending Phase 1 measurement against live primals.
pub const HEALTH_CHECK_MAX_US: u64 = 50_000;

/// Maximum acceptable latency for capability discovery (microseconds).
///
/// Source: 100ms allows for filesystem probing + env var lookup.
/// Calibration: pending Phase 1 measurement.
pub const DISCOVERY_MAX_US: u64 = 100_000;

/// Maximum acceptable latency for a single graph node execution (microseconds).
///
/// Source: 500ms budget per node, conservative for startup-heavy primals.
/// Calibration: pending Phase 3 graph execution measurements.
pub const GRAPH_NODE_MAX_US: u64 = 500_000;

/// Maximum acceptable latency for full NUCLEUS startup (microseconds).
///
/// Source: 10 seconds for all 8+ primals to start and pass health checks.
/// Calibration: pending Phase 2 full NUCLEUS deployment.
pub const NUCLEUS_STARTUP_MAX_US: u64 = 10_000_000;

/// Maximum acceptable latency for Plasmodium formation (microseconds).
///
/// Source: 30 seconds for two NUCLEUS instances to discover and bond.
/// Calibration: pending Phase 5 bonding validation.
pub const PLASMODIUM_FORMATION_MAX_US: u64 = 30_000_000;

/// Continuous graph tick budget at 60 Hz (microseconds).
///
/// Source: 1/60 seconds = 16,667 microseconds. Hard physical constraint.
pub const TICK_BUDGET_60HZ_US: u64 = 16_667;

/// Pipeline streaming throughput floor (items per second).
///
/// Source: 100 items/sec is a conservative baseline for IPC pipelines.
/// Calibration: pending Phase 3 pipeline streaming measurements.
pub const PIPELINE_THROUGHPUT_MIN: usize = 100;

// ── IPC resilience parameters ──
//
// Absorbed from sibling spring conventions (wetSpring V127, healthSpring V37,
// groundSpring V114). These replace inline magic numbers in
// `coordination/mod.rs` and `ipc/resilience.rs`.

/// Circuit breaker failure threshold — trips after this many consecutive errors.
///
/// Source: 3 failures is standard for local IPC where latency is <50ms.
pub const CIRCUIT_BREAKER_THRESHOLD: u32 = 3;

/// Circuit breaker timeout before half-open probe (seconds).
///
/// Source: 10s gives a primal time to restart after a crash.
pub const CIRCUIT_BREAKER_TIMEOUT_SECS: u64 = 10;

/// Retry policy — maximum retry attempts before giving up.
///
/// Source: 2 retries (3 total attempts) balances latency vs resilience
/// for health check probing.
pub const RETRY_MAX_ATTEMPTS: u32 = 2;

/// Retry policy — base delay between retries (milliseconds).
///
/// Source: 50ms initial backoff for local Unix socket IPC.
pub const RETRY_BASE_DELAY_MS: u64 = 50;

/// Retry policy — maximum delay cap (milliseconds).
///
/// Source: 500ms cap prevents excessive wait in coordination validation.
pub const RETRY_MAX_DELAY_MS: u64 = 500;

/// Summary output width for validation result formatting.
///
/// Source: 72 columns matches standard terminal width conventions.
pub const VALIDATION_SUMMARY_WIDTH: usize = 72;

// ── Niche cost-estimate parameters ──
//
// Used by `niche::cost_estimates()` to provide biomeOS scheduling hints.
// Factored here so magic numbers don't appear in niche.rs JSON literals.

/// Estimated latency for `coordination.validate_composition` (ms).
pub const COST_VALIDATE_COMPOSITION_MS: u64 = 500;
/// Memory budget for `coordination.validate_composition` (bytes).
pub const COST_VALIDATE_COMPOSITION_BYTES: u64 = 4096;

/// Estimated latency for `coordination.probe_primal` (ms).
pub const COST_PROBE_PRIMAL_MS: u64 = 50;
/// Memory budget for `coordination.probe_primal` (bytes).
pub const COST_PROBE_PRIMAL_BYTES: u64 = 1024;

/// Estimated latency for `coordination.discovery_sweep` (ms).
pub const COST_DISCOVERY_SWEEP_MS: u64 = 100;
/// Memory budget for `coordination.discovery_sweep` (bytes).
pub const COST_DISCOVERY_SWEEP_BYTES: u64 = 2048;

/// Estimated latency for `composition.nucleus_health` (ms).
pub const COST_NUCLEUS_HEALTH_MS: u64 = 1000;
/// Memory budget for `composition.nucleus_health` (bytes).
pub const COST_NUCLEUS_HEALTH_BYTES: u64 = 8192;

/// Estimated latency for `graph.validate` (ms).
pub const COST_GRAPH_VALIDATE_MS: u64 = 10;
/// Memory budget for `graph.validate` (bytes).
pub const COST_GRAPH_VALIDATE_BYTES: u64 = 4096;

/// Estimated latency for `health.check` (ms).
pub const COST_HEALTH_CHECK_MS: u64 = 1;
/// Memory budget for `health.check` (bytes).
pub const COST_HEALTH_CHECK_BYTES: u64 = 256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_budget_is_correct_for_60hz() {
        let expected = 1_000_000 / 60;
        let budget = i64::from(u32::try_from(TICK_BUDGET_60HZ_US).unwrap());
        let exp = i64::from(expected);
        assert!(budget.abs_diff(exp) <= 1);
    }

    #[test]
    fn latency_tolerances_are_ordered() {
        assert!(HEALTH_CHECK_MAX_US < DISCOVERY_MAX_US);
        assert!(DISCOVERY_MAX_US < GRAPH_NODE_MAX_US);
        assert!(GRAPH_NODE_MAX_US < NUCLEUS_STARTUP_MAX_US);
        assert!(NUCLEUS_STARTUP_MAX_US < PLASMODIUM_FORMATION_MAX_US);
    }

    #[test]
    fn throughput_floor_is_positive() {
        assert!(PIPELINE_THROUGHPUT_MIN > 0);
    }

    #[test]
    fn circuit_breaker_threshold_is_reasonable() {
        assert!(CIRCUIT_BREAKER_THRESHOLD >= 2);
        assert!(CIRCUIT_BREAKER_THRESHOLD <= 10);
    }

    #[test]
    fn retry_policy_is_reasonable() {
        assert!(RETRY_MAX_ATTEMPTS >= 1);
        assert!(RETRY_BASE_DELAY_MS < RETRY_MAX_DELAY_MS);
    }

    #[test]
    fn validation_summary_width_is_standard() {
        assert!(VALIDATION_SUMMARY_WIDTH >= 60);
        assert!(VALIDATION_SUMMARY_WIDTH <= 120);
    }

    #[test]
    fn cost_estimates_are_positive() {
        assert!(COST_VALIDATE_COMPOSITION_MS > 0);
        assert!(COST_VALIDATE_COMPOSITION_BYTES > 0);
        assert!(COST_PROBE_PRIMAL_MS > 0);
        assert!(COST_PROBE_PRIMAL_BYTES > 0);
        assert!(COST_DISCOVERY_SWEEP_MS > 0);
        assert!(COST_DISCOVERY_SWEEP_BYTES > 0);
        assert!(COST_NUCLEUS_HEALTH_MS > 0);
        assert!(COST_NUCLEUS_HEALTH_BYTES > 0);
        assert!(COST_GRAPH_VALIDATE_MS > 0);
        assert!(COST_GRAPH_VALIDATE_BYTES > 0);
        assert!(COST_HEALTH_CHECK_MS > 0);
        assert!(COST_HEALTH_CHECK_BYTES > 0);
    }

    #[test]
    fn cost_latencies_are_ordered() {
        assert!(COST_HEALTH_CHECK_MS < COST_GRAPH_VALIDATE_MS);
        assert!(COST_GRAPH_VALIDATE_MS < COST_PROBE_PRIMAL_MS);
        assert!(COST_PROBE_PRIMAL_MS < COST_DISCOVERY_SWEEP_MS);
        assert!(COST_DISCOVERY_SWEEP_MS < COST_VALIDATE_COMPOSITION_MS);
        assert!(COST_VALIDATE_COMPOSITION_MS < COST_NUCLEUS_HEALTH_MS);
    }
}
