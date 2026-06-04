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
    /// Remote gate endpoints (gate_id → host:port).
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
    /// Build configuration from environment variables and well-known defaults.
    ///
    /// Reads:
    /// - `HOSTNAME` or `GATE_ID` for local gate identity
    /// - `SONGBIRD_PEERS` for remote gate addresses
    /// - `FAMILY_ID` / `FAMILY_SEED` for BTSP readiness
    #[must_use]
    pub fn from_env() -> Self {
        let local_gate = std::env::var("GATE_ID")
            .or_else(|_| std::env::var("HOSTNAME"))
            .unwrap_or_else(|_| "east-gate".to_owned());

        let remote_gates = parse_songbird_peers();

        let family_id = std::env::var(crate::env_keys::FAMILY_ID)
            .ok()
            .filter(|s| !s.is_empty() && s != "default");

        let btsp_available = family_id.is_some()
            && crate::env_keys::resolve_family_seed().is_some();

        Self {
            local_gate,
            remote_gates,
            btsp_available,
            family_id,
            connect_timeout: Duration::from_secs(3),
        }
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

/// Parse `SONGBIRD_PEERS` env var into gate_id → address map.
///
/// Format: `gate_id=host:port,gate_id=host:port` or `host:port,host:port`
fn parse_songbird_peers() -> BTreeMap<String, String> {
    let mut peers = BTreeMap::new();

    let val = match std::env::var("SONGBIRD_PEERS") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            let well_known = [
                ("strand-gate", "192.168.1.132:7700"),
                ("iron-gate", "192.168.1.238:7700"),
            ];
            for (gate, addr) in well_known {
                peers.insert(gate.to_owned(), addr.to_owned());
            }
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
        .is_some_and(|addr| {
            std::net::TcpStream::connect_timeout(&addr, timeout).is_ok()
        })
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
                Duration::from_secs(2),
            ).ok()?;
            stream.set_write_timeout(Some(Duration::from_secs(2))).ok()?;
            stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;

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

    if let Some(obj) = payload.get_mut("args").and_then(serde_json::Value::as_object_mut) {
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
    fn well_known_peers_fallback() {
        let peers = parse_songbird_peers();
        if std::env::var("SONGBIRD_PEERS").is_err() {
            assert!(
                peers.contains_key("strand-gate"),
                "should fall back to well-known peers when env unset"
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
        let payload =
            build_cross_gate_call("compute", "dispatch", &serde_json::json!({}), Some("strand-gate"));
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
}
