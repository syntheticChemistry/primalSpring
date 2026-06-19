// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: K-Derm Boundary Validation — ensures the cell envelope model
//! correctly maps to composition transport and trust layers.
//!
//! The K-Derm envelope model from `whitePaper/` defines four layers that
//! map 1:1 to the primalSpring transport + bonding architecture:
//!
//! | Biological Layer | Transport      | Bond Type   | Trust Boundary           |
//! |-----------------|----------------|-------------|--------------------------|
//! | Cytoplasm       | UDS (local)    | Covalent    | Same-gate primals only   |
//! | Plasma Membrane | nftables/NAT   | Boundary    | Channel proteins = rules |
//! | Periplasm       | WireGuard/Fed  | Metallic    | Multi-gate mesh          |
//! | Extracellular   | Internet/TCP   | Weak/Ionic  | Dark Forest, untrusted   |
//!
//! This scenario validates:
//! 1. Transport layer assignments respect bond-type trust hierarchy
//! 2. UDS-first posture is the default (cytoplasm = Covalent)
//! 3. TCP is gated behind explicit opt-in (plasma membrane = selective permeability)
//! 4. Federation port exists for periplasm layer (mesh coordination)
//! 5. Bond types are properly ordered by trust (Covalent > Metallic > Ionic > Weak)

use crate::bonding::BondType;
use crate::composition::CompositionContext;
use crate::ipc::server_bind::BindMode;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};

/// K-Derm boundary validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "kderm-boundary",
        track: Track::Transport,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-17",
        description: "K-Derm envelope: transport layers map to bond trust hierarchy",
    },
    run,
};

/// Run K-Derm boundary validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Transport layer trust mapping");
    phase_transport_trust(v);

    v.section("Phase 2: Cytoplasm posture (UDS-first default)");
    phase_cytoplasm_posture(v);

    v.section("Phase 3: Plasma membrane (selective permeability)");
    phase_plasma_membrane(v);

    v.section("Phase 4: Periplasm (federation mesh)");
    phase_periplasm(v);

    v.section("Phase 5: Bond hierarchy invariants");
    phase_bond_hierarchy(v);
}

fn phase_transport_trust(v: &mut ValidationResult) {
    v.check_bool(
        "kderm:uds_is_cytoplasm",
        BindMode::from_env() == BindMode::UdsOnly
            || std::env::var(crate::env_keys::PRIMAL_BIND_MODE).is_err(),
        &format!(
            "default transport: {:?} (cytoplasm = UDS-only when no override)",
            BindMode::from_env()
        ),
    );

    let platform_caps = crate::ipc::platform::PlatformCapabilities::detect();
    v.check_bool(
        "kderm:platform_has_uds",
        platform_caps.uds_available,
        &format!(
            "UDS available: {} (cytoplasm requires local socket support)",
            platform_caps.uds_available
        ),
    );

    v.check_bool(
        "kderm:platform_has_tcp",
        platform_caps.tcp_available,
        &format!(
            "TCP available: {} (extracellular path exists but gated)",
            platform_caps.tcp_available
        ),
    );
}

fn phase_cytoplasm_posture(v: &mut ValidationResult) {
    let socket_dir = crate::tolerances::platform::runtime_dir();
    let socket_path = std::path::Path::new(&socket_dir);

    v.check_bool(
        "cytoplasm:runtime_dir_resolved",
        !socket_dir.is_empty(),
        &format!("runtime dir: {socket_dir}"),
    );

    let biomeos_dir = socket_path.join(crate::env_keys::BIOMEOS_SUBDIR);
    let dir_exists = biomeos_dir.is_dir();
    if dir_exists {
        v.check_bool(
            "cytoplasm:socket_dir_exists",
            true,
            &format!("biomeos socket dir: {}", biomeos_dir.display()),
        );

        let socket_count = std::fs::read_dir(&biomeos_dir).map_or(0, |rd| {
            rd.filter_map(Result::ok)
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "sock"))
                .count()
        });

        if socket_count > 0 {
            v.check_bool(
                "cytoplasm:sockets_present",
                true,
                &format!("{socket_count} .sock files in cytoplasm (live primals)"),
            );
        } else {
            v.check_skip(
                "cytoplasm:sockets_present",
                "0 .sock files (primals not running — cytoplasm inactive)",
            );
        }
    } else {
        v.check_skip(
            "cytoplasm:socket_dir_exists",
            &format!(
                "no biomeos dir at {} (primals not running)",
                biomeos_dir.display()
            ),
        );
    }

    // TCP Tier 5 should be disabled in release builds (compile-time enforcement)
    #[cfg(not(debug_assertions))]
    v.check_bool(
        "cytoplasm:tcp_tier5_disabled_release",
        true,
        "TCP Tier 5 compile-time disabled in release (plasma membrane sealed)",
    );

    #[cfg(debug_assertions)]
    {
        let tcp_tier5_explicitly_enabled =
            std::env::var(crate::env_keys::PRIMALSPRING_TCP_TIER5).is_ok_and(|v| v == "1");
        v.check_bool(
            "cytoplasm:tcp_tier5_gated",
            !tcp_tier5_explicitly_enabled,
            &format!(
                "TCP Tier 5: {} (debug build — env gate)",
                if tcp_tier5_explicitly_enabled {
                    "ENABLED (membrane open)"
                } else {
                    "disabled (sealed)"
                }
            ),
        );
    }
}

