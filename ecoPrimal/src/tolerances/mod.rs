// SPDX-License-Identifier: AGPL-3.0-or-later

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
//! gates, 63 experiments, and 403 tests. Individual provenance notes below.

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

// ── Composition parity tolerances ──
//
// Used by springs to validate that primal composition output matches
// local Rust math (which was already validated against Python baselines).
// Springs reference these by name so tolerances are documented in one place.
//
// Ordering: EXACT < DETERMINISTIC_FLOAT < DF64_PARITY < CPU_GPU_PARITY
//           <= IPC_ROUND_TRIP < WGSL_SHADER <= STOCHASTIC_SEED

/// Exact integer parity — zero tolerance.
///
/// For deterministic integer math where composition must produce bit-identical
/// results (e.g. hash computations, index calculations).
pub const EXACT_PARITY_TOL: f64 = 0.0;

/// Deterministic floating-point parity.
///
/// For pure CPU f64 operations where the only divergence is instruction
/// ordering. IEEE 754 guarantees identical results for identical operation
/// sequences; this covers minor reordering.
pub const DETERMINISTIC_FLOAT_TOL: f64 = 1e-15;

/// Double-float (df64) emulated precision parity.
///
/// barraCuda's df64 path uses Dekker/Knuth error-free transforms to achieve
/// ~30 digits of precision. Tolerance covers the ~1 ULP error inherent in
/// the emulation scheme.
pub const DF64_PARITY_TOL: f64 = 1e-14;

/// CPU vs GPU floating-point parity.
///
/// GPU shader execution may reorder FMA, use different rounding modes,
/// or flush denormals. This tolerance covers the expected divergence
/// between a CPU f64 path and a GPU WGSL f32→f64 promotion path.
pub const CPU_GPU_PARITY_TOL: f64 = 1e-10;

/// IPC round-trip serialization parity.
///
/// JSON serialization of f64 values may lose the least significant bits
/// depending on the serializer's precision setting. `serde_json` uses
/// sufficient precision for f64 round-trips, but this tolerance covers
/// edge cases in intermediate Value representations.
pub const IPC_ROUND_TRIP_TOL: f64 = 1e-10;

/// WGSL shader computation parity.
///
/// WGSL shaders execute in f32 by default. When comparing f32 shader output
/// against f64 Rust baselines, this tolerance covers the expected precision
/// loss from the narrower mantissa (23 vs 52 bits).
pub const WGSL_SHADER_TOL: f64 = 1e-6;

/// Stochastic algorithm parity (seed-fixed).
///
/// For algorithms with pseudorandom components (Monte Carlo, HMC, etc.)
/// where the seed is fixed. Different implementations of the same PRNG
/// may diverge after many iterations due to floating-point accumulation.
pub const STOCHASTIC_SEED_TOL: f64 = 1e-6;

// ── IPC transport timeouts ──
//
// Centralized from ipc/client.rs, ipc/transport.rs, and launcher/mod.rs.
// Replaces inline Duration literals that risked drift.

/// Default timeout for IPC socket read/write operations.
///
/// Source: 5 seconds is generous for local Unix socket IPC.
/// Validated: Phase 4+ live Tower calls consistently complete in <50ms.
/// Used by: `ipc::client::PrimalClient`, `ipc::transport::Transport`.
pub const IPC_SOCKET_TIMEOUT_SECS: u64 = 5;

/// Maximum time for the BTSP handshake phase (relay primals call BearDog).
///
/// Source: Relay primals (barraCuda, coralReef, NestGate) forward the BTSP
/// handshake to BearDog via JSON-RPC, adding a round-trip. 15 seconds
/// allows for contention when many primals bootstrap simultaneously.
/// After the handshake, the socket reverts to `IPC_SOCKET_TIMEOUT_SECS`.
pub const BTSP_HANDSHAKE_TIMEOUT_SECS: u64 = 15;

/// Maximum time to wait for a primal's socket file to appear after spawn.
///
/// Source: 30 seconds covers slow-starting primals (model loading, etc.).
/// Validated: Phase 6 NUCLEUS primals appear within ~2–5 seconds; 30s
/// provides generous margin for resource-constrained gates.
/// Used by: `launcher::spawn_primal`, `launcher::spawn_biomeos`.
pub const LAUNCHER_SOCKET_TIMEOUT_SECS: u64 = 30;

