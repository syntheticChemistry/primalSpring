// SPDX-License-Identifier: AGPL-3.0-or-later

//! Graph execution validation — all 5 coordination patterns.
//!
//! primalSpring validates Sequential, Parallel, `ConditionalDag`, Pipeline,
//! and Continuous graph execution with real primals (not mocks).

use serde::{Deserialize, Serialize};

/// quorumSignal coordination domain — the three pillars of ecosystem coordination.
///
/// Each domain uses `CoordinationPattern` variants as execution strategies.
/// Named after bacterial quorum sensing: collective behavior emerges when
/// enough gate NUCLEUS instances participate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationDomain {
    /// SENSE — observe, discover, react. Atomic signal graphs across
    /// 4 particle tiers (Tower/Node/Nest/Meta). `signal.dispatch` collapses
    /// N-squared primal calls to one semantic invocation.
    Signal,
    /// ACTION — create, mutate, prove. Emergent VCS (RootPulse) over the
    /// provenance trio. commit/branch/merge/diff/federate composed from
    /// rhizoCrypt + loamSpine + sweetGrass + BearDog + NestGate + Songbird.
    Pulse,
    /// SYNC — maintain ecosystem coherence across gates. WaterFall cascade
    /// sync through the VPS periplasm. Evolving from bash to Neural API
    /// `ecosystem` signal tier.
    Fall,
}

impl CoordinationDomain {
    /// Human-readable description of this coordination domain.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Signal => "Sense: observe, discover, react (atomic signal graphs across particle tiers)",
            Self::Pulse => "Action: create, mutate, prove (RootPulse emergent VCS)",
            Self::Fall => "Sync: ecosystem coherence across gates (WaterFall cascade)",
        }
    }

    /// Biological nervous system analogy.
    #[must_use]
    pub const fn biological_analogy(self) -> &'static str {
        match self {
            Self::Signal => "afferent (sensory neurons -> brain)",
            Self::Pulse => "efferent (brain -> motor neurons)",
            Self::Fall => "autonomic (heartbeat, breathing)",
        }
    }
}

/// Signal tier — maps atomic composition level to biological scale.
///
/// The first 4 tiers are particle-physics inspired and operate within a
/// single NUCLEUS instance. The 5th tier (`Ecosystem`) operates across
/// gates through the VPS periplasm — the waterFall coordination domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalTier {
    /// Trust boundary: bearDog + songbird + skunkBat. Quorum = 3.
    Tower,
    /// Compute: Tower + toadStool + barraCuda. Hardware dispatch.
    Node,
    /// Storage + provenance: Tower + nestGate + trio. Quorum = 4.
    Nest,
    /// Orchestration + AI + UI: biomeOS + squirrel + petalTongue.
    Meta,
    /// Cross-gate sync: waterFall cascade through VPS periplasm.
    /// Unlike particle tiers, ecosystem signals span NUCLEUS instances.
    Ecosystem,
}

impl SignalTier {
    /// Particle-physics analogy (None for ecosystem tier).
    #[must_use]
    pub const fn particle(self) -> Option<&'static str> {
        match self {
            Self::Tower => Some("electron"),
            Self::Node => Some("proton"),
            Self::Nest => Some("neutron"),
            Self::Meta => Some("above the atom"),
            Self::Ecosystem => None,
        }
    }

    /// Which coordination domain this tier primarily serves.
    #[must_use]
    pub const fn primary_domain(self) -> CoordinationDomain {
        match self {
            Self::Tower | Self::Node | Self::Meta => CoordinationDomain::Signal,
            Self::Nest => CoordinationDomain::Pulse,
            Self::Ecosystem => CoordinationDomain::Fall,
        }
    }
}

/// biomeOS graph execution pattern for coordinating primals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationPattern {
    /// Nodes executed in dependency order (A → B → C).
    Sequential,
    /// Independent nodes run concurrently.
    Parallel,
    /// DAG with `condition/skip_if` branching.
    ConditionalDag,
    /// Streaming via bounded mpsc channels.
    Pipeline,
    /// Fixed-timestep tick loop (e.g. 60 Hz).
    Continuous,
}

impl CoordinationPattern {
    /// Human-readable description of this execution pattern.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Sequential => "Nodes in dependency order (A -> B -> C)",
            Self::Parallel => "Independent nodes concurrently",
            Self::ConditionalDag => "DAG with condition/skip_if branching",
            Self::Pipeline => "Streaming via bounded mpsc channels",
            Self::Continuous => "Fixed-timestep tick loop (e.g. 60 Hz)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_domains_have_descriptions() {
        let domains = [
            CoordinationDomain::Signal,
            CoordinationDomain::Pulse,
            CoordinationDomain::Fall,
        ];
        for d in domains {
            assert!(!d.description().is_empty());
            assert!(!d.biological_analogy().is_empty());
        }
    }

    #[test]
    fn domain_round_trip_json() {
        for d in [
            CoordinationDomain::Signal,
            CoordinationDomain::Pulse,
            CoordinationDomain::Fall,
        ] {
            let json = serde_json::to_string(&d).unwrap();
            let back: CoordinationDomain = serde_json::from_str(&json).unwrap();
            assert_eq!(d, back);
        }
    }

    #[test]
    fn all_tiers_have_particles_or_not() {
        assert!(SignalTier::Tower.particle().is_some());
        assert!(SignalTier::Node.particle().is_some());
        assert!(SignalTier::Nest.particle().is_some());
        assert!(SignalTier::Meta.particle().is_some());
        assert!(SignalTier::Ecosystem.particle().is_none());
    }

    #[test]
    fn ecosystem_tier_maps_to_fall_domain() {
        assert_eq!(
            SignalTier::Ecosystem.primary_domain(),
            CoordinationDomain::Fall,
        );
    }

    #[test]
    fn tier_round_trip_json() {
        for t in [
            SignalTier::Tower,
            SignalTier::Node,
            SignalTier::Nest,
            SignalTier::Meta,
            SignalTier::Ecosystem,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let back: SignalTier = serde_json::from_str(&json).unwrap();
            assert_eq!(t, back);
        }
    }

    #[test]
    fn all_patterns_have_descriptions() {
        let patterns = [
            CoordinationPattern::Sequential,
            CoordinationPattern::Parallel,
            CoordinationPattern::ConditionalDag,
            CoordinationPattern::Pipeline,
            CoordinationPattern::Continuous,
        ];
        for p in patterns {
            assert!(!p.description().is_empty());
        }
    }

    #[test]
    fn pattern_round_trip_json() {
        for p in [
            CoordinationPattern::Sequential,
            CoordinationPattern::Parallel,
            CoordinationPattern::ConditionalDag,
            CoordinationPattern::Pipeline,
            CoordinationPattern::Continuous,
        ] {
            let json = serde_json::to_string(&p).unwrap();
            let back: CoordinationPattern = serde_json::from_str(&json).unwrap();
            assert_eq!(p, back);
        }
    }
}
