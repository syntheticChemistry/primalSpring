// SPDX-License-Identifier: AGPL-3.0-or-later

//! Atomic composition logic — Tower, Node, Nest, Full NUCLEUS.
//!
//! Each atomic layer is a testable deployment target. primalSpring deploys
//! them via biomeOS graphs and validates that every primal starts, discovers
//! peers, and responds to capability calls.

use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cast;
use crate::primal_names;

mod probes;

pub use probes::{
    PrimalHealth, SubstrateHealth, health_check, health_check_within_tolerance,
    probe_primal_at_socket, probe_substrate,
};

/// Atomic composition layer — each represents a testable deployment target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicType {
    /// `BearDog` + Songbird + skunkBat (crypto + mesh + defense). The electron shell.
    Tower,
    /// Tower + compute trio (`ToadStool` + `barraCuda` + `coralReef`). 6 primals.
    Node,
    /// Tower + `NestGate` + provenance trio (`rhizoCrypt` + `LoamSpine` + `sweetGrass`). 7 primals.
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
            Self::Tower => &["security", "discovery", "defense"],
            Self::Node => &["security", "discovery", "defense", "compute", "tensor", "shader"],
            Self::Nest => &[
                "security",
                "discovery",
                "defense",
                "storage",
                "dag",
                "ledger",
                "attribution",
            ],
            Self::FullNucleus => &[
                "security",
                "discovery",
                "defense",
                "compute",
                "tensor",
                "shader",
                "storage",
                "ai",
                "dag",
                "commit",
                "visualization",
                "ledger",
                "attribution",
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
    /// **Deprecated**: use [`required_capabilities`](Self::required_capabilities)
    /// and resolve providers at runtime via `discover_by_capability`. Primals
    /// should not hardcode peer identities — discover by capability domain.
    ///
    /// Does not include `biomeos` — use [`substrate_primal`](Self::substrate_primal)
    /// for the biomeOS orchestrator that every composition requires.
    #[must_use]
    #[deprecated(since = "0.9.31", note = "Use `required_capabilities()` + runtime discovery instead of hardcoded primal names")]
    pub const fn required_primals(self) -> &'static [&'static str] {
        match self {
            Self::Tower => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::SKUNKBAT,
            ],
            Self::Node => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::SKUNKBAT,
                primal_names::TOADSTOOL,
                primal_names::BARRACUDA,
                primal_names::CORALREEF,
            ],
            Self::Nest => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::SKUNKBAT,
                primal_names::NESTGATE,
                primal_names::RHIZOCRYPT,
                primal_names::LOAMSPINE,
                primal_names::SWEETGRASS,
            ],
            Self::FullNucleus => &[
                primal_names::BEARDOG,
                primal_names::SONGBIRD,
                primal_names::SKUNKBAT,
                primal_names::TOADSTOOL,
                primal_names::BARRACUDA,
                primal_names::CORALREEF,
                primal_names::NESTGATE,
                primal_names::SQUIRREL,
                primal_names::RHIZOCRYPT,
                primal_names::LOAMSPINE,
                primal_names::SWEETGRASS,
                primal_names::PETALTONGUE,
            ],
        }
    }

    /// The biomeOS substrate primal name.
    ///
    /// **Deprecated**: use [`substrate_capabilities`](Self::substrate_capabilities)
    /// and discover the orchestration provider at runtime.
    #[must_use]
    #[deprecated(since = "0.9.31", note = "Use `substrate_capabilities()` + runtime discovery instead")]
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
            Self::Tower => "Security + Discovery + Defense (crypto + mesh + audit)",
            Self::Node => "Tower + Compute trio (dispatch + math + shaders)",
            Self::Nest => "Tower + Storage + Provenance trio (content + DAG + ledger + attribution)",
            Self::FullNucleus => "All 13 primals: Tower + Node + Nest + meta-tier",
        }
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

