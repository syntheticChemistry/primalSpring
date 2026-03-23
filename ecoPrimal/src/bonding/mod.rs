// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bonding model validation — covalent, ionic, metallic, weak, organo-metal-salt.
//!
//! Chemistry-inspired trust levels for multi-gate coordination via Plasmodium:
//! two or more NUCLEUS instances discovering each other, sharing capabilities,
//! and degrading gracefully when gates fail.
//!
//! See `phase2/biomeOS/specs/NUCLEUS_BONDING_MODEL.md` for the full specification.

pub mod graph_metadata;
pub mod stun_tiers;

use serde::{Deserialize, Serialize};

/// Multi-gate bonding model between NUCLEUS instances.
///
/// Maps to the chemistry-inspired trust hierarchy in NUCLEUS_BONDING_MODEL.md:
/// Covalent (genetic trust) > Metallic (organizational) > Ionic (contract) > Weak (zero trust).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BondType {
    /// Shared family seed, full trust, mutual discovery via BirdSong mesh.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustModel {
    /// BearDog genetic lineage via shared `.family.seed`.
    GeneticLineage,
    /// API key, OAuth, mutual TLS — contract-verified.
    Contractual,
    /// Cluster membership, LDAP/AD — organizational boundary.
    Organizational,
    /// Assume hostile, disclose nothing.
    ZeroTrust,
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
        if self.bond_type == BondType::Covalent && self.trust_model != TrustModel::GeneticLineage {
            errors.push("covalent bonds require GeneticLineage trust model".to_owned());
        }
        if self.bond_type == BondType::Ionic && self.trust_model == TrustModel::ZeroTrust {
            errors.push("ionic bonds require at least Contractual trust".to_owned());
        }
        errors
    }
}

/// Result of a bonding validation experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondingResult {
    /// Which bonding model was tested.
    pub bond_type: BondType,
    /// Number of NestGate instances discovered during bonding.
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
    fn policy_validation_catches_inconsistency() {
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
        assert!(errors[0].contains("GeneticLineage"));
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
            TrustModel::Contractual,
            TrustModel::Organizational,
            TrustModel::ZeroTrust,
        ] {
            let json = serde_json::to_string(&tm).unwrap();
            let back: TrustModel = serde_json::from_str(&json).unwrap();
            assert_eq!(tm, back);
        }
    }
}
