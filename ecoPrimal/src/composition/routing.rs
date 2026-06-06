// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability routing tables — maps capabilities to primals and methods to domains.
//!
//! The domain→owner mapping is derived from `config/capability_registry.toml`
//! (parsed once at init via [`LazyLock`]). This eliminates hand-maintained
//! duplication between code tables and the canonical TOML.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::primal_names::Primal;

/// A validated capability domain key used for routing and client lookup.
///
/// Wraps a `String` to provide type safety for capability domain names
/// (e.g. "security", "tensor", "ai"). Implements `Borrow<str>` so
/// `HashMap<CapabilityDomain, V>` supports `.get("str")` lookups without
/// allocating.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(transparent)]
pub struct CapabilityDomain(String);

impl CapabilityDomain {
    /// Create a new `CapabilityDomain` from any string-like value.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// View the inner string as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CapabilityDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::ops::Deref for CapabilityDomain {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for CapabilityDomain {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CapabilityDomain {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for CapabilityDomain {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for CapabilityDomain {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Embedded canonical capability registry (single source of truth).
const REGISTRY_TOML: &str = include_str!("../../../config/capability_registry.toml");

/// Parsed domain→owner slug mapping from the capability registry.
///
/// Built once at first access. Each TOML section `[domain] owner = "primal"`
/// becomes an entry. Sections with `owner = "all"`, `"none"`, or `"tests"`
/// are excluded.
static DOMAIN_OWNER_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let Ok(parsed) = REGISTRY_TOML.parse::<toml::Table>() else {
        return map;
    };
    for (domain, section) in &parsed {
        if domain.starts_with("compositions") || domain == "test_fixtures" || domain == "false_positives" {
            continue;
        }
        if let Some(owner) = section.get("owner").and_then(|v| v.as_str()) {
            if owner != "all" && owner != "none" && owner != "tests" {
                map.insert(domain.clone(), owner.to_owned());
            }
        }
    }
    map
});

/// All NUCLEUS capabilities that primalSpring discovers and authenticates.
///
/// These are the *primary* discovery domains — one per primal that
/// `CompositionContext::discover()` iterates. Every entry must resolve to a
/// primal provider via `capability_to_primal()`.
///
/// Consistency with `capability_registry.toml` is enforced by
/// `s_routing_consistency` (every registered method routes through
/// `method_to_capability_domain` + `capability_to_primal` to its declared owner)
/// and by `all_caps_resolve_to_primals` (every entry here resolves to a Primal).
pub const ALL_CAPS: &[&str] = &[
    "security",
    "discovery",
    "compute",
    "tensor",
    "shader",
    "storage",
    "content",
    "ai",
    "dag",
    "commit",
    "visualization",
    "ledger",
    "attribution",
    "defense",
    "orchestration",
];

/// Extended capability aliases for BTSP proactive escalation.
///
/// Includes names that map to the same primal sockets as [`ALL_CAPS`]
/// (e.g. `inference` → Squirrel, `spine`/`merkle` → loamSpine) to ensure
/// BTSP coverage even when a client was connected under an alias name.
pub const BTSP_EXTRA_CAPS: &[&str] = &[
    "inference",
    "spine",
    "merkle",
    "braid",
    "recon",
    "threat",
    "lineage",
];

/// Map a capability domain to its canonical primal provider.
///
/// Resolves from the parsed `capability_registry.toml` first, then falls
/// through to spring/app owners. This is the ecosystem's single source of
/// truth for "which primal owns which capability domain."
///
/// ```
/// assert_eq!(primalspring::composition::capability_to_primal("tensor"), "barracuda");
/// assert_eq!(primalspring::composition::capability_to_primal("crypto"), "beardog");
/// assert_eq!(primalspring::composition::capability_to_primal("storage"), "nestgate");
/// assert_eq!(primalspring::composition::capability_to_primal("content"), "nestgate");
/// ```
#[must_use]
pub fn capability_to_primal(capability: &str) -> &str {
    capability_to_primal_typed(capability).map_or_else(
        || capability_to_spring_owner(capability),
        |p| p.slug(),
    )
}

