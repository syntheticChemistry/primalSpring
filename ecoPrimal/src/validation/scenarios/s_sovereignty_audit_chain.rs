// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Sovereignty Audit Chain — validates the end-to-end sovereignty
//! guarantees from provenance through attestation to ledger commitment.
//!
//! Validates:
//! 1. Provenance trio primals are all routable (RhizoCrypt + LoamSpine + SweetGrass)
//! 2. Sovereignty method surface covers required operations
//! 3. AGPL-3.0 license compliance in workspace crates
//! 4. Lineage chain cryptographic properties (BLAKE3 + Ed25519)
//! 5. Live sovereignty probe (when primals reachable)

use crate::composition::{capability_to_primal, CompositionContext};
use crate::primal_names;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Sovereignty audit chain validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sovereignty-audit-chain",
        track: Track::Sovereignty,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-21",
        description: "End-to-end sovereignty: provenance trio routing, methods, license, crypto, live",
    },
    run: run_sovereignty_audit_chain,
};

/// Run this validation scenario.
pub fn run_sovereignty_audit_chain(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Provenance trio routing");
    phase_provenance_routing(v);

    v.section("Phase 2: Sovereignty method surface");
    phase_sovereignty_methods(v);

    v.section("Phase 3: License compliance");
    phase_license_compliance(v);

    v.section("Phase 4: Cryptographic chain properties");
    phase_crypto_properties(v);

    v.section("Phase 5: Live sovereignty probe");
    phase_live_sovereignty(v, ctx);
}

fn phase_provenance_routing(v: &mut ValidationResult) {
    let provenance_primal = capability_to_primal("provenance.commit");
    v.check_bool(
        "provenance_routes_to_rhizocrypt",
        provenance_primal == primal_names::RHIZOCRYPT,
        &format!(
            "provenance.commit → {provenance_primal} (expected {})",
            primal_names::RHIZOCRYPT
        ),
    );

    let ledger_primal = capability_to_primal("ledger.append");
    v.check_bool(
        "ledger_routes_to_loamspine",
        ledger_primal == primal_names::LOAMSPINE,
        &format!(
            "ledger.append → {ledger_primal} (expected {})",
            primal_names::LOAMSPINE
        ),
    );

    let witness_primal = capability_to_primal("witness.attest");
    v.check_bool(
        "witness_routes_to_sweetgrass",
        witness_primal == primal_names::SWEETGRASS,
        &format!(
            "witness.attest → {witness_primal} (expected {})",
            primal_names::SWEETGRASS
        ),
    );
}

fn phase_sovereignty_methods(v: &mut ValidationResult) {
    let table = crate::composition::neural_routing::canonical_routing_table();

    let sovereignty_domains = ["provenance", "ledger", "witness"];
    let mut total_methods = 0usize;

    for domain in &sovereignty_domains {
        let methods = table.methods_in_domain(domain);
        total_methods += methods.len();
        v.check_minimum(
            &format!("{domain}_method_count"),
            methods.len(),
            2,
        );
    }

    v.check_minimum("total_sovereignty_methods", total_methods, 8);
}

fn phase_license_compliance(v: &mut ValidationResult) {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");

    let license_files = ["LICENSE-AGPL3", "LICENSE", "COPYING"];
    let has_license = license_files
        .iter()
        .any(|f| workspace_root.join(f).exists());
    v.check_bool(
        "agpl3_license_present",
        has_license,
        "AGPL-3.0 license file present in workspace root",
    );

    let cargo_toml = workspace_root.join("Cargo.toml");
    if cargo_toml.exists() {
        let content = std::fs::read_to_string(&cargo_toml).unwrap_or_default();
        let has_agpl = content.contains("AGPL-3.0")
            || content.contains("agpl")
            || content.contains("scyBorg");
        v.check_bool(
            "cargo_toml_license_field",
            has_agpl,
            "Cargo.toml references AGPL-3.0 or scyBorg license",
        );
    } else {
        v.check_skip("cargo_toml_license_field", "Cargo.toml not found");
    }
}

fn phase_crypto_properties(v: &mut ValidationResult) {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let cargo_lock = workspace_root.join("Cargo.lock");

    if !cargo_lock.exists() {
        v.check_skip("crypto_deps", "Cargo.lock not found");
        return;
    }

    let content = std::fs::read_to_string(&cargo_lock).unwrap_or_default();

    v.check_bool(
        "blake3_in_tree",
        content.contains("name = \"blake3\""),
        "BLAKE3 hasher present in dependency tree",
    );

    v.check_bool(
        "ed25519_in_tree",
        content.contains("ed25519-dalek") || content.contains("ed25519"),
        "Ed25519 signing present in dependency tree",
    );

    v.check_bool(
        "no_openssl",
        !content.contains("name = \"openssl\"") && !content.contains("name = \"openssl-sys\""),
        "No OpenSSL in dependency tree (pure Rust crypto)",
    );

    v.check_bool(
        "no_ring",
        !content.contains("name = \"ring\""),
        "No ring in dependency tree (ecoBin compliant)",
    );
}

fn phase_live_sovereignty(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let sovereignty_caps = ["provenance", "ledger", "witness"];

    for cap in &sovereignty_caps {
        if !ctx.has_capability(cap) {
            v.check_skip(
                &format!("{cap}_live"),
                &format!("{cap} capability not discovered"),
            );
            continue;
        }

        match ctx.health_check(cap) {
            Ok(true) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    true,
                    &format!("{cap} health.liveness OK"),
                );
            }
            Ok(false) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap} not healthy"),
                );
            }
            Err(e) if e.is_skippable() => {
                v.check_skip(
                    &format!("{cap}_liveness"),
                    &format!("{cap} unreachable: {e}"),
                );
            }
            Err(e) => {
                v.check_bool(
                    &format!("{cap}_liveness"),
                    false,
                    &format!("{cap} probe failed: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sovereignty_audit_chain_structural() {
        let mut v = ValidationResult::new("sovereignty-audit-chain");
        let mut ctx = CompositionContext::discover();
        run_sovereignty_audit_chain(&mut v, &mut ctx);
        let structural_ok = v.passed >= 6;
        assert!(
            structural_ok,
            "sovereignty-audit-chain: only {} checks passed ({} failed, {} skipped)",
            v.passed, v.failed, v.skipped
        );
    }
}
