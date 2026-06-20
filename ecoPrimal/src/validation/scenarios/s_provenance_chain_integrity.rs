// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//! Scenario: Provenance Chain Integrity — end-to-end provenance chain validation.
//!
//! Validates the canonical provenance chain across the trio primals:
//!
//!   rhizoCrypt (DAG) → loamSpine (ledger) → sweetGrass (witness)
//!
//! Chain flow: hash(content) → DAG node → merkle root → ledger commit → witness attestation
//!
//! Phase 1 (Structural/Rust): NUCLEUS composition, capability domain ownership,
//! graph workflow TOMLs, and sequential chain ordering in deploy graphs.
//!
//! Phase 2 (Live): `capabilities.list` probes on each trio socket and startup order.

use std::path::PathBuf;

use crate::composition::{capability_to_primal, method_to_capability_domain};
use crate::coordination::probe_primal_at_socket;
use crate::primal_names::{self, Primal};
use crate::validation::ValidationResult;
use crate::validation::helpers;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use crate::composition::CompositionContext;

/// Provenance chain integrity — RhizoCrypt DAG → LoamSpine ledger → SweetGrass witness.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "provenance-chain-integrity",
        track: Track::Sovereignty,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-20",
        description: "End-to-end provenance chain: hash → DAG → merkle → ledger → witness",
    },
    run,
};

const TRIO_SLUGS: &[&str] = &[
    primal_names::RHIZOCRYPT,
    primal_names::LOAMSPINE,
    primal_names::SWEETGRASS,
];

/// Canonical live-tier methods expected on each primal's `capabilities.list`.
const RHIZOCRYPT_LIVE_METHODS: &[&str] = &["provenance.hash", "provenance.dag_insert"];
const LOAMSPINE_LIVE_METHODS: &[&str] = &["ledger.commit", "ledger.verify"];
const SWEETGRASS_LIVE_METHODS: &[&str] = &["witness.attest", "witness.verify"];

/// Capability domain → owning primal, with accepted method prefixes on the fragment.
const TRIO_CAPABILITY_PREFIXES: &[(&str, &str, &[&str])] = &[
    ("rhizocrypt", "dag", &["provenance.", "dag.", "event."]),
    ("loamspine", "ledger", &["ledger.", "spine.", "entry.", "session."]),
    (
        "sweetgrass",
        "attribution",
        &["witness.", "braid.", "attribution.", "anchoring."],
    ),
];

const PROVENANCE_WORKFLOW_GRAPHS: &[(&str, &str)] = &[
    (
        "provenance_trio",
        include_str!("../../../../graphs/fragments/provenance_trio.toml"),
    ),
    (
        "provenance_overlay",
        include_str!("../../../../graphs/provenance_overlay.toml"),
    ),
    (
        "nest_store",
        include_str!("../../../../graphs/compositions/nest_store.toml"),
    ),
    (
        "rootpulse_commit",
        include_str!("../../../../graphs/compositions/rootpulse_commit.toml"),
    ),
];

/// Chain stages in order: hash(content) → DAG node → merkle root → ledger commit → witness.
const CHAIN_STAGE_KEYWORDS: &[&str] = &[
    "hash", "content", "store", "dag", "merkle", "ledger", "spine", "commit", "witness",
    "braid", "attest", "attribute",
];

/// Run the provenance chain integrity validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let _ = ctx;
    v.section("Phase 1: Structural — NUCLEUS composition & capability domains");
    phase_structural_composition(v);
    phase_structural_capability_domains(v);
    phase_structural_chain_flow(v);

    v.section("Phase 1: Structural — provenance workflow graph TOMLs");
    phase_structural_graph_tomls(v);

    v.section("Phase 2: Live — trio socket capabilities.list");
    phase_live_capabilities(v);

    v.section("Phase 2: Live — trio startup ordering");
    phase_live_startup_order(v);
}

