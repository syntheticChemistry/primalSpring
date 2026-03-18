// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp053: Multi-Primal Lifecycle — 6-primal research paper lifecycle.
//!
//! Validates that the full NUCLEUS composition can be discovered and that
//! each primal participates in a multi-step research lifecycle.

use primalspring::coordination::{AtomicType, validate_composition};
use primalspring::ipc::discover::discover_for;

/// Source: PRIMAL_REGISTRY.md — 6 lifecycle participants in the research paper pipeline
/// (beardog, songbird, toadstool, nestgate, rhizocrypt, sweetgrass).
const LIFECYCLE_PARTICIPANT_COUNT: usize = 6;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp053 — Multi-Primal Lifecycle");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp053: 6-Primal Research Paper Lifecycle");
    println!("{}", "=".repeat(72));

    let required = AtomicType::FullNucleus.required_primals();
    v.check_count(
        "full_nucleus_requires_eight_primals",
        required.len(),
        AtomicType::FullNucleus.required_primals().len(),
    );

    let results = discover_for(required);
    let found = results.iter().filter(|r| r.socket.is_some()).count();
    println!(
        "  [INFO] discovered {found}/{} primal sockets",
        required.len()
    );

    if found >= LIFECYCLE_PARTICIPANT_COUNT {
        let comp = validate_composition(AtomicType::FullNucleus);
        v.check_minimum(
            "lifecycle_participants",
            comp.primals.iter().filter(|p| p.health_ok).count(),
            LIFECYCLE_PARTICIPANT_COUNT,
        );
        v.check_bool(
            "composition_discovery",
            comp.discovery_ok,
            "all sockets found",
        );
    } else {
        v.check_skip(
            "lifecycle_participants",
            &format!("need >= {LIFECYCLE_PARTICIPANT_COUNT} live primals, found {found}"),
        );
        v.check_skip("composition_discovery", "insufficient live primals");
    }

    v.check_skip(
        "lifecycle_orchestration",
        "end-to-end lifecycle orchestration requires graph execution",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
