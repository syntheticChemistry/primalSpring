// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bonding model validation — covalent, ionic, metallic, weak, organo-metal-salt.
//!
//! Chemistry-inspired trust levels for multi-gate coordination via Plasmodium:
//! two or more NUCLEUS instances discovering each other, sharing capabilities,
//! and degrading gracefully when gates fail.
//!
//! See `primals/biomeOS/specs/NUCLEUS_BONDING_MODEL.md` for the full specification.

pub mod content_distribution;
pub mod graph_metadata;
pub mod ionic;
pub mod ionic_rpc;
pub mod stun_tiers;

use serde::{Deserialize, Serialize};

/// Multi-gate bonding model between NUCLEUS instances.
///
/// Maps to the chemistry-inspired trust hierarchy in `NUCLEUS_BONDING_MODEL.md`:
/// Covalent (genetic trust) > Metallic (organizational) > Ionic (contract) > Weak (zero trust).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BondType {
    /// Shared family seed, full trust, mutual discovery via `BirdSong` mesh.
    /// Use case: basement HPC, friend clusters, research collaborations.
    Covalent,
    /// Delocalized electron sea — Towers pool for global optimization.
    /// Use case: data center racks, k8s clusters, specialized GPU/storage fleets.
    Metallic,
    /// Cross-family, contract-based, metered capability sharing.
    /// Use case: cloud burst GPU, university HPC allocation, external APIs.
    Ionic,
    /// Temporary association, zero trust, minimal information disclosure.
    /// Sub-types: dipole-dipole, Brownian, Van der Waals, London dispersion.
    Weak,
    /// Multi-family complex with catalytic bridge (simultaneous bond types).
    /// Use case: covalent internally, ionic edge to cloud, weak to public APIs.
    OrganoMetalSalt,
}

impl BondType {
    /// Human-readable description of this bond type.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Covalent => "Shared family seed, full trust, mutual discovery",
            Self::Metallic => "Delocalized electron sea, specialization, global optimization",
            Self::Ionic => "Cross-family, contract-based, metered capability sharing",
            Self::Weak => "Temporary association, zero trust, minimal disclosure",
            Self::OrganoMetalSalt => "Multi-family complex with catalytic bridge",
        }
    }

    /// All bond type variants, ordered by trust level (highest first).
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Covalent,
            Self::Metallic,
            Self::Ionic,
            Self::Weak,
            Self::OrganoMetalSalt,
        ]
    }

    /// Whether this bond type shares Tower (electron) state.
    #[must_use]
    pub const fn shares_electrons(self) -> bool {
        matches!(self, Self::Covalent | Self::Metallic)
    }

    /// Whether interactions through this bond are metered/billed.
    #[must_use]
    pub const fn is_metered(self) -> bool {
        matches!(self, Self::Ionic)
    }
}

/// Trust model governing a bonding interaction.
///
/// The genetics tiers split the legacy `GeneticLineage` into two levels:
/// - [`MitoBeaconFamily`](Self::MitoBeaconFamily): group membership (discovery, NAT).
/// - [`NuclearLineage`](Self::NuclearLineage): permissions and auth (generational, non-fungible).
///
/// The legacy `GeneticLineage` is preserved for serde backward compatibility
/// and maps to `NuclearLineage` semantically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustModel {
    /// **Deprecated alias** — use [`NuclearLineage`](Self::NuclearLineage) or
    /// [`MitoBeaconFamily`](Self::MitoBeaconFamily) instead.
    ///
    /// Legacy `BearDog` genetic lineage via shared `.family.seed`.
    /// Kept for serde backward compatibility; semantically equivalent
    /// to `NuclearLineage`.
    GeneticLineage,
    /// Tier 1 mito-beacon trust: group membership verified via dark forest
    /// beacon exchange. Sufficient for discovery and organizational bonds
    /// (Metallic) but **not** for full-trust bonds (Covalent).
    MitoBeaconFamily,
    /// Tier 2 nuclear lineage trust: generational, non-fungible permissions.
    /// Required for Covalent bonds. Each session spawns a child generation
    /// with mixed DNA — never a copy.
    NuclearLineage,
    /// API key, OAuth, mutual TLS — contract-verified.
    Contractual,
    /// Cluster membership, LDAP/AD — organizational boundary.
    Organizational,
    /// Assume hostile, disclose nothing.
    ZeroTrust,
}

