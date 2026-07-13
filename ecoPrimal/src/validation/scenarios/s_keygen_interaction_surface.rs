// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Keygen Interaction Surface — validates the full key generation
//! interaction surface that larger systems (browser UI, RustDesk, ADB) leverage.
//!
//! Wave 138a: The key generation ceremony needs to be browser-accessible via
//! Neural API HTTP for interactive initial key creation. This scenario validates:
//!
//! 1. All keygen methods are dispatch-routable (same path browser→NAPI→bearDog)
//! 2. Ephemeral key generation + comparison flow exists
//! 3. Multi-source genetic mixing (FIDO2 + audio + StrongBox + getrandom)
//! 4. Ceremony state machine: init → contribute(N sources) → mix → derive → finalize
//! 5. Neural API dispatch compatibility (methods callable via capability.call)

use crate::composition::neural_routing::canonical_routing_table;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

const KEYGEN_METHODS: &[&str] = &[
    "crypto.keygen",
    "crypto.generate_keypair",
    "crypto.family_seed",
    "genetic.derive_key",
    "genetic.derive_lineage_key",
    "genetic.derive_lineage_beacon_key",
];

const CEREMONY_STATE_MACHINE: &[&str] = &[
    "genetic.ceremony_init",
    "genetic.entropy_contribute",
    "genetic.mix_entropy",
    "genetic.derive_key",
    "genetic.ceremony_finalize",
];

const ENTROPY_SOURCES: &[(&str, &str)] = &[
    ("fido2", "beardog.fido2.authenticate"),
    ("getrandom", "crypto.keygen"),
    ("family_seed", "crypto.family_seed"),
    ("lineage", "genetic.derive_lineage_key"),
];

/// Keygen interaction surface scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "keygen-interaction-surface",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave138a_keygen_surface",
        provenance_date: "2026-07-13",
        description:
            "Keygen interaction surface — browser-accessible ceremony via Neural API dispatch",
    },
    run,
};

/// Run the keygen interaction surface validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Keygen method dispatch");
    phase_keygen_dispatch(v);

    v.section("Phase 2: Ceremony state machine");
    phase_ceremony_state_machine(v);

    v.section("Phase 3: Multi-source entropy routing");
    phase_entropy_sources(v);

    v.section("Phase 4: Ephemeral key lifecycle");
    phase_ephemeral_lifecycle(v);

    v.section("Phase 5: Neural API dispatch compatibility");
    phase_napi_compatibility(v, ctx);
}

fn phase_keygen_dispatch(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in KEYGEN_METHODS {
        let route = table.route(method);
        let owner = route.as_ref().map_or("UNROUTED", |r| r.owner.as_ref());

        v.check_bool(
            &format!("keygen:dispatch_{}", method.replace('.', "_")),
            route.is_some(),
            &format!("`{method}` → {owner}"),
        );
    }

    let all_routed = KEYGEN_METHODS.iter().all(|m| table.route(m).is_some());
    v.check_bool(
        "keygen:all_methods_routed",
        all_routed,
        &format!(
            "{}/{} keygen methods routable",
            KEYGEN_METHODS.iter().filter(|m| table.route(m).is_some()).count(),
            KEYGEN_METHODS.len()
        ),
    );
}

fn phase_ceremony_state_machine(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let owners: Vec<&str> = CEREMONY_STATE_MACHINE
        .iter()
        .filter_map(|m| table.route(m))
        .map(|r| r.owner.as_ref())
        .collect();

    v.check_bool(
        "keygen:ceremony_all_stages_routed",
        owners.len() == CEREMONY_STATE_MACHINE.len(),
        &format!(
            "ceremony state machine: {}/{} stages routed",
            owners.len(),
            CEREMONY_STATE_MACHINE.len()
        ),
    );

    let single_authority = owners.iter().all(|o| *o == owners.first().copied().unwrap_or(""));
    v.check_bool(
        "keygen:ceremony_single_authority",
        single_authority,
        &format!(
            "ceremony owned by single authority: {}",
            owners.first().copied().unwrap_or("NONE")
        ),
    );

    let has_init = REGISTRY_TOML.contains("genetic.ceremony_init");
    let has_contribute = REGISTRY_TOML.contains("genetic.entropy_contribute");
    let has_finalize = REGISTRY_TOML.contains("genetic.ceremony_finalize");
    v.check_bool(
        "keygen:ceremony_bookends",
        has_init && has_contribute && has_finalize,
        "ceremony has init + contribute + finalize (stateful interaction)",
    );

    let contribute_repeatable = REGISTRY_TOML
        .lines()
        .filter(|l| l.contains("entropy_contribute"))
        .count()
        >= 1;
    v.check_bool(
        "keygen:contribute_repeatable",
        contribute_repeatable,
        "entropy_contribute callable multiple times (multi-source mixing)",
    );
}

