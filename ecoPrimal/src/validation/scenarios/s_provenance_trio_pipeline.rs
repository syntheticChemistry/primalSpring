// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scenario: Provenance Trio Pipeline — full `nest.store` signal validation.
//!
//! Exercises the four-step provenance pipeline defined by `nest_store.toml`:
//! `content.put` (NestGate) → `dag.event.append` (rhizoCrypt) →
//! `spine.seal` (loamSpine) → `braid.create` (sweetGrass).
//!
//! This is the base pattern for all NUCLEUS provenance artifacts (playbook
//! Artifact 1). Every step must be discoverable via capability routing and
//! produce the expected response shape.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "provenance-trio-pipeline",
        track: Track::AtomicComposition,
        tier: Tier::Live,
        provenance_crate: "nucleus_playbook_artifact1",
        provenance_date: "2026-05-16",
        description: "Provenance trio: content.put → dag.event.append → spine.seal → braid.create",
    },
    run,
};

/// Run the provenance trio pipeline validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Capability discovery — provenance quartet");
    phase_discovery(v, ctx);

    v.section("Phase 2: content.put (NestGate CAS store)");
    let hash = phase_content_put(v, ctx);

    v.section("Phase 3: dag.event.append (rhizoCrypt DAG vertex)");
    let vertex = phase_dag_append(v, ctx, hash.as_deref());

    v.section("Phase 4: spine.seal (loamSpine ledger commit)");
    phase_spine_seal(v, ctx, vertex.as_deref());

    v.section("Phase 5: braid.create (sweetGrass attribution)");
    phase_braid_create(v, ctx);

    v.section("Phase 6: signal dispatch — nest.store via Neural API");
    phase_signal_dispatch(v, ctx);
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    for (cap, label) in [
        ("content", "NestGate storage"),
        ("dag", "rhizoCrypt DAG"),
        ("spine", "loamSpine ledger"),
        ("braid", "sweetGrass attribution"),
    ] {
        let found = ctx.has_capability(cap);
        v.check_bool(
            &format!("trio:discover:{cap}"),
            found,
            &format!("{label} — {}", if found { "resolved" } else { "not discoverable" }),
        );
    }
}

fn phase_content_put(v: &mut ValidationResult, ctx: &mut CompositionContext) -> Option<String> {
    let data = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        b"provenance-trio-pipeline scenario - primalSpring playbook validation",
    );

    match ctx.call("content", "content.put", serde_json::json!({ "data": data })) {
        Ok(resp) => {
            let hash = resp.get("hash").and_then(|h| h.as_str()).unwrap_or("");
            v.check_bool(
                "trio:content_put:hash",
                hash.len() == 64,
                &format!("BLAKE3 hash: {}... ({})", &hash[..hash.len().min(16)], hash.len()),
            );
            if hash.is_empty() { None } else { Some(hash.to_owned()) }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("trio:content_put:hash", &format!("NestGate not available: {e}"));
            None
        }
        Err(e) => {
            v.check_bool("trio:content_put:hash", false, &format!("content.put error: {e}"));
            None
        }
    }
}

fn phase_dag_append(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    hash: Option<&str>,
) -> Option<String> {
    let Some(hash) = hash else {
        v.check_skip("trio:dag_append:vertex", "no hash from content.put");
        return None;
    };

    match ctx.call(
        "dag",
        "dag.event.append",
        serde_json::json!({ "event": { "type": "data", "payload": hash } }),
    ) {
        Ok(resp) => {
            let vertex_id = resp.get("vertex_id").and_then(|v| v.as_str()).unwrap_or("");
            let has_root = resp.get("merkle_root").is_some();
            v.check_bool(
                "trio:dag_append:vertex",
                !vertex_id.is_empty(),
                &format!("vertex_id: {vertex_id}, merkle_root present: {has_root}"),
            );
            if vertex_id.is_empty() { None } else { Some(vertex_id.to_owned()) }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("trio:dag_append:vertex", &format!("rhizoCrypt not available: {e}"));
            None
        }
        Err(e) => {
            v.check_bool("trio:dag_append:vertex", false, &format!("dag.event.append error: {e}"));
            None
        }
    }
}

fn phase_spine_seal(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    vertex_id: Option<&str>,
) {
    let Some(vertex_id) = vertex_id else {
        v.check_skip("trio:spine_seal:sealed", "no vertex from dag.event.append");
        return;
    };

    match ctx.call(
        "spine",
        "spine.seal",
        serde_json::json!({ "vertex_id": vertex_id }),
    ) {
        Ok(resp) => {
            let sealed = resp.get("sealed").and_then(serde_json::Value::as_bool).unwrap_or(false)
                || resp.get("spine_id").is_some()
                || resp.get("hash").is_some();
            v.check_bool(
                "trio:spine_seal:sealed",
                sealed,
                &format!("spine.seal response: {}", serde_json::to_string(&resp).unwrap_or_default()),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("trio:spine_seal:sealed", &format!("loamSpine not available: {e}"));
        }
        Err(e) => {
            v.check_bool("trio:spine_seal:sealed", false, &format!("spine.seal error: {e}"));
        }
    }
}

fn phase_braid_create(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    match ctx.call(
        "braid",
        "braid.create",
        serde_json::json!({
            "contributors": [{ "gate": "local", "weight": 1.0 }],
            "context": "provenance-trio-pipeline-scenario",
        }),
    ) {
        Ok(resp) => {
            let braid_id = resp.get("braid_id").and_then(|b| b.as_str()).unwrap_or("");
            v.check_bool(
                "trio:braid_create:id",
                !braid_id.is_empty() || resp.get("id").is_some(),
                &format!("braid response keys: {:?}", resp.as_object().map(|o| o.keys().collect::<Vec<_>>()).unwrap_or_default()),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("trio:braid_create:id", &format!("sweetGrass not available: {e}"));
        }
        Err(e) => {
            v.check_bool("trio:braid_create:id", false, &format!("braid.create error: {e}"));
        }
    }
}

fn phase_signal_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let params = serde_json::json!({
        "content": b"provenance-trio-pipeline scenario - signal dispatch validation".to_vec(),
        "author": "primalSpring:s_provenance_trio_pipeline",
    });

    match ctx.dispatch("nest.store", &params) {
        Ok(resp) => {
            let has_content_key = resp.get("content_hash").is_some()
                || resp.get("hash").is_some()
                || resp.get("result").is_some();
            v.check_bool(
                "trio:signal:nest_store:response_shape",
                has_content_key,
                &format!(
                    "dispatch('nest.store') should return result keys; got: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "trio:signal:nest_store:response_shape",
                &format!("biomeOS orchestration not available for signal dispatch: {e}"),
            );
        }
        Err(e) => {
            let detail = format!("{e}");
            if detail.contains("-32601") || detail.contains("not found") {
                v.check_skip(
                    "trio:signal:nest_store:response_shape",
                    &format!("signal.dispatch not available (pre-v3.56 biomeOS): {e}"),
                );
            } else {
                v.check_bool(
                    "trio:signal:nest_store:response_shape",
                    false,
                    &format!("nest.store signal dispatch error: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_trio_pipeline_no_panic() {
        let mut v = ValidationResult::new("provenance-trio-pipeline");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.evaluated() > 0 || v.skipped > 0, "scenario should produce checks");
    }
}
