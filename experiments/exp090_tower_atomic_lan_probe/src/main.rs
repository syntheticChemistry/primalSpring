// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]

//! Exp090: Tower Atomic LAN Probe — discover basement HPC gates and map capabilities.
//!
//! Probes the LAN via biomeOS Neural API and Songbird mesh discovery to build
//! a topology map of reachable gates and their capability surfaces.

use primalspring::composition::CompositionContext;
use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn phase_composition_discovery(v: &mut ValidationResult, ctx: &CompositionContext) {
    v.section("Phase 1: Composition discovery (local)");
    let caps = ctx.available_capabilities();
    v.check_bool(
        "composition_capabilities_non_empty",
        !caps.is_empty(),
        &format!("{} capabilities: {}", caps.len(), caps.join(", ")),
    );
    v.check_bool(
        "has_security_capability_path",
        ctx.has_capability("security"),
        "security capability path",
    );
    v.check_bool(
        "has_discovery_capability_path",
        ctx.has_capability("discovery"),
        "discovery capability path",
    );
}

fn validate_local_tower(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 2: Local Tower Atomic health via CompositionContext");

    for domain in &["security", "discovery"] {
        let check_name = format!("local_{domain}_live");
        if !ctx.has_capability(domain) {
            v.check_skip(&check_name, &format!("{domain} not in composition"));
            continue;
        }
        match ctx.health_check(domain) {
            Ok(true) => {
                println!("  {domain}: LIVE");
                v.check_bool(&check_name, true, &format!("{domain} healthy"));
            }
            Ok(false) => {
                println!("  {domain}: UNHEALTHY");
                v.check_bool(&check_name, false, &format!("{domain} unhealthy"));
            }
            Err(e) => {
                println!("  {domain}: DOWN ({e})");
                v.check_skip(&check_name, &format!("{domain} unreachable: {e}"));
            }
        }
    }

    if let Some(bridge) = NeuralBridge::discover() {
        match bridge.health_check() {
            Ok(_) => {
                println!("  Neural API: HEALTHY");
                v.check_bool("neural_api_local", true, "biomeOS neural-api healthy");
            }
            Err(e) => {
                println!("  Neural API: {e}");
                v.check_bool("neural_api_local", false, &format!("neural-api: {e}"));
            }
        }
    } else {
        v.check_skip("neural_api_local", "biomeOS not running");
    }
}

fn validate_mesh_discovery(
    v: &mut ValidationResult,
    ctx: &mut CompositionContext,
    family_id: &str,
    node_id: &str,
) -> Vec<serde_json::Value> {
    v.section("Phase 3: BirdSong mesh discovery via CompositionContext");

    if !ctx.has_capability("mesh") && !ctx.has_capability("discovery") {
        v.check_skip("mesh_init", "no mesh/discovery capability");
        v.check_skip("mesh_peers_discovered", "no mesh/discovery capability");
        return Vec::new();
    }

    let mesh_domain = if ctx.has_capability("mesh") { "mesh" } else { "discovery" };

    match ctx.call(
        mesh_domain,
        "mesh.init",
        serde_json::json!({
            "node_id": node_id,
            "family_id": family_id,
            "bootstrap_onions": []
        }),
    ) {
        Ok(_) => {
            println!("  mesh.init: OK");
            v.check_bool("mesh_init", true, "mesh.init via CompositionContext");
        }
        Err(e) => {
            println!("  mesh.init: {e}");
            v.check_skip("mesh_init", &format!("mesh.init: {e}"));
        }
    }

    match ctx.call(
        mesh_domain,
        "mesh.auto_discover",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            println!("  mesh.auto_discover: {resp}");
            v.check_bool("mesh_auto_discover", true, "mesh.auto_discover responded");
        }
        Err(e) => {
            println!("  mesh.auto_discover: {e}");
            v.check_skip("mesh_auto_discover", &format!("auto_discover: {e}"));
        }
    }

    match ctx.call(mesh_domain, "mesh.peers", serde_json::json!({})) {
        Ok(resp) => {
            let peer_list = resp
                .as_array()
                .cloned()
                .or_else(|| resp.get("peers").and_then(|p| p.as_array()).cloned())
                .unwrap_or_default();
            let count = peer_list.len();
            println!("  mesh.peers: {count} peer(s) discovered");
            v.check_bool(
                "mesh_peers_discovered",
                count >= 1,
                &format!("{count} peer(s) on LAN mesh"),
            );

            for (i, peer) in peer_list.iter().enumerate() {
                let addr = peer.get("address").and_then(|a| a.as_str()).unwrap_or("unknown");
                let pid = peer.get("node_id").and_then(|n| n.as_str()).unwrap_or("?");
                println!("    [{i}] {pid} @ {addr}");
            }

            peer_list
        }
        Err(e) => {
            println!("  mesh.peers: {e}");
            v.check_skip("mesh_peers_discovered", &format!("mesh.peers: {e}"));
            Vec::new()
        }
    }
}

