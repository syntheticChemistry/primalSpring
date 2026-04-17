// SPDX-License-Identifier: AGPL-3.0-or-later

//! Content distribution federation — multi-tier bonding for
//! content-addressed blob distribution across NUCLEUS gates.
//!
//! This module defines the domain types for a Steam-like content
//! distribution network built from NUCLEUS primals:
//!
//! - **Origin**: publishes content-addressed blobs to NestGate
//! - **Seeder pool**: metallic bond fleet that replicates content
//! - **Consumers**: ionic/weak bond peers that download content
//! - **Relay**: Songbird TCP for NAT traversal
//!
//! Content integrity is guaranteed by BLAKE3 hashing — tampered blobs
//! are immediately detectable regardless of which bond tier served them.

use serde::{Deserialize, Serialize};

use super::{BondType, TrustModel};

/// A content manifest listing blobs available for distribution.
///
/// The manifest itself is stored in NestGate under a human-readable key
/// (e.g., `ludospring:assets/release_v1.0.0`). Each entry maps a logical
/// path to a BLAKE3 content hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentManifest {
    /// Namespace for this content (e.g., `ludospring:assets`).
    pub namespace: String,
    /// Human-readable release identifier.
    pub release_id: String,
    /// BLAKE3 hash of the serialized manifest (self-referential integrity).
    pub manifest_hash: String,
    /// Individual content entries.
    pub entries: Vec<ContentEntry>,
    /// Total size in bytes across all entries.
    pub total_bytes: u64,
    /// Publisher's DID (e.g., `did:eco:<family_id>:origin`).
    pub publisher_did: String,
}

/// A single content blob in the manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEntry {
    /// Logical path within the namespace (e.g., `textures/terrain.dds`).
    pub path: String,
    /// BLAKE3 hash of the blob content — this is both the storage key
    /// and the integrity proof.
    pub blake3_hash: String,
    /// Size in bytes.
    pub size: u64,
    /// MIME type hint (e.g., `application/octet-stream`).
    pub mime_type: String,
}

/// Role of a gate in the content distribution network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionRole {
    /// Authoritative content source — publishes manifests and blobs.
    Origin,
    /// Same-family gate that replicates content from the origin.
    /// Participates in the metallic seeder pool.
    Seeder,
    /// Cross-family or anonymous peer that downloads content.
    Consumer,
    /// Songbird TCP relay for NAT traversal. Does not store content.
    Relay,
}

/// Bond tier assigned to a peer based on trust verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionBondTier {
    /// Same-family seeder: metallic bond, full replication rights.
    Metallic,
    /// Seeder-to-seeder mesh: covalent bond, mutual replication.
    Covalent,
    /// Cross-family consumer: ionic bond, metered downloads.
    Ionic,
    /// Anonymous retrieval: weak bond, hash-only authentication.
    Weak,
}

impl DistributionBondTier {
    /// Map to the underlying `BondType`.
    #[must_use]
    pub const fn bond_type(self) -> BondType {
        match self {
            Self::Metallic => BondType::Metallic,
            Self::Covalent => BondType::Covalent,
            Self::Ionic => BondType::Ionic,
            Self::Weak => BondType::Weak,
        }
    }

    /// Minimum trust model required for this tier.
    #[must_use]
    pub const fn required_trust(self) -> TrustModel {
        match self {
            Self::Covalent => TrustModel::NuclearLineage,
            Self::Metallic => TrustModel::MitoBeaconFamily,
            Self::Ionic => TrustModel::Contractual,
            Self::Weak => TrustModel::ZeroTrust,
        }
    }
}

/// Seeder enrollment request — a gate wanting to join the seeder pool
/// must prove nuclear lineage to the publisher's family.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeederEnrollment {
    /// The candidate seeder's family ID.
    pub family_id: String,
    /// Nuclear lineage proof (generation chain from genesis).
    pub lineage_proof: String,
    /// Mito-beacon for dark-forest discovery.
    pub mito_beacon: String,
    /// NestGate storage capacity (bytes) the seeder is willing to commit.
    pub storage_capacity_bytes: u64,
    /// Songbird TCP endpoint for cross-host transfers.
    pub songbird_tcp_endpoint: String,
}

/// Result of seeder enrollment verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentResult {
    /// Whether the enrollment was accepted.
    pub accepted: bool,
    /// If rejected, the reason.
    pub rejection_reason: Option<String>,
    /// Assigned seeder ID within the pool.
    pub seeder_id: Option<String>,
    /// Namespaces the seeder is authorized to replicate.
    pub authorized_namespaces: Vec<String>,
}

/// Replication status for content distributed to the seeder pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    /// Manifest being replicated.
    pub manifest_hash: String,
    /// Number of seeders that have a complete copy.
    pub complete_replicas: u32,
    /// Number of seeders currently replicating.
    pub in_progress: u32,
    /// Total bytes transferred across all seeders.
    pub bytes_transferred: u64,
    /// Per-seeder status.
    pub seeder_status: Vec<SeederReplicaStatus>,
}

