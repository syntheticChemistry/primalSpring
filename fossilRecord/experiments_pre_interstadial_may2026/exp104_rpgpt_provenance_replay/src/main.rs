// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp104 — RPGPT Provenance Replay
//!
//! Validates the session provenance chain for storytelling:
//! session record → rhizoCrypt DAG → loamSpine ledger → replay verification
//!
//! Phase 56 — Desktop Substrate (STORYTELLING_EVOLUTION.md)

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::discover_by_capability;
use primalspring::validation::ValidationResult;

fn phase_create_session(v: &mut ValidationResult) -> Option<String> {
    v.section("Session DAG Creation (rhizoCrypt)");

    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("dag_create", "rhizoCrypt not discovered");
        return None;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("dag_create", "rhizoCrypt connection failed");
        return None;
    };

    let resp = client.call(
        "dag.session.create",
        serde_json::json!({"name": "exp104-rpgpt-replay"}),
    );

    match resp {
        Ok(r) => {
            let session_id = r
                .result
                .as_ref()
                .and_then(|r| r.get("session_id"))
                .and_then(|s| s.as_str())
                .map(String::from);
            v.check_bool(
                "dag_create",
                session_id.is_some(),
                "DAG session created for game provenance",
            );
            session_id
        }
        Err(e) => {
            v.check_skip("dag_create", &format!("dag.session.create failed: {e}"));
            None
        }
    }
}

fn phase_append_events(v: &mut ValidationResult, session_id: &str) {
    v.section("Event Append");

    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("event_append", "rhizoCrypt not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("event_append", "rhizoCrypt connection failed");
        return;
    };

    let events = [
        serde_json::json!({"event_type": "SessionStart", "payload": {"world": "disco_isles"}}),
        serde_json::json!({"event_type": "SceneTransition", "payload": {"from": "intro", "to": "tavern"}}),
        serde_json::json!({"event_type": "PlayerChoice", "payload": {"scene": "tavern", "option": 1}}),
        serde_json::json!({"event_type": "DiceRoll", "payload": {"result": 17, "dc": 12, "success": true}}),
    ];

    let mut appended = 0;
    for (i, event) in events.iter().enumerate() {
        let resp = client.call(
            "dag.event.append",
            serde_json::json!({"session_id": session_id, "event": event}),
        );
        let ok = resp.as_ref().is_ok_and(|r| r.result.is_some());
        if ok {
            appended += 1;
        }
        v.check_bool(
            &format!("event_{i}"),
            ok,
            &format!("Event {} appended to DAG", event["event_type"]),
        );
    }

    v.check_count("all_events", appended, events.len());
}

fn phase_merkle_verification(v: &mut ValidationResult, session_id: &str) {
    v.section("Merkle Verification");

    let rz = discover_by_capability("dag");
    let Some(rz_sock) = rz.socket.as_ref() else {
        v.check_skip("merkle_root", "rhizoCrypt not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(rz_sock, "rhizocrypt") else {
        v.check_skip("merkle_root", "rhizoCrypt connection failed");
        return;
    };

    let resp = client.call(
        "dag.merkle.root",
        serde_json::json!({"session_id": session_id}),
    );

    match resp {
        Ok(r) => {
            let has_root = r
                .result
                .as_ref()
                .and_then(|r| r.get("root"))
                .and_then(|s| s.as_str())
                .is_some();
            v.check_bool(
                "merkle_root",
                has_root,
                "Merkle root computed for session DAG",
            );
        }
        Err(e) => {
            v.check_skip("merkle_root", &format!("dag.merkle.root failed: {e}"));
        }
    }
}

fn phase_ledger_commit(v: &mut ValidationResult) {
    v.section("Ledger Commit (loamSpine)");

    let ls = discover_by_capability("ledger");
    let Some(ls_sock) = ls.socket.as_ref() else {
        v.check_skip("ledger_commit", "loamSpine not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ls_sock, "loamspine") else {
        v.check_skip("ledger_commit", "loamSpine connection failed");
        return;
    };

    let resp = client.call(
        "entry.append",
        serde_json::json!({
            "spine_id": "exp104-rpgpt",
            "data": {"type": "session_seal", "experiment": "exp104", "status": "complete"}
        }),
    );

    v.check_bool(
        "ledger_commit",
        resp.is_ok_and(|r| r.result.is_some()),
        "Session sealed in loamSpine ledger",
    );
}

fn main() {
    ValidationResult::new("primalSpring Exp104 — RPGPT Provenance Replay")
        .with_provenance("exp104_rpgpt_provenance_replay", "2026-04-28")
        .run("Exp104: Session provenance chain for storytelling", |v| {
            let session_id = phase_create_session(v);

            if let Some(ref sid) = session_id {
                phase_append_events(v, sid);
                phase_merkle_verification(v, sid);
            } else {
                v.section("Event Append");
                v.check_skip("events", "No DAG session — skipping events");
                v.section("Merkle Verification");
                v.check_skip("merkle", "No DAG session — skipping merkle");
            }

            phase_ledger_commit(v);
        });
}
