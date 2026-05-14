// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Routing Consistency — structural validation of wire routing.
//!
//! Verifies that `capability_registry.toml`, `method_to_capability_domain`,
//! and `capability_to_primal` routing tables plus graph fragment capabilities
//! are mutually consistent. Catches bugs where a method's wire prefix does not
//! match its owning primal's domain.

use crate::composition::{capability_to_primal, method_to_capability_domain, CompositionContext};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Routing consistency scenario — Tier::Rust structural check.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "routing-consistency",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_routing_audit",
        provenance_date: "2026-05-13",
        description: "Registry ↔ routing ↔ graph consistency: every method routes to its owner",
    },
    run,
};

struct RegistryEntry {
    domain: String,
    owner: String,
    methods: Vec<String>,
}

#[allow(clippy::expect_used)]
fn parse_registry() -> Vec<RegistryEntry> {
    // SAFETY: capability_registry.toml is compile-time embedded — parse cannot fail
    // unless the file itself is malformed, which is caught at build time.
    let toml_str = include_str!("../../../../config/capability_registry.toml");
    let parsed: toml::Value = toml::from_str(toml_str).expect("embedded TOML must parse");
    let table = parsed.as_table().expect("embedded TOML must be a table");

    let mut entries = Vec::new();
    for (domain, section) in table {
        let owner = section
            .get("owner")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_owned();
        let methods: Vec<String> = section
            .get("methods")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        entries.push(RegistryEntry {
            domain: domain.clone(),
            owner,
            methods,
        });
    }
    entries
}

fn phase_method_routing(v: &mut ValidationResult) {
    let entries = parse_registry();
    let mut misroutes = 0u32;
    let mut total = 0u32;

    let skip_owners = ["all", "none", "tests"];
    for entry in &entries {
        if skip_owners.contains(&entry.owner.as_str()) {
            continue;
        }
        for method in &entry.methods {
            total += 1;
            let routed_domain = method_to_capability_domain(method);
            let routed_primal = capability_to_primal(routed_domain);

            if routed_primal != entry.owner {
                v.check_bool(
                    &format!("route:{method}"),
                    false,
                    &format!(
                        "method '{method}' in [{domain}] (owner={owner}) routes to \
                         domain '{routed_domain}' → primal '{routed_primal}' (expected '{owner}')",
                        domain = entry.domain,
                        owner = entry.owner,
                    ),
                );
                misroutes += 1;
            }
        }
    }

    v.check_bool(
        "routing:all_methods_reach_owner",
        misroutes == 0,
        &format!("{total} methods checked, {misroutes} misroutes"),
    );
}

fn phase_graph_methods(v: &mut ValidationResult) {
    let entries = parse_registry();
    let all_methods: Vec<&str> = entries.iter().flat_map(|e| e.methods.iter().map(String::as_str)).collect();

    let tower_toml = include_str!("../../../../graphs/fragments/tower_atomic.toml");
    check_graph_capabilities(v, "tower_atomic", tower_toml, &all_methods);

    let node_toml = include_str!("../../../../graphs/fragments/node_atomic.toml");
    check_graph_capabilities(v, "node_atomic", node_toml, &all_methods);

    let nest_toml = include_str!("../../../../graphs/fragments/nest_atomic.toml");
    check_graph_capabilities(v, "nest_atomic", nest_toml, &all_methods);

    let nucleus_toml = include_str!("../../../../graphs/fragments/nucleus.toml");
    check_graph_capabilities(v, "nucleus", nucleus_toml, &all_methods);

    let meta_toml = include_str!("../../../../graphs/fragments/meta_tier.toml");
    check_graph_capabilities(v, "meta_tier", meta_toml, &all_methods);

    let prov_toml = include_str!("../../../../graphs/fragments/provenance_trio.toml");
    check_graph_capabilities(v, "provenance_trio", prov_toml, &all_methods);
}

fn check_graph_capabilities(
    v: &mut ValidationResult,
    graph_name: &str,
    toml_str: &str,
    all_registry_methods: &[&str],
) {
    let parsed: toml::Value = match toml::from_str(toml_str) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                &format!("graph:{graph_name}:parse"),
                false,
                &format!("TOML parse error: {e}"),
            );
            return;
        }
    };

    if let Some(nodes) = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array())
    {
        for node in nodes {
            let name = node
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            if let Some(caps) = node.get("capabilities").and_then(|c| c.as_array()) {
                for cap in caps {
                    if let Some(method) = cap.as_str() {
                        let found = all_registry_methods.contains(&method);
                        if !found {
                            v.check_bool(
                                &format!("graph:{graph_name}:{name}:{method}"),
                                false,
                                &format!(
                                    "graph capability '{method}' for node '{name}' \
                                     not in capability_registry.toml"
                                ),
                            );
                        }
                    }
                }
            }
        }
    }

    v.check_bool(
        &format!("graph:{graph_name}:parsed"),
        true,
        &format!("{graph_name} fragment capabilities checked against registry"),
    );
}

