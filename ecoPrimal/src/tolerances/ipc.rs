// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! IPC transport timeouts and resilience parameters.
//!
//! Centralized from ipc/client.rs, ipc/transport.rs, and launcher/mod.rs.
//! Replaces inline Duration literals that risked drift.

// ── Socket timeouts ──

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

// ── Resilience parameters ──

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

// ── Provenance trio resilience ──

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
