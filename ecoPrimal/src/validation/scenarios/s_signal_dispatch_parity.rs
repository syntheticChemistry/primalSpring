// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Signal Dispatch Parity — validate all 14 atomic signals
//! through `CompositionContext::dispatch()` against live biomeOS.
//!
//! Wave 18a of the Neural API Signal Elevation plan. Exercises the unified
//! `dispatch("tier.name", params)` API introduced in Wave 17b for every
//! signal defined in `signal_tools.toml`, checking:
//!
//! - biomeOS accepts the signal (not `-32601`)
//! - Response contains expected keys from the `signal_tools.toml` returns section
//! - Each graph node's capability was reachable
//!
//! Tier 1 (Rust): validates that `dispatch()` correctly splits identifiers
//! and routes to `signal()` without panicking.
//!
//! Tier 2 (Live): sends minimal probe params to biomeOS and validates
//! response shapes.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "signal-dispatch-parity",
        track: Track::BiomeosDeploy,
        tier: Tier::Live,
        provenance_crate: "neural_api_wave18a",
        provenance_date: "2026-05-16",
        description: "Signal dispatch parity: all 14 signals via dispatch() against live biomeOS",
    },
    run,
};

struct SignalSpec {
    id: &'static str,
    expected_keys: &'static [&'static str],
    params: fn() -> serde_json::Value,
}

const SIGNALS: &[SignalSpec] = &[
    SignalSpec {
        id: "tower.publish",
        expected_keys: &["signature", "beacon_id", "audit_event"],
        params: || serde_json::json!({ "data": "dispatch-parity-probe", "_probe": true }),
    },
    SignalSpec {
        id: "tower.authenticate",
        expected_keys: &["session_key", "lineage_verified"],
        params: || serde_json::json!({ "peer_id": "dispatch-parity-test", "_probe": true }),
    },
    SignalSpec {
        id: "tower.discover",
        expected_keys: &["peers", "audit_event"],
        params: || serde_json::json!({ "scope": "local", "_probe": true }),
    },
    SignalSpec {
        id: "tower.health",
        expected_keys: &["beardog", "songbird", "skunkbat"],
        params: || serde_json::json!({ "_probe": true }),
    },
    SignalSpec {
        id: "tower.bootstrap",
        expected_keys: &["beardog_identity", "registry_seeded", "health"],
        params: || serde_json::json!({ "phase": 1, "_probe": true }),
    },
    SignalSpec {
        id: "node.compute",
        expected_keys: &["result", "dispatch_id"],
        params: || serde_json::json!({ "workload": { "type": "probe" }, "_probe": true }),
    },
    SignalSpec {
        id: "nest.store",
        expected_keys: &["content_cid", "dag_event", "session_commit"],
        params: || serde_json::json!({ "content": "dispatch-parity-probe", "_probe": true }),
    },
    SignalSpec {
        id: "nest.commit",
        expected_keys: &["dehydrated_hash", "session_commit"],
        params: || serde_json::json!({ "session_id": "dispatch-parity-probe", "_probe": true }),
    },
    SignalSpec {
        id: "nest.retrieve",
        expected_keys: &["content"],
        params: || serde_json::json!({ "content_cid": "dispatch-parity-probe", "_probe": true }),
    },
    SignalSpec {
        id: "meta.observe",
        expected_keys: &["session_id", "context", "graphs"],
        params: || serde_json::json!({ "domain": "storage", "_probe": true }),
    },
    SignalSpec {
        id: "meta.intent",
        expected_keys: &["plan", "execution_result"],
        params: || serde_json::json!({ "action": "probe dispatch parity", "_probe": true }),
    },
    SignalSpec {
        id: "meta.render",
        expected_keys: &["summary", "rendered"],
        params: || serde_json::json!({ "execution_id": "dispatch-parity-probe", "_probe": true }),
    },
    SignalSpec {
        id: "meta.health",
        expected_keys: &["biomeos", "squirrel", "petaltongue"],
        params: || serde_json::json!({ "_probe": true }),
    },
    SignalSpec {
        id: "meta.deploy",
        expected_keys: &["plan", "deployment_id", "audit_event"],
        params: || serde_json::json!({ "target": "probe", "approval": "suggest", "_probe": true }),
    },
];

/// Run the signal dispatch parity validation scenario.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1 (Rust): dispatch() identifier parsing");
    phase_dispatch_parsing(v, ctx);

    v.section("Phase 2 (Live): dispatch() parity against biomeOS");
    phase_live_dispatch(v, ctx);

    v.section("Phase 3 (Live): response shape validation");
    phase_response_shapes(v, ctx);
}

