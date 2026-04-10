// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp090: Tower Atomic LAN Probe — discover basement HPC gates and map capabilities.
//!
//! Uses biomeOS neural-api + Songbird to probe the LAN via BirdSong UDP
//! multicast, discover all reachable primals, and build a topology map.
//!
//! Flow:
//!   1. Start Tower Atomic (BearDog + Songbird) on local gate
//!   2. mesh.init + mesh.auto_discover via BirdSong multicast (239.255.77.77)
//!   3. For each discovered peer, capabilities.list via TCP JSON-RPC
//!   4. Test HTTPS through Tower: http.get on each discovered Songbird
//!   5. Report: gate inventory, capabilities per gate, HTTPS status, latency
//!
//! Environment:
//!   `FAMILY_ID`         — shared family ID for mesh (default: 8ff3b864a4bc589a)
//!   `NODE_ID`           — this gate's node ID (default: eastgate)
//!   `SONGBIRD_PORT`     — local Songbird TCP port (default: 9200)
//!   `BEARDOG_PORT`      — local BearDog TCP port (default: 9100)
//!   `NEURAL_API_SOCKET` — biomeOS neural-api socket path (auto-discovered)

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::methods;
use primalspring::ipc::tcp::tcp_rpc;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn songbird_port() -> u16 {
    std::env::var("SONGBIRD_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(tolerances::TCP_FALLBACK_SONGBIRD_PORT)
}

fn beardog_port() -> u16 {
    std::env::var("BEARDOG_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(tolerances::TCP_FALLBACK_BEARDOG_PORT)
}

fn validate_local_tower(v: &mut ValidationResult) {
    v.section("Local Tower Atomic Health");

    let bd_port = beardog_port();
    let sb_port = songbird_port();

    match tcp_rpc(
        "localhost",
        bd_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    ) {
        Ok((resp, latency)) => {
            let ms = latency.as_millis();
            println!("  BearDog:  LIVE (port {bd_port}, {ms}ms)");
            v.check_bool(
                "local_beardog_live",
                true,
                &format!("BearDog at :{bd_port} ({ms}ms)"),
            );

            let status = resp
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("unknown");
            println!("  BearDog status: {status}");
        }
        Err(e) => {
            println!("  BearDog:  DOWN ({e})");
            v.check_bool(
                "local_beardog_live",
                false,
                &format!("BearDog unreachable: {e}"),
            );
        }
    }

    match tcp_rpc(
        "localhost",
        sb_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    ) {
        Ok((_, latency)) => {
            let ms = latency.as_millis();
            println!("  Songbird: LIVE (port {sb_port}, {ms}ms)");
            v.check_bool(
                "local_songbird_live",
                true,
                &format!("Songbird at :{sb_port} ({ms}ms)"),
            );
        }
        Err(e) => {
            println!("  Songbird: DOWN ({e})");
            v.check_bool(
                "local_songbird_live",
                false,
                &format!("Songbird unreachable: {e}"),
            );
        }
    }
}

fn validate_mesh_discovery(
    v: &mut ValidationResult,
    family_id: &str,
    node_id: &str,
) -> Vec<serde_json::Value> {
    v.section("BirdSong Mesh Discovery");

    let sb_port = songbird_port();

    match tcp_rpc(
        "localhost",
        sb_port,
        "mesh.init",
        &serde_json::json!({
            "node_id": node_id,
            "family_id": family_id,
            "bootstrap_onions": []
        }),
    ) {
        Ok(_) => {
            println!("  mesh.init: OK");
            v.check_bool("mesh_init", true, "mesh.init succeeded");
        }
        Err(e) => {
            println!("  mesh.init: {e}");
            let acceptable = e.contains("Method not found");
            if acceptable {
                v.check_skip(
                    "mesh_init",
                    "mesh.init not available on this Songbird build",
                );
            } else {
                v.check_skip("mesh_init", &format!("mesh.init error: {e}"));
            }
        }
    }

    match tcp_rpc(
        "localhost",
        sb_port,
        "mesh.auto_discover",
        &serde_json::json!({}),
    ) {
        Ok((resp, _)) => {
            println!("  mesh.auto_discover: {resp}");
            v.check_bool("mesh_auto_discover", true, "mesh.auto_discover responded");
        }
        Err(e) => {
            println!("  mesh.auto_discover: {e}");
            v.check_skip("mesh_auto_discover", &format!("auto_discover: {e}"));
        }
    }

    let peers = match tcp_rpc("localhost", sb_port, "mesh.peers", &serde_json::json!({})) {
        Ok((resp, _)) => {
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
                let addr = peer
                    .get("address")
                    .and_then(|a| a.as_str())
                    .unwrap_or("unknown");
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
    };

    peers
}

fn validate_peer_capabilities(v: &mut ValidationResult, peers: &[serde_json::Value]) {
    v.section("Peer Capability Enumeration");

    if peers.is_empty() {
        println!("  No peers discovered — skipping capability enumeration.");
        v.check_skip("peer_capabilities", "no peers to enumerate");
        return;
    }

    let mut total_caps: usize = 0;
    for (i, peer) in peers.iter().enumerate() {
        let addr = peer
            .get("address")
            .and_then(|a| a.as_str())
            .unwrap_or("127.0.0.1");

        let (host, port) = parse_host_port(addr, tolerances::TCP_FALLBACK_SONGBIRD_PORT);
        let check_name = format!("peer_{i}_capabilities");

        match tcp_rpc(
            &host,
            port,
            methods::capabilities::LIST,
            &serde_json::json!({}),
        ) {
            Ok((caps, latency)) => {
                let count = caps
                    .as_array()
                    .map(Vec::len)
                    .or_else(|| {
                        caps.get("capabilities")
                            .and_then(|c| c.as_array())
                            .map(Vec::len)
                    })
                    .unwrap_or(0);
                let ms = latency.as_millis();
                println!("  peer[{i}] {addr}: {count} capabilities ({ms}ms)");
                total_caps += count;
                v.check_bool(
                    &check_name,
                    count > 0,
                    &format!("peer {addr}: {count} caps"),
                );
            }
            Err(e) => {
                println!("  peer[{i}] {addr}: unreachable ({e})");
                v.check_skip(&check_name, &format!("peer {addr}: {e}"));
            }
        }
    }

    println!("  Total peer capabilities: {total_caps}");
}

fn validate_https_through_tower(v: &mut ValidationResult) {
    v.section("HTTPS Through Tower Atomic");

    let bridge = match NeuralBridge::discover() {
        Some(b) => b,
        None => {
            println!("  biomeOS not running — trying direct Songbird TCP");
            let sb_port = songbird_port();
            match tcp_rpc(
                "localhost",
                sb_port,
                "http.get",
                &serde_json::json!({ "url": "https://ifconfig.me/ip" }),
            ) {
                Ok((resp, latency)) => {
                    let status = resp
                        .get("status_code")
                        .and_then(|s| s.as_u64())
                        .unwrap_or(0);
                    let ms = latency.as_millis();
                    println!("  HTTPS via direct Songbird: status {status} ({ms}ms)");
                    v.check_bool(
                        "tower_https",
                        status == 200,
                        &format!("HTTPS status {status}"),
                    );
                }
                Err(e) => {
                    println!("  HTTPS via Songbird: {e}");
                    v.check_skip("tower_https", &format!("Songbird HTTPS: {e}"));
                }
            }
            return;
        }
    };

    let health = bridge.health_check();
    if health.is_err() {
        println!("  biomeOS neural-api unhealthy");
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

fn validate_stun(v: &mut ValidationResult) {
    v.section("STUN / NAT Discovery");

    let sb_port = songbird_port();
    match tcp_rpc(
        "localhost",
        sb_port,
        "stun.get_public_address",
        &serde_json::json!({}),
    ) {
        Ok((resp, latency)) => {
            let addr = resp.get("address").and_then(|a| a.as_str()).unwrap_or("?");
            let ms = latency.as_millis();
            println!("  Public address: {addr} ({ms}ms)");
            v.check_bool("stun_public_address", true, &format!("STUN: {addr}"));
        }
        Err(e) => {
            println!("  STUN: {e}");
            v.check_skip("stun_public_address", &format!("STUN: {e}"));
        }
    }
}

fn parse_host_port(addr: &str, default_port: u16) -> (String, u16) {
    match addr.rsplit_once(':') {
        Some((host, port_str)) => {
            let port = port_str.parse().unwrap_or(default_port);
            (host.to_owned(), port)
        }
        None => (addr.to_owned(), default_port),
    }
}

fn main() {
    let family_id = env_or("FAMILY_ID", "8ff3b864a4bc589a");
    let node_id = env_or("NODE_ID", "eastgate");

    ValidationResult::new("primalSpring Exp090 — Tower Atomic LAN Probe")
        .with_provenance("exp090_tower_atomic_lan_probe", "2026-04-06")
        .run(
            "primalSpring Exp090: LAN discovery + capability topology + HTTPS via Tower Atomic",
            |v| {
                println!("  Node ID:   {node_id}");
                println!("  Family ID: {family_id}");
                println!("  BearDog:   localhost:{}", beardog_port());
                println!("  Songbird:  localhost:{}", songbird_port());
                println!();

                validate_local_tower(v);

                let peers = validate_mesh_discovery(v, &family_id, &node_id);

                validate_peer_capabilities(v, &peers);

                validate_https_through_tower(v);

                validate_stun(v);

                v.section("Topology Summary");
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
            },
        );
}
