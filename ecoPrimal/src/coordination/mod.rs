// SPDX-License-Identifier: AGPL-3.0-or-later

//! Atomic composition logic — Tower, Node, Nest, Full NUCLEUS.
//!
//! Each atomic layer is a testable deployment target. primalSpring deploys
//! them via biomeOS graphs and validates that every primal starts, discovers
//! peers, and responds to capability calls.

use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cast;
use crate::ipc::client::PrimalClient;
use crate::ipc::discover::discover_for;
use crate::primal_names;

mod probes;

pub use probes::{
    PrimalHealth, SubstrateHealth, check_capability_health, check_primal_health, health_check,
    health_check_within_tolerance, probe_primal, probe_primal_at_socket, probe_substrate,
};

/// Atomic composition layer — each represents a testable deployment target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicType {
    /// `BearDog` + Songbird (crypto + mesh). Minimal NUCLEUS composition.
    Tower,
    /// Tower + compute triangle (`ToadStool` + `barraCuda` + `coralReef`).
    Node,
    /// Tower + `NestGate` + Squirrel (adds storage + AI bridge).
    Nest,
    /// All 13 primals: Tower + Node + Nest + meta-tier.
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
            Self::Node => &["security", "discovery", "compute", "tensor", "shader"],
            Self::Nest => &["security", "discovery", "storage", "ai"],
            Self::FullNucleus => &[
                "security",
                "discovery",
                "compute",
                "tensor",
                "shader",
                "storage",
                "ai",
                "dag",
                "commit",
                "provenance",
                "visualization",
                "ledger",
                "attribution",
                "defense",
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
                primal_names::BARRACUDA,
                primal_names::CORALREEF,
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
                primal_names::BARRACUDA,
                primal_names::CORALREEF,
                primal_names::NESTGATE,
                primal_names::SQUIRREL,
                primal_names::RHIZOCRYPT,
                primal_names::LOAMSPINE,
                primal_names::SWEETGRASS,
                primal_names::PETALTONGUE,
                primal_names::SKUNKBAT,
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
            Self::Node => "Tower + Compute triangle (dispatch + math + shaders)",
            Self::Nest => "Tower + Storage + AI bridge (+ persistence)",
            Self::FullNucleus => "All 13 primals: Tower + Node + Nest + meta-tier",
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
                let (health_ok, caps) = PrimalClient::connect(socket, &primal_name)
                    .map_or_else(
                        |_| (false, Vec::new()),
                        |mut c: PrimalClient| {
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

use probes::extract_capability_names;

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
    fn node_extends_tower_with_compute_triangle() {
        let primals = AtomicType::Node.required_primals();
        assert_eq!(primals.len(), 5);
        assert!(primals.contains(&primal_names::BEARDOG));
        assert!(primals.contains(&primal_names::SONGBIRD));
        assert!(primals.contains(&primal_names::TOADSTOOL));
        assert!(primals.contains(&primal_names::BARRACUDA));
        assert!(primals.contains(&primal_names::CORALREEF));
    }

    #[test]
    fn nest_extends_tower_with_nestgate_and_squirrel() {
        let primals = AtomicType::Nest.required_primals();
        assert!(primals.contains(&primal_names::NESTGATE));
        assert!(primals.contains(&primal_names::SQUIRREL));
    }

    #[test]
    fn full_nucleus_requires_twelve_primals() {
        let primals = AtomicType::FullNucleus.required_primals();
        assert_eq!(primals.len(), 12);
        assert!(primals.contains(&primal_names::BARRACUDA));
        assert!(primals.contains(&primal_names::CORALREEF));
        assert!(primals.contains(&primal_names::PETALTONGUE));
        assert!(primals.contains(&primal_names::SKUNKBAT));
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
    fn validate_composition_graceful_when_nothing_running() {
        let result = validate_composition(AtomicType::Tower);
        assert_eq!(result.atomic, AtomicType::Tower);
        assert_eq!(result.primals.len(), 2);
        assert!(!result.discovery_ok);
        assert!(!result.all_healthy);
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
        assert_eq!(result.primals.len(), 14);
    }

    #[test]
    fn validate_composition_node() {
        let result = validate_composition(AtomicType::Node);
        assert_eq!(result.atomic, AtomicType::Node);
        assert_eq!(result.primals.len(), 5);
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
        assert_eq!(result.primals.len(), 12);
    }

    #[test]
    fn required_capabilities_tower_has_security_and_discovery() {
        let caps = AtomicType::Tower.required_capabilities();
        assert!(caps.contains(&"security"));
        assert!(caps.contains(&"discovery"));
        assert_eq!(caps.len(), 2);
    }

    #[test]
    fn required_capabilities_full_nucleus_has_fourteen() {
        let caps = AtomicType::FullNucleus.required_capabilities();
        assert_eq!(caps.len(), 14);
        assert!(caps.contains(&"tensor"));
        assert!(caps.contains(&"shader"));
        assert!(caps.contains(&"visualization"));
        assert!(caps.contains(&"ai"));
        assert!(caps.contains(&"dag"));
        assert!(caps.contains(&"commit"));
        assert!(caps.contains(&"provenance"));
        assert!(caps.contains(&"ledger"));
        assert!(caps.contains(&"attribution"));
    }
}
