// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability-based discovery, neural-api queries, and wire-format parsing.
//!
//! Primals return capabilities in up to 4 wire formats. The
//! [`extract_capability_names`] function handles all of them:
//!
//! - **Format A** — flat string array: `["crypto.sign", "crypto.verify"]`
//! - **Format B** — object array: `[{"method": "crypto.sign"}]`
//! - **Format C** — nested `method_info`: `{"method_info": [{"name": "crypto.sign"}]}`
//! - **Format D** — double-nested `semantic_mappings`: `{"semantic_mappings": {"crypto": {"sign": {}}}}`

use std::path::PathBuf;

use super::neural_bridge::NeuralBridge;

/// Result of discovering a provider by capability domain.
#[derive(Debug, Clone)]
pub struct CapabilityDiscoveryResult {
    /// The capability that was searched for (e.g. `"security"`, `"compute"`).
    pub capability: String,
    /// Primal name resolved at runtime (e.g. `"beardog"`), if found.
    pub resolved_primal: Option<String>,
    /// Socket path if discovered, `None` if unreachable.
    pub socket: Option<PathBuf>,
    /// How the capability was resolved.
    pub source: CapabilityDiscoverySource,
}

/// How a capability was resolved to a socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityDiscoverySource {
    /// Resolved via biomeOS neural-api `capability.discover`.
    NeuralApi,
    /// Resolved via capability-named socket (e.g. `security.sock`).
    CapabilitySocket,
    /// Resolved via socket registry with capability lookup.
    SocketRegistry,
    /// Not resolved.
    NotFound,
}

/// Query biomeOS neural-api for available capabilities.
///
/// Returns a JSON value describing what capabilities are registered, or
/// `None` if biomeOS is unreachable.
#[must_use]
pub fn discover_capabilities(capability: &str) -> Option<serde_json::Value> {
    NeuralBridge::discover().and_then(|b| b.discover_capability(capability).ok())
}

/// Discover a provider by capability domain rather than primal name.
///
/// This is the **loose coupling** discovery path. Instead of hardcoding
/// "discover beardog", callers ask "discover whoever provides security".
///
/// Discovery order:
/// 1. biomeOS neural-api `capability.discover` (authoritative when running)
/// 2. Capability-named socket (e.g. `$XDG_RUNTIME_DIR/biomeos/security.sock`)
/// 3. Socket registry capability scan
///
/// Returns the resolved primal name and socket so callers never need to
/// know which primal implements a capability.
#[must_use]
pub fn discover_by_capability(capability: &str) -> CapabilityDiscoveryResult {
    // Tier 1: biomeOS neural-api (authoritative)
    if let Some(resp) = discover_capabilities(capability) {
        let endpoint_str = resp
            .get("primary_endpoint")
            .or_else(|| resp.get("primary_socket"))
            .and_then(serde_json::Value::as_str);

        if let Some(raw) = endpoint_str {
            let socket_str = strip_unix_uri(raw);
            let path = PathBuf::from(socket_str);
            if path.exists() {
                let primal = resp
                    .get("primals")
                    .and_then(|p| p.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|p| p.get("name"))
                    .and_then(serde_json::Value::as_str)
                    .map(String::from);

                // Prefer .jsonrpc.sock when available (ToadStool's primary is
                // tarpc; JSON-RPC lives on the .jsonrpc.sock sibling).
                let resolved = prefer_jsonrpc_socket(&path);

                return CapabilityDiscoveryResult {
                    capability: capability.to_owned(),
                    resolved_primal: primal,
                    socket: Some(resolved),
                    source: CapabilityDiscoverySource::NeuralApi,
                };
            }
        }
    }

    // Tier 2: Capability-named socket on filesystem
    let base =
        std::env::var("XDG_RUNTIME_DIR").map_or_else(|_| std::env::temp_dir(), PathBuf::from);
    let biomeos_dir = base.join(crate::primal_names::BIOMEOS);
    let family = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());

    // 2a: {capability}-{family}.sock (multi-tenant convention)
    let family_sock = biomeos_dir.join(format!("{capability}-{family}.sock"));
    if family_sock.exists() {
        return CapabilityDiscoveryResult {
            capability: capability.to_owned(),
            resolved_primal: None,
            socket: Some(family_sock),
            source: CapabilityDiscoverySource::CapabilitySocket,
        };
    }
    // 2b: {capability}.sock (single-tenant fallback)
    let cap_sock = biomeos_dir.join(format!("{capability}.sock"));
    if cap_sock.exists() {
        return CapabilityDiscoveryResult {
            capability: capability.to_owned(),
            resolved_primal: None,
            socket: Some(cap_sock),
            source: CapabilityDiscoverySource::CapabilitySocket,
        };
    }

    // Tier 3: Socket registry scan by capability
    if let Some(socket) = discover_from_socket_registry_by_capability(capability) {
        return CapabilityDiscoveryResult {
            capability: capability.to_owned(),
            resolved_primal: socket.1,
            socket: Some(socket.0),
            source: CapabilityDiscoverySource::SocketRegistry,
        };
    }

    CapabilityDiscoveryResult {
        capability: capability.to_owned(),
        resolved_primal: None,
        socket: None,
        source: CapabilityDiscoverySource::NotFound,
    }
}

