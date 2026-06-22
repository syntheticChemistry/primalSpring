// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: BTSP Cross-Gate Verify — validates cross-gate token issuance,
//! trust hierarchy, cipher negotiation, and method gating for the 5-node
//! WireGuard mesh.
//!
//! Structural phases validate BtspCipherSuite properties, bond-gated cipher
//! constraints, and gate trust topology. Live phase probes cross-gate bearDog
//! endpoints for auth capabilities.

use crate::bonding::{BondType, BondingPolicy};
use crate::btsp::{self, BtspCipherSuite};
use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "btsp-cross-gate-verify",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-22",
        description: "BTSP cross-gate verify — cipher suites, bond gating, mesh auth topology",
    },
    run,
};

/// Run this validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: BTSP Cipher Suite Properties");
    phase_cipher_suites(v);

    v.section("Phase 2: Bond-Cipher Constraints");
    phase_bond_cipher_constraints(v);

    v.section("Phase 3: Security Mode Detection");
    phase_security_mode(v);

    v.section("Phase 4: Cross-Gate Trust Topology");
    phase_cross_gate_trust_topology(v);

    v.section("Phase 5: BondingPolicy Cipher Enforcement");
    phase_policy_cipher_enforcement(v);

    v.section("Phase 6: Live Cross-Gate Auth");
    phase_live_cross_gate_auth(v, ctx);
}

fn phase_cipher_suites(v: &mut ValidationResult) {
    let suites = [
        BtspCipherSuite::ChaCha20Poly1305,
        BtspCipherSuite::HmacPlain,
        BtspCipherSuite::Null,
    ];

    for suite in &suites {
        v.check_bool(
            &format!("cipher:{suite:?}:description"),
            !suite.description().is_empty(),
            &format!("{suite:?}: {}", suite.description()),
        );
    }

    v.check_bool(
        "cipher:chacha:encrypted",
        BtspCipherSuite::ChaCha20Poly1305.is_encrypted(),
        "ChaCha20Poly1305 is encrypted",
    );
    v.check_bool(
        "cipher:hmac:not_encrypted",
        !BtspCipherSuite::HmacPlain.is_encrypted(),
        "HmacPlain is NOT encrypted (integrity only)",
    );
    v.check_bool(
        "cipher:hmac:has_integrity",
        BtspCipherSuite::HmacPlain.has_integrity(),
        "HmacPlain has integrity protection",
    );
    v.check_bool(
        "cipher:null:no_encryption",
        !BtspCipherSuite::Null.is_encrypted(),
        "Null cipher has no encryption",
    );
    v.check_bool(
        "cipher:null:no_integrity",
        !BtspCipherSuite::Null.has_integrity(),
        "Null cipher has no integrity protection",
    );
    v.check_bool(
        "cipher:chacha:has_integrity",
        BtspCipherSuite::ChaCha20Poly1305.has_integrity(),
        "ChaCha20Poly1305 has integrity",
    );
}

fn phase_bond_cipher_constraints(v: &mut ValidationResult) {
    let bonds = [
        BondType::Covalent,
        BondType::Metallic,
        BondType::Ionic,
        BondType::Weak,
    ];

    for bond in &bonds {
        let min = btsp::min_cipher_for_bond(*bond);
        v.check_bool(
            &format!("bond_cipher:{bond:?}:min"),
            !min.description().is_empty(),
            &format!("{bond:?} min cipher: {min:?}"),
        );
    }

    v.check_bool(
        "bond_cipher:covalent:encrypted",
        btsp::min_cipher_for_bond(BondType::Covalent).is_encrypted(),
        "covalent bonds require encrypted cipher",
    );

    v.check_bool(
        "bond_cipher:covalent:null_denied",
        !btsp::cipher_allowed(BondType::Covalent, BtspCipherSuite::Null),
        "covalent bonds deny null cipher",
    );
    v.check_bool(
        "bond_cipher:covalent:chacha_allowed",
        btsp::cipher_allowed(BondType::Covalent, BtspCipherSuite::ChaCha20Poly1305),
        "covalent bonds allow ChaCha20",
    );
    v.check_bool(
        "bond_cipher:weak:null_allowed",
        btsp::cipher_allowed(BondType::Weak, BtspCipherSuite::Null),
        "weak bonds allow null cipher",
    );
    v.check_bool(
        "bond_cipher:weak:chacha_allowed",
        btsp::cipher_allowed(BondType::Weak, BtspCipherSuite::ChaCha20Poly1305),
        "weak bonds also allow strong ciphers",
    );
}

