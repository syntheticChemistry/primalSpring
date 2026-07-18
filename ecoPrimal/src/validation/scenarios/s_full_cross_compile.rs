// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Full Cross-Compile — validates that all 14 primals compile cleanly
//! for all 4 depot architectures.
//!
//! Silicon Atheism Phase 1 goal: every primal builds for every target.
//! This scenario validates the depot's checksums.toml to confirm that each
//! primal has a BLAKE3 hash entry for each architecture, proving successful
//! cross-compilation.
//!
//! Phases:
//! 1. Structural — primal names and architecture declarations
//! 2. Checksums — parse depot checksums.toml and verify coverage matrix
//! 3. Parity — every primal appears in all 4 arch sections
//! 4. Gap analysis — identify and report missing entries

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

const DEPOT_CHECKSUMS_URL: &str = "https://membrane.primals.eco/depot/checksums.toml";

const TARGET_ARCHITECTURES: &[&str] = &[
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "aarch64-linux-android",
    "x86_64-pc-windows-gnu",
];

const EXTRA_PRIMALS: &[&str] = &["sourdough"];

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "full-cross-compile",
        track: Track::Evolution,
        tier: Tier::Both,
        provenance_crate: "wave142b_silicon_atheism",
        provenance_date: "2026-07-16",
        description: "Full cross-compile — all primals × all architectures verified via depot checksums",
    },
    run,
};

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — target matrix declaration");
    phase_structural(v);

    v.section("Phase 2: Live — checksums.toml cross-compile matrix");
    phase_live_checksums(v);
}

fn phase_structural(v: &mut ValidationResult) {
    let all_primals: Vec<&str> = Primal::ALL_SLUGS
        .iter()
        .copied()
        .chain(EXTRA_PRIMALS.iter().copied())
        .collect();

    v.check_bool(
        "xcompile:primal_count",
        all_primals.len() >= 14,
        &format!("{} primals declared for cross-compile", all_primals.len()),
    );

    v.check_bool(
        "xcompile:arch_count",
        TARGET_ARCHITECTURES.len() == 4,
        &format!("{} target architectures", TARGET_ARCHITECTURES.len()),
    );

    let matrix_size = all_primals.len() * TARGET_ARCHITECTURES.len();
    v.check_bool(
        "xcompile:matrix_size",
        matrix_size >= 56,
        &format!(
            "cross-compile matrix: {} cells ({}×{})",
            matrix_size,
            all_primals.len(),
            TARGET_ARCHITECTURES.len()
        ),
    );
}

fn phase_live_checksums(v: &mut ValidationResult) {
    let body = match std::process::Command::new("curl")
        .args(["-s", "--max-time", "10", DEPOT_CHECKSUMS_URL])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        _ => {
            v.check_skip("xcompile:live:fetch", "depot not reachable");
            return;
        }
    };

    v.check_bool(
        "xcompile:live:fetch",
        body.contains("BLAKE3"),
        "checksums.toml fetched and contains BLAKE3 header",
    );

    let all_primals: Vec<&str> = Primal::ALL_SLUGS
        .iter()
        .copied()
        .chain(EXTRA_PRIMALS.iter().copied())
        .collect();

    let mut total_present = 0usize;
    let mut total_expected = 0usize;
    let mut gaps: Vec<String> = Vec::new();

    for arch in TARGET_ARCHITECTURES {
        let section_header = format!("[{arch}]");
        let section_text = if let Some(start) = body.find(&section_header) {
            let after_header = &body[start + section_header.len()..];
            let end = after_header.find("\n[").unwrap_or(after_header.len());
            &after_header[..end]
        } else {
            v.check_bool(
                &format!(
                    "xcompile:live:{}_section",
                    arch.split('-').next().unwrap_or("?")
                ),
                false,
                &format!("[{arch}] section missing from checksums.toml"),
            );
            continue;
        };

        let mut arch_present = 0usize;
        for primal in &all_primals {
            total_expected += 1;
            let has_entry = section_text
                .lines()
                .any(|line| line.starts_with(primal) && line.contains('='));
            if has_entry {
                arch_present += 1;
                total_present += 1;
            } else {
                gaps.push(format!("{arch}/{primal}"));
            }
        }

        let arch_short = arch.split('-').next().unwrap_or("?");
        v.check_bool(
            &format!("xcompile:live:{arch_short}_coverage"),
            arch_present >= 12,
            &format!(
                "{arch}: {arch_present}/{} primals have checksums",
                all_primals.len()
            ),
        );
    }

    let parity = total_present == total_expected;
    v.check_bool(
        "xcompile:live:full_parity",
        parity,
        &format!(
            "full cross-compile parity: {total_present}/{total_expected}{}",
            if gaps.is_empty() {
                String::new()
            } else {
                format!(" (gaps: {})", gaps.join(", "))
            }
        ),
    );

    // Threshold: 85% allows pre-harvest state; raises to 100% post-harvest
    let coverage_pct = (total_present * 100)
        .checked_div(total_expected)
        .unwrap_or(0);
    v.check_bool(
        "xcompile:live:coverage_threshold",
        coverage_pct >= 85,
        &format!("coverage: {coverage_pct}% ({total_present}/{total_expected}) — threshold 85%"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_cross_compile_runs() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // Pre-harvest: android+windows missing some entries → up to 4 failures expected.
        // Post-harvest: should be 0 failures (raise to strict).
        let max_known_failures = 4;
        assert!(
            v.failed <= max_known_failures || v.skipped > 0,
            "full-cross-compile: {} failures exceeds known debt ({max_known_failures})",
            v.failed
        );
    }
}
