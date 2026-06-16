// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery-provider registry seeding, capability mapping, health probes, and port resolution.
//!
//! The discovery provider (currently Songbird) is resolved via capability routing,
//! not hardcoded. Functions use generic names (`register_with_discovery`, etc.) to
//! decouple from any specific primal's identity.

use std::collections::HashMap;
use std::time::Duration;

use primalspring::ipc::tcp::env_port;
use primalspring::tolerances;

#[derive(Debug, thiserror::Error)]
pub(super) enum RegistryError {
    #[error("discovery provider unreachable: {0}")]
    Unreachable(#[source] std::io::Error),
    #[error("I/O: {0}")]
    Io(#[source] std::io::Error),
    #[error("empty response")]
    EmptyResponse,
    #[error("non-standard response: {0}")]
    BadResponse(String),
}

/// Resolve the loopback address for local IPC connections.
fn local_addr(port: u16) -> std::net::SocketAddr {
    use std::net::SocketAddr;
    let host: std::net::IpAddr = tolerances::platform::DEFAULT_HOST
        .parse()
        .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
    SocketAddr::new(host, port)
}

fn jsonrpc_payload(method: &str, params: &serde_json::Value, id: u64) -> String {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": id
    });
    serde_json::to_string(&request).unwrap_or_default()
}

fn jsonrpc_response_has_field(response: &str, field: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(response)
        .is_ok_and(|parsed| parsed.get(field).is_some())
}

/// Build primal → capability domains map from `capability_registry.toml`.
///
/// The registry TOML has `[domain] owner = "primal"` sections. We invert
/// this to build a primal → Vec<domain> mapping for discovery-provider seeding.
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
        for slug in tolerances::all_primal_slugs() {
            let caps = static_fallback_caps(slug);
            if !caps.is_empty() {
                map.insert(
                    slug.to_owned(),
                    caps.iter().map(|s| (*s).to_owned()).collect(),
                );
            }
        }
    }

    map
}

/// Minimal static fallback when `capability_registry.toml` is absent.
///
/// Derived from the canonical routing table at compile time. Each primal's
/// primary discovery domain (from `ALL_CAPS`) is included only when the
/// routing table maps it to that primal. This ensures the fallback cannot
/// drift from the TOML-driven path without a deliberate code change.
fn static_fallback_caps(primal: &str) -> &'static [&'static str] {
    use primalspring::composition::{ALL_CAPS, capability_to_primal};
    static FALLBACK: std::sync::LazyLock<
        std::collections::HashMap<&'static str, Vec<&'static str>>,
    > = std::sync::LazyLock::new(|| {
        let mut map: std::collections::HashMap<&'static str, Vec<&'static str>> =
            std::collections::HashMap::new();
        for &cap in ALL_CAPS.iter() {
            let owner = capability_to_primal(cap);
            map.entry(owner).or_default().push(cap);
        }
        map
    });
    FALLBACK.get(primal).map_or(&[], |v| v.as_slice())
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

/// Perform a NUCLEUS-aware capability probe via the neural API.
///
/// Preferred over raw TCP health checks: routes through biomeOS `capability.call`
/// to verify the primal can actually serve its registered capabilities (not just
/// liveness). Falls back to `false` if the neural bridge is unavailable.
pub(super) fn capability_probe(primal: &str) -> bool {
    use primalspring::ipc::discover::capability_call;

    let cap_map = build_capability_map();
    let Some(caps) = cap_map.get(primal) else {
        return false;
    };
    let primary_cap = caps.first().map_or("health", String::as_str);

    capability_call(primary_cap, "health.liveness", &serde_json::json!({})).is_some()
}

/// Health probe result with distinction between full health and reachable-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProbeResult {
    Healthy,
    Reachable,
    Unreachable,
}

impl ProbeResult {
    pub(super) const fn is_alive(self) -> bool {
        matches!(self, Self::Healthy | Self::Reachable)
    }
}

fn classify_jsonrpc_response(response: &str) -> ProbeResult {
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response) else {
        return ProbeResult::Unreachable;
    };
    if parsed.get("jsonrpc").is_none() {
        return ProbeResult::Unreachable;
    }
    if parsed.get("result").is_some() {
        return ProbeResult::Healthy;
    }
    // -32601 (method_not_found) means reachable but no health method implemented
    ProbeResult::Reachable
}

/// Perform a JSON-RPC health check on a primal via TCP.
///
/// Returns [`ProbeResult::Healthy`] if the primal responds with a valid `result`,
/// [`ProbeResult::Reachable`] if it responds with any JSON-RPC (including `-32601`
/// method_not_found), or [`ProbeResult::Unreachable`] on connection/timeout failure.
pub(super) fn health_check_tcp(port: u16, timeout: Duration) -> ProbeResult {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let addr = local_addr(port);
    let payload = jsonrpc_payload("health.check", &serde_json::json!({}), 1);

    let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return ProbeResult::Unreachable;
    };

    if stream.set_read_timeout(Some(timeout)).is_err()
        || stream.set_write_timeout(Some(timeout)).is_err()
    {
        return ProbeResult::Unreachable;
    }

    let mut s = stream;
    if s.write_all(&tolerances::RIBOCIPHER_CLEAR_SIGNAL).is_err() {
        return ProbeResult::Unreachable;
    }
    if s.write_all(payload.as_bytes()).is_err() {
        return ProbeResult::Unreachable;
    }
    if s.write_all(b"\n").is_err() {
        return ProbeResult::Unreachable;
    }

    let mut buf = [0u8; 4096];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => {
            let response = String::from_utf8_lossy(&buf[..n]);
            classify_jsonrpc_response(&response)
        }
        _ => ProbeResult::Unreachable,
    }
}

