// SPDX-License-Identifier: AGPL-3.0-or-later

//! Atomic composition logic — Tower, Node, Nest, Full NUCLEUS.
//!
//! Each atomic layer is a testable deployment target. primalSpring deploys
//! them via biomeOS graphs and validates that every primal starts, discovers
//! peers, and responds to capability calls.

use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cast;
use crate::ipc::IpcError;
use crate::ipc::client::{self, PrimalClient};
use crate::ipc::discover::{discover_for, discover_primal};
use crate::tolerances;

/// Atomic composition layer — each represents a testable deployment target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicType {
    /// BearDog + Songbird (crypto + mesh). Minimal NUCLEUS composition.
    Tower,
    /// Tower + ToadStool (adds compute).
    Node,
    /// Tower + NestGate (adds storage).
    Nest,
    /// All 8 primals including Squirrel and provenance trio.
    FullNucleus,
}

impl AtomicType {
    /// Primal names required for this composition.
    #[must_use]
    pub const fn required_primals(self) -> &'static [&'static str] {
        match self {
            Self::Tower => &["beardog", "songbird"],
            Self::Node => &["beardog", "songbird", "toadstool"],
            Self::Nest => &["beardog", "songbird", "nestgate"],
            Self::FullNucleus => &[
                "beardog",
                "songbird",
                "toadstool",
                "nestgate",
                "squirrel",
                "rhizocrypt",
                "loamspine",
                "sweetgrass",
            ],
        }
    }

    /// biomeOS deploy graph name for this composition.
    #[must_use]
    pub const fn graph_name(self) -> &'static str {
        match self {
            Self::Tower => "tower_atomic_bootstrap",
            Self::Node => "node_atomic_compute",
            Self::Nest => "nest_deploy",
            Self::FullNucleus => "nucleus_complete",
        }
    }

    /// Human-readable description of this composition.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Tower => "BearDog + Songbird (crypto + mesh)",
            Self::Node => "Tower + ToadStool (+ compute)",
            Self::Nest => "Tower + NestGate (+ storage)",
            Self::FullNucleus => "All primals + Squirrel (full composition)",
        }
    }
}

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

/// Result of validating an entire atomic composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionResult {
    /// Which composition was validated.
    pub atomic: AtomicType,
    /// Health status of each required primal.
    pub primals: Vec<PrimalHealth>,
    /// `true` if every primal passed its health check.
    pub all_healthy: bool,
    /// `true` if every primal's socket was discovered.
    pub discovery_ok: bool,
    /// Sum of capabilities across all primals.
    pub total_capabilities: usize,
}

/// Probe a single primal: discover socket, connect with retry, health check,
/// list capabilities.
///
/// Uses [`RetryPolicy::quick`] for transient connection failures.
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

/// Validate an entire atomic composition by probing all its required primals.
#[must_use]
pub fn validate_composition(atomic: AtomicType) -> CompositionResult {
    let required = atomic.required_primals();
    let discovery = discover_for(required);
    let discovery_ok = discovery.iter().all(|d| d.socket.is_some());

    let primals: Vec<PrimalHealth> = required.iter().map(|name| probe_primal(name)).collect();

    let all_healthy = primals.iter().all(|p| p.health_ok);
    let total_capabilities: usize = primals.iter().map(|p| p.capabilities.len()).sum();

    CompositionResult {
        atomic,
        primals,
        all_healthy,
        discovery_ok,
        total_capabilities,
    }
}

/// Try to connect to a primal and perform a health check.
///
/// Uses [`resilient_call`] with a circuit breaker and retry policy to
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

    let mut cb = CircuitBreaker::new(3, Duration::from_secs(10));
    let policy = RetryPolicy::new(2, Duration::from_millis(50), Duration::from_millis(500));
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

fn extract_capability_names(caps: Option<serde_json::Value>) -> Vec<String> {
    let Some(val) = caps else {
        return Vec::new();
    };
    match val {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        serde_json::Value::Object(map) => map.keys().cloned().collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_requires_two_primals() {
        assert_eq!(AtomicType::Tower.required_primals().len(), 2);
        assert!(AtomicType::Tower.required_primals().contains(&"beardog"));
        assert!(AtomicType::Tower.required_primals().contains(&"songbird"));
    }

    #[test]
    fn node_extends_tower_with_toadstool() {
        let primals = AtomicType::Node.required_primals();
        assert!(primals.contains(&"beardog"));
        assert!(primals.contains(&"songbird"));
        assert!(primals.contains(&"toadstool"));
    }

    #[test]
    fn nest_extends_tower_with_nestgate() {
        let primals = AtomicType::Nest.required_primals();
        assert!(primals.contains(&"nestgate"));
    }

    #[test]
    fn full_nucleus_requires_eight_primals() {
        assert_eq!(AtomicType::FullNucleus.required_primals().len(), 8);
    }

    #[test]
    fn all_types_have_graph_names() {
        let types = [
            AtomicType::Tower,
            AtomicType::Node,
            AtomicType::Nest,
            AtomicType::FullNucleus,
        ];
        for t in types {
            assert!(!t.graph_name().is_empty());
        }
    }

    #[test]
    fn all_types_have_descriptions() {
        let types = [
            AtomicType::Tower,
            AtomicType::Node,
            AtomicType::Nest,
            AtomicType::FullNucleus,
        ];
        for t in types {
            assert!(!t.description().is_empty());
        }
    }

    #[test]
    fn atomic_type_round_trip_json() {
        for t in [
            AtomicType::Tower,
            AtomicType::Node,
            AtomicType::Nest,
            AtomicType::FullNucleus,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let back: AtomicType = serde_json::from_str(&json).unwrap();
            assert_eq!(t, back);
        }
    }

    #[test]
    fn composition_result_round_trip_json() {
        let result = CompositionResult {
            atomic: AtomicType::Tower,
            primals: vec![PrimalHealth {
                name: "beardog".to_owned(),
                socket_found: true,
                health_ok: true,
                capabilities: vec!["crypto.sign".to_owned()],
                latency_us: 500,
            }],
            all_healthy: true,
            discovery_ok: true,
            total_capabilities: 1,
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: CompositionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.atomic, AtomicType::Tower);
        assert!(back.all_healthy);
    }

    #[test]
    fn probe_primal_graceful_when_not_running() {
        let health = probe_primal("nonexistent_primal_xyzzy");
        assert!(!health.socket_found);
        assert!(!health.health_ok);
        assert!(health.capabilities.is_empty());
    }

    #[test]
    fn validate_composition_graceful_when_nothing_running() {
        let result = validate_composition(AtomicType::Tower);
        assert_eq!(result.atomic, AtomicType::Tower);
        assert_eq!(result.primals.len(), 2);
        assert!(!result.discovery_ok);
        assert!(!result.all_healthy);
    }

    #[test]
    fn health_check_fails_when_not_running() {
        assert!(health_check("nonexistent_primal_xyzzy").is_err());
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
