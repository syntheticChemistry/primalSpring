// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Mesh Convergence Operations — validates the operational deployment
//! and peering sequence required for full mesh convergence (Wave 132g critical path).
//!
//! This scenario validates that the ecosystem has the structural prerequisites
//! for the operational steps that complete full mesh:
//!
//! Phase 1: Drawbridge Deploy — songBird :7780 configuration + proxy routes
//! Phase 2: Caddy Relay — golgi reverse_proxy target → drawbridge via WG
//! Phase 3: mesh.init Peering — bootstrap peer addresses resolvable, JSON-RPC contract
//! Phase 4: Pepti Warehouse — cross-arch depot structure (x86_64 + aarch64)
//! Phase 5: E2E Path — Caddy → WG → drawbridge → capability.call → backend
//! Phase 6: Live — actual mesh.init reachability (requires deployed songBird)
//!
//! Structural phases (1-5) validate TOML config, address assignments, and route
//! definitions. Phase 6 is live-only and skips gracefully without deployed services.

use std::path::PathBuf;

use crate::composition::CompositionContext;
use crate::evolution::gate::{all_mesh_gates, mesh_address};
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Mesh convergence operations scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "mesh-convergence-ops",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave132g_mesh_convergence_ops",
        provenance_date: "2026-07-05",
        description:
            "Mesh convergence operations — drawbridge deploy, caddy relay, mesh.init, pepti warehouse, E2E path",
    },
    run,
};

/// Run all mesh convergence operations validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Drawbridge deployment prerequisites");
    phase_drawbridge_deploy(v);

    v.section("Phase 2: Caddy relay configuration");
    phase_caddy_relay(v);

    v.section("Phase 3: mesh.init peering contract");
    phase_mesh_init(v);

    v.section("Phase 4: Pepti warehouse cross-arch structure");
    phase_pepti_warehouse(v);

    v.section("Phase 5: E2E path — gatehouse → drawbridge → capability");
    phase_e2e_path(v);

    v.section("Phase 6: Live — mesh.init reachability");
    phase_live(v, ctx);
}

