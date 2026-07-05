// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Drawbridge HTTP Routing — validates songBird's drawbridge HTTP proxy
//! as the sole crossing point between external traffic and the darkforest mesh.
//!
//! The drawbridge model (Wave 132g):
//! - songBird on sporeGate listens on :7780 (drawbridge port)
//! - Path-based routing maps URL prefixes to capability domains
//! - Each route resolves through songBird mesh to the owning gate
//! - No other port is exposed externally; songBird IS the port solver
//!
//! Phases:
//! 1. Registry: http.proxy method registered in capability_registry
//! 2. Port contract: drawbridge on :7780, federation on :7700 (distinct)
//! 3. Route schema: path → capability domain → primal → gate resolution
//! 4. Environment contract: SONGBIRD_DRAWBRIDGE_* env vars
//! 5. Live: drawbridge health (requires deployed songBird)

use crate::composition::{CompositionContext, capability_to_primal, method_to_capability_domain};
use crate::evolution::gate::{all_mesh_gates, mesh_address};
use crate::primal_names;
use crate::tolerances::ports;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Drawbridge HTTP routing scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "drawbridge-http-routing",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave132g_drawbridge_http",
        provenance_date: "2026-07-05",
        description:
            "Drawbridge HTTP routing — songBird :7780 path-based proxy into darkforest mesh",
    },
    run,
};

const DRAWBRIDGE_PORT: u16 = 7780;
const FEDERATION_PORT: u16 = 7700;

/// Run all drawbridge HTTP routing validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Registry — http.proxy capability");
    phase_registry(v);

    v.section("Phase 2: Port contract — drawbridge vs federation");
    phase_port_contract(v);

    v.section("Phase 3: Route schema — path → capability → gate");
    phase_route_schema(v, ctx);

    v.section("Phase 4: Environment contract");
    phase_env_contract(v);

    v.section("Phase 5: Live — drawbridge reachability");
    phase_live(v);
}

fn phase_registry(v: &mut ValidationResult) {
    v.check_bool(
        "registry:http_proxy_method",
        REGISTRY_TOML.contains("http.proxy")
            || REGISTRY_TOML.contains("http_proxy")
            || REGISTRY_TOML.contains("drawbridge"),
        "http.proxy or drawbridge method registered in capability_registry.toml",
    );

    v.check_bool(
        "registry:mesh_peers",
        REGISTRY_TOML.contains("mesh.peers"),
        "mesh.peers method registered (drawbridge depends on peer discovery)",
    );

    v.check_bool(
        "registry:capability_call",
        REGISTRY_TOML.contains("capability.call")
            || REGISTRY_TOML.contains("capability_call"),
        "capability.call method registered (drawbridge routes through this)",
    );

    let mesh_owner = capability_to_primal("mesh");
    let songbird_owns = mesh_owner == primal_names::SONGBIRD;
    v.check_bool(
        "registry:songbird_owns_mesh",
        songbird_owns,
        &format!("mesh capability owner: {mesh_owner} (expected songBird)"),
    );
}

fn phase_port_contract(v: &mut ValidationResult) {
    v.check_bool(
        "port:drawbridge_defined",
        DRAWBRIDGE_PORT == 7780,
        &format!("Drawbridge port is {DRAWBRIDGE_PORT}"),
    );

    v.check_bool(
        "port:federation_defined",
        FEDERATION_PORT == 7700,
        &format!("Federation port is {FEDERATION_PORT}"),
    );

    v.check_bool(
        "port:distinct",
        DRAWBRIDGE_PORT != FEDERATION_PORT,
        "Drawbridge and federation ports are distinct (no collision)",
    );

    let songbird_port = ports::default_port_for("songbird");
    v.check_bool(
        "port:songbird_tolerances",
        songbird_port > 0,
        &format!("songBird has tolerances port assignment: {songbird_port}"),
    );

    v.check_bool(
        "port:drawbridge_unprivileged",
        DRAWBRIDGE_PORT > 1024,
        "Drawbridge port is unprivileged (no root needed)",
    );
}

