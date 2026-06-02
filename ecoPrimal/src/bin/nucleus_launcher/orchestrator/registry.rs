// SPDX-License-Identifier: AGPL-3.0-or-later

//! Songbird registry seeding, capability mapping, health probes, and port resolution.

use std::collections::HashMap;
use std::time::Duration;

use primalspring::ipc::tcp::env_port;
use primalspring::tolerances;

/// Build primal → capability domains map from `capability_registry.toml`.
///
/// The registry TOML has `[domain] owner = "primal"` sections. We invert
/// this to build a primal → Vec<domain> mapping for Songbird seeding.
///
/// Falls back to a minimal static table if the registry file is missing.
pub(super) fn build_capability_map() -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let registry_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/capability_registry.toml");
    if let Ok(content) = std::fs::read_to_string(&registry_path) {
        if let Ok(parsed) = content.parse::<toml::Table>() {
            for (domain, section) in &parsed {
                if let Some(owner) = section.get("owner").and_then(|v| v.as_str()) {
                    if owner != "all" && owner != "none" && owner != "tests" {
                        map.entry(owner.to_owned())
                            .or_default()
                            .push(domain.clone());
                    }
                }
            }
        }
    }

    if map.is_empty() {
        tracing::warn!("capability_registry.toml not found or empty — using static fallback");
        for entry in tolerances::PORT_REGISTRY {
            let caps = static_fallback_caps(entry.slug);
            if !caps.is_empty() {
                map.insert(
                    entry.slug.to_owned(),
                    caps.iter().map(|s| (*s).to_owned()).collect(),
                );
            }
        }
    }

    map
}

fn static_fallback_caps(primal: &str) -> &'static [&'static str] {
    match primal {
        "beardog" => &["security", "crypto"],
        "songbird" => &["discovery", "http"],
        "skunkbat" => &["defense", "audit"],
        "toadstool" => &["compute"],
        "barracuda" => &["tensor", "stats"],
        "coralreef" => &["shader"],
        "nestgate" => &["storage", "content"],
        "rhizocrypt" => &["dag"],
        "loamspine" => &["ledger"],
        "sweetgrass" => &["attribution", "commit"],
        "biomeos" => &["orchestration", "graph"],
        "squirrel" => &["ai", "inference"],
        "petaltongue" => &["visualization"],
        _ => &[],
    }
}

/// Resolve the effective TCP port for a primal (env override → centralized default).
///
/// Returns 0 in UDS-only mode (VPS standard).
pub(super) fn effective_port_for(primal: &str, uds_only: bool) -> u16 {
    if uds_only {
        return 0;
    }
    effective_port(primal)
}

/// Resolve the effective TCP port for a primal (env override → centralized default).
pub(super) fn effective_port(primal: &str) -> u16 {
    let key = tolerances::port_env_key_for(primal);
    if key.is_empty() {
        return 0;
    }
    env_port(key, tolerances::default_port_for(primal))
}

/// Perform a JSON-RPC health check on a primal via TCP.
pub(super) fn health_check_tcp(port: u16, timeout: Duration) -> bool {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let payload = r#"{"jsonrpc":"2.0","method":"health.check","params":{},"id":1}"#;

    let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return false;
    };

    if stream.set_read_timeout(Some(timeout)).is_err()
        || stream.set_write_timeout(Some(timeout)).is_err()
    {
        return false;
    }

    let mut s = stream;
    if s.write_all(payload.as_bytes()).is_err() {
        return false;
    }
    if s.write_all(b"\n").is_err() {
        return false;
    }

    let mut buf = [0u8; 4096];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => {
            let response = String::from_utf8_lossy(&buf[..n]);
            response.contains("\"jsonrpc\"")
        }
        _ => false,
    }
}

/// Seed Songbird with known peer addresses for cross-gate mesh discovery.
pub(super) fn seed_songbird_peers(port: u16, peers: &[String], node_id: &str) -> usize {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let timeout = Duration::from_secs(5);
    let mut seeded = 0;

    let peers_json: Vec<String> = peers.iter().map(|p| format!("\"{p}\"")).collect();
    let payload = format!(
        r#"{{"jsonrpc":"2.0","method":"mesh.init","params":{{"node_id":"{node_id}","bootstrap_peers":[{}]}},"id":2}}"#,
        peers_json.join(",")
    );

    let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return 0;
    };
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));
    let mut s = stream;

    if s.write_all(payload.as_bytes()).is_ok() && s.write_all(b"\n").is_ok() {
        let mut buf = [0u8; 4096];
        if let Ok(n) = s.read(&mut buf) {
            if n > 0 {
                let resp = String::from_utf8_lossy(&buf[..n]);
                if resp.contains("\"result\"") {
                    seeded = peers.len();
                }
            }
        }
    }

    seeded
}

/// Send a register payload to Songbird.
pub(super) fn register_with_songbird(port: u16, payload: &str) -> Result<(), String> {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let timeout = Duration::from_secs(5);

    let stream = TcpStream::connect_timeout(&addr, timeout)
        .map_err(|e| format!("Songbird :{port} unreachable: {e}"))?;

    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));

    let mut s = stream;
    s.write_all(payload.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    s.write_all(b"\n")
        .map_err(|e| format!("write newline: {e}"))?;

    let mut buf = [0u8; 4096];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => {
            let resp = String::from_utf8_lossy(&buf[..n]);
            if resp.contains("\"result\"") {
                Ok(())
            } else {
                Err(format!(
                    "non-standard response: {}",
                    &resp[..resp.len().min(80)]
                ))
            }
        }
        Ok(_) => Err("empty response".to_owned()),
        Err(e) => Err(format!("read: {e}")),
    }
}