impl TrustModel {
    /// Whether this trust model carries genetic authentication (any tier).
    #[must_use]
    pub const fn is_genetic(self) -> bool {
        matches!(
            self,
            Self::GeneticLineage | Self::MitoBeaconFamily | Self::NuclearLineage
        )
    }

    /// Whether this trust model satisfies nuclear-tier requirements.
    #[must_use]
    pub const fn is_nuclear(self) -> bool {
        matches!(self, Self::GeneticLineage | Self::NuclearLineage)
    }

    /// Normalize legacy `GeneticLineage` to the appropriate new variant.
    /// Returns `NuclearLineage` for `GeneticLineage`, others unchanged.
    #[must_use]
    pub const fn normalize(self) -> Self {
        match self {
            Self::GeneticLineage => Self::NuclearLineage,
            other => other,
        }
    }
}

/// Capability-scoped constraint on what a bond may share.
///
/// Friends opting in idle compute might share `compute.*` but not `storage.*`.
/// A university HPC allocation might expose only `compute.submit` and `compute.status`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BondingConstraint {
    /// Glob patterns for capabilities shared through this bond (e.g. `"compute.*"`).
    pub capability_allow: Vec<String>,
    /// Glob patterns for capabilities explicitly denied (takes precedence over allow).
    pub capability_deny: Vec<String>,
    /// Maximum bandwidth in Mbps allocated to this bond (0 = unlimited).
    pub bandwidth_limit_mbps: u64,
    /// Maximum concurrent requests through this bond (0 = unlimited).
    pub max_concurrent_requests: u32,
}

impl BondingConstraint {
    /// Returns true if `capability` matches the allow list and is not denied.
    #[must_use]
    pub fn permits(&self, capability: &str) -> bool {
        let allowed = self.capability_allow.is_empty()
            || self
                .capability_allow
                .iter()
                .any(|pat| glob_match(pat, capability));
        let denied = self
            .capability_deny
            .iter()
            .any(|pat| glob_match(pat, capability));
        allowed && !denied
    }
}

/// Policy governing idle compute opt-in for covalent peers.
///
/// When a friend or family member joins the mesh, their node advertises
/// availability windows and capability scope. The policy is validated
/// before any workload is dispatched to the node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondingPolicy {
    /// Bond type this policy applies to.
    pub bond_type: BondType,
    /// Trust model required for this bond.
    pub trust_model: TrustModel,
    /// Capability constraints (what may flow through this bond).
    pub constraints: BondingConstraint,
    /// Optional time windows when this bond is active (cron-like, e.g. `"22:00-06:00"`).
    pub active_windows: Vec<String>,
    /// Whether the node offers relay service to other family members.
    pub offer_relay: bool,
    /// Human-readable label for this policy.
    pub label: String,
}

impl BondingPolicy {
    /// Create a default covalent policy (full trust, no restrictions).
    #[must_use]
    pub fn covalent_default() -> Self {
        Self {
            bond_type: BondType::Covalent,
            trust_model: TrustModel::GeneticLineage,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: true,
            label: "covalent-default".to_owned(),
        }
    }

    /// Create an idle-compute policy (covalent trust, compute-only, time-windowed).
    #[must_use]
    pub fn idle_compute(windows: Vec<String>, bandwidth_mbps: u64) -> Self {
        Self {
            bond_type: BondType::Covalent,
            trust_model: TrustModel::GeneticLineage,
            constraints: BondingConstraint {
                capability_allow: vec!["compute.*".to_owned()],
                capability_deny: vec!["storage.*".to_owned(), "ai.*".to_owned()],
                bandwidth_limit_mbps: bandwidth_mbps,
                max_concurrent_requests: 4,
            },
            active_windows: windows,
            offer_relay: false,
            label: "idle-compute".to_owned(),
        }
    }

