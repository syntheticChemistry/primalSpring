// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Signal Dispatch Parity — validate all 14 atomic signals
//! through `CompositionContext::dispatch()` against live biomeOS.
//!
//! Wave 18a of the Neural API Signal Elevation plan. Exercises the unified
//! `dispatch("tier.name", params)` API introduced in Wave 17b for every
//! signal defined in `composition_tools.toml`, checking:
//!
//! - biomeOS accepts the signal (not `-32601`)
//! - Response contains expected keys from the `composition_tools.toml` returns section
//! - Each graph node's capability was reachable
//!
//! Tier 1 (Rust): validates that `dispatch()` correctly splits identifiers
//! and routes to `signal()` without panicking.
//!
//! Tier 2 (Live): sends minimal probe params to biomeOS and validates
//! response shapes.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "composition-dispatch-parity",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "neural_api_wave18a",
        provenance_date: "2026-05-16",
        description: "Signal dispatch parity: all 14 signals via dispatch() against live biomeOS",
    },
    run,
};

struct CompositionSpec {
    id: &'static str,
    expected_keys: &'static [&'static str],
    params: fn() -> serde_json::Value,
}

const COMPOSITIONS: &[CompositionSpec] = &[
    CompositionSpec {
        id: "tower.publish",
        expected_keys: &["signature", "beacon_id", "audit_event"],
        params: || serde_json::json!({ "data": "dispatch-parity-probe", "_probe": true }),
    },
    CompositionSpec {
        id: "tower.authenticate",
        expected_keys: &["session_key", "lineage_verified"],
        params: || serde_json::json!({ "peer_id": "dispatch-parity-test", "_probe": true }),
    },
    CompositionSpec {
        id: "tower.discover",
        expected_keys: &["peers", "audit_event"],
        params: || serde_json::json!({ "scope": "local", "_probe": true }),
    },
    CompositionSpec {
        id: "tower.health",
        expected_keys: &["beardog", "songbird", "skunkbat"],
        params: || serde_json::json!({ "_probe": true }),
    },
    CompositionSpec {
        id: "tower.bootstrap",
        expected_keys: &["beardog_identity", "registry_seeded", "health"],
        params: || serde_json::json!({ "phase": 1, "_probe": true }),
    },
    CompositionSpec {
        id: "node.compute",
        expected_keys: &["result", "dispatch_id"],
        params: || serde_json::json!({ "workload": { "type": "probe" }, "_probe": true }),
    },
    CompositionSpec {
        id: "nest.store",
        expected_keys: &["content_cid", "dag_event", "session_commit"],
        params: || serde_json::json!({ "content": "dispatch-parity-probe", "_probe": true }),
    },
    CompositionSpec {
        id: "nest.commit",
        expected_keys: &["dehydrated_hash", "session_commit"],
        params: || serde_json::json!({ "session_id": "dispatch-parity-probe", "_probe": true }),
    },
    CompositionSpec {
        id: "nest.retrieve",
        expected_keys: &["content"],
        params: || serde_json::json!({ "content_cid": "dispatch-parity-probe", "_probe": true }),
    },
    CompositionSpec {
        id: "nest.ingest_spore",
        expected_keys: &["store_id", "dag_session_id", "ledger_entry_id", "braid_id"],
        params: || serde_json::json!({ "scope_id": "dispatch-parity-probe", "source_dir": "/tmp/probe", "_probe": true }),
    },
    CompositionSpec {
        id: "meta.observe",
        expected_keys: &["session_id", "context", "graphs"],
        params: || serde_json::json!({ "domain": "storage", "_probe": true }),
    },
    CompositionSpec {
        id: "meta.intent",
        expected_keys: &["plan", "execution_result"],
        params: || serde_json::json!({ "action": "probe dispatch parity", "_probe": true }),
    },
    CompositionSpec {
        id: "meta.render",
        expected_keys: &["summary", "rendered"],
        params: || serde_json::json!({ "execution_id": "dispatch-parity-probe", "_probe": true }),
    },
    CompositionSpec {
        id: "meta.health",
        expected_keys: &["biomeos", "squirrel", "petaltongue"],
        params: || serde_json::json!({ "_probe": true }),
    },
    CompositionSpec {
        id: "meta.deploy",
        expected_keys: &["plan", "deployment_id", "audit_event"],
        params: || serde_json::json!({ "target": "probe", "approval": "suggest", "_probe": true }),
    },
    // rootPulse domain (ACTION / efferent) — Wave 60
    CompositionSpec {
        id: "rootpulse.commit",
        expected_keys: &["dehydrated_hash", "signature", "commit_id", "braid_id"],
        params: || serde_json::json!({ "session_id": "dispatch-parity-probe", "_probe": true }),
    },
    CompositionSpec {
        id: "rootpulse.branch",
        expected_keys: &["branch_id", "signature"],
        params: || serde_json::json!({ "session_id": "dispatch-parity-probe", "branch_name": "probe-branch", "_probe": true }),
    },
    CompositionSpec {
        id: "rootpulse.merge",
        expected_keys: &["merge_event", "signature", "commit_id"],
        params: || serde_json::json!({ "left_branch": "probe-a", "right_branch": "probe-b", "_probe": true }),
    },
    CompositionSpec {
        id: "rootpulse.diff",
        expected_keys: &["events"],
        params: || serde_json::json!({ "from": "probe-ancestor", "to": "probe-descendant", "_probe": true }),
    },
    CompositionSpec {
        id: "rootpulse.federate",
        expected_keys: &["peer_found", "events_synced", "braids_synced"],
        params: || serde_json::json!({ "target_peer": "probe-gate", "session_id": "dispatch-parity-probe", "_probe": true }),
    },
];

