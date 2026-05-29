// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Meta-Tier Signals — agentic composition collapse validation.
//!
//! Validates the meta-tier signal layer that enables squirrel's agentic
//! dispatch: signal tool schema coverage, meta-tier graph structure,
//! capability routing for AI/visualization/orchestration domains, and
//! the intent->plan->dispatch->render cycle.
//!
//! All checks are Tier::Rust — no live primals required.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "meta-tier-signals",
        track: Track::BiomeosDeploy,
        tier: Tier::Rust,
        provenance_crate: "primalspring_meta_tier_signals",
        provenance_date: "2026-05-15",
        description:
            "Meta-tier agentic signals — tool schema, routing, intent loop, composition collapse",
    },
    run,
};

const SIGNAL_TOOLS_TOML: &str = include_str!("../../../../config/signal_tools.toml");
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const META_FRAGMENT: &str = include_str!("../../../../graphs/fragments/meta_tier.toml");

const EXPECTED_TOOL_NAMES: &[&str] = &[
    "tower.publish",
    "tower.authenticate",
    "tower.discover",
    "tower.health",
    "tower.bootstrap",
    "node.compute",
    "nest.store",
    "nest.commit",
    "nest.retrieve",
    "meta.observe",
    "meta.intent",
    "meta.render",
    "meta.health",
    "meta.deploy",
    "rootpulse.commit",
    "rootpulse.branch",
    "rootpulse.merge",
    "rootpulse.diff",
    "rootpulse.federate",
    "ecosystem.pull",
    "ecosystem.push",
    "ecosystem.check",
];

/// Core meta-tier primals (the fragment itself).
const META_FRAGMENT_PRIMALS: &[&str] = &["biomeos", "squirrel", "petaltongue"];

/// Extended meta-tier primals (includes Tower primals borrowed by meta signals like meta.deploy).
const META_SIGNAL_PRIMALS: &[&str] = &["biomeos", "squirrel", "petaltongue", "skunkbat"];

fn validate_signal_tools_parse(v: &mut ValidationResult) -> Option<toml::Value> {
    match toml::from_str::<toml::Value>(SIGNAL_TOOLS_TOML) {
        Ok(parsed) => {
            v.check_bool(
                "tools:parse",
                true,
                "signal_tools.toml parses as valid TOML",
            );
            Some(parsed)
        }
        Err(e) => {
            v.check_bool(
                "tools:parse",
                false,
                &format!("signal_tools.toml parse error: {e}"),
            );
            None
        }
    }
}

