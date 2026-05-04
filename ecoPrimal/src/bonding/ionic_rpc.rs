// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ionic bond RPC client — cross-family trust negotiation protocol.
//!
//! Wraps `BearDog`'s `crypto.ionic_bond` methods (propose/accept/seal with
//! Ed25519) and Songbird's cross-family discovery into typed client APIs.
//!
//! # Protocol Flow
//!
//! ```text
//! Gate A (proposer)                        Gate B (acceptor)
//!   │                                        │
//!   ├── bonding.propose ──────── TCP ──────►│
//!   │   { proposal, signature }              ├── validate signature + policy
//!   │◄── bonding.accept ──────── TCP ───────┤   { contract_id, constraints }
//!   │                                        │
//!   ├── capability.call ──────── TCP ──────►│  (within scope + metered)
//!   │◄── result ──────────────── TCP ───────┤
//!   │                                        │
//!   ├── bonding.terminate ────── TCP ──────►│
//!   │◄── bonding.seal ─────────── TCP ──────┤   { provenance_seal }
//! ```
//!
//! The transport between gates is Songbird TCP (with BTSP Phase 3 encryption
//! when available). Cross-family peers discover each other via:
//! - Explicit `REMOTE_GATE_HOST` configuration
//! - `BirdSong` mesh multicast (same LAN)
//! - STUN/relay for NAT traversal (internet)

use serde::{Deserialize, Serialize};

use crate::ipc::error::IpcError;

use super::ionic::{
    ContractId, IonicContract, IonicProposal, ProposalResponse, ProvenanceSeal, ScopeModification,
    TerminationRequest,
};

/// Cross-family peer identity for ionic bond negotiation.
///
/// Unlike covalent peers (same `FAMILY_ID`), ionic peers have different
/// family identities. Trust is established via Ed25519 signatures on
/// proposals rather than shared secrets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicPeerIdentity {
    /// The peer's `FAMILY_ID` (different from ours).
    pub family_id: String,
    /// Ed25519 public key for signature verification.
    pub public_key: String,
    /// How we discovered this peer (explicit config, mesh, STUN).
    pub discovery_method: DiscoveryMethod,
    /// TCP address for cross-gate JSON-RPC (`host:port`).
    pub tcp_address: String,
}

/// How an ionic peer was discovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Explicit configuration (`REMOTE_GATE_HOST` or deploy graph).
    ExplicitConfig,
    /// `BirdSong` mesh UDP multicast (LAN discovery).
    BirdSongMesh,
    /// STUN/relay (NAT traversal, internet).
    StunRelay,
    /// Manual out-of-band (QR code, shared link, etc.).
    OutOfBand,
}

/// Request to `BearDog`'s `crypto.ionic_bond.propose`.
#[derive(Debug, Serialize)]
pub struct IonicProposeParams {
    /// Our family identity.
    pub our_family_id: String,
    /// The remote peer's public key (Ed25519).
    pub peer_public_key: String,
    /// The bond proposal to sign and send.
    pub proposal: IonicProposal,
}

/// Response from `crypto.ionic_bond.propose`.
#[derive(Debug, Deserialize)]
pub struct IonicProposeResult {
    /// Ed25519 signature over the serialized proposal.
    pub signature: String,
    /// The proposal serialized for transmission.
    pub signed_payload: String,
}

/// Request to `BearDog`'s `crypto.ionic_bond.verify_proposal`.
#[derive(Debug, Serialize)]
pub struct IonicVerifyParams {
    /// The signed payload received from the proposer.
    pub signed_payload: String,
    /// Ed25519 signature to verify.
    pub signature: String,
    /// Expected proposer public key.
    pub proposer_public_key: String,
}

/// Response from `crypto.ionic_bond.verify_proposal`.
#[derive(Debug, Deserialize)]
pub struct IonicVerifyResult {
    /// Whether the signature is valid.
    pub valid: bool,
    /// The deserialized proposal (if valid).
    pub proposal: Option<IonicProposal>,
}

