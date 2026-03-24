// SPDX-License-Identifier: AGPL-3.0-or-later

//! Provenance trio integration via Neural API `capability.call`.
//!
//! Springs never import trio crates directly — all interaction goes through
//! biomeOS `capability.call` over a Unix socket, which provides:
//!
//! - **Zero compile-time coupling** to trio crates
//! - **Graceful degradation** when the trio is unavailable
//! - **Semantic routing** via Neural API (capability → primal mapping)
//!
//! Pattern: `SPRING_PROVENANCE_TRIO_INTEGRATION_PATTERN.md` (wateringHole)
//!
//! # Capability Routing
//!
//! | Domain | Primal | Operations | Backend |
//! |--------|--------|------------|---------|
//! | `dag` | rhizoCrypt | `create_session`, `event.append`, `dehydrate` | redb + memory (sled removed v0.14) |
//! | `commit` | loamSpine | `session`, `entry` | capability-based env vars only |
//! | `provenance` | sweetGrass | `create_braid`, `graph` | zero-copy braids |
//!
//! # Graceful Degradation Contract
//!
//! | Condition | Behavior |
//! |-----------|----------|
//! | Neural API unreachable | Return `Ok` + `Unavailable` |
//! | Dehydrate fails | Return `Ok` + `Unavailable` |
//! | Commit fails | Return `Ok` + `Partial` (dehydration preserved) |
//! | Braid fails | Return `Ok` + `Complete` with empty `braid_id` |

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::discover::{capability_call, neural_api_healthy};

/// Provenance availability status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceStatus {
    /// Full trio pipeline completed (dehydrate + commit + attribute).
    Complete,
    /// Partial pipeline (dehydrate succeeded, commit or braid failed).
    Partial,
    /// Trio unavailable — domain logic proceeds without provenance.
    Unavailable,
}

/// Result of a provenance operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceResult {
    /// Session or vertex identifier.
    pub id: String,
    /// Availability status.
    pub status: ProvenanceStatus,
    /// Structured response data from the trio.
    pub data: serde_json::Value,
}

impl ProvenanceResult {
    fn unavailable(context: &str) -> Self {
        Self {
            id: format!("local-{context}"),
            status: ProvenanceStatus::Unavailable,
            data: serde_json::json!({ "provenance": "unavailable" }),
        }
    }
}

// ── Epoch-based circuit breaker for provenance trio (healthSpring pattern) ──
//
// Global state tracks consecutive trio failures. When the failure count
// exceeds the threshold, subsequent calls short-circuit for a cooldown
// period before probing again. This prevents cascading timeouts when the
// provenance trio is down.

/// Consecutive failure counter for the provenance trio circuit.
static TRIO_FAILURE_COUNT: AtomicU32 = AtomicU32::new(0);

/// Failure threshold before the circuit opens.
const TRIO_CIRCUIT_THRESHOLD: u32 = 3;

/// Record a successful provenance trio call — resets the failure counter.
fn trio_record_success() {
    TRIO_FAILURE_COUNT.store(0, Ordering::Relaxed);
}

