// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: FIDO2 Entropy Ceremony — validates the hardware-backed
//! entropy pipeline: SoloKey FIDO2 → bearDog → genetic mixing → key material.
//!
//! Wave 138a: SoloKey USB on eastGate provides Tier 2 hardware entropy via
//! CTAP2 hmac-secret. This scenario validates the structural prerequisites:
//!
//! 1. FIDO2 capability domain registered and routed to bearDog
//! 2. Genetic ceremony methods exist for entropy mixing
//! 3. Entropy flow: FIDO2 discover → register → contribute → mix
//! 4. Key derivation chain: entropy → ceremony_init → derive_key → finalize
//! 5. Live FIDO2 probe when hardware is available

use crate::composition::neural_routing::canonical_routing_table;
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

const FIDO2_METHODS: &[&str] = &[
    "beardog.fido2.discover",
    "beardog.fido2.register",
    "beardog.fido2.authenticate",
    "beardog.fido2.entropy",
    "beardog.fido2.ceremony",
];

const GENETIC_CEREMONY_METHODS: &[&str] = &[
    "genetic.ceremony_init",
    "genetic.ceremony_finalize",
    "genetic.entropy_contribute",
    "genetic.mix_entropy",
    "genetic.derive_key",
    "genetic.derive_lineage_key",
];

/// FIDO2 entropy ceremony scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "fido2-entropy-ceremony",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave138a_fido2_ceremony",
        provenance_date: "2026-07-13",
        description:
            "FIDO2 entropy ceremony — SoloKey CTAP2 → bearDog → genetic mixing → key derivation",
    },
    run,
};

/// Run FIDO2 entropy ceremony validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: FIDO2 capability registry");
    phase_fido2_registry(v);

    v.section("Phase 2: FIDO2 routing");
    phase_fido2_routing(v);

    v.section("Phase 3: Genetic ceremony registry");
    phase_genetic_registry(v);

    v.section("Phase 4: Entropy flow topology");
    phase_entropy_flow(v);

    v.section("Phase 5: Live hardware probe");
    phase_live_probe(v, ctx);
}

fn phase_fido2_registry(v: &mut ValidationResult) {
    let fido2_section = REGISTRY_TOML.contains("[fido2]");
    v.check_bool(
        "fido2-ceremony:domain_declared",
        fido2_section,
        "[fido2] domain declared in capability registry",
    );

    for method in FIDO2_METHODS {
        let registered = REGISTRY_TOML.contains(method);
        v.check_bool(
            &format!("fido2-ceremony:method_{}", method_slug(method)),
            registered,
            &format!("`{method}` registered"),
        );
    }

    let owner_line = REGISTRY_TOML
        .lines()
        .skip_while(|l| !l.contains("[fido2]"))
        .find(|l| l.contains("owner"));
    let owner_is_beardog = owner_line.is_some_and(|l| l.contains("beardog"));

    v.check_bool(
        "fido2-ceremony:owner_beardog",
        owner_is_beardog,
        "FIDO2 domain owned by bearDog (crypto primal)",
    );
}

fn phase_fido2_routing(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    for method in FIDO2_METHODS {
        let route = table.route(method);
        let routed = route.is_some();
        let owner = route.map_or_else(|| "UNROUTED".to_string(), |r| r.owner.to_string());

        v.check_bool(
            &format!("fido2-ceremony:routed_{}", method_slug(method)),
            routed,
            &format!("`{method}` → {owner}"),
        );
    }
}

fn phase_genetic_registry(v: &mut ValidationResult) {
    let genetic_section = REGISTRY_TOML.contains("[genetic]");
    v.check_bool(
        "fido2-ceremony:genetic_domain",
        genetic_section,
        "[genetic] domain declared (ceremony infrastructure)",
    );

    for method in GENETIC_CEREMONY_METHODS {
        let registered = REGISTRY_TOML.contains(method);
        v.check_bool(
            &format!("fido2-ceremony:genetic_{}", method_slug(method)),
            registered,
            &format!("`{method}` registered"),
        );
    }

    let genetic_owner = REGISTRY_TOML
        .lines()
        .skip_while(|l| !l.contains("[genetic]"))
        .find(|l| l.contains("owner"));
    let genetic_is_beardog = genetic_owner.is_some_and(|l| l.contains("beardog"));

    v.check_bool(
        "fido2-ceremony:genetic_owner_beardog",
        genetic_is_beardog,
        "genetic domain owned by bearDog (key ceremony authority)",
    );
}

