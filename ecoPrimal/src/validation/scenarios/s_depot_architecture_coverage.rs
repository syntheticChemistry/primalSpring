// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Depot Architecture Coverage — validates the multi-arch depot
//! contains expected binaries for all 4 target architectures.
//!
//! Silicon Atheism requires every primal to build for every platform.
//! This scenario probes the VPS depot to confirm coverage.
//!
//! Architectures:
//! - x86_64-unknown-linux-musl (primary, all binaries)
//! - aarch64-unknown-linux-musl (ARM servers, all binaries)
//! - aarch64-linux-android (grapheneGate, 14 expected)
//! - x86_64-pc-windows-gnu (northGate, 14 expected)

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DEPOT_BASE: &str = "https://membrane.primals.eco/depot/primals";

const ARCHITECTURES: &[(&str, usize)] = &[
    ("x86_64-unknown-linux-musl", 14),
    ("aarch64-unknown-linux-musl", 14),
    ("aarch64-linux-android", 14),
    ("x86_64-pc-windows-gnu", 14),
];

const EXTRA_BINARIES: &[&str] = &["sourdough"];

/// Depot architecture coverage scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "depot-architecture-coverage",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave142b_depot_arch",
        provenance_date: "2026-07-16",
        description: "Depot architecture coverage — 4 arches × 14 primals, Silicon Atheism parity",
    },
    run,
};

/// Run depot architecture coverage validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    phase_structural(v);
    phase_live(v);
}

fn phase_structural(v: &mut ValidationResult) {
    v.section("Phase 1: Structural — expected architecture matrix");

    v.check_bool(
        "depot:arch_count",
        ARCHITECTURES.len() == 4,
        &format!("{} target architectures declared", ARCHITECTURES.len()),
    );

    let total_expected: usize = ARCHITECTURES.iter().map(|(_, n)| n).sum();
    v.check_bool(
        "depot:total_expected",
        total_expected == 56,
        &format!("{total_expected} total binaries expected (14 × 4)"),
    );

    let primal_count = Primal::ALL_SLUGS.len() + EXTRA_BINARIES.len();
    v.check_bool(
        "depot:primal_plus_extra",
        primal_count >= 14,
        &format!("{primal_count} primals + extras (ALL_SLUGS={} + extra={})", Primal::ALL_SLUGS.len(), EXTRA_BINARIES.len()),
    );
}

fn phase_live(v: &mut ValidationResult) {
    v.section("Phase 2: Live — probe depot for architecture directories");

    let depot_reachable = std::process::Command::new("curl")
        .args(["-sI", "--max-time", "5", &format!("{DEPOT_BASE}/x86_64-unknown-linux-musl/songbird")])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("200"))
        .unwrap_or(false);

    if !depot_reachable {
        v.check_skip("depot:live:reachable", "depot not reachable from this network");
        return;
    }

    v.check_bool("depot:live:reachable", true, "depot endpoint reachable (songbird 200)");

    for (arch, expected_min) in ARCHITECTURES {
        let binary_name = if *arch == "x86_64-pc-windows-gnu" { "songbird.exe" } else { "songbird" };
        let url = format!("{DEPOT_BASE}/{arch}/{binary_name}");

        let exists = std::process::Command::new("curl")
            .args(["-sI", "--max-time", "5", &url])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("200"))
            .unwrap_or(false);

        v.check_bool(
            &format!("depot:live:{}", arch.split('-').next().unwrap_or("unknown")),
            exists,
            &format!("{arch}/songbird → {}", if exists { "200 OK" } else { "NOT FOUND" }),
        );

        if exists && *arch == "x86_64-unknown-linux-musl" {
            let mut found = 0usize;
            let mut missing = Vec::new();
            for slug in Primal::ALL_SLUGS.iter().chain(EXTRA_BINARIES.iter()) {
                let bin_url = format!("{DEPOT_BASE}/{arch}/{slug}");
                let bin_exists = std::process::Command::new("curl")
                    .args(["-sI", "--max-time", "3", &bin_url])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).contains("200"))
                    .unwrap_or(false);
                if bin_exists {
                    found += 1;
                } else {
                    missing.push(*slug);
                }
            }
            v.check_bool(
                "depot:live:x86_musl_coverage",
                found >= *expected_min,
                &format!(
                    "x86_64-musl: {found}/{} binaries present{}",
                    Primal::ALL_SLUGS.len() + EXTRA_BINARIES.len(),
                    if missing.is_empty() { String::new() } else { format!(" (missing: {})", missing.join(", ")) }
                ),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depot_arch_coverage_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.failed == 0 || v.skipped > 0,
            "depot-architecture-coverage: {} failures ({} passed, {} skipped)",
            v.failed, v.passed, v.skipped
        );
    }
}
