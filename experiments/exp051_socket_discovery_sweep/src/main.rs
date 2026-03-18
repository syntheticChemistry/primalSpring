// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp051: Socket Discovery Sweep — enumerate all NUCLEUS primals.
//!
//! Uses the full NUCLEUS composition definition to drive discovery rather
//! than a hardcoded primal roster. Each primal is probed at
//! `$XDG_RUNTIME_DIR/biomeos/` via the standard 3-tier resolution.

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::{discover_for, discover_reachable_for, socket_path};
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp051 — Socket Discovery Sweep");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp051: Enumerate NUCLEUS Primals at $XDG_RUNTIME_DIR/biomeos/");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let all_primals = AtomicType::FullNucleus.required_primals();
    let results = discover_for(all_primals);

    v.check_count(
        "discover_returns_expected_count",
        results.len(),
        all_primals.len(),
    );

    let names_match = results
        .iter()
        .zip(all_primals.iter())
        .all(|(r, &name)| r.primal == name);
    v.check_bool(
        "all_results_match_primals",
        names_match,
        "all results have primal names matching FullNucleus composition",
    );

    let all_contain_biomeos = all_primals
        .iter()
        .all(|name| socket_path(name).to_string_lossy().contains("biomeos"));
    v.check_bool(
        "socket_path_contains_biomeos",
        all_contain_biomeos,
        "socket_path for each primal contains \"biomeos\" in path",
    );

    let reachable = discover_reachable_for(all_primals);
    let unreachable_count = results.iter().filter(|r| r.socket.is_none()).count();
    println!(
        "  [INFO] reachable: {}, unreachable: {}",
        reachable.len(),
        unreachable_count
    );
    v.check_bool(
        "reachable_count_consistent",
        reachable.len() + unreachable_count == all_primals.len(),
        "count of reachable + unreachable equals composition size",
    );

    for r in &results {
        let status = if r.socket.is_some() { "UP" } else { "DOWN" };
        println!(
            "  [{status}] {} @ {}",
            r.primal,
            r.socket
                .as_ref()
                .map_or_else(|| "not found".to_owned(), |p| p.display().to_string())
        );
    }

    v.finish();
    std::process::exit(v.exit_code());
}
