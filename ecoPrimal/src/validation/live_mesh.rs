// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Live mesh validation infrastructure.
//!
//! Provides the runtime context for cross-gate validation scenarios that
//! require real BTSP credentials and network connectivity to remote gates.
//!
//! # Usage
//!
//! ```rust,no_run
//! use primalspring::validation::live_mesh::LiveMeshConfig;
//!
//! let config = LiveMeshConfig::from_env();
//! if config.is_ready() {
//!     // Cross-gate validation can proceed
//! }
//! ```

use std::collections::BTreeMap;
use std::time::Duration;

/// Configuration for live cross-gate mesh validation.
#[derive(Debug, Clone)]
pub struct LiveMeshConfig {
    /// Local gate identifier.
    pub local_gate: String,
    /// Remote gate endpoints (`gate_id` → host:port).
    pub remote_gates: BTreeMap<String, String>,
    /// Whether BTSP credentials are available.
    pub btsp_available: bool,
    /// Family ID for BTSP session establishment.
    pub family_id: Option<String>,
    /// Connection timeout for remote endpoints.
    pub connect_timeout: Duration,
}

/// Readiness check result for a single remote gate.
#[derive(Debug, Clone)]
pub struct GateReadiness {
    /// Gate identifier.
    pub gate_id: String,
    /// Network address.
    pub address: String,
    /// Whether TCP connection succeeded.
    pub tcp_reachable: bool,
    /// Whether Songbird HTTP endpoint responded.
    pub songbird_responding: bool,
    /// Whether BTSP handshake can be attempted.
    pub btsp_ready: bool,
}

impl LiveMeshConfig {
    /// Build from environment, optionally loading a benchScale topology.
    ///
    /// Checks `BENCHSCALE_TOPOLOGY` env var first. If set, loads the named
    /// topology from `benchScale/topologies/{name}.toml` and uses its gate
    /// definitions to seed remote peers and BTSP configuration. Falls back
    /// to pure environment-based config.
    #[must_use]
    pub fn from_env() -> Self {
        if let Ok(topo_name) = std::env::var(crate::env_keys::BENCHSCALE_TOPOLOGY) {
            if let Some(cfg) = Self::from_topology_file(&topo_name) {
                return cfg;
            }
        }
        Self::from_env_only()
    }

    /// Build configuration purely from environment variables.
    ///
    /// Reads:
    /// - `HOSTNAME` or `GATE_ID` for local gate identity
    /// - `MESH_PEERS` (or deprecated `SONGBIRD_PEERS`) for remote gate addresses
    /// - `FAMILY_ID` / `FAMILY_SEED` for BTSP readiness
    #[must_use]
    pub fn from_env_only() -> Self {
        let local_gate = std::env::var(crate::env_keys::GATE_ID)
            .or_else(|_| std::env::var(crate::env_keys::HOSTNAME))
            .unwrap_or_else(|_| {
                crate::tolerances::platform::hostname().unwrap_or_else(|| "unknown-gate".to_owned())
            });

        let remote_gates = parse_songbird_peers();

        let family_id = std::env::var(crate::env_keys::FAMILY_ID)
            .ok()
            .filter(|s| !s.is_empty() && s != "default");

        let btsp_available =
            family_id.is_some() && crate::env_keys::resolve_family_seed().is_some();

        Self {
            local_gate,
            remote_gates,
            btsp_available,
            family_id,
            connect_timeout: Duration::from_secs(crate::tolerances::TCP_CONNECT_TIMEOUT_SECS),
        }
    }

