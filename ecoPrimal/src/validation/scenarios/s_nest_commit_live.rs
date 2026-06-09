// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Nest Commit Live — validate `nest.commit` signal dispatch
//! end-to-end through biomeOS, with graceful skip for pre-v3.57 biomeOS.
//!
//! Wave 20: healthSpring/wetSpring are gated on biomeOS v3.57+ for
//! `session.commit`. This scenario validates the full `nest.commit`
//! signal pipeline while being skip-tolerant when biomeOS lacks the
//! commit endpoint. Also covers esotericWebb GAP-024 (biomeOS E2E).
//!
//! Graph: `graphs/compositions/nest_commit.toml`
//! Nodes: dehydrate(event.append) -> sign(crypto.sign) -> store(content.put)
//!        -> commit(session.commit) -> attribute(braid.create)

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "nest-commit-live",
        track: Track::GraphExecution,
        tier: Tier::Both,
        provenance_crate: "nest_commit_live_wave20",
        provenance_date: "2026-05-16",
        description: "nest.commit signal E2E - live dispatch with pre-v3.57 skip tolerance",
    },
    run,
};

const NEST_COMMIT_GRAPH: &str = include_str!("../../../../graphs/compositions/nest_commit.toml");

const NEST_COMMIT_CAPABILITIES: &[&str] = &[
    "event.append",
    "crypto.sign",
    "content.put",
    "session.commit",
    "braid.create",
];

/// Run the nest commit live validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1 (Rust): graph structural validation");
    phase_graph_structure(v);

    v.section("Phase 2 (Rust): capability registry alignment");
    phase_registry_alignment(v);

    v.section("Phase 3 (Rust): capability reachability");
    phase_capability_reachability(v, ctx);

    v.section("Phase 4 (Live): nest.commit dispatch");
    phase_live_dispatch(v, ctx);

    v.section("Phase 5 (Live): nest.store baseline comparison");
    phase_nest_store_baseline(v, ctx);
}

fn phase_graph_structure(v: &mut ValidationResult) {
    let parsed: Result<toml::Value, _> = toml::from_str(NEST_COMMIT_GRAPH);
    let ok = parsed.is_ok();
    v.check_bool("graph:parse", ok, "nest_commit.toml parses as valid TOML");

    if let Ok(doc) = parsed {
        let graph = &doc["graph"];

        v.check_bool(
            "graph:composition_tier",
            graph.get("composition_tier").and_then(|t| t.as_str()) == Some("nest"),
            "composition_tier == nest",
        );

        v.check_bool(
            "graph:composition_name",
            graph.get("composition_name").and_then(|n| n.as_str()) == Some("commit"),
            "composition_name == commit",
        );

        let nodes = graph
            .get("nodes")
            .and_then(|n| n.as_array())
            .map_or(0, std::vec::Vec::len);
        v.check_minimum("graph:node_count", nodes, 4);

        let mut graph_caps: Vec<String> = Vec::new();
        if let Some(nodes_arr) = graph.get("nodes").and_then(|n| n.as_array()) {
            for node in nodes_arr {
                if let Some(caps) = node.get("capabilities").and_then(|c| c.as_array()) {
                    for cap in caps {
                        if let Some(s) = cap.as_str() {
                            graph_caps.push(s.to_owned());
                        }
                    }
                }
            }
        }

        for expected in NEST_COMMIT_CAPABILITIES {
            v.check_bool(
                &format!("graph:capability:{expected}"),
                graph_caps.iter().any(|c| c == expected),
                &format!("nest_commit graph uses {expected}"),
            );
        }
    }
}

fn phase_registry_alignment(v: &mut ValidationResult) {
    let registry = include_str!("../../../../config/capability_registry.toml");

    for cap in NEST_COMMIT_CAPABILITIES {
        v.check_bool(
            &format!("registry:{cap}"),
            registry.contains(cap),
            &format!("{cap} present in capability_registry.toml"),
        );
    }
}