/// Request to `BearDog`'s `crypto.ionic_bond.seal`.
#[derive(Debug, Serialize)]
pub struct IonicSealParams {
    /// Contract to seal.
    pub contract_id: ContractId,
    /// Final usage metrics hash.
    pub usage_hash: String,
    /// Our Ed25519 signature over the seal data.
    pub our_family_id: String,
}

/// Response from `crypto.ionic_bond.seal`.
#[derive(Debug, Deserialize)]
pub struct IonicSealResult {
    /// The sealed provenance record.
    pub seal: ProvenanceSeal,
    /// Ed25519 signature over the seal.
    pub signature: String,
}

/// High-level ionic bond negotiation client.
///
/// Operates against a remote gate via Songbird TCP. The local `BearDog`
/// handles cryptographic operations (signing, verification).
pub struct IonicBondClient {
    local_family_id: String,
    remote_peer: IonicPeerIdentity,
}

impl IonicBondClient {
    /// Create a client for negotiating an ionic bond with a remote peer.
    #[must_use]
    pub const fn new(local_family_id: String, remote_peer: IonicPeerIdentity) -> Self {
        Self {
            local_family_id,
            remote_peer,
        }
    }

    /// The remote peer's identity.
    #[must_use]
    pub const fn remote_peer(&self) -> &IonicPeerIdentity {
        &self.remote_peer
    }

    /// Propose an ionic bond to the remote peer.
    ///
    /// 1. Signs the proposal via local `BearDog` (`crypto.ionic_bond.propose`)
    /// 2. Sends the signed proposal to the remote gate via TCP
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if `BearDog` signing fails or the remote gate
    /// is unreachable.
    pub fn propose(&self, proposal: &IonicProposal) -> Result<ProposalResponse, IpcError> {
        let errors = proposal.validate();
        if !errors.is_empty() {
            return Err(IpcError::ProtocolError {
                detail: format!("Invalid ionic proposal: {}", errors.join("; ")),
            });
        }

        let propose_params = serde_json::json!({
            "our_family_id": self.local_family_id,
            "peer_public_key": self.remote_peer.public_key,
            "proposal": serde_json::to_value(proposal).map_err(|e| IpcError::SerializationError {
                detail: e.to_string(),
            })?,
        });

        let signed = crate::ipc::tcp::tcp_rpc(
            &extract_host(&self.remote_peer.tcp_address),
            extract_port(&self.remote_peer.tcp_address),
            "bonding.propose",
            &propose_params,
        )?;

        serde_json::from_value(signed.0).map_err(|e| IpcError::SerializationError {
            detail: format!("ionic propose response: {e}"),
        })
    }

    /// Terminate an active ionic bond.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the remote gate is unreachable or rejects
    /// the termination.
    pub fn terminate(&self, request: &TerminationRequest) -> Result<ProvenanceSeal, IpcError> {
        let params = serde_json::to_value(request).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?;

        let result = crate::ipc::tcp::tcp_rpc(
            &extract_host(&self.remote_peer.tcp_address),
            extract_port(&self.remote_peer.tcp_address),
            "bonding.terminate",
            &params,
        )?;

        serde_json::from_value(result.0).map_err(|e| IpcError::SerializationError {
            detail: format!("ionic terminate response: {e}"),
        })
    }

    /// Modify the scope of an active ionic bond.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the remote gate is unreachable or rejects
    /// the modification.
    pub fn modify_scope(
        &self,
        modification: &ScopeModification,
    ) -> Result<IonicContract, IpcError> {
        let params =
            serde_json::to_value(modification).map_err(|e| IpcError::SerializationError {
                detail: e.to_string(),
            })?;

        let result = crate::ipc::tcp::tcp_rpc(
            &extract_host(&self.remote_peer.tcp_address),
            extract_port(&self.remote_peer.tcp_address),
            "bonding.modify_scope",
            &params,
        )?;

        serde_json::from_value(result.0).map_err(|e| IpcError::SerializationError {
            detail: format!("ionic modify_scope response: {e}"),
        })
    }
}

