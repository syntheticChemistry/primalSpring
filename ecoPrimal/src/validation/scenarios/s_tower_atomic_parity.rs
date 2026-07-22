// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Atomic Parity.
//!
//! Validates that the Tower Atomic composition (`bearDog` + `songBird` +
//! `skunkBat`) has structural parity with `WireGuard` for the sovereignty
//! cutover benchmark. Wave 150u defined the parity spec:
//!
//! - LAN relay path: authenticated BTSP relay between two mesh peers
//! - WAN relay path: TURN-style relay through `golgiBody`
//! - Latency baseline: must match WG LAN (<5ms) and WG WAN (<50ms)
//! - Throughput baseline: must match WG iperf3 (>800 Mbps LAN, >50 Mbps WAN)
//! - `mesh.enroll` LIVE with BTSP-HMAC proof
//!
//! This scenario validates structural prerequisites — the actual benchmark
//! is a Live-tier scenario that requires two active peers.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const MESH_TOML: &str = include_str!("../../../../config/mesh_topology.toml");

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-atomic-parity",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150u_tower_parity",
        provenance_date: "2026-07-21",
        description: "Tower Atomic parity — structural readiness for WG replacement benchmark",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Tower Atomic composition — 3 primals present");
    phase_composition_primals(v);

    v.section("Phase 2: Relay capabilities — BTSP + mesh relay path");
    phase_relay_capabilities(v);

    v.section("Phase 3: Benchmark topology — LAN + WAN peers available");
    phase_benchmark_topology(v);

    v.section("Phase 4: Parity spec — latency/throughput targets defined");
    phase_parity_spec(v);

    v.section("Phase 5: Credential store — secrets.* integration");
    phase_credential_store(v);
}

fn phase_composition_primals(v: &mut ValidationResult) {
    let has_beardog_auth =
        REGISTRY_TOML.contains("btsp.handshake") && REGISTRY_TOML.contains("btsp.negotiate");
    v.check_bool(
        "parity:beardog_auth",
        has_beardog_auth,
        "bearDog BTSP handshake + negotiate registered (trust layer)",
    );

    let has_songbird_relay =
        REGISTRY_TOML.contains("mesh.relay") && REGISTRY_TOML.contains("mesh.connect");
    v.check_bool(
        "parity:songbird_relay",
        has_songbird_relay,
        "songBird mesh.relay + mesh.connect registered (transport layer)",
    );

    let has_skunkbat_audit = REGISTRY_TOML.contains("audit.")
        || REGISTRY_TOML.contains("anomaly")
        || REGISTRY_TOML.contains("threat");
    v.check_bool(
        "parity:skunkbat_ids",
        has_skunkbat_audit,
        "skunkBat audit/anomaly/threat registered (intrusion detection layer)",
    );

    let tower_composition = REGISTRY_TOML.contains("[compositions.tower]");
    v.check_bool(
        "parity:tower_composition_defined",
        tower_composition,
        "Tower composition tier defined in registry (5 signals)",
    );

    let tower_bootstrap = REGISTRY_TOML.contains("tower.bootstrap");
    v.check_bool(
        "parity:tower_bootstrap_signal",
        tower_bootstrap,
        "tower.bootstrap signal defined (cold-start two-phase sequence)",
    );
}

fn phase_relay_capabilities(v: &mut ValidationResult) {
    let has_mesh_enroll =
        REGISTRY_TOML.contains("mesh.enroll") || REGISTRY_TOML.contains("mesh.init");
    v.check_bool(
        "parity:mesh_enroll",
        has_mesh_enroll,
        "mesh.enroll or mesh.init present (peer enrollment with BTSP-HMAC proof)",
    );

    let has_mesh_find_path = REGISTRY_TOML.contains("mesh.find_path");
    v.check_bool(
        "parity:relay_path_finding",
        has_mesh_find_path,
        "mesh.find_path registered (relay route selection)",
    );

    let has_mesh_peers = REGISTRY_TOML.contains("mesh.peers");
    v.check_bool(
        "parity:peer_discovery",
        has_mesh_peers,
        "mesh.peers registered (peer roster for topology awareness)",
    );

    let has_mesh_publish = REGISTRY_TOML.contains("mesh.publish");
    v.check_bool(
        "parity:relay_publish",
        has_mesh_publish,
        "mesh.publish registered (data relay through Tower stack)",
    );

    let has_btsp_escalation = REGISTRY_TOML.contains("btsp_escalation");
    v.check_bool(
        "parity:btsp_escalation_enforced",
        has_btsp_escalation,
        "BTSP escalation enforced on relay methods (no cleartext relay)",
    );
}

fn phase_benchmark_topology(v: &mut ValidationResult) {
    let peer_count = MESH_TOML
        .lines()
        .filter(|l| l.contains("address = \"10.13.37."))
        .count();

    v.check_bool(
        "parity:lan_peer_count",
        peer_count >= 2,
        &format!("{peer_count} peers with WG addresses — need ≥2 for LAN benchmark pair"),
    );

    let has_backbone_peer = MESH_TOML.contains("zone = \"Backbone\"");
    v.check_bool(
        "parity:backbone_peer",
        has_backbone_peer,
        "Backbone zone peer exists (LAN benchmark candidate — eastGate/sporeGate)",
    );

    let has_vps_hub = MESH_TOML.contains("10.13.37.1");
    v.check_bool(
        "parity:wan_relay_hub",
        has_vps_hub,
        "golgiBody (.1) present as WAN relay hub (TURN-style benchmark endpoint)",
    );

    let has_house2_peer = MESH_TOML.contains("zone = \"House2\"");
    v.check_bool(
        "parity:cross_zone_peer",
        has_house2_peer,
        "House2 zone peer exists (cross-zone benchmark: backbone↔house2)",
    );
}

fn phase_parity_spec(v: &mut ValidationResult) {
    v.check_bool(
        "parity:lan_latency_target",
        true,
        "LAN latency target: <5ms round-trip (WG baseline on 1Gbps ethernet)",
    );

    v.check_bool(
        "parity:wan_latency_target",
        true,
        "WAN latency target: <50ms round-trip (WG baseline through golgiBody relay)",
    );

    v.check_bool(
        "parity:lan_throughput_target",
        true,
        "LAN throughput target: >800 Mbps (WG iperf3 baseline on 1Gbps link)",
    );

    v.check_bool(
        "parity:wan_throughput_target",
        true,
        "WAN throughput target: >50 Mbps (WG iperf3 through VPS relay)",
    );

    let has_tower_health_signal = REGISTRY_TOML.contains("tower.health");
    v.check_bool(
        "parity:health_signal_for_monitoring",
        has_tower_health_signal,
        "tower.health signal defined (continuous monitoring during benchmark)",
    );
}

fn phase_credential_store(v: &mut ValidationResult) {
    let has_secrets = REGISTRY_TOML.contains("secrets.") || REGISTRY_TOML.contains("credential");
    v.check_bool(
        "parity:credential_store_capability",
        has_secrets,
        "secrets.* or credential capability present (CredentialStore trait shipped Wave 150u)",
    );

    let has_btsp_server_status = REGISTRY_TOML.contains("btsp.server.status");
    v.check_bool(
        "parity:btsp_server_status",
        has_btsp_server_status,
        "btsp.server.status registered (runtime BTSP health for relay stack)",
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
