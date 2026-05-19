// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Agentic Tower — structural validation of the minimal
//! autonomous deployment unit (Tower + biomeOS + squirrel).
//!
//! Validates that `tower_agent.toml` is a complete, Dark-Forest-compliant
//! composition with all three Tower primals, biomeOS orchestration, and
//! squirrel AI with the signal tool schema. Also validates that the
//! `meta.deploy` signal graph is wired correctly and that all AI overlay
//! graphs include skunkBat for audit coverage.
//!
//! All checks are Tier::Rust — no live primals required.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::helpers;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "agentic-tower",
        track: Track::BiomeosDeploy,
        tier: Tier::Rust,
        provenance_crate: "primalspring_agentic_tower",
        provenance_date: "2026-05-15",
        description:
            "Agentic tower — Tower+squirrel deployment unit, meta.deploy signal, AI overlay audit",
    },
    run,
};

const TOWER_AGENT_TOML: &str = include_str!("../../../../graphs/tower_agent.toml");
const TOWER_AI_TOML: &str = include_str!("../../../../graphs/tower_ai.toml");
const TOWER_AI_VIZ_TOML: &str = include_str!("../../../../graphs/tower_ai_viz.toml");
const NODE_AI_TOML: &str = include_str!("../../../../graphs/node_ai.toml");
const META_DEPLOY_TOML: &str = include_str!("../../../../graphs/signals/meta_deploy.toml");
const TOWER_BOOTSTRAP_TOML: &str = include_str!("../../../../graphs/signals/tower_bootstrap.toml");
const SIGNAL_TOOLS_TOML: &str = include_str!("../../../../config/signal_tools.toml");

const TOWER_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat"];
const AGENT_REQUIRED: &[&str] = &["beardog", "songbird", "skunkbat", "biomeos", "squirrel"];

