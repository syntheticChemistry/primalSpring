// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Exceed Exploration.
//!
//! Validates structural readiness for 6 domains where Tower Atomic can
//! exceed `WireGuard`. Phase 1 achieved parity (150w). Phase 2 explores
//! specialization advantages:
//!
//! 1. Capability-aware routing (per-capability latency vs single tunnel)
//! 2. Multi-stack routing (N `songBird` instances, per-purpose tuning)
//! 3. Large data transfer (CAS dedup, blob routing)
//! 4. Secure compute mesh (per-session keys vs tunnel crypto)
//! 5. Distributed compute (cross-gate dispatch, aggregation)
//! 6. Edge/SFF profile (minimal relay on constrained hardware)
//!
//! Tower is a *specialized* capability-routed mesh. `WireGuard` is a
//! general-purpose kernel VPN. The specialization opens these domains.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-exceed-exploration",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_exceed",
        provenance_date: "2026-07-23",
        description: "Tower Exceed — 6 domains where Tower Atomic surpasses WireGuard (Phase 2 exploration)",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Domain 1: Capability-aware routing");
    domain_capability_routing(v);

    v.section("Domain 2: Multi-stack routing");
    domain_multi_stack(v);

    v.section("Domain 3: Large data transfer");
    domain_large_data(v);

    v.section("Domain 4: Secure compute mesh");
    domain_secure_compute(v);

    v.section("Domain 5: Distributed compute");
    domain_distributed_compute(v);

    v.section("Domain 6: Edge/SFF profile");
    domain_edge_profile(v);
}

fn domain_capability_routing(v: &mut ValidationResult) {
    let has_capability_call = REGISTRY_TOML.contains("capability.call");
    v.check_bool(
        "exceed:capability_call",
        has_capability_call,
        "capability.call: per-capability dispatch (vs WG single tunnel for all traffic)",
    );

    let has_capabilities_query = REGISTRY_TOML.contains("mesh.capabilities_query");
    v.check_bool(
        "exceed:capability_query",
        has_capabilities_query,
        "mesh.capabilities_query: runtime routing table (Tower knows WHAT traffic is)",
    );

    let has_capabilities_announce = REGISTRY_TOML.contains("mesh.capabilities_announce");
    v.check_bool(
        "exceed:capability_announce",
        has_capabilities_announce,
        "mesh.capabilities_announce: peers advertise capabilities (WG has no content awareness)",
    );

    let has_composition_tier = REGISTRY_TOML.contains("[compositions.");
    v.check_bool(
        "exceed:composition_routing",
        has_composition_tier,
        "Composition tiers defined: compound operations route via optimal graph path (vs WG dumb pipe)",
    );
}

fn domain_multi_stack(v: &mut ValidationResult) {
    let has_relay = REGISTRY_TOML.contains("mesh.relay");
    v.check_bool(
        "exceed:relay_base",
        has_relay,
        "mesh.relay: base relay capability (N instances = N traffic classes)",
    );

    let has_mesh_mirror = REGISTRY_TOML.contains("mesh.mirror");
    v.check_bool(
        "exceed:mirror_support",
        has_mesh_mirror,
        "mesh.mirror: traffic mirroring for shadow/benchmark (WG cannot split by purpose)",
    );

    let golgi_is_hub = MESH_TOML.contains("10.13.37.1");
    v.check_bool(
        "exceed:multi_stack_hub",
        golgi_is_hub,
        "golgiBody (.1) present: multi-stack target (RPC + blob + relay profiles)",
    );
}

fn domain_large_data(v: &mut ValidationResult) {
    let has_cas = REGISTRY_TOML.contains("cas.") || REGISTRY_TOML.contains("content.");
    v.check_bool(
        "exceed:cas_routing",
        has_cas,
        "CAS/content methods: content-addressed blob routing (dedup = skip redundant transfer)",
    );

    let has_storage = REGISTRY_TOML.contains("storage.") || REGISTRY_TOML.contains("nest.");
    v.check_bool(
        "exceed:storage_layer",
        has_storage,
        "Storage layer: nestGate CAS for nearest-copy routing (WG just pipes bytes)",
    );

    let has_mesh_publish = REGISTRY_TOML.contains("mesh.publish");
    v.check_bool(
        "exceed:blob_relay",
        has_mesh_publish,
        "mesh.publish: data relay through Tower (negotiable framing vs WG fixed MTU 1420)",
    );
}

fn domain_secure_compute(v: &mut ValidationResult) {
    let has_btsp_negotiate = REGISTRY_TOML.contains("btsp.negotiate");
    v.check_bool(
        "exceed:per_session_crypto",
        has_btsp_negotiate,
        "btsp.negotiate: per-capability crypto policy (vs WG tunnel-level encryption for all)",
    );

    let has_btsp_handshake = REGISTRY_TOML.contains("btsp.handshake");
    v.check_bool(
        "exceed:session_keys",
        has_btsp_handshake,
        "btsp.handshake: per-session HKDF key derivation (PostPrimordial = strongest per-flow)",
    );

    let has_credential = REGISTRY_TOML.contains("secrets.") || REGISTRY_TOML.contains("credential");
    v.check_bool(
        "exceed:credential_store",
        has_credential,
        "CredentialStore: hardware-backed key material (vs WG static privkey file)",
    );
}

fn domain_distributed_compute(v: &mut ValidationResult) {
    let has_compute = REGISTRY_TOML.contains("compute.") || REGISTRY_TOML.contains("dispatch.");
    v.check_bool(
        "exceed:compute_dispatch",
        has_compute,
        "Compute dispatch methods: workloads route to right substrate (Tower is compute-aware)",
    );

    let has_mesh_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "exceed:topology_aware_dispatch",
        has_mesh_find_path,
        "mesh.find_path: topology-aware routing for cross-gate GPU/NPU dispatch",
    );

    let multi_zone =
        MESH_TOML.contains("zone = \"Backbone\"") && MESH_TOML.contains("zone = \"House2\"");
    v.check_bool(
        "exceed:multi_zone_compute",
        multi_zone,
        "Multi-zone topology: backbone + house2 (cross-zone dispatch targets exist)",
    );
}

fn domain_edge_profile(v: &mut ValidationResult) {
    let has_mesh_connect = REGISTRY_TOML.contains("mesh.connect");
    v.check_bool(
        "exceed:minimal_connect",
        has_mesh_connect,
        "mesh.connect: minimal relay profile (tunable overhead for Celeron/NUC hardware)",
    );

    let has_mesh_announce = REGISTRY_TOML.contains("mesh.announce");
    v.check_bool(
        "exceed:lightweight_announce",
        has_mesh_announce,
        "mesh.announce: lightweight peer advertisement (WG same overhead on Celeron as EPYC)",
    );

    let gate_count = MESH_TOML.matches("[[gate]]").count();
    v.check_bool(
        "exceed:edge_targets_exist",
        gate_count >= 6,
        &format!("{gate_count} gates in topology: edge profile targets available (NUC, mobile)"),
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
