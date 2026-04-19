// SPDX-License-Identifier: AGPL-3.0-or-later

//! Provenance trio Neural API call sites for **experiments** and integration tests.
//!
//! This crate holds capability routing for the provenance trio so the
//! `primalspring` library [`provenance`](primalspring::ipc::provenance) module
//! stays limited to shared JSON types.

#![expect(clippy::expect_used, clippy::unwrap_used, reason = "experiment helper — panics acceptable")]

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{Duration, Instant};

use primalspring::ipc::discover::{capability_call, neural_api_healthy};
use primalspring::ipc::provenance::{PipelineResult, ProvenanceResult, ProvenanceStatus};
use primalspring::tolerances;

// ── Epoch-based circuit breaker for provenance trio (healthSpring pattern) ──

/// Consecutive failure counter for the provenance trio circuit.
static TRIO_FAILURE_COUNT: AtomicU32 = AtomicU32::new(0);

/// When the circuit last tripped (`Some`) — used for cooldown / half-open.
static TRIO_OPENED_AT: Mutex<Option<Instant>> = Mutex::new(None);

/// `true` while a half-open probe is in flight (blocks other callers).
static TRIO_HALF_OPEN_PROBE: AtomicBool = AtomicBool::new(false);

const fn circuit_breaker_timeout() -> Duration {
    Duration::from_secs(tolerances::CIRCUIT_BREAKER_TIMEOUT_SECS)
}

/// Record a successful provenance trio call — resets the failure counter.
fn trio_record_success() {
    TRIO_FAILURE_COUNT.store(0, Ordering::Relaxed);
    if let Ok(mut guard) = TRIO_OPENED_AT.lock() {
        *guard = None;
    }
    TRIO_HALF_OPEN_PROBE.store(false, Ordering::Release);
}

/// Record a failed provenance trio call — increments the failure counter.
fn trio_record_failure() {
    let prev = TRIO_FAILURE_COUNT.fetch_add(1, Ordering::Relaxed);
    let new = prev.saturating_add(1);
    if new == tolerances::CIRCUIT_BREAKER_THRESHOLD {
        if let Ok(mut guard) = TRIO_OPENED_AT.lock() {
            *guard = Some(Instant::now());
        }
    }
    if TRIO_HALF_OPEN_PROBE.load(Ordering::Acquire) {
        if let Ok(mut guard) = TRIO_OPENED_AT.lock() {
            *guard = Some(Instant::now());
        }
        TRIO_HALF_OPEN_PROBE.store(false, Ordering::Release);
    }
}

/// Whether callers should short-circuit without attempting a trio call.
#[must_use]
fn trio_circuit_is_open() -> bool {
    let count = TRIO_FAILURE_COUNT.load(Ordering::Relaxed);
    if count < tolerances::CIRCUIT_BREAKER_THRESHOLD {
        return false;
    }
    let Ok(guard) = TRIO_OPENED_AT.lock() else {
        return true;
    };
    let cooldown_active = guard.is_none_or(|opened| opened.elapsed() < circuit_breaker_timeout());
    drop(guard);
    if cooldown_active {
        return true;
    }
    TRIO_HALF_OPEN_PROBE.load(Ordering::Acquire)
}