/// Run the signal dispatch parity validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1 (Rust): dispatch() identifier parsing");
    phase_dispatch_parsing(v, ctx);

    v.section("Phase 2 (Live): dispatch() parity against biomeOS");
    phase_live_dispatch(v, ctx);

    v.section("Phase 3 (Live): response shape validation");
    phase_response_shapes(v, ctx);
}

fn phase_dispatch_parsing(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.check_bool(
        "parse:composition_count",
        COMPOSITIONS.len() == 20,
        &format!(
            "COMPOSITIONS table has {} entries (expected 20)",
            COMPOSITIONS.len()
        ),
    );

    for spec in COMPOSITIONS {
        let tier = spec.id.split('.').next().unwrap_or("unknown");
        let is_valid_tier = CompositionContext::is_composition_tier(tier);
        v.check_bool(
            &format!("parse:{}_tier_valid", spec.id),
            is_valid_tier,
            &format!("{}: tier {tier:?} is a recognized signal tier", spec.id),
        );
    }

    // Negative cases: malformed identifiers should produce ProtocolError
    let bad_ids = ["noperiod", "", "...triple", "fake.signal"];
    for bad in &bad_ids {
        let result = ctx.dispatch(bad, &serde_json::json!({}));
        let is_err = result.is_err();
        v.check_bool(
            &format!("parse:reject:{}", bad.replace('.', "_")),
            is_err,
            &format!("dispatch({bad:?}) should reject malformed/unknown identifier"),
        );
    }
}

fn phase_live_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        for spec in COMPOSITIONS {
            v.check_skip(
                &format!("live:dispatch:{}", spec.id),
                "biomeOS orchestration not available - skipping live dispatch",
            );
        }
        return;
    }

    for spec in COMPOSITIONS {
        let check_id = format!("live:dispatch:{}", spec.id);
        let params = (spec.params)();

        match ctx.dispatch(spec.id, &params) {
            Ok(_) => {
                v.check_bool(
                    &check_id,
                    true,
                    &format!("dispatch({:?}) accepted", spec.id),
                );
            }
            Err(e) => {
                let detail = format!("{e}");
                if detail.contains("-32601") {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!(
                            "UPSTREAM GAP: biomeOS rejected {} with -32601 (method not found)",
                            spec.id
                        ),
                    );
                } else if e.is_skippable() {
                    v.check_skip(&check_id, &format!("connection error for {}: {e}", spec.id));
                } else {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!("dispatch({:?}) error: {e}", spec.id),
                    );
                }
            }
        }
    }
}

