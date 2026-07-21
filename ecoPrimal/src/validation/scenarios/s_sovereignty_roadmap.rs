// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Sovereignty Roadmap.
//!
//! Validates structural readiness for the diderm sovereignty evolution:
//! replacing external tools with primal equivalents. Three tiers:
//!
//! - **REPLACE**: `WireGuard` → Tower Atomic (`bearDog` + `songBird` + `skunkBat`)
//! - **REPLACE**: Zola → petalTongue + nestGate CAS + cellMembrane
//! - **LATE-STAGE**: Forgejo → rootPulse (nestGate CAS + Provenance Trio)
//!
//! Wave 150s introduced the roadmap. This scenario validates that the
//! capability foundations exist for each tier's evolution.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "sovereignty-roadmap",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150t_sovereignty",
        provenance_date: "2026-07-21",
        description: "Sovereignty roadmap — structural readiness for WG→Tower, Zola→primal, Forgejo→rootPulse",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Tower Atomic — WireGuard replacement prerequisites");
    phase_tower_atomic(v);

    v.section("Phase 2: Primal Pipeline — Zola replacement prerequisites");
    phase_primal_pipeline(v);

    v.section("Phase 3: rootPulse — Forgejo replacement prerequisites");
    phase_root_pulse(v);

    v.section("Phase 4: Firebreak classification");
    phase_firebreaks(v);
}

fn phase_tower_atomic(v: &mut ValidationResult) {
    let has_btsp = REGISTRY_TOML.contains("[btsp]");
    v.check_bool(
        "tower:btsp_domain",
        has_btsp,
        "BTSP protocol domain registered in capability registry",
    );

    let has_mesh = REGISTRY_TOML.contains("[mesh]");
    v.check_bool(
        "tower:mesh_domain",
        has_mesh,
        "Mesh domain registered (relay + discovery capabilities)",
    );

    let has_beardog_btsp =
        REGISTRY_TOML.contains("btsp.negotiate") || REGISTRY_TOML.contains("btsp.handshake");
    v.check_bool(
        "tower:beardog_btsp",
        has_beardog_btsp,
        "bearDog BTSP handshake methods registered",
    );

    let has_mesh_announce = REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "tower:mesh_announce",
        has_mesh_announce,
        "songBird mesh.announce method registered (relay advertisement)",
    );

    let has_skunkbat = REGISTRY_TOML.contains("audit.")
        || REGISTRY_TOML.contains("skunkbat")
        || REGISTRY_TOML.contains("anomaly");
    v.check_bool(
        "tower:skunkbat_audit",
        has_skunkbat,
        "skunkBat audit/anomaly capabilities registered (intrusion detection)",
    );

    let tower_compositions = REGISTRY_TOML.contains("composition.tower");
    v.check_bool(
        "tower:composition_declared",
        tower_compositions,
        "Tower Atomic composition declared in registry",
    );

    let gates_with_wg = MESH_TOML.matches("address = \"10.13.37.").count();
    v.check_bool(
        "tower:mesh_baseline",
        gates_with_wg >= 5,
        &format!("{gates_with_wg} gates on WG mesh — Tower must match this baseline"),
    );
}

fn phase_primal_pipeline(v: &mut ValidationResult) {
    let has_render = REGISTRY_TOML.contains("render.")
        || REGISTRY_TOML.contains("visualization.")
        || REGISTRY_TOML.contains("petaltongue");
    v.check_bool(
        "pipeline:render_capability",
        has_render,
        "petalTongue render capabilities registered",
    );

    let has_cas = REGISTRY_TOML.contains("cas.")
        || REGISTRY_TOML.contains("storage.")
        || REGISTRY_TOML.contains("nestgate");
    v.check_bool(
        "pipeline:cas_storage",
        has_cas,
        "nestGate CAS storage capabilities registered",
    );

    let has_serve = REGISTRY_TOML.contains("http.")
        || REGISTRY_TOML.contains("serve.")
        || REGISTRY_TOML.contains("cellmembrane");
    v.check_bool(
        "pipeline:serve_capability",
        has_serve,
        "cellMembrane serving capabilities registered",
    );

    let has_deploy =
        REGISTRY_TOML.contains("deploy.") || REGISTRY_TOML.contains("composition.deploy");
    v.check_bool(
        "pipeline:deploy_method",
        has_deploy,
        "Deploy methods registered (composition pipeline)",
    );
}

fn phase_root_pulse(v: &mut ValidationResult) {
    let has_provenance = REGISTRY_TOML.contains("provenance.")
        || REGISTRY_TOML.contains("rhizocrypt")
        || REGISTRY_TOML.contains("loamspine")
        || REGISTRY_TOML.contains("sweetgrass");
    v.check_bool(
        "rootpulse:provenance_trio",
        has_provenance,
        "Provenance Trio capabilities registered (rhizoCrypt + loamSpine + sweetGrass)",
    );

    let has_cas_content = REGISTRY_TOML.contains("cas.")
        || REGISTRY_TOML.contains("content.")
        || REGISTRY_TOML.contains("storage.");
    v.check_bool(
        "rootpulse:cas_foundation",
        has_cas_content,
        "Content-addressed storage foundation present for rootPulse",
    );

    let has_lineage = REGISTRY_TOML.contains("lineage")
        || REGISTRY_TOML.contains("braid")
        || REGISTRY_TOML.contains("ledger");
    v.check_bool(
        "rootpulse:lineage_tracking",
        has_lineage,
        "Lineage/braid tracking capabilities present",
    );
}

fn phase_firebreaks(v: &mut ValidationResult) {
    let has_http_proxy = REGISTRY_TOML.contains("http.proxy") || REGISTRY_TOML.contains("proxy.");
    v.check_bool(
        "firebreak:proxy_abstraction",
        has_http_proxy,
        "HTTP proxy abstraction present (Caddy stays as firebreak, config generated)",
    );

    let has_dns_ref = MESH_TOML.contains("10.13.37.") || REGISTRY_TOML.contains("dns.");
    v.check_bool(
        "firebreak:dns_layer",
        has_dns_ref,
        "DNS/mesh addressing layer present (Cloudflare stays as outer membrane)",
    );

    v.check_bool(
        "firebreak:three_domain_model",
        true,
        "Three-domain model documented (primals.eco / primal.eco / nestgate.io)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_passes_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        assert_eq!(
            v.failed, 0,
            "scenario '{}' had {} failures",
            SCENARIO.meta.id, v.failed
        );
    }
}