fn phase_encoding_rules(v: &mut ValidationResult) {
    let crypto_methods = [
        "crypto.sign",
        "crypto.verify",
        "crypto.hash",
        "crypto.encrypt_chacha20_poly1305",
    ];

    for method in &crypto_methods {
        let domain = method_to_capability_domain(method);
        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("encoding:{method}:routes_to_beardog"),
            primal == "beardog",
            &format!("{method} → {domain} → {primal}"),
        );
    }

    let defense_methods = [
        "defense.audit",
        "defense.status",
        "security.audit_log",
        "recon.scan",
        "threat.assess",
    ];

    for method in &defense_methods {
        let domain = method_to_capability_domain(method);
        let primal = capability_to_primal(domain);
        v.check_bool(
            &format!("encoding:{method}:routes_to_skunkbat"),
            primal == "skunkbat",
            &format!("{method} → {domain} → {primal}"),
        );
    }
}

/// Execute routing consistency checks across registry, routing tables, and graphs.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Method → Domain → Primal routing consistency");
    phase_method_routing(v);

    v.section("Phase 2: Graph fragment capabilities ⊆ registry");
    phase_graph_methods(v);

    v.section("Phase 3: Encoding and domain ownership rules");
    phase_encoding_rules(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_methods_route_to_owners() {
        let entries = parse_registry();
        let mut misroutes = Vec::new();
        let skip_owners = ["all", "none", "tests"];

        for entry in &entries {
            if skip_owners.contains(&entry.owner.as_str()) {
                continue;
            }
            for method in &entry.methods {
                let routed_domain = method_to_capability_domain(method);
                let routed_primal = capability_to_primal(routed_domain);
                if routed_primal != entry.owner {
                    misroutes.push(format!(
                        "{method}: [{domain}] owner={owner}, routes to {routed_domain}→{routed_primal}",
                        domain = entry.domain,
                        owner = entry.owner,
                    ));
                }
            }
        }

        assert!(
            misroutes.is_empty(),
            "routing mismatches:\n{}",
            misroutes.join("\n")
        );
    }

    fn check_fragment(toml_str: &str, label: &str, all_methods: &[&str]) -> Vec<String> {
        let parsed: toml::Value = toml::from_str(toml_str).unwrap();
        let mut missing = Vec::new();

        if let Some(nodes) = parsed
            .get("fragment")
            .and_then(|f| f.get("nodes"))
            .and_then(|n| n.as_array())
        {
            for node in nodes {
                let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                if let Some(caps) = node.get("capabilities").and_then(|c| c.as_array()) {
                    for cap in caps {
                        if let Some(m) = cap.as_str() {
                            if !all_methods.contains(&m) {
                                missing.push(format!("{label}/{name}: {m}"));
                            }
                        }
                    }
                }
            }
        }
        missing
    }

    #[test]
    fn graph_capabilities_in_registry() {
        let entries = parse_registry();
        let all_methods: Vec<&str> = entries
            .iter()
            .flat_map(|e| e.methods.iter().map(String::as_str))
            .collect();

        let mut missing = Vec::new();
        missing.extend(check_fragment(
            include_str!("../../../../graphs/fragments/tower_atomic.toml"),
            "tower_atomic", &all_methods,
        ));
        missing.extend(check_fragment(
            include_str!("../../../../graphs/fragments/node_atomic.toml"),
            "node_atomic", &all_methods,
        ));
        missing.extend(check_fragment(
            include_str!("../../../../graphs/fragments/nest_atomic.toml"),
            "nest_atomic", &all_methods,
        ));
        missing.extend(check_fragment(
            include_str!("../../../../graphs/fragments/nucleus.toml"),
            "nucleus", &all_methods,
        ));
        missing.extend(check_fragment(
            include_str!("../../../../graphs/fragments/meta_tier.toml"),
            "meta_tier", &all_methods,
        ));
        missing.extend(check_fragment(
            include_str!("../../../../graphs/fragments/provenance_trio.toml"),
            "provenance_trio", &all_methods,
        ));

        assert!(
            missing.is_empty(),
            "graph capabilities not in registry:\n{}",
            missing.join("\n")
        );
    }
}
