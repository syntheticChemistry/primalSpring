// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: WAN IPC Tolerance — validates that IPC round-trips complete
//! within acceptable latency over high-latency links.
//!
//! Divergence pressure: LAN gates enjoy sub-millisecond IPC. WAN gates
//! (flockGate, future ionic/weak bonds) face 10-200ms network RTT on top
//! of primal processing time. This scenario measures actual IPC latency
//! and validates it stays within configurable tolerance thresholds.
//!
//! Phase 1 (Structural): Verify tolerance constants are defined.
//! Phase 2 (Live): Measure actual IPC round-trip times and compare.

use std::time::Instant;

use crate::composition::CompositionContext;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// WAN IPC tolerance scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "wan-ipc-tolerance",
        track: Track::Transport,
        tier: Tier::Both,
        provenance_crate: "wave111_divergence_pressure",
        provenance_date: "2026-06-11",
        description: "Validates IPC round-trips complete within tolerance over high-latency links",
    },
    run,
};

/// Configurable WAN latency threshold (milliseconds).
///
/// On LAN, health checks complete in <10ms. For WAN links (flockGate,
/// ionic bonds), we accept up to 2000ms for a full RPC round-trip including
/// TLS negotiation, TCP RTT, and primal processing.
const WAN_HEALTH_MAX_MS: u64 = 2000;

/// Number of samples to collect for statistical confidence.
const SAMPLE_COUNT: usize = 3;

/// Execute WAN IPC tolerance validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — tolerance definition");
    phase_structural(v);

    v.section("Phase 2: Live — latency measurement");
    phase_live(v, ctx);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "tol:health_check_defined",
        tolerances::HEALTH_CHECK_MAX_US > 0,
        &format!(
            "HEALTH_CHECK_MAX_US = {}µs (local budget)",
            tolerances::HEALTH_CHECK_MAX_US
        ),
    );

    v.check_bool(
        "tol:tcp_connect_defined",
        tolerances::TCP_CONNECT_TIMEOUT_SECS > 0,
        &format!(
            "TCP_CONNECT_TIMEOUT_SECS = {}s",
            tolerances::TCP_CONNECT_TIMEOUT_SECS
        ),
    );

    v.check_bool(
        "tol:tcp_read_defined",
        tolerances::TCP_READ_TIMEOUT_SECS > 0,
        &format!(
            "TCP_READ_TIMEOUT_SECS = {}s",
            tolerances::TCP_READ_TIMEOUT_SECS
        ),
    );

    v.check_bool(
        "tol:wan_budget_defined",
        WAN_HEALTH_MAX_MS > 0,
        &format!("WAN health RPC budget: {WAN_HEALTH_MAX_MS}ms"),
    );

    #[expect(clippy::cast_precision_loss, reason = "tolerance constants are small")]
    let ratio = WAN_HEALTH_MAX_MS as f64 / (tolerances::HEALTH_CHECK_MAX_US as f64 / 1000.0);
    v.check_bool(
        "tol:wan_to_lan_ratio",
        (10.0..=200.0).contains(&ratio),
        &format!(
            "WAN/LAN ratio: {ratio:.1}x (WAN budget is {ratio:.0}x the LAN budget)"
        ),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability("orchestration") && !ctx.has_capability("security") {
        v.check_skip(
            "wan:latency_sample",
            "no reachable capability for latency measurement",
        );
        return;
    }

    let target_cap = if ctx.has_capability("security") {
        "security"
    } else {
        "orchestration"
    };

    let mut latencies_ms = Vec::with_capacity(SAMPLE_COUNT);

    for i in 0..SAMPLE_COUNT {
        let start = Instant::now();
        let result = ctx.call(target_cap, "health", serde_json::json!({}));
        let elapsed_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        match result {
            Ok(_) => {
                latencies_ms.push(elapsed_ms);
            }
            Err(e) => {
                v.check_bool(
                    &format!("wan:sample_{i}"),
                    false,
                    &format!("sample {i} failed: {e}"),
                );
            }
        }
    }

    if latencies_ms.is_empty() {
        v.check_skip(
            "wan:latency_sample",
            "all samples failed — cannot measure latency",
        );
        return;
    }

    let min_ms = *latencies_ms.iter().min().unwrap_or(&0);
    let max_ms = *latencies_ms.iter().max().unwrap_or(&0);
    let avg_ms = latencies_ms.iter().sum::<u64>() / latencies_ms.len() as u64;

    v.check_bool(
        "wan:latency_sample",
        true,
        &format!(
            "{} samples: min={}ms avg={}ms max={}ms (target: {target_cap})",
            latencies_ms.len(),
            min_ms,
            avg_ms,
            max_ms
        ),
    );

    v.check_bool(
        "wan:within_budget",
        max_ms <= WAN_HEALTH_MAX_MS,
        &format!(
            "max latency {max_ms}ms {} budget {WAN_HEALTH_MAX_MS}ms",
            if max_ms <= WAN_HEALTH_MAX_MS {
                "<="
            } else {
                "EXCEEDS"
            }
        ),
    );

    let link_class = match max_ms {
        0..50 => "LAN (<50ms)",
        50..200 => "near-WAN (50-200ms)",
        _ => "WAN (>200ms)",
    };
    v.check_bool(
        "wan:link_classification",
        true,
        &format!("link type: {link_class} (max RTT {max_ms}ms)"),
    );

    let jitter_ms = max_ms.saturating_sub(min_ms);
    let jitter_acceptable = jitter_ms < WAN_HEALTH_MAX_MS / 2;
    v.check_bool(
        "wan:jitter_acceptable",
        jitter_acceptable,
        &format!(
            "jitter: {jitter_ms}ms (max-min), threshold: {}ms",
            WAN_HEALTH_MAX_MS / 2
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn structural_and_live_no_panic() {
        let mut v = ValidationResult::new("wan-ipc-tolerance");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }

    #[test]
    fn wan_budget_exceeds_lan_budget() {
        let lan_ms = tolerances::HEALTH_CHECK_MAX_US / 1000;
        assert!(WAN_HEALTH_MAX_MS > lan_ms * 10);
    }
}