    /// Load configuration from a benchScale topology TOML file.
    ///
    /// Looks for `benchScale/topologies/{name}.toml` relative to the workspace
    /// root. Parses the local gate's env overrides and populates remote peers
    /// from other gates in the topology.
    #[must_use]
    pub fn from_topology_file(name: &str) -> Option<Self> {
        let candidates = [
            format!("benchScale/topologies/{name}.toml"),
            format!("../benchScale/topologies/{name}.toml"),
        ];

        let content = candidates
            .iter()
            .find_map(|p| std::fs::read_to_string(p).ok())?;
        let parsed: toml::Value = toml::from_str(&content).ok()?;

        let gates_table = parsed.get("gates")?.as_table()?;
        let local_gate = std::env::var(crate::env_keys::GATE_ID)
            .or_else(|_| std::env::var(crate::env_keys::HOSTNAME))
            .unwrap_or_else(|_| {
                crate::tolerances::platform::hostname().unwrap_or_else(|| "unknown-gate".to_owned())
            });

        let local_key = gates_table
            .keys()
            .find(|k| k.replace('-', "") == local_gate.replace('-', ""))
            .cloned()
            .unwrap_or_else(|| local_gate.clone());

        let mut remote_gates = BTreeMap::new();
        for (gate_id, gate_val) in gates_table {
            if gate_id == &local_key {
                continue;
            }
            let addr = gate_val
                .get("address")
                .and_then(toml::Value::as_str)
                .unwrap_or(crate::tolerances::platform::DEFAULT_HOST);
            let port = gate_val
                .get("songbird_port")
                .and_then(toml::Value::as_integer)
                .unwrap_or_else(|| i64::from(crate::tolerances::FEDERATION_PORT));
            remote_gates.insert(gate_id.clone(), format!("{addr}:{port}"));
        }

        let family_id = std::env::var(crate::env_keys::FAMILY_ID)
            .ok()
            .filter(|s| !s.is_empty() && s != "default");
        let btsp_available =
            family_id.is_some() && crate::env_keys::resolve_family_seed().is_some();

        Some(Self {
            local_gate: local_key,
            remote_gates,
            btsp_available,
            family_id,
            connect_timeout: Duration::from_secs(crate::tolerances::TCP_CONNECT_TIMEOUT_SECS),
        })
    }

    /// Whether the minimum requirements for live cross-gate testing are met.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        !self.remote_gates.is_empty() && self.btsp_available
    }

    /// Whether basic cross-gate connectivity (without BTSP) can be tested.
    #[must_use]
    pub fn is_connectable(&self) -> bool {
        !self.remote_gates.is_empty()
    }

    /// Check reachability of all configured remote gates.
    #[must_use]
    pub fn check_readiness(&self) -> Vec<GateReadiness> {
        self.remote_gates
            .iter()
            .map(|(gate_id, address)| {
                let tcp_reachable = check_tcp_reachable(address, self.connect_timeout);
                let songbird_responding = tcp_reachable && check_songbird_http(address);
                let btsp_ready = songbird_responding && self.btsp_available;

                GateReadiness {
                    gate_id: gate_id.clone(),
                    address: address.clone(),
                    tcp_reachable,
                    songbird_responding,
                    btsp_ready,
                }
            })
            .collect()
    }

    /// Summary suitable for validation output.
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "local={}, remotes={}, btsp={}, family_id={}",
            self.local_gate,
            self.remote_gates.len(),
            if self.btsp_available { "yes" } else { "no" },
            self.family_id.as_deref().unwrap_or("none"),
        )
    }
}

/// Parse mesh peer env vars into `gate_id` → address map.
///
/// Reads `MESH_PEERS` first, falling back to deprecated `SONGBIRD_PEERS`.
/// Format: `gate_id=host:port,gate_id=host:port` or `host:port,host:port`
#[expect(
    deprecated,
    reason = "SONGBIRD_PEERS fallback for backward compatibility"
)]
fn parse_songbird_peers() -> BTreeMap<String, String> {
    let mut peers = BTreeMap::new();

    let val = match std::env::var(crate::env_keys::MESH_PEERS)
        .or_else(|_| std::env::var(crate::env_keys::SONGBIRD_PEERS))
    {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return peers;
        }
    };

    for (i, entry) in val.split(',').enumerate() {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        if let Some((gate_id, addr)) = entry.split_once('=') {
            peers.insert(gate_id.to_owned(), addr.to_owned());
        } else {
            peers.insert(format!("peer-{i}"), entry.to_owned());
        }
    }

    peers
}

fn check_tcp_reachable(address: &str, timeout: Duration) -> bool {
    address
        .parse::<std::net::SocketAddr>()
        .ok()
        .is_some_and(|addr| std::net::TcpStream::connect_timeout(&addr, timeout).is_ok())
}