/// Replication status for a single seeder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeederReplicaStatus {
    /// Seeder identifier.
    pub seeder_id: String,
    /// Whether this seeder has a complete copy.
    pub complete: bool,
    /// Number of blobs replicated out of total.
    pub blobs_replicated: u32,
    /// Total blobs in the manifest.
    pub blobs_total: u32,
    /// Bytes transferred to this seeder.
    pub bytes_transferred: u64,
}

/// Consumer download request — presented to a seeder via ionic or weak bond.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    /// BLAKE3 hash of the blob to download.
    pub blake3_hash: String,
    /// Optional byte range for partial/resumable downloads.
    pub range: Option<ByteRange>,
    /// Bond tier the consumer is operating under.
    pub bond_tier: DistributionBondTier,
    /// Ionic contract ID (required for ionic tier, None for weak).
    pub contract_id: Option<String>,
}

/// Byte range for partial content retrieval (resumable downloads).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ByteRange {
    /// Start offset (inclusive).
    pub start: u64,
    /// End offset (exclusive). None = read to end.
    pub end: Option<u64>,
}

/// Consumer download response from a seeder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResponse {
    /// BLAKE3 hash confirming the blob identity.
    pub blake3_hash: String,
    /// Total size of the blob.
    pub total_size: u64,
    /// Range served (matches request or full blob).
    pub range_served: Option<ByteRange>,
    /// Whether the seeder has the complete blob.
    pub available: bool,
    /// Bytes transferred in this response.
    pub bytes_served: u64,
}

/// Content announcement broadcast via Songbird mesh.
///
/// Seeders and the origin broadcast these so consumers can discover
/// available content without knowing seeder endpoints in advance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnnouncement {
    /// Namespace being announced.
    pub namespace: String,
    /// Manifest hash for integrity verification.
    pub manifest_hash: String,
    /// Number of seeders currently holding this content.
    pub seeder_count: u32,
    /// Bond types accepted for download.
    pub accepted_bond_types: Vec<BondType>,
    /// Mito-beacon topic for discovery (derived from namespace).
    pub discovery_topic: String,
    /// Total content size in bytes.
    pub total_bytes: u64,
}

impl ContentManifest {
    /// Validate manifest internal consistency.
    #[must_use]
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.namespace.is_empty() {
            errors.push("namespace must not be empty".into());
        }
        if self.release_id.is_empty() {
            errors.push("release_id must not be empty".into());
        }
        if self.entries.is_empty() {
            errors.push("manifest must contain at least one entry".into());
        }
        let computed_total: u64 = self.entries.iter().map(|e| e.size).sum();
        if computed_total != self.total_bytes {
            errors.push(format!(
                "total_bytes mismatch: declared {}, computed {computed_total}",
                self.total_bytes,
            ));
        }
        for entry in &self.entries {
            if entry.blake3_hash.is_empty() {
                errors.push(format!(
                    "entry '{}': blake3_hash must not be empty",
                    entry.path
                ));
            }
            if entry.path.is_empty() {
                errors.push("entry path must not be empty".into());
            }
        }
        errors
    }
}

impl ContentAnnouncement {
    /// Derive the Songbird discovery topic from a content namespace.
    #[must_use]
    pub fn topic_for_namespace(namespace: &str) -> String {
        format!("content:{namespace}")
    }
}

