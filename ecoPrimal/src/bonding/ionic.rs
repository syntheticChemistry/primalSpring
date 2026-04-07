// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ionic bond negotiation protocol — Track 4.
//!
//! Ionic bonds enable cross-family, contract-based capability sharing between
//! NUCLEUS instances. Unlike covalent bonds (shared family seed, full trust),
//! ionic bonds are metered, scoped, and time-bounded.
//!
//! # Lifecycle
//!
//! ```text
//! Proposer                         Acceptor
//!    │                                │
//!    ├─── bonding.propose ──────────►│
//!    │                                ├── validate policy + constraints
//!    │◄── bonding.accept/reject ─────┤
//!    │                                │
//!    ├─── capability.call ──────────►│  (within scope)
//!    │◄── result ────────────────────┤
//!    │         ... metered ...        │
//!    │                                │
//!    ├─── bonding.modify_scope ─────►│  (optional)
//!    │◄── bonding.scope_ack ─────────┤
//!    │                                │
//!    ├─── bonding.terminate ────────►│  (or TTL expires)
//!    │◄── bonding.seal ──────────────┤  (provenance seal)
//! ```
//!
//! All negotiation is JSON-RPC 2.0 over the Neural API. Neither party
//! imports the other's code — only `capability.call` with `bonding.*`
//! operations.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{BondingConstraint, TrustModel};

/// Unique identifier for an ionic bond contract.
pub type ContractId = String;

/// Proposal to establish an ionic bond.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicProposal {
    /// Identity of the proposing NUCLEUS (`family_id` or DID).
    pub proposer_identity: String,
    /// Capability scopes requested (glob patterns like `"compute.*"`).
    pub requested_capabilities: Vec<String>,
    /// How long the bond should last (seconds). 0 = indefinite until terminated.
    pub duration_secs: u64,
    /// Trust model the proposer offers.
    pub trust_model: TrustModel,
    /// Attribution requirements for provenance.
    pub attribution: AttributionTerms,
    /// Data return policy after bond termination.
    pub data_return_policy: DataReturnPolicy,
    /// Maximum requests per second the proposer expects.
    pub rate_limit_rps: u32,
}

/// Attribution terms embedded in every provenance record during the bond.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionTerms {
    /// How the acceptor should be credited in provenance braids.
    pub credit_method: CreditMethod,
    /// Whether derived works must include the acceptor's attribution.
    pub propagate_to_derivatives: bool,
}

impl Default for AttributionTerms {
    fn default() -> Self {
        Self {
            credit_method: CreditMethod::CapabilityProvider,
            propagate_to_derivatives: true,
        }
    }
}

/// How the capability provider is credited in provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditMethod {
    /// Named as capability provider in sweetGrass braids.
    CapabilityProvider,
    /// Named as infrastructure contributor (anonymous compute).
    InfrastructureContributor,
    /// Full co-author attribution.
    CoAuthor,
}

/// What happens to data processed through the bond after termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataReturnPolicy {
    /// All intermediate data deleted from acceptor's `NestGate`.
    DeleteOnTermination,
    /// Data retained for the provenance trio only (no operational access).
    ProvenanceRetentionOnly,
    /// Data retained indefinitely (mutual agreement).
    RetainIndefinitely,
}

/// State machine for an ionic bond contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractState {
    /// Proposal sent, awaiting response.
    Proposed,
    /// Accepted and active — capability calls flow.
    Active,
    /// Rejected by the acceptor.
    Rejected,
    /// Scope modification in progress.
    Modifying,
    /// Termination initiated, awaiting provenance seal.
    Terminating,
    /// Sealed — provenance finalized, bond dissolved.
    Sealed,
    /// Expired — TTL reached without explicit termination.
    Expired,
}

/// A live ionic bond contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicContract {
    /// Unique contract identifier (BLAKE3 of proposal + timestamp).
    pub contract_id: ContractId,
    /// Current state of the contract.
    pub state: ContractState,
    /// The original proposal.
    pub proposal: IonicProposal,
    /// Negotiated constraints (may differ from proposal after acceptance).
    pub negotiated_constraints: BondingConstraint,
    /// When the contract was accepted (ISO 8601).
    pub accepted_at: Option<String>,
    /// When the contract expires (ISO 8601), if duration was specified.
    pub expires_at: Option<String>,
    /// Provenance session ID tracking this bond's activity.
    pub provenance_session_id: Option<String>,
    /// Cumulative usage metrics.
    pub usage: UsageMetrics,
}