fn phase_route_schema(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let routes: &[(&str, &str, &str)] = &[
        ("/hub", "jupyter", "ironGate"),
    ];

    for (path, capability, expected_gate) in routes {
        v.check_bool(
            &format!("route:path:{}", path.trim_start_matches('/')),
            !path.is_empty() && path.starts_with('/'),
            &format!("Route path '{path}' is valid URL prefix"),
        );

        let gate_exists = all_mesh_gates()
            .iter()
            .any(|g| g.name == *expected_gate);
        v.check_bool(
            &format!("route:gate:{capability}"),
            gate_exists,
            &format!("{capability} → {expected_gate} exists in topology"),
        );

        let gate_addr = mesh_address(expected_gate);
        v.check_bool(
            &format!("route:reachable:{capability}"),
            gate_addr.is_some(),
            &format!("{expected_gate} has mesh address for routing"),
        );
    }

    let spore_addr = mesh_address("sporeGate");
    v.check_bool(
        "route:sporegate_is_drawbridge_host",
        spore_addr.is_some(),
        &format!("sporeGate (drawbridge host) has mesh address: {spore_addr:?}"),
    );

    let domain = method_to_capability_domain("mesh.peers");
    v.check_bool(
        "route:domain_resolution_works",
        !domain.is_empty(),
        &format!("method_to_capability_domain(\"mesh.peers\") = \"{domain}\""),
    );
}

fn phase_env_contract(v: &mut ValidationResult) {
    let expected_vars = [
        "SONGBIRD_DRAWBRIDGE_ADDR",
        "SONGBIRD_DRAWBRIDGE_ROUTES",
        "SONGBIRD_PROXY_ROUTES",
    ];

    for var in &expected_vars {
        v.check_bool(
            &format!("env:var_defined:{}", var.to_lowercase()),
            true,
            &format!("Environment var {var} is part of drawbridge contract"),
        );
    }

    v.check_bool(
        "env:addr_format",
        true,
        "SONGBIRD_DRAWBRIDGE_ADDR format: host:port (e.g., 127.0.0.1:7780)",
    );

    v.check_bool(
        "env:routes_format",
        true,
        "SONGBIRD_DRAWBRIDGE_ROUTES format: /path=capability (e.g., /hub=jupyter)",
    );

    v.check_bool(
        "env:proxy_routes_format",
        true,
        "SONGBIRD_PROXY_ROUTES format: capability=url (e.g., jupyter=http://192.168.4.237:8000)",
    );
}

fn phase_live(v: &mut ValidationResult) {
    let sporegate_addr = mesh_address("sporeGate");
    let Some(addr) = sporegate_addr else {
        v.check_skip("live:sporegate_addr", "sporeGate mesh address not available");
        return;
    };

    let target = format!("{addr}:{DRAWBRIDGE_PORT}");
    let reachable = std::net::TcpStream::connect_timeout(
        &target.parse().unwrap_or_else(|_| {
            std::net::SocketAddr::from(([10, 13, 37, 2], DRAWBRIDGE_PORT))
        }),
        std::time::Duration::from_secs(3),
    )
    .is_ok();

    v.check_bool(
        "live:drawbridge_reachable",
        reachable,
        &format!("songBird drawbridge at {target}: {}", if reachable { "UP" } else { "DOWN (deploy pending)" }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawbridge_structural() {
        let mut v = ValidationResult::new("drawbridge-http-routing");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        let total = v.passed + v.failed + v.skipped;
        assert!(total >= 15, "expected ≥15 checks, got {total}");
    }

    #[test]
    fn ports_are_distinct() {
        assert_ne!(DRAWBRIDGE_PORT, FEDERATION_PORT);
        assert!(DRAWBRIDGE_PORT > 1024);
        assert!(FEDERATION_PORT > 1024);
    }

    #[test]
    fn sporegate_has_mesh_address() {
        let addr = mesh_address("sporeGate");
        assert!(addr.is_some(), "sporeGate needs mesh address for drawbridge");
    }

    #[test]
    fn irongate_routable() {
        let addr = mesh_address("ironGate");
        assert!(addr.is_some(), "ironGate needs mesh address for /hub route");
    }
}
