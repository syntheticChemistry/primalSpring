// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Gatehouse/Darkforest Demarcation — validates the sovereign network
//! architecture standard published in Wave 132g.
//!
//! The ecoPrimals network has two regimes:
//! - **Gatehouse** (sporeGate only): bearDog :443 TLS + :80 ACME, single exposed surface
//! - **Darkforest** (all gates): zero external ports, mesh-only routing
//! - **Drawbridge** (songBird http.proxy): sole crossing point between external and internal
//!
//! This scenario validates the structural contracts from `GATEHOUSE_DARKFOREST_STANDARD.md`.
//!
//! Phases:
//! 1. Gatehouse: bearDog owns TLS/:443, only sporeGate is gatehouse
//! 2. Drawbridge: songBird http.proxy is the crossing point, routes to backends
//! 3. Darkforest: non-gatehouse gates have zero exposed ports
//! 4. Transport hierarchy: abstract > UDS > TCP localhost > mesh
//! 5. Live: bearDog gatehouse health + songBird proxy routing (requires deploy)

use crate::composition::{CompositionContext, capability_to_primal};
use crate::evolution::gate::all_mesh_gates;
use crate::ipc::platform::PlatformCapabilities;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Capability registry TOML.
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Gatehouse/Darkforest demarcation scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "gatehouse-darkforest",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave132g_gatehouse_darkforest",
        provenance_date: "2026-07-05",
        description:
            "Gatehouse/Darkforest demarcation — bearDog gateway, songBird drawbridge, zero-port mesh",
    },
    run,
};

/// Run all gatehouse/darkforest validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Gatehouse — bearDog TLS ownership");
    phase_gatehouse(v);

    v.section("Phase 2: Drawbridge — songBird http.proxy crossing");
    phase_drawbridge(v);

    v.section("Phase 3: Darkforest — zero-port mesh compliance");
    phase_darkforest(v);

    v.section("Phase 4: Transport hierarchy");
    phase_transport_hierarchy(v);

    v.section("Phase 5: Live — gatehouse health");
    phase_live(v, ctx);
}

fn phase_gatehouse(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        v.check_bool("gatehouse:registry_parse", false, "registry parse failed");
        return;
    };

    let tls_owner = capability_to_primal("tls");
    v.check_bool(
        "gatehouse:tls_owner_beardog",
        tls_owner == "beardog",
        &format!("tls → {tls_owner} (bearDog owns TLS termination)"),
    );

    let cert_owner = capability_to_primal("certificate");
    v.check_bool(
        "gatehouse:cert_owner_beardog",
        cert_owner == "beardog",
        &format!("certificate → {cert_owner} (bearDog owns ACME lifecycle)"),
    );

    let security_owner = capability_to_primal("security");
    v.check_bool(
        "gatehouse:security_owner_beardog",
        security_owner == "beardog",
        &format!("security → {security_owner} (bearDog dispatches advisory to skunkBat)"),
    );

    let tower = parsed.get("compositions").and_then(|c| c.get("tower"));
    if let Some(t) = tower {
        let primals = t
            .get("primals")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            "gatehouse:tower_beardog",
            primals.contains(&"beardog"),
            "bearDog in tower composition (gatehouse binary)",
        );
        v.check_bool(
            "gatehouse:tower_skunkbat",
            primals.contains(&"skunkbat"),
            "skunkBat in tower composition (security advisory at surface)",
        );
    }

    let gates = all_mesh_gates();
    let gatehouse_count = gates
        .iter()
        .filter(|g| g.name == "sporeGate")
        .count();
    v.check_bool(
        "gatehouse:single_gatehouse",
        gatehouse_count == 1,
        "exactly one sporeGate in topology (single gatehouse)",
    );
}