/// Metered usage during an active ionic bond.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageMetrics {
    /// Total capability calls made through this bond.
    pub total_calls: u64,
    /// Total bytes transferred.
    pub total_bytes: u64,
    /// Distinct capability methods invoked.
    pub distinct_methods: Vec<String>,
}

/// Response to an ionic bond proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalResponse {
    /// Contract ID assigned to this bond.
    pub contract_id: ContractId,
    /// Whether the proposal was accepted.
    pub accepted: bool,
    /// If rejected, reason string.
    pub rejection_reason: Option<String>,
    /// Negotiated constraints (may narrow the requested scope).
    pub constraints: Option<BondingConstraint>,
}

/// Request to modify the scope of an active ionic bond.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeModification {
    /// Contract to modify.
    pub contract_id: ContractId,
    /// New capability allow list (replaces existing).
    pub new_capabilities: Vec<String>,
    /// New rate limit (0 = keep current).
    pub new_rate_limit_rps: u32,
}

/// Termination request with provenance seal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationRequest {
    /// Contract to terminate.
    pub contract_id: ContractId,
    /// Reason for termination.
    pub reason: TerminationReason,
}

/// Why a bond is being terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationReason {
    /// Normal completion — work finished.
    Complete,
    /// Mutual agreement to end early.
    MutualAgreement,
    /// Policy violation detected.
    PolicyViolation,
    /// TTL expired.
    Expired,
}

/// Provenance seal emitted when a bond terminates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceSeal {
    /// Contract that was sealed.
    pub contract_id: ContractId,
    /// Final usage metrics.
    pub final_usage: UsageMetrics,
    /// rhizoCrypt Merkle root of the bond's activity DAG.
    pub merkle_root: String,
    /// loamSpine commit ID for permanent record.
    pub commit_id: String,
    /// sweetGrass braid ID for attribution.
    pub braid_id: String,
    /// Timestamp of seal.
    pub sealed_at: String,
}

impl IonicProposal {
    /// Validate that the proposal is internally consistent.
    #[must_use]
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.proposer_identity.is_empty() {
            errors.push("proposer_identity must not be empty".into());
        }
        if self.requested_capabilities.is_empty() {
            errors.push("must request at least one capability".into());
        }
        if self.trust_model == TrustModel::GeneticLineage {
            errors.push("ionic bonds cannot use GeneticLineage trust (use covalent)".into());
        }
        if self.trust_model == TrustModel::ZeroTrust {
            errors.push("ionic bonds require at least Contractual trust".into());
        }
        errors
    }

    /// Estimated bond duration, or None for indefinite.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        if self.duration_secs == 0 {
            None
        } else {
            Some(Duration::from_secs(self.duration_secs))
        }
    }
}

impl IonicContract {
    /// Whether this contract permits a given capability call.
    #[must_use]
    pub fn permits(&self, capability: &str) -> bool {
        self.state == ContractState::Active && self.negotiated_constraints.permits(capability)
    }

    /// Whether the contract has reached a terminal state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self.state,
            ContractState::Rejected | ContractState::Sealed | ContractState::Expired
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_proposal() -> IonicProposal {
        IonicProposal {
            proposer_identity: "did:key:wetspring-lab".into(),
            requested_capabilities: vec!["compute.*".into(), "storage.retrieve".into()],
            duration_secs: 3600,
            trust_model: TrustModel::Contractual,
            attribution: AttributionTerms::default(),
            data_return_policy: DataReturnPolicy::DeleteOnTermination,
            rate_limit_rps: 100,
        }
    }

    #[test]
    fn proposal_validates() {
        let p = sample_proposal();
        assert!(p.validate().is_empty());
    }

    #[test]
    fn proposal_rejects_genetic_trust() {
        let mut p = sample_proposal();
        p.trust_model = TrustModel::GeneticLineage;
        let errors = p.validate();
        assert!(errors.iter().any(|e| e.contains("GeneticLineage")));
    }

    #[test]
    fn proposal_rejects_zero_trust() {
        let mut p = sample_proposal();
        p.trust_model = TrustModel::ZeroTrust;
        let errors = p.validate();
        assert!(errors.iter().any(|e| e.contains("Contractual")));
    }

    #[test]
    fn proposal_rejects_empty_identity() {
        let mut p = sample_proposal();
        p.proposer_identity = String::new();
        assert!(!p.validate().is_empty());
    }

