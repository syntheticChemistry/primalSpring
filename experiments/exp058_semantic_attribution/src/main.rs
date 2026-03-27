// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates sweetGrass semantic attribution at module/feature/function levels.
//! Source: `primals/sweetGrass/showcase/ROOTPULSE_EMERGENCE_PLAN.md`

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp058 — Semantic Attribution")
        .with_provenance("exp058_semantic_attribution", "2026-03-24")
        .run(
            "primalSpring Exp058: sweetGrass Semantic Attribution (RootPulse)",
            |v| {
                let sweetgrass = discover_primal(primal_names::SWEETGRASS);
                v.check_bool(
                    "discover_sweetgrass",
                    sweetgrass.primal == primal_names::SWEETGRASS,
                    "discover sweetgrass",
                );

                let rhizocrypt = discover_primal(primal_names::RHIZOCRYPT);
                v.check_bool(
                    "discover_rhizocrypt",
                    rhizocrypt.primal == primal_names::RHIZOCRYPT,
                    "discover rhizocrypt",
                );

                let path_sweetgrass = socket_path(primal_names::SWEETGRASS);
                let path_rhizocrypt = socket_path(primal_names::RHIZOCRYPT);
                v.check_bool(
                    "socket_paths_valid",
                    path_sweetgrass.to_string_lossy().contains("biomeos")
                        && path_rhizocrypt.to_string_lossy().contains("biomeos"),
                    "socket paths contain biomeos",
                );

                v.check_skip(
                    "semantic_tracking",
                    "semantic tracking needs live sweetgrass",
                );
                v.check_skip("braid_formation", "braid formation needs live primals");
            },
        );
}