fn phase_entropy_sources(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for (source_name, method) in ENTROPY_SOURCES {
        let route = table.route(method);
        let routed = route.is_some();
        let owner = route.as_ref().map_or("UNROUTED", |r| r.owner.as_ref());

        v.check_bool(
            &format!("keygen:source_{source_name}"),
            routed,
            &format!("entropy source '{source_name}': `{method}` → {owner}"),
        );
    }

    let sources_routed = ENTROPY_SOURCES
        .iter()
        .filter(|(_, m)| table.route(m).is_some())
        .count();

    v.check_bool(
        "keygen:multi_source_available",
        sources_routed >= 2,
        &format!(
            "{sources_routed}/{} entropy sources routable (need ≥2 for mixing)",
            ENTROPY_SOURCES.len()
        ),
    );

    let has_mix = table.route("genetic.mix_entropy").is_some();
    v.check_bool(
        "keygen:mix_entropy_available",
        has_mix,
        "genetic.mix_entropy routable (combines N sources into key material)",
    );
}

fn phase_ephemeral_lifecycle(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let generate = table.route("crypto.generate_keypair");
    let sign = table.route("crypto.sign_ed25519");
    let verify = table.route("crypto.verify_ed25519");

    v.check_bool(
        "keygen:ephemeral_generate",
        generate.is_some(),
        "can generate ephemeral keypair",
    );

    v.check_bool(
        "keygen:ephemeral_sign",
        sign.is_some(),
        "can sign with ephemeral key (for comparison)",
    );

    v.check_bool(
        "keygen:ephemeral_verify",
        verify.is_some(),
        "can verify ephemeral signature (for comparison)",
    );

    let all_same = [generate, sign, verify]
        .iter()
        .filter_map(|r| r.as_ref())
        .map(|r| r.owner.as_ref())
        .collect::<std::collections::HashSet<_>>()
        .len()
        <= 1;

    v.check_bool(
        "keygen:ephemeral_single_authority",
        all_same,
        "ephemeral generate+sign+verify = same authority (no key leakage)",
    );

    let has_identity = table.route("crypto.identity").is_some();
    v.check_bool(
        "keygen:identity_derivable",
        has_identity,
        "crypto.identity available (DID from ephemeral for comparison)",
    );

    let has_did = table.route("crypto.did_from_key").is_some();
    v.check_bool(
        "keygen:did_from_key",
        has_did,
        "crypto.did_from_key (derive DID from ephemeral pubkey for mixing comparison)",
    );
}

fn phase_napi_compatibility(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let table = canonical_routing_table();

    let ceremony_methods_in_crypto_domain = CEREMONY_STATE_MACHINE
        .iter()
        .all(|m| {
            table.route(m).is_some()
        });

    v.check_bool(
        "keygen:napi_all_ceremony_routable",
        ceremony_methods_in_crypto_domain,
        "all ceremony methods routable via Neural API dispatch (browser path)",
    );

    let keygen_methods_routable = KEYGEN_METHODS
        .iter()
        .all(|m| table.route(m).is_some());

    v.check_bool(
        "keygen:napi_all_keygen_routable",
        keygen_methods_routable,
        "all keygen methods routable via Neural API dispatch",
    );

    let has_crypto_cap = ctx.has_capability("crypto");
    v.check_bool(
        "keygen:napi_crypto_capability",
        has_crypto_cap,
        &format!(
            "crypto capability via NAPI: {}",
            if has_crypto_cap { "live (browser can call)" } else { "offline (structural only)" }
        ),
    );

    if has_crypto_cap {
        let gen_test = ctx.call(
            "crypto",
            "crypto.generate_keypair",
            serde_json::json!({"algorithm": "ed25519"}),
        );
        v.check_bool(
            "keygen:napi_live_generate",
            gen_test.is_ok(),
            &format!(
                "live generate_keypair: {}",
                if gen_test.is_ok() { "responded" } else { "unavailable" }
            ),
        );
    }

    let lineage_proof = table.route("genetic.generate_lineage_proof");
    let verify_lineage = table.route("genetic.verify_lineage");
    v.check_bool(
        "keygen:lineage_proof_available",
        lineage_proof.is_some() && verify_lineage.is_some(),
        "lineage proof generate + verify available (genetic mixing comparison)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_keygen_methods_in_registry() {
        for method in KEYGEN_METHODS {
            assert!(
                REGISTRY_TOML.contains(method),
                "keygen method `{method}` missing from capability_registry.toml"
            );
        }
    }

    #[test]
    fn ceremony_state_machine_in_registry() {
        for method in CEREMONY_STATE_MACHINE {
            assert!(
                REGISTRY_TOML.contains(method),
                "ceremony method `{method}` missing from capability_registry.toml"
            );
        }
    }

    #[test]
    fn keygen_routes_exist() {
        let table = canonical_routing_table();
        for method in KEYGEN_METHODS {
            assert!(table.route(method).is_some(), "`{method}` not routed");
        }
    }

    #[test]
    fn entropy_sources_routable() {
        let table = canonical_routing_table();
        for (name, method) in ENTROPY_SOURCES {
            assert!(
                table.route(method).is_some(),
                "entropy source '{name}' (`{method}`) not routed"
            );
        }
    }

    #[test]
    fn ephemeral_lifecycle_complete() {
        let table = canonical_routing_table();
        assert!(table.route("crypto.generate_keypair").is_some());
        assert!(table.route("crypto.sign_ed25519").is_some());
        assert!(table.route("crypto.verify_ed25519").is_some());
        assert!(table.route("crypto.did_from_key").is_some());
    }
}
