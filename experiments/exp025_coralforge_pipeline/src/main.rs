// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp025: CoralForge Pipeline — validates pipeline graph over neuralSpring + wetSpring + toadStool.

use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp025 — CoralForge Pipeline");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp025: Pipeline Graph over neuralSpring + wetSpring + toadStool");
    println!("{}", "=".repeat(72));

    let graphs = EmergentSystem::CoralForge.required_graphs();
    let has_pipeline = graphs.contains(&"coralforge_pipeline");
    v.check_bool(
        "coralforge_has_pipeline_graph",
        has_pipeline,
        &format!(
            "EmergentSystem::CoralForge.required_graphs() contains coralforge_pipeline: {graphs:?}"
        ),
    );

    let toadstool = discover_primal("toadstool");
    let path = socket_path("toadstool");
    v.check_bool(
        "discover_toadstool_socket_path",
        toadstool.primal == "toadstool" && path.to_string_lossy().contains("toadstool"),
        "discover toadstool socket path",
    );

    v.check_skip(
        "actual_pipeline_execution",
        "actual pipeline execution needs live IPC",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