/// Discover providers for a set of required capabilities.
///
/// Capability-based analog of [`crate::ipc::discover::discover_for`]. Instead of a primal name
/// list, takes a capability list and resolves each at runtime.
#[must_use]
pub fn discover_capabilities_for(capabilities: &[&str]) -> Vec<CapabilityDiscoveryResult> {
    capabilities
        .iter()
        .map(|cap| discover_by_capability(cap))
        .collect()
}

/// If a `.jsonrpc.sock` sibling exists for a tarpc `.sock`, prefer it.
///
/// Some primals (`ToadStool`) bind separate tarpc and JSON-RPC sockets.
/// biomeOS returns the tarpc socket as `primary_endpoint`, but our IPC
/// client speaks JSON-RPC. This helper upgrades the path transparently.
fn prefer_jsonrpc_socket(path: &std::path::Path) -> PathBuf {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name.ends_with(".jsonrpc.sock") {
        return path.to_path_buf();
    }
    if let Some(stem) = name.strip_suffix(".sock") {
        let jsonrpc = path.with_file_name(format!("{stem}.jsonrpc.sock"));
        if jsonrpc.exists() {
            return jsonrpc;
        }
    }
    path.to_path_buf()
}

/// Scan the socket registry for a primal that provides a given capability.
///
/// Reads `$XDG_RUNTIME_DIR/biomeos/socket-registry.json` and checks each
/// entry's `capabilities` array for a match.
fn discover_from_socket_registry_by_capability(
    capability: &str,
) -> Option<(PathBuf, Option<String>)> {
    let base = std::env::var("XDG_RUNTIME_DIR").ok()?;
    let registry_path = PathBuf::from(base)
        .join(crate::primal_names::BIOMEOS)
        .join("socket-registry.json");

    let contents = std::fs::read_to_string(registry_path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let registry = parsed.as_object()?;

    for (primal_name, entry) in registry {
        let caps = entry
            .get("capabilities")
            .and_then(serde_json::Value::as_array);
        let has_cap = caps.is_some_and(|arr| {
            arr.iter().any(|c| {
                c.as_str()
                    .is_some_and(|s| s.eq_ignore_ascii_case(capability))
            })
        });

        if has_cap {
            if let Some(sock_str) = entry.get("socket_path").and_then(serde_json::Value::as_str) {
                let path = PathBuf::from(sock_str);
                if path.exists() {
                    return Some((path, Some(primal_name.clone())));
                }
            }
        }
    }

    None
}

/// Call `capability.call` via biomeOS to invoke a semantic capability.
///
/// This is the **loose standard** for cross-primal invocation: the caller
/// specifies a semantic capability and operation, and biomeOS routes to the
/// correct provider primal, translates the method, and returns the result.
///
/// # Example
///
/// ```rust,no_run
/// use primalspring::ipc::discover::capability_call;
///
/// let args = serde_json::json!({ "data": "hello" });
/// let result = capability_call("crypto", "encrypt", &args);
/// ```
#[must_use]
pub fn capability_call(
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Option<serde_json::Value> {
    let bridge = NeuralBridge::discover()?;
    bridge
        .capability_call(capability, operation, args)
        .ok()
        .map(|result| result.value)
}

/// Extract capability method names from any of the 4 ecosystem wire formats.
///
/// Handles:
/// - **Format A** — flat string array: `["crypto.sign", "crypto.verify"]`
/// - **Format B** — object array: `[{"method": "crypto.sign", ...}]`
/// - **Format C** — nested `method_info`: `{"method_info": [{"name": "crypto.sign", ...}]}`
/// - **Format D** — double-nested `semantic_mappings`: `{"semantic_mappings": {"crypto": {"sign": {}}}}`
///
/// Returns an empty `Vec` if the input is `None` or an unrecognised format.
#[must_use]
pub fn extract_capability_names(caps: Option<serde_json::Value>) -> Vec<String> {
    let Some(val) = caps else {
        return Vec::new();
    };
    match &val {
        // Format A: ["method.name", ...]
        serde_json::Value::Array(arr) => extract_from_array(arr),

        serde_json::Value::Object(map) => {
            // Format C: {"method_info": [{"name": "...", ...}]}
            if let Some(serde_json::Value::Array(info)) = map.get("method_info") {
                return info
                    .iter()
                    .filter_map(|item| item.get("name")?.as_str().map(String::from))
                    .collect();
            }

            // Format D: {"semantic_mappings": {"domain": {"verb": {...}}}}
            if let Some(serde_json::Value::Object(domains)) = map.get("semantic_mappings") {
                return domains
                    .iter()
                    .flat_map(|(domain, verbs)| {
                        if let serde_json::Value::Object(verb_map) = verbs {
                            verb_map
                                .keys()
                                .map(|verb| format!("{domain}.{verb}"))
                                .collect::<Vec<_>>()
                        } else {
                            vec![domain.clone()]
                        }
                    })
                    .collect();
            }

            // Format E: {"capabilities": ["cap1", ...], ...} (loamSpine wraps them)
            if let Some(serde_json::Value::Array(caps_arr)) = map.get("capabilities") {
                return extract_from_array(caps_arr);
            }

            // Format F: {"methods": ["m1", ...], ...} (method-list style)
            if let Some(serde_json::Value::Array(methods)) = map.get("methods") {
                return extract_from_array(methods);
            }

            // Fallback: treat top-level object keys as capability names
            map.keys().cloned().collect()
        }
        _ => Vec::new(),
    }
}

/// Strip `unix://` prefix from an endpoint URI to get the raw filesystem path.
///
/// biomeOS returns endpoints as `unix:///run/user/1000/biomeos/foo.sock`;
/// primalSpring needs the bare path for `std::os::unix::net::UnixStream`.
#[must_use]
pub fn strip_unix_uri(endpoint: &str) -> &str {
    endpoint.strip_prefix("unix://").unwrap_or(endpoint)
}

/// Extract names from a JSON array (Formats A and B).
fn extract_from_array(arr: &[serde_json::Value]) -> Vec<String> {
    arr.iter()
        .filter_map(|v| {
            // Format A: bare strings
            if let Some(s) = v.as_str() {
                return Some(s.to_owned());
            }
            // Format B: {"method": "name"} objects
            v.get("method")
                .and_then(serde_json::Value::as_str)
                .map(String::from)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_capabilities_none_when_no_biomeos() {
        assert!(discover_capabilities("crypto").is_none());
    }

    // --- 4-format capability parsing tests ---

    #[test]
    fn format_a_flat_string_array() {
        let val = serde_json::json!(["crypto.sign", "crypto.verify", "storage.put"]);
        let names = extract_capability_names(Some(val));
        assert_eq!(names, vec!["crypto.sign", "crypto.verify", "storage.put"]);
    }

    #[test]
    fn format_b_object_array() {
        let val = serde_json::json!([
            {"method": "crypto.sign", "version": "1.0"},
            {"method": "crypto.verify"},
        ]);
        let names = extract_capability_names(Some(val));
        assert_eq!(names, vec!["crypto.sign", "crypto.verify"]);
    }

    #[test]
    fn format_c_method_info() {
        let val = serde_json::json!({
            "method_info": [
                {"name": "crypto.sign", "params": []},
                {"name": "crypto.verify", "params": []},
            ]
        });
        let names = extract_capability_names(Some(val));
        assert_eq!(names, vec!["crypto.sign", "crypto.verify"]);
    }

    #[test]
    fn format_d_semantic_mappings() {
        let val = serde_json::json!({
            "semantic_mappings": {
                "crypto": {"sign": {}, "verify": {}},
                "storage": {"put": {}},
            }
        });
        let names = extract_capability_names(Some(val));
        assert!(names.contains(&"crypto.sign".to_owned()));
        assert!(names.contains(&"crypto.verify".to_owned()));
        assert!(names.contains(&"storage.put".to_owned()));
    }

    #[test]
    fn fallback_object_keys() {
        let val = serde_json::json!({"crypto": {}, "storage": {}});
        let names = extract_capability_names(Some(val));
        assert!(names.contains(&"crypto".to_owned()));
        assert!(names.contains(&"storage".to_owned()));
    }

    #[test]
    fn extract_from_none() {
        assert!(extract_capability_names(None).is_empty());
    }

    #[test]
    fn mixed_format_b_array() {
        let val = serde_json::json!(["direct.method", {"method": "object.method"}]);
        let names = extract_capability_names(Some(val));
        assert_eq!(names, vec!["direct.method", "object.method"]);
    }

    mod proptest_fuzz {
        use super::*;
        use proptest::prelude::*;

        fn arb_capability_string() -> impl Strategy<Value = String> {
            prop::string::string_regex("[a-z]{1,10}\\.[a-z]{1,10}").expect("valid regex")
        }

        proptest! {
            #[test]
            fn extract_capability_names_never_panics(
                input in "[\\PC]{0,200}",
            ) {
                let val = serde_json::from_str::<serde_json::Value>(&input).ok();
                let _ = extract_capability_names(val);
            }

            #[test]
            fn format_a_round_trips(
                caps in prop::collection::vec(arb_capability_string(), 0..10),
            ) {
                let val = serde_json::json!(caps);
                let names = extract_capability_names(Some(val));
                prop_assert_eq!(names.len(), caps.len());
                for (a, b) in names.iter().zip(caps.iter()) {
                    prop_assert_eq!(a, b);
                }
            }

            #[test]
            fn format_b_round_trips(
                caps in prop::collection::vec(arb_capability_string(), 0..10),
            ) {
                let arr: Vec<serde_json::Value> = caps
                    .iter()
                    .map(|c| serde_json::json!({"method": c}))
                    .collect();
                let val = serde_json::Value::Array(arr);
                let names = extract_capability_names(Some(val));
                prop_assert_eq!(names.len(), caps.len());
            }
        }
    }
}
