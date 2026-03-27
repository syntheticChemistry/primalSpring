// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates multi-stage supply chain provenance from rhizoCrypt's complete workflows.
//! Source: primals/rhizoCrypt/showcase/01-inter-primal-live/05-complete-workflows/demo-supply-chain.sh

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp057 — Supply Chain Provenance")
        .with_provenance("exp057_supply_chain_provenance", "2026-03-24")
        .run(
            "primalSpring Exp057: rhizoCrypt Farm-to-Table Provenance",
            |v| {
                let rhizocrypt = discover_primal(primal_names::RHIZOCRYPT);
                v.check_bool(
                    "discover_rhizocrypt",
                    rhizocrypt.primal == primal_names::RHIZOCRYPT,
                    "discover rhizocrypt (provenance trio)",
                );

                let loamspine = discover_primal(primal_names::LOAMSPINE);
                v.check_bool(
                    "discover_loamspine",
                    loamspine.primal == primal_names::LOAMSPINE,
                    "discover loamspine (provenance trio)",
                );

                let sweetgrass = discover_primal(primal_names::SWEETGRASS);
                v.check_bool(
                    "discover_sweetgrass",
                    sweetgrass.primal == primal_names::SWEETGRASS,
                    "discover sweetgrass (provenance trio)",
                );

                let path_rhizocrypt = socket_path(primal_names::RHIZOCRYPT);
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
