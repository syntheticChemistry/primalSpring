// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Dark Forest Glacial Gate — five-pillar security invariant validation.
//!
//! Structural validation of the Dark Forest standard: zero metadata leakage,
//! zero port exposure, Songbird-only network surface, BTSP crypto integrity,
//! and enclave computing boundaries. All checks are Tier::Rust — no live
//! primals required.
//!
//! See `specs/DARK_FOREST_GLACIAL_GATE.md` for the full standard.

use crate::composition::{capability_to_primal, tcp_fallback_table, CompositionContext};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "dark-forest-gate",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "primalspring_dark_forest_gate",
        provenance_date: "2026-05-14",
        description:
            "Dark Forest glacial gate — metadata, ports, network surface, BTSP, enclave",
    },
    run,
};

// ─── Pillar 1: Zero Metadata Leakage ────────────────────────────────────────

fn pillar_metadata_leakage(v: &mut ValidationResult) {
    let manifest_toml = include_str!("../../../../config/capability_registry.toml");
    let parsed: toml::Value = match toml::from_str(manifest_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "metadata:registry_parse",
                false,
                &format!("capability_registry.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "metadata:registry_parse",
        true,
        "capability_registry.toml parses as valid TOML",
    );

    let empty_map = toml::map::Map::new();
    let table = parsed.as_table().unwrap_or(&empty_map);
    let skip_sections = ["test_fixtures", "false_positives"];
    let domain_count = table
        .keys()
        .filter(|k| !skip_sections.contains(&k.as_str()))
        .count();

    v.check_bool(
        "metadata:domain_count",
        domain_count >= 10,
        &format!("{domain_count} capability domains (expect ≥10 for NUCLEUS coverage)"),
    );

    let tower_toml = include_str!("../../../../graphs/fragments/tower_atomic.toml");
    let tower_has_btsp = tower_toml.contains("security_model = \"btsp\"");
    v.check_bool(
        "metadata:tower_btsp_security_model",
        tower_has_btsp,
        "tower_atomic fragment declares security_model = btsp (encrypted identity)",
    );

    let beacon_genetics_reference = tower_toml.contains("beardog")
        && tower_toml.contains("songbird")
        && tower_toml.contains("skunkbat");
    v.check_bool(
        "metadata:tower_complete",
        beacon_genetics_reference,
        "tower_atomic includes all 3 primals (BearDog + Songbird + skunkBat)",
    );
}

// ─── Pillar 2: Zero Port Exposure ───────────────────────────────────────────

fn pillar_zero_port(v: &mut ValidationResult) {
    let tier5_env = std::env::var(crate::env_keys::PRIMALSPRING_TCP_TIER5).unwrap_or_default();
    let tier5_on = tier5_env == "1" || tier5_env.eq_ignore_ascii_case("true");
    v.check_bool(
        "port:tier5_default_off",
        !tier5_on,
        "PRIMALSPRING_TCP_TIER5 is unset/false (zero-port standard)",
    );

    let table = tcp_fallback_table();
    let mut port_to_primal: std::collections::HashMap<u16, &str> =
        std::collections::HashMap::new();
    let mut real_collisions = Vec::new();
    for &(_, primal, _, port) in &table {
        if let Some(&existing) = port_to_primal.get(&port) {
            if existing != primal {
                real_collisions.push(format!("{port}: {existing} vs {primal}"));
            }
        } else {
            port_to_primal.insert(port, primal);
        }
    }

    v.check_bool(
        "port:no_primal_collisions",
        real_collisions.is_empty(),
        &format!(
            "{} real port collisions (distinct primals on same port): {:?}",
            real_collisions.len(),
            real_collisions
        ),
    );

    v.check_bool(
        "port:13_primals_assigned",
        table.len() >= 13,
        &format!("{} primals in tcp_fallback_table (expect ≥13)", table.len()),
    );

    let deployment_matrix_toml = include_str!("../../../../config/deployment_matrix.toml");
    let dm_parsed: Result<toml::Value, _> = toml::from_str(deployment_matrix_toml);
    match dm_parsed {
        Ok(dm) => {
            let uds_default = dm
                .get("transports")
                .and_then(|t| t.get("uds_only"))
                .and_then(|u| u.get("description"))
                .and_then(|d| d.as_str())
                .unwrap_or("")
                .contains("DEFAULT");
            v.check_bool(
                "port:uds_only_is_default",
                uds_default,
                "deployment_matrix marks uds_only as DEFAULT transport",
            );
        }
        Err(e) => {
            v.check_bool(
                "port:deployment_matrix_parse",
                false,
                &format!("deployment_matrix.toml parse error: {e}"),
            );
        }
    }
}

// ─── Pillar 3: Songbird as Sole Network Surface ─────────────────────────────

fn pillar_songbird_network(v: &mut ValidationResult) {
    let tower_toml = include_str!("../../../../graphs/fragments/tower_atomic.toml");
    let parsed: toml::Value = match toml::from_str(tower_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "network:tower_parse",
                false,
                &format!("tower_atomic fragment parse error: {e}"),
            );
            return;
        }
    };

    let nodes = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array())
        .map_or(Vec::new(), std::clone::Clone::clone);

    let songbird_node = nodes.iter().find(|n| {
        n.get("name")
            .and_then(|v| v.as_str()) == Some("songbird")
    });

    v.check_bool(
        "network:songbird_in_tower",
        songbird_node.is_some(),
        "Songbird is present in tower_atomic fragment",
    );

    if let Some(sb) = songbird_node {
        let caps = sb
            .get("capabilities")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let has_http = caps.iter().any(|c| c.starts_with("http."));
        let has_discovery = caps.iter().any(|c| c.starts_with("discovery."));
        let has_ipc = caps.iter().any(|c| c.starts_with("ipc."));

        v.check_bool(
            "network:songbird_has_http",
            has_http,
            "Songbird owns http.* capabilities",
        );
        v.check_bool(
            "network:songbird_has_discovery",
            has_discovery,
            "Songbird owns discovery.* capabilities",
        );
        v.check_bool(
            "network:songbird_has_ipc",
            has_ipc,
            "Songbird owns ipc.* capabilities",
        );
    }

    let network_nodes_with_http: Vec<_> = nodes
        .iter()
        .filter(|n| {
            let name = n.get("name").and_then(|v| v.as_str()).unwrap_or("");
            name != "songbird"
        })
        .filter(|n| {
            n.get("capabilities")
                .and_then(|c| c.as_array())
                .is_some_and(|arr| {
                    arr.iter().any(|v| {
                        v.as_str()
                            .is_some_and(|s| s.starts_with("http.") || s.starts_with("tls."))
                    })
                })
        })
        .collect();

    v.check_bool(
        "network:no_non_songbird_http",
        network_nodes_with_http.is_empty(),
        &format!(
            "{} non-Songbird nodes with http/tls capabilities in tower",
            network_nodes_with_http.len()
        ),
    );

    let discovery_primal = capability_to_primal("discovery");
    v.check_bool(
        "network:discovery_routes_to_songbird",
        discovery_primal == "songbird",
        &format!("capability 'discovery' routes to: {discovery_primal}"),
    );

    let network_primal = capability_to_primal("network");
    let network_ok = network_primal == "songbird" || network_primal == "unknown";
    v.check_bool(
        "network:network_routes_to_songbird",
        network_ok,
        &format!("capability 'network' routes to: {network_primal}"),
    );
}

