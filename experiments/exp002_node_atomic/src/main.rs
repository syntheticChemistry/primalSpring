// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp002: Node Atomic — validates beardog, songbird, toadstool socket discovery and Node required primals (Tower + ToadStool).

use primalspring::coordination::AtomicType;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp002 — Node Atomic");
    println!("{}", "=".repeat(72));
    println!("primalSpring Exp002: Node Atomic (Tower + ToadStool)");
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

    // Real check: toadstool socket exists
    let toadstool = discover_primal("toadstool");
    v.check_bool(
        "socket_toadstool",
        toadstool.socket.is_some(),
        &format!(
            "toadstool socket {} (source: {:?})",
            if toadstool.socket.is_some() {
                "found"
            } else {
                "not found"
            },
            toadstool.source
        ),
    );

    // Real check: AtomicType::Node.required_primals() matches expected (3)
    let node_primals = AtomicType::Node.required_primals();
    v.check_bool(
        "node_required_count",
        node_primals.len() == 3,
        &format!("Node requires {} primals (expected 3)", node_primals.len()),
    );

    // Skip: health checks need live primals
    v.check_skip("health_checks", "needs live primals");

    // Skip: compute.execute needs live primals
    v.check_skip("compute_execute", "needs live primals");

    v.summary();
    std::process::exit(i32::from(!v.all_passed()));
}
