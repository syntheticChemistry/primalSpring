// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Atomic composition logic — Tower, Node, Nest, Full NUCLEUS.
//!
//! Each atomic layer is a testable deployment target. primalSpring deploys
//! them via biomeOS graphs and validates that every primal starts, discovers
//! peers, and responds to capability calls.

use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cast;

mod probes;

pub use probes::{
    PrimalHealth, SubstrateHealth, health_check, health_check_within_tolerance,
    probe_primal_at_socket, probe_substrate,
};

/// Atomic composition layer — each represents a testable deployment target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtomicType {
    /// `BearDog` only (optional `SkunkBat`). The micro-atomic: minimal crypto identity.
    /// Used on fieldMouse-class embedded devices (≤256MB, single-core, TCP-only).
    Micro,
    /// `BearDog` + Songbird + skunkBat (crypto + mesh + defense). The electron shell.
    Tower,
    /// Tower + compute trio (`ToadStool` + `barraCuda` + `coralReef`). 6 primals.
    Node,
    /// Tower + `NestGate` + provenance trio (`rhizoCrypt` + `LoamSpine` + `sweetGrass`). 7 primals.
    Nest,
    /// All 13 primals: Tower + Node + Nest + meta-tier.
    FullNucleus,
}

impl fmt::Display for AtomicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Micro => "micro",
            Self::Tower => "tower",
            Self::Node => "node",
            Self::Nest => "nest",
            Self::FullNucleus => "nucleus",
        })
    }
}

/// Error returned when parsing an unknown atomic composition type.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown composition type: {0} (valid: micro, tower, node, nest, nucleus)")]
pub struct UnknownAtomicType(pub String);

impl FromStr for AtomicType {
    type Err = UnknownAtomicType;

    /// Accepts both lowercase CLI form (`tower`, `nucleus`, `full`) and
    /// `PascalCase` JSON-RPC form (`Tower`, `FullNucleus`, `Full`).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "micro" | "Micro" => Ok(Self::Micro),
            "tower" | "Tower" => Ok(Self::Tower),
            "node" | "Node" => Ok(Self::Node),
            "nest" | "Nest" => Ok(Self::Nest),
            "nucleus" | "full" | "FullNucleus" | "Full" => Ok(Self::FullNucleus),
            other => Err(UnknownAtomicType(other.to_owned())),
        }
    }
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
            Self::Micro => &["security"],
            Self::Tower => &["security", "discovery", "defense"],
            Self::Node => &[
                "security",
                "discovery",
                "defense",
                "compute",
                "tensor",
                "shader",
            ],
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

    /// Derive unique primal slugs from [`required_capabilities`](Self::required_capabilities)
    /// via the capability registry.
    ///
    /// This is the capability-based replacement for the removed `required_primals()`.
    /// Returns slugs in deterministic order (sorted, deduplicated).
    #[must_use]
    pub fn required_primal_slugs(self) -> Vec<&'static str> {
        let mut slugs: Vec<&str> = self
            .required_capabilities()
            .iter()
            .filter_map(|cap| {
                crate::composition::capability_to_primal_typed(cap)
                    .map(crate::primal_names::Primal::slug)
            })
            .collect();
        slugs.sort_unstable();
        slugs.dedup();
        slugs
    }

    /// biomeOS deploy graph name for this composition.
    #[must_use]
    pub const fn graph_name(self) -> &'static str {
        match self {
            Self::Micro => "micro_atomic_bootstrap",
            Self::Tower => "tower_atomic_bootstrap",
            Self::Node => "node_atomic_compute",
            Self::Nest => "nest_deploy",
            Self::FullNucleus => "nucleus_complete",
        }
    }

    /// Infer the closest atomic type from a primal count.
    ///
    /// Used for status display when the exact composition type is unknown
    /// but the number of running primals is observable.
    #[must_use]
    pub const fn from_primal_count(count: usize) -> &'static str {
        match count {
            0..=2 => "Micro",
            3..=4 => "Tower Atomic",
            5..=6 => "Node Atomic",
            7..=12 => "Nest",
            _ => "Full NUCLEUS",
        }
    }

    /// Human-readable description of this composition.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Micro => "BearDog only — minimal crypto identity (embedded/fieldMouse)",
            Self::Tower => "Security + Discovery + Defense (crypto + mesh + audit)",
            Self::Node => "Tower + Compute trio (dispatch + math + shaders)",
            Self::Nest => {
                "Tower + Storage + Provenance trio (content + DAG + ledger + attribution)"
            }
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

    let mut ctx = CompositionContext::discover();
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
            let primal_caps: Vec<String> = if has_client {
                caps.iter()
                    .filter(|other_cap| {
                        capability_to_primal(other_cap) == primal_name
                            && ctx.has_capability(other_cap)
                    })
                    .map(|s| (*s).to_owned())
                    .collect()
            } else {
                Vec::new()
            };
            PrimalHealth {
                name: primal_name,
                socket_found: has_client,
                health_ok,
                capabilities: primal_caps,
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
mod tests {
    use super::*;
    use crate::primal_names;

    #[test]
    fn tower_derives_three_primal_slugs() {
        let slugs = AtomicType::Tower.required_primal_slugs();
        assert_eq!(slugs.len(), 3);
        assert!(slugs.contains(&primal_names::BEARDOG));
        assert!(slugs.contains(&primal_names::SONGBIRD));
        assert!(slugs.contains(&primal_names::SKUNKBAT));
    }

    #[test]
    fn node_derives_six_primal_slugs() {
        let slugs = AtomicType::Node.required_primal_slugs();
        assert_eq!(slugs.len(), 6);
        assert!(slugs.contains(&primal_names::BEARDOG));
        assert!(slugs.contains(&primal_names::SONGBIRD));
        assert!(slugs.contains(&primal_names::SKUNKBAT));
        assert!(slugs.contains(&primal_names::TOADSTOOL));
        assert!(slugs.contains(&primal_names::BARRACUDA));
        assert!(slugs.contains(&primal_names::CORALREEF));
    }

    #[test]
    fn nest_derives_seven_primal_slugs() {
        let slugs = AtomicType::Nest.required_primal_slugs();
        assert_eq!(slugs.len(), 7);
        assert!(slugs.contains(&primal_names::BEARDOG));
        assert!(slugs.contains(&primal_names::SONGBIRD));
        assert!(slugs.contains(&primal_names::SKUNKBAT));
        assert!(slugs.contains(&primal_names::NESTGATE));
        assert!(slugs.contains(&primal_names::RHIZOCRYPT));
        assert!(slugs.contains(&primal_names::LOAMSPINE));
        assert!(slugs.contains(&primal_names::SWEETGRASS));
    }

    #[test]
    fn full_nucleus_derives_twelve_primal_slugs() {
        let slugs = AtomicType::FullNucleus.required_primal_slugs();
        assert_eq!(slugs.len(), 12);
        assert!(slugs.contains(&primal_names::SKUNKBAT));
        assert!(slugs.contains(&primal_names::BARRACUDA));
        assert!(slugs.contains(&primal_names::CORALREEF));
        assert!(slugs.contains(&primal_names::PETALTONGUE));
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
    fn validate_composition_ctx_graceful_regardless_of_environment() {
        let result = validate_composition_ctx(AtomicType::Tower);
        assert_eq!(result.atomic, AtomicType::Tower);
        // all_healthy depends on whether NUCLEUS is deployed — both states valid
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
