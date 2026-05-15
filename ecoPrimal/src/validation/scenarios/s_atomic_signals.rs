// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Atomic Signals — structural validation of the composition
//! collapse layer (Tower/Node/Nest/Meta signal graphs).
//!
//! Validates that all signal graphs parse as valid TOML, reference
//! capabilities registered in the canonical registry, use coordination
//! patterns consistent with their declared intent, and respect Dark Forest
//! security invariants (secure_by_default, btsp_enforced).
//!
//! All checks are Tier::Rust — no live primals required.

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
    ("meta.observe", "graphs/signals/meta_observe.toml", include_str!("../../../../graphs/signals/meta_observe.toml")),
    ("meta.intent", "graphs/signals/meta_intent.toml", include_str!("../../../../graphs/signals/meta_intent.toml")),
    ("meta.render", "graphs/signals/meta_render.toml", include_str!("../../../../graphs/signals/meta_render.toml")),
    ("meta.health", "graphs/signals/meta_health.toml", include_str!("../../../../graphs/signals/meta_health.toml")),
    ("meta.deploy", "graphs/signals/meta_deploy.toml", include_str!("../../../../graphs/signals/meta_deploy.toml")),
];

const TOWER_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat"];
const NODE_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat", "toadstool", "barracuda", "coralreef"];
const NEST_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat", "nestgate", "rhizocrypt", "loamspine", "sweetgrass"];
const META_PRIMALS: &[&str] = &["biomeos", "squirrel", "petaltongue", "skunkbat"];

fn tier_primals(tier: &str) -> &'static [&'static str] {
    match tier {
        "tower" => TOWER_PRIMALS,
        "node" => NODE_PRIMALS,
        "nest" => NEST_PRIMALS,
        "meta" => META_PRIMALS,
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
            .and_then(|r| r.as_bool())
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

/// Run the atomic signals validation scenario.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let registry_caps = helpers::load_registry_capabilities();

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
        assert_eq!(SIGNAL_GRAPHS.len(), 14, "expected 14 signal graphs (9 foundation + 5 meta)");
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