#[must_use]
fn resilient_capability_call(
    domain: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Option<serde_json::Value> {
    if trio_circuit_is_open() {
        return None;
    }

    let count = TRIO_FAILURE_COUNT.load(Ordering::Relaxed);
    if count >= tolerances::CIRCUIT_BREAKER_THRESHOLD
        && TRIO_HALF_OPEN_PROBE
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
    {
        return None;
    }

    let half_open_probe = TRIO_FAILURE_COUNT.load(Ordering::Relaxed)
        >= tolerances::CIRCUIT_BREAKER_THRESHOLD
        && TRIO_HALF_OPEN_PROBE.load(Ordering::Acquire);

    let backoff_base = Duration::from_millis(tolerances::TRIO_RETRY_BASE_DELAY_MS);
    let max_attempts = if half_open_probe {
        0
    } else {
        tolerances::TRIO_RETRY_ATTEMPTS
    };

    for attempt in 0..=max_attempts {
        if let Some(result) = capability_call(domain, operation, args) {
            trio_record_success();
            return Some(result);
        }

        trio_record_failure();

        if attempt < max_attempts {
            let delay = backoff_base.saturating_mul(1u32.wrapping_shl(attempt));
            std::thread::sleep(delay);
        }
    }

    None
}

#[cfg(test)]
fn reset_trio_circuit() {
    TRIO_FAILURE_COUNT.store(0, Ordering::Relaxed);
    *TRIO_OPENED_AT.lock().expect("TRIO_OPENED_AT poisoned") = None;
    TRIO_HALF_OPEN_PROBE.store(false, Ordering::Release);
}

fn extract_id(value: &serde_json::Value, key: &str, alt: &str) -> String {
    value
        .get(key)
        .or_else(|| value.get(alt))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_owned()
}

fn ok_result(data: serde_json::Value, key: &str, alt: &str) -> ProvenanceResult {
    let id = extract_id(&data, key, alt);
    ProvenanceResult {
        id,
        status: ProvenanceStatus::Complete,
        data,
    }
}

/// Check whether all three provenance trio capability domains are routable.
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
#[must_use]
pub fn complete_experiment(session_id: &str) -> PipelineResult {
    let empty_pipeline = |status| PipelineResult {
        status,
        session_id: session_id.to_owned(),
        merkle_root: String::new(),
        commit_id: String::new(),
        braid_id: String::new(),
    };

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

/// Execute `RootPulse` branch operation via Neural API.
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

/// Execute `RootPulse` merge operation via Neural API.
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

/// Execute `RootPulse` diff operation via Neural API.
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

/// Execute `RootPulse` federate operation via Neural API.
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
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    static TRIO_TEST_MUTEX: Mutex<()> = Mutex::new(());

    fn sleep_past_circuit_breaker_timeout() {
        thread::sleep(
            Duration::from_secs(tolerances::CIRCUIT_BREAKER_TIMEOUT_SECS)
                + Duration::from_millis(100),
        );
    }

    #[test]
    fn trio_available_false_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        assert!(!trio_available());
    }

    #[test]
    fn trio_health_returns_three_domains() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let health = trio_health();
        assert_eq!(health.len(), 3);
        assert_eq!(health[0].0, "dag");
        assert_eq!(health[1].0, "commit");
        assert_eq!(health[2].0, "provenance");
    }

    #[test]
    fn begin_session_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let r = begin_experiment_session("test-exp");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn record_step_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let step = serde_json::json!({ "action": "validate", "result": "pass" });
        let r = record_experiment_step("sess-1", &step);
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn complete_experiment_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let r = complete_experiment("sess-1");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
        assert!(r.merkle_root.is_empty());
        assert!(r.commit_id.is_empty());
        assert!(r.braid_id.is_empty());
    }

    #[test]
    fn rootpulse_branch_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let r = rootpulse_branch("sess-1", "feature-x");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn rootpulse_merge_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let r = rootpulse_merge("branch-1", "main-1");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn rootpulse_diff_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let r = rootpulse_diff("sess-a", "sess-b");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn rootpulse_federate_unavailable_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        let r = rootpulse_federate("sess-1", "https://remote.example.com");
        assert_eq!(r.status, ProvenanceStatus::Unavailable);
    }

    #[test]
    fn trio_circuit_starts_closed() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        assert!(!trio_circuit_is_open());
    }

    #[test]
    fn trio_circuit_opens_after_threshold() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        for _ in 0..tolerances::CIRCUIT_BREAKER_THRESHOLD {
            trio_record_failure();
        }
        assert!(trio_circuit_is_open());
        reset_trio_circuit();
    }

    #[test]
    fn trio_circuit_resets_on_success() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        trio_record_failure();
        trio_record_failure();
        trio_record_success();
        assert!(!trio_circuit_is_open());
    }

    #[test]
    fn resilient_capability_call_returns_none_without_biomeos() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        let result = resilient_capability_call("dag", "health", &serde_json::json!({}));
        assert!(result.is_none());
    }

    #[test]
    fn trio_circuit_half_open_after_cooldown() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        for _ in 0..tolerances::CIRCUIT_BREAKER_THRESHOLD {
            trio_record_failure();
        }
        assert!(trio_circuit_is_open());
        sleep_past_circuit_breaker_timeout();
        assert!(!trio_circuit_is_open());
        reset_trio_circuit();
    }

    #[test]
    fn trio_circuit_failed_probe_restarts_cooldown() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        for _ in 0..tolerances::CIRCUIT_BREAKER_THRESHOLD {
            trio_record_failure();
        }
        sleep_past_circuit_breaker_timeout();
        assert!(!trio_circuit_is_open());
        let result = resilient_capability_call("dag", "health", &serde_json::json!({}));
        assert!(result.is_none());
        assert!(trio_circuit_is_open());
        reset_trio_circuit();
    }

    #[test]
    fn trio_circuit_half_open_only_one_probe_wins_cas() {
        let _lock = TRIO_TEST_MUTEX.lock().expect("TRIO_TEST_MUTEX poisoned");
        reset_trio_circuit();
        for _ in 0..tolerances::CIRCUIT_BREAKER_THRESHOLD {
            trio_record_failure();
        }
        sleep_past_circuit_breaker_timeout();

        let barrier = Arc::new(Barrier::new(2));
        let b1 = Arc::clone(&barrier);
        let b2 = Arc::clone(&barrier);
        let h1 = thread::spawn(move || {
            b1.wait();
            resilient_capability_call("dag", "health", &serde_json::json!({}))
        });
        let h2 = thread::spawn(move || {
            b2.wait();
            resilient_capability_call("dag", "health", &serde_json::json!({}))
        });
        let _ = h1.join().unwrap();
        let _ = h2.join().unwrap();
        assert!(trio_circuit_is_open());
        reset_trio_circuit();
    }
}
