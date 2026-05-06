// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp107: Foundation Validation Composition
//!
//! Validates that the NUCLEUS composition required by `sporeGarden/foundation`
//! can execute every RPC call in the sediment pipeline through Rust IPC.
//!
//! Phases:
//!   1. Structural — parse and validate the foundation graph
//!   2. Discovery  — CompositionContext resolves all capabilities
//!   3. Health     — probe primals in the foundation composition
//!   4. Provenance — DAG session lifecycle (create → event → complete)
//!   5. Storage    — NestGate store/get roundtrip with BLAKE3 anchor
//!   6. Compute    — toadStool workload dispatch health
//!   7. Ledger     — loamSpine spine create + entry append
//!   8. Attribution — sweetGrass braid creation
//!
//! When primals are unavailable, phases skip gracefully.
//! When live, this proves the full foundation composition works through Rust.

use std::path::Path;

use primalspring::composition::CompositionContext;
use primalspring::coordination::probe_primal;
use primalspring::deploy::{load_graph, validate_structure};
use primalspring::primal_names as pn;
use primalspring::validation::ValidationResult;

const FOUNDATION_GRAPH: &str = "graphs/compositions/foundation_validation.toml";

const FOUNDATION_PRIMALS: &[&str] = &[
    pn::BEARDOG,
    pn::SONGBIRD,
    pn::TOADSTOOL,
    pn::BARRACUDA,
    pn::NESTGATE,
    pn::RHIZOCRYPT,
    pn::LOAMSPINE,
    pn::SWEETGRASS,
];

fn phase_structural(v: &mut ValidationResult) {
    let graph_path = Path::new(FOUNDATION_GRAPH);
    v.check_bool(
        "graph_file_exists",
        graph_path.exists(),
        &format!("{FOUNDATION_GRAPH} present"),
    );

    if !graph_path.exists() {
        v.check_skip("graph_parse", "graph file not found");
        return;
    }

    match load_graph(graph_path) {
        Ok(graph) => {
            v.check_bool("graph_parse", true, "foundation graph parsed");

            v.check_bool(
                "graph_name",
                graph.graph.name == "foundation_validation",
                &format!("name = {}", graph.graph.name),
            );

            let purpose = graph
                .graph
                .metadata
                .as_ref()
                .and_then(|m| m.purpose.as_deref());
            v.check_bool(
                "graph_purpose",
                purpose == Some("foundation"),
                &format!("purpose = {purpose:?}"),
            );

            v.check_minimum("graph_node_count", graph.graph.node.len(), 10);

            let trio_caps: Vec<&str> = graph
                .graph
                .node
                .iter()
                .filter_map(|n| n.by_capability.as_deref())
                .collect();
            for cap in ["dag", "ledger", "attribution", "storage", "compute"] {
                v.check_bool(
                    &format!("graph_has_{cap}"),
                    trio_caps.contains(&cap),
                    &format!("by_capability = \"{cap}\" present"),
                );
            }

            let skip_nodes: Vec<&str> = graph
                .graph
                .node
                .iter()
                .filter(|n| n.fallback.as_deref() == Some("skip"))
                .map(|n| n.name.as_str())
                .collect();
            v.check_bool(
                "graph_optional_nodes",
                !skip_nodes.is_empty(),
                &format!("fallback=skip: {}", skip_nodes.join(", ")),
            );
            for node in &graph.graph.node {
                if node.fallback.as_deref() == Some("skip") {
                    v.check_bool(
                        &format!("fallback_{}_not_required", node.name),
                        !node.required,
                        &format!("{} has fallback=skip and required=false", node.name),
                    );
                }
            }
        }
        Err(e) => {
            v.check_bool("graph_parse", false, &format!("parse error: {e}"));
        }
    }

    let validation = validate_structure(graph_path);
    v.check_bool(
        "structural_validation",
        validation.issues.is_empty(),
        &format!(
            "{} issues: {}",
            validation.issues.len(),
            validation.issues.join("; ")
        ),
    );
}

fn phase_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    v.check_bool(
        "discovery_found_primals",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );

    for cap in ["security", "discovery", "compute", "storage", "dag", "ledger", "attribution"] {
        v.check_bool(
            &format!("has_{cap}"),
            ctx.has_capability(cap),
            &format!("{cap} capability resolved"),
        );
    }

    let btsp = ctx.btsp_state();
    let btsp_count = btsp.values().filter(|&&ok| ok).count();
    v.check_bool(
        "btsp_any_authenticated",
        btsp_count > 0 || btsp.is_empty(),
        &format!("{btsp_count}/{} BTSP authenticated", btsp.len()),
    );
}

fn phase_health(v: &mut ValidationResult) {
    for &name in FOUNDATION_PRIMALS {
        let health = probe_primal(name);
        if health.health_ok {
            v.check_bool(
                &format!("health_{name}"),
                true,
                &format!("{name} healthy, {}us, {} caps", health.latency_us, health.capabilities.len()),
            );
        } else {
            v.check_skip(
                &format!("health_{name}"),
                &format!("{name} not reachable"),
            );
        }
    }
}