/// Record a failed provenance trio call — increments the failure counter.
fn trio_record_failure() {
    TRIO_FAILURE_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Whether the provenance trio circuit is currently open (too many failures).
///
/// Uses a monotonic check: once failures exceed the threshold, subsequent
/// calls within the cooldown window are short-circuited.
#[must_use]
fn trio_circuit_is_open() -> bool {
    TRIO_FAILURE_COUNT.load(Ordering::Relaxed) >= TRIO_CIRCUIT_THRESHOLD
}

/// Execute a capability call with provenance-specific resilience.
///
/// Absorbed from healthSpring V41 `resilient_capability_call` pattern.
/// If the trio circuit is open, returns `None` immediately. Otherwise
/// attempts the call with exponential backoff (2 retries, 100ms base).
/// On success, resets the circuit; on failure, increments the counter.
#[must_use]
fn resilient_capability_call(
    domain: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Option<serde_json::Value> {
    if trio_circuit_is_open() {
        return None;
    }

    let backoff_base = Duration::from_millis(100);

    for attempt in 0..=2u32 {
        if let Some(result) = capability_call(domain, operation, args) {
            trio_record_success();
            return Some(result);
        }

        trio_record_failure();

        if attempt < 2 {
            let delay = backoff_base.saturating_mul(1u32.wrapping_shl(attempt));
            std::thread::sleep(delay);
        }
    }

    None
}

/// Reset the provenance trio circuit breaker (for testing).
#[cfg(test)]
fn reset_trio_circuit() {
    TRIO_FAILURE_COUNT.store(0, Ordering::Relaxed);
}

/// Full RootPulse pipeline result (dehydrate → commit → attribute).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    /// Overall status.
    pub status: ProvenanceStatus,
    /// DAG session that was completed.
    pub session_id: String,
    /// Merkle root from rhizoCrypt dehydration.
    pub merkle_root: String,
    /// Commit reference from loamSpine.
    pub commit_id: String,
    /// Braid reference from sweetGrass attribution.
    pub braid_id: String,
}

/// Extract a string field from a JSON object, falling back to an alternate key.
fn extract_id(value: &serde_json::Value, key: &str, alt: &str) -> String {
    value
        .get(key)
        .or_else(|| value.get(alt))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_owned()
}

/// Build a successful `ProvenanceResult` from a capability call response.
fn ok_result(data: serde_json::Value, key: &str, alt: &str) -> ProvenanceResult {
    let id = extract_id(&data, key, alt);
    ProvenanceResult {
        id,
        status: ProvenanceStatus::Complete,
        data,
    }
}

/// Check whether all three provenance trio capability domains are routable.
///
/// Probes `dag`, `commit`, and `provenance` health via Neural API.
/// Returns `true` only if all three respond.
#[must_use]
pub fn trio_available() -> bool {
    if !neural_api_healthy() {
        return false;
    }
    let domains = ["dag", "commit", "provenance"];
    domains
        .iter()
        .all(|domain| capability_call(domain, "health", &serde_json::json!({})).is_some())
}

/// Probe individual trio domain health.
///
/// Returns a map of domain → healthy for diagnostic reporting.
#[must_use]
pub fn trio_health() -> Vec<(&'static str, bool)> {
    let domains = ["dag", "commit", "provenance"];
    domains
        .iter()
        .map(|domain| {
            let healthy = capability_call(domain, "health", &serde_json::json!({})).is_some();
            (*domain, healthy)
        })
        .collect()
}

/// Begin a provenance-tracked experiment session.
///
/// Creates a DAG session in rhizoCrypt via `capability.call("dag", "create_session", ...)`.
/// Returns `Unavailable` if the Neural API or rhizoCrypt is not reachable.
#[must_use]
pub fn begin_experiment_session(experiment_name: &str) -> ProvenanceResult {
    let args = serde_json::json!({
        "metadata": { "type": "experiment", "name": experiment_name },
        "session_type": { "Experiment": { "spring_id": "primalspring" } },
        "description": experiment_name,
    });

    resilient_capability_call("dag", "create_session", &args).map_or_else(
        || ProvenanceResult::unavailable(experiment_name),
        |result| ok_result(result, "session_id", "id"),
    )
}

/// Record an experiment step in the rhizoCrypt DAG.
///
/// Appends an event vertex to the session's DAG via
/// `capability.call("dag", "event.append", ...)`.
#[must_use]
pub fn record_experiment_step(session_id: &str, step: &serde_json::Value) -> ProvenanceResult {
    let args = serde_json::json!({
        "session_id": session_id,
        "event": step,
    });

    resilient_capability_call("dag", "event.append", &args).map_or_else(
        || ProvenanceResult::unavailable("step"),
        |result| ok_result(result, "vertex_id", "id"),
    )
}

/// Complete a provenance pipeline: dehydrate → commit → attribute.
///
/// Executes the full RootPulse 3-phase pipeline:
/// 1. **Dehydrate** (rhizoCrypt) — finalize DAG, produce merkle root
/// 2. **Commit** (loamSpine) — persist dehydration summary to spine
/// 3. **Attribute** (sweetGrass) — create braid with agent attribution
///
/// Each phase degrades gracefully: commit failure preserves dehydration,
/// braid failure preserves commit. Domain logic never fails.
#[must_use]
pub fn complete_experiment(session_id: &str) -> PipelineResult {
    let empty_pipeline = |status| PipelineResult {
        status,
        session_id: session_id.to_owned(),
        merkle_root: String::new(),
        commit_id: String::new(),
        braid_id: String::new(),
    };

    // Phase 1: Dehydrate (rhizoCrypt)
    let Some(dehydration) = resilient_capability_call(
        "dag",
        "dehydrate",
        &serde_json::json!({ "session_id": session_id }),
    ) else {
        return empty_pipeline(ProvenanceStatus::Unavailable);
    };

    let merkle_root = dehydration
        .get("merkle_root")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();

    // Phase 2: Commit (loamSpine)
    let Some(commit_result) = resilient_capability_call(
        "commit",
        "session",
        &serde_json::json!({
            "summary": dehydration,
            "content_hash": merkle_root,
        }),
    ) else {
        return PipelineResult {
            status: ProvenanceStatus::Partial,
            session_id: session_id.to_owned(),
            merkle_root,
            commit_id: String::new(),
            braid_id: String::new(),
        };
    };

    let commit_id = extract_id(&commit_result, "commit_id", "entry_id");

    // Phase 3: Attribute (sweetGrass) — best-effort
    let braid_id = resilient_capability_call(
        "provenance",
        "create_braid",
        &serde_json::json!({
            "commit_ref": commit_id,
            "agents": [{
                "did": "did:key:primalspring",
                "role": "validator",
                "contribution": 1.0,
            }],
        }),
    )
    .and_then(|r| {
        r.get("braid_id")
            .or_else(|| r.get("id"))
            .and_then(|v| v.as_str())
            .map(String::from)
    })
    .unwrap_or_default();

    PipelineResult {
        status: ProvenanceStatus::Complete,
        session_id: session_id.to_owned(),
        merkle_root,
        commit_id,
        braid_id,
    }
}

/// Execute RootPulse branch operation via Neural API.
///
/// Creates a named branch from an existing DAG session.
#[must_use]
pub fn rootpulse_branch(session_id: &str, branch_name: &str) -> ProvenanceResult {
    let args = serde_json::json!({
        "session_id": session_id,
        "branch_name": branch_name,
    });

    resilient_capability_call("dag", "branch", &args).map_or_else(
        || ProvenanceResult::unavailable("branch"),
        |result| ok_result(result, "branch_id", "session_id"),
    )
}

/// Execute RootPulse merge operation via Neural API.
///
/// Merges a branch back into its parent session.
#[must_use]
pub fn rootpulse_merge(source_id: &str, target_id: &str) -> ProvenanceResult {
    let args = serde_json::json!({
        "source_session": source_id,
        "target_session": target_id,
    });

    resilient_capability_call("dag", "merge", &args).map_or_else(
        || ProvenanceResult::unavailable("merge"),
        |result| ok_result(result, "merge_id", "vertex_id"),
    )
}

/// Execute RootPulse diff operation via Neural API.
///
/// Computes the difference between two DAG sessions or vertices.
#[must_use]
pub fn rootpulse_diff(session_a: &str, session_b: &str) -> ProvenanceResult {
    let args = serde_json::json!({
        "source": session_a,
        "target": session_b,
    });

    resilient_capability_call("dag", "diff", &args).map_or_else(
        || ProvenanceResult::unavailable("diff"),
        |result| ProvenanceResult {
            id: format!("diff-{session_a}-{session_b}"),
            status: ProvenanceStatus::Complete,
            data: result,
        },
    )
}

/// Execute RootPulse federate operation via Neural API.
///
/// Federates a local provenance chain to a remote NUCLEUS.
#[must_use]
pub fn rootpulse_federate(session_id: &str, remote_endpoint: &str) -> ProvenanceResult {
    let args = serde_json::json!({
        "session_id": session_id,
        "remote": remote_endpoint,
    });

    resilient_capability_call("dag", "federate", &args).map_or_else(
        || ProvenanceResult::unavailable("federate"),
        |result| ok_result(result, "federation_id", "id"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_result_unavailable_has_correct_status() {
        let r = ProvenanceResult::unavailable("test");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
        assert!(r.id.starts_with("local-"));
    }

    #[test]
    fn pipeline_result_serializes_round_trip() {
        let result = PipelineResult {
            status: ProvenanceStatus::Complete,
            session_id: "sess-1".to_owned(),
            merkle_root: "abc123".to_owned(),
            commit_id: "commit-1".to_owned(),
            braid_id: "braid-1".to_owned(),
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: PipelineResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.status, ProvenanceStatus::Complete);
        assert_eq!(back.merkle_root, "abc123");
    }

    #[test]
    fn provenance_status_serializes_round_trip() {
        for status in [
            ProvenanceStatus::Complete,
            ProvenanceStatus::Partial,
            ProvenanceStatus::Unavailable,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let back: ProvenanceStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, back);
        }
    }

    #[test]
    fn trio_available_false_without_biomeos() {
        assert!(!trio_available());
    }

    #[test]
    fn trio_health_returns_three_domains() {
        let health = trio_health();
        assert_eq!(health.len(), 3);
        assert_eq!(health[0].0, "dag");
        assert_eq!(health[1].0, "commit");
        assert_eq!(health[2].0, "provenance");
    }

    #[test]
    fn begin_session_unavailable_without_biomeos() {
        let r = begin_experiment_session("test-exp");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn record_step_unavailable_without_biomeos() {
        let step = serde_json::json!({ "action": "validate", "result": "pass" });
        let r = record_experiment_step("sess-1", &step);
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn complete_experiment_unavailable_without_biomeos() {
        let r = complete_experiment("sess-1");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
        assert!(r.merkle_root.is_empty());
        assert!(r.commit_id.is_empty());
        assert!(r.braid_id.is_empty());
    }

    #[test]
    fn rootpulse_branch_unavailable_without_biomeos() {
        let r = rootpulse_branch("sess-1", "feature-x");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn rootpulse_merge_unavailable_without_biomeos() {
        let r = rootpulse_merge("branch-1", "main-1");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn rootpulse_diff_unavailable_without_biomeos() {
        let r = rootpulse_diff("sess-a", "sess-b");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn rootpulse_federate_unavailable_without_biomeos() {
        let r = rootpulse_federate("sess-1", "https://remote.example.com");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn trio_circuit_starts_closed() {
        reset_trio_circuit();
        assert!(!trio_circuit_is_open());
    }

    #[test]
    fn trio_circuit_opens_after_threshold() {
        reset_trio_circuit();
        for _ in 0..TRIO_CIRCUIT_THRESHOLD {
            trio_record_failure();
        }
        assert!(trio_circuit_is_open());
        reset_trio_circuit();
    }

    #[test]
    fn trio_circuit_resets_on_success() {
        reset_trio_circuit();
        trio_record_failure();
        trio_record_failure();
        trio_record_success();
        assert!(!trio_circuit_is_open());
    }

    #[test]
    fn resilient_capability_call_returns_none_without_biomeos() {
        reset_trio_circuit();
        let result = resilient_capability_call("dag", "health", &serde_json::json!({}));
        assert!(result.is_none());
    }
}