/// Polling interval for socket readiness checks (milliseconds).
///
/// Source: 100ms gives responsive detection without busy-wait overhead.
pub const LAUNCHER_POLL_INTERVAL_MS: u64 = 100;

/// Settle delay after socket appears before declaring ready (milliseconds).
///
/// Source: 50ms allows the primal's listener to fully bind after the
/// socket file is created.
pub const LAUNCHER_SOCKET_SETTLE_MS: u64 = 50;

// ── IPC resilience parameters ──
//
// Absorbed from sibling spring conventions (wetSpring V127, healthSpring V37,
// groundSpring V114). These replace inline magic numbers in
// `coordination/mod.rs`, `ipc/resilience.rs`, and `ipc/provenance.rs`.

/// Circuit breaker failure threshold — trips after this many consecutive errors.
///
/// Source: 3 failures is standard for local IPC where latency is <50ms.
/// Used by: coordination health checks, provenance trio circuit.
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

// ── Provenance trio resilience ──
//
// Absorbed from healthSpring V41 epoch-based circuit breaker pattern.
// Used by `ipc/provenance.rs` for resilient trio capability calls.

/// Provenance trio retry attempts (per capability call).
///
/// Source: 2 retries (3 total) balances latency vs reliability for trio calls
/// that traverse Neural API → primal → backend.
pub const TRIO_RETRY_ATTEMPTS: u32 = 2;

/// Provenance trio retry base delay (milliseconds).
///
/// Source: 100ms base with exponential backoff covers transient trio latency
/// spikes during session creation or DAG dehydration.
pub const TRIO_RETRY_BASE_DELAY_MS: u64 = 100;

// ── TCP cross-gate transport timeouts ──
//
// Used by `ipc::tcp` helpers for cross-gate probing experiments.
// On the same machine, Unix socket timeouts in IPC_SOCKET_TIMEOUT_SECS apply.

/// TCP connect timeout for remote gate probing (seconds).
///
/// Source: 5 seconds is generous for LAN/WAN TCP connect.
/// Validated: Phase 15 cross-gate experiments connect within <2s on LAN.
pub const TCP_CONNECT_TIMEOUT_SECS: u64 = 5;

/// TCP read timeout for remote gate probing (seconds).
///
/// Source: 10 seconds covers slow primals and high-latency WAN links.
pub const TCP_READ_TIMEOUT_SECS: u64 = 10;

/// TCP write timeout for remote gate probing (seconds).
///
/// Source: 5 seconds matches connect timeout for symmetric behavior.
pub const TCP_WRITE_TIMEOUT_SECS: u64 = 5;

// ── Discovery Tier 5: TCP fallback ports ──
//
// Part of the discovery escalation hierarchy:
//
//   Tier 1 — Songbird routing (full NUCLEUS, cross-gate, transport-agnostic)
//   Tier 2 — biomeOS Neural API (capability.discover)
//   Tier 3 — UDS filesystem convention (primal-family.sock)
//   Tier 4 — Socket registry / primal manifests
//   Tier 5 — TCP probing on well-known ports (THIS SECTION)
//
// Every tier is a valid deployment model. Tier 5 serves containers,
// architectures without UDS, shell scripts, and standalone compositions
// that choose not to run Tower. The system doesn't ask why.
//
// Port assignments confirmed against ironGate live deployment (2026-05-04).
// Canonical source: plasmidBin/ports.env