/// Perform a JSON-RPC health check on a primal via UDS socket.
pub(super) fn health_check_uds(socket: &std::path::Path) -> ProbeResult {
    let payload = jsonrpc_payload("health.check", &serde_json::json!({}), 1);
    send_uds_rpc(socket, &payload)
        .map_or(ProbeResult::Unreachable, |resp| classify_jsonrpc_response(&resp))
}

/// Resolve the UDS socket path for a primal.
pub(super) fn socket_path_for(primal: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(tolerances::runtime_dir())
        .join(primalspring::env_keys::BIOMEOS_SUBDIR)
        .join(format!("{primal}.sock"))
}

/// Seed the discovery provider with known peer addresses for cross-gate mesh.
pub(super) fn seed_discovery_peers(port: u16, peers: &[String], node_id: &str) -> usize {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let addr = local_addr(port);
    let timeout = Duration::from_secs(5);
    let mut seeded = 0;

    let payload = jsonrpc_payload(
        "mesh.init",
        &serde_json::json!({
            "node_id": node_id,
            "bootstrap_peers": peers,
        }),
        2,
    );

    let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) else {
        return 0;
    };
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));
    let mut s = stream;

    if s.write_all(&tolerances::RIBOCIPHER_CLEAR_SIGNAL).is_ok()
        && s.write_all(payload.as_bytes()).is_ok()
        && s.write_all(b"\n").is_ok()
    {
        let mut buf = [0u8; 4096];
        if let Ok(n) = s.read(&mut buf) {
            if n > 0 {
                let resp = String::from_utf8_lossy(&buf[..n]);
                if jsonrpc_response_has_field(&resp, "result") {
                    seeded = peers.len();
                }
            }
        }
    }

    seeded
}

/// Send a register payload to the discovery provider via TCP.
pub(super) fn register_with_discovery(port: u16, payload: &str) -> Result<(), RegistryError> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let addr = local_addr(port);
    let timeout = Duration::from_secs(5);

    let stream = TcpStream::connect_timeout(&addr, timeout).map_err(RegistryError::Unreachable)?;

    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));

    let mut s = stream;
    s.write_all(&tolerances::RIBOCIPHER_CLEAR_SIGNAL)
        .map_err(RegistryError::Io)?;
    s.write_all(payload.as_bytes()).map_err(RegistryError::Io)?;
    s.write_all(b"\n").map_err(RegistryError::Io)?;

    let mut buf = [0u8; 4096];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => {
            let resp = String::from_utf8_lossy(&buf[..n]);
            if jsonrpc_response_has_field(&resp, "result") {
                Ok(())
            } else {
                let truncated = &resp[..resp.len().min(80)];
                Err(RegistryError::BadResponse(truncated.to_string()))
            }
        }
        Ok(_) => Err(RegistryError::EmptyResponse),
        Err(e) => Err(RegistryError::Io(e)),
    }
}

/// Send a JSON-RPC payload over a Unix domain socket.
fn send_uds_rpc(socket: &std::path::Path, payload: &str) -> Result<String, RegistryError> {
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;

    let timeout = Duration::from_secs(5);
    let mut stream = UnixStream::connect(socket).map_err(RegistryError::Unreachable)?;

    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));

    stream
        .write_all(&tolerances::RIBOCIPHER_CLEAR_SIGNAL)
        .map_err(RegistryError::Io)?;
    stream
        .write_all(payload.as_bytes())
        .map_err(RegistryError::Io)?;
    stream.write_all(b"\n").map_err(RegistryError::Io)?;

    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            let resp = String::from_utf8_lossy(&buf[..n]).to_string();
            Ok(resp)
        }
        Ok(_) => Err(RegistryError::EmptyResponse),
        Err(e) => Err(RegistryError::Io(e)),
    }
}

/// Send a register payload to the discovery provider via UDS.
pub(super) fn register_with_discovery_uds(
    socket: &std::path::Path,
    payload: &str,
) -> Result<(), RegistryError> {
    let resp = send_uds_rpc(socket, payload)?;
    if jsonrpc_response_has_field(&resp, "result") {
        Ok(())
    } else {
        let truncated = &resp[..resp.len().min(80)];
        Err(RegistryError::BadResponse(truncated.to_string()))
    }
}

/// Seed the discovery provider with known peer addresses via UDS.
pub(super) fn seed_discovery_peers_uds(
    socket: &std::path::Path,
    peers: &[String],
    node_id: &str,
) -> usize {
    let payload = jsonrpc_payload(
        "mesh.init",
        &serde_json::json!({
            "node_id": node_id,
            "bootstrap_peers": peers,
        }),
        2,
    );

    match send_uds_rpc(socket, &payload) {
        Ok(resp) if jsonrpc_response_has_field(&resp, "result") => peers.len(),
        _ => 0,
    }
}
