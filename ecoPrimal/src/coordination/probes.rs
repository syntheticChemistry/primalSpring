// SPDX-License-Identifier: AGPL-3.0-or-later

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
use crate::ipc::discover::discover_primal;
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

/// Probe a single primal: discover socket, connect, health check,
/// list capabilities.
///
/// Single-attempt connection (no retry) followed by a health check and
/// capability enumeration. For resilient probing with circuit breaker
/// and exponential backoff, use [`health_check`] instead.
///
/// Returns a [`PrimalHealth`] with whatever information could be gathered.
/// Gracefully degrades: socket not found → `health_ok: false`.
#[must_use]
pub fn probe_primal(name: &str) -> PrimalHealth {
    let discovery = discover_primal(name);
    let Some(socket) = discovery.socket else {
        return PrimalHealth {
            name: name.to_owned(),
            socket_found: false,
            health_ok: false,
            capabilities: Vec::new(),
            latency_us: 0,
        };
    };

    let start = Instant::now();
    let Ok(mut client) = PrimalClient::connect(&socket, name) else {
        return PrimalHealth {
            name: name.to_owned(),
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
        name: name.to_owned(),
        socket_found: true,
        health_ok,
        capabilities,
        latency_us,
    }
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

/// Probe a primal's health and record check results on a [`crate::validation::ValidationResult`].
///
/// Reduces boilerplate across experiments: each primal probe produces 3 checks
/// (`health_{name}`, `latency_{name}`, `caps_{name}`), either as PASS/FAIL
/// when the primal is reachable or SKIP when it is not.
pub fn check_primal_health(v: &mut crate::validation::ValidationResult, primal: &str) {
    let health = probe_primal(primal);
    if health.socket_found {
        v.check_bool(
            &format!("health_{primal}"),
            health.health_ok,
            &format!("{primal} health.check"),
        );
        v.check_latency(
            &format!("latency_{primal}"),
            health.latency_us,
            tolerances::HEALTH_CHECK_MAX_US,
        );
        v.check_minimum(&format!("caps_{primal}"), health.capabilities.len(), 1);
    } else {
        v.check_skip(
            &format!("health_{primal}"),
            &format!("{primal} not reachable"),
        );
        v.check_skip(
            &format!("latency_{primal}"),
            &format!("{primal} not reachable"),
        );
        v.check_skip(
            &format!("caps_{primal}"),
            &format!("{primal} not reachable"),
        );
    }
}

/// Probe a capability provider's health and record check results on a
/// [`crate::validation::ValidationResult`].
///
/// Capability-based analog of [`check_primal_health`]: discovers whatever
/// primal provides the given capability at runtime, then records health,
/// latency, and capabilities checks. Never hardcodes primal names.
pub fn check_capability_health(v: &mut crate::validation::ValidationResult, capability: &str) {
    let disc = crate::ipc::discover::discover_by_capability(capability);
    let provider = disc.resolved_primal.as_deref().unwrap_or("unresolved");

    if let Some(ref socket) = disc.socket {
        let start = std::time::Instant::now();
        let (health_ok, caps) = crate::ipc::client::PrimalClient::connect(socket, provider)
            .map_or_else(
                |_| (false, Vec::new()),
                |mut c| {
                    let caps = extract_capability_names(c.capabilities().ok());
                    let h = !caps.is_empty() || c.health_check().unwrap_or(false);
                    (h, caps)
                },
            );
        let latency_us = crate::cast::micros_u64(start.elapsed());

        v.check_bool(
            &format!("health_{capability}"),
            health_ok,
            &format!("{capability} provider ({provider}) health.check"),
        );
        v.check_latency(
            &format!("latency_{capability}"),
            latency_us,
            tolerances::HEALTH_CHECK_MAX_US,
        );
        v.check_minimum(&format!("caps_{capability}"), caps.len(), 1);
    } else {
        v.check_skip(
            &format!("health_{capability}"),
            &format!("{capability} provider not discovered"),
        );
        v.check_skip(
            &format!("latency_{capability}"),
            &format!("{capability} provider not discovered"),
        );
        v.check_skip(
            &format!("caps_{capability}"),
            &format!("{capability} provider not discovered"),
        );
    }
}

pub fn extract_capability_names(caps: Option<serde_json::Value>) -> Vec<String> {
    crate::ipc::discover::extract_capability_names(caps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_primal_graceful_when_not_running() {
        let health = probe_primal("nonexistent_primal_xyzzy");
        assert!(!health.socket_found);
        assert!(!health.health_ok);
        assert!(health.capabilities.is_empty());
    }

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
    fn primal_health_socket_not_found_has_zero_latency() {
        let health = probe_primal("nonexistent_xyzzy_zero_latency");
        assert_eq!(health.latency_us, 0);
        assert_eq!(health.name, "nonexistent_xyzzy_zero_latency");
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

    #[test]
    fn check_primal_health_graceful_when_not_running() {
        use crate::validation::{NullSink, ValidationResult};
        use std::sync::Arc;
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        check_primal_health(&mut v, "nonexistent_primal_xyzzy_test_12345");
        assert_eq!(v.skipped, 3, "should skip health, latency, caps");
        assert_eq!(v.failed, 0);
    }

    #[test]
    fn check_capability_health_graceful_when_not_running() {
        use crate::validation::{NullSink, ValidationResult};
        use std::sync::Arc;
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        check_capability_health(&mut v, "nonexistent_capability_xyzzy_12345");
        assert_eq!(v.skipped, 3, "should skip health, latency, caps");
        assert_eq!(v.failed, 0);
    }
}
