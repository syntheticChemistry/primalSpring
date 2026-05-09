// SPDX-License-Identifier: AGPL-3.0-or-later

//! Layer 5: Bonding model verification — structural + live ionic bond attempt.

use crate::bonding::{BondType, BondingPolicy};
use crate::btsp;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;

/// Layer 5 (structural): bonding policies, cipher minima, and trust ordering checks.
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

/// Live ionic bond negotiation — attempt `bonding.propose` via BearDog.
///
/// This pressures BearDog to implement runtime bond signing. Until
/// then, this documents the gap with a graceful skip.
///
/// Layer 5 (live): attempt `bonding.propose` on the Tower security primal.
pub fn validate_live_ionic_bond(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "bonding:live_ionic:propose",
            "BearDog not available — live ionic bond requires Tower",
        );
        return;
    }

    let propose_result = ctx.call(
        "security",
        "bonding.propose",
        serde_json::json!({
            "bond_type": "ionic",
            "requester": "primalspring",
            "target_capability": "compute",
            "scope": "tensor.*",
        }),
    );

    match propose_result {
        Ok(resp) => {
            let accepted = resp
                .get("accepted")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let bond_id = resp
                .get("bond_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("none");
            v.check_bool(
                "bonding:live_ionic:propose",
                accepted,
                &format!("bond_id={bond_id}, accepted={accepted}"),
            );
        }
        Err(e) if e.is_method_not_found() => {
            v.check_skip(
                "bonding:live_ionic:propose",
                &format!(
                    "UPSTREAM GAP: bonding.propose not implemented in BearDog — \
                     runtime bond signing deferred ({e})"
                ),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "bonding:live_ionic:propose",
                &format!("security not reachable: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "bonding:live_ionic:propose",
                false,
                &format!("bonding.propose failed: {e}"),
            );
        }
    }
}
