// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp025: CoralForge Pipeline — validates pipeline graph over neuralSpring + wetSpring + toadStool.

use primalspring::coordination::probe_primal;
use primalspring::emergent::EmergentSystem;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp025 — CoralForge Pipeline")
        .with_provenance("exp025_coralforge_pipeline", "2026-03-24")
        .run(
            "primalSpring Exp025: Pipeline Graph over neuralSpring + wetSpring + toadStool",
            |v| {
                let graphs = EmergentSystem::CoralForge.required_graphs();
                let has_pipeline = graphs.contains(&"coralforge_pipeline");
                v.check_bool(
                    "coralforge_has_pipeline_graph",
                    has_pipeline,
                    &format!(
                        "EmergentSystem::CoralForge.required_graphs() contains coralforge_pipeline: {graphs:?}"
                    ),
                );

                let toadstool = discover_primal(primal_names::TOADSTOOL);
                let path = socket_path(primal_names::TOADSTOOL);
                v.check_bool(
                    "discover_toadstool_socket_path",
                    toadstool.primal == primal_names::TOADSTOOL
                        && path.to_string_lossy().contains(primal_names::TOADSTOOL),
                    "discover toadstool socket path",
                );

                let coralreef = discover_primal(primal_names::CORALREEF);
                let nestgate = discover_primal(primal_names::NESTGATE);
                for (name, discovery) in [
                    ("toadstool", toadstool),
                    ("coralreef", coralreef),
                    ("nestgate", nestgate),
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
                    "actual_pipeline_execution",
                    "actual pipeline execution needs live IPC",
                );
            },
        );
}
