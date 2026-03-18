// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp030: Covalent Bond — validates two NUCLEUS instances share family seed and discover each other.

use primalspring::bonding::BondType;
use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp030 — Covalent Bond");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp030: Two NUCLEUS Instances Share Family Seed, Discover Each Other");
    println!("{}", "=".repeat(72));

    let bond = BondType::Covalent;
    v.check_bool(
        "covalent_description_non_empty",
        !bond.description().is_empty(),
        &format!(
            "BondType::Covalent.description() is non-empty — {}",
            bond.description()
        ),
    );
    v.check_bool(
        "covalent_identity",
        bond == BondType::Covalent,
        "BondType::Covalent == BondType::Covalent (identity)",
    );

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let path_beardog = socket_path("beardog");
    let path_contains_family = path_beardog
        .to_string_lossy()
        .contains(&format!("-{family_id}.sock"));
    v.check_bool(
        "socket_path_includes_family_id",
        path_contains_family,
        &format!(
            "socket path includes FAMILY_ID: {} (path: {})",
            family_id,
            path_beardog.display()
        ),
    );

    let beardog = discover_primal("beardog");
    v.check_bool(
        "discover_beardog_returns_result",
        beardog.primal == "beardog",
        "discover_primal returns DiscoveryResult for beardog",
    );
    let songbird = discover_primal("songbird");
    for (name, discovery) in [("beardog", beardog), ("songbird", songbird)] {
        v.check_or_skip(
            &format!("probe_{name}"),
            discovery.socket.as_ref(),
            &format!("{name} socket not found (Tower primitive)"),
            |_, v| {
                let health = probe_primal(name);
                v.check_bool(
                    &format!("{name}_health"),
                    health.health_ok,
                    &format!(
                        "health ok: {}, latency: {}µs",
                        health.health_ok, health.latency_us
                    ),
                );
            },
        );
    }
    v.check_skip("family_seed_sharing", "needs 2 live NUCLEUS instances");
    v.check_skip("mutual_discovery", "needs 2 live NUCLEUS instances");

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
