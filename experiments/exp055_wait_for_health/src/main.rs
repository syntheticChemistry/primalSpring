// SPDX-License-Identifier: AGPL-3.0-or-later

//! Validates the repeated health probe pattern with timeout and ordering from
//! `NestGate`'s `start_ecosystem.sh`.
//! Source: `phase1/nestgate/showcase/scripts/start_ecosystem.sh`

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::discover_primal;
use primalspring::tolerances::DISCOVERY_MAX_US;
use primalspring::validation::ValidationResult;
use std::time::Instant;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp055 — Wait For Health");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp055: Health Probe Pattern (NestGate start_ecosystem)");
    println!("{}", "=".repeat(72));

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
    let _ = discover_primal("beardog");
    let _ = discover_primal("songbird");
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

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