// ─── Pillar 4: BTSP Crypto Integrity ────────────────────────────────────────

fn pillar_btsp_crypto(v: &mut ValidationResult) {
    let registry_toml = include_str!("../../../../config/capability_registry.toml");
    let parsed: toml::Value = match toml::from_str(registry_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "btsp:registry_parse",
                false,
                &format!("registry parse error: {e}"),
            );
            return;
        }
    };

    let empty_map = toml::map::Map::new();
    let table = parsed.as_table().unwrap_or(&empty_map);
    let has_btsp_negotiate = table.iter().any(|(_, section)| {
        section
            .get("methods")
            .and_then(|m| m.as_array())
            .is_some_and(|arr| {
                arr.iter()
                    .any(|v| v.as_str() == Some("btsp.negotiate"))
            })
    });

    v.check_bool(
        "btsp:negotiate_registered",
        has_btsp_negotiate,
        "btsp.negotiate is registered in capability_registry",
    );

    let has_btsp_capabilities = table.iter().any(|(_, section)| {
        section
            .get("methods")
            .and_then(|m| m.as_array())
            .is_some_and(|arr| {
                arr.iter()
                    .any(|v| v.as_str() == Some("btsp.capabilities"))
            })
    });

    v.check_bool(
        "btsp:capabilities_registered",
        has_btsp_capabilities,
        "btsp.capabilities is registered in capability_registry",
    );

    let all_fragments = [
        (
            "tower_atomic",
            include_str!("../../../../graphs/fragments/tower_atomic.toml"),
        ),
        (
            "node_atomic",
            include_str!("../../../../graphs/fragments/node_atomic.toml"),
        ),
        (
            "nest_atomic",
            include_str!("../../../../graphs/fragments/nest_atomic.toml"),
        ),
        (
            "nucleus",
            include_str!("../../../../graphs/fragments/nucleus.toml"),
        ),
    ];

    for (frag_name, frag_toml) in &all_fragments {
        let has_btsp_model = frag_toml.contains("security_model = \"btsp\"");
        v.check_bool(
            &format!("btsp:fragment:{frag_name}:has_btsp"),
            has_btsp_model,
            &format!("{frag_name} declares security_model = btsp on tower nodes"),
        );

        let has_trust_model = frag_toml.contains("trust_model = \"MethodGate\"");
        v.check_bool(
            &format!("btsp:fragment:{frag_name}:method_gate"),
            has_trust_model,
            &format!("{frag_name} declares trust_model = MethodGate"),
        );
    }

    let crypto_methods = [
        "crypto.encrypt_chacha20_poly1305",
        "crypto.sign",
        "crypto.verify",
        "crypto.hash",
    ];

    let crypto_section = table
        .get("crypto")
        .and_then(|s| s.get("methods"))
        .and_then(|m| m.as_array());

    if let Some(methods) = crypto_section {
        let method_strs: Vec<&str> = methods.iter().filter_map(|v| v.as_str()).collect();
        for expected in &crypto_methods {
            v.check_bool(
                &format!("btsp:crypto:{expected}"),
                method_strs.contains(expected),
                &format!("{expected} in crypto domain"),
            );
        }
    } else {
        v.check_bool(
            "btsp:crypto_domain_exists",
            false,
            "crypto domain not found in capability_registry",
        );
    }
}