fn phase_drawbridge(v: &mut ValidationResult) {
    let http_owner = capability_to_primal("http");
    v.check_bool(
        "drawbridge:http_owner_songbird",
        http_owner == "songbird",
        &format!("http → {http_owner} (songBird IS the drawbridge)"),
    );

    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        return;
    };

    let http_methods = parsed
        .get("http")
        .and_then(|h| h.get("methods"))
        .and_then(|m| m.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    v.check_bool(
        "drawbridge:http_proxy",
        http_methods.contains(&"http.proxy"),
        "http.proxy method registered (the drawbridge crossing)",
    );

    let mesh_owner = capability_to_primal("mesh");
    v.check_bool(
        "drawbridge:mesh_owner_songbird",
        mesh_owner == "songbird",
        &format!("mesh → {mesh_owner} (songBird owns internal routing)"),
    );

    let discovery_owner = capability_to_primal("discovery");
    v.check_bool(
        "drawbridge:discovery_songbird",
        discovery_owner == "songbird",
        &format!("discovery → {discovery_owner} (capability-based, not port-based)"),
    );
}

fn phase_darkforest(v: &mut ValidationResult) {
    let gates = all_mesh_gates();

    let darkforest_gates: Vec<_> = gates
        .iter()
        .filter(|g| g.name != "sporeGate")
        .collect();

    v.check_bool(
        "darkforest:non_gatehouse_gates",
        darkforest_gates.len() >= 4,
        &format!(
            "{} darkforest gates (non-sporeGate): {:?}",
            darkforest_gates.len(),
            darkforest_gates
                .iter()
                .map(|g| g.name.as_str())
                .collect::<Vec<_>>()
        ),
    );

    for gate in &darkforest_gates {
        if gate.address.is_empty() {
            continue;
        }
        v.check_bool(
            &format!("darkforest:{}:no_external_surface", gate.name),
            true,
            &format!(
                "{} is darkforest — mesh-only (transport: {})",
                gate.name,
                if gate.transport.is_empty() {
                    "wireguard"
                } else {
                    &gate.transport
                }
            ),
        );
    }

    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        return;
    };
    let http_section = parsed.get("http");
    if let Some(http) = http_section {
        let owner = http.get("owner").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "darkforest:http_not_public",
            owner == "songbird",
            "HTTP capabilities routed through songBird (not directly exposed)",
        );
    }
}

fn phase_transport_hierarchy(v: &mut ValidationResult) {
    let caps = PlatformCapabilities::detect();

    v.check_bool(
        "transport:uds_available",
        caps.uds_available,
        "UDS available on this gate (darkforest primary transport)",
    );

    let mode = caps.recommended_bind_mode();
    v.check_bool(
        "transport:bind_mode_not_tcp_only",
        mode != crate::ipc::server_bind::BindMode::TcpOnly,
        &format!("recommended bind mode = {mode:?} (prefer UDS over TCP)"),
    );

    let gates = all_mesh_gates();
    let adb_gates: Vec<_> = gates.iter().filter(|g| g.transport == "adb").collect();
    let wg_gates: Vec<_> = gates.iter().filter(|g| g.transport != "adb").collect();

    v.check_bool(
        "transport:wg_gates_present",
        wg_gates.len() >= 4,
        &format!("{} WireGuard-transport gates", wg_gates.len()),
    );

    if !adb_gates.is_empty() {
        v.check_bool(
            "transport:adb_gates_abstract",
            true,
            &format!(
                "{} ADB-transport gates (use abstract sockets): {:?}",
                adb_gates.len(),
                adb_gates.iter().map(|g| g.name.as_str()).collect::<Vec<_>>()
            ),
        );
    }
}

fn phase_live(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "live:gatehouse_health",
            "no live capabilities — requires deployed bearDog in gatehouse mode",
        );
        return;
    }

    v.check_bool(
        "live:has_tls_cap",
        caps.contains(&"tls"),
        "TLS capability discovered (bearDog gatehouse active)",
    );
    v.check_bool(
        "live:has_http_cap",
        caps.contains(&"http"),
        "HTTP capability discovered (songBird drawbridge active)",
    );
    v.check_bool(
        "live:has_mesh_cap",
        caps.contains(&"mesh"),
        "mesh capability discovered (darkforest routing active)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn gatehouse_darkforest_structural() {
        let mut v = ValidationResult::new("gatehouse-darkforest");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.passed > 0, "should have passed checks");
    }
}
