// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp051: Socket Discovery Sweep — capability-based enumeration.
//!
//! Discovers all NUCLEUS capabilities at runtime via the loose-coupling
//! path. Each capability is resolved to its provider without hardcoding
//! any primal names.

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::{discover_capabilities_for, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp051 — Socket Discovery Sweep")
        .with_provenance("exp051_socket_discovery_sweep", "2026-03-24")
        .run(
            "primalSpring Exp051: Capability-Based Discovery Sweep",
            |v| {
                let all_caps = AtomicType::FullNucleus.required_capabilities();
                let results = discover_capabilities_for(all_caps);

                v.check_count(
                    "discover_returns_expected_count",
                    results.len(),
                    all_caps.len(),
                );

                let all_have_capability = results
                    .iter()
                    .zip(all_caps.iter())
                    .all(|(r, &cap)| r.capability == cap);
                v.check_bool(
                    "all_results_match_capabilities",
                    all_have_capability,
                    "all results have capability names matching FullNucleus composition",
                );

                let all_contain_biomeos = all_caps
                    .iter()
                    .all(|_| socket_path("probe").to_string_lossy().contains("biomeos"));
                v.check_bool(
                    "socket_path_contains_biomeos",
                    all_contain_biomeos,
                    "socket_path convention includes \"biomeos\" in path",
                );

                let reachable = results.iter().filter(|r| r.socket.is_some()).count();
                let unreachable = results.iter().filter(|r| r.socket.is_none()).count();
                println!("  [INFO] reachable: {reachable}, unreachable: {unreachable}");
                v.check_bool(
                    "reachable_count_consistent",
                    reachable + unreachable == all_caps.len(),
                    "count of reachable + unreachable equals capability count",
                );

                for r in &results {
                    let status = if r.socket.is_some() { "UP" } else { "DOWN" };
                    let provider = r.resolved_primal.as_deref().unwrap_or("unresolved");
                    println!(
                        "  [{status}] {} -> {provider} @ {}",
                        r.capability,
                        r.socket
                            .as_ref()
                            .map_or_else(|| "not found".to_owned(), |p| p.display().to_string())
                    );
                }
            },
        );
}
