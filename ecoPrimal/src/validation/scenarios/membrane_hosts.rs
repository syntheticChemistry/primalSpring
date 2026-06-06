// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared membrane host resolution for sovereignty validation scenarios.
//!
//! Authoritative source: `config/membrane_hosts.toml`.
//! Env vars (`MEMBRANE_NS1`, `MEMBRANE_NS2`, `MEMBRANE_RELAY`) override
//! TOML values for local testing or infrastructure migrations.

use std::sync::LazyLock;

static HOSTS_TOML: &str = include_str!("../../../../config/membrane_hosts.toml");

#[allow(dead_code)]
struct MembraneHosts {
    ns1: String,
    ns2: String,
    relay: String,
    inner_records: Vec<(String, String)>,
    content_records: Vec<(String, String)>,
    sovereign_records: Vec<(String, String)>,
}

static HOSTS: LazyLock<MembraneHosts> = LazyLock::new(|| {
    let parsed: toml::Table = HOSTS_TOML.parse().unwrap_or_default();

    let ns_table = parsed.get("nameservers").and_then(|v| v.as_table());
    let hosts_table = parsed.get("hosts").and_then(|v| v.as_table());

    let toml_ns1 = ns_table
        .and_then(|t| t.get("ns1"))
        .and_then(|v| v.as_str())
        .unwrap_or("127.0.0.1");
    let toml_ns2 = ns_table
        .and_then(|t| t.get("ns2"))
        .and_then(|v| v.as_str())
        .unwrap_or("127.0.0.1");
    let toml_relay = hosts_table
        .and_then(|t| t.get("relay"))
        .and_then(|v| v.as_str())
        .unwrap_or("127.0.0.1");

    let ns1 = std::env::var("MEMBRANE_NS1").unwrap_or_else(|_| toml_ns1.to_owned());
    let ns2 = std::env::var("MEMBRANE_NS2").unwrap_or_else(|_| toml_ns2.to_owned());
    let relay = std::env::var("MEMBRANE_RELAY").unwrap_or_else(|_| toml_relay.to_owned());

    let load_records = |key: &str| -> Vec<(String, String)> {
        parsed
            .get(key)
            .and_then(|v| v.as_table())
            .map(|t| {
                t.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
                    .collect()
            })
            .unwrap_or_default()
    };

    MembraneHosts {
        ns1,
        ns2,
        relay,
        inner_records: load_records("inner_records"),
        content_records: load_records("content_records"),
        sovereign_records: load_records("sovereign_records"),
    }
});

/// Primary sovereign nameserver IP.
pub fn ns1_ip() -> &'static str {
    &HOSTS.ns1
}

/// Secondary sovereign nameserver IP.
pub fn ns2_ip() -> &'static str {
    &HOSTS.ns2
}

/// Relay VPS IP.
#[allow(dead_code)]
pub fn relay_ip() -> &'static str {
    &HOSTS.relay
}

/// Expected inner membrane DNS records (domain, IP).
pub fn inner_records() -> &'static [(String, String)] {
    &HOSTS.inner_records
}

/// Expected content layer DNS records (domain, IP).
pub fn content_records() -> &'static [(String, String)] {
    &HOSTS.content_records
}

/// Expected sovereign DNS records (domain, IP).
pub fn sovereign_records() -> &'static [(String, String)] {
    &HOSTS.sovereign_records
}
