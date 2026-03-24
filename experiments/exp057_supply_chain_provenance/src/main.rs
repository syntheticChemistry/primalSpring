// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates multi-stage supply chain provenance from rhizoCrypt's complete workflows.
//! Source: phase2/rhizoCrypt/showcase/01-inter-primal-live/05-complete-workflows/demo-supply-chain.sh

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp057 — Supply Chain Provenance")
        .with_provenance("exp057_supply_chain_provenance", "2026-03-24")
        .run(
            "primalSpring Exp057: rhizoCrypt Farm-to-Table Provenance",
            |v| {
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

                let path_rhizocrypt = socket_path("rhizocrypt");
                v.check_bool(
                    "socket_path_contains_biomeos",
                    path_rhizocrypt.to_string_lossy().contains("biomeos"),
                    "socket_path for rhizocrypt contains biomeos",
                );

                v.check_skip(
                    "actual_dag_execution",
                    "actual DAG execution with signing needs live primals",
                );
            },
        );
}
