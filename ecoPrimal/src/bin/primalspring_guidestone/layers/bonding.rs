// SPDX-License-Identifier: AGPL-3.0-or-later

//! Layer 5: Bonding model verification.

use primalspring::bonding::{BondType, BondingPolicy};
use primalspring::btsp;
use primalspring::validation::ValidationResult;

pub fn validate_bonding_policies(v: &mut ValidationResult) {
    for &bond in BondType::all() {
        let policy = match bond {
            BondType::Ionic => BondingPolicy::ionic_contract(vec!["compute".to_owned()]),
            _ => BondingPolicy::covalent_default(),
        };

        let errors = policy.validate();
        let detail = if errors.is_empty() {
            "policy well-formed".to_owned()
        } else {
            errors.join("; ")
        };
        v.check_bool(
            &format!("bonding:policy:{bond:?}:valid"),
            errors.is_empty(),
            &detail,
        );

        let min_cipher = btsp::min_cipher_for_bond(bond);
        v.check_bool(
            &format!("bonding:cipher:{bond:?}:min_allowed"),
            btsp::cipher_allowed(bond, min_cipher),
            &format!("min cipher {min_cipher:?} accepted for {bond:?}"),
        );

        if min_cipher.is_encrypted() {
            v.check_bool(
                &format!("bonding:cipher:{bond:?}:encrypted"),
                true,
                &format!("{bond:?} requires encrypted cipher"),
            );
        }
    }

    v.check_bool(
        "bonding:trust_ordering",
        BondType::Covalent.shares_electrons()
            && BondType::Metallic.shares_electrons()
            && !BondType::Ionic.shares_electrons()
            && !BondType::Weak.shares_electrons(),
        "Covalent+Metallic share electrons, Ionic+Weak do not",
    );

    v.check_bool(
        "bonding:ionic_metered",
        BondType::Ionic.is_metered() && !BondType::Covalent.is_metered(),
        "only Ionic is metered",
    );
}
