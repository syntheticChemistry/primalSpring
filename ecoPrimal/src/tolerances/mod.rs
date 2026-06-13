// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Named tolerances for primalSpring validation.
//!
//! primalSpring validates coordination, not numerical accuracy. Tolerances
//! are expressed as latency bounds, count expectations, and boolean
//! conditions rather than floating-point epsilons.
//!
//! # Provenance
//!
//! Initial values were chosen from first-principles analysis of Unix socket
//! IPC timing, then validated through 15 phases of live NUCLEUS deployment
//! testing (March 2–28, 2026). All values have proven stable across 87/87
//! gates, 63 experiments, and 689+ tests. Individual provenance notes below.
//!
//! # Module structure
//!
//! - [`latency`] — IPC and composition latency budgets (microseconds)
//! - [`parity`] — Cross-implementation floating-point parity tolerances
//! - [`ipc`] — Transport timeouts and resilience parameters
//! - [`ports`] — TCP port registry and federation assignments
//! - [`validation`] — Gate thresholds and cost estimates
//! - [`platform`] — Runtime directory resolution and target detection

pub mod ipc;
pub mod latency;
pub mod parity;
pub mod platform;
pub mod ports;
pub mod validation;

// Re-export all public items at module root for backward compatibility.
// Existing code uses `tolerances::HEALTH_CHECK_MAX_US` etc.
pub use ipc::*;
pub use latency::*;
pub use parity::*;
pub use platform::*;
pub use ports::*;
pub use validation::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_budget_is_correct_for_60hz() {
        let expected = 1_000_000 / 60;
        let budget = i64::from(u32::try_from(TICK_BUDGET_60HZ_US).unwrap());
        let exp = i64::from(expected);
        assert!(budget.abs_diff(exp) <= TICK_BUDGET_60HZ_SLACK_US);
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

    #[test]
    fn trio_resilience_params_are_reasonable() {
        assert!(TRIO_RETRY_ATTEMPTS >= 1);
        assert!(TRIO_RETRY_ATTEMPTS <= 5);
        assert!(TRIO_RETRY_BASE_DELAY_MS >= 50);
        assert!(TRIO_RETRY_BASE_DELAY_MS <= 500);
    }

    #[test]
    fn port_registry_entries_are_in_valid_range() {
        for entry in PORT_REGISTRY {
            assert!(
                entry.port >= 1024,
                "{}: port {} below unprivileged range",
                entry.slug,
                entry.port
            );
            assert!(
                entry.port <= 49151,
                "{}: port {} above registered range",
                entry.slug,
                entry.port
            );
        }
    }

    #[test]
    fn port_registry_entries_are_unique() {
        let ports: Vec<u16> = PORT_REGISTRY.iter().map(|e| e.port).collect();
        let mut sorted = ports.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            ports.len(),
            sorted.len(),
            "PORT_REGISTRY ports must be unique"
        );
    }

    #[test]
    fn ipc_socket_timeout_is_reasonable() {
        assert!(IPC_SOCKET_TIMEOUT_SECS >= 1);
        assert!(IPC_SOCKET_TIMEOUT_SECS <= 30);
    }

    #[test]
    fn launcher_timeouts_are_reasonable() {
        assert!(LAUNCHER_SOCKET_TIMEOUT_SECS >= 10);
        assert!(LAUNCHER_SOCKET_TIMEOUT_SECS <= 120);
        assert!(LAUNCHER_POLL_INTERVAL_MS >= 10);
        assert!(LAUNCHER_POLL_INTERVAL_MS <= 1000);
        assert!(LAUNCHER_SOCKET_SETTLE_MS >= 10);
        assert!(LAUNCHER_SOCKET_SETTLE_MS <= 500);
    }

    #[test]
    fn tick_slack_is_minimal() {
        assert!(TICK_BUDGET_60HZ_SLACK_US <= 5);
    }

    #[test]
    fn composition_parity_tolerances_are_ordered() {
        assert!(EXACT_PARITY_TOL < DETERMINISTIC_FLOAT_TOL);
        assert!(DETERMINISTIC_FLOAT_TOL < DF64_PARITY_TOL);
        assert!(DF64_PARITY_TOL < CPU_GPU_PARITY_TOL);
        assert!(CPU_GPU_PARITY_TOL <= IPC_ROUND_TRIP_TOL);
        assert!(IPC_ROUND_TRIP_TOL < WGSL_SHADER_TOL);
        assert!(WGSL_SHADER_TOL <= STOCHASTIC_SEED_TOL);
    }

    #[test]
    fn composition_parity_tolerances_are_non_negative() {
        assert!(EXACT_PARITY_TOL >= 0.0);
        assert!(DETERMINISTIC_FLOAT_TOL >= 0.0);
        assert!(DF64_PARITY_TOL >= 0.0);
        assert!(CPU_GPU_PARITY_TOL >= 0.0);
        assert!(IPC_ROUND_TRIP_TOL >= 0.0);
        assert!(WGSL_SHADER_TOL >= 0.0);
        assert!(STOCHASTIC_SEED_TOL >= 0.0);
    }
}