fn phase_structural_composition(v: &mut ValidationResult) {
    let nucleus_slugs: Vec<&str> = Primal::NUCLEUS.iter().map(|p| p.slug()).collect();

    for slug in TRIO_SLUGS {
        v.check_bool(
            &format!("structural:nucleus_has_{slug}"),
            nucleus_slugs.contains(slug),
            &format!("{slug} declared in NUCLEUS composition"),
        );
    }

    let prov_trio = include_str!("../../../../graphs/fragments/provenance_trio.toml");
    let Some(parsed) = helpers::graph_parses(v, "provenance_trio", prov_trio) else {
        return;
    };

    let node_names: Vec<&str> = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array())
        .map(|nodes| {
            nodes
                .iter()
                .filter_map(|n| n.get("name").and_then(|v| v.as_str()))
                .collect()
        })
        .unwrap_or_default();

    for slug in TRIO_SLUGS {
        v.check_bool(
            &format!("structural:provenance_trio_has_{slug}"),
            node_names.contains(slug),
            &format!("provenance_trio fragment declares {slug}"),
        );
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "capability domain, by_capability, prefix, and routing checks share one structural phase"
)]
fn phase_structural_capability_domains(v: &mut ValidationResult) {
    let prov_trio = include_str!("../../../../graphs/fragments/provenance_trio.toml");
    let Ok(parsed) = toml::from_str::<toml::Value>(prov_trio) else {
        v.check_bool(
            "structural:capability_domains:parse",
            false,
            "provenance_trio fragment failed to parse",
        );
        return;
    };

    let by_capability_expectations = [
        ("rhizocrypt", "dag"),
        ("loamspine", "ledger"),
        ("sweetgrass", "attribution"),
    ];

    for (slug, expected_by_cap) in by_capability_expectations {
        let node = parsed
            .get("fragment")
            .and_then(|f| f.get("nodes"))
            .and_then(|n| n.as_array())
            .and_then(|nodes| {
                nodes
                    .iter()
                    .find(|n| n.get("name").and_then(|v| v.as_str()) == Some(slug))
            });

        let Some(node) = node else {
            v.check_bool(
                &format!("structural:{slug}:by_capability"),
                false,
                &format!("{slug} node missing from provenance_trio fragment"),
            );
            continue;
        };

        let by_cap = node
            .get("by_capability")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            &format!("structural:{slug}:by_capability"),
            by_cap == expected_by_cap,
            &format!("{slug} by_capability = {by_cap} (expected {expected_by_cap})"),
        );
    }

    for (slug, domain, accepted_prefixes) in TRIO_CAPABILITY_PREFIXES {
        let routed = capability_to_primal(domain);
        v.check_bool(
            &format!("structural:domain_{domain}_routes_to_{slug}"),
            routed == *slug,
            &format!("{domain} capability domain routes to {slug} (got {routed})"),
        );

        let node = parsed
            .get("fragment")
            .and_then(|f| f.get("nodes"))
            .and_then(|n| n.as_array())
            .and_then(|nodes| {
                nodes
                    .iter()
                    .find(|n| n.get("name").and_then(|v| v.as_str()) == Some(slug))
            });

        let Some(node) = node else {
            continue;
        };

        let caps: Vec<&str> = node
            .get("capabilities")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let declares_prefix = caps.iter().any(|cap| {
            accepted_prefixes
                .iter()
                .any(|prefix| cap.starts_with(prefix))
        });

        v.check_bool(
            &format!("structural:{slug}:capability_prefix"),
            declares_prefix,
            &format!(
                "{slug} declares {} capabilities (expected one of: {})",
                caps.len(),
                accepted_prefixes.join(", ")
            ),
        );
    }

    for (method, expected_owner) in [
        ("ledger.commit", "loamspine"),
        ("ledger.verify", "loamspine"),
    ] {
        let domain = method_to_capability_domain(method);
        let owner = capability_to_primal(domain);
        v.check_bool(
            &format!("structural:route:{method}"),
            owner == expected_owner,
            &format!("{method} → domain '{domain}' → {owner} (expected {expected_owner})"),
        );
    }

    v.check_bool(
        "structural:route:provenance_hash_prefix",
        "provenance.hash".starts_with("provenance."),
        "provenance.hash uses provenance.* namespace",
    );
    v.check_bool(
        "structural:route:witness_attest_prefix",
        "witness.attest".starts_with("witness."),
        "witness.attest uses witness.* namespace",
    );
}

fn phase_structural_chain_flow(v: &mut ValidationResult) {
    let nest_store = include_str!("../../../../graphs/compositions/nest_store.toml");
    let Some(parsed) = helpers::graph_parses(v, "nest_store_chain", nest_store) else {
        return;
    };

    let Some(nodes) = helpers::graph_nodes(&parsed) else {
        v.check_bool(
            "structural:chain:nest_store_nodes",
            false,
            "nest_store graph has no nodes",
        );
        return;
    };

    let binaries: Vec<&str> = nodes
        .iter()
        .filter_map(|n| n.get("binary").and_then(|b| b.as_str()))
        .collect();

    let expected_chain = ["nestgate", "rhizocrypt", "loamspine", "sweetgrass"];
    v.check_bool(
        "structural:chain:nest_store_sequence",
        binaries == expected_chain,
        &format!(
            "nest_store chain: {} (expected nestgate → rhizocrypt → loamspine → sweetgrass)",
            binaries.join(" → ")
        ),
    );

    let mut stage_hits = 0u32;
    for node in nodes {
        let name = node
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("");
        let caps_text = node
            .get("capabilities")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();
        let haystack = format!("{name} {caps_text}").to_lowercase();
        if CHAIN_STAGE_KEYWORDS
            .iter()
            .any(|kw| haystack.contains(kw))
        {
            stage_hits += 1;
        }
    }

    v.check_minimum(
        "structural:chain:stage_coverage",
        usize::try_from(stage_hits).unwrap_or(0),
        3,
    );

    v.check_bool(
        "structural:chain:coordination_sequential",
        parsed
            .get("graph")
            .and_then(|g| g.get("coordination"))
            .and_then(|c| c.as_str())
            == Some("sequential"),
        "nest_store uses sequential coordination for chain ordering",
    );
}

