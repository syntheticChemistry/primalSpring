// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Membrane Composition — structural validation of the VPS membrane
//! deploy graph (`graphs/membrane/tower_membrane.toml`).
//!
//! Validates that the membrane graph declares the correct security model,
//! bonding policy, channel architecture, and node set for a sovereign VPS
//! boundary deployment. All checks are Tier::Rust — no live primals required.
//!
//! Four pillars:
//! 1. Graph metadata (secure_by_default, composition_model, channels)
//! 2. Tower node completeness (BearDog + Songbird + SkunkBat + NestGate)
//! 3. Bonding policy alignment (encryption tiers, covalent tower_internal)
//! 4. Telemetry contract (shadow_mode, cutover_gate_days, skunkbat_correlation)

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "membrane-composition",
        track: Track::Sovereignty,
        tier: Tier::Rust,
        provenance_crate: "primalspring_sovereignty",
        provenance_date: "2026-05-15",
        description:
            "Membrane deploy graph — metadata, nodes, bonding, telemetry structural validation",
    },
    run,
};

// ─── Pillar 1: Graph Metadata ────────────────────────────────────────────────

fn pillar_graph_metadata(v: &mut ValidationResult, parsed: &toml::Value) {
    let graph = parsed.get("graph");
    v.check_bool(
        "metadata:graph_section_present",
        graph.is_some(),
        "top-level [graph] section exists",
    );

    let Some(graph) = graph else { return };

    let name = graph.get("name").and_then(|v| v.as_str()).unwrap_or("");
    v.check_bool(
        "metadata:graph_name",
        name == "tower_membrane",
        &format!("graph.name = \"{name}\" (expect tower_membrane)"),
    );

    let meta = graph.get("metadata");
    v.check_bool(
        "metadata:metadata_section",
        meta.is_some(),
        "[graph.metadata] section exists",
    );

    let Some(meta) = meta else { return };

    let secure = meta
        .get("secure_by_default")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        "metadata:secure_by_default",
        secure,
        &format!("secure_by_default = {secure} (expect true)"),
    );

    let comp_model = meta
        .get("composition_model")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "metadata:composition_model",
        comp_model == "membrane",
        &format!("composition_model = \"{comp_model}\" (expect membrane)"),
    );

    let transport = meta
        .get("transport")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "metadata:transport_uds_only",
        transport == "uds_only",
        &format!("transport = \"{transport}\" (expect uds_only)"),
    );

    let tcp_ports = meta
        .get("tcp_ports")
        .and_then(toml::Value::as_integer)
        .unwrap_or(-1);
    v.check_bool(
        "metadata:zero_tcp_ports",
        tcp_ports == 0,
        &format!("tcp_ports = {tcp_ports} (expect 0)"),
    );

    let channels = meta
        .get("channels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let expected_channels = ["signal", "relay", "surface"];
    v.check_bool(
        "metadata:three_channels",
        channels.len() == 3
            && expected_channels
                .iter()
                .all(|c| channels.contains(c)),
        &format!(
            "channels = {channels:?} (expect signal, relay, surface)"
        ),
    );
}

// ─── Pillar 2: Tower Node Completeness ───────────────────────────────────────

fn pillar_tower_nodes(v: &mut ValidationResult, parsed: &toml::Value) {
    let nodes = parsed
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .cloned()
        .unwrap_or_default();

    let node_names: Vec<&str> = nodes
        .iter()
        .filter_map(|n| n.get("name").and_then(|v| v.as_str()))
        .collect();

    let required_tower = ["beardog", "songbird", "skunkbat"];
    for name in &required_tower {
        v.check_bool(
            &format!("nodes:tower:{name}_present"),
            node_names.contains(name),
            &format!("{name} node in membrane graph"),
        );
    }

    v.check_bool(
        "nodes:nestgate_present",
        node_names.contains(&"nestgate"),
        "NestGate cache node in membrane graph",
    );

    v.check_bool(
        "nodes:biomeos_present",
        node_names.contains(&"biomeos_neural_api"),
        "biomeOS orchestration substrate in membrane graph",
    );

    for node in &nodes {
        let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("");
        if name == "biomeos_neural_api" {
            continue;
        }
        let sec_model = node
            .get("security_model")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            &format!("nodes:{name}:btsp_security"),
            sec_model == "btsp",
            &format!("{name} security_model = \"{sec_model}\" (expect btsp)"),
        );
    }

    let biomeos = nodes.iter().find(|n| {
        n.get("name")
            .and_then(|v| v.as_str()) == Some("biomeos_neural_api")
    });
    if let Some(bio) = biomeos {
        let spawn = bio
            .get("spawn")
            .and_then(toml::Value::as_bool)
            .unwrap_or(true);
        v.check_bool(
            "nodes:biomeos:spawn_false",
            !spawn,
            &format!("biomeOS spawn = {spawn} (expect false — external orchestrator)"),
        );
    }

    let nestgate = nodes.iter().find(|n| {
        n.get("name")
            .and_then(|v| v.as_str()) == Some("nestgate")
    });
    if let Some(ng) = nestgate {
        let cap = ng
            .get("by_capability")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "nodes:nestgate:content_capability",
            cap == "content",
            &format!("NestGate by_capability = \"{cap}\" (expect content for cache role)"),
        );
    }
}

