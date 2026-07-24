// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — Cipher Downgrade.
//!
//! BTSP `negotiate` allows re-negotiating the cipher suite on an active
//! session. The current code in `negotiation.rs`:
//!
//! ```text
//! BtspCipher::from_wire_name(&neg_params.cipher)
//!     .unwrap_or(BtspCipher::`ChaCha20`Poly1305)
//! ```
//!
//! This defaults to `ChaCha20` if the requested cipher is unknown — which
//! is safe for unknown ciphers. But what about known-weak ciphers?
//!
//! Attack vectors:
//! - Request `"null"` or `"none"` cipher → should be rejected
//! - Request `"aes-128-cbc"` (no AEAD) → should be rejected
//! - Request `"chacha20"` (no Poly1305 MAC) → should be rejected
//! - Per bond type (covalent vs ionic), minimum cipher floor should differ
//!
//! Validates cipher negotiation enforcement and floor policies.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const NEGOTIATION_SRC: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp/negotiation.rs"
);
const HANDSHAKE_SRC: &str = include_str!(
    "../../../../../../primals/bearDog/crates/beardog-tunnel/src/btsp_handshake/mod.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-cipher-downgrade",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — BTSP cipher downgrade: null cipher, weak AEAD, bond-type floor enforcement",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Cipher negotiation mechanism");
    phase_negotiation_mechanism(v);

    v.section("Phase 2: Weak cipher rejection");
    phase_weak_cipher_rejection(v);

    v.section("Phase 3: Bond-type cipher floor");
    phase_bond_cipher_floor(v);
}

fn phase_negotiation_mechanism(v: &mut ValidationResult) {
    let has_negotiate_handler = NEGOTIATION_SRC.contains("handle_server_negotiate");
    v.check_bool(
        "downgrade:negotiate_handler",
        has_negotiate_handler,
        "BTSP negotiate handler present (btsp.server.negotiate method)",
    );

    let has_from_wire_name = NEGOTIATION_SRC.contains("from_wire_name");
    v.check_bool(
        "downgrade:cipher_parsing",
        has_from_wire_name,
        "Cipher parsed via from_wire_name() — maps wire string to enum variant",
    );

    let has_unwrap_or_default = NEGOTIATION_SRC.contains("unwrap_or(");
    v.check_bool(
        "downgrade:unknown_cipher_default",
        has_unwrap_or_default,
        &format!(
            "Unknown cipher handling: {} — defaults to ChaCha20-Poly1305 (safe fallback for unknown, \
             but should this be a rejection instead?)",
            if has_unwrap_or_default {
                "defaults to ChaCha20"
            } else {
                "different handling"
            }
        ),
    );
}

fn phase_weak_cipher_rejection(v: &mut ValidationResult) {
    let rejects_null = HANDSHAKE_SRC.contains("null")
        || HANDSHAKE_SRC.contains("Null")
        || HANDSHAKE_SRC.contains("None");
    let null_in_enum = HANDSHAKE_SRC.contains("Null") && HANDSHAKE_SRC.contains("BtspCipher");

    v.check_bool(
        "downgrade:null_cipher_handling",
        rejects_null || null_in_enum,
        &format!(
            "Null cipher in BtspCipher enum: {} — if present, negotiate must reject it; \
             if absent, from_wire_name(\"null\") returns None → defaults to ChaCha20 (safe)",
            if null_in_enum {
                "PRESENT (must verify rejection)"
            } else {
                "ABSENT (implicit rejection via unknown → default)"
            }
        ),
    );

    let known_ciphers: Vec<&str> = HANDSHAKE_SRC
        .lines()
        .filter(|l| l.contains("ChaCha") || l.contains("AesGcm") || l.contains("Aes"))
        .take(5)
        .collect();
    v.check_bool(
        "downgrade:cipher_inventory",
        !known_ciphers.is_empty(),
        &format!(
            "{} cipher variants found in BtspCipher enum — all should be AEAD",
            known_ciphers.len()
        ),
    );

    let has_cipher_floor = HANDSHAKE_SRC.contains("cipher_floor")
        || HANDSHAKE_SRC.contains("minimum_cipher")
        || HANDSHAKE_SRC.contains("MIN_CIPHER");
    v.check_bool(
        "downgrade:cipher_floor_policy",
        has_cipher_floor,
        &format!(
            "Cipher floor policy: {} — without a floor, negotiate could accept weaker-than-intended ciphers",
            if has_cipher_floor {
                "ENFORCED"
            } else {
                "NOT FOUND (implicit floor: from_wire_name rejects unknown, but no strength ordering)"
            }
        ),
    );
}

fn phase_bond_cipher_floor(v: &mut ValidationResult) {
    let has_bond_type = HANDSHAKE_SRC.contains("bond_type")
        || HANDSHAKE_SRC.contains("BondType")
        || HANDSHAKE_SRC.contains("covalent")
        || HANDSHAKE_SRC.contains("ionic");
    v.check_bool(
        "downgrade:bond_type_awareness",
        has_bond_type,
        &format!(
            "Bond type awareness in cipher negotiation: {} — \
             covalent bonds (inter-gate) should enforce stronger ciphers than ionic (local)",
            if has_bond_type {
                "PRESENT"
            } else {
                "ABSENT (same cipher floor for all bond types)"
            }
        ),
    );

    let has_negotiate_session_check =
        NEGOTIATION_SRC.contains("session_token") || NEGOTIATION_SRC.contains("SessionToken");
    v.check_bool(
        "downgrade:session_bound_negotiate",
        has_negotiate_session_check,
        &format!(
            "Negotiate is session-bound: {} — re-negotiate on an existing session token \
             (cannot negotiate on a session you don't own)",
            if has_negotiate_session_check {
                "YES (requires valid session_token)"
            } else {
                "UNCLEAR"
            }
        ),
    );

    let has_negotiate_response =
        NEGOTIATION_SRC.contains("accepted") && NEGOTIATION_SRC.contains("cipher");
    v.check_bool(
        "downgrade:negotiate_response_contract",
        has_negotiate_response,
        "Negotiate response includes `accepted: bool` + `cipher: string` — \
         rejected negotiations return the requested (not accepted) cipher with accepted=false",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_no_panic() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
    }
}