/// Resolve non-primal capability owners (springs and apps).
///
/// Falls through to the capability name itself if no spring claims it.
fn capability_to_spring_owner(capability: &str) -> &str {
    use crate::primal_names::Spring;
    if let Some(owner) = DOMAIN_OWNER_MAP.get(capability) {
        return leak_or_match(owner);
    }
    match capability {
        "tool" | "primalspring" | "coordination" | "bonding" | "composition" | "mcp" => {
            Spring::PrimalSpring.slug()
        }
        "game" => Spring::LudoSpring.slug(),
        "science" => Spring::NeuralSpring.slug(),
        "webb" | "esotericwebb" => "esotericwebb",
        other => other,
    }
}

/// Typed version — consults the TOML-derived domain→owner map.
///
/// All aliases (`orchestration`, `commit`, `ledger`, `merkle`, etc.) are
/// resolved from `capability_registry.toml` alias sections — no static
/// fallback table needed.
///
/// Returns `None` for non-primal targets (springs, unknown).
#[must_use]
pub fn capability_to_primal_typed(capability: &str) -> Option<Primal> {
    DOMAIN_OWNER_MAP
        .get(capability)
        .and_then(|owner| owner.parse::<Primal>().ok())
}

/// Return a `&'static str` for known owner slugs via enum-driven resolution.
///
/// Resolves through [`Primal`] and [`Spring`] enums first. For non-enum
/// owners (apps, membranes), falls back to a static match table. Truly
/// unknown owners are leaked once with a tracing warning so callers never
/// silently receive `"unknown"`.
fn leak_or_match(owner: &str) -> &'static str {
    use crate::primal_names::Spring;
    if let Ok(p) = owner.parse::<Primal>() {
        return p.slug();
    }
    if let Ok(s) = owner.parse::<Spring>() {
        return s.slug();
    }
    match owner {
        "esotericwebb" => "esotericwebb",
        "membrane" => "membrane",
        other => {
            tracing::warn!(owner = other, "TOML owner not matched by Primal or Spring enum — leaking");
            Box::leak(other.to_owned().into_boxed_str())
        }
    }
}

/// Primal→home composition tier, derived from `[compositions.*].primals`
/// in `capability_registry.toml`.
///
/// For each composition, the primals list is mapped to the tier. When a
/// primal appears in multiple compositions, it gets the *smallest* tier
/// (Tower < Node < Nest < Meta < Orchestration). This replaces the
/// hardcoded primal-name match arms in `neural_routing::CompositionTier::from_domain`.
static PRIMAL_HOME_TIER: LazyLock<HashMap<String, u8>> = LazyLock::new(|| {
    let Ok(parsed) = REGISTRY_TOML.parse::<toml::Table>() else {
        return HashMap::new();
    };
    let Some(compositions) = parsed.get("compositions").and_then(|v| v.as_table()) else {
        return HashMap::new();
    };
    let tier_priority = |name: &str| -> u8 {
        match name {
            "tower" => 0,
            "node" => 1,
            "nest" | "rootpulse" => 2,
            "meta" => 3,
            _ => 4,
        }
    };
    let mut map: HashMap<String, u8> = HashMap::new();
    for (comp_name, comp_val) in compositions {
        let priority = tier_priority(comp_name);
        if let Some(primals) = comp_val.get("primals").and_then(|v| v.as_array()) {
            for p in primals {
                if let Some(slug) = p.as_str() {
                    map.entry(slug.to_owned())
                        .and_modify(|existing| {
                            if priority < *existing {
                                *existing = priority;
                            }
                        })
                        .or_insert(priority);
                }
            }
        }
    }
    map
});

/// Look up a primal's home composition tier from the TOML-derived map.
///
/// Returns the tier priority (0=Tower, 1=Node, 2=Nest, 3=Meta, 4=Orchestration).
/// Returns `None` for primals not listed in any composition.
#[must_use]
pub fn primal_home_tier_priority(primal: &str) -> Option<u8> {
    PRIMAL_HOME_TIER.get(primal).copied()
}

