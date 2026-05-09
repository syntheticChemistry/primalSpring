// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exp096: Pixel Cross-Architecture Bonding

mod config;
mod phases;

use primalspring::validation::ValidationResult;

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
                phases::validate_pixel_tower_health(v);
                phases::validate_cross_arch_genetics(v);
                phases::validate_btsp_phase3_readiness(v);
                phases::validate_hsm_capabilities(v);
                phases::validate_beacon_exchange(v);
                phases::validate_bonding_model(v);
                phases::validate_stun_nat(v);
            },
        );
}