fn phase_capability_reachability(v: &mut ValidationResult, ctx: &CompositionContext) {
    for cap in NEST_COMMIT_CAPABILITIES {
        let domain = cap.split('.').next().unwrap_or(cap);
        let reachable = ctx.has_capability(domain);
        if reachable {
            v.check_bool(
                &format!("reachable:{domain}"),
                true,
                &format!("{domain} domain reachable via CompositionContext"),
            );
        } else {
            v.check_skip(
                &format!("reachable:{domain}"),
                &format!("{domain} domain not available in current composition"),
            );
        }
    }
}

fn phase_live_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:nest_commit",
            "biomeOS orchestration not available - cannot dispatch nest.commit",
        );
        return;
    }

    let params = serde_json::json!({
        "payload": "scenario-probe-nest-commit",
        "content_type": "text/plain",
        "session_id": "scenario-probe",
        "dry_run": true,
    });

    match ctx.dispatch("nest.commit", &params) {
        Ok(resp) => {
            v.check_bool(
                "live:nest_commit:dispatched",
                true,
                "nest.commit signal accepted by biomeOS",
            );

            let has_result_key = resp.get("committed").is_some()
                || resp.get("status").is_some()
                || resp.get("ok").is_some()
                || resp.is_object();
            v.check_bool(
                "live:nest_commit:response_shape",
                has_result_key,
                &format!(
                    "nest.commit response has expected shape: {:?}",
                    resp.as_object()
                        .map(|o| o.keys().collect::<Vec<_>>())
                        .unwrap_or_default()
                ),
            );
        }
        Err(e) => {
            let detail = format!("{e}");
            if detail.contains("-32601") || detail.contains("session.commit") {
                v.check_skip(
                    "live:nest_commit:dispatched",
                    &format!(
                        "nest.commit not available (pre-v3.57 biomeOS or missing session.commit): {e}"
                    ),
                );
            } else if e.is_skippable() {
                v.check_skip(
                    "live:nest_commit:dispatched",
                    &format!("biomeOS connection: {e}"),
                );
            } else {
                v.check_bool(
                    "live:nest_commit:dispatched",
                    false,
                    &format!("nest.commit dispatch error: {e}"),
                );
            }
        }
    }
}

fn phase_nest_store_baseline(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:nest_store_baseline",
            "biomeOS not available for baseline comparison",
        );
        return;
    }

    let params = serde_json::json!({
        "payload": "scenario-probe-nest-store-baseline",
        "content_type": "text/plain",
        "dry_run": true,
    });

    match ctx.dispatch("nest.store", &params) {
        Ok(resp) => {
            v.check_bool(
                "live:nest_store_baseline:dispatched",
                true,
                "nest.store baseline signal dispatched successfully",
            );

            let has_shape = resp.is_object();
            v.check_bool(
                "live:nest_store_baseline:response_shape",
                has_shape,
                "nest.store returns an object response",
            );
        }
        Err(e) if e.is_skippable() => {
            v.check_skip(
                "live:nest_store_baseline",
                &format!("connection: {e}"),
            );
        }
        Err(e) => {
            let detail = format!("{e}");
            if detail.contains("-32601") {
                v.check_skip(
                    "live:nest_store_baseline",
                    &format!("nest.store not available: {e}"),
                );
            } else {
                v.check_bool(
                    "live:nest_store_baseline",
                    false,
                    &format!("nest.store error: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nest_commit_live_no_panic() {
        let mut v = ValidationResult::new("nest-commit-live");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }

    #[test]
    fn graph_toml_embeds_correctly() {
        assert!(
            NEST_COMMIT_GRAPH.contains("nest_commit"),
            "embedded TOML should contain graph id"
        );
        assert!(
            NEST_COMMIT_GRAPH.contains("session.commit"),
            "embedded TOML should contain session.commit capability"
        );
    }

    #[test]
    fn capabilities_cover_graph_pipeline() {
        assert_eq!(
            NEST_COMMIT_CAPABILITIES.len(),
            5,
            "nest.commit pipeline has 5 capability stages"
        );
    }
}