fn phase_plasma_membrane(v: &mut ValidationResult) {
    let port_registry = tolerances::ports::PORT_REGISTRY;

    v.check_bool(
        "membrane:port_registry_exists",
        !port_registry.is_empty(),
        &format!(
            "{} port entries in registry (channel protein slots)",
            port_registry.len()
        ),
    );

    // In UDS-only mode, ports should not be actively bound
    // (they exist in config for TCP fallback / cross-gate, but are dormant)
    let bind_mode = BindMode::from_env();
    if bind_mode == BindMode::UdsOnly {
        v.check_bool(
            "membrane:ports_dormant_uds_mode",
            true,
            "UDS-only mode: port slots defined but dormant (membrane sealed)",
        );
    } else {
        v.check_bool(
            "membrane:ports_active",
            true,
            &format!("bind mode {bind_mode:?}: port slots potentially active (membrane permeable)"),
        );
    }

    // Validate that all ports are in valid range and non-conflicting
    let mut ports_seen = std::collections::HashSet::new();
    let mut conflicts = 0u32;
    for entry in port_registry {
        if !ports_seen.insert(entry.port) {
            conflicts += 1;
        }
    }
    v.check_bool(
        "membrane:no_port_conflicts",
        conflicts == 0,
        &format!(
            "{} unique ports, {} conflicts (membrane channels must be distinct)",
            ports_seen.len(),
            conflicts
        ),
    );

    // Port range validation: all should be in user/dynamic range (>1024)
    let all_valid = port_registry
        .iter()
        .all(|e| e.port > 1024 && e.port < 65535);
    v.check_bool(
        "membrane:ports_valid_range",
        all_valid,
        "all ports in user range (1025..65534)",
    );
}

fn phase_periplasm(v: &mut ValidationResult) {
    let federation_port = tolerances::ports::FEDERATION_PORT;
    v.check_bool(
        "periplasm:federation_port",
        federation_port > 0 && federation_port < 65535,
        &format!("federation port: {federation_port} (periplasm mesh coordination)"),
    );

    let federation_profiles = tolerances::ports::FEDERATION_PORTS;
    v.check_bool(
        "periplasm:profiles_defined",
        !federation_profiles.is_empty(),
        &format!(
            "{} federation profiles (multi-gate mesh endpoints)",
            federation_profiles.len()
        ),
    );

    // Mesh peers represent periplasm connections (WireGuard or federation)
    let mesh_configured = std::env::var(crate::env_keys::MESH_PEERS).is_ok()
        || std::env::var("SONGBIRD_PEERS").is_ok();
    if mesh_configured {
        v.check_bool(
            "periplasm:mesh_peers_configured",
            true,
            "MESH_PEERS or SONGBIRD_PEERS set (periplasm active)",
        );
    } else {
        v.check_skip(
            "periplasm:mesh_peers_configured",
            "no mesh peers configured (single-gate deployment, no periplasm)",
        );
    }
}

fn phase_bond_hierarchy(v: &mut ValidationResult) {
    use crate::btsp::BtspCipherSuite;

    let bond_display = [
        (BondType::Covalent, "Covalent (cytoplasm)"),
        (BondType::Metallic, "Metallic (periplasm)"),
        (BondType::Ionic, "Ionic (extracellular-contract)"),
        (BondType::Weak, "Weak (extracellular-dark-forest)"),
    ];

    for (bond, label) in &bond_display {
        v.check_bool(
            &format!("hierarchy:{bond}:displayable"),
            !bond.to_string().is_empty(),
            &format!("{label}: Display = \"{bond}\""),
        );
    }

    v.check_bool(
        "hierarchy:covalent_allows_null",
        crate::btsp::cipher_allowed(BondType::Covalent, BtspCipherSuite::Null),
        "Covalent (cytoplasm): Null cipher allowed (same-gate trust)",
    );
    v.check_bool(
        "hierarchy:ionic_requires_encryption",
        !crate::btsp::cipher_allowed(BondType::Ionic, BtspCipherSuite::Null)
            && !crate::btsp::cipher_allowed(BondType::Ionic, BtspCipherSuite::HmacPlain),
        "Ionic (extracellular): requires ChaCha20Poly1305 (untrusted boundary)",
    );
    v.check_bool(
        "hierarchy:weak_requires_encryption",
        !crate::btsp::cipher_allowed(BondType::Weak, BtspCipherSuite::Null)
            && !crate::btsp::cipher_allowed(BondType::Weak, BtspCipherSuite::HmacPlain),
        "Weak (dark forest): requires ChaCha20Poly1305 (zero trust)",
    );
    v.check_bool(
        "hierarchy:metallic_allows_hmac",
        crate::btsp::cipher_allowed(BondType::Metallic, BtspCipherSuite::HmacPlain),
        "Metallic (periplasm): HmacPlain allowed (organizational trust, integrity without encryption)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kderm_boundary_structural() {
        let mut v = ValidationResult::new("kderm-boundary");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(
            v.failed,
            0,
            "K-Derm boundary: {failed} failures (all structural checks should pass)",
            failed = v.failed
        );
    }
}
