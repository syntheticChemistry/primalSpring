// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: protoKarya WAN Deploy — validates that live compositions on
//! `*.primals.eco` return HTTP 200, correct Content-Type, and security headers.
//!
//! Tests the existing live surfaces and the Caddy routing configuration.
//! No deployment blockers — validates what's already running.

use std::process::Command;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "protokarya-wan-deploy",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave140a_protokarya_tangibles",
        provenance_date: "2026-07-15",
        description:
            "protoKarya WAN deploy — live composition surfaces on *.primals.eco",
    },
    run,
};

const LIVE_SURFACES: &[(&str, &str, &str)] = &[
    ("sporePrint", "https://primals.eco", "text/html"),
    ("footPrint", "https://primals.eco/footprint/", "text/html"),
    ("petalTongue TOPO-VIS", "https://live.primals.eco", "text/html"),
];

const REQUIRED_HEADERS: &[&str] = &[
    "strict-transport-security",
    "x-content-type-options",
];

pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — Caddy routing for compositions");
    phase_caddy_structural(v);

    v.section("Phase 2: Live — HTTP 200 from each surface");
    phase_live_reachability(v);

    v.section("Phase 3: Live — Security headers on live surfaces");
    phase_security_headers(v);

    v.section("Phase 4: Live — Content-Type correctness");
    phase_content_type(v);

    v.section("Phase 5: Depot serving via WAN");
    phase_depot_wan(v);
}

fn phase_caddy_structural(v: &mut ValidationResult) {
    v.check_bool(
        "wan:surfaces_defined",
        LIVE_SURFACES.len() >= 3,
        &format!("{} live surfaces defined", LIVE_SURFACES.len()),
    );

    for (name, url, _) in LIVE_SURFACES {
        v.check_bool(
            &format!("wan:surface_https:{}", name.to_lowercase().replace(' ', "_")),
            url.starts_with("https://"),
            &format!("{name} uses HTTPS"),
        );
    }
}

fn phase_live_reachability(v: &mut ValidationResult) {
    for (name, url, _) in LIVE_SURFACES {
        let slug = name.to_lowercase().replace(' ', "_");
        let result = curl_status(url);
        match result {
            Ok(status) => {
                v.check_bool(
                    &format!("wan:live:{slug}"),
                    status == 200,
                    &format!("{name} at {url} → HTTP {status}"),
                );
            }
            Err(e) => {
                v.check_skip(
                    &format!("wan:live:{slug}"),
                    &format!("{name}: {e} (network unavailable or DNS not resolving)"),
                );
            }
        }
    }
}

fn phase_security_headers(v: &mut ValidationResult) {
    let test_url = "https://primals.eco";
    let headers = match curl_headers(test_url) {
        Ok(h) => h,
        Err(e) => {
            v.check_skip(
                "wan:headers",
                &format!("cannot fetch headers: {e}"),
            );
            return;
        }
    };

    let lower = headers.to_lowercase();

    for hdr in REQUIRED_HEADERS {
        v.check_bool(
            &format!("wan:header:{}", hdr.replace('-', "_")),
            lower.contains(hdr),
            &format!("Header '{hdr}' present on primals.eco"),
        );
    }

    let has_csp = lower.contains("content-security-policy");
    v.check_bool(
        "wan:header:csp",
        has_csp,
        "Content-Security-Policy header present",
    );
}

fn phase_content_type(v: &mut ValidationResult) {
    for (name, url, expected_ct) in LIVE_SURFACES {
        let slug = name.to_lowercase().replace(' ', "_");
        match curl_content_type(url) {
            Ok(ct) => {
                let matches = ct.contains(expected_ct);
                v.check_bool(
                    &format!("wan:ct:{slug}"),
                    matches,
                    &format!("{name}: Content-Type contains '{expected_ct}' (got '{ct}')"),
                );
            }
            Err(e) => {
                v.check_skip(
                    &format!("wan:ct:{slug}"),
                    &format!("{name}: {e}"),
                );
            }
        }
    }
}

fn phase_depot_wan(v: &mut ValidationResult) {
    let depot_url = "https://membrane.primals.eco/depot/checksums.toml";
    match curl_status(depot_url) {
        Ok(status) => {
            v.check_bool(
                "wan:depot:checksums",
                status == 200,
                &format!("Depot checksums.toml → HTTP {status}"),
            );
        }
        Err(e) => {
            v.check_skip(
                "wan:depot:checksums",
                &format!("depot unreachable: {e}"),
            );
        }
    }
}

fn curl_status(url: &str) -> Result<u16, String> {
    let output = Command::new("curl")
        .args(["-s", "-o", "/dev/null", "-w", "%{http_code}", "--max-time", "10", url])
        .output()
        .map_err(|e| format!("curl failed: {e}"))?;

    let code_str = String::from_utf8_lossy(&output.stdout);
    code_str
        .trim()
        .parse::<u16>()
        .map_err(|e| format!("parse error: {e} (raw: '{code_str}')"))
}

fn curl_headers(url: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-s", "-I", "--max-time", "10", url])
        .output()
        .map_err(|e| format!("curl failed: {e}"))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn curl_content_type(url: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-s", "-o", "/dev/null", "-w", "%{content_type}", "--max-time", "10", url])
        .output()
        .map_err(|e| format!("curl failed: {e}"))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn scenario_metadata_valid() {
        assert_eq!(SCENARIO.meta.id, "protokarya-wan-deploy");
        assert!(matches!(SCENARIO.meta.tier, Tier::Both));
    }

    #[test]
    fn live_surfaces_all_https() {
        for (name, url, _) in LIVE_SURFACES {
            assert!(url.starts_with("https://"), "{name} must use HTTPS: {url}");
        }
    }

    #[test]
    fn scenario_runs_gracefully() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        (SCENARIO.run)(&mut v, &mut ctx);
        if v.skipped > 0 {
            eprintln!(
                "protokarya-wan-deploy: {} passed, {} skipped (network-dependent)",
                v.passed, v.skipped
            );
        }
    }
}
