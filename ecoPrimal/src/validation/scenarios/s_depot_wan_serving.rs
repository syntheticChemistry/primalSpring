// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Depot WAN Serving — validates that the VPS depot endpoint is live
//! and correctly serving binaries, checksums, and signatures over HTTPS.
//!
//! The sporeGate build authority harvests binaries and pushes them to the VPS
//! depot. This scenario validates the WAN-facing endpoint that all gates fetch
//! from, ensuring the push pipeline is intact end-to-end.
//!
//! Phases:
//! 1. Endpoint reachability — HTTPS serving from membrane.primals.eco/depot/
//! 2. Checksums integrity — checksums.toml is parseable and has expected sections
//! 3. Signature presence — signatures.toml exists and contains Ed25519 attestation
//! 4. Architecture coverage — all expected target directories present
//! 5. Live probe — fetch a known binary HEAD request (skip if no network)

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario registration metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "depot-wan-serving",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave139c_depot_wan",
        provenance_date: "2026-07-15",
        description: "Depot WAN serving — validates VPS depot endpoint, checksums, signatures, architecture coverage",
    },
    run,
};

const DEPOT_BASE_URL: &str = "https://membrane.primals.eco/depot";

const EXPECTED_ARCHITECTURES: &[&str] =
    &["x86_64-unknown-linux-musl", "aarch64-unknown-linux-musl"];

const WINDOWS_ARCHITECTURE: &str = "x86_64-pc-windows-gnu";

/// Execute this scenario's validation phases.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Endpoint reachability");
    phase_reachability(v);

    v.section("Phase 2: Checksums integrity");
    phase_checksums(v);

    v.section("Phase 3: Signature presence");
    phase_signatures(v);

    v.section("Phase 4: Architecture coverage");
    phase_architecture(v);

    v.section("Phase 5: Live binary probe");
    phase_live_probe(v);
}

fn phase_reachability(v: &mut ValidationResult) {
    let checksums_url = format!("{DEPOT_BASE_URL}/checksums.toml");

    match std::process::Command::new("curl")
        .args(["-sI", "--max-time", "5", &checksums_url])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let has_200 = stdout.contains("200");
            v.check_bool(
                "wan:checksums_reachable",
                has_200,
                &format!("{checksums_url} returns HTTP 200"),
            );

            let has_hsts = stdout.contains("strict-transport-security");
            v.check_bool(
                "wan:hsts_header",
                has_hsts,
                "HSTS header present on depot endpoint",
            );

            let has_csp = stdout.contains("content-security-policy");
            v.check_bool(
                "wan:csp_header",
                has_csp,
                "CSP header present on depot endpoint",
            );
        }
        Err(_) => {
            v.check_skip(
                "wan:checksums_reachable",
                "curl not available or network unreachable",
            );
        }
    }
}

fn phase_checksums(v: &mut ValidationResult) {
    let checksums_url = format!("{DEPOT_BASE_URL}/checksums.toml");

    match std::process::Command::new("curl")
        .args(["-s", "--max-time", "10", &checksums_url])
        .output()
    {
        Ok(output) if output.status.success() => {
            let body = String::from_utf8_lossy(&output.stdout);

            let has_blake3_header = body.contains("BLAKE3");
            v.check_bool(
                "wan:checksums_blake3",
                has_blake3_header,
                "checksums.toml contains BLAKE3 header",
            );

            let has_generated = body.contains("Generated:");
            v.check_bool(
                "wan:checksums_timestamp",
                has_generated,
                "checksums.toml has generation timestamp",
            );

            for arch in EXPECTED_ARCHITECTURES {
                let section = format!("[{arch}]");
                let has_section = body.contains(&section);
                v.check_bool(
                    &format!("wan:checksums_arch_{}", arch.replace('-', "_")),
                    has_section,
                    &format!("checksums.toml has [{arch}] section"),
                );

                if has_section {
                    let section_start = body.find(&section).unwrap_or(0);
                    let section_text = &body[section_start..];
                    let primal_count = Primal::ALL_SLUGS
                        .iter()
                        .filter(|slug| {
                            section_text
                                .lines()
                                .take(20)
                                .any(|line| line.starts_with(*slug))
                        })
                        .count();

                    v.check_bool(
                        &format!(
                            "wan:checksums_{}_coverage",
                            arch.split('-').next().unwrap_or("unknown")
                        ),
                        primal_count >= 10,
                        &format!(
                            "{arch}: {primal_count}/{} primals in checksums",
                            Primal::ALL_SLUGS.len()
                        ),
                    );
                }
            }

            let has_windows = body.contains(&format!("[{WINDOWS_ARCHITECTURE}]"));
            v.check_bool(
                "wan:checksums_windows",
                has_windows,
                &format!("checksums.toml has [{WINDOWS_ARCHITECTURE}] section (northGate)"),
            );
        }
        _ => {
            v.check_skip(
                "wan:checksums_blake3",
                "could not fetch checksums.toml from WAN depot",
            );
        }
    }
}

