// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp031: Ionic Bond — validates cross-family limited capability sharing.

use primalspring::bonding::BondType;
use primalspring::coordination::probe_primal;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp031 — Ionic Bond")
        .with_provenance("exp031_ionic_bond", "2026-04-13")
        .run(
            "primalSpring Exp031: Cross-Family Limited Capability Sharing",
            |v| {
                let bond = BondType::Ionic;
                v.check_bool(
                    "ionic_description_non_empty",
                    !bond.description().is_empty(),
                    &format!(
                        "BondType::Ionic.description() is non-empty — {}",
                        bond.description()
                    ),
                );

                let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
                let path_beardog = socket_path(primal_names::BEARDOG);
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

                let beardog = discover_primal(primal_names::BEARDOG);
                v.check_bool(
                    "discover_beardog_returns_result",
                    beardog.primal == primal_names::BEARDOG,
                    "discover_primal returns DiscoveryResult for beardog",
                );
                let songbird = discover_primal(primal_names::SONGBIRD);
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

                v.check_skip(
                    "cross_family_capability_sharing",
                    "needs live primals from different families",
                );
            },
        );
}
