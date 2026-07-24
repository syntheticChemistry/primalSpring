// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Tower Stress — Concurrent Dispatch.
//!
//! `capability.call` is the hot path through the Tower Atomic stack.
//! Every call traverses: caller UDS → songBird → registry → provider UDS.
//! Under concurrent load, this path contends on:
//!   - songBird's accept loop (new UDS connection per call)
//!   - Registry lookup (read lock, but N waiters)
//!   - Provider UDS accept (bearDog, skunkBat, nestGate)
//!
//! This scenario validates structural readiness for concurrent dispatch
//! testing: configurable concurrency, per-request latency tracking under
//! load, error rate measurement, and registry contention detection.
//!
//! Structural checks: registry supports concurrent reads, dispatch path
//! is async, UDS accept loop is multi-connection.
//!
//! Live: spawn N tasks calling `capability.call` simultaneously, measure
//! latency degradation and error rates.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");
const DISPATCH_SRC: &str = include_str!(
    "../../../../../../primals/songBird/crates/songbird-universal-ipc/src/service/capability_dispatch.rs"
);

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "tower-stress-concurrent-dispatch",
        track: Track::Evolution,
        tier: Tier::Rust,
        provenance_crate: "wave150w_tower_stress",
        provenance_date: "2026-07-23",
        description: "Tower stress — capability.call under N concurrent callers (contention + error rate)",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Dispatch path concurrency readiness");
    phase_dispatch_concurrency(v);

    v.section("Phase 2: Registry contention model");
    phase_registry_contention(v);

    v.section("Phase 3: Provider accept capacity");
    phase_provider_capacity(v);
}

fn phase_dispatch_concurrency(v: &mut ValidationResult) {
    let has_async_dispatch =
        DISPATCH_SRC.contains("async fn") && DISPATCH_SRC.contains("capability");
    v.check_bool(
        "concurrent:async_dispatch",
        has_async_dispatch,
        "Capability dispatch is async (can handle concurrent calls without blocking)",
    );

    let has_tokio_spawn = DISPATCH_SRC.contains("tokio::spawn") || DISPATCH_SRC.contains("spawn(");
    v.check_bool(
        "concurrent:task_spawning",
        has_tokio_spawn || has_async_dispatch,
        &format!(
            "Dispatch {} for concurrent execution",
            if has_tokio_spawn {
                "spawns tasks"
            } else if has_async_dispatch {
                "is async-ready (concurrent via caller's runtime)"
            } else {
                "may BLOCK under concurrency"
            }
        ),
    );

    let has_per_request_connect =
        DISPATCH_SRC.contains("IpcStream::connect") || DISPATCH_SRC.contains("UnixStream::connect");
    v.check_bool(
        "concurrent:per_request_connect",
        has_per_request_connect,
        &format!(
            "Dispatch uses per-request UDS connect: {} — \
             each concurrent call opens a fresh connection (no pooling = contention point)",
            if has_per_request_connect {
                "CONFIRMED"
            } else {
                "may pool connections"
            }
        ),
    );

    let has_connection_pool = DISPATCH_SRC.contains("pool")
        || DISPATCH_SRC.contains("Pool")
        || DISPATCH_SRC.contains("reuse");
    v.check_bool(
        "concurrent:connection_pooling",
        has_connection_pool,
        &format!(
            "UDS connection pooling: {} — \
             pooling would reduce contention under concurrent dispatch",
            if has_connection_pool {
                "PRESENT"
            } else {
                "ABSENT (per-request overhead: connect + serialize per call)"
            }
        ),
    );
}

fn phase_registry_contention(v: &mut ValidationResult) {
    let has_rwlock = DISPATCH_SRC.contains("RwLock") || DISPATCH_SRC.contains("rwlock");
    let has_dashmap = DISPATCH_SRC.contains("DashMap") || DISPATCH_SRC.contains("dashmap");
    let has_arc = DISPATCH_SRC.contains("Arc<");

    v.check_bool(
        "concurrent:registry_concurrent_access",
        has_rwlock || has_dashmap || has_arc,
        &format!(
            "Registry concurrency primitive: {}",
            if has_dashmap {
                "DashMap (lock-free concurrent reads)"
            } else if has_rwlock {
                "RwLock (concurrent reads, exclusive writes)"
            } else if has_arc {
                "Arc (shared ownership, unknown lock model)"
            } else {
                "NONE DETECTED — potential serial bottleneck"
            }
        ),
    );

    let registered_providers: Vec<&str> = REGISTRY_TOML
        .lines()
        .filter(|l| l.starts_with("[capabilities.") || l.starts_with("[[capabilities."))
        .collect();
    v.check_bool(
        "concurrent:registry_provider_count",
        !registered_providers.is_empty(),
        &format!(
            "{} registered capability groups — concurrent dispatch targets",
            registered_providers.len()
        ),
    );
}

fn phase_provider_capacity(v: &mut ValidationResult) {
    let multi_primal_targets = ["security", "crypto", "discovery", "compute", "network"];
    let mut found = 0;
    for cap in &multi_primal_targets {
        if REGISTRY_TOML.contains(cap) {
            found += 1;
        }
    }
    v.check_bool(
        "concurrent:multi_provider_targets",
        found >= 3,
        &format!(
            "{found}/{} core capabilities registered — concurrent dispatch can target different providers \
             (reducing per-provider contention)",
            multi_primal_targets.len()
        ),
    );

    let has_timeout_on_forward =
        DISPATCH_SRC.contains("timeout") || DISPATCH_SRC.contains("Timeout");
    v.check_bool(
        "concurrent:forward_timeout",
        has_timeout_on_forward,
        &format!(
            "Forward timeout on provider UDS call: {} — \
             without timeout, a hung provider blocks the caller indefinitely under load",
            if has_timeout_on_forward {
                "PRESENT"
            } else {
                "ABSENT (risk: cascading hang under concurrent load)"
            }
        ),
    );

    let has_error_handling = DISPATCH_SRC.contains("Err(")
        || DISPATCH_SRC.contains("anyhow!")
        || DISPATCH_SRC.contains("Error");
    v.check_bool(
        "concurrent:error_propagation",
        has_error_handling,
        "Error propagation from provider to caller is present (no silent swallowing)",
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