/// Validate an atomic composition using a [`CompositionContext`].
///
/// Replaces the deprecated [`validate_composition`] with a context-aware
/// path that reuses discovered clients rather than probing each primal
/// individually. Maps capability domains back to primal names for the
/// result's `primals` field.
#[must_use]
pub fn validate_composition_ctx(atomic: AtomicType) -> CompositionResult {
    use crate::composition::{CompositionContext, capability_to_primal};

    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let caps = atomic.required_capabilities();

    let primals: Vec<PrimalHealth> = caps
        .iter()
        .map(|cap| {
            let start = Instant::now();
            let primal_name = capability_to_primal(cap).to_owned();
            let has_client = ctx.has_capability(cap);
            let health_ok = if has_client {
                ctx.health_check(cap).unwrap_or(false)
            } else {
                false
            };
            PrimalHealth {
                name: primal_name,
                socket_found: has_client,
                health_ok,
                capabilities: if has_client {
                    ctx.available_capabilities()
                        .iter()
                        .map(|s| (*s).to_owned())
                        .collect()
                } else {
                    Vec::new()
                },
                latency_us: cast::micros_u64(start.elapsed()),
            }
        })
        .collect();

    let substrate = probe_substrate();
    let primal_healthy = primals.iter().all(|p| p.health_ok);
    let substrate_healthy = substrate.as_ref().is_some_and(|s| s.health_ok);
    let all_healthy = primal_healthy && substrate_healthy;
    let discovery_ok = primals.iter().all(|p| p.socket_found);
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

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn tower_requires_three_primals() {
        let primals = AtomicType::Tower.required_primals();
        assert_eq!(primals.len(), 3);
        assert!(primals.contains(&primal_names::BEARDOG));
        assert!(primals.contains(&primal_names::SONGBIRD));
        assert!(primals.contains(&primal_names::SKUNKBAT));
    }

    #[test]
    fn node_extends_tower_with_compute_trio() {
        let primals = AtomicType::Node.required_primals();
        assert_eq!(primals.len(), 6);
        assert!(primals.contains(&primal_names::BEARDOG));
        assert!(primals.contains(&primal_names::SONGBIRD));
        assert!(primals.contains(&primal_names::SKUNKBAT));
        assert!(primals.contains(&primal_names::TOADSTOOL));
        assert!(primals.contains(&primal_names::BARRACUDA));
        assert!(primals.contains(&primal_names::CORALREEF));
    }

    #[test]
    fn nest_extends_tower_with_nestgate_and_provenance_trio() {
        let primals = AtomicType::Nest.required_primals();
        assert_eq!(primals.len(), 7);
        assert!(primals.contains(&primal_names::BEARDOG));
        assert!(primals.contains(&primal_names::SONGBIRD));
        assert!(primals.contains(&primal_names::SKUNKBAT));
        assert!(primals.contains(&primal_names::NESTGATE));
        assert!(primals.contains(&primal_names::RHIZOCRYPT));
        assert!(primals.contains(&primal_names::LOAMSPINE));
        assert!(primals.contains(&primal_names::SWEETGRASS));
    }

    #[test]
    fn full_nucleus_requires_twelve_primals() {
        let primals = AtomicType::FullNucleus.required_primals();
        assert_eq!(primals.len(), 12);
        assert!(primals.contains(&primal_names::SKUNKBAT));
        assert!(primals.contains(&primal_names::BARRACUDA));
        assert!(primals.contains(&primal_names::CORALREEF));
        assert!(primals.contains(&primal_names::PETALTONGUE));
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
    fn validate_composition_ctx_graceful_when_nothing_running() {
        let result = validate_composition_ctx(AtomicType::Tower);
        assert_eq!(result.atomic, AtomicType::Tower);
        assert!(!result.all_healthy);
    }

    #[test]
    fn required_capabilities_tower_has_security_discovery_defense() {
        let caps = AtomicType::Tower.required_capabilities();
        assert!(caps.contains(&"security"));
        assert!(caps.contains(&"discovery"));
        assert!(caps.contains(&"defense"));
        assert_eq!(caps.len(), 3);
    }

    #[test]
    fn required_capabilities_full_nucleus_has_thirteen() {
        let caps = AtomicType::FullNucleus.required_capabilities();
        assert_eq!(caps.len(), 13);
        assert!(caps.contains(&"defense"));
        assert!(caps.contains(&"tensor"));
        assert!(caps.contains(&"shader"));
        assert!(caps.contains(&"visualization"));
        assert!(caps.contains(&"ai"));
        assert!(caps.contains(&"dag"));
        assert!(caps.contains(&"commit"));
        assert!(caps.contains(&"ledger"));
        assert!(caps.contains(&"attribution"));
    }
}
