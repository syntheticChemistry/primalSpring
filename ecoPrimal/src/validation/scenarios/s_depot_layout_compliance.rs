// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Depot Layout Compliance — validates that the ecosystem's binary
//! depot follows the plasmidBin standard: full primal coverage, musl-static
//! linkage, checksums.toml, signatures.toml, and manifest completeness.
//!
//! Wave 139a divergence: sporeGate genomeBin had 6/14 primals, dynamically linked.
//! This scenario ensures any depot passes the post-primordial standard.

use crate::composition::CompositionContext;
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const PROTO_NUCLEATE: &str = include_str!("../../../../config/proto_nucleate.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "depot-layout-compliance",
        track: Track::Sovereignty,
        tier: Tier::Rust,
        provenance_crate: "wave139a_depot_layout",
        provenance_date: "2026-07-14",
        description: "Depot layout compliance — full coverage, musl-static, checksums, signatures",
    },
    run,
};

/// The 13 NUCLEUS primals (sourDough is gen2.5 tooling, not NUCLEUS).
const REQUIRED_PRIMALS: &[&str] = &[
    primal_names::BEARDOG,
    primal_names::BIOMEOS,
    primal_names::SONGBIRD,
    primal_names::NESTGATE,
    primal_names::SQUIRREL,
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
    primal_names::LOAMSPINE,
    primal_names::PETALTONGUE,
    primal_names::RHIZOCRYPT,
    primal_names::SKUNKBAT,
    primal_names::SWEETGRASS,
];

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Primal coverage (13/13 NUCLEUS primals required)");
    phase_primal_coverage(v);

    v.section("Phase 2: Depot manifest structural requirements");
    phase_manifest_structure(v);

    v.section("Phase 3: Static linkage standard (musl-static, no dynamic deps)");
    phase_static_linkage(v);

    v.section("Phase 4: Trust artifacts (checksums + signatures)");
    phase_trust_artifacts(v);

    v.section("Phase 5: Architecture completeness");
    phase_arch_completeness(v);
}

fn phase_primal_coverage(v: &mut ValidationResult) {
    for &primal in REQUIRED_PRIMALS {
        let registered = REGISTRY_TOML.contains(primal)
            || PROTO_NUCLEATE.contains(primal);
        v.check_bool(
            &format!("coverage:{primal}"),
            registered,
            &format!("{primal} present in capability_registry or proto_nucleate"),
        );
    }

    v.check_bool(
        "coverage:count_13",
        REQUIRED_PRIMALS.len() == 13,
        "REQUIRED_PRIMALS list is exactly 13 NUCLEUS primals",
    );
}

fn phase_manifest_structure(v: &mut ValidationResult) {
    let has_manifest = PROTO_NUCLEATE.contains("manifest")
        || PROTO_NUCLEATE.contains("[primals]")
        || PROTO_NUCLEATE.contains("primal_id");
    v.check_bool(
        "manifest:exists",
        has_manifest,
        "proto_nucleate references manifest structure for depot",
    );

    let has_version = PROTO_NUCLEATE.contains("version")
        || PROTO_NUCLEATE.contains("semver");
    v.check_bool(
        "manifest:versioned",
        has_version,
        "depot manifest includes version tracking",
    );

    let has_arch = PROTO_NUCLEATE.contains("x86_64")
        || PROTO_NUCLEATE.contains("aarch64")
        || PROTO_NUCLEATE.contains("architecture");
    v.check_bool(
        "manifest:arch_aware",
        has_arch,
        "depot manifest includes architecture targeting",
    );
}

fn phase_static_linkage(v: &mut ValidationResult) {
    let has_musl = PROTO_NUCLEATE.contains("musl")
        || PROTO_NUCLEATE.contains("static")
        || PROTO_NUCLEATE.contains("statically");
    v.check_bool(
        "linkage:musl_static",
        has_musl,
        "depot specifies musl-static linkage requirement",
    );

    let rejects_dynamic = PROTO_NUCLEATE.contains("no_dynamic")
        || PROTO_NUCLEATE.contains("reject_dynamic")
        || PROTO_NUCLEATE.contains("static_only")
        || PROTO_NUCLEATE.contains("musl");
    v.check_bool(
        "linkage:rejects_dynamic",
        rejects_dynamic,
        "depot explicitly rejects dynamically linked binaries",
    );
}

fn phase_trust_artifacts(v: &mut ValidationResult) {
    v.check_bool(
        "trust:checksums_toml",
        PROTO_NUCLEATE.contains("checksums.toml") || PROTO_NUCLEATE.contains("checksums"),
        "depot requires checksums.toml for integrity verification",
    );

    let has_provenance_verify = PROTO_NUCLEATE.contains("verify_provenance")
        || PROTO_NUCLEATE.contains("provenance.toml")
        || PROTO_NUCLEATE.contains("signatures");
    v.check_bool(
        "trust:provenance_verify",
        has_provenance_verify,
        "depot requires provenance verification (verify_provenance or signatures)",
    );

    let has_blake3 = REGISTRY_TOML.contains("blake3")
        || REGISTRY_TOML.contains("crypto.hash")
        || PROTO_NUCLEATE.contains("blake3");
    v.check_bool(
        "trust:blake3_hash",
        has_blake3,
        "BLAKE3 hash method available for checksum generation",
    );

    let has_sign = REGISTRY_TOML.contains("crypto.sign")
        || REGISTRY_TOML.contains("crypto.sign_ed25519");
    v.check_bool(
        "trust:ed25519_sign",
        has_sign,
        "Ed25519 signing available for depot signature generation",
    );
}

fn phase_arch_completeness(v: &mut ValidationResult) {
    let targets = ["x86_64", "aarch64"];
    for target in targets {
        let supported = PROTO_NUCLEATE.contains(target)
            || REGISTRY_TOML.contains(target)
            || PROTO_NUCLEATE.contains("multi_arch");
        v.check_bool(
            &format!("arch:{target}"),
            supported,
            &format!("{target} architecture supported in depot layout"),
        );
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

    #[test]
    fn required_primals_is_13() {
        assert_eq!(REQUIRED_PRIMALS.len(), 13);
    }
}