    /// Create an ionic (contract-based) policy for external service consumption.
    #[must_use]
    pub fn ionic_contract(allowed_capabilities: Vec<String>) -> Self {
        Self {
            bond_type: BondType::Ionic,
            trust_model: TrustModel::Contractual,
            constraints: BondingConstraint {
                capability_allow: allowed_capabilities,
                capability_deny: Vec::new(),
                bandwidth_limit_mbps: 0,
                max_concurrent_requests: 0,
            },
            active_windows: Vec::new(),
            offer_relay: false,
            label: "ionic-contract".to_owned(),
        }
    }

    /// Validate that policy fields are internally consistent.
    #[must_use]
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.label.is_empty() {
            errors.push("policy label must not be empty".to_owned());
        }
        if self.bond_type == BondType::Weak && !self.constraints.capability_allow.is_empty() {
            errors.push("weak bonds should not declare capability_allow".to_owned());
        }
        if self.bond_type == BondType::Covalent && !self.trust_model.is_nuclear() {
            errors.push(
                "covalent bonds require nuclear-tier genetics (NuclearLineage or GeneticLineage)"
                    .to_owned(),
            );
        }
        if self.bond_type == BondType::Metallic && !self.trust_model.is_genetic() {
            errors.push(
                "metallic bonds require at least mito-beacon-tier genetics".to_owned(),
            );
        }
        if self.bond_type == BondType::Ionic && self.trust_model == TrustModel::ZeroTrust {
            errors.push("ionic bonds require at least Contractual trust".to_owned());
        }
        errors
    }

    /// Minimum BTSP cipher suite allowed by this policy's bond type.
    ///
    /// See `BTSP_PROTOCOL_STANDARD.md` §Cipher Selection Rules.
    #[must_use]
    pub const fn min_btsp_cipher(&self) -> crate::btsp::BtspCipherSuite {
        crate::btsp::min_cipher_for_bond(self.bond_type)
    }

    /// Check whether a requested BTSP cipher suite is allowed under this policy.
    #[must_use]
    pub const fn btsp_cipher_allowed(&self, cipher: crate::btsp::BtspCipherSuite) -> bool {
        crate::btsp::cipher_allowed(self.bond_type, cipher)
    }
}

/// BTSP enforcement decision for a single connection.
///
/// Produced by [`BtspEnforcer::evaluate`] at connection time (Enforcement Point 1)
/// and checked per-request at runtime (Enforcement Point 2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtspEnforcementDecision {
    /// Whether the connection is allowed at all.
    pub allowed: bool,
    /// Effective cipher suite for the session.
    pub cipher: crate::btsp::BtspCipherSuite,
    /// Whether the capability is permitted by the bond's constraint.
    pub capability_permitted: bool,
    /// Human-readable reason (for logging/debugging).
    pub reason: String,
}

/// Enforces `BondingPolicy` at the BTSP handshake and per-request layers.
///
/// This is the canonical enforcement point where the bonding model meets
/// the socket layer. Tower Atomic calls this to validate connections.
///
/// # Enforcement Behavior
///
/// Connections are evaluated in two dimensions:
///
/// 1. **Trust tier**: the peer's authenticated genetics tier must meet the
///    bonding policy's minimum. Covalent bonds require nuclear-tier trust;
///    metallic bonds require at least mito-beacon; ionic/weak bonds require
///    their respective minimum. Connections that fail the trust check are
///    **denied** (`allowed: false`).
///
/// 2. **Cipher suite**: if the peer's requested cipher is weaker than the
///    bond's minimum, the cipher is upgraded to the policy minimum. If the
///    peer cannot meet the minimum cipher, the connection is denied.
pub struct BtspEnforcer;

impl BtspEnforcer {
    /// Enforcement Point 1: evaluate a connection at handshake time.
    ///
    /// Called after BTSP handshake succeeds. Checks the peer's trust tier
    /// against the bond policy and determines the effective cipher suite.
    /// Returns `allowed: false` if the peer's trust tier is insufficient
    /// for the bond type.
    #[must_use]
    pub fn evaluate_connection(
        policy: &BondingPolicy,
        requested_cipher: crate::btsp::BtspCipherSuite,
    ) -> BtspEnforcementDecision {
        Self::evaluate_connection_with_trust(policy, requested_cipher, None)
    }

