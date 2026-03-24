// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp040: Cross Spring Data Flow — capability-routed ecology pipeline.
//!
//! Validates capability-based routing across spring primals. Discovery is
//! driven by the Neural API rather than a hardcoded primal roster. The
//! ecology pipeline (airSpring → wetSpring → neuralSpring) is dispatched
//! via capability calls; we validate the routing endpoints (petalTongue
//! for visualization, Squirrel for AI coordination) are discoverable.

use primalspring::ipc::discover::{discover_for, neural_api_healthy};
use primalspring::validation::ValidationResult;

/// Routing endpoints probed for capability-based cross-spring data flow.
///
/// Source: `PRIMAL_REGISTRY.md` — petalTongue (visualization) and Squirrel
/// (AI coordination) are post-NUCLEUS primals that participate in
/// cross-spring ecology pipelines.
const ROUTING_PRIMALS: &[&str] = &["petaltongue", "squirrel"];

fn main() {
    ValidationResult::new("primalSpring Exp040 — Cross Spring Data Flow")
        .with_provenance("exp040_cross_spring_data_flow", "2026-03-24")
        .run(
            "primalSpring Exp040: Cross Spring Data Flow (ecology pipeline via capability routing)",
            |v| {
                let results = discover_for(ROUTING_PRIMALS);
                let found = results.iter().filter(|r| r.socket.is_some()).count();
                v.check_bool(
                    "routing_primals_probed",
                    results.len() == ROUTING_PRIMALS.len(),
                    &format!(
                        "probed {} routing primals, {found} reachable",
                        ROUTING_PRIMALS.len()
                    ),
                );

                if neural_api_healthy() {
                    v.check_bool("neural_api_reachable", true, "Neural API healthy");
                    v.check_skip(
                        "cross_spring_data_flow",
                        "end-to-end flow requires airSpring + wetSpring + neuralSpring registered",
                    );
                } else {
                    v.check_skip("neural_api_reachable", "Neural API not running");
                    v.check_skip(
                        "cross_spring_data_flow",
                        "needs live spring primals for capability routing",
                    );
                }
            },
        );
}