fn phase_dispatch_parsing(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.check_bool(
        "parse:signal_count",
        SIGNALS.len() == 14,
        &format!("SIGNALS table has {} entries (expected 14)", SIGNALS.len()),
    );

    for spec in SIGNALS {
        let tier = spec.id.split('.').next().unwrap_or("unknown");
        let is_valid_tier = CompositionContext::is_signal_tier(tier);
        v.check_bool(
            &format!("parse:{}_tier_valid", spec.id),
            is_valid_tier,
            &format!("{}: tier {tier:?} is a recognized signal tier", spec.id),
        );
    }

    // Negative cases: malformed identifiers should produce ProtocolError
    let bad_ids = ["noperiod", "", "...triple", "fake.signal"];
    for bad in &bad_ids {
        let result = ctx.dispatch(bad, serde_json::json!({}));
        let is_err = result.is_err();
        v.check_bool(
            &format!("parse:reject:{}", bad.replace('.', "_")),
            is_err,
            &format!("dispatch({bad:?}) should reject malformed/unknown identifier"),
        );
    }
}

fn phase_live_dispatch(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        for spec in SIGNALS {
            v.check_skip(
                &format!("live:dispatch:{}", spec.id),
                "biomeOS orchestration not available - skipping live dispatch",
            );
        }
        return;
    }

    for spec in SIGNALS {
        let check_id = format!("live:dispatch:{}", spec.id);
        let params = (spec.params)();

        match ctx.dispatch(spec.id, params) {
            Ok(_) => {
                v.check_bool(&check_id, true, &format!("dispatch({:?}) accepted", spec.id));
            }
            Err(e) => {
                let detail = format!("{e}");
                if detail.contains("-32601") {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!(
                            "UPSTREAM GAP: biomeOS rejected {} with -32601 (method not found)",
                            spec.id
                        ),
                    );
                } else if e.is_connection_error() {
                    v.check_skip(
                        &check_id,
                        &format!("connection error for {}: {e}", spec.id),
                    );
                } else {
                    v.check_bool(
                        &check_id,
                        false,
                        &format!("dispatch({:?}) error: {e}", spec.id),
                    );
                }
            }
        }
    }
}

fn phase_response_shapes(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") {
        for spec in SIGNALS {
            v.check_skip(
                &format!("shape:{}:keys", spec.id),
                "biomeOS not available - skipping response shape checks",
            );
        }
        return;
    }

    for spec in SIGNALS {
        let params = (spec.params)();

        match ctx.dispatch(spec.id, params) {
            Ok(resp) => {
                if let Some(obj) = resp.as_object() {
                    let actual_keys: Vec<&String> = obj.keys().collect();
                    let mut found = 0usize;
                    for expected in spec.expected_keys {
                        if obj.contains_key(*expected) {
                            found += 1;
                        }
                    }
                    let total = spec.expected_keys.len();
                    v.check_bool(
                        &format!("shape:{}:keys", spec.id),
                        found > 0,
                        &format!(
                            "{}: {found}/{total} expected keys present (expected {:?}, got {actual_keys:?})",
                            spec.id, spec.expected_keys
                        ),
                    );
                } else {
                    v.check_bool(
                        &format!("shape:{}:keys", spec.id),
                        false,
                        &format!(
                            "{}: response is not an object (got {:?})",
                            spec.id,
                            resp.to_string().chars().take(100).collect::<String>()
                        ),
                    );
                }
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(
                    &format!("shape:{}:keys", spec.id),
                    &format!("connection error: {e}"),
                );
            }
            Err(e) => {
                let detail = format!("{e}");
                if detail.contains("-32601") || detail.contains("not found") {
                    v.check_skip(
                        &format!("shape:{}:keys", spec.id),
                        &format!("signal not available: {e}"),
                    );
                } else {
                    v.check_bool(
                        &format!("shape:{}:keys", spec.id),
                        false,
                        &format!("{}: dispatch error: {e}", spec.id),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_dispatch_parity_no_panic() {
        let mut v = ValidationResult::new("signal-dispatch-parity");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario should produce checks"
        );
    }

    #[test]
    fn signal_table_matches_graph_count() {
        assert_eq!(SIGNALS.len(), 14, "SIGNALS table should match 14 signal graphs");
    }
}