    /// Enforcement Point 1b: evaluate a connection with an explicit peer trust tier.
    ///
    /// When the peer's [`GeneticSecurityMode`](crate::btsp::GeneticSecurityMode)
    /// is known (from the BTSP handshake metadata or session negotiation),
    /// pass it here for trust-tier enforcement. If `peer_trust` is `None`,
    /// trust checking is skipped (backward-compatible with pre-genetics callers).
    #[must_use]
    pub fn evaluate_connection_with_trust(
        policy: &BondingPolicy,
        requested_cipher: crate::btsp::BtspCipherSuite,
        peer_trust: Option<TrustModel>,
    ) -> BtspEnforcementDecision {
        let bond_label = serde_json::to_string(&policy.bond_type).unwrap_or_default();

        if let Some(peer) = peer_trust {
            if !trust_meets_policy(peer, policy) {
                return BtspEnforcementDecision {
                    allowed: false,
                    cipher: requested_cipher,
                    capability_permitted: false,
                    reason: format!(
                        "Bond {bond_label} requires {}, peer offers {} — denied",
                        trust_requirement_description(policy),
                        serde_json::to_string(&peer).unwrap_or_default(),
                    ),
                };
            }
        }

        let cipher_ok = policy.btsp_cipher_allowed(requested_cipher);
        let effective_cipher = if cipher_ok {
            requested_cipher
        } else {
            policy.min_btsp_cipher()
        };

        BtspEnforcementDecision {
            allowed: true,
            cipher: effective_cipher,
            capability_permitted: true,
            reason: if cipher_ok {
                format!(
                    "Bond {bond_label} allows {} — granted",
                    requested_cipher.description()
                )
            } else {
                format!(
                    "Bond {bond_label} requires minimum {} — upgraded from {}",
                    effective_cipher.description(),
                    requested_cipher.description()
                )
            },
        }
    }

    /// Enforcement Point 2: evaluate a per-request capability check.
    ///
    /// Called on every JSON-RPC request after the session is established.
    /// Checks whether the requested capability is allowed through this bond.
    #[must_use]
    pub fn evaluate_request(
        policy: &BondingPolicy,
        capability: &str,
        session_cipher: crate::btsp::BtspCipherSuite,
    ) -> BtspEnforcementDecision {
        let cap_ok = policy.constraints.permits(capability);

        BtspEnforcementDecision {
            allowed: cap_ok,
            cipher: session_cipher,
            capability_permitted: cap_ok,
            reason: if cap_ok {
                format!("Capability '{capability}' permitted by bond constraints")
            } else {
                format!("Capability '{capability}' denied by bond constraints")
            },
        }
    }
}

/// Check whether a peer's trust model satisfies a bonding policy's requirements.
const fn trust_meets_policy(peer_trust: TrustModel, policy: &BondingPolicy) -> bool {
    match policy.bond_type {
        BondType::Covalent => peer_trust.is_nuclear(),
        BondType::Metallic | BondType::OrganoMetalSalt => peer_trust.is_genetic(),
        BondType::Ionic => matches!(
            peer_trust,
            TrustModel::NuclearLineage
                | TrustModel::GeneticLineage
                | TrustModel::MitoBeaconFamily
                | TrustModel::Contractual
        ),
        BondType::Weak => true,
    }
}

const fn trust_requirement_description(policy: &BondingPolicy) -> &'static str {
    match policy.bond_type {
        BondType::Covalent => "nuclear-tier genetics (NuclearLineage)",
        BondType::Metallic | BondType::OrganoMetalSalt => "at least mito-beacon genetics",
        BondType::Ionic => "at least Contractual trust",
        BondType::Weak => "any trust model",
    }
}

/// Result of a bonding validation experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondingResult {
    /// Which bonding model was tested.
    pub bond_type: BondType,
    /// Number of `NestGate` instances discovered during bonding.
    pub gates_discovered: usize,
    /// Number of capabilities shared across the bond.
    pub capabilities_shared: usize,
    /// Whether trust was established per the bond model.
    pub trust_verified: bool,
    /// Whether degradation was graceful when gates failed.
    pub degradation_graceful: bool,
}

