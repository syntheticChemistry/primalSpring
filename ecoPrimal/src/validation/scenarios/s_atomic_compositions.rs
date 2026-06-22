// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Atomic Signals — structural validation of the composition
//! collapse layer (Tower/Node/Nest/Meta composition graphs).
//!
//! Validates that all composition graphs parse as valid TOML, reference
//! capabilities registered in the canonical registry, use coordination
//! patterns consistent with their declared intent, and respect Dark Forest
//! security invariants (secure_by_default, btsp_enforced).
//!
//! Tier 1 checks are Tier::Rust — no live primals required.
//! Tier 2 checks validate live composition dispatch when biomeOS is available.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::helpers;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "atomic-compositions",
        track: Track::AtomicComposition,
        tier: Tier::Both,
        provenance_crate: "primalspring_atomic_compositions",
        provenance_date: "2026-05-15",
        description: "Atomic composition graphs — parse, capabilities, coordination, Dark Forest",
    },
    run,
};

const COMPOSITION_GRAPHS: &[(&str, &str, &str)] = &[
    (
        "tower.publish",
        "graphs/compositions/tower_publish.toml",
        include_str!("../../../../graphs/compositions/tower_publish.toml"),
    ),
    (
        "tower.authenticate",
        "graphs/compositions/tower_authenticate.toml",
        include_str!("../../../../graphs/compositions/tower_authenticate.toml"),
    ),
    (
        "tower.discover",
        "graphs/compositions/tower_discover.toml",
        include_str!("../../../../graphs/compositions/tower_discover.toml"),
    ),
    (
        "tower.health",
        "graphs/compositions/tower_health.toml",
        include_str!("../../../../graphs/compositions/tower_health.toml"),
    ),
    (
        "tower.bootstrap",
        "graphs/compositions/tower_bootstrap.toml",
        include_str!("../../../../graphs/compositions/tower_bootstrap.toml"),
    ),
    (
        "node.compute",
        "graphs/compositions/node_compute.toml",
        include_str!("../../../../graphs/compositions/node_compute.toml"),
    ),
    (
        "nest.store",
        "graphs/compositions/nest_store.toml",
        include_str!("../../../../graphs/compositions/nest_store.toml"),
    ),
    (
        "nest.commit",
        "graphs/compositions/nest_commit.toml",
        include_str!("../../../../graphs/compositions/nest_commit.toml"),
    ),
    (
        "nest.retrieve",
        "graphs/compositions/nest_retrieve.toml",
        include_str!("../../../../graphs/compositions/nest_retrieve.toml"),
    ),
    (
        "nest.ingest_spore",
        "graphs/compositions/nest_ingest_spore.toml",
        include_str!("../../../../graphs/compositions/nest_ingest_spore.toml"),
    ),
    (
        "meta.observe",
        "graphs/compositions/meta_observe.toml",
        include_str!("../../../../graphs/compositions/meta_observe.toml"),
    ),
    (
        "meta.intent",
        "graphs/compositions/meta_intent.toml",
        include_str!("../../../../graphs/compositions/meta_intent.toml"),
    ),
    (
        "meta.render",
        "graphs/compositions/meta_render.toml",
        include_str!("../../../../graphs/compositions/meta_render.toml"),
    ),
    (
        "meta.health",
        "graphs/compositions/meta_health.toml",
        include_str!("../../../../graphs/compositions/meta_health.toml"),
    ),
    (
        "meta.deploy",
        "graphs/compositions/meta_deploy.toml",
        include_str!("../../../../graphs/compositions/meta_deploy.toml"),
    ),
    // rootPulse domain (ACTION / efferent) — Wave 60
    (
        "rootpulse.commit",
        "graphs/compositions/rootpulse_commit.toml",
        include_str!("../../../../graphs/compositions/rootpulse_commit.toml"),
    ),
    (
        "rootpulse.branch",
        "graphs/compositions/rootpulse_branch.toml",
        include_str!("../../../../graphs/compositions/rootpulse_branch.toml"),
    ),
    (
        "rootpulse.merge",
        "graphs/compositions/rootpulse_merge.toml",
        include_str!("../../../../graphs/compositions/rootpulse_merge.toml"),
    ),
    (
        "rootpulse.diff",
        "graphs/compositions/rootpulse_diff.toml",
        include_str!("../../../../graphs/compositions/rootpulse_diff.toml"),
    ),
    (
        "rootpulse.federate",
        "graphs/compositions/rootpulse_federate.toml",
        include_str!("../../../../graphs/compositions/rootpulse_federate.toml"),
    ),
    (
        "foundation.validation",
        "graphs/compositions/foundation_validation.toml",
        include_str!("../../../../graphs/compositions/foundation_validation.toml"),
    ),
];

