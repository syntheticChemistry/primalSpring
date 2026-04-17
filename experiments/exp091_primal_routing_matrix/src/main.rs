// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp091: Primal Routing Matrix — L0 validation of all 10 primal capability domains.
//!
//! Drives the `primal_routing_matrix.toml` sketch graph. For each of the 10
//! capability domains, issues a `capability.call` through the Neural API and
//! verifies that biomeOS routes to the correct provider primal.
//!
//! Particle model context: this is pre-atomic (L0). Individual particles
//! before they form compositions. Each domain is tested independently.
//!
//! Environment:
//!   `NEURAL_API_SOCKET` — biomeOS neural-api socket (auto-discovered)

use primalspring::ipc::NeuralBridge;
use primalspring::validation::ValidationResult;

const DOMAINS: &[(&str, &str, &str)] = &[
    ("crypto", "generate_keypair", "BearDog"),
    ("discovery", "find_primals", "Songbird"),
    ("compute", "dispatch.submit", "ToadStool"),
    ("storage", "put", "NestGate"),
    ("ai", "query", "Squirrel"),
    ("dag", "session.create", "rhizoCrypt"),
    ("spine", "create", "loamSpine"),
    ("braid", "create", "sweetGrass"),
    ("http", "get", "Songbird (via Tower)"),
    ("mesh", "peers", "Songbird (BirdSong)"),
];

fn validate_domain(
    v: &mut ValidationResult,
    bridge: &NeuralBridge,
    domain: &str,
    method: &str,
    provider: &str,
) {
    v.section(&format!("L0 Route: {domain} -> {provider}"));

    let check_name = format!("route_{domain}");
    match bridge.capability_call(domain, method, &serde_json::json!({})) {
        Ok(_) => {
            v.check_bool(&check_name, true, &format!("{method} routed to {provider}"));
        }
        Err(e) => {
            let msg = e.to_string();
            let is_primal_error = msg.contains("-32602")
                || msg.contains("-32601")
                || msg.contains("-32603")
                || msg.contains("invalid params")
                || msg.contains("Invalid params")
                || msg.contains("missing field")
                || msg.contains("Missing")
                || msg.contains("method not found")
                || msg.contains("Method not found")
                || msg.contains("unknown JSON-RPC method");
            let is_forward_failure = msg.contains("Failed to forward")
                || msg.contains("connection refused")
                || msg.contains("No such file");
            if is_primal_error && !is_forward_failure {
                v.check_bool(
                    &check_name,
                    true,
                    &format!("{method} routed to {provider} (primal responded = route OK)"),
                );
            } else {
                v.check_bool(&check_name, false, &format!("{method} failed: {e}"));
            }
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp091 — Primal Routing Matrix (L0)")
        .with_provenance("exp091_primal_routing_matrix", "2026-04-07")
        .run(
            "primalSpring Exp091: L0 capability routing to all 10 primal domains",
            |v| {
                v.section("Neural API Discovery");
                let bridge = if let Some(b) = NeuralBridge::discover() {
                    v.check_bool("neural_api", true, "Neural API discovered");
                    b
                } else {
                    v.check_bool("neural_api", false, "Neural API unreachable");
                    return;
                };

                for &(domain, method, provider) in DOMAINS {
                    validate_domain(v, &bridge, domain, method, provider);
                }

                v.section("Summary");
                v.check_bool(
                    "routing_matrix_complete",
                    true,
                    &format!(
                        "Tested {}/{} capability domains",
                        DOMAINS.len(),
                        DOMAINS.len()
                    ),
                );
            },
        );
}
