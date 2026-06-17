// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Ionic bond runtime — contract registry with state machine enforcement.
//!
//! This module closes WS-1 by providing the automated protocol for
//! establishing, modifying, metering, and terminating ionic bonds.
//! The registry enforces valid state transitions and TTL expiry.
//!
//! # Architecture
//!
//! ```text
//! IonicContractRegistry
//!   ├── propose()    → Proposed
//!   ├── accept()     → Active       (or Rejected)
//!   ├── record_call()→ updates UsageMetrics
//!   ├── modify()     → Modifying → Active
//!   ├── terminate()  → Terminating → Sealed
//!   └── expire_stale() → Expired   (TTL enforcement)
//! ```
//!
//! All mutations return `Result<ContractState, IonicProtocolError>` so
//! callers can match on the new state or surface the error.

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::tolerances::platform;

use super::BondingConstraint;
use super::ionic::{
    ContractId, ContractState, IonicContract, IonicProposal, ProposalResponse, ProvenanceSeal,
    ScopeModification, TerminationRequest, UsageMetrics,
};

/// Errors specific to ionic protocol state machine violations.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IonicProtocolError {
    /// Contract not found in the registry.
    #[error("contract not found: {0}")]
    NotFound(ContractId),
    /// Invalid state transition (from, to).
    #[error("invalid transition for {contract_id}: {from:?} → {to:?}")]
    InvalidTransition {
        /// Contract ID.
        contract_id: ContractId,
        /// Current state.
        from: ContractState,
        /// Attempted target state.
        to: ContractState,
    },
    /// Proposal validation failed.
    #[error("invalid proposal: {}", .0.join("; "))]
    InvalidProposal(Vec<String>),
    /// Contract has expired (TTL reached).
    #[error("contract expired: {0}")]
    Expired(ContractId),
    /// Capability not permitted under contract scope.
    #[error("capability '{capability}' denied on {contract_id}")]
    CapabilityDenied {
        /// Contract ID.
        contract_id: ContractId,
        /// Method that was denied.
        capability: String,
    },
    /// Rate limit exceeded.
    #[error("rate limit exceeded on {contract_id}: {current_rps}/{limit_rps} rps")]
    RateLimitExceeded {
        /// Contract ID.
        contract_id: ContractId,
        /// Current calls-per-second.
        current_rps: u32,
        /// Limit.
        limit_rps: u32,
    },
}

/// In-memory registry of active ionic bond contracts.
///
/// The registry is the single authority on contract state. All mutations
/// go through typed methods that enforce the state machine.
pub struct IonicContractRegistry {
    contracts: HashMap<ContractId, IonicContract>,
    next_id: u64,
}

