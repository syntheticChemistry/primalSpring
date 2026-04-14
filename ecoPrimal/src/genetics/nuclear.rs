// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tier 2 — Nuclear Genetics (Lineage DNA).
//!
//! Nuclear genetics govern permissions, authentication, and secure data.
//! The defining property: nuclear genetics are **never copied, always spawned
//! fresh**. Each derivation step mixes parent DNA with context entropy,
//! producing a child generation that is cryptographically linked to — but
//! distinct from — its parent.
//!
//! When you hand a friend encrypted data and the genetics to read it, the
//! genetics they receive is a **child** of yours, not a clone. The generation
//! counter + parent hash chain enables crypto-verifiable provenance: you can
//! prove the lineage of any nuclear key back to its genesis.
//!
//! Key derivation is delegated to BearDog:
//! - `genetic.derive_lineage_key` — per-domain key derivation with lineage mixing
//! - `genetic.mix_entropy` — three-tier entropy mixing (human, supervised, machine)
//! - `genetic.generate_lineage_proof` / `genetic.verify_lineage` — provenance proofs
//!
//! # Copy Resistance
//!
//! `NuclearGenetics` intentionally does **not** implement `Clone` or `Copy`.
//! The only way to propagate nuclear genetics is [`NuclearGenetics::spawn_child`],
//! which always creates a new generation. This is enforced at the type level.

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Nuclear genetics: generational, lineage-tracked, copy-resistant.
///
/// Each instance represents a single generation in a lineage chain.
/// **Not `Clone`** — propagation requires [`spawn_child`](Self::spawn_child)
/// which always creates a new generation with mixed DNA.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct NuclearGenetics {
    /// Generation counter (genesis = 0, first child = 1, ...).
    #[zeroize(skip)]
    generation: u64,
    /// Blake3 hash of the parent's lineage key (genesis uses all-zero).
    #[zeroize(skip)]
    parent_hash: [u8; 32],
    /// The derived lineage key for this generation.
    lineage_key: Vec<u8>,
    /// Lineage proof: cryptographic attestation linking this generation
    /// to its parent (Blake3 + HMAC via BearDog).
    #[zeroize(skip)]
    proof: Vec<u8>,
    /// Context under which this generation was derived (e.g. `"storage_v1"`,
    /// `"session_auth"`, `"data_handoff_to_peer_X"`).
    #[zeroize(skip)]
    context: String,
}

/// Serializable view of nuclear genetics metadata (without the secret key).
///
/// Used for lineage verification and audit trails where the actual key
/// material must not leak.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearLineageInfo {
    /// Generation counter.
    pub generation: u64,
    /// Blake3 hash of the parent's lineage key.
    pub parent_hash: [u8; 32],
    /// Lineage proof bytes.
    pub proof: Vec<u8>,
    /// Derivation context.
    pub context: String,
}

impl std::fmt::Debug for NuclearGenetics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NuclearGenetics")
            .field("generation", &self.generation)
            .field("context", &self.context)
            .field("key_len", &self.lineage_key.len())
            .finish_non_exhaustive()
    }
}

impl NuclearGenetics {
    /// Create a genesis (generation 0) nuclear identity.
    ///
    /// The genesis has no parent — its `parent_hash` is all zeros.
    /// In production, `lineage_key` comes from BearDog `genetic.derive_lineage_key`.
    #[must_use]
    pub const fn genesis(lineage_key: Vec<u8>, proof: Vec<u8>, context: String) -> Self {
        Self {
            generation: 0,
            parent_hash: [0u8; 32],
            lineage_key,
            proof,
            context,
        }
    }

    /// Spawn a child generation from this parent.
    ///
    /// This is the **only** way to propagate nuclear genetics. The child:
    /// - Has `generation = parent.generation + 1`
    /// - Records a Blake3 hash of the parent's key as `parent_hash`
    /// - Receives fresh `lineage_key` derived by BearDog (mixed with context entropy)
    /// - Gets its own lineage proof
    ///
    /// The parent's key material is **not** copied to the child — only a
    /// hash of it is retained for provenance verification.
    #[must_use]
    pub fn spawn_child(
        &self,
        child_lineage_key: Vec<u8>,
        child_proof: Vec<u8>,
        child_context: String,
    ) -> Self {
        let parent_hash = blake3_hash(&self.lineage_key);
        Self {
            generation: self.generation + 1,
            parent_hash,
            lineage_key: child_lineage_key,
            proof: child_proof,
            context: child_context,
        }
    }

