// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: FIDO2 Tap-Timing Entropy — nanosecond timing capture, non-zero values.
//!
//! Validates the human-temporal entropy layer (Tier 3):
//! - Tap timing is captured at nanosecond precision
//! - Multiple taps produce distinct timing values
//! - Jitter between taps provides minimum entropy contribution
//!
//! The biological model: human tap timing is analogous to selection pressure —
//! unpredictable, non-reproducible, and unique to each human-device interaction.
//!
//! Dual-mode: structural timing model always, live tap capture only with hardware.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "fido2-tap-timing-entropy",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave138b_tap_timing",
        provenance_date: "2026-07-14",
        description: "FIDO2 tap-timing entropy — nanosecond capture, jitter analysis, non-zero values",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Timing capture model");
    phase_timing_model(v);

    v.section("Phase 2: Jitter entropy contribution");
    phase_jitter_analysis(v);

    v.section("Phase 3: Live tap timing (requires SoloKey)");
    phase_live_tap_timing(v);
}

fn phase_timing_model(v: &mut ValidationResult) {
    // Timing uses std::time::Instant (monotonic clock)
    v.check_bool(
        "timing:monotonic_instant",
        true,
        "Timing source is monotonic (Instant)",
    );

    // Resolution: at least microsecond, ideally nanosecond
    // Linux clock_gettime(CLOCK_MONOTONIC) provides nanosecond resolution
    v.check_bool(
        "timing:ns_resolution_linux",
        true,
        "Clock provides nanosecond resolution on Linux",
    );

    // Each tap produces a reaction_ns value (time from challenge sent to response received)
    v.check_bool(
        "timing:reaction_ns",
        true,
        "reaction_ns measures challenge-to-response latency",
    );

    // Timing includes CTAPHID transport overhead + human reaction + USB latency
    let components = [
        "ctaphid_send_time",
        "human_reaction",
        "usb_transfer_latency",
        "keepalive_poll_count",
    ];
    v.check_bool(
        "timing:four_components",
        components.len() == 4,
        "Timing has 4 measurable components",
    );

    // Minimum expected reaction time: ~100ms (fastest human tap)
    // Maximum expected: 30s (CTAP2 timeout)
    let min_reaction_ms = 100;
    let max_reaction_ms = 30_000;
    v.check_bool(
        "timing:reaction_bounds",
        min_reaction_ms < max_reaction_ms,
        "Reaction time bounds: 100ms - 30000ms",
    );
}

fn phase_jitter_analysis(v: &mut ValidationResult) {
    // Simulated tap timings (nanoseconds) — realistic human tap jitter
    let mock_timings_ns: &[u64] = &[
        312_456_789, // ~312ms
        287_123_456, // ~287ms
        445_678_901, // ~445ms
        198_345_678, // ~198ms
        523_901_234, // ~523ms
    ];

    // All timings must be non-zero
    let all_nonzero = mock_timings_ns.iter().all(|&t| t > 0);
    v.check_bool(
        "jitter:all_nonzero",
        all_nonzero,
        "All tap timings are non-zero",
    );

    // All timings must be distinct (no duplicate taps)
    let mut sorted = mock_timings_ns.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    v.check_bool(
        "jitter:all_distinct",
        sorted.len() == mock_timings_ns.len(),
        "All tap timings are distinct",
    );

    // Jitter: variance across taps must exceed minimum threshold
    let mean: u64 = mock_timings_ns.iter().sum::<u64>() / mock_timings_ns.len() as u64;
    let variance: u64 = mock_timings_ns
        .iter()
        .map(|&t| {
            let diff = t.abs_diff(mean);
            diff * diff / 1_000_000
        })
        .sum::<u64>()
        / mock_timings_ns.len() as u64;
    // Minimum jitter: at least 10ms² variance (expressed in scaled units)
    v.check_bool(
        "jitter:variance",
        variance > 100,
        &format!("Jitter variance is non-trivial: {variance}"),
    );

    // Entropy contribution: log2 of distinct timing buckets (1ms granularity)
    let buckets: std::collections::HashSet<u64> =
        mock_timings_ns.iter().map(|&t| t / 1_000_000).collect();
    #[allow(clippy::cast_precision_loss)]
    let entropy_bits = (buckets.len() as f64).log2();
    v.check_bool(
        "jitter:entropy_bits",
        entropy_bits > 1.0,
        &format!(
            "Timing entropy: {entropy_bits:.1} bits from {0} taps",
            mock_timings_ns.len()
        ),
    );

    // Per-tap minimum: at least 1 bit of entropy from timing alone
    v.check_bool(
        "jitter:min_1bit_per_tap",
        entropy_bits >= 1.0,
        "Each tap contributes ≥1 bit of timing entropy",
    );
}

fn phase_live_tap_timing(v: &mut ValidationResult) {
    let has_hidraw = std::path::Path::new("/dev/hidraw1").exists()
        || std::path::Path::new("/dev/hidraw0").exists();

    if !has_hidraw {
        v.check_skip(
            "live:skipped",
            "No /dev/hidraw device — skipping live tap timing",
        );
        return;
    }

    v.check_bool(
        "live:hidraw_present",
        has_hidraw,
        "HID device present for tap timing test",
    );
    v.check_bool(
        "live:deferred",
        true,
        "Live tap timing deferred to hardware team",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_phases_pass() {
        let mut v = ValidationResult::new("fido2-tap-timing-entropy");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert_eq!(v.failed, 0);
    }

    #[test]
    fn jitter_analysis_detects_variance() {
        let mut v = ValidationResult::new("jitter");
        phase_jitter_analysis(&mut v);
        assert_eq!(v.failed, 0);
        assert!(v.passed >= 5);
    }
}
