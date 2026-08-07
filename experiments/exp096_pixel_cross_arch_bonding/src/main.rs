// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp096: Pixel Cross-Architecture Bonding

mod config;
mod phases;

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 0: Composition discovery (local)");

    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    v.check_bool(
        "has_security_capability_path",
        ctx.has_capability("security"),
        "security capability path",
    );
    v.check_bool(
        "has_discovery_capability_path",
        ctx.has_capability("discovery"),
        "discovery capability path",
    );
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Exp096: Pixel Cross-Architecture Bonding Validation        ║");
    println!("║  x86_64 (Eastgate) ↔ aarch64 (Pixel/GrapheneOS + Titan M2) ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    ValidationResult::new("primalSpring Exp096 — Pixel Cross-Arch Bonding")
        .with_provenance("exp096_pixel_cross_arch_bonding", "2026-05-09")
        .run(
            "Pixel cross-architecture bonding, genetics, BTSP, HSM validation",
            |v| {
                let ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_composition_discovery(v, &ctx);

                let bridge = NeuralBridge::discover();
                if bridge.is_none() {
                    v.check_skip("neural_api", "biomeOS not running — using TCP fallback if enabled");
                } else if let Some(ref b) = bridge {
                    match b.health_check() {
                        Ok(_) => v.check_bool("neural_api_health", true, "biomeOS neural-api healthy"),
                        Err(e) => v.check_skip("neural_api_health", &format!("neural-api: {e}")),
                    }
                }

                phases::validate_pixel_tower_health(v, bridge.as_ref());
                phases::validate_cross_arch_genetics(v, bridge.as_ref());
                phases::validate_btsp_phase3_readiness(v, bridge.as_ref());
                phases::validate_hsm_capabilities(v, bridge.as_ref());
                phases::validate_beacon_exchange(v, bridge.as_ref());
                phases::validate_bonding_model(v, bridge.as_ref());
                phases::validate_stun_nat(v, bridge.as_ref());
            },
        );
}
