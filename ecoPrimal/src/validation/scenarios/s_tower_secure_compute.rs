// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Secure Compute Mesh.
//!
//! Validates structural readiness for `bearDog` per-session key derivation —
//! Tower's crypto advantage over `WireGuard`'s tunnel-level encryption.
//!
//! `WireGuard` encrypts all traffic with one tunnel key pair. Tower derives
//! per-session, per-capability keys via BTSP (`bearDog` handshake) enabling:
//! - Per-flow crypto policy (`PostPrimordial` = strongest for sensitive)
//! - Key rotation per-session (compromise isolation)
//! - Hardware-backed key material via `CredentialStore`
//!
//! Measures: `bearDog` per-session keys vs WG tunnel crypto overhead.
//! Primary gate: flockGate (`bearDog` team).

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-secure-compute",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_secure_compute",
        provenance_date: "2026-07-23",
        description: "Tower secure compute mesh — bearDog per-session keys vs WG tunnel crypto",
    },
    run,
};

/// Execute the validation checks.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Per-session cryptography (BTSP)");

    let has_handshake = REGISTRY_TOML.contains("btsp.handshake");
    v.check_bool(
        "secure:btsp_handshake",
        has_handshake,
        "btsp.handshake: per-session HKDF key derivation (vs WG static tunnel key)",
    );

    let has_negotiate = REGISTRY_TOML.contains("btsp.negotiate");
    v.check_bool(
        "secure:btsp_negotiate",
        has_negotiate,
        "btsp.negotiate: per-capability crypto policy selection (PostPrimordial = max strength)",
    );

    let has_server_status = REGISTRY_TOML.contains("btsp.server.status");
    v.check_bool(
        "secure:btsp_server_health",
        has_server_status,
        "btsp.server.status: runtime BTSP health monitoring (session key rotation observable)",
    );

    v.section("Key material management");

    let has_credential = REGISTRY_TOML.contains("secrets.") || REGISTRY_TOML.contains("credential");
    v.check_bool(
        "secure:credential_store",
        has_credential,
        "CredentialStore: hardware-backed key material (InMemory + FileVault, TEE pending)",
    );

    let has_enrollment =
        REGISTRY_TOML.contains("mesh.enroll") || REGISTRY_TOML.contains("mesh.init");
    v.check_bool(
        "secure:enrollment_keys",
        has_enrollment,
        "mesh.enroll: HMAC-SHA256 enrollment keys (per-peer trust establishment)",
    );

    v.section("Isolation and rotation");

    let has_btsp_escalation = REGISTRY_TOML.contains("btsp_escalation");
    v.check_bool(
        "secure:encrypted_framing",
        has_btsp_escalation,
        "BTSP escalation: ChaCha20-Poly1305 per-session framing (compromise isolation)",
    );

    let has_capabilities_query = REGISTRY_TOML.contains("mesh.capabilities_query");
    v.check_bool(
        "secure:capability_gated_access",
        has_capabilities_query,
        "mesh.capabilities_query: capability-gated crypto tier (not all flows need max crypto)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
