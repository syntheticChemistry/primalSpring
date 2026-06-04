// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Primal and substrate probing — single-shot and resilient health checks.
//!
//! Separated from the atomic composition types in `coordination/mod.rs` so
//! that probing logic (socket discovery, health checks, capability extraction,
//! circuit breakers) lives in its own module.

use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cast;
use crate::ipc::IpcError;
use crate::ipc::client::{self, PrimalClient};
use crate::tolerances;

/// Health status of a single primal after probing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealth {
    /// Primal name (e.g. `"beardog"`).
    pub name: String,
    /// Whether a socket file was found for this primal.
    pub socket_found: bool,
    /// Whether the primal responded to `health.check`.
    pub health_ok: bool,
    /// Capability names reported by `capabilities.list`.
    pub capabilities: Vec<String>,
    /// Round-trip latency of the health check in microseconds.
    pub latency_us: u64,
}

/// Health status of the biomeOS Neural API substrate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateHealth {
    /// Whether the Neural API socket was discovered.
    pub socket_found: bool,
    /// Whether the Neural API responded to health check.
    pub health_ok: bool,
    /// Socket path used.
    pub socket_path: Option<String>,
    /// Round-trip latency of the health check in microseconds.
    pub latency_us: u64,
}

/// Probe a primal at a known socket path (no discovery step).
///
/// Used when capability-based discovery resolved a socket but not a primal
/// name — e.g. a capability-named socket like `security.sock` in Tier 2.
#[must_use]
pub fn probe_primal_at_socket(label: &str, socket: &std::path::Path) -> PrimalHealth {
    let start = Instant::now();
    let Ok(mut client) = PrimalClient::connect(socket, label) else {
        return PrimalHealth {
            name: label.to_owned(),
            socket_found: true,
            health_ok: false,
            capabilities: Vec::new(),
            latency_us: cast::micros_u64(start.elapsed()),
        };
    };

    let health_ok = client.health_check().unwrap_or(false);
    let capabilities = extract_capability_names(client.capabilities().ok());
    let latency_us = cast::micros_u64(start.elapsed());

    PrimalHealth {
        name: label.to_owned(),
        socket_found: true,
        health_ok,
        capabilities,
        latency_us,
    }
}

/// Probe the biomeOS Neural API substrate.
///
/// Attempts to discover and health-check the Neural API. Returns `None` if
/// biomeOS is not discoverable (no socket found), or `Some` with health status.
#[must_use]
pub fn probe_substrate() -> Option<SubstrateHealth> {
    let bridge = crate::ipc::neural_bridge::NeuralBridge::discover()?;
    let socket_path = Some(bridge.socket_path().to_string_lossy().into_owned());
    let start = Instant::now();
    let health_ok = bridge.health_check().unwrap_or(false);
    let latency_us = cast::micros_u64(start.elapsed());
    Some(SubstrateHealth {
        socket_found: true,
        health_ok,
        socket_path,
        latency_us,
    })
}

/// Try to connect to a primal and perform a health check.
///
/// Uses [`crate::ipc::resilience::resilient_call`] with a circuit breaker and retry policy to
/// handle transient IPC failures gracefully. Returns `Ok(latency_us)`
/// if the primal responds to `health.check`.
///
/// # Errors
///
/// Returns [`IpcError`] if the primal socket is unreachable, the circuit
/// is open, or the health check call fails after retries.
pub fn health_check(primal: &str) -> Result<u64, IpcError> {
    use crate::ipc::resilience::{CircuitBreaker, RetryPolicy, resilient_call};
    use std::time::Duration;

    let mut cb = CircuitBreaker::new(
        tolerances::CIRCUIT_BREAKER_THRESHOLD,
        Duration::from_secs(tolerances::CIRCUIT_BREAKER_TIMEOUT_SECS),
    );
    let policy = RetryPolicy::new(
        tolerances::RETRY_MAX_ATTEMPTS,
        Duration::from_millis(tolerances::RETRY_BASE_DELAY_MS),
        Duration::from_millis(tolerances::RETRY_MAX_DELAY_MS),
    );
    resilient_call(&mut cb, &policy, || {
        let mut c = client::connect_primal(primal)?;
        let start = Instant::now();
        c.health_check()?;
        Ok(cast::micros_u64(start.elapsed()))
    })
}

/// Check whether a primal's health check latency is within tolerance.
#[must_use]
pub fn health_check_within_tolerance(primal: &str) -> Option<bool> {
    health_check(primal)
        .ok()
        .map(|us| us <= tolerances::HEALTH_CHECK_MAX_US)
}

pub fn extract_capability_names(caps: Option<serde_json::Value>) -> Vec<String> {
    crate::ipc::discover::extract_capability_names(caps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_substrate_returns_none_when_biomeos_not_running() {
        assert!(probe_substrate().is_none());
    }

    #[test]
    fn health_check_fails_when_not_running() {
        assert!(health_check("nonexistent_primal_xyzzy").is_err());
    }

    #[test]
    fn health_check_within_tolerance_returns_none_when_unreachable() {
        assert!(health_check_within_tolerance("nonexistent_primal_xyzzy").is_none());
    }

    #[test]
    fn extract_capability_names_from_array() {
        let val = serde_json::json!(["crypto.sign", "crypto.verify"]);
        let names = extract_capability_names(Some(val));
        assert_eq!(names, vec!["crypto.sign", "crypto.verify"]);
    }

    #[test]
    fn extract_capability_names_from_object() {
        let val = serde_json::json!({"crypto": {}, "storage": {}});
        let names = extract_capability_names(Some(val));
        assert!(names.contains(&"crypto".to_owned()));
        assert!(names.contains(&"storage".to_owned()));
    }

    #[test]
    fn extract_capability_names_from_none() {
        assert!(extract_capability_names(None).is_empty());
    }
}
