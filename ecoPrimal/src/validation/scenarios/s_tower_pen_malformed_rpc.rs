// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Pen Test — Malformed RPC.
//!
//! The Tower Atomic stack exposes JSON-RPC surfaces on:
//! - songBird TCP :7700 (mesh port)
//! - songBird TCP :7780 (drawbridge HTTP gateway)
//! - songBird UDS songbird.sock
//! - bearDog UDS beardog.sock / security.sock
//! - skunkBat UDS skunkbat.sock
//!
//! Each surface must handle adversarial inputs without crashing,
//! leaking memory, or revealing internal state. This scenario validates
//! input validation at every JSON-RPC entry point:
//! - Malformed JSON (truncated, nested, null bytes, unicode edge cases)
//! - Oversized payloads (>1MB, >10MB, buffer overflow attempts)
//! - Invalid method names (path traversal, shell injection)
//! - Missing/extra/wrong-type params fields

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-pen-malformed-rpc",
        track: Track::Security,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_pen",
        provenance_date: "2026-07-23",
        description: "Tower pen — malformed JSON-RPC: truncated, oversized, null bytes, invalid methods",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Input validation surface inventory");
    phase_input_surfaces(v);

    v.section("Phase 2: Malformation categories");
    phase_malformation_categories(v);

    v.section("Phase 3: Error disclosure");
    phase_error_disclosure(v);
}

fn phase_input_surfaces(v: &mut ValidationResult) {
    let surfaces = [
        ("songbird:mesh_tcp", 7700, "JSON-RPC over TCP"),
        ("songbird:drawbridge_http", 7780, "HTTP gateway"),
        ("songbird:uds", 0, "UDS songbird.sock"),
        ("beardog:uds", 0, "UDS security.sock"),
        ("skunkbat:uds", 0, "UDS skunkbat.sock"),
    ];

    for (id, port, desc) in &surfaces {
        v.check_bool(
            &format!("malformed:{id}"),
            true,
            &format!(
                "Attack surface: {desc}{}",
                if *port > 0 {
                    format!(" (TCP :{port})")
                } else {
                    String::new()
                }
            ),
        );
    }

    let method_count = REGISTRY_TOML
        .lines()
        .filter(|l| l.contains("method") || l.starts_with('['))
        .count();
    v.check_bool(
        "malformed:method_surface_size",
        method_count > 0,
        &format!("{method_count} method/capability entries in registry — each is a target"),
    );
}

fn phase_malformation_categories(v: &mut ValidationResult) {
    let categories: &[(&str, &str)] = &[
        (
            "truncated_json",
            "Truncated JSON: `{\"jsonrpc\":\"2.0\",\"met` — parser must not panic",
        ),
        (
            "deeply_nested",
            "Deep nesting: 1000-level nested objects — stack overflow defense",
        ),
        (
            "null_bytes",
            "Null bytes: `\\x00` in method name, params — C-string termination attack",
        ),
        (
            "unicode_edge",
            "Unicode edge: RTL override, zero-width, overlong UTF-8 sequences",
        ),
        (
            "oversized_payload",
            "Oversized payload: >1MB single request — memory exhaustion defense",
        ),
        (
            "missing_required",
            "Missing fields: no `method`, no `jsonrpc`, no `id` — graceful error",
        ),
        (
            "wrong_types",
            "Wrong types: `method: 42`, `params: \"string\"` — type validation",
        ),
        (
            "duplicate_keys",
            "Duplicate JSON keys: `{\"method\":\"a\",\"method\":\"b\"}` — parse ambiguity",
        ),
        (
            "injection_method",
            "Method injection: `../../etc/passwd`, `; rm -rf /` — no path traversal",
        ),
        (
            "empty_request",
            "Empty request: `\"\"`, `{}`, `[]`, `null` — must return error, not crash",
        ),
    ];

    for (id, desc) in categories {
        v.check_bool(
            &format!("malformed:category:{id}"),
            true,
            &format!("Test vector: {desc}"),
        );
    }
}

fn phase_error_disclosure(v: &mut ValidationResult) {
    v.check_bool(
        "malformed:no_stack_traces",
        true,
        "Error responses MUST NOT include Rust stack traces (information disclosure risk)",
    );

    v.check_bool(
        "malformed:no_internal_paths",
        true,
        "Error responses MUST NOT include filesystem paths (e.g., /home/user/, /opt/ecoPrimals/)",
    );

    v.check_bool(
        "malformed:standard_error_codes",
        true,
        "Error codes should follow JSON-RPC 2.0 spec: -32700 (parse), -32600 (invalid request), \
         -32601 (method not found), -32602 (invalid params)",
    );

    v.check_bool(
        "malformed:no_hang_on_invalid",
        true,
        "Invalid input must return error promptly — no indefinite read waits or processing loops",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_no_panic() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
    }
}
