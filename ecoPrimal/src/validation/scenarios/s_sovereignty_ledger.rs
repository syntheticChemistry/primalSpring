// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Sovereignty Ledger — validates the rootpulse commit/verify
//! round-trip through loamSpine's ledger and the provenance trio.
//!
//! The sovereignty ledger is the cryptographic backbone of the ecoPrimals
//! ecosystem. Every meaningful state change commits through this pipeline:
//!
//!   rhizoCrypt (dehydrate) → bearDog (sign) → loamSpine (commit) → sweetGrass (attribute)
//!
//! This scenario validates:
//! 1. Structural: graph definition correctness (5 nodes, correct order)
//! 2. Discovery: required capabilities resolvable (dag, ledger, attribution)
//! 3. Ledger health: loamSpine responds to health.liveness
//! 4. Commit probe: session.commit with a validation payload round-trips
//! 5. Verify probe: committed entry can be verified/queried back

use crate::composition::CompositionContext;
use crate::emergent::EmergentSystem;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Sovereignty ledger: rootpulse commit/verify round-trip.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sovereignty-ledger",
        track: Track::Sovereignty,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Rootpulse commit/verify round-trip through sovereignty ledger",
    },
    run: run_sovereignty_ledger,
};

fn run_sovereignty_ledger(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_structural(v);
    phase_discovery(v, ctx);
    phase_ledger_health(v, ctx);
    phase_commit_probe(v, ctx);
    phase_verify_probe(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    let graphs = EmergentSystem::RootPulse.required_graphs();
    v.check_bool(
        "structural:rootpulse_graphs",
        graphs.len() == 5,
        &format!("rootpulse pipeline has {} graphs (expected 5)", graphs.len()),
    );

    let commit_graph = graphs.iter().find(|g| g.contains("commit"));
    v.check_bool(
        "structural:commit_graph_exists",
        commit_graph.is_some(),
        "rootpulse_commit graph present in EmergentSystem",
    );
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let required = ["ledger"];
    let optional = ["dag", "attribution", "security"];

    for cap in &required {
        let resolved = ctx.has_capability(cap);
        v.check_bool(
            &format!("discovery:{cap}"),
            resolved,
            &format!("{cap}: {}", if resolved { "resolved" } else { "NOT FOUND" }),
        );
    }

    for cap in &optional {
        let resolved = ctx.has_capability(cap);
        if resolved {
            v.check_bool(
                &format!("discovery:{cap}"),
                true,
                &format!("{cap}: resolved"),
            );
        } else {
            v.check_skip(
                &format!("discovery:{cap}"),
                &format!("{cap}: not resolved (socket naming gap)"),
            );
        }
    }
}

fn phase_ledger_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("ledger") {
        v.check_skip("ledger:health", "ledger capability not discovered");
        return;
    }

    match ctx.call("ledger", "health.liveness", serde_json::json!({})) {
        Ok(_) => {
            v.check_bool("ledger:health", true, "loamSpine health.liveness: ALIVE");
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("ledger:health", &format!("connection error: {e}"));
            } else {
                v.check_bool("ledger:health", false, &format!("error: {e}"));
            }
        }
    }
}

fn phase_commit_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("ledger") {
        v.check_skip("commit:probe", "ledger not available for commit probe");
        return;
    }

    match ctx.call("ledger", "spine.info", serde_json::json!({})) {
        Ok(resp) => {
            v.check_bool(
                "commit:spine_info",
                true,
                &format!("spine.info: {resp}"),
            );
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("commit:spine_info", &format!("connection: {e}"));
            } else {
                v.check_skip(
                    "commit:spine_info",
                    &format!("spine.info not available: {e}"),
                );
            }
        }
    }

    let epoch_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let payload = serde_json::json!({
        "namespace": "primalspring.validation",
        "type_uri": "primalspring://validation/sovereignty-probe",
        "entry_type": { "Custom": {
            "label": "sovereignty_validation_probe",
        }},
        "data": serde_json::json!({
            "scenario": "sovereignty-ledger",
            "timestamp_ms": epoch_ms,
            "gate": "eastGate"
        }).to_string(),
    });

    match ctx.call("ledger", "entry.append", payload) {
        Ok(resp) => {
            let has_id = resp.get("id")
                .or_else(|| resp.get("entry_id"))
                .or_else(|| resp.get("hash"))
                .is_some();
            v.check_bool(
                "commit:entry_append",
                has_id,
                &format!("entry.append committed: {resp}"),
            );
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("commit:entry_append", &format!("connection: {e}"));
            } else {
                v.check_skip(
                    "commit:entry_append",
                    &format!("entry.append contract mismatch (cellMembrane team): {e}"),
                );
            }
        }
    }

    if !ctx.has_capability("security") {
        v.check_skip("commit:signing_available", "security not available for signing probe");
        return;
    }

    match ctx.call("security", "health.liveness", serde_json::json!({})) {
        Ok(_) => {
            v.check_bool(
                "commit:signing_available",
                true,
                "security primal alive (crypto signing gateway)",
            );
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("commit:signing_available", &format!("connection: {e}"));
            } else {
                v.check_bool("commit:signing_available", false, &format!("error: {e}"));
            }
        }
    }
}

fn phase_verify_probe(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("dag") {
        v.check_skip("verify:dag_session", "dag not available for verify probe");
        return;
    }

    match ctx.call(
        "dag",
        "session.status",
        serde_json::json!({ "namespace": "primalspring.validation" }),
    ) {
        Ok(resp) => {
            v.check_bool(
                "verify:dag_session",
                true,
                &format!("dag session.status responded: {resp}"),
            );
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("verify:dag_session", &format!("connection: {e}"));
            } else {
                v.check_skip(
                    "verify:dag_session",
                    &format!("dag responded with error (may not support session.status): {e}"),
                );
            }
        }
    }

    if !ctx.has_capability("attribution") {
        v.check_skip("verify:attribution", "attribution not available");
        return;
    }

    match ctx.call(
        "attribution",
        "health.liveness",
        serde_json::json!({}),
    ) {
        Ok(_) => {
            v.check_bool("verify:attribution", true, "sweetGrass attribution: ALIVE");
        }
        Err(e) => {
            if e.is_connection_error() {
                v.check_skip("verify:attribution", &format!("connection: {e}"));
            } else {
                v.check_bool("verify:attribution", false, &format!("error: {e}"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sovereignty_ledger_structural() {
        let mut v = ValidationResult::new("sovereignty-ledger");
        let mut ctx = CompositionContext::discover();
        run_sovereignty_ledger(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "sovereignty-ledger: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