    #[test]
    fn proposal_rejects_empty_capabilities() {
        let mut p = sample_proposal();
        p.requested_capabilities.clear();
        assert!(!p.validate().is_empty());
    }

    #[test]
    fn proposal_duration() {
        let p = sample_proposal();
        assert_eq!(p.duration(), Some(Duration::from_secs(3600)));

        let mut indefinite = sample_proposal();
        indefinite.duration_secs = 0;
        assert_eq!(indefinite.duration(), None);
    }

    #[test]
    fn contract_permits_only_when_active() {
        let contract = IonicContract {
            contract_id: "test-001".into(),
            state: ContractState::Active,
            proposal: sample_proposal(),
            negotiated_constraints: BondingConstraint {
                capability_allow: vec!["compute.*".into()],
                capability_deny: vec![],
                bandwidth_limit_mbps: 0,
                max_concurrent_requests: 0,
            },
            accepted_at: Some("2026-04-06T00:00:00Z".into()),
            expires_at: None,
            provenance_session_id: None,
            usage: UsageMetrics::default(),
        };

        assert!(contract.permits("compute.submit"));
        assert!(!contract.permits("storage.store"));
    }

    #[test]
    fn contract_denies_when_not_active() {
        let contract = IonicContract {
            contract_id: "test-002".into(),
            state: ContractState::Proposed,
            proposal: sample_proposal(),
            negotiated_constraints: BondingConstraint {
                capability_allow: vec!["compute.*".into()],
                ..BondingConstraint::default()
            },
            accepted_at: None,
            expires_at: None,
            provenance_session_id: None,
            usage: UsageMetrics::default(),
        };

        assert!(!contract.permits("compute.submit"));
    }

    #[test]
    fn terminal_states() {
        for state in [
            ContractState::Rejected,
            ContractState::Sealed,
            ContractState::Expired,
        ] {
            let contract = IonicContract {
                contract_id: "test".into(),
                state,
                proposal: sample_proposal(),
                negotiated_constraints: BondingConstraint::default(),
                accepted_at: None,
                expires_at: None,
                provenance_session_id: None,
                usage: UsageMetrics::default(),
            };
            assert!(contract.is_terminal());
        }
    }

    #[test]
    fn non_terminal_states() {
        for state in [
            ContractState::Proposed,
            ContractState::Active,
            ContractState::Modifying,
            ContractState::Terminating,
        ] {
            let contract = IonicContract {
                contract_id: "test".into(),
                state,
                proposal: sample_proposal(),
                negotiated_constraints: BondingConstraint::default(),
                accepted_at: None,
                expires_at: None,
                provenance_session_id: None,
                usage: UsageMetrics::default(),
            };
            assert!(!contract.is_terminal());
        }
    }

    #[test]
    fn proposal_round_trip_json() {
        let p = sample_proposal();
        let json = serde_json::to_string(&p).unwrap();
        let back: IonicProposal = serde_json::from_str(&json).unwrap();
        assert_eq!(back.proposer_identity, p.proposer_identity);
        assert_eq!(back.duration_secs, 3600);
    }

    #[test]
    fn contract_state_round_trip_json() {
        for state in [
            ContractState::Proposed,
            ContractState::Active,
            ContractState::Rejected,
            ContractState::Modifying,
            ContractState::Terminating,
            ContractState::Sealed,
            ContractState::Expired,
        ] {
            let json = serde_json::to_string(&state).unwrap();
            let back: ContractState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, back);
        }
    }

    #[test]
    fn usage_metrics_default() {
        let u = UsageMetrics::default();
        assert_eq!(u.total_calls, 0);
        assert_eq!(u.total_bytes, 0);
        assert!(u.distinct_methods.is_empty());
    }

    #[test]
    fn provenance_seal_round_trip_json() {
        let seal = ProvenanceSeal {
            contract_id: "contract-001".into(),
            final_usage: UsageMetrics::default(),
            merkle_root: "abc123".into(),
            commit_id: "commit-001".into(),
            braid_id: "braid-001".into(),
            sealed_at: "2026-04-06T01:00:00Z".into(),
        };
        let json = serde_json::to_string(&seal).unwrap();
        let back: ProvenanceSeal = serde_json::from_str(&json).unwrap();
        assert_eq!(back.contract_id, "contract-001");
        assert_eq!(back.merkle_root, "abc123");
    }
}
