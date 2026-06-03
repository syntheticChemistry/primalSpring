// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability routing tables — maps capabilities to primals and methods to domains.
//!
//! The domain→owner mapping is derived from `config/capability_registry.toml`
//! (parsed once at init via [`LazyLock`]). This eliminates hand-maintained
//! duplication between code tables and the canonical TOML.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::primal_names::Primal;

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
/// `CompositionContext::discover()` iterates. Curated subset of the full
/// registry because not every TOML section is a discovery target.
///
/// Routing consistency is enforced by `s_routing_consistency` which verifies
/// that every method in `capability_registry.toml` routes through
/// `method_to_capability_domain` + `capability_to_primal` to its declared owner.
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

/// Typed version — consults the TOML-derived domain→owner map first,
/// falls back to static match for aliases not directly in the TOML.
///
/// Returns `None` for non-primal targets (springs, unknown).
#[must_use]
pub fn capability_to_primal_typed(capability: &str) -> Option<Primal> {
    if let Some(owner) = DOMAIN_OWNER_MAP.get(capability) {
        return owner.parse::<Primal>().ok();
    }
    static_capability_fallback(capability)
}

/// Static fallback for capability aliases that aren't direct TOML sections
/// but still map to primals (e.g. `"network"` → Songbird, `"orchestration"` → BiomeOS).
///
/// These arise because `ALL_CAPS` names the *primary discovery domain* per primal,
/// which may not match any single TOML section key. The TOML has finer-grained
/// sections (e.g. `[graph]`, `[lifecycle]`, `[topology]` all owned by BiomeOS,
/// but the discovery domain is `"orchestration"`).
fn static_capability_fallback(capability: &str) -> Option<Primal> {
    use Primal::{BarraCuda, BiomeOS, LoamSpine, Songbird, SkunkBat, SweetGrass};
    match capability {
        "orchestration" | "manifest" => Some(BiomeOS),
        "commit" => Some(SweetGrass),
        "ledger" | "spine" | "merkle" => Some(LoamSpine),
        "network" => Some(Songbird),
        "math" => Some(BarraCuda),
        "recon" | "threat" | "audit" => Some(SkunkBat),
        _ => None,
    }
}

/// Return a `&'static str` for known owner slugs via enum-driven resolution.
///
/// Avoids leaking heap strings by resolving through [`Primal`] and [`Spring`]
/// enums — adding a new primal/spring forces a compiler error rather than
/// silently falling through to `"unknown"`.
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
        _ => "unknown",
    }
}

/// Map a JSON-RPC method name to the capability domain that owns it.
///
/// Given a method like `"tensor.matmul"` or `"stats.mean"`, returns the
/// capability domain string that [`super::CompositionContext`] uses for routing.
/// Springs use this to determine which `call()` domain to use for a given
/// method from their `validation_capabilities` manifest entry.
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
    // Full-method overrides for cross-domain methods where the prefix
    // doesn't match the owning primal's domain.
    match method {
        "security.audit_log" | "security.audit_event" => return "defense",
        _ => {}
    }
    let prefix = method.split('.').next().unwrap_or(method);
    match prefix {
        "crypto" | "health" | "identity" | "primal" | "tls" | "btsp" | "beacon" | "genetic"
        | "birdsong" | "lineage" => "security",
        "ipc" | "discovery" | "tor" | "relay" | "http" | "dns" | "stun" | "turn" | "network"
        | "mesh" | "onion" => "discovery",
        "compute" | "dispatch" | "workload" | "sovereign" => "compute",
        "tensor" | "stats" | "math" | "noise" | "activation" | "rng" | "fhe" | "tolerances"
        | "validate" | "device" | "linalg" | "spectral" | "nautilus" | "ml" | "ode" | "nn"
        | "ops" | "signal" => "tensor",
        "shader" => "shader",
        "storage" | "secrets" => "storage",
        "content" => "content",
        "inference" | "ai" | "squirrel" => "ai",
        "dag" | "event" | "merkle" | "vertex" | "dehydration" | "slice" => "dag",
        "spine" | "entry" | "certificate" | "session" | "permanence" | "anchor" | "proof" => {
            "ledger"
        }
        "braid" | "anchoring" | "contribution" => "commit",
        "visualization" | "viz" | "render" | "interaction" => "visualization",
        "defense" | "recon" | "threat" | "audit" => "defense",
        "graph" | "capability" | "lifecycle" | "topology" | "federation" | "route" | "system"
        | "biomeos" | "nucleus" | "membrane" | "cell" | "proprioception"
        | "neural_api" => "orchestration",
        "impulse" | "potential" | "git" | "temporal" => "membrane",
        "tool" | "tools" | "auth" | "primalspring" | "bonding" | "composition" | "context"
        | "ionic" | "mcp" | "coordination" => "tool",
        "webb" | "esotericwebb" => "webb",
        "game" => "game",
        "provenance" => "provenance",
        _ => prefix,
    }
}
