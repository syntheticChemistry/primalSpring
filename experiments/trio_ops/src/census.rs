// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared port and socket census tables for trio audit experiments (exp112–114).
//!
//! Data is derived from `config/ports.toml` and `config/capability_registry.toml`
//! via primalspring tolerances/composition APIs — no hardcoded port or socket lists.

use std::sync::LazyLock;

use primalspring::composition::{ALL_CAPS, capability_to_primal};
use primalspring::tolerances::ports::{all_primal_slugs, default_port_for};

const PORTS_TOML: &str = include_str!("../../../config/ports.toml");

/// One TCP port entry in the deployment census.
#[derive(Clone)]
pub struct PortCensusEntry {
    /// Owning primal slug (e.g. `"songbird"`).
    pub primal: &'static str,
    /// TCP port number.
    pub port: u16,
    /// Human-readable role / migration status (e.g. `"federation (nucleus01) — KEEP"`).
    pub status: &'static str,
}

/// Maps a primary capability domain to its flat biomeOS socket filename.
#[derive(Clone)]
pub struct SocketOwnershipEntry {
    /// Primary capability domain from the registry (e.g. `"crypto"`).
    pub capability: &'static str,
    /// Flat socket basename under `$XDG_RUNTIME_DIR/biomeos/` (e.g. `"crypto.sock"`).
    pub socket_suffix: &'static str,
}

static PORT_CENSUS: LazyLock<Vec<PortCensusEntry>> = LazyLock::new(build_port_census);
static SOCKET_OWNERSHIP: LazyLock<Vec<SocketOwnershipEntry>> =
    LazyLock::new(build_socket_ownership_map);

fn leak_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn build_port_census() -> Vec<PortCensusEntry> {
    let mut entries = Vec::new();

    for slug in all_primal_slugs() {
        let port = default_port_for(slug);
        if port > 0 {
            entries.push(PortCensusEntry {
                primal: slug,
                port,
                status: "tier5_tcp_fallback",
            });
        }
    }

    let Ok(parsed) = PORTS_TOML.parse::<toml::Table>() else {
        return entries;
    };

    if let Some(federation) = parsed.get("federation").and_then(toml::Value::as_table) {
        for (profile, section) in federation {
            let Some(table) = section.as_table() else {
                continue;
            };
            push_federation_entry(&mut entries, profile, table);
        }
    }

    for (key, section) in &parsed {
        if !key.starts_with("federation.") {
            continue;
        }
        let Some(table) = section.as_table() else {
            continue;
        };
        let profile = key.strip_prefix("federation.").unwrap_or(key.as_str());
        push_federation_entry(&mut entries, profile, table);
    }

    entries
}

fn push_federation_entry(entries: &mut Vec<PortCensusEntry>, profile: &str, table: &toml::Table) {
    let Some(port) = table
        .get("port")
        .and_then(toml::Value::as_integer)
        .and_then(|p| u16::try_from(p).ok())
    else {
        return;
    };
    let primal = table
        .get("primal")
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| "unknown".to_owned());
    let role = table
        .get("role")
        .and_then(toml::Value::as_str)
        .unwrap_or("unknown");
    let droppable = table
        .get("droppable")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    let migration = if droppable { "DROPPABLE" } else { "KEEP" };
    let status = format!("{role} ({profile}) — {migration}");

    entries.push(PortCensusEntry {
        primal: leak_str(primal),
        port,
        status: leak_str(status),
    });
}

fn build_socket_ownership_map() -> Vec<SocketOwnershipEntry> {
    ALL_CAPS
        .iter()
        .map(|&capability| SocketOwnershipEntry {
            capability,
            socket_suffix: leak_str(format!("{capability}.sock")),
        })
        .collect()
}

/// Return the full port census: tier-5 TCP fallbacks plus federation/CNS ports.
#[must_use]
pub fn port_census() -> Vec<PortCensusEntry> {
    (*PORT_CENSUS).clone()
}

/// Return federation/CNS audit ports (excludes tier-5 per-primal fallbacks).
#[must_use]
pub fn federation_port_census() -> Vec<PortCensusEntry> {
    port_census()
        .into_iter()
        .filter(|entry| entry.status != "tier5_tcp_fallback")
        .collect()
}

/// Whether a port census entry is a UDS migration candidate (Tower CNS droppable).
#[must_use]
pub fn port_is_droppable(entry: &PortCensusEntry) -> bool {
    entry.status.ends_with("— DROPPABLE")
}

/// Build the capability → flat socket ownership map from primary registry domains.
#[must_use]
pub fn socket_ownership_map() -> Vec<SocketOwnershipEntry> {
    (*SOCKET_OWNERSHIP).clone()
}

/// Resolve the owning primal for a socket ownership entry.
#[must_use]
pub fn socket_owner(entry: &SocketOwnershipEntry) -> &str {
    capability_to_primal(entry.capability)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_census_includes_all_primals() {
        let census = port_census();
        let slugs = all_primal_slugs();
        for slug in slugs {
            assert!(
                census.iter().any(|e| e.primal == slug && e.port == default_port_for(slug)),
                "missing tier5 entry for {slug}",
            );
        }
    }

    #[test]
    fn federation_ports_match_toml() {
        let federation = federation_port_census();
        assert!(federation.iter().any(|e| e.port == 7700 && e.primal == "songbird"));
        assert!(federation.iter().any(|e| e.port == 7701 && e.primal == "songbird"));
        assert!(federation.iter().any(|e| e.port == 9900 && e.primal == "beardog"));
        assert!(federation.iter().any(|e| e.port == 9101 && e.primal == "beardog"));
        assert!(federation.iter().any(|e| e.port == 9750 && e.primal == "skunkbat"));
    }

    #[test]
    fn socket_ownership_derived_from_all_caps() {
        let map = socket_ownership_map();
        assert!(!map.is_empty());
        assert!(map.iter().all(|e| e.socket_suffix.ends_with(".sock")));
        assert!(map.iter().any(|e| e.capability == "discovery"));
        assert!(map.iter().any(|e| e.capability == "security"));
    }
}
