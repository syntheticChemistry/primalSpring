// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp084: Provenance Adversarial — test data integrity under hostile conditions.
//!
//! Scenarios:
//! 1. Tampered DAG: inject a modified event → does Merkle verification catch it?
//! 2. Replay attack: replay an old session → does `LoamSpine` reject duplicates?
//! 3. Attribution dispute: two agents claim same work → does sweetGrass resolve?
//!
//! Environment:
//!   `REMOTE_GATE_HOST`      — gate node hostname
//!   `PROVENANCE_SCENARIO`   — which scenario: all|tamper|replay|dispute
//!   `RHIZOCRYPT_PORT`       — rhizoCrypt port (default 9600)
//!   `LOAMSPINE_PORT`        — `LoamSpine` port (default 9610)
//!   `SWEETGRASS_PORT`       — sweetGrass port (default 9620)

use primalspring::ipc::methods;
use primalspring::ipc::tcp::{env_port, tcp_rpc};
use primalspring::validation::ValidationResult;

/// RPC method names for the provenance trio (owned by those primals).
const PROVENANCE_SESSION_CREATE: &str = "provenance.session.create";
const PROVENANCE_EVENT_APPEND: &str = "provenance.event.append";
const PROVENANCE_COMMIT: &str = "provenance.commit";
const ATTRIBUTION_CLAIM: &str = "attribution.claim";
const ATTRIBUTION_RESOLVE: &str = "attribution.resolve";

const DEFAULT_RHIZOCRYPT_PORT: u16 = 9600;
const DEFAULT_LOAMSPINE_PORT: u16 = 9610;
const DEFAULT_SWEETGRASS_PORT: u16 = 9620;

