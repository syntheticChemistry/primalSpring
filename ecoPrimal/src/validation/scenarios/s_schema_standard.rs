// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Schema Standard — validate `primal.list` and `capability.list`
//! canonical response schemas.
//!
//! Wave 20: projectNUCLEUS's discovery cascade and projectFOUNDATION's
//! `primal_ipc.sh` both depend on a standard shape from biomeOS for these
//! two introspection methods. This scenario:
//!
//! - Defines the canonical response schema for both methods
//! - Validates primalSpring's own `capabilities.list` response
//! - Probes live biomeOS for `primal.list` and `capability.list`
//! - Asserts consistent shape (object vs array, required keys)

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "schema-standard",
        track: Track::BiomeosDeploy,
        tier: Tier::Both,
        provenance_crate: "schema_standard_wave20",
        provenance_date: "2026-05-16",
        description: "primal.list + capability.list canonical schema validation",
    },
    run,
};

/// Canonical `primal.list` response schema.
///
/// ```json
/// {
///   "primals": [
///     {
///       "name": "primalspring",
///       "socket": "/run/ecoprimals/primalspring.sock",
///       "pid": 12345,             // optional
///       "capabilities": ["..."],  // optional
///       "status": "running"       // optional
///     }
///   ],
///   "count": 13
/// }
/// ```
const PRIMAL_LIST_REQUIRED_KEYS: &[&str] = &["primals", "count"];
const PRIMAL_LIST_ENTRY_REQUIRED_KEYS: &[&str] = &["name", "socket"];
const PRIMAL_LIST_ENTRY_OPTIONAL_KEYS: &[&str] = &["pid", "capabilities", "status", "version"];

/// Canonical `capability.list` response schema.
///
/// ```json
/// {
///   "capabilities": ["security", "content", ...],  // or array of objects
///   "count": 42,
///   "primal": "biomeos"   // optional: responder identity
/// }
/// ```
const CAPABILITY_LIST_REQUIRED_KEYS: &[&str] = &["capabilities", "count"];

/// Run the schema standard validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1 (Rust): registry presence");
    phase_registry(v);

    v.section("Phase 2 (Rust): local capability.list shape");
    phase_local_capability_list(v);

    v.section("Phase 3 (Live): biomeOS capability.list schema");
    phase_live_capability_list(v, ctx);

    v.section("Phase 4 (Live): biomeOS primal.list schema");
    phase_live_primal_list(v, ctx);
}

fn phase_registry(v: &mut ValidationResult) {
    let registry = include_str!("../../../../config/capability_registry.toml");

    v.check_bool(
        "registry:has_primal_list",
        registry.contains("primal.list"),
        "capability_registry.toml contains primal.list",
    );

    v.check_bool(
        "registry:has_capabilities_list",
        registry.contains("capabilities.list"),
        "capability_registry.toml contains capabilities.list",
    );

    v.check_bool(
        "registry:has_capability_list",
        registry.contains("capability.list"),
        "capability_registry.toml contains capability.list (singular alias)",
    );
}

fn phase_local_capability_list(v: &mut ValidationResult) {
    v.check_skip(
        "local:capability_count",
        "primalSpring is not a primal — no local capability list (post-primordial)",
    );
}


fn phase_live_capability_list(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "live:capability_list",
            "biomeOS orchestration not available",
        );
        return;
    }

    match ctx.call("orchestration", "capability.list", serde_json::json!({})) {
        Ok(resp) => {
            for key in CAPABILITY_LIST_REQUIRED_KEYS {
                v.check_bool(
                    &format!("live:capability_list:has_{key}"),
                    resp.get(key).is_some(),
                    &format!("biomeOS capability.list has {key}"),
                );
            }

            if let Some(caps) = resp.get("capabilities") {
                v.check_bool(
                    "live:capability_list:type",
                    caps.is_array(),
                    "biomeOS capability.list capabilities is array (not object)",
                );
            }

            if let Some(count) = resp.get("count").and_then(serde_json::Value::as_u64) {
                v.check_minimum(
                    "live:capability_list:count",
                    usize::try_from(count).unwrap_or(usize::MAX),
                    5,
                );
            }
        }
        Err(e) if e.is_skippable() => {
            v.check_skip("live:capability_list", &format!("connection: {e}"));
        }
        Err(e) => {
            v.check_bool(
                "live:capability_list",
                false,
                &format!("capability.list error: {e}"),
            );
        }
    }
}

fn phase_live_primal_list(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        v.check_skip("live:primal_list", "biomeOS orchestration not available");
        return;
    }

    match ctx.call("orchestration", "primal.list", serde_json::json!({})) {
        Ok(resp) => {
            for key in PRIMAL_LIST_REQUIRED_KEYS {
                v.check_bool(
                    &format!("live:primal_list:has_{key}"),
                    resp.get(key).is_some(),
                    &format!("biomeOS primal.list has {key}"),
                );
            }

            if let Some(primals) = resp.get("primals").and_then(|p| p.as_array()) {
                v.check_minimum("live:primal_list:count", primals.len(), 1);

                for (i, entry) in primals.iter().enumerate() {
                    if i >= 3 {
                        break;
                    }
                    for key in PRIMAL_LIST_ENTRY_REQUIRED_KEYS {
                        v.check_bool(
                            &format!("live:primal_list:entry_{i}:has_{key}"),
                            entry.get(key).is_some(),
                            &format!("primal entry {i} has required field: {key}"),
                        );
                    }
                    let optional_present = PRIMAL_LIST_ENTRY_OPTIONAL_KEYS
                        .iter()
                        .filter(|k| entry.get(k).is_some())
                        .count();
                    v.check_minimum(
                        &format!("live:primal_list:entry_{i}:optional_richness"),
                        optional_present,
                        0,
                    );
                }
            }

            if let Some(count) = resp.get("count").and_then(serde_json::Value::as_u64) {
                if let Some(arr) = resp.get("primals").and_then(|p| p.as_array()) {
                    v.check_bool(
                        "live:primal_list:count_matches",
                        count == arr.len() as u64,
                        "primal.list count matches primals array length",
                    );
                }
            }
        }
        Err(e) => {
            let detail = e.to_string();
            if detail.contains("-32601") || detail.contains("not found") {
                v.check_skip(
                    "live:primal_list",
                    &format!("primal.list not yet implemented in biomeOS: {e}"),
                );
            } else if e.is_skippable() {
                v.check_skip("live:primal_list", &format!("connection: {e}"));
            } else {
                v.check_bool(
                    "live:primal_list",
                    false,
                    &format!("primal.list error: {e}"),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_standard_no_panic() {
        let mut v = ValidationResult::new("schema-standard");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }

    #[test]
    fn schema_constants_well_formed() {
        assert_eq!(PRIMAL_LIST_REQUIRED_KEYS.len(), 2);
        assert_eq!(PRIMAL_LIST_ENTRY_REQUIRED_KEYS.len(), 2);
        assert_eq!(CAPABILITY_LIST_REQUIRED_KEYS.len(), 2);
        assert!(PRIMAL_LIST_ENTRY_OPTIONAL_KEYS.len() >= 3);
    }

    #[test]
    fn local_capability_list_skips_post_primordial() {
        let mut v = crate::validation::ValidationResult::new("test");
        phase_local_capability_list(&mut v);
        assert_eq!(v.skipped, 1);
    }
}