/// TCP fallback port for remote `BearDog` (security).
pub const TCP_FALLBACK_BEARDOG_PORT: u16 = 9100;
/// TCP fallback port for remote Songbird (discovery/mesh).
pub const TCP_FALLBACK_SONGBIRD_PORT: u16 = 9200;
/// TCP fallback port for remote Squirrel (AI).
pub const TCP_FALLBACK_SQUIRREL_PORT: u16 = 9300;
/// Default `SQUIRREL_PORT` when unset (same as [`TCP_FALLBACK_SQUIRREL_PORT`]).
pub const DEFAULT_SQUIRREL_PORT: u16 = TCP_FALLBACK_SQUIRREL_PORT;
/// TCP fallback port for remote `ToadStool` (compute).
pub const TCP_FALLBACK_TOADSTOOL_PORT: u16 = 9400;
/// TCP fallback port for remote `NestGate` (storage).
/// Confirmed live on ironGate at 9500 (2026-05-04).
pub const TCP_FALLBACK_NESTGATE_PORT: u16 = 9500;
/// TCP fallback port for remote rhizoCrypt JSON-RPC (ephemeral DAG).
/// tarpc on 9600, JSON-RPC on 9601; this is the JSON-RPC port.
pub const TCP_FALLBACK_RHIZOCRYPT_PORT: u16 = 9601;
/// TCP fallback port for remote loamSpine JSON-RPC (permanent ledger).
/// Confirmed live on ironGate at 9700 (2026-05-04).
pub const TCP_FALLBACK_LOAMSPINE_PORT: u16 = 9700;
/// TCP fallback port for remote coralReef (shader compilation).
/// Confirmed live on ironGate at 9730 (2026-05-04).
pub const TCP_FALLBACK_CORALREEF_PORT: u16 = 9730;
/// TCP fallback port for remote barraCuda (GPU compute).
/// Confirmed live on ironGate at 9740 (2026-05-04).
pub const TCP_FALLBACK_BARRACUDA_PORT: u16 = 9740;
/// TCP fallback port for remote skunkBat (defense/recon).
pub const TCP_FALLBACK_SKUNKBAT_PORT: u16 = 9140;
/// TCP fallback port for remote biomeOS (substrate).
pub const TCP_FALLBACK_BIOMEOS_PORT: u16 = 9800;
/// TCP fallback port for remote sweetGrass (attribution braids).
/// Canonical BTSP TCP port is 9850. The legacy HTTP endpoint (39085) is
/// deprecated; downstream consumers should use 9850 for all JSON-RPC.
pub const TCP_FALLBACK_SWEETGRASS_PORT: u16 = 9850;
/// TCP fallback port for remote `petalTongue` (visualization).
pub const TCP_FALLBACK_PETALTONGUE_PORT: u16 = 9900;

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
    fn tcp_fallback_ports_are_in_valid_range() {
        for port in [
            TCP_FALLBACK_BEARDOG_PORT,
            TCP_FALLBACK_SONGBIRD_PORT,
            TCP_FALLBACK_SQUIRREL_PORT,
            TCP_FALLBACK_TOADSTOOL_PORT,
            TCP_FALLBACK_NESTGATE_PORT,
            TCP_FALLBACK_RHIZOCRYPT_PORT,
            TCP_FALLBACK_LOAMSPINE_PORT,
            TCP_FALLBACK_CORALREEF_PORT,
            TCP_FALLBACK_BARRACUDA_PORT,
            TCP_FALLBACK_SKUNKBAT_PORT,
            TCP_FALLBACK_BIOMEOS_PORT,
            TCP_FALLBACK_SWEETGRASS_PORT,
            TCP_FALLBACK_PETALTONGUE_PORT,
        ] {
            assert!(port >= 1024, "port {port} below unprivileged range");
            assert!(port <= 49151, "port {port} above registered range");
        }
    }

    #[test]
    fn tcp_fallback_ports_are_unique() {
        let ports = [
            TCP_FALLBACK_BEARDOG_PORT,
            TCP_FALLBACK_SONGBIRD_PORT,
            TCP_FALLBACK_SQUIRREL_PORT,
            TCP_FALLBACK_TOADSTOOL_PORT,
            TCP_FALLBACK_NESTGATE_PORT,
            TCP_FALLBACK_RHIZOCRYPT_PORT,
            TCP_FALLBACK_LOAMSPINE_PORT,
            TCP_FALLBACK_CORALREEF_PORT,
            TCP_FALLBACK_BARRACUDA_PORT,
            TCP_FALLBACK_SKUNKBAT_PORT,
            TCP_FALLBACK_BIOMEOS_PORT,
            TCP_FALLBACK_SWEETGRASS_PORT,
            TCP_FALLBACK_PETALTONGUE_PORT,
        ];
        let mut sorted = ports.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            ports.len(),
            sorted.len(),
            "TCP fallback ports must be unique"
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
