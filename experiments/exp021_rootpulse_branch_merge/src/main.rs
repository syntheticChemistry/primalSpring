// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp021: RootPulse Branch Merge — validates RootPulse has branch and merge graphs.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp021 — RootPulse Branch Merge");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp021: Branch + Merge");
    println!("{}", "=".repeat(72));

    let graphs = EmergentSystem::RootPulse.required_graphs();
    let has_branch = graphs.contains(&"rootpulse_branch");
    let has_merge = graphs.contains(&"rootpulse_merge");
    v.check_bool(
        "rootpulse_has_branch_and_merge",
        has_branch && has_merge,
        &format!(
            "EmergentSystem::RootPulse.required_graphs() contains rootpulse_branch and rootpulse_merge: {graphs:?}"
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
        "actual_branch_merge_operations",
        "actual branch/merge operations need live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
