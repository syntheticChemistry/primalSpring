// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp022: RootPulse Diff Federate — validates RootPulse has diff and federate graphs for Merkle diff.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp022 — RootPulse Diff Federate");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp022: Merkle Diff");
    println!("{}", "=".repeat(72));

    let graphs = EmergentSystem::RootPulse.required_graphs();
    let has_diff = graphs.contains(&"rootpulse_diff");
    let has_federate = graphs.contains(&"rootpulse_federate");
    v.check_bool(
        "rootpulse_has_diff_and_federate",
        has_diff && has_federate,
        &format!(
            "EmergentSystem::RootPulse.required_graphs() contains rootpulse_diff and rootpulse_federate: {graphs:?}"
        ),
    );

    for (name, discovery) in [
        ("rhizocrypt", discover_primal("rhizocrypt")),
        ("loamspine", discover_primal("loamspine")),
        ("sweetgrass", discover_primal("sweetgrass")),
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

    v.check_skip(
        "actual_diff_federate",
        "actual diff/federate operations need live IPC",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