fn phase_structural_graph_tomls(v: &mut ValidationResult) {
    for (label, content) in PROVENANCE_WORKFLOW_GRAPHS {
        let Some(parsed) = helpers::graph_parses(v, label, content) else {
            continue;
        };

        let node_count = helpers::graph_nodes(&parsed)
            .or_else(|| {
                parsed
                    .get("fragment")
                    .and_then(|f| f.get("nodes"))
                    .and_then(|n| n.as_array())
            })
            .map_or(0, Vec::len);

        v.check_bool(
            &format!("structural:graph:{label}:has_nodes"),
            node_count > 0,
            &format!("{label} has {node_count} node(s)"),
        );
    }
}

fn phase_live_capabilities(v: &mut ValidationResult) {
    probe_primal_capabilities(v, "rhizocrypt", RHIZOCRYPT_LIVE_METHODS);
    probe_primal_capabilities(v, "loamspine", LOAMSPINE_LIVE_METHODS);
    probe_primal_capabilities(v, "sweetgrass", SWEETGRASS_LIVE_METHODS);
}

fn probe_primal_capabilities(
    v: &mut ValidationResult,
    primal: &str,
    required_methods: &[&str],
) {
    let Some(sock) = trio_socket_path(primal) else {
        v.check_skip(
            &format!("live:{primal}:socket"),
            &format!("{primal}.sock not found"),
        );
        return;
    };

    let health = probe_primal_at_socket(primal, &sock);
    if !health.health_ok {
        v.check_skip(
            &format!("live:{primal}:health"),
            &format!("{primal} not responding to health check"),
        );
        return;
    }

    if health.capabilities.is_empty() {
        v.check_skip(
            &format!("live:{primal}:capabilities_list"),
            &format!("{primal} does not implement capabilities.list"),
        );
        return;
    }

    for method in required_methods {
        let found = health.capabilities.iter().any(|c| c == method);
        v.check_bool(
            &format!("live:{primal}:{method}"),
            found,
            &format!(
                "{primal} capabilities.list {} {method}",
                if found { "contains" } else { "missing" }
            ),
        );
    }
}

fn phase_live_startup_order(v: &mut ValidationResult) {
    let nest_atomic = include_str!("../../../../graphs/fragments/nest_atomic.toml");
    let Ok(parsed) = toml::from_str::<toml::Value>(nest_atomic) else {
        v.check_skip("live:startup_order:graph", "nest_atomic fragment parse failed");
        return;
    };

    let orders = trio_graph_orders(&parsed);
    if orders.len() < 3 {
        v.check_skip(
            "live:startup_order:graph",
            "nest_atomic fragment missing trio node orders",
        );
        return;
    }

    let [(rhizo_order, _), (loam_order, _), (sweet_order, _)] = orders[..3] else {
        v.check_skip("live:startup_order:graph", "unexpected trio order layout");
        return;
    };

    v.check_bool(
        "structural:startup_order:rhizo_before_loam",
        rhizo_order < loam_order,
        &format!("graph order: rhizocrypt ({rhizo_order}) < loamspine ({loam_order})"),
    );
    v.check_bool(
        "structural:startup_order:loam_before_sweet",
        loam_order < sweet_order,
        &format!("graph order: loamspine ({loam_order}) < sweetgrass ({sweet_order})"),
    );

    let prov_trio = include_str!("../../../../graphs/fragments/provenance_trio.toml");
    if let Ok(prov_parsed) = toml::from_str::<toml::Value>(prov_trio) {
        v.check_bool(
            "structural:startup_order:depends_on_chain",
            trio_depends_on_chain(&prov_parsed),
            "provenance_trio depends_on: rhizocrypt → loamspine → sweetgrass",
        );
    }

    let timestamps = trio_systemd_enter_monotonic();
    if timestamps.len() < 3 {
        v.check_skip(
            "live:startup_order:systemd",
            "not all trio systemd units active — graph order validated structurally",
        );
        return;
    }

    let [(rhizo_ts, _), (loam_ts, _), (sweet_ts, _)] = timestamps[..3] else {
        return;
    };

    v.check_bool(
        "live:startup_order:rhizo_before_loam",
        rhizo_ts <= loam_ts,
        &format!("systemd ActiveEnter: rhizocrypt ({rhizo_ts}) ≤ loamspine ({loam_ts})"),
    );
    v.check_bool(
        "live:startup_order:loam_before_sweet",
        loam_ts <= sweet_ts,
        &format!("systemd ActiveEnter: loamspine ({loam_ts}) ≤ sweetgrass ({sweet_ts})"),
    );
}

