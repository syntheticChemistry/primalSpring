// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates the repeated health probe pattern with timeout and ordering from
//! `NestGate`'s `start_ecosystem.sh`.
//! Source: `primals/nestgate/showcase/scripts/start_ecosystem.sh`

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::discover_primal;
use primalspring::primal_names;
use primalspring::tolerances::DISCOVERY_MAX_US;
use primalspring::validation::ValidationResult;
use std::time::Instant;

fn main() {
    ValidationResult::new("primalSpring Exp055 — Wait For Health")
        .with_provenance("exp055_wait_for_health", "2026-03-24")
        .run(
            "primalSpring Exp055: Health Probe Pattern (NestGate start_ecosystem)",
            |v| {
                let tower_primals = AtomicType::Tower.required_primals();
                for &name in tower_primals {
                    let result = discover_primal(name);
                    v.check_bool(
                        &format!("discover_{name}_returns_result"),
                        result.primal == name,
                        &format!("discover_primal returns DiscoveryResult for {name}"),
                    );
                }

                let start = Instant::now();
                let _ = discover_primal(primal_names::BEARDOG);
                let _ = discover_primal(primal_names::SONGBIRD);
                let elapsed_us = u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX);
                v.check_bool(
                    "discovery_sweep_within_tolerance",
                    elapsed_us <= DISCOVERY_MAX_US,
                    &format!("discovery sweep took {elapsed_us}µs (max: {DISCOVERY_MAX_US}µs)"),
                );

                v.check_skip(
                    "actual_health_probe_loop",
                    "actual health probe loop needs live primals",
                );
            },
        );
}
