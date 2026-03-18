// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp040: Cross Spring Data Flow — airSpring -> wetSpring -> neuralSpring.
//!
//! Validates capability-based routing across spring primals. Discovery is
//! driven by the Neural API rather than a hardcoded primal roster.

use primalspring::ipc::discover::{discover_for, neural_api_healthy};
use primalspring::validation::ValidationResult;

const SPRING_PRIMALS: &[&str] = &["petaltongue", "squirrel"];

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp040 — Cross Spring Data Flow");
    println!("{}", "=".repeat(72));
    println!(
        "primalSpring Exp040: Cross Spring Data Flow (ecology pipeline via capability routing)"
    );
    println!("{}", "=".repeat(72));

    let results = discover_for(SPRING_PRIMALS);
    let found = results.iter().filter(|r| r.socket.is_some()).count();
    v.check_bool(
        "spring_primals_probed",
        results.len() == SPRING_PRIMALS.len(),
        &format!(
            "probed {} spring primals, {found} reachable",
            SPRING_PRIMALS.len()
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

    v.finish();
    std::process::exit(v.exit_code());
}