fn validate_tool_coverage(v: &mut ValidationResult, parsed: &toml::Value) {
    let tools = parsed.get("tools").and_then(|t| t.as_array());
    let tool_count = tools.map_or(0, Vec::len);

    v.check_bool(
        "tools:has_entries",
        tool_count > 0,
        &format!("signal_tools.toml has {tool_count} tool entries"),
    );

    let tool_names: Vec<String> = tools
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.get("name").and_then(|n| n.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();

    for &expected in EXPECTED_TOOL_NAMES {
        let present = tool_names.iter().any(|n| n == expected);
        v.check_bool(
            &format!("tools:covers:{expected}"),
            present,
            &format!("signal_tools.toml includes tool definition for {expected}"),
        );
    }

    v.check_bool(
        "tools:complete_coverage",
        tool_count == EXPECTED_TOOL_NAMES.len(),
        &format!(
            "tool schema has {tool_count} tools, expected {}",
            EXPECTED_TOOL_NAMES.len()
        ),
    );
}

fn validate_tool_schema_fields(v: &mut ValidationResult, parsed: &toml::Value) {
    let Some(tools) = parsed.get("tools").and_then(|t| t.as_array()) else { return };

    for tool in tools {
        let name = tool
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");

        let has_tier = tool.get("tier").and_then(|t| t.as_str()).is_some();
        v.check_bool(
            &format!("tools:{name}:has_tier"),
            has_tier,
            &format!("{name} has tier field"),
        );

        let has_description = tool
            .get("description")
            .and_then(|d| d.as_str())
            .is_some();
        v.check_bool(
            &format!("tools:{name}:has_description"),
            has_description,
            &format!("{name} has description for LLM reasoning"),
        );

        let has_graph = tool.get("graph").and_then(|g| g.as_str()).is_some();
        v.check_bool(
            &format!("tools:{name}:has_graph"),
            has_graph,
            &format!("{name} references a graph file"),
        );

        let has_coordination = tool
            .get("coordination")
            .and_then(|c| c.as_str())
            .is_some();
        v.check_bool(
            &format!("tools:{name}:has_coordination"),
            has_coordination,
            &format!("{name} declares coordination pattern"),
        );

        let has_primals = tool
            .get("primals")
            .and_then(|p| p.as_array())
            .is_some_and(|a| !a.is_empty());
        v.check_bool(
            &format!("tools:{name}:has_primals"),
            has_primals,
            &format!("{name} lists participating primals"),
        );
    }
}

fn validate_meta_fragment_alignment(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(META_FRAGMENT) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "fragment:parse",
                false,
                &format!("meta_tier.toml parse error: {e}"),
            );
            return;
        }
    };

    let nodes = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array());

    let Some(nodes) = nodes else {
        v.check_bool(
            "fragment:has_nodes",
            false,
            "meta_tier.toml has no fragment.nodes",
        );
        return;
    };

    let binaries: Vec<&str> = nodes
        .iter()
        .filter_map(|n| n.get("binary").and_then(|b| b.as_str()))
        .collect();

    for &primal in META_FRAGMENT_PRIMALS {
        let present = binaries.contains(&primal);
        v.check_bool(
            &format!("fragment:{primal}:present"),
            present,
            &format!("{primal} is present in meta_tier.toml fragment"),
        );
    }

    v.check_bool(
        "fragment:node_count",
        nodes.len() == META_FRAGMENT_PRIMALS.len(),
        &format!(
            "meta_tier.toml has {} nodes, expected {}",
            nodes.len(),
            META_FRAGMENT_PRIMALS.len()
        ),
    );
}

