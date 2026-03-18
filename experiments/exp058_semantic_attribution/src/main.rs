// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates sweetGrass semantic attribution at module/feature/function levels.
//! Source: `phase2/sweetGrass/showcase/ROOTPULSE_EMERGENCE_PLAN.md`

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp058 — Semantic Attribution");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp058: sweetGrass Semantic Attribution (RootPulse)");
    println!("{}", "=".repeat(72));

    let sweetgrass = discover_primal("sweetgrass");
    v.check_bool(
        "discover_sweetgrass",
        sweetgrass.primal == "sweetgrass",
        "discover sweetgrass",
    );

    let rhizocrypt = discover_primal("rhizocrypt");
    v.check_bool(
        "discover_rhizocrypt",
        rhizocrypt.primal == "rhizocrypt",
        "discover rhizocrypt",
    );

    let path_sweetgrass = socket_path("sweetgrass");
    let path_rhizocrypt = socket_path("rhizocrypt");
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

    v.finish();
    std::process::exit(v.exit_code());
}