fn phase_entropy_flow(v: &mut ValidationResult) {
    let table = canonical_routing_table();

    let ceremony_init = table.route("genetic.ceremony_init");
    let entropy_contribute = table.route("genetic.entropy_contribute");
    let mix_entropy = table.route("genetic.mix_entropy");
    let derive_key = table.route("genetic.derive_key");
    let ceremony_finalize = table.route("genetic.ceremony_finalize");

    let flow_checks: &[(&str, &str, Option<_>)] = &[
        ("init", "ceremony_init", ceremony_init),
        ("contribute", "entropy_contribute", entropy_contribute),
        ("mix", "mix_entropy", mix_entropy),
        ("derive", "derive_key", derive_key),
        ("finalize", "ceremony_finalize", ceremony_finalize),
    ];

    for (slug, label, route) in flow_checks {
        let owner = route.as_ref().map_or("UNROUTED", |r| r.owner.as_ref());
        v.check_bool(
            &format!("fido2-ceremony:flow_{slug}_routed"),
            route.is_some(),
            &format!("{label}: {owner}"),
        );
    }

    let all_same_owner = [ceremony_init, entropy_contribute, mix_entropy, derive_key, ceremony_finalize]
        .iter()
        .filter_map(|r| r.as_ref())
        .all(|r| r.owner.as_ref() == "beardog");

    v.check_bool(
        "fido2-ceremony:flow_single_authority",
        all_same_owner,
        "entire entropy→key flow owned by single authority (bearDog)",
    );
}

fn phase_live_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let has_security = ctx.has_capability("security");
    let has_crypto = ctx.has_capability("crypto");

    v.check_bool(
        "fido2-ceremony:crypto_capability",
        has_crypto,
        &format!(
            "crypto capability: {}",
            if has_crypto { "available" } else { "offline (structural only)" }
        ),
    );

    if has_crypto {
        let discover = ctx.call(
            "crypto",
            "beardog.fido2.discover",
            serde_json::json!({}),
        );
        v.check_bool(
            "fido2-ceremony:live_fido2_discover",
            discover.is_ok(),
            &format!(
                "fido2.discover: {}",
                match &discover {
                    Ok(resp) => format!("responded ({resp})"),
                    Err(e) => format!("error: {e}"),
                }
            ),
        );
    }

    if has_security {
        let status = ctx.call(
            "security",
            "genetic.ceremony_init",
            serde_json::json!({"tier": 2, "source": "fido2"}),
        );
        v.check_bool(
            "fido2-ceremony:live_ceremony_init",
            status.is_ok(),
            &format!(
                "ceremony_init(tier=2): {}",
                if status.is_ok() { "responded" } else { "unavailable" }
            ),
        );
    }
}

fn method_slug(method: &str) -> String {
    method.replace('.', "_").replace("beardog_", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fido2_methods_in_registry() {
        for method in FIDO2_METHODS {
            assert!(
                REGISTRY_TOML.contains(method),
                "FIDO2 method `{method}` missing from capability_registry.toml"
            );
        }
    }

    #[test]
    fn genetic_methods_in_registry() {
        for method in GENETIC_CEREMONY_METHODS {
            assert!(
                REGISTRY_TOML.contains(method),
                "genetic method `{method}` missing from capability_registry.toml"
            );
        }
    }

    #[test]
    fn fido2_routes_to_beardog() {
        let table = canonical_routing_table();
        for method in FIDO2_METHODS {
            let route = table.route(method);
            assert!(route.is_some(), "`{method}` not routed");
            assert_eq!(
                route.unwrap().owner.as_ref(),
                "beardog",
                "`{method}` should route to beardog"
            );
        }
    }

    #[test]
    fn entropy_flow_all_beardog() {
        let table = canonical_routing_table();
        for method in GENETIC_CEREMONY_METHODS {
            let route = table.route(method);
            assert!(route.is_some(), "`{method}` not routed");
            assert_eq!(
                route.unwrap().owner.as_ref(),
                "beardog",
                "`{method}` should route to beardog"
            );
        }
    }
}