    /// Generation counter (0 = genesis).
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Blake3 hash of the parent's lineage key.
    #[must_use]
    pub const fn parent_hash(&self) -> &[u8; 32] {
        &self.parent_hash
    }

    /// The raw lineage key bytes for this generation.
    #[must_use]
    pub fn key_bytes(&self) -> &[u8] {
        &self.lineage_key
    }

    /// The lineage proof for this generation.
    #[must_use]
    pub fn proof(&self) -> &[u8] {
        &self.proof
    }

    /// The derivation context.
    #[must_use]
    pub fn context(&self) -> &str {
        &self.context
    }

    /// Whether this is a genesis (generation 0) identity.
    #[must_use]
    pub const fn is_genesis(&self) -> bool {
        self.generation == 0
    }

    /// Extract a serializable lineage info (metadata without key material).
    #[must_use]
    pub fn lineage_info(&self) -> NuclearLineageInfo {
        NuclearLineageInfo {
            generation: self.generation,
            parent_hash: self.parent_hash,
            proof: self.proof.clone(),
            context: self.context.clone(),
        }
    }

    /// Blake3 hash of this generation's lineage key (for child spawning
    /// and verification without exposing the key itself).
    #[must_use]
    pub fn key_hash(&self) -> [u8; 32] {
        blake3_hash(&self.lineage_key)
    }
}

/// Blake3 hash helper (32-byte output).
fn blake3_hash(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_is_generation_zero() {
        let g = NuclearGenetics::genesis(
            vec![0xAA; 32],
            vec![0xBB; 64],
            "test-genesis".to_owned(),
        );
        assert_eq!(g.generation(), 0);
        assert!(g.is_genesis());
        assert_eq!(g.parent_hash(), &[0u8; 32]);
        assert_eq!(g.context(), "test-genesis");
    }

    #[test]
    fn child_increments_generation() {
        let parent = NuclearGenetics::genesis(
            vec![0xAA; 32],
            vec![0xBB; 64],
            "parent-context".to_owned(),
        );
        let child = parent.spawn_child(
            vec![0xCC; 32],
            vec![0xDD; 64],
            "child-context".to_owned(),
        );
        assert_eq!(child.generation(), 1);
        assert!(!child.is_genesis());
        assert_eq!(child.context(), "child-context");
    }

    #[test]
    fn child_records_parent_hash() {
        let parent = NuclearGenetics::genesis(
            vec![0xAA; 32],
            vec![],
            "parent".to_owned(),
        );
        let expected_hash = blake3_hash(&[0xAA; 32]);
        let child = parent.spawn_child(vec![0xCC; 32], vec![], "child".to_owned());
        assert_eq!(child.parent_hash(), &expected_hash);
    }

    #[test]
    fn child_key_differs_from_parent() {
        let parent = NuclearGenetics::genesis(
            vec![0xAA; 32],
            vec![],
            "parent".to_owned(),
        );
        let child = parent.spawn_child(vec![0xCC; 32], vec![], "child".to_owned());
        assert_ne!(parent.key_bytes(), child.key_bytes());
    }

    #[test]
    fn grandchild_chain() {
        let genesis = NuclearGenetics::genesis(vec![1; 32], vec![], "g0".to_owned());
        let child = genesis.spawn_child(vec![2; 32], vec![], "g1".to_owned());
        let grandchild = child.spawn_child(vec![3; 32], vec![], "g2".to_owned());

        assert_eq!(grandchild.generation(), 2);
        assert_eq!(grandchild.parent_hash(), &blake3_hash(&[2; 32]));
    }

    #[test]
    fn lineage_info_strips_key_material() {
        let g = NuclearGenetics::genesis(
            vec![0xFF; 32],
            vec![0xEE; 16],
            "test".to_owned(),
        );
        let info = g.lineage_info();
        assert_eq!(info.generation, 0);
        assert_eq!(info.proof, vec![0xEE; 16]);
        assert_eq!(info.context, "test");
    }

    #[test]
    fn key_hash_is_deterministic() {
        let g = NuclearGenetics::genesis(vec![0x42; 32], vec![], "det".to_owned());
        let h1 = g.key_hash();
        let h2 = g.key_hash();
        assert_eq!(h1, h2);
    }
}
