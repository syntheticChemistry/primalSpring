// SPDX-License-Identifier: AGPL-3.0-or-later

//! Atomic composition logic — Tower, Node, Nest, Full NUCLEUS.
//!
//! Each atomic layer is a testable deployment target. primalSpring deploys
//! them via biomeOS graphs and validates that every primal starts, discovers
//! peers, and responds to capability calls.

use std::time::Instant;

use crate::primal_names;

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
    /// Tower + NestGate + Squirrel (adds storage + AI bridge).
    Nest,
    /// All 8 primals including Squirrel and provenance trio.
    FullNucleus,
}

impl AtomicType {
    /// Capability domains required for this composition (primal-provided).
    ///
    /// **Preferred**: resolve providers at runtime via
    /// [`crate::ipc::discover::discover_by_capability`]. This is the loose
    /// coupling path — callers ask for capabilities, not primal identities.
    ///
    /// Does not include `orchestration` — use [`substrate_capabilities`](Self::substrate_capabilities)
    /// for the biomeOS Neural API capabilities that every composition requires.
    #[must_use]
    pub const fn required_capabilities(self) -> &'static [&'static str] {
        match self {
            Self::Tower => &["security", "discovery"],
            Self::Node => &["security", "discovery", "compute"],
            Self::Nest => &["security", "discovery", "storage", "ai"],
            Self::FullNucleus => &[
                "security",
                "discovery",
                "compute",
                "storage",
                "ai",
                "dag",
                "commit",
                "provenance",
            ],
        }
    }

    /// biomeOS Neural API capabilities that every composition requires.
    ///
    /// All NUCLEUS compositions run on top of biomeOS's Neural API, which
    /// provides orchestration, graph execution, capability routing, and
    /// lifecycle management. These are the substrate capabilities.
    #[must_use]
    pub const fn substrate_capabilities() -> &'static [&'static str] {
        &[
            "orchestration",
            "graph.deploy",
            "graph.status",
            "graph.rollback",
            "capability.discover",
            "capability.route",
            "health.liveness",
        ]
    }

    /// Primal names required for this composition.
    ///
    /// **Legacy**: prefer [`required_capabilities`](Self::required_capabilities)
    /// for loose coupling. These names are retained for backward compatibility
    /// with deploy graphs and experiments that haven't migrated yet.
    ///
    /// Does not include `biomeos` — use [`substrate_primal`](Self::substrate_primal)
    /// for the biomeOS orchestrator that every composition requires.
    #[must_use]
    pub const fn required_primals(self) -> &'static [&'static str] {
        match self {
            Self::Tower => &[primal_names::BEARDOG, primal_names::SONGBIRD],
            Self::Node => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::TOADSTOOL,
            ],
            Self::Nest => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::NESTGATE,
                primal_names::SQUIRREL,
            ],
            Self::FullNucleus => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::TOADSTOOL,
                primal_names::NESTGATE,
                primal_names::SQUIRREL,
                primal_names::RHIZOCRYPT,
                primal_names::LOAMSPINE,
                primal_names::SWEETGRASS,
            ],
        }
    }

    /// The biomeOS substrate primal name.
    ///
    /// Every NUCLEUS composition requires biomeOS running in neural-api mode
    /// as the orchestration substrate. This is always `"biomeos"`.
    #[must_use]
    pub const fn substrate_primal() -> &'static str {
        primal_names::BIOMEOS
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
            Self::Tower => "Security + Discovery (crypto + mesh)",
            Self::Node => "Tower + Compute (+ GPU dispatch)",
            Self::Nest => "Tower + Storage + AI bridge (+ persistence)",
            Self::FullNucleus => "All capabilities (full composition)",
        }
    }
}