fn check_songbird_http(address: &str) -> bool {
    use std::io::{Read, Write};

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "discovery.peers",
        "params": {},
        "id": 1
    });

    std::thread::scope(|s| {
        let handle = s.spawn(|| {
            let stream = std::net::TcpStream::connect_timeout(
                &address.parse().ok()?,
                Duration::from_secs(crate::tolerances::TCP_CONNECT_TIMEOUT_SECS),
            ).ok()?;
            stream.set_write_timeout(Some(Duration::from_secs(crate::tolerances::TCP_WRITE_TIMEOUT_SECS))).ok()?;
            stream.set_read_timeout(Some(Duration::from_secs(crate::tolerances::TCP_READ_TIMEOUT_SECS))).ok()?;

            let body_str = body.to_string();
            let request = format!(
                "POST /jsonrpc HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body_str}",
                body_str.len()
            );

            let mut stream = stream;
            stream.write_all(request.as_bytes()).ok()?;
            let mut response = String::new();
            stream.read_to_string(&mut response).ok()?;
            Some(response.contains("jsonrpc") || response.contains("200 OK"))
        });

        handle.join().ok().flatten().unwrap_or(false)
    })
}

/// Construct a cross-gate `capability.call` JSON-RPC payload.
#[must_use]
pub fn build_cross_gate_call(
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
    target_gate: Option<&str>,
) -> serde_json::Value {
    let mut payload = serde_json::json!({
        "capability": capability,
        "operation": operation,
        "args": args,
    });

    if let Some(gate) = target_gate {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("gate".to_owned(), serde_json::json!(gate));
        }
    }

    payload
}

/// Construct an authenticated `capability.call` payload with BTSP token.
#[must_use]
pub fn build_authenticated_call(
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
    target_gate: Option<&str>,
    bearer_token: &str,
) -> serde_json::Value {
    let mut payload = build_cross_gate_call(capability, operation, args, target_gate);

    if let Some(obj) = payload
        .get_mut("args")
        .and_then(serde_json::Value::as_object_mut)
    {
        obj.insert("_bearer_token".to_owned(), serde_json::json!(bearer_token));
    }

    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_env_does_not_panic() {
        let config = LiveMeshConfig::from_env();
        assert!(!config.local_gate.is_empty());
    }

    #[test]
    fn no_hardcoded_peers_when_env_unset() {
        let peers = parse_songbird_peers();
        if std::env::var("SONGBIRD_PEERS").is_err() {
            assert!(
                peers.is_empty(),
                "should return empty peers when SONGBIRD_PEERS env unset (no hardcoded fallback)"
            );
        }
    }

    #[test]
    fn build_call_without_gate() {
        let payload = build_cross_gate_call("security", "health", &serde_json::json!({}), None);
        assert_eq!(payload["capability"], "security");
        assert!(payload.get("gate").is_none());
    }

    #[test]
    fn build_call_with_gate() {
        let payload = build_cross_gate_call(
            "compute",
            "dispatch",
            &serde_json::json!({}),
            Some("strand-gate"),
        );
        assert_eq!(payload["gate"], "strand-gate");
    }

    #[test]
    fn build_authenticated_injects_token() {
        let payload = build_authenticated_call(
            "security",
            "auth.verify_ionic",
            &serde_json::json!({"token": "test"}),
            Some("strand-gate"),
            "my-bearer-token",
        );
        assert_eq!(payload["args"]["_bearer_token"], "my-bearer-token");
    }

    #[test]
    fn summary_format() {
        let config = LiveMeshConfig::from_env();
        let summary = config.summary();
        assert!(summary.contains("local="));
        assert!(summary.contains("btsp="));
    }

    #[test]
    fn unreachable_gate_returns_false() {
        assert!(!check_tcp_reachable(
            "192.168.99.99:7700",
            Duration::from_millis(100)
        ));
    }

    #[test]
    fn topology_file_loads_when_present() {
        if let Some(cfg) = LiveMeshConfig::from_topology_file("cross_gate_trust") {
            assert!(
                cfg.remote_gates.contains_key("strand-gate")
                    || cfg.remote_gates.contains_key("east-gate"),
                "topology should populate remote gates"
            );
        }
    }
}