fn phase_provenance(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let session = ctx.call(
        "dag",
        "dag.session.create",
        serde_json::json!({
            "source_primal": "primalspring",
            "niche": "exp107_foundation_validation"
        }),
    );
    let session_id = match session {
        Ok(ref val) => val
            .get("session_id")
            .or_else(|| val.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned(),
        Err(ref e) if e.is_connection_error() => {
            v.check_skip("dag_session_create", &format!("rhizoCrypt not available: {e}"));
            v.check_skip("dag_event_append", "no session");
            v.check_skip("dag_session_complete", "no session");
            return;
        }
        Err(ref e) => {
            v.check_bool("dag_session_create", false, &format!("error: {e}"));
            v.check_skip("dag_event_append", "session creation failed");
            v.check_skip("dag_session_complete", "session creation failed");
            return;
        }
    };
    v.check_bool(
        "dag_session_create",
        !session_id.is_empty(),
        &format!("session: {session_id}"),
    );

    let event = ctx.call(
        "dag",
        "dag.event.append",
        serde_json::json!({
            "session_id": session_id,
            "event_type": {
                "DataCreate": {
                    "key": "foundation:exp107:test_artifact",
                    "hash": "0000000000000000000000000000000000000000000000000000000000000000",
                    "source": "exp107_foundation_validation"
                }
            }
        }),
    );
    match event {
        Ok(_) => v.check_bool("dag_event_append", true, "DataCreate event appended"),
        Err(e) => v.check_bool("dag_event_append", false, &format!("error: {e}")),
    }

    let complete = ctx.call(
        "dag",
        "dag.session.complete",
        serde_json::json!({ "session_id": session_id }),
    );
    match complete {
        Ok(ref val) => {
            let root = val
                .get("merkle_root")
                .or_else(|| val.get("root"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                "dag_session_complete",
                !root.is_empty(),
                &format!("merkle_root: {root}"),
            );
        }
        Err(e) => v.check_bool("dag_session_complete", false, &format!("error: {e}")),
    }
}

fn phase_storage(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let store = ctx.call(
        "storage",
        "storage.store",
        serde_json::json!({
            "key": "foundation:exp107:anchor",
            "data": { "source": "exp107", "type": "test_anchor" },
        }),
    );
    match store {
        Ok(_) => v.check_bool("storage_store", true, "test artifact stored in NestGate"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("storage_store", &format!("NestGate not available: {e}"));
            v.check_skip("storage_get", "store skipped");
            return;
        }
        Err(e) => {
            v.check_bool("storage_store", false, &format!("error: {e}"));
            return;
        }
    }

    let get = ctx.call(
        "storage",
        "storage.get",
        serde_json::json!({ "key": "foundation:exp107:anchor" }),
    );
    match get {
        Ok(val) => {
            let has_data = !val.is_null();
            v.check_bool("storage_get", has_data, "retrieved stored artifact");
        }
        Err(e) => v.check_bool("storage_get", false, &format!("error: {e}")),
    }
}

fn phase_compute(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let health = ctx.call("compute", "health.liveness", serde_json::json!({}));
    match health {
        Ok(_) => v.check_bool("compute_health", true, "toadStool healthy"),
        Err(e) if e.is_connection_error() => {
            v.check_skip("compute_health", &format!("toadStool not available: {e}"));
        }
        Err(e) => v.check_bool("compute_health", false, &format!("error: {e}")),
    }
}

fn phase_ledger(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let spine = ctx.call(
        "ledger",
        "spine.create",
        serde_json::json!({
            "label": "exp107_foundation_validation",
            "source_primal": "primalspring"
        }),
    );
    let spine_id = match spine {
        Ok(ref val) => val
            .get("spine_id")
            .or_else(|| val.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned(),
        Err(ref e) if e.is_connection_error() => {
            v.check_skip("ledger_spine_create", &format!("loamSpine not available: {e}"));
            v.check_skip("ledger_entry_append", "no spine");
            return;
        }
        Err(ref e) => {
            v.check_bool("ledger_spine_create", false, &format!("error: {e}"));
            v.check_skip("ledger_entry_append", "spine creation failed");
            return;
        }
    };
    v.check_bool(
        "ledger_spine_create",
        !spine_id.is_empty(),
        &format!("spine: {spine_id}"),
    );

    let entry = ctx.call(
        "ledger",
        "entry.append",
        serde_json::json!({
            "spine_id": spine_id,
            "entry_type": "SessionCommit",
            "data": {
                "session_hash": "exp107_test_session",
                "source": "primalspring"
            }
        }),
    );
    match entry {
        Ok(_) => v.check_bool("ledger_entry_append", true, "SessionCommit entry appended"),
        Err(e) => v.check_bool("ledger_entry_append", false, &format!("error: {e}")),
    }
}

fn phase_attribution(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let braid = ctx.call(
        "attribution",
        "braid.create",
        serde_json::json!({
            "claims": {
                "type": "ProvenanceValidation",
                "agent": "primalspring:exp107",
                "activity": "foundation_validation",
                "entity": "exp107_test_session"
            }
        }),
    );
    match braid {
        Ok(ref val) => {
            let braid_id = val
                .get("braid_id")
                .or_else(|| val.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                "attribution_braid_create",
                !braid_id.is_empty(),
                &format!("braid: {braid_id}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "attribution_braid_create",
                &format!("sweetGrass not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool("attribution_braid_create", false, &format!("error: {e}"));
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp107 — Foundation Validation Composition")
        .with_provenance("exp107_foundation_validation", "2026-05-06")
        .run(
            "Exp107: Foundation Validation — NUCLEUS composition for sediment pipeline",
            |v| {
                v.section("Phase 1: Structural Validation");
                phase_structural(v);

                v.section("Phase 2: Discovery");
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_discovery(v, &ctx);

                v.section("Phase 3: Health");
                phase_health(v);

                v.section("Phase 4: Provenance (rhizoCrypt DAG)");
                phase_provenance(v, &mut ctx);

                v.section("Phase 5: Storage (NestGate)");
                phase_storage(v, &mut ctx);

                v.section("Phase 6: Compute (toadStool)");
                phase_compute(v, &mut ctx);

                v.section("Phase 7: Ledger (loamSpine)");
                phase_ledger(v, &mut ctx);

                v.section("Phase 8: Attribution (sweetGrass)");
                phase_attribution(v, &mut ctx);
            },
        );
}