// ─── Pillar 3: Bonding Policy ────────────────────────────────────────────────

fn pillar_bonding_policy(v: &mut ValidationResult, parsed: &toml::Value) {
    let policy = parsed
        .get("graph")
        .and_then(|g| g.get("bonding_policy"));

    v.check_bool(
        "bonding:policy_present",
        policy.is_some(),
        "[graph.bonding_policy] section exists",
    );

    let Some(policy) = policy else { return };

    let tower_internal = policy
        .get("tower_internal")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "bonding:tower_internal_covalent",
        tower_internal == "covalent",
        &format!("tower_internal = \"{tower_internal}\" (expect covalent)"),
    );

    let enc_tiers = policy.get("encryption_tiers");
    v.check_bool(
        "bonding:encryption_tiers_present",
        enc_tiers.is_some(),
        "encryption_tiers section exists",
    );

    if let Some(et) = enc_tiers {
        let tower_enc = et.get("tower").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "bonding:tower_encryption_full",
            tower_enc == "full",
            &format!("encryption_tiers.tower = \"{tower_enc}\" (expect full)"),
        );
    }

    let bonding_default = parsed
        .get("graph")
        .and_then(|g| g.get("bonding_default"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "bonding:default_btsp_required",
        bonding_default == "btsp_required",
        &format!("bonding_default = \"{bonding_default}\" (expect btsp_required)"),
    );
}

// ─── Pillar 4: Telemetry Contract ────────────────────────────────────────────

fn pillar_telemetry(v: &mut ValidationResult, parsed: &toml::Value) {
    let telemetry = parsed
        .get("graph")
        .and_then(|g| g.get("telemetry"));

    v.check_bool(
        "telemetry:section_present",
        telemetry.is_some(),
        "[graph.telemetry] section exists",
    );

    let Some(telemetry) = telemetry else { return };

    let enabled = telemetry
        .get("enabled")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        "telemetry:enabled",
        enabled,
        &format!("telemetry.enabled = {enabled} (expect true)"),
    );

    let shadow = telemetry
        .get("shadow_mode")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    v.check_bool(
        "telemetry:shadow_mode_permanent",
        shadow == "permanent",
        &format!("shadow_mode = \"{shadow}\" (expect permanent)"),
    );

    let cutover_days = telemetry
        .get("cutover_gate_days")
        .and_then(toml::Value::as_integer)
        .unwrap_or(0);
    v.check_bool(
        "telemetry:cutover_gate_days",
        cutover_days >= 7,
        &format!("cutover_gate_days = {cutover_days} (expect >= 7)"),
    );

    let skunkbat = telemetry
        .get("skunkbat_correlation")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    v.check_bool(
        "telemetry:skunkbat_correlation",
        skunkbat,
        &format!("skunkbat_correlation = {skunkbat} (expect true)"),
    );
}

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Run the membrane composition validation across all four pillars.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let membrane_toml = include_str!("../../../../graphs/membrane/tower_membrane.toml");
    let parsed: toml::Value = match toml::from_str(membrane_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "parse:tower_membrane",
                false,
                &format!("tower_membrane.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "parse:tower_membrane",
        true,
        "tower_membrane.toml parses as valid TOML",
    );

    v.section("Pillar 1: Graph Metadata");
    pillar_graph_metadata(v, &parsed);

    v.section("Pillar 2: Tower Node Completeness");
    pillar_tower_nodes(v, &parsed);

    v.section("Pillar 3: Bonding Policy");
    pillar_bonding_policy(v, &parsed);

    v.section("Pillar 4: Telemetry Contract");
    pillar_telemetry(v, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn membrane_composition_structural() {
        let mut v = ValidationResult::new("membrane-composition");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.failed == 0,
            "Membrane composition has {} failures (run with --nocapture to see summary)",
            v.failed,
        );
    }

    #[test]
    fn membrane_graph_has_five_nodes() {
        let toml_str = include_str!("../../../../graphs/membrane/tower_membrane.toml");
        let parsed: toml::Value = toml::from_str(toml_str).expect("valid TOML");
        let nodes = parsed
            .get("graph")
            .and_then(|g| g.get("nodes"))
            .and_then(|n| n.as_array())
            .expect("graph.nodes array");
        assert_eq!(nodes.len(), 5, "membrane graph should have 5 nodes");
    }

    #[test]
    fn membrane_composition_model_distinct() {
        let membrane_str = include_str!("../../../../graphs/membrane/tower_membrane.toml");
        let nucleus_str = include_str!("../../../../graphs/nucleus_complete.toml");
        assert!(
            membrane_str.contains("composition_model = \"membrane\""),
            "membrane graph must use composition_model = membrane"
        );
        assert!(
            nucleus_str.contains("composition_model = \"nucleated\""),
            "nucleus graph uses composition_model = nucleated"
        );
    }
}