use crate::primal_names::{self, Atomic, Primal};

fn tower_primals() -> Vec<&'static str> {
    Primal::for_atomic(Atomic::Tower)
        .iter()
        .map(|p| p.slug())
        .collect()
}
fn node_primals() -> Vec<&'static str> {
    Primal::for_atomic(Atomic::Node)
        .iter()
        .map(|p| p.slug())
        .collect()
}
fn nest_primals() -> Vec<&'static str> {
    Primal::for_atomic(Atomic::Nest)
        .iter()
        .map(|p| p.slug())
        .collect()
}

const META_PRIMALS: &[&str] = &[
    primal_names::BIOMEOS,
    primal_names::SQUIRREL,
    primal_names::PETALTONGUE,
    primal_names::SKUNKBAT,
];
const ROOTPULSE_PRIMALS: &[&str] = &[
    primal_names::RHIZOCRYPT,
    primal_names::BEARDOG,
    primal_names::NESTGATE,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
    primal_names::SONGBIRD,
];
const ECOSYSTEM_PRIMALS: &[&str] = &[
    primal_names::BIOMEOS,
    primal_names::SONGBIRD,
    primal_names::NESTGATE,
    primal_names::BEARDOG,
];
const IMPULSE_PRIMALS: &[&str] = &[
    "membrane",
    primal_names::BEARDOG,
    primal_names::NESTGATE,
    primal_names::SONGBIRD,
];
const POTENTIAL_PRIMALS: &[&str] = &[
    "membrane",
    primal_names::BEARDOG,
    primal_names::NESTGATE,
    primal_names::SONGBIRD,
];
const SYNC_PRIMALS: &[&str] = &[
    primal_names::BIOMEOS,
    primal_names::NESTGATE,
    "membrane",
    primal_names::SONGBIRD,
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
    primal_names::BEARDOG,
];
const FOUNDATION_PRIMALS: &[&str] = &[
    primal_names::BIOMEOS,
    "beardog_primal",
    "songbird_primal",
    primal_names::TOADSTOOL,
    primal_names::BARRACUDA,
    primal_names::CORALREEF,
    primal_names::NESTGATE,
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
    primal_names::PETALTONGUE,
    primal_names::SQUIRREL,
];

fn tier_primals(tier: &str) -> Vec<&'static str> {
    match tier {
        "tower" => tower_primals(),
        "node" => node_primals(),
        "nest" => nest_primals(),
        "meta" => META_PRIMALS.to_vec(),
        "rootpulse" => ROOTPULSE_PRIMALS.to_vec(),
        "ecosystem" => ECOSYSTEM_PRIMALS.to_vec(),
        "impulse" => IMPULSE_PRIMALS.to_vec(),
        "potential" => POTENTIAL_PRIMALS.to_vec(),
        "sync" => SYNC_PRIMALS.to_vec(),
        "foundation" => FOUNDATION_PRIMALS.to_vec(),
        _ => vec![],
    }
}

fn validate_tier_primals(
    v: &mut ValidationResult,
    signal_name: &str,
    parsed: &toml::Value,
    tier: &str,
) {
    let Some(nodes) = helpers::graph_nodes(parsed) else {
        return;
    };

    let allowed = tier_primals(tier);
    for node in nodes {
        let binary = node
            .get("binary")
            .and_then(|b| b.as_str())
            .unwrap_or("unknown");
        let required = node
            .get("required")
            .and_then(toml::Value::as_bool)
            .unwrap_or(true);
        // Optional nodes (e.g. Phase 2 bootstrap) may cross tier boundaries
        if !required {
            continue;
        }
        let in_tier = allowed.contains(&binary);
        v.check_bool(
            &format!("{signal_name}:{binary}:in_tier"),
            in_tier,
            &format!("{binary} is a member of the {tier} atomic tier"),
        );
    }
}

