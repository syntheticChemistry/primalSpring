// SPDX-License-Identifier: AGPL-3.0-or-later

//! primalSpring guideStone — Composition Certification.
//!
//! Self-validating deployable that certifies a NUCLEUS composition is
//! structurally sound, IPC-healthy, and cryptographically functional.
//! Domain guideStones (hotSpring, healthSpring, etc.) inherit this base
//! certification and only need to validate their own science on top.
//!
//! # Layers (each depends on the previous)
//!
//! | Layer | Name | Description |
//! |-------|------|-------------|
//! | 0     | Bare | graph/fragment/manifest structural validation (no primals needed) |
//! | 0.5   | Seed Provenance | mito seed resolved, fingerprints verified, BTSP mode set |
//! | 1     | Discovery | all primals in the graph discoverable via capability scan |
//! | 1.5   | BTSP Escalation | per-atomic security posture (cleartext vs BTSP per tier) |
//! | 2     | Health | every discovered primal responds to `health.liveness` |
//! | 3     | Capability Parity | math, storage, shader IPC calls produce correct results |
//! | 4     | Cross-Atomic Pipeline | Tower hash → Nest store → retrieve → verify |
//! | 5     | Bonding Model | bonding policies correctly enforced between atomics |
//! | 6     | BTSP + Crypto | crypto.hash parity, cipher policy, Ed25519 roundtrip |
//! | 7     | Cellular | per-spring deploy graphs parse, declare live mode, cover capabilities |
//!
//! # Exit Codes
//!
//! - `0` — all layers passed (NUCLEUS certified)
//! - `1` — one or more layers failed
//! - `2` — bare-only mode (no primals discovered, structural checks only)

#![deny(unsafe_code)]

mod entropy;
mod layers;

use primalspring::composition::{CompositionContext, validate_liveness};
use primalspring::coordination::AtomicType;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring guideStone — Composition Certification");

    ValidationResult::print_banner("primalSpring guideStone — Base Composition Certification");

    // Layer 0: Bare Properties (always runs, no primals needed)
    v.section("Layer 0: Bare Properties");
    layers::bare::validate_bare_properties(&mut v);

    // Layer 0.5: Seed Provenance — resolve entropy, set BTSP credentials
    v.section("Layer 0.5: Seed Provenance");
    let mito_seed = entropy::resolve_mito_seed();

    let family_id = std::env::var(primalspring::env_keys::FAMILY_ID)
        .ok()
        .filter(|s| !s.is_empty() && s != "default")
        .unwrap_or_else(|| "guidestone-validation".to_owned());
    // SAFETY: called in main() before any threads are spawned.
    // Rust 2024 marks set_var as unsafe due to data races in multithreaded
    // programs. This binary is single-threaded at this point — no async
    // runtime has been started and no std::thread::spawn has been called.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(primalspring::env_keys::FAMILY_ID, &family_id);
        std::env::set_var(primalspring::env_keys::FAMILY_SEED, &mito_seed.hex_seed);
        std::env::set_var(primalspring::env_keys::BEARDOG_FAMILY_SEED, &mito_seed.hex_seed);
    }

    entropy::validate_seed_provenance(&mut v, &mito_seed);

    // Layer 1: Discovery — can we find primals?
    v.section("Layer 1: Discovery");
    let mut ctx = CompositionContext::discover();

    let full_caps = AtomicType::FullNucleus.required_capabilities();
    let alive = validate_liveness(&mut ctx, &mut v, full_caps);

    if alive == 0 {
        eprintln!("[guideStone] No NUCLEUS primals discovered — bare certification only.");
        eprintln!("  Deploy from plasmidBin and rerun for full certification.");
        v.finish();
        let code = if v.exit_code() == 0 { 2 } else { 1 };
        std::process::exit(code);
    }

    // Layer 1.5: BTSP Escalation — per-atomic security posture
    v.section("Layer 1.5: BTSP Escalation");
    layers::btsp::validate_btsp_escalation(&ctx, &mut v);
    layers::btsp::validate_substrate_health(&mut v);

    // Layer 2: Atomic Health
    v.section("Layer 2: Atomic Health");
    layers::health::validate_atomic_health(&mut ctx, &mut v);

    // Layer 3: Capability Parity — math, storage, shader
    v.section("Layer 3: Capability Parity");
    layers::health::validate_math_parity(&mut ctx, &mut v);
    layers::health::validate_storage_roundtrip(&mut ctx, &mut v);
    layers::health::validate_shader_capabilities(&mut ctx, &mut v);

    // Layer 4: Cross-Atomic Pipeline
    v.section("Layer 4: Cross-Atomic Pipeline");
    layers::health::validate_cross_atomic_pipeline(&mut ctx, &mut v);

    // Layer 5: Bonding Model
    v.section("Layer 5: Bonding Model");
    layers::bonding::validate_bonding_policies(&mut v);

    // Layer 6: BTSP + Crypto
    v.section("Layer 6: BTSP + Crypto");
    layers::btsp::validate_crypto(&mut ctx, &mut v);

    // Layer 7: Cellular Deployment — per-spring deploy graphs
    v.section("Layer 7: Cellular Deployment");
    layers::cellular::validate_cellular_graphs(&mut v);

    v.finish();
    std::process::exit(v.exit_code());
}