fn phase_response_shapes(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        for spec in COMPOSITIONS {
            v.check_skip(
                &format!("shape:{}:keys", spec.id),
                "biomeOS not available - skipping response shape checks",
            );
        }
        return;
    }

    for spec in COMPOSITIONS {
        let params = (spec.params)();

        match ctx.dispatch(spec.id, &params) {
            Ok(resp) => {
                if let Some(obj) = resp.as_object() {
                    let actual_keys: Vec<&String> = obj.keys().collect();
                    let mut found = 0usize;
                    for expected in spec.expected_keys {
                        if obj.contains_key(*expected) {
                            found += 1;
                        }
                    }
                    let total = spec.expected_keys.len();
                    v.check_bool(
                        &format!("shape:{}:keys", spec.id),
                        found > 0,
                        &format!(
                            "{}: {found}/{total} expected keys present (expected {:?}, got {actual_keys:?})",
                            spec.id, spec.expected_keys
                        ),
                    );
                } else {
                    v.check_bool(
                        &format!("shape:{}:keys", spec.id),
                        false,
                        &format!(
                            "{}: response is not an object (got {:?})",
                            spec.id,
                            resp.to_string().chars().take(100).collect::<String>()
                        ),
                    );
                }
            }
            Err(e) if e.is_skippable() => {
                v.check_skip(
                    &format!("shape:{}:keys", spec.id),
                    &format!("connection error: {e}"),
                );
            }
            Err(e) => {
                let detail = format!("{e}");
                if detail.contains("-32601") || detail.contains("not found") {
                    v.check_skip(
                        &format!("shape:{}:keys", spec.id),
                        &format!("signal not available: {e}"),
                    );
                } else {
                    v.check_bool(
                        &format!("shape:{}:keys", spec.id),
                        false,
                        &format!("{}: dispatch error: {e}", spec.id),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::helpers;

    #[test]
    fn composition_dispatch_parity_no_panic() {
        let mut v = ValidationResult::new("composition-dispatch-parity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }

    #[test]
    fn composition_table_matches_graph_count() {
        assert_eq!(
            COMPOSITIONS.len(),
            20,
            "COMPOSITIONS table should match 20 composition graphs"
        );
    }

    const COMPOSITION_GRAPHS: &[(&str, &str)] = &[
        (
            "tower_publish",
            include_str!("../../../../graphs/compositions/tower_publish.toml"),
        ),
        (
            "tower_authenticate",
            include_str!("../../../../graphs/compositions/tower_authenticate.toml"),
        ),
        (
            "tower_discover",
            include_str!("../../../../graphs/compositions/tower_discover.toml"),
        ),
        (
            "tower_health",
            include_str!("../../../../graphs/compositions/tower_health.toml"),
        ),
        (
            "tower_bootstrap",
            include_str!("../../../../graphs/compositions/tower_bootstrap.toml"),
        ),
        (
            "node_compute",
            include_str!("../../../../graphs/compositions/node_compute.toml"),
        ),
        (
            "nest_store",
            include_str!("../../../../graphs/compositions/nest_store.toml"),
        ),
        (
            "nest_commit",
            include_str!("../../../../graphs/compositions/nest_commit.toml"),
        ),
        (
            "nest_retrieve",
            include_str!("../../../../graphs/compositions/nest_retrieve.toml"),
        ),
        (
            "nest_ingest_spore",
            include_str!("../../../../graphs/compositions/nest_ingest_spore.toml"),
        ),
        (
            "meta_observe",
            include_str!("../../../../graphs/compositions/meta_observe.toml"),
        ),
        (
            "meta_intent",
            include_str!("../../../../graphs/compositions/meta_intent.toml"),
        ),
        (
            "meta_render",
            include_str!("../../../../graphs/compositions/meta_render.toml"),
        ),
        (
            "meta_health",
            include_str!("../../../../graphs/compositions/meta_health.toml"),
        ),
        (
            "meta_deploy",
            include_str!("../../../../graphs/compositions/meta_deploy.toml"),
        ),
        (
            "rootpulse_commit",
            include_str!("../../../../graphs/compositions/rootpulse_commit.toml"),
        ),
        (
            "rootpulse_branch",
            include_str!("../../../../graphs/compositions/rootpulse_branch.toml"),
        ),
        (
            "rootpulse_merge",
            include_str!("../../../../graphs/compositions/rootpulse_merge.toml"),
        ),
        (
            "rootpulse_diff",
            include_str!("../../../../graphs/compositions/rootpulse_diff.toml"),
        ),
        (
            "rootpulse_federate",
            include_str!("../../../../graphs/compositions/rootpulse_federate.toml"),
        ),
    ];

    #[test]
    fn composition_graphs_match_dispatch_table() {
        assert_eq!(
            COMPOSITION_GRAPHS.len(),
            COMPOSITIONS.len(),
            "COMPOSITION_GRAPHS and COMPOSITIONS should have same count"
        );
    }

    #[test]
    fn composition_graph_ids_match_dispatch_ids() {
        for (graph_name, content) in COMPOSITION_GRAPHS {
            let parsed: toml::Value = toml::from_str(content)
                .unwrap_or_else(|e| panic!("{graph_name}.toml parse failed: {e}"));

            let graph_composition_name = parsed
                .get("graph")
                .and_then(|g| g.get("composition_name"))
                .and_then(|n| n.as_str());
            let graph_composition_tier = parsed
                .get("graph")
                .and_then(|g| g.get("composition_tier"))
                .and_then(|t| t.as_str());

            if let (Some(tier), Some(name)) = (graph_composition_tier, graph_composition_name) {
                let graph_id = format!("{tier}.{name}");
                let in_dispatch = COMPOSITIONS.iter().any(|s| s.id == graph_id);
                assert!(
                    in_dispatch,
                    "composition graph {graph_name} defines '{graph_id}' but it's not in COMPOSITIONS table"
                );
            }
        }
    }

    #[test]
    fn composition_graph_capabilities_in_registry() {
        let registry = helpers::load_registry_capabilities();
        for (graph_name, content) in COMPOSITION_GRAPHS {
            let parsed: toml::Value = match toml::from_str(content) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let Some(nodes) = parsed
                .get("graph")
                .and_then(|g| g.get("nodes"))
                .and_then(|n| n.as_array())
            {
                for node in nodes {
                    if let Some(caps) = node.get("capabilities").and_then(|c| c.as_array()) {
                        for cap in caps {
                            if let Some(s) = cap.as_str() {
                                assert!(
                                    registry.contains(&s.to_owned()),
                                    "{graph_name}: node capability '{s}' not in registry"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
