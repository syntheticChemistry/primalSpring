// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp043: PetalTongue Viz — validates biomeOS SSE events to petalTongue rendering pipeline.

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp043 — PetalTongue Viz");
    println!("{}", "=".repeat(72));
    println!(
        "primalSpring Exp043: PetalTongue Viz (biomeOS SSE events -> petalTongue rendering pipeline)"
    );
    println!("{}", "=".repeat(72));

    let petaltongue = discover_primal("petaltongue");
    v.check_bool(
        "discover_petaltongue",
        petaltongue.primal == "petaltongue",
        "discover petaltongue",
    );

    let path = socket_path("petaltongue");
    v.check_bool(
        "petaltongue_socket_path_valid",
        path.to_string_lossy().contains("petaltongue")
            && path.to_string_lossy().contains("biomeos"),
        "petaltongue socket path contains petaltongue and biomeos",
    );

    v.check_skip(
        "actual_sse_visualization",
        "needs live biomeOS SSE and petalTongue for visualization",
    );

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
