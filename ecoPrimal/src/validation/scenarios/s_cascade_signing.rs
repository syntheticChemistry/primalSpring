// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cascade Signing — validates cascade signing infrastructure and
//! Ed25519 key pipeline in the capability registry.

use crate::composition::CompositionContext;
use crate::composition::neural_routing::canonical_routing_table;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const PROTO_NUCLEATE: &str = include_str!("../../../../config/proto_nucleate.toml");

/// Cascade signing scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cascade-signing",
        track: Track::Sovereignty,
        tier: Tier::Rust,
        provenance_crate: "wave138a_cascade_signing",
        provenance_date: "2026-07-14",
        description: "Cascade signing — Ed25519 signing pipeline and depot verification contracts",
    },
    run,
};

/// Run cascade signing validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Ed25519 signing infrastructure");

    let ed25519_methods = [
        "crypto.sign_ed25519",
        "crypto.verify_ed25519",
        "crypto.keygen",
    ];
    for method in ed25519_methods {
        v.check_bool(
            &format!("crypto:{}", method.replace("crypto.", "")),
            REGISTRY_TOML.contains(method),
            &format!("{method} registered"),
        );
    }

    let table = canonical_routing_table();
    if let Some(entry) = table.route("crypto.sign_ed25519") {
        v.check_bool(
            "routing:ed25519_owner",
            &*entry.owner == primal_names::BEARDOG,
            &format!("crypto.sign_ed25519 → {} (expected bearDog)", entry.owner),
        );
    }

    v.section("Phase 2: Depot signing and verification contracts");

    v.check_bool(
        "depot:verify_checksums",
        PROTO_NUCLEATE.contains("verify_checksums = true"),
        "proto_nucleate.toml requires checksum verification before install",
    );
    v.check_bool(
        "depot:verify_provenance",
        PROTO_NUCLEATE.contains("verify_provenance = true"),
        "proto_nucleate.toml requires provenance.toml match",
    );
    v.check_bool(
        "depot:checksums_path",
        PROTO_NUCLEATE.contains("checksums.toml"),
        "proto_nucleate.toml references checksums.toml validation path",
    );

    v.section("Phase 3: Cascade signing method surface");

    let signing_methods = [
        "crypto.sign",
        "crypto.verify",
        "certificate.verify",
        "provenance.verify",
        "anchor.verify",
    ];
    let mut present = 0;
    for method in signing_methods {
        if REGISTRY_TOML.contains(method) {
            present += 1;
        }
        v.check_bool(
            &format!("signing:{}", method.replace('.', "_")),
            REGISTRY_TOML.contains(method),
            &format!("{method} in signing cascade"),
        );
    }

    v.check_bool(
        "signing:cascade_breadth",
        present >= 4,
        &format!(
            "{present}/{} cascade signing methods registered",
            signing_methods.len()
        ),
    );
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
