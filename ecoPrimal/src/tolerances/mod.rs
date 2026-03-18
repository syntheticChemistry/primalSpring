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
}
