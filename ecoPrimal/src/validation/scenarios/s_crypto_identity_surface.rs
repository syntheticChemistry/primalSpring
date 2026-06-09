// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Crypto & identity surface — exercises bearDog crypto.*,
//! genetic.*, auth.*, and identity.* methods when the security
//! capability is available (Wave 47 method coverage push).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "crypto-identity-surface",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave47_method_coverage",
        provenance_date: "2026-05-24",
        description: "Crypto & identity surface: sign/verify, contract, genetic ceremony, DID, sessions",
    },
    run,
};

const CRYPTO_METHODS: &[&str] = &[
    "crypto.sign_ed25519",
    "crypto.verify_ed25519",
    "crypto.encrypt_chacha20_poly1305",
    "crypto.did_from_key",
    "crypto.contract.propose",
    "crypto.contract.verify",
    "crypto.contract.countersign",
    "crypto.ionic_bond.capabilities",
];

const GENETIC_METHODS: &[&str] = &[
    "genetic.ceremony_init",
    "genetic.ceremony_finalize",
    "genetic.derive_key",
    "genetic.derive_lineage_key",
    "genetic.entropy_contribute",
    "genetic.mix_entropy",
];

const AUTH_IDENTITY_METHODS: &[&str] = &[
    "auth.issue_session",
    "auth.peer_info",
    "identity.create",
];

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — method registry");
    phase_structural(v);

    v.section("Phase 2: Live crypto probes");
    phase_live_crypto(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let registry_src = include_str!("../../../../config/capability_registry.toml");

    for method in CRYPTO_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in registry"),
        );
    }

    for method in GENETIC_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in registry"),
        );
    }

    for method in AUTH_IDENTITY_METHODS {
        v.check_bool(
            &format!("registry:{method}"),
            registry_src.contains(method),
            &format!("{method} in registry"),
        );
    }
}

fn phase_live_crypto(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("security") {
        v.check_skip(
            "live:crypto",
            "security (bearDog) not available — skipping live crypto probes",
        );
        return;
    }

    match ctx.call(
        "security",
        "crypto.ionic_bond.capabilities",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:ionic_bond_capabilities",
                resp.is_object() || resp.is_array(),
                &format!("crypto.ionic_bond.capabilities → {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:ionic_bond_capabilities",
                &format!("crypto.ionic_bond.capabilities: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:ionic_bond_capabilities",
                false,
                &format!("error: {e}"),
            );
        }
    }

    match ctx.call(
        "security",
        "crypto.did_from_key",
        serde_json::json!({ "key_type": "ed25519" }),
    ) {
        Ok(resp) => {
            let has_did = resp.get("did").is_some();
            v.check_bool(
                "live:did_from_key",
                has_did,
                &format!("crypto.did_from_key → {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:did_from_key", &format!("crypto.did_from_key: {e}"));
        }
        Err(e) => {
            v.check_bool("live:did_from_key", false, &format!("error: {e}"));
        }
    }

    match ctx.call(
        "security",
        "auth.peer_info",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            v.check_bool(
                "live:peer_info",
                resp.is_object(),
                &format!("auth.peer_info → {resp}"),
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:peer_info", &format!("auth.peer_info: {e}"));
        }
        Err(e) => {
            v.check_bool("live:peer_info", false, &format!("error: {e}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypto_identity_surface_pass() {
        let mut v = ValidationResult::new("crypto-identity-surface");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0, "had {} failures", v.failed);
    }
}
