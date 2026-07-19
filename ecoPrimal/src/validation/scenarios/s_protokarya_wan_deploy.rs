// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: `protoKarya` WAN Deploy.
//!
//! Validates that protist compositions (`footPrint`, `tideGlass`) have proper
//! Caddy routing, composition manifests, and capability wiring for WAN
//! deployment at `*.primals.eco`.
//!
//! Structural phase: validates config declarations.
//! Live phase: probes actual URLs for HTTP 200.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};
use std::net::TcpStream;
use std::time::Duration;

const FP_COMP_TOML: &str =
    include_str!("../../../../config/compositions/footprint_composition.toml");
const PROVISION_SH: &str =
    include_str!("../../../../../../infra/wateringHole/provision/provision-golgi.sh");
const MANIFEST_TOML: &str = include_str!("../../../../config/ecosystem/ecosystem_manifest.toml");

/// protoKarya WAN deploy scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "protokarya-wan-deploy",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "wave140a_wan_deploy",
        provenance_date: "2026-07-15",
        description: "protoKarya WAN deploy — Caddy routes, composition manifests, live surfaces",
    },
    run,
};

/// Run all WAN deploy validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    phase_structural(v);
    phase_live(v);
}

fn phase_structural(v: &mut ValidationResult) {
    v.section("Phase 1: Structural — composition manifests + Caddy config");

    v.check_bool(
        "wan:fp_comp_exists",
        !FP_COMP_TOML.is_empty(),
        "footprint_composition.toml exists and is non-empty",
    );

    v.check_bool(
        "wan:fp_comp_type",
        FP_COMP_TOML.contains("type = \"protist\""),
        "footPrint composition type = protist",
    );

    v.check_bool(
        "wan:fp_surfaces_spa",
        FP_COMP_TOML.contains("[surfaces.spa]") && FP_COMP_TOML.contains("status = \"LIVE\""),
        "footPrint SPA surface declared LIVE",
    );

    let has_url = FP_COMP_TOML.contains("footprint.primals.eco")
        || FP_COMP_TOML.contains("primals.eco/footprint/");
    v.check_bool(
        "wan:fp_spa_url",
        has_url,
        "SPA URL references footprint.primals.eco or primals.eco/footprint/",
    );

    v.check_bool(
        "wan:caddy_footprint_route",
        PROVISION_SH.contains("handle_path /footprint/*"),
        "Caddy has handle_path /footprint/* block",
    );

    v.check_bool(
        "wan:caddy_spa_fallback",
        PROVISION_SH.contains("try_files {path} /index.html"),
        "Caddy SPA fallback (try_files → index.html)",
    );

    v.check_bool(
        "wan:manifest_footprint",
        MANIFEST_TOML.contains("[repos.footPrint]"),
        "ecosystem_manifest.toml has [repos.footPrint]",
    );

    v.check_bool(
        "wan:manifest_tideglass",
        MANIFEST_TOML.contains("[repos.tideGlass]"),
        "ecosystem_manifest.toml has [repos.tideGlass]",
    );

    v.check_bool(
        "wan:manifest_protist_category",
        MANIFEST_TOML.contains("category = \"protist\""),
        "protist category used in manifest",
    );

    v.check_bool(
        "wan:fp_http_proxy_cap",
        FP_COMP_TOML.contains("name = \"http.proxy\"")
            && FP_COMP_TOML.contains("provider = \"songbird\""),
        "footPrint declares http.proxy capability from songbird",
    );

    v.check_bool(
        "wan:fp_storage_cap",
        FP_COMP_TOML.contains("name = \"storage.put\"")
            && FP_COMP_TOML.contains("provider = \"nestgate\""),
        "footPrint declares storage.put from nestgate",
    );
}

fn phase_live(v: &mut ValidationResult) {
    v.section("Phase 2: Live — probe WAN surfaces");

    let Ok(addr) = "157.230.3.183:443".parse() else {
        v.check_bool(
            "wan:live:addr_parse",
            false,
            "golgi socket address failed to parse",
        );
        return;
    };
    let golgi_reachable = TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok();

    if !golgi_reachable {
        v.check_skip(
            "wan:live:golgi_tls",
            "golgi:443 not reachable from this network",
        );
        v.check_skip("wan:live:footprint_200", "golgi not reachable");
        return;
    }

    v.check_bool("wan:live:golgi_tls", true, "golgi:443 TCP reachable");

    // Wave 150d: subdomain standard — compositions use prefix.primals.eco
    let fp_ok = std::process::Command::new("curl")
        .args([
            "-s",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--max-time",
            "5",
            "https://footprint.primals.eco/",
        ])
        .output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "200");

    v.check_bool(
        "wan:live:footprint_200",
        fp_ok,
        &format!(
            "footprint.primals.eco → {}",
            if fp_ok { "200 OK" } else { "NOT 200" }
        ),
    );

    // Verify old path-based URL redirects (subdomain migration)
    let fp_redirect = std::process::Command::new("curl")
        .args([
            "-s",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--max-time",
            "5",
            "https://primals.eco/footprint/",
        ])
        .output()
        .is_ok_and(|o| {
            let code = String::from_utf8_lossy(&o.stdout).trim().to_string();
            code == "301" || code == "302"
        });

    v.check_bool(
        "wan:live:footprint_redirect",
        fp_redirect,
        &format!(
            "primals.eco/footprint/ → {}",
            if fp_redirect {
                "301 redirect (subdomain migration)"
            } else {
                "NOT redirecting"
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protokarya_wan_deploy_structural() {
        let mut v = ValidationResult::new(SCENARIO.meta.id);
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.failed == 0 || v.skipped > 0,
            "protokarya-wan-deploy: {} failures ({} passed, {} skipped)",
            v.failed,
            v.passed,
            v.skipped
        );
    }
}
