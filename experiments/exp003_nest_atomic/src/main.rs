// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp003: Nest Atomic — validates beardog, songbird, nestgate socket discovery and Nest required primals (Tower + NestGate).

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp003 — Nest Atomic");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp003: Nest Atomic (Tower + NestGate)");
    println!("{}", "=".repeat(72));

    // Real check: beardog socket exists
    let beardog = discover_primal("beardog");
    v.check_bool(
        "socket_beardog",
        beardog.socket.is_some(),
        &format!(
            "beardog socket {} (source: {:?})",
            if beardog.socket.is_some() {
                "found"
            } else {
                "not found"
            },
            beardog.source
        ),
    );

    // Real check: songbird socket exists
    let songbird = discover_primal("songbird");
    v.check_bool(
        "socket_songbird",
        songbird.socket.is_some(),
        &format!(
            "songbird socket {} (source: {:?})",
            if songbird.socket.is_some() {
                "found"
            } else {
                "not found"
            },
            songbird.source
        ),
    );

    // Real check: nestgate socket exists
    let nestgate = discover_primal("nestgate");
    v.check_bool(
        "socket_nestgate",
        nestgate.socket.is_some(),
        &format!(
            "nestgate socket {} (source: {:?})",
            if nestgate.socket.is_some() {
                "found"
            } else {
                "not found"
            },
            nestgate.source
        ),
    );

    // Real check: AtomicType::Nest.required_primals() matches expected (3)
    let nest_primals = AtomicType::Nest.required_primals();
    v.check_bool(
        "nest_required_count",
        nest_primals.len() == 3,
        &format!("Nest requires {} primals (expected 3)", nest_primals.len()),
    );

    // Skip: storage.store needs live primals
    v.check_skip("storage_store", "needs live primals");

    // Skip: storage.retrieve needs live primals
    v.check_skip("storage_retrieve", "needs live primals");

    v.finish();
    std::process::exit(v.exit_code());
}