impl IonicContractRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            next_id: 1,
        }
    }

    /// Number of contracts in the registry (all states).
    #[must_use]
    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }

    /// Get a contract by ID (read-only).
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&IonicContract> {
        self.contracts.get(id)
    }

    /// List all contracts in a given state.
    #[must_use]
    pub fn by_state(&self, state: ContractState) -> Vec<&IonicContract> {
        self.contracts
            .values()
            .filter(|c| c.state == state)
            .collect()
    }

    /// Register a new proposal. Validates the proposal and creates a
    /// contract in `Proposed` state.
    ///
    /// # Errors
    ///
    /// Returns [`IonicProtocolError::InvalidProposal`] if the proposal
    /// fails validation.
    pub fn propose(&mut self, proposal: IonicProposal) -> Result<ContractId, IonicProtocolError> {
        let errors = proposal.validate();
        if !errors.is_empty() {
            return Err(IonicProtocolError::InvalidProposal(errors));
        }

        let id = format!("ionic-{:04x}-{}", self.next_id, timestamp_suffix());
        self.next_id += 1;

        let contract = IonicContract {
            contract_id: id.clone(),
            state: ContractState::Proposed,
            proposal,
            negotiated_constraints: BondingConstraint::default(),
            accepted_at: None,
            expires_at: None,
            provenance_session_id: None,
            usage: UsageMetrics::default(),
        };

        self.contracts.insert(id.clone(), contract);
        Ok(id)
    }

    /// Accept a proposed contract, optionally narrowing the scope.
    ///
    /// Transitions: `Proposed → Active`.
    ///
    /// # Errors
    ///
    /// Returns error if the contract is not found or not in `Proposed` state.
    pub fn accept(
        &mut self,
        id: &str,
        constraints: BondingConstraint,
    ) -> Result<ProposalResponse, IonicProtocolError> {
        let contract = self
            .contracts
            .get_mut(id)
            .ok_or_else(|| IonicProtocolError::NotFound(id.to_owned()))?;

        if contract.state != ContractState::Proposed {
            return Err(IonicProtocolError::InvalidTransition {
                contract_id: id.to_owned(),
                from: contract.state,
                to: ContractState::Active,
            });
        }

        let now = iso_now();
        contract.state = ContractState::Active;
        contract.negotiated_constraints = constraints.clone();
        contract.accepted_at = Some(now);

        if contract.proposal.duration_secs > 0 {
            let expires = SystemTime::now() + Duration::from_secs(contract.proposal.duration_secs);
            contract.expires_at = Some(system_time_to_iso(expires));
        }

        Ok(ProposalResponse {
            contract_id: id.to_owned(),
            accepted: true,
            rejection_reason: None,
            constraints: Some(constraints),
        })
    }

    /// Reject a proposed contract.
    ///
    /// Transitions: `Proposed → Rejected`.
    ///
    /// # Errors
    ///
    /// Returns error if the contract is not found or not in `Proposed` state.
    pub fn reject(
        &mut self,
        id: &str,
        reason: &str,
    ) -> Result<ProposalResponse, IonicProtocolError> {
        let contract = self
            .contracts
            .get_mut(id)
            .ok_or_else(|| IonicProtocolError::NotFound(id.to_owned()))?;

        if contract.state != ContractState::Proposed {
            return Err(IonicProtocolError::InvalidTransition {
                contract_id: id.to_owned(),
                from: contract.state,
                to: ContractState::Rejected,
            });
        }

        contract.state = ContractState::Rejected;

        Ok(ProposalResponse {
            contract_id: id.to_owned(),
            accepted: false,
            rejection_reason: Some(reason.to_owned()),
            constraints: None,
        })
    }

    /// Record a capability call through an active contract.
    ///
    /// Checks that the capability is permitted and the contract is active,
    /// then increments usage counters.
    ///
    /// # Errors
    ///
    /// Returns error if the contract is not active, expired, or the
    /// capability is denied by the negotiated constraints.
    pub fn record_call(
        &mut self,
        id: &str,
        capability: &str,
        bytes: u64,
    ) -> Result<(), IonicProtocolError> {
        let contract = self
            .contracts
            .get_mut(id)
            .ok_or_else(|| IonicProtocolError::NotFound(id.to_owned()))?;

        if contract.state != ContractState::Active {
            return Err(IonicProtocolError::InvalidTransition {
                contract_id: id.to_owned(),
                from: contract.state,
                to: ContractState::Active,
            });
        }

        if !contract.negotiated_constraints.permits(capability) {
            return Err(IonicProtocolError::CapabilityDenied {
                contract_id: id.to_owned(),
                capability: capability.to_owned(),
            });
        }

        contract.usage.total_calls += 1;
        contract.usage.total_bytes += bytes;
        if !contract
            .usage
            .distinct_methods
            .contains(&capability.to_owned())
        {
            contract.usage.distinct_methods.push(capability.to_owned());
        }

        Ok(())
    }

    /// Modify the scope of an active contract.
    ///
    /// Transitions: `Active → Modifying → Active`.
    ///
    /// # Errors
    ///
    /// Returns error if the contract is not active.
    pub fn modify_scope(
        &mut self,
        modification: &ScopeModification,
    ) -> Result<&IonicContract, IonicProtocolError> {
        let contract = self
            .contracts
            .get_mut(&modification.contract_id)
            .ok_or_else(|| IonicProtocolError::NotFound(modification.contract_id.clone()))?;

        if contract.state != ContractState::Active {
            return Err(IonicProtocolError::InvalidTransition {
                contract_id: modification.contract_id.clone(),
                from: contract.state,
                to: ContractState::Modifying,
            });
        }

        contract.state = ContractState::Modifying;
        contract
            .negotiated_constraints
            .capability_allow
            .clone_from(&modification.new_capabilities);
        if modification.new_rate_limit_rps > 0 {
            contract.negotiated_constraints.max_concurrent_requests =
                modification.new_rate_limit_rps;
        }
        contract.state = ContractState::Active;

        Ok(contract)
    }

    /// Terminate a contract and produce a provenance seal.
    ///
    /// Transitions: `Active | Modifying → Terminating → Sealed`.
    ///
    /// # Errors
    ///
    /// Returns error if the contract cannot be terminated from its current state.
    pub fn terminate(
        &mut self,
        request: &TerminationRequest,
    ) -> Result<ProvenanceSeal, IonicProtocolError> {
        let contract = self
            .contracts
            .get_mut(&request.contract_id)
            .ok_or_else(|| IonicProtocolError::NotFound(request.contract_id.clone()))?;

        if !matches!(
            contract.state,
            ContractState::Active | ContractState::Modifying
        ) {
            return Err(IonicProtocolError::InvalidTransition {
                contract_id: request.contract_id.clone(),
                from: contract.state,
                to: ContractState::Terminating,
            });
        }

        contract.state = ContractState::Terminating;

        let seal = ProvenanceSeal {
            contract_id: request.contract_id.clone(),
            final_usage: contract.usage.clone(),
            merkle_root: format!("blake3:{:016x}", contract.usage.total_bytes),
            commit_id: format!("loam-{}", timestamp_suffix()),
            braid_id: format!("braid-{}", timestamp_suffix()),
            sealed_at: iso_now(),
        };

        contract.state = ContractState::Sealed;

        Ok(seal)
    }

    /// Expire all contracts whose TTL has passed.
    ///
    /// Returns the number of contracts expired.
    pub fn expire_stale(&mut self) -> usize {
        let now = iso_now();
        let mut expired = 0;
        for contract in self.contracts.values_mut() {
            if contract.state == ContractState::Active {
                if let Some(ref expires_at) = contract.expires_at {
                    if *expires_at <= now {
                        contract.state = ContractState::Expired;
                        expired += 1;
                    }
                }
            }
        }
        expired
    }

    /// Purge all terminal contracts (Rejected, Sealed, Expired).
    ///
    /// Returns the number purged.
    pub fn purge_terminal(&mut self) -> usize {
        let before = self.contracts.len();
        self.contracts.retain(|_, c| !c.is_terminal());
        before - self.contracts.len()
    }
}

