// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation gate thresholds and niche cost-estimate parameters.
//!
//! Minimum compliance rates and latency ceilings for live NUCLEUS validation
//! scenarios. Centralized here so scenarios reference named constants rather
//! than inline magic numbers.

// ── Validation gate thresholds ──

/// Minimum health compliance rate across probed primals (percentage).
///
/// Source: 80% allows 2-3 primals to be unreachable during mesh formation.
/// Used by: `s_health_standard`, `s_tower_cns`.
pub const HEALTH_COMPLIANCE_MIN_PCT: f64 = 80.0;

/// Minimum entity resolution rate for composition dispatch parity (percentage).
///
/// Source: 90% accommodates primals that haven't registered capabilities yet.
/// Used by: `s_sporeprint_pure_primal`.
pub const ENTITY_RESOLUTION_MIN_PCT: f64 = 90.0;

/// Maximum acceptable average latency for a single IPC method call (ms).
///
/// Source: 500ms generous ceiling for primal dispatch round-trip.
/// Used by: `s_feedback_loop` latency assertions.
pub const IPC_METHOD_AVG_LATENCY_MAX_MS: f64 = 500.0;

/// Maximum acceptable error rate for a single IPC method (fraction 0.0–1.0).
///
/// Source: 50% threshold — anything higher indicates a broken path.
/// Used by: `s_feedback_loop` error rate assertions.
pub const IPC_METHOD_ERROR_RATE_MAX: f64 = 0.5;

/// Maximum acceptable dispatch latency for scenario validation (ms).
///
/// Source: 1000ms generous ceiling for neural dispatch round-trip (includes
/// biomeOS routing + primal execution + response serialization).
/// Used by: `s_neural_dispatch_live` latency checks.
pub const SCENARIO_DISPATCH_LATENCY_MAX_MS: u64 = 1000;

/// Summary output width for validation result formatting.
///
/// Source: 72 columns matches standard terminal width conventions.
pub const VALIDATION_SUMMARY_WIDTH: usize = 72;

// ── Niche cost-estimate parameters ──
//
// Scheduling hints for biomeOS — factored here so magic numbers don't
// appear in caller code.

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