/// Validate an atomic composition by discovering providers for each
/// required capability at runtime.
///
/// **Loose coupling**: this function doesn't hardcode primal names.
/// It asks the Neural API (or filesystem) who provides each capability,
/// then probes whatever primal responds.
#[must_use]
pub fn validate_composition_by_capability(atomic: AtomicType) -> CompositionResult {
    let capabilities = atomic.required_capabilities();
    let results: Vec<_> = capabilities
        .iter()
        .map(|cap| {
            let disc = crate::ipc::discover::discover_by_capability(cap);
            let primal_name = disc
                .resolved_primal
                .unwrap_or_else(|| format!("capability:{cap}"));
            if let Some(ref socket) = disc.socket {
                let start = Instant::now();
                let (health_ok, caps) = client::PrimalClient::connect(socket, &primal_name)
                    .map_or_else(
                        |_| (false, Vec::new()),
                        |mut c| {
                            let h = c.health_check().unwrap_or(false);
                            let caps = extract_capability_names(c.capabilities().ok());
                            (h, caps)
                        },
                    );
                PrimalHealth {
                    name: primal_name,
                    socket_found: true,
                    health_ok,
                    capabilities: caps,
                    latency_us: cast::micros_u64(start.elapsed()),
                }
            } else {
                PrimalHealth {
                    name: primal_name,
                    socket_found: false,
                    health_ok: false,
                    capabilities: Vec::new(),
                    latency_us: 0,
                }
            }
        })
        .collect();

    let substrate = probe_substrate();

    let primal_healthy = results.iter().all(|p| p.health_ok);
    let substrate_healthy = substrate.as_ref().is_some_and(|s| s.health_ok);
    let all_healthy = primal_healthy && substrate_healthy;
    let discovery_ok = results.iter().all(|p| p.socket_found);
    let total_capabilities: usize = results.iter().map(|p| p.capabilities.len()).sum();

    CompositionResult {
        atomic,
        primals: results,
        substrate,
        all_healthy,
        discovery_ok,
        total_capabilities,
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
    /// biomeOS Neural API substrate health (if probed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substrate: Option<SubstrateHealth>,
    /// `true` if every primal passed its health check.
    pub all_healthy: bool,
    /// `true` if every primal's socket was discovered.
    pub discovery_ok: bool,
    /// Sum of capabilities across all primals.
    pub total_capabilities: usize,
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

/// Validate an entire atomic composition by probing all its required primals
/// and the biomeOS Neural API substrate.
#[must_use]
pub fn validate_composition(atomic: AtomicType) -> CompositionResult {
    let required = atomic.required_primals();
    let discovery = discover_for(required);
    let discovery_ok = discovery.iter().all(|d| d.socket.is_some());

    let primals: Vec<PrimalHealth> = required.iter().map(|name| probe_primal(name)).collect();
    let substrate = probe_substrate();

    let primal_healthy = primals.iter().all(|p| p.health_ok);
    let substrate_healthy = substrate.as_ref().is_some_and(|s| s.health_ok);
    let all_healthy = primal_healthy && substrate_healthy;
    let total_capabilities: usize = primals.iter().map(|p| p.capabilities.len()).sum();

    CompositionResult {
        atomic,
        primals,
        substrate,
        all_healthy,
        discovery_ok,
        total_capabilities,
    }
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
                    let h = c.health_check().unwrap_or(false);
                    let caps = extract_capability_names(c.capabilities().ok());
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

fn extract_capability_names(caps: Option<serde_json::Value>) -> Vec<String> {
    crate::ipc::discover::extract_capability_names(caps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_requires_two_primals() {
        assert_eq!(AtomicType::Tower.required_primals().len(), 2);
        assert!(
            AtomicType::Tower
                .required_primals()
                .contains(&primal_names::BEARDOG)
        );
        assert!(
            AtomicType::Tower
                .required_primals()
                .contains(&primal_names::SONGBIRD)
        );
    }

    #[test]
    fn node_extends_tower_with_toadstool() {
        let primals = AtomicType::Node.required_primals();
        assert!(primals.contains(&primal_names::BEARDOG));
        assert!(primals.contains(&primal_names::SONGBIRD));
        assert!(primals.contains(&primal_names::TOADSTOOL));
    }

    #[test]
    fn nest_extends_tower_with_nestgate_and_squirrel() {
        let primals = AtomicType::Nest.required_primals();
        assert!(primals.contains(&primal_names::NESTGATE));
        assert!(primals.contains(&primal_names::SQUIRREL));
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
            substrate: Some(SubstrateHealth {
                socket_found: true,
                health_ok: true,
                socket_path: Some("/tmp/biomeos/neural-api.sock".to_owned()),
                latency_us: 200,
            }),
            all_healthy: true,
            discovery_ok: true,
            total_capabilities: 1,
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: CompositionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.atomic, AtomicType::Tower);
        assert!(back.all_healthy);
        assert!(back.substrate.unwrap().health_ok);
    }

    #[test]
    fn substrate_capabilities_are_not_empty() {
        let caps = AtomicType::substrate_capabilities();
        assert!(!caps.is_empty());
        assert!(caps.contains(&"orchestration"));
        assert!(caps.contains(&"graph.deploy"));
    }

    #[test]
    fn substrate_primal_is_biomeos() {
        assert_eq!(AtomicType::substrate_primal(), "biomeos");
    }

    #[test]
    fn probe_substrate_returns_none_when_biomeos_not_running() {
        assert!(probe_substrate().is_none());
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

    #[test]
    fn check_capability_health_graceful_when_not_running() {
        use crate::validation::{NullSink, ValidationResult};
        use std::sync::Arc;
        let mut v = ValidationResult::new("test").with_sink(Arc::new(NullSink));
        check_capability_health(&mut v, "nonexistent_capability_xyzzy_12345");
        assert_eq!(v.skipped, 3, "should skip health, latency, caps");
        assert_eq!(v.failed, 0);
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
    fn validate_composition_by_capability_graceful_when_nothing_running() {
        let result = validate_composition_by_capability(AtomicType::Tower);
        assert_eq!(result.atomic, AtomicType::Tower);
        assert_eq!(result.primals.len(), 2);
        assert!(!result.all_healthy);
    }

    #[test]
    fn validate_composition_by_capability_full_nucleus() {
        let result = validate_composition_by_capability(AtomicType::FullNucleus);
        assert_eq!(result.primals.len(), 8);
        assert!(!result.all_healthy);
        assert_eq!(result.total_capabilities, 0);
    }

    #[test]
    fn health_check_within_tolerance_returns_none_when_unreachable() {
        assert!(health_check_within_tolerance("nonexistent_primal_xyzzy").is_none());
    }

    #[test]
    fn validate_composition_node() {
        let result = validate_composition(AtomicType::Node);
        assert_eq!(result.atomic, AtomicType::Node);
        assert_eq!(result.primals.len(), 3);
    }

    #[test]
    fn validate_composition_nest() {
        let result = validate_composition(AtomicType::Nest);
        assert_eq!(result.atomic, AtomicType::Nest);
        assert_eq!(result.primals.len(), 4);
    }

    #[test]
    fn validate_composition_full_nucleus() {
        let result = validate_composition(AtomicType::FullNucleus);
        assert_eq!(result.atomic, AtomicType::FullNucleus);
        assert_eq!(result.primals.len(), 8);
    }

    #[test]
    fn primal_health_socket_not_found_has_zero_latency() {
        let health = probe_primal("nonexistent_xyzzy_zero_latency");
        assert_eq!(health.latency_us, 0);
        assert_eq!(health.name, "nonexistent_xyzzy_zero_latency");
    }

    #[test]
    fn required_capabilities_tower_has_security_and_discovery() {
        let caps = AtomicType::Tower.required_capabilities();
        assert!(caps.contains(&"security"));
        assert!(caps.contains(&"discovery"));
        assert_eq!(caps.len(), 2);
    }

    #[test]
    fn required_capabilities_full_nucleus_has_eight() {
        let caps = AtomicType::FullNucleus.required_capabilities();
        assert_eq!(caps.len(), 8);
        assert!(caps.contains(&"ai"));
        assert!(caps.contains(&"dag"));
        assert!(caps.contains(&"commit"));
        assert!(caps.contains(&"provenance"));
    }
}
