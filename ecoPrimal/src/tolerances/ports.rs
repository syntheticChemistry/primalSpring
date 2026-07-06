// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Port registry and TCP fallback constants.
//!
//! All port assignments derive from `config/ports.toml` (single source of truth).
//! The TOML-driven registry is the authoritative runtime path; static constants
//! remain for backward compatibility in contexts requiring `const`.

use crate::primal_names;

/// Per-primal port metadata: slug, TCP fallback, env override key.
///
/// Derived at init time from `config/ports.toml` — adding a new primal
/// means adding one `[primal]` section to the TOML.
pub struct PortEntry {
    /// Lowercase primal slug (e.g. `"beardog"`).
    pub slug: &'static str,
    /// TCP fallback port (Tier 5, debug-only since Wave 79).
    pub port: u16,
    /// Env var name for port override (e.g. `"BEARDOG_PORT"`).
    pub env_key: &'static str,
}

/// Embedded ports TOML (single source of truth for port assignments).
const PORTS_TOML: &str = include_str!("../../../config/ports.toml");

/// TOML-derived port registry, built once at first access.
static TOML_PORT_REGISTRY: std::sync::LazyLock<Vec<PortEntryOwned>> =
    std::sync::LazyLock::new(|| {
        let Ok(parsed) = PORTS_TOML.parse::<toml::Table>() else {
            return Vec::new();
        };
        let mut entries = Vec::new();
        for (slug, section) in &parsed {
            if slug == "federation" {
                continue;
            }
            let Some(table) = section.as_table() else {
                continue;
            };
            let Some(port) = table
                .get("port")
                .and_then(toml::Value::as_integer)
                .and_then(|p| u16::try_from(p).ok())
            else {
                continue;
            };
            let Some(env_key) = table.get("env_key").and_then(|v| v.as_str()) else {
                continue;
            };
            entries.push(PortEntryOwned {
                slug: slug.clone(),
                port,
                _env_key: env_key.to_owned(),
            });
        }
        entries.sort_by(|a, b| a.slug.cmp(&b.slug));
        entries
    });

/// Owned version of [`PortEntry`] stored in the TOML-derived registry.
struct PortEntryOwned {
    slug: String,
    port: u16,
    _env_key: String,
}

/// Look up a primal's port entry from the static registry.
///
/// Static registry is the primary lookup path for callers needing
/// `&'static PortEntry`. TOML drift is caught by compile-time tests.
#[must_use]
#[expect(deprecated)]
pub fn port_entry_for(primal: &str) -> Option<&'static PortEntry> {
    PORT_REGISTRY.iter().find(|e| e.slug == primal)
}

#[expect(deprecated)]
fn slug_list_fallback_from_static_registry() -> Vec<String> {
    PORT_REGISTRY.iter().map(|e| e.slug.to_owned()).collect()
}

/// Stable slug list derived once from TOML (no per-call leaking).
static SLUG_LIST: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    if TOML_PORT_REGISTRY.is_empty() {
        slug_list_fallback_from_static_registry()
    } else {
        TOML_PORT_REGISTRY.iter().map(|e| e.slug.clone()).collect()
    }
});

/// Return all primal slugs known to the TOML-derived port registry.
///
/// Prefers the runtime TOML data; falls back to the static `PORT_REGISTRY`
/// if TOML parsing failed. This enables capability-based enumeration
/// without hardcoding the 13-primal list at each call site.
#[must_use]
pub fn all_primal_slugs() -> Vec<&'static str> {
    SLUG_LIST.iter().map(String::as_str).collect()
}

/// Resolve a primal's default TCP port from the port registry.
#[must_use]
pub fn default_port_for(primal: &str) -> u16 {
    toml_port_for(primal).unwrap_or(0)
}

/// Resolve the env var key for a primal's port override.
#[must_use]
pub fn port_env_key_for(primal: &str) -> &'static str {
    port_entry_for(primal).map_or("", |e| e.env_key)
}

/// Look up a port from the TOML-derived registry (no static leaking).
fn toml_port_for(primal: &str) -> Option<u16> {
    TOML_PORT_REGISTRY
        .iter()
        .find(|e| e.slug == primal)
        .map(|e| e.port)
}