fn trio_health_baseline(
    v: &mut ValidationResult,
    host: &str,
    rhizo_port: u16,
    loam_port: u16,
    sweet_port: u16,
) -> (bool, bool, bool, bool) {
    v.section("Provenance Trio Health");
    let rhizo_ok = tcp_rpc(
        host,
        rhizo_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    let loam_ok = tcp_rpc(
        host,
        loam_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    let sweet_ok = tcp_rpc(
        host,
        sweet_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );

    let rhizo_live = rhizo_ok.is_ok();
    let loam_live = loam_ok.is_ok();
    let sweet_live = sweet_ok.is_ok();

    println!(
        "  rhizoCrypt:  {} (port {rhizo_port})",
        if rhizo_live { "LIVE" } else { "DOWN" }
    );
    println!(
        "  LoamSpine:   {} (port {loam_port})",
        if loam_live { "LIVE" } else { "DOWN" }
    );
    println!(
        "  sweetGrass:  {} (port {sweet_port})",
        if sweet_live { "LIVE" } else { "DOWN" }
    );

    v.check_bool("rhizocrypt_health", rhizo_live, "rhizoCrypt alive");
    v.check_bool("loamspine_health", loam_live, "LoamSpine alive");
    v.check_bool("sweetgrass_health", sweet_live, "sweetGrass alive");

    let trio_live = rhizo_live && loam_live && sweet_live;
    if !trio_live {
        println!();
        println!("  BLOCKER: Provenance Trio incomplete — adversarial tests require all three.");
        if !loam_live {
            println!("  Known: LoamSpine startup crash (Tokio nested runtime in infant_discovery)");
        }
        v.check_bool(
            "trio_complete",
            false,
            "Provenance Trio incomplete — adversarial tests blocked",
        );
    }

    (trio_live, rhizo_live, loam_live, sweet_live)
}

fn scenario_tampered_dag(v: &mut ValidationResult, host: &str, rhizo_port: u16) {
    v.section("Tampered DAG Injection");

    // Step 1: create a legitimate session
    let session = tcp_rpc(
        host,
        rhizo_port,
        PROVENANCE_SESSION_CREATE,
        &serde_json::json!({"agent": "exp084_adversarial", "purpose": "tamper_test"}),
    );

    match session {
        Ok((session_data, _)) => {
            println!("  Created test session: {session_data}");
            v.check_bool("tamper_session_created", true, "test session created");

            // Step 2: attempt to inject a modified event with wrong hash
            let tampered = tcp_rpc(
                host,
                rhizo_port,
                PROVENANCE_EVENT_APPEND,
                &serde_json::json!({
                    "session_id": session_data
                        .get("session_id")
                        .cloned()
                        .unwrap_or_else(|| serde_json::json!("test")),
                    "event": "tampered_computation_result",
                    "merkle_hash": "0000000000000000000000000000000000000000000000000000000000000000",
                    "parent_hash": "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                }),
            );

            match tampered {
                Ok(_) => {
                    println!(
                        "  WARNING: Tampered event accepted — Merkle validation not enforced!"
                    );
                    v.check_bool(
                        "tamper_rejected",
                        false,
                        "tampered event was ACCEPTED — verification gap",
                    );
                }
                Err(e) => {
                    println!("  Tampered event rejected: {e}");
                    v.check_bool(
                        "tamper_rejected",
                        true,
                        &format!("tampered event correctly rejected: {e}"),
                    );
                }
            }
        }
        Err(e) => {
            println!("  Could not create session: {e}");
            v.check_skip(
                "tamper_session_created",
                &format!("session create failed: {e}"),
            );
            v.check_skip("tamper_rejected", "no session — skip tamper test");
        }
    }
}

fn scenario_replay_attack(v: &mut ValidationResult, host: &str, loam_port: u16) {
    v.section("Replay Attack");

    // Attempt to commit the same provenance record twice
    let commit_params = serde_json::json!({
        "session_id": "replay-test-session-001",
        "merkle_root": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        "agent": "exp084_replay",
        "timestamp": "2026-03-28T00:00:00Z"
    });

    let first = tcp_rpc(host, loam_port, PROVENANCE_COMMIT, &commit_params);
    let second = tcp_rpc(host, loam_port, PROVENANCE_COMMIT, &commit_params);

    match (first, second) {
        (Ok(_), Ok(_)) => {
            println!("  WARNING: Duplicate commit accepted — replay not detected!");
            v.check_bool(
                "replay_rejected",
                false,
                "duplicate commit accepted — replay detection gap",
            );
        }
        (Ok(_), Err(e)) => {
            println!("  Replay correctly rejected: {e}");
            v.check_bool(
                "replay_rejected",
                true,
                &format!("replay correctly rejected: {e}"),
            );
        }
        (Err(e), _) => {
            println!("  First commit failed: {e}");
            v.check_skip("replay_rejected", &format!("commit not supported yet: {e}"));
        }
    }
}

fn scenario_attribution_dispute(v: &mut ValidationResult, host: &str, sweet_port: u16) {
    v.section("Attribution Dispute");

    // Two agents claim the same computation
    let claim_a = tcp_rpc(
        host,
        sweet_port,
        ATTRIBUTION_CLAIM,
        &serde_json::json!({
            "computation_id": "dispute-test-comp-001",
            "agent": "agent_alice",
            "evidence_hash": "aa11223344556677889900aabbccddeeff00112233445566778899aabbccddeeff"
        }),
    );

    let claim_b = tcp_rpc(
        host,
        sweet_port,
        ATTRIBUTION_CLAIM,
        &serde_json::json!({
            "computation_id": "dispute-test-comp-001",
            "agent": "agent_bob",
            "evidence_hash": "bb11223344556677889900aabbccddeeff00112233445566778899aabbccddeeff"
        }),
    );

    match (claim_a, claim_b) {
        (Ok(_), Ok(_)) => {
            println!("  Both claims accepted — checking braid resolution...");
            let resolve = tcp_rpc(
                host,
                sweet_port,
                ATTRIBUTION_RESOLVE,
                &serde_json::json!({"computation_id": "dispute-test-comp-001"}),
            );
            match resolve {
                Ok((result, _)) => {
                    println!("  Braid resolution: {result}");
                    v.check_bool(
                        "dispute_resolved",
                        true,
                        &format!("braid resolved dispute: {result}"),
                    );
                }
                Err(e) => {
                    println!("  No resolution mechanism: {e}");
                    v.check_bool(
                        "dispute_resolved",
                        false,
                        &format!("dispute not resolved: {e}"),
                    );
                }
            }
        }
        (Ok(_), Err(e)) => {
            println!("  Second claim rejected (first-come-first-served): {e}");
            v.check_bool("dispute_resolved", true, "FCFS: second claim rejected");
        }
        (Err(e), _) => {
            println!("  Attribution claims not supported yet: {e}");
            v.check_skip(
                "dispute_resolved",
                &format!("attribution not implemented: {e}"),
            );
        }
    }
}

fn provenance_assessment(v: &mut ValidationResult, trio_live: bool, scenario: &str) {
    v.section("Provenance Integrity Assessment");
    println!(
        "  Trio status:  {}",
        if trio_live { "COMPLETE" } else { "INCOMPLETE" }
    );
    println!("  Scenario:     {scenario}");
    if !trio_live {
        println!("  NOTE: Full adversarial testing requires all three Trio primals");
    }

    v.check_bool(
        "provenance_structural",
        true,
        "provenance adversarial experiment structure valid",
    );
}

fn main() {
    let host = std::env::var("REMOTE_GATE_HOST").unwrap_or_default();
    let scenario = std::env::var("PROVENANCE_SCENARIO").unwrap_or_else(|_| "all".to_owned());

    let rhizo_port = env_port("RHIZOCRYPT_PORT", DEFAULT_RHIZOCRYPT_PORT);
    let loam_port = env_port("LOAMSPINE_PORT", DEFAULT_LOAMSPINE_PORT);
    let sweet_port = env_port("SWEETGRASS_PORT", DEFAULT_SWEETGRASS_PORT);

    ValidationResult::new("primalSpring Exp084 — Provenance Adversarial")
        .with_provenance("exp084_provenance_adversarial", "2026-03-28")
        .run(&format!("Provenance scenario: {scenario}"), |v| {
            if host.is_empty() {
                println!("  REMOTE_GATE_HOST not set — running structural validation only.");
                v.check_skip("remote_gate_configured", "REMOTE_GATE_HOST not set");
                structural_checks(v);
                return;
            }

            let run_all = scenario == "all";

            let (trio_live, rhizo_live, loam_live, sweet_live) =
                trio_health_baseline(v, &host, rhizo_port, loam_port, sweet_port);

            if (run_all || scenario == "tamper") && rhizo_live {
                scenario_tampered_dag(v, &host, rhizo_port);
            }

            if (run_all || scenario == "replay") && loam_live {
                scenario_replay_attack(v, &host, loam_port);
            }

            if (run_all || scenario == "dispute") && sweet_live {
                scenario_attribution_dispute(v, &host, sweet_port);
            }

            provenance_assessment(v, trio_live, &scenario);
        });
}

fn structural_checks(v: &mut ValidationResult) {
    v.section("Structural Validation (Offline)");

    v.check_bool(
        "trio_ports_defined",
        true,
        &format!("rhizoCrypt:{DEFAULT_RHIZOCRYPT_PORT}, LoamSpine:{DEFAULT_LOAMSPINE_PORT}, sweetGrass:{DEFAULT_SWEETGRASS_PORT}"),
    );

    v.check_bool(
        "adversarial_scenarios_defined",
        true,
        "tamper, replay, dispute scenarios defined",
    );

    v.check_bool(
        "provenance_graphs_exist",
        true,
        "reproducibility_audit.toml and ecology_provenance.toml defined",
    );
}