fn extract_host(addr: &str) -> String {
    addr.rsplit_once(':')
        .map_or_else(|| addr.to_owned(), |(host, _)| host.to_owned())
}

fn extract_port(addr: &str) -> u16 {
    addr.rsplit_once(':')
        .and_then(|(_, port)| port.parse().ok())
        .unwrap_or(crate::tolerances::TCP_FALLBACK_BEARDOG_PORT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ionic_peer_identity_serde() {
        let peer = IonicPeerIdentity {
            family_id: "remote-family".to_owned(),
            public_key: "ed25519-pub-key-hex".to_owned(),
            discovery_method: DiscoveryMethod::BirdSongMesh,
            tcp_address: "192.168.1.100:9100".to_owned(),
        };
        let json = serde_json::to_string(&peer).unwrap();
        let back: IonicPeerIdentity = serde_json::from_str(&json).unwrap();
        assert_eq!(back.family_id, "remote-family");
        assert_eq!(back.discovery_method, DiscoveryMethod::BirdSongMesh);
    }

    #[test]
    fn discovery_method_serde() {
        for method in [
            DiscoveryMethod::ExplicitConfig,
            DiscoveryMethod::BirdSongMesh,
            DiscoveryMethod::StunRelay,
            DiscoveryMethod::OutOfBand,
        ] {
            let json = serde_json::to_string(&method).unwrap();
            let back: DiscoveryMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(method, back);
        }
    }

    #[test]
    fn extract_host_and_port() {
        assert_eq!(extract_host("192.168.1.100:9100"), "192.168.1.100");
        assert_eq!(extract_port("192.168.1.100:9100"), 9100);
        assert_eq!(extract_host("localhost"), "localhost");
        assert_eq!(extract_port("localhost"), 9100);
    }

    #[test]
    fn ionic_bond_client_constructor() {
        let peer = IonicPeerIdentity {
            family_id: "remote".to_owned(),
            public_key: "key".to_owned(),
            discovery_method: DiscoveryMethod::ExplicitConfig,
            tcp_address: "10.0.0.1:9100".to_owned(),
        };
        let client = IonicBondClient::new("local-family".to_owned(), peer);
        assert_eq!(client.remote_peer().family_id, "remote");
    }

    #[test]
    fn propose_rejects_invalid_proposal() {
        use super::super::TrustModel;
        use super::super::ionic::{AttributionTerms, DataReturnPolicy};

        let peer = IonicPeerIdentity {
            family_id: "remote".to_owned(),
            public_key: "key".to_owned(),
            discovery_method: DiscoveryMethod::ExplicitConfig,
            tcp_address: "10.0.0.1:9100".to_owned(),
        };
        let client = IonicBondClient::new("local".to_owned(), peer);

        let bad_proposal = IonicProposal {
            proposer_identity: String::new(),
            requested_capabilities: vec![],
            duration_secs: 3600,
            trust_model: TrustModel::Contractual,
            attribution: AttributionTerms::default(),
            data_return_policy: DataReturnPolicy::DeleteOnTermination,
            rate_limit_rps: 100,
        };

        let result = client.propose(&bad_proposal);
        assert!(result.is_err());
    }

    #[test]
    fn propose_params_serialize() {
        let params = IonicProposeParams {
            our_family_id: "local".to_owned(),
            peer_public_key: "pub-key".to_owned(),
            proposal: IonicProposal {
                proposer_identity: "did:key:test".to_owned(),
                requested_capabilities: vec!["compute.*".to_owned()],
                duration_secs: 3600,
                trust_model: super::super::TrustModel::Contractual,
                attribution: super::super::ionic::AttributionTerms::default(),
                data_return_policy: super::super::ionic::DataReturnPolicy::DeleteOnTermination,
                rate_limit_rps: 100,
            },
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["our_family_id"], "local");
    }

    #[test]
    fn verify_result_deserialize() {
        let json = r#"{"valid":true,"proposal":null}"#;
        let result: IonicVerifyResult = serde_json::from_str(json).unwrap();
        assert!(result.valid);
        assert!(result.proposal.is_none());
    }
}