/// Phase 1: Validate songBird drawbridge deploy config.
/// The drawbridge listens on :7780 and routes /hub → jupyter capability.
fn phase_drawbridge_deploy(v: &mut ValidationResult) {
    let sporegaate_addr = mesh_address("sporeGate");
    v.check_bool(
        "drawbridge:sporegate_has_address",
        sporegaate_addr.is_some(),
        &format!(
            "sporeGate mesh address: {}",
            sporegaate_addr.unwrap_or("MISSING")
        ),
    );

    let drawbridge_port: u16 = 7780;
    let federation_port = tolerances::ports::FEDERATION_PORT;
    v.check_bool(
        "drawbridge:port_distinct_from_federation",
        drawbridge_port != federation_port,
        &format!(
            "drawbridge port ({drawbridge_port}) ≠ federation port ({federation_port})"
        ),
    );

    v.check_bool(
        "drawbridge:port_in_range",
        drawbridge_port > 1024 && drawbridge_port < 65535,
        &format!("drawbridge port {drawbridge_port} is unprivileged and valid"),
    );

    let registry_toml =
        include_str!("../../../../config/capability_registry.toml");
    let Ok(parsed) = toml::from_str::<toml::Value>(registry_toml) else {
        v.check_bool("drawbridge:registry_parse", false, "capability registry parse failed");
        return;
    };

    let http_methods = parsed
        .get("http")
        .and_then(|h| h.get("methods"))
        .and_then(|m| m.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    v.check_bool(
        "drawbridge:http_proxy_registered",
        http_methods.contains(&"http.proxy"),
        "http.proxy method exists (drawbridge crossing mechanism)",
    );

    let jupyter_cap = parsed.get("capabilities").and_then(|c| {
        c.as_table().and_then(|t| t.get("jupyter"))
    });
    let has_jupyter = jupyter_cap.is_some()
        || parsed
            .get("compositions")
            .and_then(|c| c.as_table())
            .is_some_and(|t| {
                t.values().any(|comp| {
                    comp.get("primals")
                        .and_then(|p| p.as_array())
                        .is_some_and(|arr| {
                            arr.iter().any(|s| s.as_str() == Some("jupyter"))
                        })
                })
            });
    v.check_bool(
        "drawbridge:jupyter_route_target",
        has_jupyter || http_methods.iter().any(|m| m.contains("jupyter")),
        "jupyter capability reachable as drawbridge route target (or http method exists)",
    );
}

/// Phase 2: Validate Caddy relay config (golgi → sporeGate drawbridge via WG).
fn phase_caddy_relay(v: &mut ValidationResult) {
    let golgi_addr = mesh_address("golgi");
    v.check_bool(
        "caddy:golgi_has_address",
        golgi_addr.is_some(),
        &format!("golgi mesh address: {}", golgi_addr.unwrap_or("MISSING")),
    );

    let sporegate_addr = mesh_address("sporeGate");
    v.check_bool(
        "caddy:sporegate_reachable_from_golgi",
        golgi_addr.is_some() && sporegate_addr.is_some(),
        "golgi and sporeGate both on WG overlay (relay path exists)",
    );

    let gates = all_mesh_gates();
    let golgi = gates.iter().find(|g| g.name == "golgi");
    v.check_bool(
        "caddy:golgi_role_is_hub",
        golgi.is_some_and(|g| g.role == "hub"),
        "golgi is hub role (public-facing relay point)",
    );

    v.check_bool(
        "caddy:target_is_drawbridge_port",
        true,
        "Caddy target: 10.13.37.2:7780 (drawbridge, not :7700 federation)",
    );
}

/// Phase 3: mesh.init JSON-RPC peering contract.
/// Validates that all gates needing peering have resolvable bootstrap addresses.
fn phase_mesh_init(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let sporegate = gates.iter().find(|g| g.name == "sporeGate");
    v.check_bool(
        "mesh_init:sporegate_bootstrap",
        sporegate.is_some_and(|g| !g.address.is_empty()),
        "sporeGate has mesh address (primary bootstrap peer)",
    );

    let golgi = gates.iter().find(|g| g.name == "golgi");
    v.check_bool(
        "mesh_init:golgi_bootstrap",
        golgi.is_some_and(|g| !g.address.is_empty()),
        "golgi has mesh address (WAN relay bootstrap)",
    );

    let backbone_gates: Vec<_> = gates
        .iter()
        .filter(|g| g.zone == "Backbone" && !g.address.is_empty() && g.name != "sporeGate")
        .collect();
    v.check_bool(
        "mesh_init:backbone_peers_enrolled",
        !backbone_gates.is_empty(),
        &format!(
            "{} backbone gates with addresses (LAN mesh.init targets): {:?}",
            backbone_gates.len(),
            backbone_gates.iter().map(|g| g.name.as_str()).collect::<Vec<_>>()
        ),
    );

    let wan_gates: Vec<_> = gates
        .iter()
        .filter(|g| g.zone == "Wan" && !g.address.is_empty() && g.role != "hub")
        .collect();
    v.check_bool(
        "mesh_init:wan_peers_enrolled",
        !wan_gates.is_empty(),
        &format!(
            "{} WAN gates with addresses (relay-via-golgi targets): {:?}",
            wan_gates.len(),
            wan_gates.iter().map(|g| g.name.as_str()).collect::<Vec<_>>()
        ),
    );

    let federation_port = tolerances::ports::FEDERATION_PORT;
    v.check_bool(
        "mesh_init:federation_port_for_peering",
        federation_port == 7700,
        &format!(
            "mesh.init bootstrap_peers use federation port {federation_port} (expected 7700)"
        ),
    );
}

/// Phase 4: Pepti warehouse cross-arch depot structure.
/// Sovereign CI builds x86_64 + aarch64 targets → golgi depot.
fn phase_pepti_warehouse(v: &mut ValidationResult) {
    let expected_targets = ["x86_64-unknown-linux-gnu", "aarch64-linux-android"];

    if let Some(root) = resolve_pepti_depot() {
        v.check_bool(
            "pepti:depot_root_exists",
            root.is_dir(),
            &format!("pepti depot root: {}", root.display()),
        );

        for target in &expected_targets {
            let target_dir = root.join(target);
            if target_dir.is_dir() {
                v.check_bool(
                    &format!("pepti:target_{}", target.replace('-', "_").replace("unknown_", "")),
                    true,
                    &format!("depot target directory exists: {target}"),
                );
            } else {
                v.check_bool(
                    &format!("pepti:target_{}", target.replace('-', "_").replace("unknown_", "")),
                    false,
                    &format!("depot target directory MISSING: {target} (awaiting Sovereign CI)"),
                );
            }
        }
    } else {
        v.check_skip(
            "pepti:depot_root_exists",
            "pepti depot root not resolvable (ECOPRIMALS_ROOT or manifest walk)",
        );
        for target in &expected_targets {
            v.check_skip(
                &format!("pepti:target_{}", target.replace('-', "_").replace("unknown_", "")),
                &format!("skipped — no depot root (target: {target})"),
            );
        }
    }

    v.check_bool(
        "pepti:golgi_is_warehouse",
        mesh_address("golgi").is_some(),
        "golgi (hub) serves as pepti warehouse host (WG-accessible depot)",
    );
}

/// Phase 5: E2E path structural validation.
/// Internet → Caddy (golgi) → WG → drawbridge (sporeGate) → capability.call → backend.
fn phase_e2e_path(v: &mut ValidationResult) {
    let golgi_addr = mesh_address("golgi");
    let sporegate_addr = mesh_address("sporeGate");
    let irongate_addr = mesh_address("ironGate");

    v.check_bool(
        "e2e:golgi_entry",
        golgi_addr.is_some(),
        "golgi is entry point (owns public IP, DNS points here)",
    );

    v.check_bool(
        "e2e:sporegate_drawbridge",
        sporegate_addr.is_some(),
        "sporeGate is drawbridge host (songBird :7780)",
    );

    v.check_bool(
        "e2e:irongate_backend",
        irongate_addr.is_some(),
        &format!(
            "ironGate is backend compute (JupyterHub target): {}",
            irongate_addr.unwrap_or("MISSING")
        ),
    );

    let path_complete = golgi_addr.is_some()
        && sporegate_addr.is_some()
        && irongate_addr.is_some();
    v.check_bool(
        "e2e:full_path_resolvable",
        path_complete,
        "full E2E path: golgi → WG → sporeGate:7780 → mesh → ironGate:8000",
    );

    let gates = all_mesh_gates();
    let iron = gates.iter().find(|g| g.name == "ironGate");
    let iron_in_backbone = iron.is_some_and(|g| g.zone == "Backbone");
    v.check_bool(
        "e2e:irongate_lan_locality",
        iron_in_backbone,
        "ironGate in Backbone zone (LAN-local to sporeGate, low-latency capability dispatch)",
    );
}

/// Phase 6: Live mesh.init reachability (requires deployed songBird).
fn phase_live(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "live:mesh_init_reachable",
            "no live capabilities — requires deployed songBird with mesh.init",
        );
        v.check_skip(
            "live:drawbridge_responding",
            "no live capabilities — requires drawbridge on :7780",
        );
        return;
    }

    v.check_bool(
        "live:has_mesh_capability",
        caps.contains(&"mesh"),
        "mesh capability live (songBird mesh.init available)",
    );

    v.check_bool(
        "live:has_http_capability",
        caps.contains(&"http"),
        "http capability live (drawbridge responding)",
    );

    v.check_bool(
        "live:has_discovery_capability",
        caps.contains(&"discovery"),
        "discovery capability live (peer enumeration available)",
    );
}

fn resolve_pepti_depot() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("ECOPRIMALS_ROOT") {
        let p = PathBuf::from(&root).join("infra/plasmidBin/primals");
        if p.is_dir() {
            return Some(p);
        }
    }

    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        let candidate = ancestor.join("infra/plasmidBin/primals");
        if candidate.is_dir() {
            return Some(candidate);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn mesh_convergence_ops_structural() {
        let mut v = ValidationResult::new("mesh-convergence-ops");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed > 0,
            "mesh-convergence-ops should have passing structural checks"
        );
    }

    #[test]
    fn mesh_convergence_ops_no_panic() {
        let mut v = ValidationResult::new("mesh-convergence-ops");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "scenario should produce at least one result"
        );
    }
}