fn validate_tower_agent_structure(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(TOWER_AGENT_TOML) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "agent:parse",
                false,
                &format!("tower_agent.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool("agent:parse", true, "tower_agent.toml parses as valid TOML");

    let metadata = parsed.get("graph").and_then(|g| g.get("metadata"));
    let is_agentic = metadata
        .and_then(|m| m.get("agentic"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        "agent:agentic_flag",
        is_agentic,
        "tower_agent.toml has agentic = true metadata",
    );

    let uds_only = metadata
        .and_then(|m| m.get("transport"))
        .and_then(|t| t.as_str()) == Some("uds_only");
    v.check_bool(
        "agent:uds_only",
        uds_only,
        "tower_agent.toml transport = uds_only (Dark Forest)",
    );

    let btsp = metadata
        .and_then(|m| m.get("security_model"))
        .and_then(|s| s.as_str()) == Some("btsp_enforced");
    v.check_bool(
        "agent:btsp_enforced",
        btsp,
        "tower_agent.toml security_model = btsp_enforced",
    );

    let binaries = helpers::graph_binaries(TOWER_AGENT_TOML);
    for &required in AGENT_REQUIRED {
        let present = binaries.iter().any(|b| b == required);
        v.check_bool(
            &format!("agent:{required}:present"),
            present,
            &format!("{required} is present in tower_agent.toml"),
        );
    }

    let nodes = parsed
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array());
    if let Some(nodes) = nodes {
        for node in nodes {
            let name = node
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");
            let required = node
                .get("required")
                .and_then(toml::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                &format!("agent:{name}:required"),
                required,
                &format!("{name} is required = true in agentic tower"),
            );
        }
    }
}

fn validate_ai_overlays_have_skunkbat(v: &mut ValidationResult) {
    let overlays: &[(&str, &str)] = &[
        ("tower_ai", TOWER_AI_TOML),
        ("tower_ai_viz", TOWER_AI_VIZ_TOML),
        ("node_ai", NODE_AI_TOML),
    ];

    for &(name, content) in overlays {
        let binaries = helpers::graph_binaries(content);
        let has_skunkbat = binaries.iter().any(|b| b == "skunkbat");
        v.check_bool(
            &format!("overlay:{name}:skunkbat"),
            has_skunkbat,
            &format!("{name}.toml includes skunkBat for audit coverage"),
        );

        for &tower_primal in TOWER_PRIMALS {
            let present = binaries.iter().any(|b| b == tower_primal);
            v.check_bool(
                &format!("overlay:{name}:{tower_primal}"),
                present,
                &format!("{name}.toml includes {tower_primal} (canonical Tower)"),
            );
        }
    }
}

fn validate_meta_deploy_signal(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(META_DEPLOY_TOML) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "deploy:parse",
                false,
                &format!("meta_deploy.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool("deploy:parse", true, "meta_deploy.toml parses");

    let binaries = helpers::graph_binaries(META_DEPLOY_TOML);

    let has_squirrel = binaries.iter().any(|b| b == "squirrel");
    v.check_bool(
        "deploy:squirrel_plans",
        has_squirrel,
        "meta.deploy has squirrel for AI planning",
    );

    let has_biomeos = binaries.iter().any(|b| b == "biomeos");
    v.check_bool(
        "deploy:biomeos_executes",
        has_biomeos,
        "meta.deploy has biomeOS for graph execution",
    );

    let has_skunkbat = binaries.iter().any(|b| b == "skunkbat");
    v.check_bool(
        "deploy:skunkbat_audits",
        has_skunkbat,
        "meta.deploy has skunkBat for deployment audit",
    );

    let graph = parsed.get("graph");
    let is_agentic = graph
        .and_then(|g| g.get("metadata"))
        .and_then(|m| m.get("agentic"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        "deploy:agentic_flag",
        is_agentic,
        "meta_deploy.toml has agentic = true",
    );
}

fn validate_signal_tools_deploy(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(SIGNAL_TOOLS_TOML) {
        Ok(p) => p,
        Err(_) => return,
    };

    let tools = parsed.get("tools").and_then(|t| t.as_array());
    let has_deploy = tools
        .is_some_and(|arr| {
            arr.iter().any(|t| {
                t.get("name")
                    .and_then(|n| n.as_str()) == Some("meta.deploy")
            })
        });

    v.check_bool(
        "tools:meta.deploy",
        has_deploy,
        "signal_tools.toml includes meta.deploy tool definition",
    );

    if let Some(tools) = tools {
        if let Some(deploy_tool) = tools
            .iter()
            .find(|t| t.get("name").and_then(|n| n.as_str()) == Some("meta.deploy"))
        {
            let has_target = deploy_tool
                .get("parameters")
                .and_then(|p| p.get("target"))
                .is_some();
            v.check_bool(
                "tools:deploy:has_target_param",
                has_target,
                "meta.deploy tool has 'target' parameter for deployment target",
            );

            let has_approval = deploy_tool
                .get("parameters")
                .and_then(|p| p.get("approval"))
                .is_some();
            v.check_bool(
                "tools:deploy:has_approval_param",
                has_approval,
                "meta.deploy tool has 'approval' parameter for policy gating",
            );
        }
    }
}

fn validate_squirrel_signal_context(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(TOWER_AGENT_TOML) {
        Ok(p) => p,
        Err(_) => return,
    };

    let nodes = parsed
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array());

    let squirrel_caps: Vec<&str> = nodes
        .map(|arr| {
            arr.iter()
                .filter(|n| {
                    n.get("binary").and_then(|b| b.as_str()) == Some("squirrel")
                })
                .flat_map(|n| {
                    n.get("capabilities")
                        .and_then(|c| c.as_array())
                        .into_iter()
                        .flat_map(|a| a.iter().filter_map(|v| v.as_str()))
                })
                .collect()
        })
        .unwrap_or_default();

    let has_context_push = squirrel_caps.contains(&"squirrel.context.push");
    v.check_bool(
        "squirrel:context_push",
        has_context_push,
        "squirrel node in tower_agent has squirrel.context.push (signal tool schema ingestion)",
    );

    let has_mcp = squirrel_caps.contains(&"mcp.tools.list");
    v.check_bool(
        "squirrel:mcp_tools",
        has_mcp,
        "squirrel node in tower_agent has mcp.tools.list (tool discovery)",
    );

    let has_providers = squirrel_caps.contains(&"squirrel.providers");
    v.check_bool(
        "squirrel:providers",
        has_providers,
        "squirrel node in tower_agent has squirrel.providers (AI backend introspection)",
    );

    let has_list_providers = squirrel_caps.contains(&"ai.list_providers");
    v.check_bool(
        "squirrel:list_providers",
        has_list_providers,
        "squirrel node in tower_agent has ai.list_providers (backend enumeration)",
    );
}

fn validate_tower_bootstrap(v: &mut ValidationResult) {
    let parsed: toml::Value = match toml::from_str(TOWER_BOOTSTRAP_TOML) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "bootstrap:parse",
                false,
                &format!("tower_bootstrap.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "bootstrap:parse",
        true,
        "tower_bootstrap.toml parses as valid TOML",
    );

    let graph = parsed.get("graph");

    let is_bootstrap = graph
        .and_then(|g| g.get("metadata"))
        .and_then(|m| m.get("bootstrap"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        "bootstrap:metadata_flag",
        is_bootstrap,
        "tower_bootstrap.toml has bootstrap = true metadata",
    );

    let signal_name = graph
        .and_then(|g| g.get("signal_name"))
        .and_then(|s| s.as_str())
        .unwrap_or("");
    v.check_bool(
        "bootstrap:signal_name",
        signal_name == "bootstrap",
        "tower_bootstrap.toml signal_name = bootstrap",
    );

    let nodes = graph
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array());

    let Some(nodes) = nodes else {
        v.check_bool("bootstrap:has_nodes", false, "tower_bootstrap.toml has no nodes");
        return;
    };

    v.check_bool(
        "bootstrap:node_count",
        nodes.len() == 6,
        &format!("tower_bootstrap.toml has {} nodes, expected 6", nodes.len()),
    );

    // Phase 1 nodes: beardog, songbird, skunkbat (spawned, required)
    let phase1_nodes: Vec<&toml::Value> = nodes
        .iter()
        .filter(|n| {
            n.get("phase")
                .and_then(toml::Value::as_integer) == Some(1)
        })
        .collect();

    v.check_bool(
        "bootstrap:phase1_count",
        phase1_nodes.len() == 3,
        &format!(
            "Phase 1 has {} nodes, expected 3 (beardog, songbird, skunkbat)",
            phase1_nodes.len()
        ),
    );

    let phase1_binaries: Vec<&str> = phase1_nodes
        .iter()
        .filter_map(|n| n.get("binary").and_then(|b| b.as_str()))
        .collect();

    for &primal in TOWER_PRIMALS {
        let in_phase1 = phase1_binaries.contains(&primal);
        v.check_bool(
            &format!("bootstrap:phase1:{primal}"),
            in_phase1,
            &format!("{primal} is a Phase 1 bootstrap node"),
        );
    }

    // All Phase 1 nodes must have spawn = true and required = true
    for node in &phase1_nodes {
        let name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");
        let spawn = node
            .get("spawn")
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        v.check_bool(
            &format!("bootstrap:phase1:{name}:spawn"),
            spawn,
            &format!("{name} has spawn = true (Phase 1 creates processes)"),
        );
        let required = node
            .get("required")
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        v.check_bool(
            &format!("bootstrap:phase1:{name}:required"),
            required,
            &format!("{name} is required = true in Phase 1"),
        );
    }

    // bearDog is the root: it must have no depends_on
    let beardog_node = nodes
        .iter()
        .find(|n| n.get("binary").and_then(|b| b.as_str()) == Some("beardog"));
    if let Some(bd) = beardog_node {
        let deps = bd
            .get("depends_on")
            .and_then(|d| d.as_array())
            .map_or(0, std::vec::Vec::len);
        let no_depends_on = !bd.as_table().is_some_and(|t| t.contains_key("depends_on"));
        v.check_bool(
            "bootstrap:beardog_root",
            no_depends_on || deps == 0,
            "bearDog has no dependencies (crypto identity root)",
        );

        let order = bd
            .get("order")
            .and_then(toml::Value::as_integer)
            .unwrap_or(0);
        v.check_bool(
            "bootstrap:beardog_first",
            order == 1,
            &format!("bearDog is order 1, got {order}"),
        );
    }

    // No circular dependencies in Phase 1: songbird/skunkbat depend on beardog, not each other
    for node in &phase1_nodes {
        let name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");
        let binary = node
            .get("binary")
            .and_then(|b| b.as_str())
            .unwrap_or("");
        let deps: Vec<&str> = node
            .get("depends_on")
            .and_then(|d| d.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        if binary == "beardog" {
            continue;
        }

        let depends_only_on_beardog = deps.iter().all(|d| d.contains("beardog"));
        v.check_bool(
            &format!("bootstrap:no_circular:{name}"),
            depends_only_on_beardog,
            &format!("{name} depends only on beardog node(s), not on other Tower primals"),
        );
    }

    // Phase 2 nodes must be required = false (Tower can run without biomeOS)
    let phase2_nodes: Vec<&toml::Value> = nodes
        .iter()
        .filter(|n| {
            n.get("phase")
                .and_then(toml::Value::as_integer) == Some(2)
        })
        .collect();

    v.check_bool(
        "bootstrap:phase2_count",
        phase2_nodes.len() == 3,
        &format!(
            "Phase 2 has {} nodes, expected 3 (biomeos discover, registry seed, health verify)",
            phase2_nodes.len()
        ),
    );

    for node in &phase2_nodes {
        let name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");
        let required = node
            .get("required")
            .and_then(toml::Value::as_bool)
            .unwrap_or(true);
        v.check_bool(
            &format!("bootstrap:phase2:{name}:optional"),
            !required,
            &format!("{name} is required = false (Phase 2 is optional without biomeOS)"),
        );
    }
}

/// Run the agentic tower validation scenario.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Tower Agent Composition");
    validate_tower_agent_structure(v);

    v.section("AI Overlay Audit Coverage");
    validate_ai_overlays_have_skunkbat(v);

    v.section("Meta Deploy Signal");
    validate_meta_deploy_signal(v);

    v.section("Signal Tool Schema Deploy");
    validate_signal_tools_deploy(v);

    v.section("Squirrel Signal Context");
    validate_squirrel_signal_context(v);

    v.section("Tower Bootstrap");
    validate_tower_bootstrap(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agentic_tower_pass() {
        let mut v = ValidationResult::new("agentic-tower");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "agentic tower scenario had {} failures (use --nocapture for details)",
            v.failed
        );
    }

    #[test]
    fn tower_agent_parses() {
        let result: Result<toml::Value, _> = toml::from_str(TOWER_AGENT_TOML);
        assert!(result.is_ok(), "tower_agent.toml parse failed: {:?}", result.err());
    }

    #[test]
    fn tower_bootstrap_parses_and_validates() {
        let parsed: toml::Value =
            toml::from_str(TOWER_BOOTSTRAP_TOML).expect("tower_bootstrap.toml should parse");
        let nodes = parsed["graph"]["nodes"]
            .as_array()
            .expect("bootstrap graph should have nodes");

        let beardog = nodes
            .iter()
            .find(|n| n.get("binary").and_then(|b| b.as_str()) == Some("beardog"))
            .expect("beardog node should exist");
        assert!(
            !beardog
                .as_table()
                .unwrap()
                .contains_key("depends_on"),
            "beardog must have no depends_on (it is the bootstrap root)"
        );
        assert_eq!(
            beardog["order"].as_integer(),
            Some(1),
            "beardog must be order 1"
        );
    }

    #[test]
    fn all_ai_overlays_have_skunkbat() {
        for (name, content) in [
            ("tower_ai", TOWER_AI_TOML),
            ("tower_ai_viz", TOWER_AI_VIZ_TOML),
            ("node_ai", NODE_AI_TOML),
        ] {
            let binaries = helpers::graph_binaries(content);
            assert!(
                binaries.iter().any(|b| b == "skunkbat"),
                "{name}.toml missing skunkbat"
            );
        }
    }
}
