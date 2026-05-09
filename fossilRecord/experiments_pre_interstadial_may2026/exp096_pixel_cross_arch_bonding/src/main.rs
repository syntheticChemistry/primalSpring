// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp096: Pixel Cross-Architecture Bonding — validate BTSP, genetics, and
//! bonding model enforcement across `x86_64` (Eastgate) ↔ aarch64 (Pixel/GrapheneOS).
//!
//! The Pixel runs `GrapheneOS` with a Titan M2 HSM on aarch64-linux-musl.
//! This experiment validates that the full NUCLEUS security stack works
//! identically across architectures:
//!
//! 1. **Cross-arch tower health** — `BearDog` + Songbird on Pixel reachable via TCP
//! 2. **Three-tier genetics** — mito-beacon derivation, nuclear lineage chain,
//!    lineage proof generation/verification across `x86_64` → aarch64
//! 3. **BTSP Phase 3 readiness** — cipher negotiation capability probing
//! 4. **Bonding model enforcement** — covalent (same family), ionic (cross-family)
//!    trust tier validation between architectures
//! 5. **HSM probing** — check if Pixel `BearDog` supports hardware-backed key ops
//! 6. **Beacon exchange** — `BirdSong` encrypted beacon round-trip cross-device
//! 7. **Content integrity** — BLAKE3 hash verification across architectures
//!
//! Environment:
//!   `PIXEL_HOST`           — Pixel IP or `localhost` if ADB-forwarded (default: `localhost`)
//!   `PIXEL_BEARDOG_PORT`   — `BearDog` TCP port on Pixel (default: 9900)
//!   `PIXEL_SONGBIRD_PORT`  — Songbird TCP port on Pixel (default: 9901)
//!   `PIXEL_NESTGATE_PORT`  — `NestGate` TCP port on Pixel (default: 9902)
//!   `FAMILY_ID`            — shared family ID for covalent bond testing
//!   `CROSS_FAMILY_ID`      — different family ID for ionic bond testing (optional)

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
        .with_provenance("exp096_pixel_cross_arch_bonding", "2026-04-14")
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