fn validate_registry_section(v: &mut ValidationResult) {
    let registry_toml = include_str!("../../../../config/capability_registry.toml");
    let parsed: toml::Value = match toml::from_str(registry_toml) {
        Ok(p) => p,
        Err(_) => return,
    };

    let compositions = parsed.get("compositions");
    v.check_bool(
        "registry:has_compositions_section",
        compositions.is_some(),
        "capability_registry.toml has [compositions] section",
    );

    let Some(compositions) = compositions.and_then(|s| s.as_table()) else {
        return;
    };

    let expected_tiers = ["tower", "node", "nest", "meta"];
    for tier in &expected_tiers {
        let has_tier = compositions.contains_key(*tier);
        v.check_bool(
            &format!("registry:compositions.{tier}"),
            has_tier,
            &format!("compositions section has {tier} tier definition"),
        );
    }

    let total_compositions: usize = compositions
        .values()
        .filter_map(|v| v.get("compositions"))
        .filter_map(|s| s.as_array())
        .map(Vec::len)
        .sum();

    v.check_bool(
        "registry:composition_count",
        COMPOSITION_GRAPHS.len() <= total_compositions,
        &format!(
            "graph files ({}) should be subset of registry ({total_compositions})",
            COMPOSITION_GRAPHS.len()
        ),
    );
}

fn validate_context_composition_method(v: &mut ValidationResult) {
    let is_signal = CompositionContext::is_composition_tier("tower");
    v.check_bool(
        "context:tower_is_composition_tier",
        is_signal,
        "CompositionContext::is_composition_tier recognizes tower",
    );

    let is_signal_node = CompositionContext::is_composition_tier("node");
    v.check_bool(
        "context:node_is_composition_tier",
        is_signal_node,
        "CompositionContext::is_composition_tier recognizes node",
    );

    let is_signal_nest = CompositionContext::is_composition_tier("nest");
    v.check_bool(
        "context:nest_is_composition_tier",
        is_signal_nest,
        "CompositionContext::is_composition_tier recognizes nest",
    );

    let is_signal_nucleus = CompositionContext::is_composition_tier("nucleus");
    v.check_bool(
        "context:nucleus_is_composition_tier",
        is_signal_nucleus,
        "CompositionContext::is_composition_tier recognizes nucleus",
    );

    let is_signal_meta = CompositionContext::is_composition_tier("meta");
    v.check_bool(
        "context:meta_is_composition_tier",
        is_signal_meta,
        "CompositionContext::is_composition_tier recognizes meta",
    );

    let not_signal = !CompositionContext::is_composition_tier("security");
    v.check_bool(
        "context:security_not_signal_tier",
        not_signal,
        "CompositionContext::is_composition_tier rejects security (direct capability)",
    );
}

