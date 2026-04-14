// SPDX-License-Identifier: AGPL-3.0-or-later

//! Three-tier genetics identity system.
//!
//! The genetics model is inspired by biological DNA inheritance:
//!
//! - **Tier 1 — Mito-Beacon** ([`mito_beacon`]): Mitochondrial genetics.
//!   Inherited group membership tokens for dark forest discovery, NAT
//!   negotiation, and metadata-level comms. Multiple per system. Freely
//!   cloneable (group membership is shared). BearDog:
//!   `genetic.derive_lineage_beacon_key`.
//!
//! - **Tier 2 — Nuclear** ([`nuclear`]): Lineage DNA. Permissions,
//!   authentication, secure tunnels. **Never copied, always spawned fresh.**
//!   Each derivation mixes parent DNA with context entropy. Generation
//!   counter + parent hash chain enables crypto-verifiable provenance.
//!   BearDog: `genetic.derive_lineage_key` + `genetic.mix_entropy`.
//!
//! - **Tier 3 — Tags** ([`tag`]): Deprecated plaintext `FAMILY_SEED`
//!   transformed into open participation channels. Hashtag/chat/Reddit
//!   comms layer. Freely copyable.
//!
//! # Two-Phase Connection Model
//!
//! 1. **Phase 1 (Mito)**: Mito-beacon establishes the tunnel. Proves group
//!    membership. Others with the beacon can interact with metadata.
//! 2. **Phase 2 (Nuclear)**: Within the mito tunnel, nuclear genetics spawn
//!    authenticated secure channels. All permissions and data flow through
//!    nuclear genetics, which are always child generations (never copies).
//!
//! # RPC Delegation
//!
//! All cryptographic operations are delegated to BearDog via JSON-RPC.
//! The [`rpc`] module provides typed wrappers around BearDog's `genetic.*`
//! methods.

pub mod mito_beacon;
pub mod nuclear;
pub mod rpc;
pub mod tag;

pub use mito_beacon::{MitoBeacon, MitoBeaconSet};
pub use nuclear::{NuclearGenetics, NuclearLineageInfo};
pub use tag::GeneticTag;

use serde::{Deserialize, Serialize};

/// Which genetics tier a security operation belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneticTier {
    /// Tier 1: mito-beacon (discovery, NAT, metadata).
    MitoBeacon,
    /// Tier 2: nuclear (permissions, auth, secure data).
    Nuclear,
    /// Tier 3: tag (open participation, legacy `FAMILY_SEED`).
    Tag,
}

impl GeneticTier {
    /// Human-readable description.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::MitoBeacon => "Mito-beacon: discovery, NAT, dark forest metadata",
            Self::Nuclear => "Nuclear: permissions, auth, secure tunnels (generational)",
            Self::Tag => "Tag: open participation channel (legacy FAMILY_SEED tier)",
        }
    }

    /// Whether this tier carries authentication/permission weight.
    #[must_use]
    pub const fn is_auth_tier(self) -> bool {
        matches!(self, Self::Nuclear)
    }

    /// Whether this tier is freely copyable.
    #[must_use]
    pub const fn is_copyable(self) -> bool {
        matches!(self, Self::MitoBeacon | Self::Tag)
    }
}

/// Composite genetics identity: a system's full genetic profile.
///
/// Combines mito-beacon set (discovery tier), optional nuclear genesis
/// (permission tier), and optional tags (open participation tier).
#[derive(Debug)]
pub struct GeneticIdentity {
    /// Mito-beacon set (Tier 1): multiple group memberships.
    pub beacons: MitoBeaconSet,
    /// Nuclear genetics (Tier 2): generational lineage key.
    /// `None` if no nuclear identity has been established yet.
    pub nuclear: Option<NuclearGenetics>,
    /// Tags (Tier 3): open participation channels.
    pub tags: Vec<GeneticTag>,
}

impl GeneticIdentity {
    /// Create an empty genetics identity (no beacons, no nuclear, no tags).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            beacons: MitoBeaconSet::new(),
            nuclear: None,
            tags: Vec::new(),
        }
    }

    /// Whether any genetics tier is populated.
    #[must_use]
    pub const fn has_any(&self) -> bool {
        !self.beacons.is_empty() || self.nuclear.is_some() || !self.tags.is_empty()
    }

    /// Whether the nuclear tier is established (required for permissions).
    #[must_use]
    pub const fn has_nuclear(&self) -> bool {
        self.nuclear.is_some()
    }

    /// The highest populated tier.
    #[must_use]
    pub const fn highest_tier(&self) -> Option<GeneticTier> {
        if self.nuclear.is_some() {
            Some(GeneticTier::Nuclear)
        } else if !self.beacons.is_empty() {
            Some(GeneticTier::MitoBeacon)
        } else if !self.tags.is_empty() {
            Some(GeneticTier::Tag)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genetic_tier_properties() {
        assert!(GeneticTier::Nuclear.is_auth_tier());
        assert!(!GeneticTier::MitoBeacon.is_auth_tier());
        assert!(!GeneticTier::Tag.is_auth_tier());

        assert!(!GeneticTier::Nuclear.is_copyable());
        assert!(GeneticTier::MitoBeacon.is_copyable());
        assert!(GeneticTier::Tag.is_copyable());
    }

    #[test]
    fn empty_identity() {
        let id = GeneticIdentity::empty();
        assert!(!id.has_any());
        assert!(!id.has_nuclear());
        assert_eq!(id.highest_tier(), None);
    }

    #[test]
    fn identity_with_beacon_only() {
        let mut id = GeneticIdentity::empty();
        id.beacons.add(MitoBeacon::new(
            "test".to_owned(),
            "Test".to_owned(),
            vec![1; 32],
        ));
        assert!(id.has_any());
        assert!(!id.has_nuclear());
        assert_eq!(id.highest_tier(), Some(GeneticTier::MitoBeacon));
    }

    #[test]
    fn identity_with_nuclear() {
        let mut id = GeneticIdentity::empty();
        id.nuclear = Some(NuclearGenetics::genesis(
            vec![0xAA; 32],
            vec![],
            "test".to_owned(),
        ));
        assert!(id.has_nuclear());
        assert_eq!(id.highest_tier(), Some(GeneticTier::Nuclear));
    }

    #[test]
    fn identity_with_tag_only() {
        let mut id = GeneticIdentity::empty();
        id.tags.push(GeneticTag::new("#test".to_owned()));
        assert!(id.has_any());
        assert_eq!(id.highest_tier(), Some(GeneticTier::Tag));
    }

    #[test]
    fn tier_serde_round_trip() {
        for tier in [GeneticTier::MitoBeacon, GeneticTier::Nuclear, GeneticTier::Tag] {
            let json = serde_json::to_string(&tier).unwrap();
            let back: GeneticTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier, back);
        }
    }
}