/// Prefix→routing-domain map derived from `capability_registry.toml`.
///
/// For each TOML section with methods:
/// 1. If `routes_to` is specified, use it (explicit routing override).
/// 2. If the section name is already an ALL_CAPS discovery domain, route to itself.
/// 3. Otherwise, find the owner primal's primary ALL_CAPS domain by inverting
///    the capability→owner map.
///
/// This replaces 30+ hand-maintained match arms with a single TOML-derived table.
static PREFIX_ROUTING: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let all_caps_set: std::collections::HashSet<&str> = ALL_CAPS.iter().copied().collect();

    // Build owner → primary ALL_CAPS domain (inverted from DOMAIN_OWNER_MAP)
    let mut owner_primary: HashMap<String, String> = HashMap::new();
    for &cap in ALL_CAPS {
        if let Some(owner) = DOMAIN_OWNER_MAP.get(cap) {
            owner_primary.entry(owner.clone()).or_insert_with(|| cap.to_owned());
        }
    }

    let Ok(parsed) = REGISTRY_TOML.parse::<toml::Table>() else {
        return HashMap::new();
    };

    let mut map = HashMap::new();
    for (section, val) in &parsed {
        if section.starts_with("compositions")
            || section == "test_fixtures"
            || section == "false_positives"
        {
            continue;
        }
        let Some(table) = val.as_table() else { continue };

        // Explicit routes_to takes highest priority
        if let Some(rt) = table.get("routes_to").and_then(|v| v.as_str()) {
            map.insert(section.clone(), rt.to_owned());
            continue;
        }

        // Section name is already a discovery domain — routes to itself
        if all_caps_set.contains(section.as_str()) {
            map.insert(section.clone(), section.clone());
            continue;
        }

        // Derive from owner's primary ALL_CAPS domain
        if let Some(owner) = table.get("owner").and_then(|v| v.as_str()) {
            if owner == "all" || owner == "none" || owner == "tests" {
                continue;
            }
            if let Some(primary) = owner_primary.get(owner) {
                map.insert(section.clone(), primary.clone());
            }
        }
    }
    map
});

/// Full-method overrides derived from `capability_registry.toml`.
///
/// For methods listed under a TOML section whose name differs from the method
/// prefix (e.g. `security.audit_log` listed under `[defense]`), we build an
/// exact-method → routing-domain map so cross-domain methods route correctly.
static METHOD_OVERRIDES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let Ok(parsed) = REGISTRY_TOML.parse::<toml::Table>() else {
        return HashMap::new();
    };

    let mut overrides = HashMap::new();
    for (section, val) in &parsed {
        if section.starts_with("compositions")
            || section == "test_fixtures"
            || section == "false_positives"
        {
            continue;
        }
        let Some(table) = val.as_table() else { continue };
        let Some(methods) = table.get("methods").and_then(|v| v.as_array()) else {
            continue;
        };

        // Get routing domain for this section
        let routing_domain = PREFIX_ROUTING.get(section.as_str());

        for m in methods {
            let Some(method) = m.as_str() else { continue };
            let prefix = method.split('.').next().unwrap_or(method);
            // If the method's prefix differs from the section name, this method
            // needs a full-method override to route to the correct domain
            if prefix != section {
                if let Some(domain) = routing_domain {
                    overrides.insert(method.to_owned(), domain.clone());
                }
            }
        }
    }
    overrides
});

/// Map a JSON-RPC method name to the capability domain that owns it.
///
/// Given a method like `"tensor.matmul"` or `"stats.mean"`, returns the
/// capability domain string that [`super::CompositionContext`] uses for routing.
/// Springs use this to determine which `call()` domain to use for a given
/// method from their `validation_capabilities` manifest entry.
///
/// The routing is derived entirely from `config/capability_registry.toml`:
/// - Full-method overrides for cross-domain methods (e.g. `security.audit_log` → defense)
/// - Prefix-based routing from TOML section → owner → ALL_CAPS domain
/// - Fallback: the prefix itself (for springs, apps, and unknown domains)
///
/// ```
/// assert_eq!(primalspring::composition::method_to_capability_domain("tensor.matmul"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("stats.mean"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("crypto.hash"), "security");
/// assert_eq!(primalspring::composition::method_to_capability_domain("storage.store"), "storage");
/// assert_eq!(primalspring::composition::method_to_capability_domain("content.put"), "content");
/// assert_eq!(primalspring::composition::method_to_capability_domain("compute.dispatch"), "compute");
/// assert_eq!(primalspring::composition::method_to_capability_domain("linalg.solve"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("spectral.fft"), "tensor");
/// ```
#[must_use]
pub fn method_to_capability_domain(method: &str) -> &str {
    // Check full-method overrides first (cross-domain methods like security.audit_log → defense)
    if let Some(domain) = METHOD_OVERRIDES.get(method) {
        return domain.as_str();
    }

    let prefix = method.split('.').next().unwrap_or(method);

    // Check TOML-derived prefix → routing domain map
    if let Some(domain) = PREFIX_ROUTING.get(prefix) {
        return domain.as_str();
    }

    // Fallback: the prefix itself (springs, apps, unknown domains)
    prefix
}
