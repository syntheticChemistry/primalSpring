// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Atomic Signals — structural validation of the composition
//! collapse layer (Tower/Node/Nest/Meta signal graphs).
//!
//! Validates that all signal graphs parse as valid TOML, reference
//! capabilities registered in the canonical registry, use coordination
//! patterns consistent with their declared intent, and respect Dark Forest
//! security invariants (secure_by_default, btsp_enforced).
//!
//! Tier 1 checks are Tier::Rust — no live primals required.
//! Tier 2 checks validate live signal dispatch when biomeOS is available.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::helpers;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "atomic-signals",
        track: Track::AtomicComposition,
        tier: Tier::Rust,
        provenance_crate: "primalspring_atomic_signals",
        provenance_date: "2026-05-15",
        description: "Atomic signal graphs — parse, capabilities, coordination, Dark Forest",
    },
    run,
};

const SIGNAL_GRAPHS: &[(&str, &str, &str)] = &[
    ("tower.publish", "graphs/signals/tower_publish.toml", include_str!("../../../../graphs/signals/tower_publish.toml")),
    ("tower.authenticate", "graphs/signals/tower_authenticate.toml", include_str!("../../../../graphs/signals/tower_authenticate.toml")),
    ("tower.discover", "graphs/signals/tower_discover.toml", include_str!("../../../../graphs/signals/tower_discover.toml")),
    ("tower.health", "graphs/signals/tower_health.toml", include_str!("../../../../graphs/signals/tower_health.toml")),
    ("tower.bootstrap", "graphs/signals/tower_bootstrap.toml", include_str!("../../../../graphs/signals/tower_bootstrap.toml")),
    ("node.compute", "graphs/signals/node_compute.toml", include_str!("../../../../graphs/signals/node_compute.toml")),
    ("nest.store", "graphs/signals/nest_store.toml", include_str!("../../../../graphs/signals/nest_store.toml")),
    ("nest.commit", "graphs/signals/nest_commit.toml", include_str!("../../../../graphs/signals/nest_commit.toml")),
    ("nest.retrieve", "graphs/signals/nest_retrieve.toml", include_str!("../../../../graphs/signals/nest_retrieve.toml")),
    ("nest.ingest_spore", "graphs/signals/nest_ingest_spore.toml", include_str!("../../../../graphs/signals/nest_ingest_spore.toml")),
    ("meta.observe", "graphs/signals/meta_observe.toml", include_str!("../../../../graphs/signals/meta_observe.toml")),
    ("meta.intent", "graphs/signals/meta_intent.toml", include_str!("../../../../graphs/signals/meta_intent.toml")),
    ("meta.render", "graphs/signals/meta_render.toml", include_str!("../../../../graphs/signals/meta_render.toml")),
    ("meta.health", "graphs/signals/meta_health.toml", include_str!("../../../../graphs/signals/meta_health.toml")),
    ("meta.deploy", "graphs/signals/meta_deploy.toml", include_str!("../../../../graphs/signals/meta_deploy.toml")),
    // rootPulse domain (ACTION / efferent) — Wave 60
    ("rootpulse.commit", "graphs/signals/rootpulse_commit.toml", include_str!("../../../../graphs/signals/rootpulse_commit.toml")),
    ("rootpulse.branch", "graphs/signals/rootpulse_branch.toml", include_str!("../../../../graphs/signals/rootpulse_branch.toml")),
    ("rootpulse.merge", "graphs/signals/rootpulse_merge.toml", include_str!("../../../../graphs/signals/rootpulse_merge.toml")),
    ("rootpulse.diff", "graphs/signals/rootpulse_diff.toml", include_str!("../../../../graphs/signals/rootpulse_diff.toml")),
    ("rootpulse.federate", "graphs/signals/rootpulse_federate.toml", include_str!("../../../../graphs/signals/rootpulse_federate.toml")),
    // ecosystem domain (SYNC / autonomic) — Wave 60
    ("ecosystem.pull", "graphs/signals/ecosystem_pull.toml", include_str!("../../../../graphs/signals/ecosystem_pull.toml")),
    ("ecosystem.push", "graphs/signals/ecosystem_push.toml", include_str!("../../../../graphs/signals/ecosystem_push.toml")),
    ("ecosystem.check", "graphs/signals/ecosystem_check.toml", include_str!("../../../../graphs/signals/ecosystem_check.toml")),
    // impulsePotential tier (inter-gate coordination)
    ("impulse.post", "graphs/signals/impulse_post.toml", include_str!("../../../../graphs/signals/impulse_post.toml")),
    ("impulse.ack", "graphs/signals/impulse_ack.toml", include_str!("../../../../graphs/signals/impulse_ack.toml")),
    ("potential.sense", "graphs/signals/potential_sense.toml", include_str!("../../../../graphs/signals/potential_sense.toml")),
    ("potential.check", "graphs/signals/potential_check.toml", include_str!("../../../../graphs/signals/potential_check.toml")),
    ("sync.diverge", "graphs/signals/sync_diverge.toml", include_str!("../../../../graphs/signals/sync_diverge.toml")),
    ("sync.resolve", "graphs/signals/sync_resolve.toml", include_str!("../../../../graphs/signals/sync_resolve.toml")),
    ("sync.resolve.crossgate", "graphs/signals/sync_resolve_crossgate.toml", include_str!("../../../../graphs/signals/sync_resolve_crossgate.toml")),
];

