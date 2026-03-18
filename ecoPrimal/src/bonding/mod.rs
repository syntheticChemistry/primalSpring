// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bonding model validation — covalent, ionic, weak, organo-metal-salt.
//!
//! Tests multi-gate coordination via Plasmodium: two or more NUCLEUS
//! instances discovering each other, sharing capabilities, and degrading
//! gracefully when gates fail.

use serde::{Deserialize, Serialize};

/// Multi-gate bonding model between NUCLEUS instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BondType {
    /// Shared family seed, full trust, mutual discovery.
    Covalent,
    /// Cross-family, limited capability sharing.
    Ionic,
    /// Temporary association, no persistent trust.
    Weak,
    /// Multi-family complex with catalytic bridge.
    OrganoMetalSalt,
}

impl BondType {
    /// Human-readable description of this bond type.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Covalent => "Shared family seed, full trust, mutual discovery",
            Self::Ionic => "Cross-family, limited capability sharing",
            Self::Weak => "Temporary association, no persistent trust",
            Self::OrganoMetalSalt => "Multi-family complex with catalytic bridge",
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bond_types_have_descriptions() {
        let types = [
            BondType::Covalent,
            BondType::Ionic,
            BondType::Weak,
            BondType::OrganoMetalSalt,
        ];
        for bt in types {
            assert!(!bt.description().is_empty());
        }
    }

    #[test]
    fn bond_type_round_trip_json() {
        for bt in [
            BondType::Covalent,
            BondType::Ionic,
            BondType::Weak,
            BondType::OrganoMetalSalt,
        ] {
            let json = serde_json::to_string(&bt).unwrap();
            let back: BondType = serde_json::from_str(&json).unwrap();
            assert_eq!(bt, back);
        }
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
}