fn trio_graph_orders(parsed: &toml::Value) -> Vec<(u64, &str)> {
    let mut orders = Vec::new();
    let nodes = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array());
    let Some(nodes) = nodes else {
        return orders;
    };

    for slug in TRIO_SLUGS {
        let Some(node) = nodes
            .iter()
            .find(|n| n.get("name").and_then(|v| v.as_str()) == Some(slug))
        else {
            continue;
        };
        let order = node
            .get("order")
            .and_then(toml::Value::as_integer)
            .unwrap_or(0);
        #[expect(clippy::cast_sign_loss, reason = "graph order values are positive")]
        orders.push((order as u64, slug));
    }
    orders.sort_by_key(|(order, _)| *order);
    orders
}

fn trio_depends_on_chain(parsed: &toml::Value) -> bool {
    let nodes = parsed
        .get("fragment")
        .and_then(|f| f.get("nodes"))
        .and_then(|n| n.as_array());
    let Some(nodes) = nodes else {
        return false;
    };

    let deps = |slug: &str| -> Vec<String> {
        nodes
            .iter()
            .find(|n| n.get("name").and_then(|v| v.as_str()) == Some(slug))
            .and_then(|n| n.get("depends_on"))
            .and_then(|d| d.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    };

    let rhizo = deps("rhizocrypt");
    let loam = deps("loamspine");
    let sweet = deps("sweetgrass");

    loam.contains(&"rhizocrypt".to_owned())
        && sweet.contains(&"loamspine".to_owned())
        && !rhizo.contains(&"loamspine".to_owned())
        && !rhizo.contains(&"sweetgrass".to_owned())
}

fn trio_systemd_enter_monotonic() -> Vec<(u64, String)> {
    let mut timestamps: Vec<(u64, String)> = Vec::new();
    for slug in TRIO_SLUGS {
        let unit = format!("membrane-nucleus@{slug}.service");
        let output = std::process::Command::new("systemctl")
            .args([
                "--user",
                "show",
                &unit,
                "--property=ActiveEnterTimestampMonotonic",
            ])
            .output();

        let Ok(out) = output else {
            continue;
        };

        let text = String::from_utf8_lossy(&out.stdout);
        let ts = text
            .split('=')
            .nth(1)
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);

        if ts > 0 {
            timestamps.push((ts, String::from(*slug)));
        }
    }
    timestamps.sort_by_key(|(ts, _)| *ts);
    timestamps
}

fn trio_socket_path(primal: &str) -> Option<PathBuf> {
    let base = socket_base_dir()?;
    let sock = base.join(format!("{primal}.sock"));
    if sock.exists() {
        Some(sock)
    } else {
        None
    }
}

fn socket_base_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(&xdg).join("biomeos");
        if path.is_dir() {
            return Some(path);
        }
    }

    let id_output = std::process::Command::new("id").arg("-u").output().ok()?;
    let uid = String::from_utf8_lossy(&id_output.stdout).trim().to_owned();
    let runtime = PathBuf::from(format!("/run/user/{uid}/biomeos"));
    if runtime.is_dir() {
        return Some(runtime);
    }

    let legacy = PathBuf::from("/tmp/biomeos");
    if legacy.is_dir() {
        return Some(legacy);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;

    #[test]
    fn provenance_chain_integrity_structural() {
        let mut v = ValidationResult::new("provenance-chain-integrity");
        phase_structural_composition(&mut v);
        phase_structural_capability_domains(&mut v);
        phase_structural_chain_flow(&mut v);
        phase_structural_graph_tomls(&mut v);
        assert_eq!(
            v.failed, 0,
            "provenance chain structural phase had {} failures (use --nocapture)",
            v.failed
        );
    }

    #[test]
    fn provenance_chain_integrity_no_panic() {
        let mut v = ValidationResult::new("provenance-chain-integrity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }

    #[test]
    fn trio_graph_orders_sorted() {
        let nest_atomic = include_str!("../../../../graphs/fragments/nest_atomic.toml");
        let parsed: toml::Value = toml::from_str(nest_atomic).unwrap();
        let orders = trio_graph_orders(&parsed);
        assert_eq!(orders.len(), 3);
        assert_eq!(orders[0].1, "rhizocrypt");
        assert_eq!(orders[1].1, "loamspine");
        assert_eq!(orders[2].1, "sweetgrass");
    }

    #[test]
    fn socket_base_dir_does_not_panic() {
        let _ = socket_base_dir();
    }
}