const TOWER_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat"];
const NODE_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat", "toadstool", "barracuda", "coralreef"];
const NEST_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat", "nestgate", "rhizocrypt", "loamspine", "sweetgrass"];
const META_PRIMALS: &[&str] = &["biomeos", "squirrel", "petaltongue", "skunkbat"];
const ROOTPULSE_PRIMALS: &[&str] = &["rhizocrypt", "beardog", "nestgate", "loamspine", "sweetgrass", "songbird"];
const ECOSYSTEM_PRIMALS: &[&str] = &["biomeos", "songbird", "nestgate", "beardog"];
const IMPULSE_PRIMALS: &[&str] = &["membrane", "beardog", "nestgate", "songbird"];
const POTENTIAL_PRIMALS: &[&str] = &["membrane", "beardog", "nestgate", "songbird"];
const SYNC_PRIMALS: &[&str] = &[
    "biomeos", "nestgate", "membrane", "songbird",
    "rhizocrypt", "loamspine", "sweetgrass", "beardog",
];

fn tier_primals(tier: &str) -> &'static [&'static str] {
    match tier {
        "tower" => TOWER_PRIMALS,
        "node" => NODE_PRIMALS,
        "nest" => NEST_PRIMALS,
        "meta" => META_PRIMALS,
        "rootpulse" => ROOTPULSE_PRIMALS,
        "ecosystem" => ECOSYSTEM_PRIMALS,
        "impulse" => IMPULSE_PRIMALS,
        "potential" => POTENTIAL_PRIMALS,
        "sync" => SYNC_PRIMALS,
        _ => &[],
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

    let signals = parsed.get("signals");
    v.check_bool(
        "registry:has_signals_section",
        signals.is_some(),
        "capability_registry.toml has [signals] section",
    );

    let Some(signals) = signals.and_then(|s| s.as_table()) else {
        return;
    };

    let expected_tiers = ["tower", "node", "nest", "meta"];
    for tier in &expected_tiers {
        let has_tier = signals.contains_key(*tier);
        v.check_bool(
            &format!("registry:signals.{tier}"),
            has_tier,
            &format!("signals section has {tier} tier definition"),
        );
    }

    let total_signals: usize = signals
        .values()
        .filter_map(|v| v.get("signals"))
        .filter_map(|s| s.as_array())
        .map(Vec::len)
        .sum();

    v.check_bool(
        "registry:signal_count",
        total_signals == SIGNAL_GRAPHS.len(),
        &format!(
            "registry declares {total_signals} signals, graph files provide {}",
            SIGNAL_GRAPHS.len()
        ),
    );
}

fn validate_context_signal_method(v: &mut ValidationResult) {
    let is_signal = CompositionContext::is_signal_tier("tower");
    v.check_bool(
        "context:tower_is_signal_tier",
        is_signal,
        "CompositionContext::is_signal_tier recognizes tower",
    );

    let is_signal_node = CompositionContext::is_signal_tier("node");
    v.check_bool(
        "context:node_is_signal_tier",
        is_signal_node,
        "CompositionContext::is_signal_tier recognizes node",
    );

    let is_signal_nest = CompositionContext::is_signal_tier("nest");
    v.check_bool(
        "context:nest_is_signal_tier",
        is_signal_nest,
        "CompositionContext::is_signal_tier recognizes nest",
    );

    let is_signal_nucleus = CompositionContext::is_signal_tier("nucleus");
    v.check_bool(
        "context:nucleus_is_signal_tier",
        is_signal_nucleus,
        "CompositionContext::is_signal_tier recognizes nucleus",
    );

    let is_signal_meta = CompositionContext::is_signal_tier("meta");
    v.check_bool(
        "context:meta_is_signal_tier",
        is_signal_meta,
        "CompositionContext::is_signal_tier recognizes meta",
    );

    let not_signal = !CompositionContext::is_signal_tier("security");
    v.check_bool(
        "context:security_not_signal_tier",
        not_signal,
        "CompositionContext::is_signal_tier rejects security (direct capability)",
    );
}

/// Tier 2 (Live) validation: if a biomeOS/orchestration capability is
/// reachable, verify that `signal.list` and `signal.schema` respond correctly.
fn validate_live_signal_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let biomeos_available = ctx.has_capability("orchestration");

    v.check_bool(
        "live:biomeos_available",
        true,
        &format!(
            "biomeOS availability check ({})",
            if biomeos_available { "live" } else { "offline — skipping Tier 2" }
        ),
    );

    if !biomeos_available {
        return;
    }

    // signal.list should return signals matching our structural count
    let expected_count = SIGNAL_GRAPHS.len() as u64;
    match ctx.call("orchestration", "signal.list", serde_json::json!({})) {
        Ok(response) => {
            let count = response
                .get("count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            v.check_bool(
                "live:signal.list:responds",
                true,
                "biomeOS signal.list responds",
            );
            v.check_bool(
                "live:signal.list:count",
                count == expected_count,
                &format!(
                    "signal.list reports {count} signals (expected {expected_count} per structural validation)"
                ),
            );
        }
        Err(e) => {
            v.check_bool(
                "live:signal.list:responds",
                false,
                &format!("biomeOS signal.list failed: {e}"),
            );
        }
    }

    // signal.schema should load the tools definition
    match ctx.call("orchestration", "signal.schema", serde_json::json!({})) {
        Ok(response) => {
            let has_tools = response.get("tools").is_some();
            v.check_bool(
                "live:signal.schema:has_tools",
                has_tools,
                "signal.schema returns tools definition",
            );
        }
        Err(e) => {
            v.check_bool(
                "live:signal.schema:responds",
                false,
                &format!("biomeOS signal.schema failed: {e}"),
            );
        }
    }

    // Tier 2b: dispatch() API — validate each signal through the unified API
    for &(signal_name, _, _) in SIGNAL_GRAPHS {
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
                let detail = format!("{e}");
                if detail.contains("-32601") || detail.contains("not found") {
                    v.check_skip(
                        &check_id,
                        &format!("signal.dispatch not available for {signal_name}: {e}"),
                    );
                } else if e.is_connection_error() {
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
    for &(signal_name, _path, content) in SIGNAL_GRAPHS {
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
    validate_context_signal_method(v);

    // Tier 2 (Live): signal dispatch via biomeOS
    validate_live_signal_dispatch(v, ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_signals_pass() {
        let mut v = ValidationResult::new("atomic-signals");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "atomic signals scenario had {} failures (use --nocapture for details)",
            v.failed
        );
    }

    #[test]
    fn signal_graph_count() {
        assert_eq!(SIGNAL_GRAPHS.len(), 30, "expected 30 signal graphs (10 foundation + 5 meta + 5 rootPulse + 3 ecosystem + 4 impulse_potential + 3 agentic_sync)");
    }

    #[test]
    fn all_signal_graphs_parse() {
        for &(name, _, content) in SIGNAL_GRAPHS {
            let result: Result<toml::Value, _> = toml::from_str(content);
            assert!(result.is_ok(), "{name} failed to parse: {:?}", result.err());
        }
    }

    #[test]
    fn tier_detection() {
        assert!(CompositionContext::is_signal_tier("tower"));
        assert!(CompositionContext::is_signal_tier("node"));
        assert!(CompositionContext::is_signal_tier("nest"));
        assert!(CompositionContext::is_signal_tier("nucleus"));
        assert!(CompositionContext::is_signal_tier("meta"));
        assert!(!CompositionContext::is_signal_tier("security"));
        assert!(!CompositionContext::is_signal_tier("tensor"));
    }
}