// ─── Pillar 5: Enclave Computing ────────────────────────────────────────────

fn pillar_enclave(v: &mut ValidationResult) {
    let nest_toml = include_str!("../../../../graphs/fragments/nest_atomic.toml");
    let parsed: toml::Value = match toml::from_str(nest_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "enclave:nest_parse",
                false,
                &format!("nest_atomic parse error: {e}"),
            );
            return;
        }
    };

    let nodes = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array());

    let nestgate_node = nodes.and_then(|arr| {
        arr.iter().find(|n| {
            n.get("name")
                .and_then(|v| v.as_str()) == Some("nestgate")
        })
    });

    v.check_bool(
        "enclave:nestgate_present",
        nestgate_node.is_some(),
        "NestGate node present in nest_atomic fragment",
    );

    if let Some(ng) = nestgate_node {
        let by_cap = ng
            .get("by_capability")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "enclave:nestgate_storage_capability",
            by_cap == "storage",
            &format!("NestGate by_capability = {by_cap} (expect storage)"),
        );
    }

    let storage_routes_to = capability_to_primal("storage");
    v.check_bool(
        "enclave:storage_routes_to_nestgate",
        storage_routes_to == "nestgate",
        &format!("storage capability routes to: {storage_routes_to}"),
    );

    let prov_toml = include_str!("../../../../graphs/fragments/provenance_trio.toml");
    let prov_parsed: toml::Value = match toml::from_str(prov_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "enclave:provenance_parse",
                false,
                &format!("provenance_trio parse error: {e}"),
            );
            return;
        }
    };

    let prov_nodes = prov_parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array());

    let prov_names: Vec<&str> = prov_nodes
        .map(|arr| {
            arr.iter()
                .filter_map(|n| n.get("name").and_then(|v| v.as_str()))
                .collect()
        })
        .unwrap_or_default();

    v.check_bool(
        "enclave:provenance_has_rhizocrypt",
        prov_names.contains(&"rhizocrypt"),
        "provenance_trio includes rhizoCrypt (DAG lineage)",
    );
    v.check_bool(
        "enclave:provenance_has_loamspine",
        prov_names.contains(&"loamspine"),
        "provenance_trio includes loamSpine (permanent ledger)",
    );
    v.check_bool(
        "enclave:provenance_has_sweetgrass",
        prov_names.contains(&"sweetgrass"),
        "provenance_trio includes sweetGrass (attribution)",
    );

    let attribution_primal = capability_to_primal("attribution");
    v.check_bool(
        "enclave:attribution_routes_to_sweetgrass",
        attribution_primal == "sweetgrass",
        &format!("attribution routes to: {attribution_primal} (opaque agent identifiers)"),
    );

    let bonding_policy = parsed.get("graph").and_then(|g| g.get("bonding_policy"));
    if let Some(bp) = bonding_policy {
        let trust = bp
            .get("trust_model")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "enclave:nest_trust_model",
            trust == "MethodGate",
            &format!("nest_atomic trust_model = {trust} (expect MethodGate)"),
        );
    } else {
        v.check_bool(
            "enclave:nest_bonding_policy",
            false,
            "nest_atomic missing bonding_policy",
        );
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Run the Dark Forest glacial gate validation across all five pillars.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Pillar 1: Zero Metadata Leakage");
    pillar_metadata_leakage(v);

    v.section("Pillar 2: Zero Port Exposure");
    pillar_zero_port(v);

    v.section("Pillar 3: Songbird as Sole Network Surface");
    pillar_songbird_network(v);

    v.section("Pillar 4: BTSP Crypto Integrity");
    pillar_btsp_crypto(v);

    v.section("Pillar 5: Enclave Computing");
    pillar_enclave(v);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn dark_forest_gate_structural() {
        let mut v = ValidationResult::new("dark-forest-gate");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.failed == 0,
            "Dark Forest gate has {} failures (run with --nocapture to see summary)",
            v.failed,
        );
    }

    #[test]
    fn tier5_tcp_off_in_test_env() {
        let tier5 = std::env::var(crate::env_keys::PRIMALSPRING_TCP_TIER5).unwrap_or_default();
        assert!(
            tier5.is_empty() || tier5 == "0" || tier5.eq_ignore_ascii_case("false"),
            "PRIMALSPRING_TCP_TIER5 should be unset/false in test environment, got: '{tier5}'"
        );
    }

    #[test]
    fn songbird_owns_network_capabilities() {
        let primal = crate::composition::capability_to_primal("discovery");
        assert_eq!(primal, "songbird", "discovery should route to songbird");
    }

    #[test]
    fn btsp_negotiate_registered() {
        let toml_str = include_str!("../../../../config/capability_registry.toml");
        assert!(
            toml_str.contains("btsp.negotiate"),
            "btsp.negotiate must be in capability_registry.toml"
        );
    }

    #[test]
    fn provenance_trio_complete() {
        let toml_str = include_str!("../../../../graphs/fragments/provenance_trio.toml");
        assert!(toml_str.contains("rhizocrypt"), "missing rhizocrypt");
        assert!(toml_str.contains("loamspine"), "missing loamspine");
        assert!(toml_str.contains("sweetgrass"), "missing sweetgrass");
    }
}
