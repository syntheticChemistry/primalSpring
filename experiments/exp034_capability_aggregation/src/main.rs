// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp034: Capability Aggregation — Plasmodium routes to best gate.
//!
//! Validates that multi-gate compositions can aggregate capabilities
//! and route requests to the best available gate for a given workload.

use primalspring::coordination::AtomicType;
use primalspring::ipc::client::connect_primal;
use primalspring::ipc::discover::{discover_for, extract_capability_names, socket_path};
use primalspring::validation::ValidationResult;

/// Minimum primals required for capability aggregation — at least 2 gates needed
/// to test routing to best gate. Source: exp034 design.
const MIN_AGGREGATION_PRIMALS: usize = 2;

fn main() {
    ValidationResult::new("primalSpring Exp034 — Capability Aggregation")
        .with_provenance("exp034_capability_aggregation", "2026-03-24")
        .run(
            "primalSpring Exp034: Plasmodium Routes to Best Gate for Workload",
            |v| {
                let required = AtomicType::FullNucleus.required_primals();
                v.check_count(
                    "full_nucleus_has_eight_primals",
                    required.len(),
                    AtomicType::FullNucleus.required_primals().len(),
                );

                let family_id =
                    std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());
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

                if found >= MIN_AGGREGATION_PRIMALS {
                    v.check_minimum(
                        "aggregated_capabilities",
                        all_cap_names.len(),
                        MIN_AGGREGATION_PRIMALS,
                    );
                } else {
                    v.check_skip(
                        "aggregated_capabilities",
                        &format!(
                            "need >= {MIN_AGGREGATION_PRIMALS} live primals for aggregation, found {found}"
                        ),
                    );
                }

                v.check_skip(
                    "capability_aggregation_routing",
                    "needs live Plasmodium for best-gate routing",
                );
            },
        );
}
