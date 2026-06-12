// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Latency budget constants for IPC and composition operations.
//!
//! primalSpring validates coordination timing, not numerical accuracy.
//! Tolerances are expressed as latency bounds derived from first-principles
//! analysis of Unix socket IPC timing, validated through 15 phases of live
//! NUCLEUS deployment testing (March 2–28, 2026).

/// Maximum acceptable latency for a health check round-trip (microseconds).
///
/// Source: 50ms round-trip is generous for local Unix socket IPC.
/// Validated: Phase 4+ live Tower probes consistently complete in <10ms.
pub const HEALTH_CHECK_MAX_US: u64 = 50_000;

/// Maximum acceptable latency for capability discovery (microseconds).
///
/// Source: 100ms allows for filesystem probing + env var lookup.
/// Validated: Phase 3+ 5-tier discovery completes in <30ms on local gate.
pub const DISCOVERY_MAX_US: u64 = 100_000;

/// Upper bound for neural API / primal routing round-trip latency checks (microseconds).
///
/// Same budget as [`DISCOVERY_MAX_US`]; used by experiments validating capability
/// routing through biomeOS (e.g. exp087).
pub const PRIMAL_STARTUP_LATENCY_US: u64 = DISCOVERY_MAX_US;

/// Maximum acceptable latency for a single graph node execution (microseconds).
///
/// Source: 500ms budget per node, conservative for startup-heavy primals.
/// Validated: Phase 9 live graph execution — sequential/parallel/DAG all
/// complete individual nodes well within budget.
pub const GRAPH_NODE_MAX_US: u64 = 500_000;

/// Maximum acceptable latency for full NUCLEUS startup (microseconds).
///
/// Source: 10 seconds for all 8+ primals to start and pass health checks.
/// Validated: Phase 6 NUCLEUS composition (Tower+Nest+Node) starts within
/// ~3–5 seconds on dev hardware; 10s budget provides margin for slower gates.
pub const NUCLEUS_STARTUP_MAX_US: u64 = 10_000_000;

/// Maximum acceptable latency for Plasmodium formation (microseconds).
///
/// Source: 30 seconds for two NUCLEUS instances to discover and bond.
/// Validated: Phase 12 bonding structural tests pass; live multi-gate
/// measurement pending Phase 17 LAN deployment.
pub const PLASMODIUM_FORMATION_MAX_US: u64 = 30_000_000;

/// Continuous graph tick budget at 60 Hz (microseconds).
///
/// Source: 1/60 seconds = 16,667 microseconds. Hard physical constraint.
pub const TICK_BUDGET_60HZ_US: u64 = 16_667;

/// Acceptable jitter for 60 Hz tick timing assertions (microseconds).
///
/// Source: integer division of `1_000_000/60` drops the fractional part;
/// 1 µs slack covers the rounding. Used by exp014 and exp023.
pub const TICK_BUDGET_60HZ_SLACK_US: u64 = 1;

/// Pipeline streaming throughput floor (items per second).
///
/// Source: 100 items/sec is a conservative baseline for IPC pipelines.
/// Validated: Phase 9 pipeline pattern structural checks pass.
pub const PIPELINE_THROUGHPUT_MIN: usize = 100;