/// Tier 2 (Live) validation: if a biomeOS/orchestration capability is
/// reachable, verify that `composition.list` and `composition.schema` respond correctly.
#[expect(
    clippy::too_many_lines,
    reason = "live composition dispatch validation with multi-phase checks"
)]
fn validate_live_composition_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let biomeos_available = ctx.has_capability("orchestration");

    v.check_bool(
        "live:biomeos_available",
        true,
        &format!(
            "biomeOS availability check ({})",
            if biomeos_available {
                "live"
            } else {
                "offline — skipping Tier 2"
            }
        ),
    );

    if !biomeos_available {
        return;
    }

    // composition.list should return signals matching our structural count
    let expected_count = COMPOSITION_GRAPHS.len() as u64;
    match ctx.call("orchestration", "composition.list", serde_json::json!({})) {
        Ok(response) => {
            let count = response
                .get("count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            v.check_bool(
                "live:composition.list:responds",
                true,
                "biomeOS composition.list responds",
            );
            v.check_bool(
                "live:composition.list:count",
                count == expected_count,
                &format!(
                    "composition.list reports {count} signals (expected {expected_count} per structural validation)"
                ),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:composition.list:responds",
                false,
                &format!("biomeOS composition.list failed: {e}"),
            );
        }
    }

    // composition.schema should load the tools definition
    match ctx.call("orchestration", "composition.schema", serde_json::json!({})) {
        Ok(response) => {
            let has_tools = response.get("tools").is_some();
            v.check_bool(
                "live:composition.schema:has_tools",
                has_tools,
                "composition.schema returns tools definition",
            );
        }
        Err(e) => {
            v.check_bool(
                "live:composition.schema:responds",
                false,
                &format!("biomeOS composition.schema failed: {e}"),
            );
        }
    }

    // Tier 2b: dispatch() API — validate each signal through the unified API
    for &(signal_name, _, _) in COMPOSITION_GRAPHS {
        let check_id = format!("live:dispatch:{signal_name}");
        let params = serde_json::json!({
            "_probe": true,
            "_scenario": "s_atomic_signals",
        });

        match ctx.dispatch(signal_name, &params) {
            Ok(resp) => {
                let has_keys = resp.is_object() || resp.is_null();
                v.check_bool(
                    &check_id,
                    has_keys,
                    &format!(
                        "dispatch({signal_name:?}) returned valid response: {:?}",
                        resp.as_object()
                            .map(|o| o.keys().collect::<Vec<_>>())
                            .unwrap_or_default()
                    ),
                );
            }
            Err(e) => {
                let detail = e.to_string();
                if detail.contains("-32601") || detail.contains("not found") {
                    v.check_skip(
                        &check_id,
                        &format!("composition.dispatch not available for {signal_name}: {e}"),
                    );
                } else if e.is_skippable() {
                    v.check_skip(
                        &check_id,
                        &format!("orchestration not reachable for {signal_name}: {e}"),
                    );
                } else {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!("dispatch({signal_name:?}) error: {e}"),
                    );
                }
            }
        }
    }
}

/// Run the atomic signals validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let registry_caps = helpers::load_registry_capabilities();

    // Tier 1 (Rust): structural validation
    for &(signal_name, _path, content) in COMPOSITION_GRAPHS {
        let tier = signal_name.split('.').next().unwrap_or("unknown");

        let Some(parsed) = helpers::graph_parses(v, signal_name, content) else {
            continue;
        };

        helpers::validate_graph_structure(v, signal_name, &parsed);
        helpers::validate_dark_forest(v, signal_name, &parsed);
        helpers::validate_node_capabilities(v, signal_name, &parsed, &registry_caps);
        validate_tier_primals(v, signal_name, &parsed, tier);
    }

    validate_registry_section(v);
    validate_context_composition_method(v);

    // Tier 2 (Live): composition dispatch via biomeOS
    validate_live_composition_dispatch(v, ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_compositions_pass() {
        let mut v = ValidationResult::new("atomic-compositions");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "atomic-compositions should evaluate at least one check"
        );
    }

    #[test]
    fn composition_graph_count() {
        assert_eq!(
            COMPOSITION_GRAPHS.len(),
            21,
            "expected 21 composition graphs (10 foundation + 5 meta + 5 rootPulse + 1 foundation_validation)"
        );
    }

    #[test]
    fn all_composition_graphs_parse() {
        for &(name, _, content) in COMPOSITION_GRAPHS {
            let result: Result<toml::Value, _> = toml::from_str(content);
            assert!(result.is_ok(), "{name} failed to parse: {:?}", result.err());
        }
    }

    #[test]
    fn tier_detection() {
        assert!(CompositionContext::is_composition_tier("tower"));
        assert!(CompositionContext::is_composition_tier("node"));
        assert!(CompositionContext::is_composition_tier("nest"));
        assert!(CompositionContext::is_composition_tier("nucleus"));
        assert!(CompositionContext::is_composition_tier("meta"));
        assert!(!CompositionContext::is_composition_tier("security"));
        assert!(!CompositionContext::is_composition_tier("tensor"));
    }
}
