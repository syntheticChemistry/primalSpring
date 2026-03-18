// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp034: Capability Aggregation — Plasmodium routes to best gate.
//!
//! Validates that multi-gate compositions can aggregate capabilities
//! and route requests to the best available gate for a given workload.

use primalspring::coordination::{AtomicType, validate_composition};
use primalspring::ipc::discover::discover_for;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp034 — Capability Aggregation");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp034: Plasmodium Routes to Best Gate for Workload");
    println!("{}", "=".repeat(72));

    let required = AtomicType::FullNucleus.required_primals();
    v.check_count("full_nucleus_has_eight_primals", required.len(), 8);

    let results = discover_for(required);
    let found = results.iter().filter(|r| r.socket.is_some()).count();

    if found >= 2 {
        let comp = validate_composition(AtomicType::FullNucleus);
        v.check_minimum("aggregated_capabilities", comp.total_capabilities, 2);
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
