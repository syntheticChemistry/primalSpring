// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp044: Squirrel AI Coordination — validates multi-MCP routing via Squirrel + biomeOS capability graph.

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp044 — Squirrel AI Coordination");
    println!("{}", "=".repeat(72));
    println!(
        "primalSpring Exp044: Squirrel AI Coordination (Multi-MCP routing via Squirrel + biomeOS capability graph)"
    );
    println!("{}", "=".repeat(72));

    let squirrel = discover_primal("squirrel");
    v.check_bool(
        "discover_squirrel",
        squirrel.primal == "squirrel",
        "discover squirrel",
    );

    let path = socket_path("squirrel");
    v.check_bool(
        "squirrel_socket_path_valid",
        path.to_string_lossy().contains("squirrel") && path.to_string_lossy().contains("biomeos"),
        "squirrel socket path contains squirrel and biomeos",
    );

    v.check_skip(
        "actual_mcp_coordination",
        "needs live Squirrel for Multi-MCP coordination",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
