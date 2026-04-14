// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tier 1 — Mitochondrial Beacon Genetics.
//!
//! Mito-beacons are inherited group membership tokens used for dark forest
//! discovery, NAT negotiation, and metadata-level comms. A system can hold
//! **multiple** beacons (school alumni, guild, family, etc.).
//!
//! Mito-beacons are effectively group public keys: anyone with the beacon
//! can hear birdsong and find rendezvous points. Grandma can tell a cousin
//! how to reach you without giving away all your contacts.
//!
//! Key derivation is delegated to BearDog via `genetic.derive_lineage_beacon_key`
//! (HKDF-SHA256, domain `birdsong_beacon_v1`).
//!
//! # Inheritance
//!
//! Mito-beacons are matrilineal: children inherit their parent's full beacon
//! set. Beacons are freely `Clone`-able — this is by design (group membership
//! is shared, not exclusive).

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A single mito-beacon: group membership token for dark forest discovery.
///
/// Freely cloneable — multiple systems in the same family/group/guild share
/// the same beacon key material. This is the "public key" tier: knowing the
/// beacon lets you participate in metadata-level comms (hear birdsong, find
/// NAT rendezvous), but grants **no permissions**.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitoBeacon {
    /// Opaque identifier for this beacon group (e.g. `"family-eastgate"`,
    /// `"guild-stormforge"`, `"alumni-2024"`).
    pub beacon_id: String,
    /// Human-readable name for the group.
    pub group_name: String,
    /// Derived beacon key material (from BearDog `genetic.derive_lineage_beacon_key`).
    /// Zeroized on drop to avoid lingering in memory.
    #[serde(skip)]
    key_material: SecretBytes,
}

/// Wrapper around secret bytes that zeroizes on drop but allows Clone
/// (mito-beacon keys are inheritable group secrets, not exclusive).
#[derive(Clone, Default, Zeroize, ZeroizeOnDrop)]
struct SecretBytes(Vec<u8>);

impl std::fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} bytes]", self.0.len())
    }
}

impl MitoBeacon {
    /// Create a mito-beacon from raw key material.
    #[must_use]
    pub const fn new(beacon_id: String, group_name: String, key_material: Vec<u8>) -> Self {
        Self {
            beacon_id,
            group_name,
            key_material: SecretBytes(key_material),
        }
    }

    /// The raw beacon key bytes (for BTSP mito-tier handshake or beacon encryption).
    #[must_use]
    pub fn key_bytes(&self) -> &[u8] {
        &self.key_material.0
    }

    /// Whether this beacon has valid key material (non-empty).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.key_material.0.is_empty()
    }
}

/// A set of mito-beacons held by a single system.
///
/// A system participates in multiple discovery groups simultaneously:
/// family beacons, guild beacons, organizational beacons, etc.
/// The full set is inherited by child systems (matrilineal inheritance).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MitoBeaconSet {
    beacons: Vec<MitoBeacon>,
}

impl MitoBeaconSet {
    /// Create an empty beacon set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a beacon to this set.
    pub fn add(&mut self, beacon: MitoBeacon) {
        self.beacons.push(beacon);
    }

    /// Look up a beacon by its group ID.
    #[must_use]
    pub fn get(&self, beacon_id: &str) -> Option<&MitoBeacon> {
        self.beacons.iter().find(|b| b.beacon_id == beacon_id)
    }

    /// All beacons in this set.
    #[must_use]
    pub fn all(&self) -> &[MitoBeacon] {
        &self.beacons
    }

    /// Number of beacons in this set.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.beacons.len()
    }

    /// Whether the set is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.beacons.is_empty()
    }

    /// Inherit (clone) the full beacon set — matrilineal inheritance.
    #[must_use]
    pub fn inherit(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mito_beacon_construction_and_key_access() {
        let beacon = MitoBeacon::new(
            "family-east".to_owned(),
            "Eastgate Family".to_owned(),
            vec![0xAA; 32],
        );
        assert_eq!(beacon.beacon_id, "family-east");
        assert_eq!(beacon.key_bytes().len(), 32);
        assert!(beacon.is_valid());
    }

    #[test]
    fn empty_beacon_is_invalid() {
        let beacon = MitoBeacon::new("empty".to_owned(), "Empty".to_owned(), vec![]);
        assert!(!beacon.is_valid());
    }

    #[test]
    fn beacon_set_operations() {
        let mut set = MitoBeaconSet::new();
        assert!(set.is_empty());

        set.add(MitoBeacon::new(
            "guild-storm".to_owned(),
            "Stormforge Guild".to_owned(),
            vec![1; 32],
        ));
        set.add(MitoBeacon::new(
            "alumni-24".to_owned(),
            "Class of 2024".to_owned(),
            vec![2; 32],
        ));

        assert_eq!(set.len(), 2);
        assert!(set.get("guild-storm").is_some());
        assert!(set.get("nonexistent").is_none());
    }

    #[test]
    fn beacon_set_inherits_fully() {
        let mut parent = MitoBeaconSet::new();
        parent.add(MitoBeacon::new(
            "family".to_owned(),
            "Family".to_owned(),
            vec![0xFF; 32],
        ));

        let child = parent.inherit();
        assert_eq!(child.len(), parent.len());
        assert_eq!(
            child.get("family").unwrap().key_bytes(),
            parent.get("family").unwrap().key_bytes(),
        );
    }

    #[test]
    fn beacon_clone_shares_key_material() {
        let original = MitoBeacon::new(
            "test".to_owned(),
            "Test".to_owned(),
            vec![0x42; 32],
        );
        let cloned = original.clone();
        assert_eq!(original.key_bytes(), cloned.key_bytes());
    }
}