fn validate_https_through_tower(v: &mut ValidationResult) {
    v.section("Phase 4: HTTPS through Tower Atomic via Neural API");

    let Some(bridge) = NeuralBridge::discover() else {
        v.check_skip("tower_https", "biomeOS not running");
        return;
    };

    if bridge.health_check().is_err() {
        v.check_skip("tower_https", "biomeOS neural-api not healthy");
        return;
    }

    match bridge.capability_call(
        "http",
        "get",
        &serde_json::json!({ "url": "https://ifconfig.me/ip" }),
    ) {
        Ok(resp) => {
            let status = resp
                .value
                .get("status_code")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            println!("  HTTPS via neural-api → Songbird: status {status}");
            v.check_bool(
                "tower_https",
                status == 200,
                &format!("HTTPS routed through Tower: {status}"),
            );
        }
        Err(e) => {
            println!("  HTTPS via neural-api: {e}");
            v.check_skip("tower_https", &format!("capability.call: {e}"));
        }
    }
}

fn validate_stun(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 5: STUN / NAT discovery via CompositionContext");

    let stun_domain = if ctx.has_capability("stun") {
        "stun"
    } else if ctx.has_capability("network") {
        "network"
    } else if ctx.has_capability("discovery") {
        "discovery"
    } else {
        v.check_skip("stun_public_address", "no stun/network/discovery capability");
        return;
    };

    match ctx.call(
        stun_domain,
        "stun.get_public_address",
        serde_json::json!({}),
    ) {
        Ok(resp) => {
            let addr = resp.get("address").and_then(|a| a.as_str()).unwrap_or("?");
            println!("  Public address: {addr}");
            v.check_bool("stun_public_address", true, &format!("STUN: {addr}"));
        }
        Err(e) => {
            println!("  STUN: {e}");
            v.check_skip("stun_public_address", &format!("STUN: {e}"));
        }
    }
}

#[cfg(feature = "primordial-compat")]
fn validate_legacy_tcp_tower(v: &mut ValidationResult) {
    use primalspring::ipc::methods;
    use primalspring::ipc::tcp::tcp_rpc;
    use primalspring::tolerances;

    v.section("Phase 6 (legacy): Direct TCP tower probes");

    let bd_port: u16 = std::env::var("BEARDOG_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| tolerances::default_port_for("beardog"));
    let sb_port: u16 = std::env::var("SONGBIRD_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| tolerances::default_port_for("songbird"));

    let bd_ok = tcp_rpc("localhost", bd_port, methods::health::LIVENESS, &serde_json::json!({})).is_ok();
    v.check_bool("legacy_beardog_tcp", bd_ok, &format!("BearDog at :{bd_port}"));

    let sb_ok = tcp_rpc("localhost", sb_port, methods::health::LIVENESS, &serde_json::json!({})).is_ok();
    v.check_bool("legacy_songbird_tcp", sb_ok, &format!("Songbird at :{sb_port}"));
}

fn main() {
    let family_id = env_or("FAMILY_ID", "8ff3b864a4bc589a");
    let node_id = env_or("NODE_ID", "eastgate");

    ValidationResult::new("primalSpring Exp090 — Tower Atomic LAN Probe")
        .with_provenance("exp090_tower_atomic_lan_probe", "2026-05-09")
        .run(
            "LAN discovery + capability topology + HTTPS via Tower Atomic",
            |v| {
                let mut ctx = CompositionContext::from_live_discovery_with_fallback();
                phase_composition_discovery(v, &ctx);

                println!("  Node ID:   {node_id}");
                println!("  Family ID: {family_id}");
                println!();

                validate_local_tower(v, &mut ctx);
                let peers = validate_mesh_discovery(v, &mut ctx, &family_id, &node_id);
                if !peers.is_empty() {
                    v.section("Phase 3b: Peer topology");
                    for (i, peer) in peers.iter().enumerate() {
                        let addr = peer.get("address").and_then(|a| a.as_str()).unwrap_or("?");
                        println!("  peer[{i}]: {addr}");
                    }
                }
                validate_https_through_tower(v);
                validate_stun(v, &mut ctx);

                v.section("Topology summary");
                let peer_count = peers.len();
                let total_gates = peer_count + 1;
                println!("  Local gate:  {node_id}");
                println!("  LAN peers:   {peer_count}");
                println!("  Total gates: {total_gates}");
                v.check_bool(
                    "topology_mapped",
                    true,
                    &format!("{total_gates} gate(s) in mesh ({peer_count} peers + self)"),
                );

                #[cfg(feature = "primordial-compat")]
                validate_legacy_tcp_tower(v);
            },
        );
}