fn validate_registry_meta_signals(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(REGISTRY_TOML) {
        Ok(p) => p,
        Err(_) => return,
    };

    let meta_signals = parsed
        .get("signals")
        .and_then(|s| s.get("meta"));

    v.check_bool(
        "registry:signals.meta",
        meta_signals.is_some(),
        "capability_registry.toml has [signals.meta] section",
    );

    let Some(meta) = meta_signals else { return };

    let tier = meta.get("tier").and_then(|t| t.as_str());
    v.check_bool(
        "registry:meta_tier_value",
        tier == Some("meta"),
        "signals.meta tier = meta",
    );

    let primals = meta
        .get("primals")
        .and_then(|p| p.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    for &primal in META_SIGNAL_PRIMALS {
        let present = primals.contains(&primal);
        v.check_bool(
            &format!("registry:meta_primals:{primal}"),
            present,
            &format!("{primal} listed in signals.meta.primals"),
        );
    }

    let signal_count = meta
        .get("signals")
        .and_then(|s| s.as_array())
        .map_or(0, Vec::len);
    v.check_bool(
        "registry:meta_signal_count",
        signal_count == 5,
        &format!("signals.meta declares {signal_count} signals, expected 5"),
    );
}

fn validate_capability_routing(v: &mut ValidationResult) {
    use crate::composition::capability_to_primal;

    let ai_owner = capability_to_primal("ai");
    v.check_bool(
        "routing:ai_to_squirrel",
        ai_owner == "squirrel",
        &format!("ai capability routes to {ai_owner}, expected squirrel"),
    );

    let viz_owner = capability_to_primal("visualization");
    v.check_bool(
        "routing:viz_to_petaltongue",
        viz_owner == "petaltongue",
        &format!("visualization capability routes to {viz_owner}, expected petaltongue"),
    );

    let orch_owner = capability_to_primal("orchestration");
    v.check_bool(
        "routing:orchestration_to_biomeos",
        orch_owner == "biomeos",
        &format!("orchestration capability routes to {orch_owner}, expected biomeos"),
    );
}

fn validate_intent_loop_structure(v: &mut ValidationResult) {
    let intent_graph: toml::Value = match toml::from_str(
        include_str!("../../../../graphs/signals/meta_intent.toml"),
    ) {
        Ok(p) => p,
        Err(_) => return,
    };

    let nodes = intent_graph
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .unwrap_or(&Vec::new())
        .clone();

    let node_binaries: Vec<&str> = nodes
        .iter()
        .filter_map(|n| n.get("binary").and_then(|b| b.as_str()))
        .collect();

    v.check_bool(
        "intent:starts_with_petaltongue",
        node_binaries.first() == Some(&"petaltongue"),
        "meta.intent graph starts with petalTongue (user interaction capture)",
    );

    v.check_bool(
        "intent:squirrel_plans",
        node_binaries.get(1) == Some(&"squirrel"),
        "meta.intent graph has squirrel as second node (AI planning)",
    );

    v.check_bool(
        "intent:biomeos_dispatches",
        node_binaries.last() == Some(&"biomeos"),
        "meta.intent graph ends with biomeOS (orchestration dispatch)",
    );

    let render_graph: toml::Value = match toml::from_str(
        include_str!("../../../../graphs/signals/meta_render.toml"),
    ) {
        Ok(p) => p,
        Err(_) => return,
    };

    let render_nodes = render_graph
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .unwrap_or(&Vec::new())
        .clone();

    let render_binaries: Vec<&str> = render_nodes
        .iter()
        .filter_map(|n| n.get("binary").and_then(|b| b.as_str()))
        .collect();

    v.check_bool(
        "render:starts_with_biomeos",
        render_binaries.first() == Some(&"biomeos"),
        "meta.render graph starts with biomeOS (result collection)",
    );

    v.check_bool(
        "render:ends_with_petaltongue",
        render_binaries.last() == Some(&"petaltongue"),
        "meta.render graph ends with petalTongue (user-facing render)",
    );

    v.check_bool(
        "cycle:intent_render_complement",
        node_binaries.first() == render_binaries.last()
            && node_binaries.last() == render_binaries.first(),
        "meta.intent and meta.render form complementary cycle (PT→SQ→BOS / BOS→SQ→PT)",
    );
}

/// Run the meta-tier signals validation scenario.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Signal Tool Schema");
    if let Some(parsed) = validate_signal_tools_parse(v) {
        validate_tool_coverage(v, &parsed);
        validate_tool_schema_fields(v, &parsed);
    }

    v.section("Meta Fragment Alignment");
    validate_meta_fragment_alignment(v);

    v.section("Registry Meta Signals");
    validate_registry_meta_signals(v);

    v.section("Capability Routing");
    validate_capability_routing(v);

    v.section("Intent Loop Structure");
    validate_intent_loop_structure(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_tier_signals_pass() {
        let mut v = ValidationResult::new("meta-tier-signals");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "meta-tier signals scenario had {} failures (use --nocapture for details)",
            v.failed
        );
    }

    #[test]
    fn signal_tools_parse() {
        let result: Result<toml::Value, _> = toml::from_str(SIGNAL_TOOLS_TOML);
        assert!(
            result.is_ok(),
            "signal_tools.toml failed to parse: {:?}",
            result.err()
        );
    }

    #[test]
    fn all_tiers_covered_in_tool_schema() {
        let parsed: toml::Value = toml::from_str(SIGNAL_TOOLS_TOML).unwrap();
        let tools = parsed.get("tools").and_then(|t| t.as_array()).unwrap();
        let tiers: std::collections::HashSet<&str> = tools
            .iter()
            .filter_map(|t| t.get("tier").and_then(|t| t.as_str()))
            .collect();
        assert!(tiers.contains("tower"), "missing tower tier in tool schema");
        assert!(tiers.contains("node"), "missing node tier in tool schema");
        assert!(tiers.contains("nest"), "missing nest tier in tool schema");
        assert!(tiers.contains("meta"), "missing meta tier in tool schema");
    }

    #[test]
    fn meta_is_signal_tier() {
        assert!(CompositionContext::is_signal_tier("meta"));
    }
}
