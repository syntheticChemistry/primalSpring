// SPDX-License-Identifier: AGPL-3.0-or-later

//! Serializable provenance **types** for coordination experiments.
//!
//! Concrete Neural API routing to specific backends (for example the
//! provenance trio) does **not** belong in this module. primalSpring’s library
//! keeps only domain-agnostic shapes. Per the **self-knowledge principle**
//! described in the crate root, each spring owns how it wires capabilities;
//! coordination exposes generic [`ProvenanceStatus`], [`ProvenanceResult`],
//! and [`PipelineResult`] so callers can share JSON contracts without embedding
//! backend-specific operations here.

use serde::{Deserialize, Serialize};

/// Provenance availability status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceStatus {
    /// Full pipeline completed (e.g. dehydrate + commit + attribution).
    Complete,
    /// Partial pipeline (e.g. dehydration succeeded, later phase failed).
    Partial,
    /// Provenance backends unavailable — domain logic proceeds without provenance.
    Unavailable,
}

/// Result of a provenance operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceResult {
    /// Session or vertex identifier.
    pub id: String,
    /// Availability status.
    pub status: ProvenanceStatus,
    /// Structured response data from the backend.
    pub data: serde_json::Value,
}

impl ProvenanceResult {
    /// Placeholder result when provenance is not available.
    #[must_use]
    pub fn unavailable(context: &str) -> Self {
        Self {
            id: format!("local-{context}"),
            status: ProvenanceStatus::Unavailable,
            data: serde_json::json!({ "provenance": "unavailable" }),
        }
    }
}

/// Full multi-phase pipeline result (caller-defined semantics per phase).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    /// Overall status.
    pub status: ProvenanceStatus,
    /// Session that was completed.
    pub session_id: String,
    /// Merkle or content root from dehydration.
    pub merkle_root: String,
    /// Commit or spine reference.
    pub commit_id: String,
    /// Braid or attribution reference.
    pub braid_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_result_unavailable_has_correct_status() {
        let r = ProvenanceResult::unavailable("test");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
        assert!(r.id.starts_with("local-"));
    }

    #[test]
    fn pipeline_result_serializes_round_trip() {
        let result = PipelineResult {
            status: ProvenanceStatus::Complete,
            session_id: "sess-1".to_owned(),
            merkle_root: "abc123".to_owned(),
            commit_id: "commit-1".to_owned(),
            braid_id: "braid-1".to_owned(),
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: PipelineResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.status, ProvenanceStatus::Complete);
        assert_eq!(back.merkle_root, "abc123");
    }

    #[test]
    fn provenance_status_serializes_round_trip() {
        for status in [
            ProvenanceStatus::Complete,
            ProvenanceStatus::Partial,
            ProvenanceStatus::Unavailable,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let back: ProvenanceStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, back);
        }
    }
}
