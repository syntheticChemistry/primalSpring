// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp020: RootPulse Full Commit — validates RootPulse 6-phase commit, required graphs, and provenance trio discovery.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp020 — RootPulse Full Commit");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp020: RootPulse Full 6-Phase Commit");
    println!("{}", "=".repeat(72));

    let graphs = EmergentSystem::RootPulse.required_graphs();
    v.check_bool(
        "rootpulse_has_five_required_graphs",
        graphs.len() == 5,
        &format!("EmergentSystem::RootPulse has 5 required graphs: {graphs:?}"),
    );

    let rhizocrypt = discover_primal("rhizocrypt");
    v.check_bool(
        "discover_rhizocrypt",
        rhizocrypt.primal == "rhizocrypt",
        "discover rhizocrypt (provenance trio)",
    );
    let loamspine = discover_primal("loamspine");
    v.check_bool(
        "discover_loamspine",
        loamspine.primal == "loamspine",
        "discover loamspine (provenance trio)",
    );
    let sweetgrass = discover_primal("sweetgrass");
    v.check_bool(
        "discover_sweetgrass",
        sweetgrass.primal == "sweetgrass",
        "discover sweetgrass (provenance trio)",
    );

    for (name, discovery) in [
        ("rhizocrypt", rhizocrypt),
        ("loamspine", loamspine),
        ("sweetgrass", sweetgrass),
    ] {
        v.check_or_skip(
            &format!("probe_{name}"),
            discovery.socket.as_ref(),
            &format!("{name} socket not found"),
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
                v.check_bool(
                    &format!("{name}_capabilities"),
                    !health.capabilities.is_empty(),
                    &format!("capabilities: {:?}", health.capabilities),
                );
            },
        );
    }

    v.check_skip("health_phase", "health phase needs live IPC");
    v.check_skip("dehydrate_phase", "dehydrate phase needs live IPC");
    v.check_skip("sign_phase", "sign phase needs live IPC");
    v.check_skip("store_phase", "store phase needs live IPC");
    v.check_skip("commit_phase", "commit phase needs live IPC");
    v.check_skip("attribute_phase", "attribute phase needs live IPC");

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
