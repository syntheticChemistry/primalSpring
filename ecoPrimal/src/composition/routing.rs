// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability routing tables — maps capabilities to primals and methods to domains.

/// All NUCLEUS capabilities that primalSpring discovers and authenticates.
///
/// Single source of truth for `discover`, `from_live_discovery`, `PROACTIVE_CAPS`
/// in `upgrade_btsp_clients`, and the TCP fallback table. Each entry is
/// the *primary* capability domain for a primal. Aliases like `"provenance"`
/// (→ rhizoCrypt, same as `"dag"`) are handled by the routing match in
/// `capability_to_primal()` but do not appear here to avoid duplicate
/// discovery attempts against the same socket.
pub const ALL_CAPS: &[&str] = &[
    "security",
    "discovery",
    "compute",
    "tensor",
    "shader",
    "storage",
    "ai",
    "dag",
    "commit",
    "visualization",
    "ledger",
    "attribution",
    "defense",
];

/// Extended capability aliases for BTSP proactive escalation.
///
/// Includes names that map to the same primal sockets as [`ALL_CAPS`]
/// (e.g. `inference` → Squirrel, `spine`/`merkle` → loamSpine) to ensure
/// BTSP coverage even when a client was connected under an alias name.
pub const BTSP_EXTRA_CAPS: &[&str] =
    &["inference", "spine", "merkle", "braid", "recon", "threat", "lineage"];

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
/// ```
#[must_use]
pub fn capability_to_primal(capability: &str) -> &str {
    use crate::primal_names as pn;
    match capability {
        "security" | "crypto" => pn::BEARDOG,
        "discovery" | "network" => pn::SONGBIRD,
        "compute" => pn::TOADSTOOL,
        "tensor" | "math" => pn::BARRACUDA,
        "shader" => pn::CORALREEF,
        "storage" => pn::NESTGATE,
        "ai" | "inference" => pn::SQUIRREL,
        "dag" | "provenance" => pn::RHIZOCRYPT,
        "ledger" | "spine" | "merkle" => pn::LOAMSPINE,
        "commit" | "attribution" | "braid" => pn::SWEETGRASS,
        "visualization" => pn::PETALTONGUE,
        "defense" | "recon" | "threat" => pn::SKUNKBAT,
        "orchestration" => pn::BIOMEOS,
        other => other,
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
/// assert_eq!(primalspring::composition::method_to_capability_domain("compute.dispatch"), "compute");
/// assert_eq!(primalspring::composition::method_to_capability_domain("linalg.solve"), "tensor");
/// assert_eq!(primalspring::composition::method_to_capability_domain("spectral.fft"), "tensor");
/// ```
#[must_use]
pub fn method_to_capability_domain(method: &str) -> &str {
    let prefix = method.split('.').next().unwrap_or(method);
    match prefix {
        "crypto" | "health" | "identity" | "primal" => "security",
        "ipc" | "discovery" => "discovery",
        "compute" => "compute",
        "tensor" | "stats" | "math" | "noise" | "activation" | "rng" | "fhe" | "tolerances"
        | "validate" | "device" | "linalg" | "spectral" => "tensor",
        "shader" => "shader",
        "storage" => "storage",
        "inference" | "ai" | "squirrel" | "mcp" => "ai",
        "dag" => "dag",
        "spine" | "entry" | "certificate" => "ledger",
        "braid" | "anchoring" => "commit",
        "visualization" | "viz" | "proprioception" => "visualization",
        "defense" | "recon" | "threat" | "lineage" => "defense",
        "graph" | "capability" | "lifecycle" | "coordination" => "orchestration",
        _ => prefix,
    }
}
