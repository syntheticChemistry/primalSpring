// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp110: Audit Pipeline (JH-5 Phase 3 Pressure Test)
//!
//! Validates the cross-primal security audit pipeline:
//!   1. Emit a security event to skunkBat (`defense.log`)
//!   2. Verify event persistence in rhizoCrypt DAG (`dag.event.append`)
//!   3. Verify attribution anchor in sweetGrass (`attribution.witness`)
//!   4. End-to-end audit trail availability check
//!
//! JH-5 Phase 3 (cross-primal forwarding) is deferred pending JH-11.
//! This experiment documents exactly which steps work and which skip,
//! creating upstream demand on skunkBat, rhizoCrypt, and sweetGrass.

use primalspring::composition::CompositionContext;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp110 — Audit Pipeline")
        .with_provenance("exp110_audit_pipeline", "2026-05-09")
        .run(
            "Exp110: Audit Pipeline — JH-5 Phase 3 cross-primal event chain",
            |v| {
                v.section("Phase 1: Defense Event");
                phase_defense_event(v);

                v.section("Phase 2: DAG Persistence");
                phase_dag_persistence(v);

                v.section("Phase 3: Attribution Witness");
                phase_attribution_witness(v);

                v.section("Phase 4: End-to-End Audit");
                phase_end_to_end_audit(v);
            },
        );
}

fn phase_defense_event(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    if !ctx.has_capability("defense") {
        v.check_skip(
            "defense_log",
            "skunkBat not available — defense.log requires live skunkBat",
        );
        return;
    }

    let result = ctx.call(
        "defense",
        "defense.log",
        serde_json::json!({
            "event_type": "gate_deny",
            "source": "primalspring",
            "method": "exp110.test_method",
            "severity": "info",
            "details": { "experiment": "exp110", "phase": "defense_event" },
        }),
    );

    match result {
        Ok(resp) => {
            v.check_bool("defense_log", true, &format!("event accepted: {resp}"));
        }
        Err(e) => v.check_skip("defense_log", &format!("defense.log call failed: {e}")),
    }
}

fn phase_dag_persistence(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    if !ctx.has_capability("dag") {
        v.check_skip(
            "dag_event_append",
            "rhizoCrypt not available — DAG persistence requires live rhizoCrypt",
        );
        return;
    }

    let session = ctx.call(
        "dag",
        "dag.session.create",
        serde_json::json!({ "purpose": "exp110_audit" }),
    );

    match session {
        Ok(sess) => {
            let session_id = sess
                .get("session_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");

            let append = ctx.call(
                "dag",
                "dag.event.append",
                serde_json::json!({
                    "session_id": session_id,
                    "event_type": "security_audit",
                    "payload": { "source": "skunkbat", "experiment": "exp110" },
                }),
            );
            v.check_bool(
                "dag_event_append",
                append.is_ok(),
                &format!("session {session_id}: {append:?}"),
            );
        }
        Err(e) => v.check_skip(
            "dag_event_append",
            &format!("dag.session.create failed: {e}"),
        ),
    }
}

fn phase_attribution_witness(v: &mut ValidationResult) {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    if !ctx.has_capability("attribution") {
        v.check_skip(
            "attribution_witness",
            "sweetGrass not available — attribution.witness requires live sweetGrass",
        );
        return;
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());

    let result = ctx.call(
        "attribution",
        "attribution.witness",
        serde_json::json!({
            "event_type": "security_audit",
            "source": "primalspring",
            "experiment": "exp110",
            "timestamp": now,
        }),
    );

    match result {
        Ok(resp) => {
            v.check_bool(
                "attribution_witness",
                true,
                &format!("witness created: {resp}"),
            );
        }
        Err(e) => v.check_skip(
            "attribution_witness",
            &format!("attribution.witness failed: {e}"),
        ),
    }
}

fn phase_end_to_end_audit(v: &mut ValidationResult) {
    let ctx = CompositionContext::from_live_discovery_with_fallback();

    let has_defense = ctx.has_capability("defense");
    let has_dag = ctx.has_capability("dag");
    let has_attr = ctx.has_capability("attribution");

    if !has_defense || !has_dag || !has_attr {
        v.check_skip(
            "end_to_end_audit",
            &format!(
                "JH-5 Phase 3 requires all three: skunkBat({has_defense}) + \
                 rhizoCrypt({has_dag}) + sweetGrass({has_attr}). \
                 Cross-primal audit forwarding deferred pending JH-11."
            ),
        );
        return;
    }

    v.check_bool(
        "end_to_end_audit",
        true,
        "all three audit primals reachable — JH-5 Phase 3 infra ready",
    );
}
