// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp034: Capability Aggregation — Plasmodium routes to best gate.
//!
//! Validates that multi-gate compositions can aggregate capabilities
//! and route requests to the best available gate for a given workload.

use primalspring::coordination::AtomicType;
use primalspring::ipc::client::connect_primal;
use primalspring::ipc::discover::{discover_for, extract_capability_names, socket_path};
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp034 — Capability Aggregation");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp034: Plasmodium Routes to Best Gate for Workload");
    println!("{}", "=".repeat(72));

    let required = AtomicType::FullNucleus.required_primals();
    v.check_count("full_nucleus_has_eight_primals", required.len(), 8);

    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
    let path_beardog = socket_path("beardog");
    let path_contains_family = path_beardog
        .to_string_lossy()
        .contains(&format!("-{family_id}.sock"));
    v.check_bool(
        "socket_path_includes_family_id",
        path_contains_family,
        &format!(
            "socket path includes FAMILY_ID: {} (path: {})",
            family_id,
            path_beardog.display()
        ),
    );

    let results = discover_for(required);
    let found = results.iter().filter(|r| r.socket.is_some()).count();

    let mut all_cap_names = Vec::new();
    for r in &results {
        if r.socket.is_some() {
            if let Ok(mut client) = connect_primal(&r.primal) {
                let caps_json = client.capabilities().ok();
                all_cap_names.extend(extract_capability_names(caps_json));
            }
        }
    }

    if found >= 2 {
        v.check_minimum("aggregated_capabilities", all_cap_names.len(), 2);
    } else {
        v.check_skip(
            "aggregated_capabilities",
            &format!("need >= 2 live primals for aggregation, found {found}"),
        );
    }

    v.check_skip(
        "capability_aggregation_routing",
        "needs live Plasmodium for best-gate routing",
    );

    v.finish();
    std::process::exit(v.exit_code());
}
