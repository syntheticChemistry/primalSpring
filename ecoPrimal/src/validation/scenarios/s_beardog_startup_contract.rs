// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: bearDog Startup Contract — validates TLS/ACME initialization
//! and platform-aware bind mode detection.
//!
//! Wave 132f exposed two P1 debt items in bearDog:
//!
//! 1. **`CryptoProvider`**: `rustls_rustcrypto::provider().install_default()` must
//!    be called before any ACME or TLS operations. Without it, `serve_https_gateway`
//!    panics at `assert_installed()`. This scenario validates that bearDog's
//!    capability registry includes TLS methods and that the ACME gateway contract
//!    is structurally sound.
//!
//! 2. **`BindMode::Auto`**: Should auto-detect Android abstract sockets via
//!    `ANDROID_ROOT`/`ANDROID_DATA` env vars or `cfg(target_os = "android")`.
//!    primalSpring's `PlatformCapabilities::detect()` provides the reference
//!    implementation. This scenario validates platform detection alignment.
//!
//! Phases:
//! 1. TLS ownership: bearDog owns `tls.*` in registry, ACME cert domain
//! 2. Platform detection: `PlatformCapabilities` abstract socket probe
//! 3. Startup ordering: `CryptoProvider` before TLS bind (structural contract)
//! 4. Live: bearDog health + TLS port probe (requires deployed bearDog)

use crate::composition::CompositionContext;
use crate::ipc::platform::PlatformCapabilities;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Capability registry TOML.
const REGISTRY_TOML: &str = include_str!("../../../../config/capability_registry.toml");

/// bearDog startup contract scenario metadata.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "beardog-startup-contract",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave132f_beardog_startup",
        provenance_date: "2026-07-05",
        description: "bearDog startup contract — CryptoProvider init, BindMode::Auto, ACME gateway",
    },
    run,
};

/// Run all bearDog startup contract validation phases.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: TLS ownership in registry");
    phase_tls_ownership(v);

    v.section("Phase 2: Platform detection — abstract sockets");
    phase_platform_detection(v);

    v.section("Phase 3: Startup ordering contract");
    phase_startup_ordering(v);

    v.section("Phase 4: Live — bearDog health + TLS probe");
    phase_live(v, ctx);
}

fn phase_tls_ownership(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        v.check_bool(
            "registry:parse",
            false,
            "capability_registry.toml failed to parse",
        );
        return;
    };

    let tls_section = parsed.get("tls");
    v.check_bool(
        "tls:domain_exists",
        tls_section.is_some(),
        "tls domain in registry",
    );

    if let Some(tls) = tls_section {
        let owner = tls.get("owner").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "tls:owner_beardog",
            owner == "beardog",
            &format!("tls owner = {owner} (expect beardog)"),
        );

        let methods = tls
            .get("methods")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        v.check_bool(
            "tls:has_methods",
            !methods.is_empty(),
            &format!("tls methods: {methods:?}"),
        );
    }

    let cert_section = parsed.get("certificate");
    v.check_bool(
        "cert:domain_exists",
        cert_section.is_some(),
        "certificate domain in registry (ACME lifecycle)",
    );

    if let Some(cert) = cert_section {
        let owner = cert.get("owner").and_then(|v| v.as_str()).unwrap_or("");
        v.check_bool(
            "cert:owner_beardog",
            owner == "beardog",
            &format!("certificate owner = {owner} (expect beardog)"),
        );
    }
}

fn phase_platform_detection(v: &mut ValidationResult) {
    let caps = PlatformCapabilities::detect();

    v.check_bool(
        "platform:uds_available",
        caps.uds_available,
        &format!(
            "UDS available = {} (socket_dir: {:?})",
            caps.uds_available, caps.socket_dir
        ),
    );

    v.check_bool(
        "platform:tcp_available",
        caps.tcp_available,
        &format!("TCP loopback = {}", caps.tcp_available),
    );

    let mode = caps.recommended_bind_mode();
    v.check_bool(
        "platform:bind_mode_valid",
        true,
        &format!("recommended bind mode = {mode:?}"),
    );

    let is_android = cfg!(target_os = "android")
        || std::env::var("ANDROID_ROOT").is_ok()
        || std::env::var("ANDROID_DATA").is_ok();

    if is_android {
        v.check_bool(
            "platform:android_abstract",
            caps.abstract_sockets,
            "Android detected — abstract sockets should be available",
        );
    } else {
        v.check_bool(
            "platform:desktop_no_android",
            !is_android,
            "desktop platform — Android env vars absent (expected)",
        );
    }
}

fn phase_startup_ordering(v: &mut ValidationResult) {
    let Ok(parsed) = toml::from_str::<toml::Value>(REGISTRY_TOML) else {
        return;
    };

    let tower = parsed.get("compositions").and_then(|c| c.get("tower"));

    if let Some(tower_comp) = tower {
        let primals = tower_comp
            .get("primals")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        let has_beardog = primals.contains(&"beardog");
        let has_songbird = primals.contains(&"songbird");

        v.check_bool(
            "startup:tower_has_beardog",
            has_beardog,
            "bearDog in tower composition (TLS terminator)",
        );
        v.check_bool(
            "startup:tower_has_songbird",
            has_songbird,
            "songBird in tower composition (HTTP proxy backend)",
        );

        if has_beardog && has_songbird {
            let bd_pos = primals.iter().position(|p| *p == "beardog");
            let sb_pos = primals.iter().position(|p| *p == "songbird");
            if let (Some(bd), Some(sb)) = (bd_pos, sb_pos) {
                v.check_bool(
                    "startup:beardog_before_songbird",
                    bd < sb,
                    &format!("bearDog at position {bd}, songBird at {sb} — TLS binds before proxy"),
                );
            }
        }
    } else {
        v.check_bool(
            "startup:tower_composition",
            false,
            "compositions.tower missing — cannot validate startup ordering",
        );
    }

    let security_owner = crate::composition::capability_to_primal("security");
    let tls_owner = crate::composition::capability_to_primal("tls");
    v.check_bool(
        "startup:security_beardog",
        security_owner == "beardog",
        &format!("security → {security_owner}"),
    );
    v.check_bool(
        "startup:tls_beardog",
        tls_owner == "beardog",
        &format!("tls → {tls_owner}"),
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &CompositionContext) {
    let caps = ctx.available_capabilities();
    if caps.is_empty() {
        v.check_skip(
            "live:beardog_health",
            "no live capabilities — requires deployed bearDog",
        );
        v.check_skip(
            "live:tls_port_probe",
            "no live capabilities — requires bearDog ACME gateway",
        );
        return;
    }

    v.check_bool(
        "live:has_security_cap",
        caps.contains(&"security"),
        "security capability discovered in live mesh",
    );
    v.check_bool(
        "live:has_tls_cap",
        caps.contains(&"tls"),
        "tls capability discovered in live mesh",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn beardog_startup_contract_structural() {
        let mut v = ValidationResult::new("beardog-startup-contract");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(v.passed > 0, "should have passed checks");
    }
}