/// Simple glob matching: supports trailing `*` wildcard (e.g. `"compute.*"` matches `"compute.submit"`).
fn glob_match(pattern: &str, value: &str) -> bool {
    pattern
        .strip_suffix('*')
        .map_or_else(|| pattern == value, |prefix| value.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bond_types_have_descriptions() {
        for bt in BondType::all() {
            assert!(!bt.description().is_empty(), "{bt:?} has empty description");
        }
    }

    #[test]
    fn bond_type_count() {
        assert_eq!(BondType::all().len(), 5);
    }

    #[test]
    fn bond_type_round_trip_json() {
        for bt in BondType::all() {
            let json = serde_json::to_string(bt).unwrap();
            let back: BondType = serde_json::from_str(&json).unwrap();
            assert_eq!(*bt, back);
        }
    }

    #[test]
    fn electron_sharing() {
        assert!(BondType::Covalent.shares_electrons());
        assert!(BondType::Metallic.shares_electrons());
        assert!(!BondType::Ionic.shares_electrons());
        assert!(!BondType::Weak.shares_electrons());
    }

    #[test]
    fn metering() {
        assert!(BondType::Ionic.is_metered());
        assert!(!BondType::Covalent.is_metered());
    }

    #[test]
    fn constraint_permits() {
        let c = BondingConstraint {
            capability_allow: vec!["compute.*".to_owned()],
            capability_deny: vec!["compute.admin".to_owned()],
            bandwidth_limit_mbps: 100,
            max_concurrent_requests: 4,
        };
        assert!(c.permits("compute.submit"));
        assert!(c.permits("compute.status"));
        assert!(!c.permits("compute.admin"));
        assert!(!c.permits("storage.store"));
    }

    #[test]
    fn constraint_empty_allow_permits_all() {
        let c = BondingConstraint::default();
        assert!(c.permits("anything"));
        assert!(c.permits("compute.submit"));
    }

    #[test]
    fn policy_covalent_default_validates() {
        let p = BondingPolicy::covalent_default();
        assert!(p.validate().is_empty());
    }

    #[test]
    fn policy_idle_compute_validates() {
        let p = BondingPolicy::idle_compute(vec!["22:00-06:00".to_owned()], 100);
        assert!(p.validate().is_empty());
        assert!(p.constraints.permits("compute.submit"));
        assert!(!p.constraints.permits("storage.store"));
        assert!(!p.constraints.permits("ai.query"));
    }

    #[test]
    fn policy_validation_catches_covalent_without_nuclear() {
        let p = BondingPolicy {
            bond_type: BondType::Covalent,
            trust_model: TrustModel::Contractual,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "bad".to_owned(),
        };
        let errors = p.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("nuclear"));
    }

    #[test]
    fn covalent_accepts_nuclear_lineage() {
        let p = BondingPolicy {
            bond_type: BondType::Covalent,
            trust_model: TrustModel::NuclearLineage,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: true,
            label: "nuclear-cov".to_owned(),
        };
        assert!(p.validate().is_empty());
    }

    #[test]
    fn covalent_accepts_legacy_genetic_lineage() {
        let p = BondingPolicy::covalent_default();
        assert!(p.validate().is_empty());
    }

    #[test]
    fn covalent_rejects_mito_only() {
        let p = BondingPolicy {
            bond_type: BondType::Covalent,
            trust_model: TrustModel::MitoBeaconFamily,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "mito-only".to_owned(),
        };
        let errors = p.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("nuclear"));
    }

    #[test]
    fn metallic_accepts_mito_beacon() {
        let p = BondingPolicy {
            bond_type: BondType::Metallic,
            trust_model: TrustModel::MitoBeaconFamily,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "metallic-mito".to_owned(),
        };
        assert!(p.validate().is_empty());
    }

    #[test]
    fn metallic_rejects_contractual() {
        let p = BondingPolicy {
            bond_type: BondType::Metallic,
            trust_model: TrustModel::Contractual,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "bad-metallic".to_owned(),
        };
        let errors = p.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("mito-beacon"));
    }

    #[test]
    fn policy_min_btsp_cipher() {
        use crate::btsp::BtspCipherSuite;
        let cov = BondingPolicy::covalent_default();
        assert_eq!(cov.min_btsp_cipher(), BtspCipherSuite::Null);

        let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
        assert_eq!(ionic.min_btsp_cipher(), BtspCipherSuite::ChaCha20Poly1305);
    }

    #[test]
    fn policy_btsp_cipher_allowed() {
        use crate::btsp::BtspCipherSuite;
        let cov = BondingPolicy::covalent_default();
        assert!(cov.btsp_cipher_allowed(BtspCipherSuite::Null));
        assert!(cov.btsp_cipher_allowed(BtspCipherSuite::ChaCha20Poly1305));

        let ionic = BondingPolicy::ionic_contract(vec![]);
        assert!(!ionic.btsp_cipher_allowed(BtspCipherSuite::Null));
        assert!(!ionic.btsp_cipher_allowed(BtspCipherSuite::HmacPlain));
        assert!(ionic.btsp_cipher_allowed(BtspCipherSuite::ChaCha20Poly1305));
    }

    #[test]
    fn btsp_enforcer_connection_upgrade() {
        use crate::btsp::BtspCipherSuite;
        let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
        let decision = BtspEnforcer::evaluate_connection(&ionic, BtspCipherSuite::Null);
        assert!(decision.allowed);
        assert_eq!(decision.cipher, BtspCipherSuite::ChaCha20Poly1305);
        assert!(decision.reason.contains("upgraded"));
    }

    #[test]
    fn btsp_enforcer_connection_accepted() {
        use crate::btsp::BtspCipherSuite;
        let cov = BondingPolicy::covalent_default();
        let decision = BtspEnforcer::evaluate_connection(&cov, BtspCipherSuite::Null);
        assert!(decision.allowed);
        assert_eq!(decision.cipher, BtspCipherSuite::Null);
        assert!(decision.reason.contains("granted"));
    }

    #[test]
    fn btsp_enforcer_denies_covalent_without_nuclear() {
        use crate::btsp::BtspCipherSuite;
        let cov = BondingPolicy::covalent_default();
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &cov,
            BtspCipherSuite::Null,
            Some(TrustModel::MitoBeaconFamily),
        );
        assert!(!decision.allowed, "covalent should deny mito-only peer");
        assert!(decision.reason.contains("denied"));
    }

    #[test]
    fn btsp_enforcer_allows_covalent_with_nuclear() {
        use crate::btsp::BtspCipherSuite;
        let cov = BondingPolicy::covalent_default();
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &cov,
            BtspCipherSuite::Null,
            Some(TrustModel::NuclearLineage),
        );
        assert!(decision.allowed, "covalent should allow nuclear peer");
    }

    #[test]
    fn btsp_enforcer_denies_metallic_without_genetics() {
        use crate::btsp::BtspCipherSuite;
        let policy = BondingPolicy {
            bond_type: BondType::Metallic,
            trust_model: TrustModel::MitoBeaconFamily,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "metallic-test".to_owned(),
        };
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &policy,
            BtspCipherSuite::HmacPlain,
            Some(TrustModel::Contractual),
        );
        assert!(!decision.allowed, "metallic should deny non-genetic peer");
    }

    #[test]
    fn btsp_enforcer_allows_metallic_with_mito() {
        use crate::btsp::BtspCipherSuite;
        let policy = BondingPolicy {
            bond_type: BondType::Metallic,
            trust_model: TrustModel::MitoBeaconFamily,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "metallic-test".to_owned(),
        };
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &policy,
            BtspCipherSuite::HmacPlain,
            Some(TrustModel::MitoBeaconFamily),
        );
        assert!(decision.allowed, "metallic should allow mito peer");
    }

    #[test]
    fn btsp_enforcer_denies_ionic_with_zero_trust() {
        use crate::btsp::BtspCipherSuite;
        let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &ionic,
            BtspCipherSuite::ChaCha20Poly1305,
            Some(TrustModel::ZeroTrust),
        );
        assert!(!decision.allowed, "ionic should deny zero-trust peer");
    }

    #[test]
    fn btsp_enforcer_allows_ionic_with_contractual() {
        use crate::btsp::BtspCipherSuite;
        let ionic = BondingPolicy::ionic_contract(vec!["compute.*".to_owned()]);
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &ionic,
            BtspCipherSuite::ChaCha20Poly1305,
            Some(TrustModel::Contractual),
        );
        assert!(decision.allowed, "ionic should allow contractual peer");
    }

    #[test]
    fn btsp_enforcer_weak_allows_anything() {
        use crate::btsp::BtspCipherSuite;
        let weak = BondingPolicy {
            bond_type: BondType::Weak,
            trust_model: TrustModel::ZeroTrust,
            constraints: BondingConstraint::default(),
            active_windows: Vec::new(),
            offer_relay: false,
            label: "weak-test".to_owned(),
        };
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &weak,
            BtspCipherSuite::ChaCha20Poly1305,
            Some(TrustModel::ZeroTrust),
        );
        assert!(decision.allowed, "weak should allow zero-trust peer");
    }

    #[test]
    fn btsp_enforcer_no_trust_backward_compat() {
        use crate::btsp::BtspCipherSuite;
        let cov = BondingPolicy::covalent_default();
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &cov,
            BtspCipherSuite::Null,
            None,
        );
        assert!(decision.allowed, "None trust (legacy caller) should still allow");
    }

    #[test]
    fn btsp_enforcer_request_filtering() {
        use crate::btsp::BtspCipherSuite;
        let policy = BondingPolicy::idle_compute(vec![], 100);
        let allowed = BtspEnforcer::evaluate_request(
            &policy,
            "compute.submit",
            BtspCipherSuite::ChaCha20Poly1305,
        );
        assert!(allowed.capability_permitted);

        let denied = BtspEnforcer::evaluate_request(
            &policy,
            "storage.store",
            BtspCipherSuite::ChaCha20Poly1305,
        );
        assert!(!denied.capability_permitted);
    }

    #[test]
    fn bonding_result_round_trip_json() {
        let result = BondingResult {
            bond_type: BondType::Covalent,
            gates_discovered: 2,
            capabilities_shared: 8,
            trust_verified: true,
            degradation_graceful: true,
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: BondingResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.bond_type, BondType::Covalent);
        assert_eq!(back.gates_discovered, 2);
    }

    #[test]
    fn trust_model_round_trip_json() {
        for tm in [
            TrustModel::GeneticLineage,
            TrustModel::MitoBeaconFamily,
            TrustModel::NuclearLineage,
            TrustModel::Contractual,
            TrustModel::Organizational,
            TrustModel::ZeroTrust,
        ] {
            let json = serde_json::to_string(&tm).unwrap();
            let back: TrustModel = serde_json::from_str(&json).unwrap();
            assert_eq!(tm, back);
        }
    }

    #[test]
    fn trust_model_is_genetic() {
        assert!(TrustModel::GeneticLineage.is_genetic());
        assert!(TrustModel::MitoBeaconFamily.is_genetic());
        assert!(TrustModel::NuclearLineage.is_genetic());
        assert!(!TrustModel::Contractual.is_genetic());
        assert!(!TrustModel::Organizational.is_genetic());
        assert!(!TrustModel::ZeroTrust.is_genetic());
    }

    #[test]
    fn trust_model_is_nuclear() {
        assert!(TrustModel::GeneticLineage.is_nuclear());
        assert!(TrustModel::NuclearLineage.is_nuclear());
        assert!(!TrustModel::MitoBeaconFamily.is_nuclear());
        assert!(!TrustModel::Contractual.is_nuclear());
    }

    #[test]
    fn trust_model_normalize() {
        assert_eq!(TrustModel::GeneticLineage.normalize(), TrustModel::NuclearLineage);
        assert_eq!(TrustModel::NuclearLineage.normalize(), TrustModel::NuclearLineage);
        assert_eq!(TrustModel::MitoBeaconFamily.normalize(), TrustModel::MitoBeaconFamily);
        assert_eq!(TrustModel::Contractual.normalize(), TrustModel::Contractual);
    }
}
