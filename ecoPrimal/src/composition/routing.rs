// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability routing tables — maps capabilities to primals and methods to domains.

use crate::primal_names::Primal;

/// All NUCLEUS capabilities that primalSpring discovers and authenticates.
///
/// Single source of truth for `discover`, `from_live_discovery`, `PROACTIVE_CAPS`
/// in `upgrade_btsp_clients`, and the TCP fallback table. Each entry is
/// the *primary* capability domain for a primal.
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
/// This is the ecosystem's single source of truth for "which primal owns
/// which capability domain." Springs use this to route IPC calls without
/// hardcoding primal names.
///
/// ```
/// assert_eq!(primalspring::composition::capability_to_primal("tensor"), "barracuda");
/// assert_eq!(primalspring::composition::capability_to_primal("crypto"), "beardog");
/// assert_eq!(primalspring::composition::capability_to_primal("storage"), "nestgate");
/// assert_eq!(primalspring::composition::capability_to_primal("content"), "nestgate");
/// ```
#[must_use]
pub fn capability_to_primal(capability: &str) -> &str {
    match capability_to_primal_typed(capability) {
        Some(p) => p.slug(),
        None => match capability {
            "tool" | "primalspring" => "primalspring",
            "webb" => "esotericwebb",
            "game" => "ludospring",
            other => other,
        },
    }
}

/// Typed version — returns `None` for non-primal targets (springs, unknown).
#[must_use]
pub fn capability_to_primal_typed(capability: &str) -> Option<Primal> {
    use Primal::{BearDog, Songbird, ToadStool, BarraCuda, CoralReef, NestGate, Squirrel, RhizoCrypt, SweetGrass, LoamSpine, PetalTongue, SkunkBat, BiomeOS};
    match capability {
        "security" | "crypto" => Some(BearDog),
        "discovery" | "network" => Some(Songbird),
        "compute" => Some(ToadStool),
        "tensor" | "math" => Some(BarraCuda),
        "shader" => Some(CoralReef),
        "storage" | "content" => Some(NestGate),
        "ai" | "inference" => Some(Squirrel),
        "dag" => Some(RhizoCrypt),
        "provenance" | "commit" | "attribution" | "braid" => Some(SweetGrass),
        "ledger" | "spine" | "merkle" => Some(LoamSpine),
        "visualization" => Some(PetalTongue),
        "defense" | "recon" | "threat" | "audit" => Some(SkunkBat),
        "orchestration" | "federation" => Some(BiomeOS),
        _ => None,
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
        | "biomeos" | "nucleus" | "membrane" | "cell" | "proprioception" => "orchestration",
        "tool" | "tools" | "auth" | "primalspring" | "bonding" | "composition" | "context"
        | "ionic" | "mcp" | "coordination" => "tool",
        "webb" | "esotericwebb" => "webb",
        "game" => "game",
        "provenance" => "provenance",
        _ => prefix,
    }
}
