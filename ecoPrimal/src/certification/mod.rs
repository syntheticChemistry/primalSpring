// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Composition certification engine — absorbed guidestone organelle.
//!
//! Proves NUCLEUS composition correctness through layered validation:
//!
//! | Layer | Name | Description |
//! |-------|------|-------------|
//! | 0     | Bare | graph/fragment/manifest structural validation (no primals needed) |
//! | 0.5   | Seed Provenance | mito seed resolved, fingerprints verified |
//! | 1     | Discovery | all primals in the graph discoverable |
//! | 1.5   | BTSP Escalation | per-atomic security posture |
//! | 2     | Health | every discovered primal responds to `health.liveness` |
//! | 3     | Capability Parity | math, storage, shader IPC calls produce correct results |
//! | 4     | Cross-Atomic Pipeline | Tower hash → Nest store → retrieve → verify |
//! | 5     | Bonding Model | bonding policies + live ionic bond attempt |
//! | 6     | BTSP + Crypto | crypto.hash parity, cipher policy, Ed25519 roundtrip |
//! | 7     | Cellular | per-spring deploy graphs parse, declare live mode |
//! | 8     | Lifecycle | composition.reload + rediscovery + post-reload liveness |
//!
//! Originally evolved as the `primalspring_guidestone` binary.
//! Endosymbiosed into the library at the interstadial transition.

pub mod bare;
pub mod bonding;
pub mod btsp;
pub mod cellular;
pub mod crypto_bootstrap;
pub mod entropy;
pub mod health;
pub mod lifecycle;

use crate::composition::{CompositionContext, validate_liveness};
use crate::coordination::AtomicType;
use crate::env_keys::{SeedConfig, init_seed_config};
use crate::validation::ValidationResult;

/// Maximum certification layer (inclusive).
pub const MAX_LAYER: u8 = 8;

/// Run the full certification engine up to the specified layer.
///
/// Returns the `ValidationResult` after all layers complete. Callers
/// can inspect `exit_code()` for pass/fail/bare-only status.
///
/// # Exit semantics
///
/// - `0` — all layers passed (NUCLEUS certified)
/// - `1` — one or more layers failed
/// - `2` — bare-only mode (no primals discovered, structural checks only)
#[must_use]
pub fn certify(max_layer: u8) -> ValidationResult {
    let mut v = ValidationResult::new("primalSpring Certification — Composition Correctness");

    ValidationResult::print_banner("primalSpring Certification — Composition Correctness");

    // Layer 0: Bare Properties (always runs, no primals needed)
    v.section("Layer 0: Bare Properties");
    bare::validate_bare_properties(&mut v);

    if max_layer == 0 {
        v.finish();
        return v;
    }

    // Layer 0.5: Seed Provenance
    v.section("Layer 0.5: Seed Provenance");
    let mito_seed = entropy::resolve_mito_seed();

    let family_id = std::env::var(crate::env_keys::FAMILY_ID)
        .ok()
        .filter(|s| !s.is_empty() && s != "default")
        .unwrap_or_else(|| "certification-validation".to_owned());

    let _ = init_seed_config(SeedConfig {
        family_id,
        hex_seed: mito_seed.hex_seed.clone(),
    });

    entropy::validate_seed_provenance(&mut v, &mito_seed);

    // Layer 1: Discovery
    v.section("Layer 1: Discovery");
    let mut ctx = CompositionContext::discover();

    let full_caps = AtomicType::FullNucleus.required_capabilities();
    let alive = validate_liveness(&mut ctx, &mut v, full_caps);

    if alive == 0 {
        tracing::warn!("No NUCLEUS primals discovered — bare certification only. Deploy from plasmidBin and rerun for full certification.");
        v.finish();
        return v;
    }

    if max_layer < 2 {
        v.finish();
        return v;
    }

    // Layer 1.5: BTSP Escalation
    v.section("Layer 1.5: BTSP Escalation");
    btsp::validate_btsp_escalation(&ctx, &mut v);
    btsp::validate_substrate_health(&mut v);

    // Layer 1.6: Method Gate (JH-0)
    v.section("Layer 1.6: Method Gate (JH-0)");
    btsp::validate_method_gate(&mut v);

    // Layer 2: Atomic Health
    v.section("Layer 2: Atomic Health");
    health::validate_atomic_health(&mut ctx, &mut v);

    if max_layer < 3 {
        v.finish();
        return v;
    }

    // Layer 3: Capability Parity
    v.section("Layer 3: Capability Parity");
    health::validate_math_parity(&mut ctx, &mut v);
    health::validate_storage_roundtrip(&mut ctx, &mut v);
    health::validate_shader_capabilities(&mut ctx, &mut v);

    if max_layer < 4 {
        v.finish();
        return v;
    }

    // Layer 4: Cross-Atomic Pipeline
    v.section("Layer 4: Cross-Atomic Pipeline");
    health::validate_cross_atomic_pipeline(&mut ctx, &mut v);

    if max_layer < 5 {
        v.finish();
        return v;
    }

    // Layer 5: Bonding Model
    v.section("Layer 5: Bonding Model");
    bonding::validate_bonding_policies(&mut v);
    bonding::validate_live_ionic_bond(&mut ctx, &mut v);

    if max_layer < 6 {
        v.finish();
        return v;
    }

    // Layer 6: BTSP + Crypto
    v.section("Layer 6: BTSP + Crypto");
    btsp::validate_crypto(&mut ctx, &mut v);

    if max_layer < 7 {
        v.finish();
        return v;
    }

    // Layer 7: Cellular Deployment
    v.section("Layer 7: Cellular Deployment");
    cellular::validate_cellular_graphs(&mut v);

    if max_layer < 8 {
        v.finish();
        return v;
    }

    // Layer 8: Composition Lifecycle
    v.section("Layer 8: Composition Lifecycle");
    lifecycle::validate_lifecycle(&mut ctx, &mut v);

    v.finish();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_layer_is_eight() {
        assert_eq!(MAX_LAYER, 8);
    }

    #[test]
    fn certify_layer_zero_produces_checks() {
        let v = certify(0);
        let total = v.passed + v.failed + v.skipped;
        assert!(total > 0, "certify(0) should produce structural bare checks");
    }

    #[test]
    fn certify_layer_zero_exit_code() {
        let v = certify(0);
        let code = v.exit_code();
        assert!(code == 0 || code == 1, "exit code should be 0 (pass) or 1 (fail), got {code}");
    }
}