impl DownloadRequest {
    /// Validate the download request.
    #[must_use]
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.blake3_hash.is_empty() {
            errors.push("blake3_hash must not be empty".into());
        }
        if self.bond_tier == DistributionBondTier::Ionic && self.contract_id.is_none() {
            errors.push("ionic bond tier requires a contract_id".into());
        }
        if let Some(range) = &self.range {
            if let Some(end) = range.end {
                if end <= range.start {
                    errors.push(format!(
                        "invalid byte range: start={} >= end={end}",
                        range.start,
                    ));
                }
            }
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> ContentManifest {
        ContentManifest {
            namespace: "ludospring:assets".into(),
            release_id: "v1.0.0".into(),
            manifest_hash: "abc123".into(),
            entries: vec![
                ContentEntry {
                    path: "textures/terrain.dds".into(),
                    blake3_hash: "hash_terrain".into(),
                    size: 1024,
                    mime_type: "application/octet-stream".into(),
                },
                ContentEntry {
                    path: "models/player.glb".into(),
                    blake3_hash: "hash_player".into(),
                    size: 2048,
                    mime_type: "model/gltf-binary".into(),
                },
            ],
            total_bytes: 3072,
            publisher_did: "did:eco:test:origin".into(),
        }
    }

    #[test]
    fn manifest_validates() {
        let m = sample_manifest();
        assert!(m.validate().is_empty());
    }

    #[test]
    fn manifest_rejects_empty_namespace() {
        let mut m = sample_manifest();
        m.namespace = String::new();
        assert!(m.validate().iter().any(|e| e.contains("namespace")));
    }

    #[test]
    fn manifest_rejects_empty_entries() {
        let mut m = sample_manifest();
        m.entries.clear();
        m.total_bytes = 0;
        assert!(m.validate().iter().any(|e| e.contains("at least one")));
    }

    #[test]
    fn manifest_detects_total_bytes_mismatch() {
        let mut m = sample_manifest();
        m.total_bytes = 999;
        assert!(
            m.validate()
                .iter()
                .any(|e| e.contains("total_bytes mismatch"))
        );
    }

    #[test]
    fn manifest_rejects_empty_hash() {
        let mut m = sample_manifest();
        m.entries[0].blake3_hash = String::new();
        assert!(m.validate().iter().any(|e| e.contains("blake3_hash")));
    }

    #[test]
    fn bond_tier_maps_correctly() {
        assert_eq!(
            DistributionBondTier::Metallic.bond_type(),
            BondType::Metallic
        );
        assert_eq!(
            DistributionBondTier::Covalent.bond_type(),
            BondType::Covalent
        );
        assert_eq!(DistributionBondTier::Ionic.bond_type(), BondType::Ionic);
        assert_eq!(DistributionBondTier::Weak.bond_type(), BondType::Weak);
    }

    #[test]
    fn trust_requirements() {
        assert_eq!(
            DistributionBondTier::Covalent.required_trust(),
            TrustModel::NuclearLineage
        );
        assert_eq!(
            DistributionBondTier::Metallic.required_trust(),
            TrustModel::MitoBeaconFamily
        );
        assert_eq!(
            DistributionBondTier::Ionic.required_trust(),
            TrustModel::Contractual
        );
        assert_eq!(
            DistributionBondTier::Weak.required_trust(),
            TrustModel::ZeroTrust
        );
    }

    #[test]
    fn download_request_validates() {
        let req = DownloadRequest {
            blake3_hash: "somehash".into(),
            range: None,
            bond_tier: DistributionBondTier::Weak,
            contract_id: None,
        };
        assert!(req.validate().is_empty());
    }

    #[test]
    fn download_request_requires_contract_for_ionic() {
        let req = DownloadRequest {
            blake3_hash: "somehash".into(),
            range: None,
            bond_tier: DistributionBondTier::Ionic,
            contract_id: None,
        };
        assert!(req.validate().iter().any(|e| e.contains("contract_id")));
    }

    #[test]
    fn download_request_rejects_invalid_range() {
        let req = DownloadRequest {
            blake3_hash: "somehash".into(),
            range: Some(ByteRange {
                start: 100,
                end: Some(50),
            }),
            bond_tier: DistributionBondTier::Weak,
            contract_id: None,
        };
        assert!(
            req.validate()
                .iter()
                .any(|e| e.contains("invalid byte range"))
        );
    }

    #[test]
    fn download_request_rejects_empty_hash() {
        let req = DownloadRequest {
            blake3_hash: String::new(),
            range: None,
            bond_tier: DistributionBondTier::Weak,
            contract_id: None,
        };
        assert!(req.validate().iter().any(|e| e.contains("blake3_hash")));
    }

    #[test]
    fn content_announcement_topic() {
        assert_eq!(
            ContentAnnouncement::topic_for_namespace("ludospring:assets"),
            "content:ludospring:assets"
        );
    }

    #[test]
    fn seeder_enrollment_round_trip_json() {
        let enrollment = SeederEnrollment {
            family_id: "test-family".into(),
            lineage_proof: "proof-data".into(),
            mito_beacon: "beacon-data".into(),
            storage_capacity_bytes: 1_073_741_824,
            songbird_tcp_endpoint: "192.168.1.100:9090".into(),
        };
        let json = serde_json::to_string(&enrollment).unwrap();
        let back: SeederEnrollment = serde_json::from_str(&json).unwrap();
        assert_eq!(back.family_id, "test-family");
        assert_eq!(back.storage_capacity_bytes, 1_073_741_824);
    }

    #[test]
    fn replication_status_round_trip_json() {
        let status = ReplicationStatus {
            manifest_hash: "hash123".into(),
            complete_replicas: 2,
            in_progress: 1,
            bytes_transferred: 5000,
            seeder_status: vec![SeederReplicaStatus {
                seeder_id: "seeder-1".into(),
                complete: true,
                blobs_replicated: 10,
                blobs_total: 10,
                bytes_transferred: 3000,
            }],
        };
        let json = serde_json::to_string(&status).unwrap();
        let back: ReplicationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(back.complete_replicas, 2);
        assert_eq!(back.seeder_status.len(), 1);
        assert!(back.seeder_status[0].complete);
    }

    #[test]
    fn distribution_role_round_trip_json() {
        for role in [
            DistributionRole::Origin,
            DistributionRole::Seeder,
            DistributionRole::Consumer,
            DistributionRole::Relay,
        ] {
            let json = serde_json::to_string(&role).unwrap();
            let back: DistributionRole = serde_json::from_str(&json).unwrap();
            assert_eq!(role, back);
        }
    }
}