/// Static port registry — backward compatibility only.
///
/// **Prefer [`default_port_for`] and [`all_primal_slugs`]** for new code.
/// These constants duplicate `config/ports.toml`; drift is caught by tests.
/// Port assignments confirmed against ironGate live deployment (2026-05-04).
#[deprecated(
    since = "0.9.33",
    note = "use default_port_for() / all_primal_slugs() instead of static PORT_REGISTRY"
)]
pub static PORT_REGISTRY: &[PortEntry] = &[
    PortEntry {
        slug: primal_names::BEARDOG,
        port: 9100,
        env_key: "BEARDOG_PORT",
    },
    PortEntry {
        slug: primal_names::SONGBIRD,
        port: 9200,
        env_key: "SONGBIRD_PORT",
    },
    PortEntry {
        slug: primal_names::SQUIRREL,
        port: 9300,
        env_key: "SQUIRREL_PORT",
    },
    PortEntry {
        slug: primal_names::TOADSTOOL,
        port: 9400,
        env_key: "TOADSTOOL_PORT",
    },
    PortEntry {
        slug: primal_names::NESTGATE,
        port: 9500,
        env_key: "NESTGATE_PORT",
    },
    PortEntry {
        slug: primal_names::RHIZOCRYPT,
        port: 9601,
        env_key: "RHIZOCRYPT_PORT",
    },
    PortEntry {
        slug: primal_names::LOAMSPINE,
        port: 9700,
        env_key: "LOAMSPINE_PORT",
    },
    PortEntry {
        slug: primal_names::CORALREEF,
        port: 9730,
        env_key: "CORALREEF_PORT",
    },
    PortEntry {
        slug: primal_names::BARRACUDA,
        port: 9740,
        env_key: "BARRACUDA_PORT",
    },
    PortEntry {
        slug: primal_names::SKUNKBAT,
        port: 9140,
        env_key: "SKUNKBAT_PORT",
    },
    PortEntry {
        slug: primal_names::BIOMEOS,
        port: 9800,
        env_key: "BIOMEOS_PORT",
    },
    PortEntry {
        slug: primal_names::SWEETGRASS,
        port: 9850,
        env_key: "SWEETGRASS_PORT",
    },
    PortEntry {
        slug: primal_names::PETALTONGUE,
        port: 9900,
        env_key: "PETALTONGUE_PORT",
    },
];

/// Federation / CNS port assignments — deployment-profile variants.
///
/// These are separate from the canonical PORT_REGISTRY because they are
/// per-deployment-profile (nucleus01 vs primalspring01) rather than
/// per-primal. Federation ports are Songbird mesh coordination endpoints;
/// CNS ports are profile-specific crypto/defense RPC endpoints.
/// Authoritative source: `config/ports.toml [federation.*]`.
pub struct FederationPort {
    /// Deployment profile (e.g. "nucleus01", "primalspring01").
    pub profile: &'static str,
    /// Primal slug.
    pub primal: &'static str,
    /// Role description.
    pub role: &'static str,
    /// Port number.
    pub port: u16,
    /// Whether this port is droppable (UDS migration candidate).
    pub droppable: bool,
}

/// Known federation and CNS ports across deployment profiles.
pub static FEDERATION_PORTS: &[FederationPort] = &[
    FederationPort {
        profile: "nucleus01",
        primal: primal_names::SONGBIRD,
        role: "federation",
        port: 7700,
        droppable: false,
    },
    FederationPort {
        profile: "primalspring01",
        primal: primal_names::SONGBIRD,
        role: "federation",
        port: 7701,
        droppable: false,
    },
];

/// Default federation port for discovery provider mesh coordination.
///
/// Protocol-level constant — standard across all gates. This is a protocol
/// contract, not a primal-specific port assignment. Currently implemented
/// by Songbird, but any discovery provider uses this port.
///
/// Authoritative: `config/ports.toml` federation section.
pub const FEDERATION_PORT: u16 = 7700;

/// Legacy alias — prefer [`FEDERATION_PORT`].
#[deprecated(since = "0.9.31", note = "use FEDERATION_PORT instead")]
pub const SONGBIRD_FEDERATION_PORT: u16 = FEDERATION_PORT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(deprecated)]
    fn static_registry_matches_toml() {
        for entry in PORT_REGISTRY {
            let toml_port = toml_port_for(entry.slug);
            assert_eq!(
                toml_port,
                Some(entry.port),
                "DRIFT: static PORT_REGISTRY says {}={} but ports.toml says {:?}",
                entry.slug,
                entry.port,
                toml_port
            );
        }
    }

    #[test]
    #[expect(deprecated)]
    fn toml_registry_covers_all_primals() {
        assert!(
            !TOML_PORT_REGISTRY.is_empty(),
            "ports.toml failed to parse — TOML registry is empty"
        );
        assert!(
            TOML_PORT_REGISTRY.len() >= PORT_REGISTRY.len(),
            "TOML has fewer entries ({}) than static ({})",
            TOML_PORT_REGISTRY.len(),
            PORT_REGISTRY.len()
        );
    }

    #[test]
    #[expect(deprecated)]
    fn no_port_collisions() {
        let mut seen: std::collections::HashMap<u16, &str> = std::collections::HashMap::new();
        for entry in PORT_REGISTRY {
            if let Some(existing) = seen.insert(entry.port, entry.slug) {
                panic!(
                    "PORT COLLISION: {} and {} both claim port {}",
                    existing, entry.slug, entry.port
                );
            }
        }
    }
}