fn phase_signatures(v: &mut ValidationResult) {
    let sig_url = format!("{DEPOT_BASE_URL}/signatures.toml");

    match std::process::Command::new("curl")
        .args(["-s", "--max-time", "5", &sig_url])
        .output()
    {
        Ok(output) if output.status.success() => {
            let body = String::from_utf8_lossy(&output.stdout);

            v.check_bool(
                "wan:sig_ed25519",
                body.contains("ed25519") || body.contains("Ed25519"),
                "signatures.toml contains Ed25519 algorithm",
            );

            v.check_bool(
                "wan:sig_public_key",
                body.contains("public_key"),
                "signatures.toml has public_key field",
            );

            v.check_bool(
                "wan:sig_signer_gate",
                body.contains("signer_gate"),
                "signatures.toml identifies signer gate",
            );

            v.check_bool(
                "wan:sig_sporegate",
                body.contains("sporeGate"),
                "signatures.toml signed by sporeGate",
            );

            v.check_bool(
                "wan:sig_timestamp",
                body.contains("signed_at"),
                "signatures.toml has signed_at timestamp",
            );
        }
        _ => {
            v.check_skip(
                "wan:sig_ed25519",
                "could not fetch signatures.toml from WAN depot",
            );
        }
    }
}

fn phase_architecture(v: &mut ValidationResult) {
    for arch in EXPECTED_ARCHITECTURES {
        let probe_url = format!("{DEPOT_BASE_URL}/primals/{arch}/songbird");

        match std::process::Command::new("curl")
            .args(["-sI", "--max-time", "5", &probe_url])
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                v.check_bool(
                    &format!(
                        "wan:arch_{}_present",
                        arch.split('-').next().unwrap_or("unknown")
                    ),
                    stdout.contains("200"),
                    &format!("{arch}/songbird exists in WAN depot"),
                );
            }
            Err(_) => {
                v.check_skip(
                    &format!(
                        "wan:arch_{}_present",
                        arch.split('-').next().unwrap_or("unknown")
                    ),
                    "network unreachable",
                );
            }
        }
    }

    let win_url = format!("{DEPOT_BASE_URL}/primals/{WINDOWS_ARCHITECTURE}/songbird.exe");
    match std::process::Command::new("curl")
        .args(["-sI", "--max-time", "5", &win_url])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            v.check_bool(
                "wan:arch_windows_present",
                stdout.contains("200"),
                "songbird.exe exists in WAN depot (northGate)",
            );
        }
        Err(_) => {
            v.check_skip("wan:arch_windows_present", "network unreachable");
        }
    }
}

fn phase_live_probe(v: &mut ValidationResult) {
    let probe_url = format!("{DEPOT_BASE_URL}/primals/x86_64-unknown-linux-musl/membrane");

    match std::process::Command::new("curl")
        .args(["-sI", "--max-time", "5", &probe_url])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            v.check_bool(
                "wan:membrane_binary",
                stdout.contains("200"),
                "membrane binary accessible in WAN depot",
            );

            if let Some(line) = stdout
                .lines()
                .find(|l| l.to_lowercase().starts_with("content-type"))
            {
                let is_binary = line.contains("octet-stream")
                    || line.contains("x-executable")
                    || line.contains("x-msdos-program")
                    || line.contains("application/");
                v.check_bool(
                    "wan:membrane_content_type",
                    is_binary,
                    &format!("membrane binary content-type is binary-like: {line}"),
                );
            }
        }
        Err(_) => {
            v.check_skip(
                "wan:membrane_binary",
                "network unreachable — skipping live binary probe",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_metadata_valid() {
        assert_eq!(SCENARIO.meta.id, "depot-wan-serving");
        assert!(matches!(SCENARIO.meta.track, Track::Infrastructure));
        assert!(!DEPOT_BASE_URL.is_empty());
    }

    #[test]
    fn expected_architectures_valid() {
        assert!(EXPECTED_ARCHITECTURES.len() >= 2);
        for arch in EXPECTED_ARCHITECTURES {
            assert!(arch.contains("musl"), "expected musl target: {arch}");
        }
    }

    #[test]
    fn scenario_runs_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        // Network-dependent — may skip all checks if offline.
        // No assertion on pass/fail, just verifying it doesn't panic.
    }
}
