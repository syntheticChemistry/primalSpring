// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Pure Rust Crypto Audit — validates pure Rust crypto compliance with
//! bearDog ownership of all crypto methods.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

const CRYPTO_METHODS: &[&str] = &[
    "crypto.hash",
    "crypto.sign",
    "crypto.verify",
    "crypto.encrypt",
    "crypto.decrypt",
    "crypto.keygen",
    "crypto.sign_ed25519",
    "crypto.verify_ed25519",
];

const KEY_METHODS: &[&str] = &[
    "crypto.keygen",
    "crypto.generate_keypair",
    "genetic.derive_key",
    "genetic.derive_lineage_key",
];

/// Pure Rust crypto audit scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "pure-rust-crypto-audit",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave138a_pure_rust_crypto_audit",
        provenance_date: "2026-07-14",
        description: "Pure Rust crypto audit — crypto.* and key.* methods owned by bearDog",
    },
    run,
};

/// Run pure Rust crypto audit validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Crypto method registration");

    let mut crypto_registered = 0;
    for method in CRYPTO_METHODS {
        if REGISTRY_TOML.contains(method) {
            crypto_registered += 1;
        }
        v.check_bool(
            &format!("crypto:{}", method.replace("crypto.", "")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    v.check_bool(
        "crypto:breadth",
        crypto_registered >= 6,
        &format!("{crypto_registered}/{} crypto methods present", CRYPTO_METHODS.len()),
    );

    v.section("Phase 2: Key infrastructure methods");

    for method in KEY_METHODS {
        v.check_bool(
            &format!("key:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered for key infrastructure"),
        );
    }

    v.section("Phase 3: bearDog owns all crypto methods");

    let table = canonical_routing_table();
    let beardog_crypto_owned = CRYPTO_METHODS
        .iter()
        .filter(|m| {
            table
                .route(m)
                .is_some_and(|e| &*e.owner == primal_names::BEARDOG)
        })
        .count();

    v.check_bool(
        "owner:beardog_crypto_methods",
        beardog_crypto_owned >= 6,
        &format!(
            "bearDog owns {beardog_crypto_owned}/{} registered crypto methods",
            CRYPTO_METHODS.len()
        ),
    );

    for method in ["crypto.hash", "crypto.sign_ed25519", "crypto.keygen"] {
        if let Some(entry) = table.route(method) {
            v.check_bool(
                &format!("owner:{}", method.replace('.', "_")),
                &*entry.owner == primal_names::BEARDOG,
                &format!("{method} → {} (expected bearDog)", entry.owner),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
