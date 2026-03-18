// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp022: RootPulse Diff Federate — validates RootPulse has diff and federate graphs for Merkle diff.

use primalspring::emergent::EmergentSystem;
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

    v.check_skip(
        "actual_diff_federate",
        "actual diff/federate operations need live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