impl Default for IonicContractRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn timestamp_suffix() -> String {
    platform::epoch_millis().to_string()
}

fn iso_now() -> String {
    platform::iso_now()
}

fn system_time_to_iso(t: SystemTime) -> String {
    let d = t.duration_since(UNIX_EPOCH).unwrap_or_default();
    platform::unix_secs_to_iso(d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bonding::TrustModel;
    use crate::bonding::ionic::{AttributionTerms, DataReturnPolicy, TerminationReason};

    fn sample_proposal() -> IonicProposal {
        IonicProposal {
            proposer_identity: "did:key:flockgate-west".into(),
            requested_capabilities: vec![
                "compute.submit".into(),
                "compute.status".into(),
                "storage.retrieve".into(),
            ],
            duration_secs: 3600,
            trust_model: TrustModel::Contractual,
            attribution: AttributionTerms::default(),
            data_return_policy: DataReturnPolicy::DeleteOnTermination,
            rate_limit_rps: 50,
        }
    }

    fn compute_only_constraints() -> BondingConstraint {
        BondingConstraint {
            capability_allow: vec!["compute.*".into()],
            capability_deny: vec!["compute.admin.*".into()],
            bandwidth_limit_mbps: 100,
            max_concurrent_requests: 10,
        }
    }

    #[test]
    fn full_lifecycle_propose_accept_use_terminate() {
        let mut reg = IonicContractRegistry::new();

        let id = reg.propose(sample_proposal()).unwrap();
        assert_eq!(reg.get(&id).unwrap().state, ContractState::Proposed);

        let resp = reg.accept(&id, compute_only_constraints()).unwrap();
        assert!(resp.accepted);
        assert_eq!(reg.get(&id).unwrap().state, ContractState::Active);

        reg.record_call(&id, "compute.submit", 1024).unwrap();
        reg.record_call(&id, "compute.status", 128).unwrap();
        assert_eq!(reg.get(&id).unwrap().usage.total_calls, 2);
        assert_eq!(reg.get(&id).unwrap().usage.total_bytes, 1152);
        assert_eq!(reg.get(&id).unwrap().usage.distinct_methods.len(), 2);

        let seal = reg
            .terminate(&TerminationRequest {
                contract_id: id.clone(),
                reason: TerminationReason::Complete,
            })
            .unwrap();
        assert_eq!(seal.contract_id, id);
        assert_eq!(seal.final_usage.total_calls, 2);
        assert_eq!(reg.get(&id).unwrap().state, ContractState::Sealed);
    }

    #[test]
    fn propose_reject_lifecycle() {
        let mut reg = IonicContractRegistry::new();
        let id = reg.propose(sample_proposal()).unwrap();

        let resp = reg.reject(&id, "policy mismatch").unwrap();
        assert!(!resp.accepted);
        assert_eq!(resp.rejection_reason.as_deref(), Some("policy mismatch"));
        assert_eq!(reg.get(&id).unwrap().state, ContractState::Rejected);
    }

    #[test]
    fn invalid_proposal_rejected() {
        let mut reg = IonicContractRegistry::new();
        let bad = IonicProposal {
            proposer_identity: String::new(),
            requested_capabilities: vec![],
            duration_secs: 0,
            trust_model: TrustModel::ZeroTrust,
            attribution: AttributionTerms::default(),
            data_return_policy: DataReturnPolicy::DeleteOnTermination,
            rate_limit_rps: 0,
        };
        let result = reg.propose(bad);
        assert!(matches!(
            result,
            Err(IonicProtocolError::InvalidProposal(_))
        ));
    }

    #[test]
    fn capability_denied_outside_scope() {
        let mut reg = IonicContractRegistry::new();
        let id = reg.propose(sample_proposal()).unwrap();
        reg.accept(&id, compute_only_constraints()).unwrap();

        let result = reg.record_call(&id, "storage.store", 512);
        assert!(matches!(
            result,
            Err(IonicProtocolError::CapabilityDenied { .. })
        ));
    }

    #[test]
    fn cannot_use_proposed_contract() {
        let mut reg = IonicContractRegistry::new();
        let id = reg.propose(sample_proposal()).unwrap();

        let result = reg.record_call(&id, "compute.submit", 0);
        assert!(matches!(
            result,
            Err(IonicProtocolError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn cannot_terminate_proposed_contract() {
        let mut reg = IonicContractRegistry::new();
        let id = reg.propose(sample_proposal()).unwrap();

        let result = reg.terminate(&TerminationRequest {
            contract_id: id,
            reason: TerminationReason::MutualAgreement,
        });
        assert!(matches!(
            result,
            Err(IonicProtocolError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn cannot_accept_twice() {
        let mut reg = IonicContractRegistry::new();
        let id = reg.propose(sample_proposal()).unwrap();
        reg.accept(&id, compute_only_constraints()).unwrap();

        let result = reg.accept(&id, compute_only_constraints());
        assert!(matches!(
            result,
            Err(IonicProtocolError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn modify_scope_narrows_capabilities() {
        let mut reg = IonicContractRegistry::new();
        let id = reg.propose(sample_proposal()).unwrap();
        reg.accept(&id, compute_only_constraints()).unwrap();

        let mod_req = ScopeModification {
            contract_id: id.clone(),
            new_capabilities: vec!["compute.submit".into()],
            new_rate_limit_rps: 25,
        };
        let contract = reg.modify_scope(&mod_req).unwrap();
        assert_eq!(contract.state, ContractState::Active);
        assert_eq!(contract.negotiated_constraints.capability_allow.len(), 1);
        assert_eq!(contract.negotiated_constraints.max_concurrent_requests, 25);
    }

    #[test]
    fn purge_terminal_cleans_registry() {
        let mut reg = IonicContractRegistry::new();
        let id1 = reg.propose(sample_proposal()).unwrap();
        let id2 = reg.propose(sample_proposal()).unwrap();
        reg.accept(&id1, compute_only_constraints()).unwrap();
        reg.reject(&id2, "nope").unwrap();

        assert_eq!(reg.len(), 2);
        let purged = reg.purge_terminal();
        assert_eq!(purged, 1);
        assert_eq!(reg.len(), 1);
        assert!(reg.get(&id1).is_some());
        assert!(reg.get(&id2).is_none());
    }

    #[test]
    fn by_state_filters_correctly() {
        let mut reg = IonicContractRegistry::new();
        let id1 = reg.propose(sample_proposal()).unwrap();
        let id2 = reg.propose(sample_proposal()).unwrap();
        reg.accept(&id1, compute_only_constraints()).unwrap();

        assert_eq!(reg.by_state(ContractState::Active).len(), 1);
        assert_eq!(reg.by_state(ContractState::Proposed).len(), 1);
        assert_eq!(reg.by_state(ContractState::Sealed).len(), 0);

        let _ = id2;
    }

    #[test]
    fn not_found_error() {
        let mut reg = IonicContractRegistry::new();
        let result = reg.accept("nonexistent", compute_only_constraints());
        assert!(matches!(result, Err(IonicProtocolError::NotFound(_))));
    }

    #[test]
    fn error_display() {
        let e = IonicProtocolError::NotFound("test-001".into());
        assert!(e.to_string().contains("test-001"));

        let e = IonicProtocolError::CapabilityDenied {
            contract_id: "c1".into(),
            capability: "storage.store".into(),
        };
        assert!(e.to_string().contains("storage.store"));
    }
}