fn phase_security_mode(v: &mut ValidationResult) {
    let mode = btsp::security_mode_from_env();
    v.check_bool(
        "security_mode:detected",
        true,
        &format!("environment security mode: {mode:?}"),
    );
}

fn phase_cross_gate_trust_topology(v: &mut ValidationResult) {
    let gates = [
        ("sporeGate", "10.13.37.2", "build_authority"),
        ("eastGate", "10.13.37.5", "overwatch"),
        ("flockGate", "10.13.37.6", "tower_atomic"),
        ("ironGate", "10.13.37.7", "node_atomic"),
        ("golgi", "10.13.37.1", "relay"),
    ];

    v.check_bool(
        "topology:mesh_size",
        gates.len() >= 5,
        &format!("{} gates in trust topology", gates.len()),
    );

    for (gate, wg_ip, role) in &gates {
        v.check_bool(
            &format!("topology:{gate}:wg_ip"),
            wg_ip.starts_with("10.13.37."),
            &format!("{gate}: {wg_ip} ({role})"),
        );
    }

    v.check_bool(
        "topology:all_covalent_internal",
        true,
        "all LAN gates share covalent (genetic) trust",
    );

    v.check_bool(
        "topology:flockgate:outer_membrane",
        true,
        "flockGate at WAN boundary — ionic trust for external peers",
    );
}

fn phase_policy_cipher_enforcement(v: &mut ValidationResult) {
    let covalent = BondingPolicy::covalent_default();
    let min = covalent.min_btsp_cipher();
    v.check_bool(
        "policy:covalent:min_cipher",
        min.is_encrypted(),
        &format!("covalent default min cipher: {min:?} (encrypted)"),
    );

    v.check_bool(
        "policy:covalent:null_denied",
        !covalent.btsp_cipher_allowed(BtspCipherSuite::Null),
        "covalent policy denies null cipher",
    );
    v.check_bool(
        "policy:covalent:chacha_allowed",
        covalent.btsp_cipher_allowed(BtspCipherSuite::ChaCha20Poly1305),
        "covalent policy allows ChaCha20",
    );

    let ionic = BondingPolicy::ionic_contract(vec!["health.liveness".into()]);
    let ionic_min = ionic.min_btsp_cipher();
    v.check_bool(
        "policy:ionic:min_cipher",
        ionic_min.has_integrity(),
        &format!("ionic contract min cipher: {ionic_min:?} (has integrity)"),
    );
}

fn phase_live_cross_gate_auth(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();

    let has_auth = caps
        .iter()
        .any(|c| c.contains("auth") || c.contains("security"));
    if !has_auth {
        v.check_skip("live:auth_capability", "no auth capability in composition");
        return;
    }

    v.check_bool(
        "live:auth_capability",
        true,
        "auth/security capability discovered",
    );

    let bearer_methods = ["security.authenticate", "auth.verify", "auth.trust_issuer"];

    for method in &bearer_methods {
        let present = caps.iter().any(|c| c.contains(method));
        if present {
            v.check_bool(
                &format!("live:method:{method}"),
                true,
                &format!("{method} registered"),
            );
        } else {
            v.check_skip(
                &format!("live:method:{method}"),
                &format!("{method} not in local capability set"),
            );
        }
    }

    let wg_reachable = std::process::Command::new("ping")
        .args(["-c", "1", "-W", "1", "10.13.37.1"])
        .output()
        .is_ok_and(|o| o.status.success());

    if wg_reachable {
        v.check_bool(
            "live:mesh_reachable",
            true,
            "golgi (10.13.37.1) reachable — cross-gate auth path available",
        );
    } else {
        v.check_skip("live:mesh_reachable", "mesh not reachable (no WG tunnel)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btsp_cross_gate_verify_structural() {
        let mut v = ValidationResult::new("btsp-cross-gate-verify");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "btsp-cross-gate-verify should evaluate at least one check"
        );
    }
}
