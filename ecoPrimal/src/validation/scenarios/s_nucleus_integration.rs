// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: NUCLEUS Integration — validates the Spring→NUCLEUS integration
//! patterns that make the user-level systemd deployment work.
//!
//! The integration contract between primalSpring and NUCLEUS consists of:
//!
//! 1. **Template pattern**: `membrane-nucleus@.service` — parameterized by primal
//!    name via systemd %i specifier. Binary path, socket path, env vars.
//! 2. **Songbird relay**: `songbird-federation.service` — mesh relay that other
//!    primals depend on (After= relationship).
//! 3. **Socket alignment**: user-level at `$XDG_RUNTIME_DIR/biomeos/<primal>.sock`
//!    (resolves to `/run/user/<uid>/biomeos/`).
//! 4. **Graceful degradation**: when primals fail (biomeos, nestgate), the
//!    composition continues with reduced capabilities.
//!
//! This scenario validates the integration contract is met on the running gate.

use std::path::PathBuf;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// NUCLEUS integration pattern validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "nucleus-integration",
        track: Track::Lifecycle,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "Spring→NUCLEUS: template pattern, socket paths, songbird relay, degradation",
    },
    run: run_nucleus_integration,
};

fn run_nucleus_integration(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_template_pattern(v);
    phase_songbird_relay(v);
    phase_socket_alignment(v);
    phase_graceful_degradation(v, ctx);
}

fn phase_template_pattern(v: &mut ValidationResult) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/eastgate".to_owned());
    let template_path = PathBuf::from(&home).join(".config/systemd/user/membrane-nucleus@.service");

    if !template_path.exists() {
        v.check_skip(
            "template:file_exists",
            &format!("template not at {}", template_path.display()),
        );
        return;
    }

    v.check_bool(
        "template:file_exists",
        true,
        "membrane-nucleus@.service present",
    );

    let content = std::fs::read_to_string(&template_path).unwrap_or_default();

    v.check_bool(
        "template:specifier_i",
        content.contains("%i"),
        "template uses %i specifier for primal name",
    );

    v.check_bool(
        "template:socket_path",
        content.contains("%t/biomeos/%i.sock"),
        "socket path uses %t/biomeos/%i.sock (XDG runtime)",
    );

    v.check_bool(
        "template:after_songbird",
        content.contains("After=songbird-federation.service"),
        "After= dependency on songbird-federation",
    );

    v.check_bool(
        "template:restart_policy",
        content.contains("Restart=always"),
        "restart policy: always",
    );

    let binary_path_correct =
        content.contains("plasmidBin/primals/") && content.contains("%i server");
    v.check_bool(
        "template:binary_path",
        binary_path_correct,
        "ExecStart references plasmidBin depot + %i server",
    );

    v.check_bool(
        "template:env_ecoprimals_root",
        content.contains("ECOPRIMALS_ROOT="),
        "ECOPRIMALS_ROOT env set",
    );
}

fn phase_songbird_relay(v: &mut ValidationResult) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/eastgate".to_owned());
    let songbird_path =
        PathBuf::from(&home).join(".config/systemd/user/songbird-federation.service");

    if !songbird_path.exists() {
        v.check_skip(
            "songbird:file_exists",
            "songbird-federation.service not found",
        );
        return;
    }

    v.check_bool(
        "songbird:file_exists",
        true,
        "songbird-federation.service present",
    );

    let content = std::fs::read_to_string(&songbird_path).unwrap_or_default();

    v.check_bool(
        "songbird:federation_port",
        content.contains("--federation-port"),
        "songbird configured with --federation-port",
    );

    v.check_bool(
        "songbird:socket_path",
        content.contains("%t/biomeos/songbird.sock"),
        "songbird socket at %t/biomeos/songbird.sock",
    );

    let is_active = std::process::Command::new("systemctl")
        .args(["--user", "is-active", "songbird-federation.service"])
        .output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "active");

    v.check_bool(
        "songbird:running",
        is_active,
        &format!(
            "songbird-federation: {}",
            if is_active { "active" } else { "inactive" }
        ),
    );
}

fn phase_socket_alignment(v: &mut ValidationResult) {
    let socket_dir = crate::tolerances::platform::biomeos_socket_dir();

    v.check_bool(
        "sockets:dir_exists",
        socket_dir.is_dir(),
        &format!("biomeos socket dir: {}", socket_dir.display()),
    );

    if !socket_dir.is_dir() {
        return;
    }

    let entries: Vec<String> = std::fs::read_dir(&socket_dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sock"))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    v.check_bool(
        "sockets:count",
        entries.len() >= 8,
        &format!("{} .sock files in biomeos dir", entries.len()),
    );

    let expected_primals = [
        "barracuda",
        "skunkbat",
        "squirrel",
        "sweetgrass",
        "loamspine",
        "rhizocrypt",
        "petaltongue",
        "toadstool",
        "coralreef",
        "beardog",
        "songbird",
    ];

    let mut matched = 0usize;
    for primal in &expected_primals {
        let sock_name = format!("{primal}.sock");
        if entries.iter().any(|e| e == &sock_name) {
            matched += 1;
        }
    }

    v.check_bool(
        "sockets:primal_alignment",
        matched >= 8,
        &format!(
            "{matched}/{} expected primal sockets present",
            expected_primals.len()
        ),
    );

    let template_socket_pattern = format!("{}/", socket_dir.display());
    let systemd_output = std::process::Command::new("systemctl")
        .args([
            "--user",
            "show",
            "membrane-nucleus@barracuda.service",
            "--property=ExecStart",
        ])
        .output();

    if let Ok(out) = systemd_output {
        let text = String::from_utf8_lossy(&out.stdout);
        let uses_runtime = text.contains(&template_socket_pattern) || text.contains("/biomeos/");
        v.check_bool(
            "sockets:systemd_runtime_match",
            uses_runtime,
            "systemd ExecStart socket path aligns with runtime dir",
        );
    }
}

fn phase_graceful_degradation(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let degraded_primals = [
        ("biomeos", "needs different CLI entrypoint"),
        ("nestgate", "needs NESTGATE_JWT_SECRET"),
    ];

    for (primal, reason) in &degraded_primals {
        let unit = format!("membrane-nucleus@{primal}.service");
        let is_active = std::process::Command::new("systemctl")
            .args(["--user", "is-active", &unit])
            .output()
            .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "active");

        if is_active {
            v.check_bool(
                &format!("degradation:{primal}:recovered"),
                true,
                &format!("{primal} now active (was known-degraded: {reason})"),
            );
        } else {
            v.check_skip(
                &format!("degradation:{primal}"),
                &format!("{primal}: expected-degraded — {reason}"),
            );
        }
    }

    let core_caps = ["security", "ledger", "ai", "visualization", "shader"];
    let mut core_alive = 0u32;
    for cap in &core_caps {
        if ctx.has_capability(cap)
            && ctx
                .call(cap, "health.liveness", serde_json::json!({}))
                .is_ok()
        {
            core_alive += 1;
        }
    }

    v.check_bool(
        "degradation:core_unaffected",
        core_alive >= 4,
        &format!(
            "{core_alive}/{} core capabilities alive despite degraded primals",
            core_caps.len()
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nucleus_integration_structural() {
        let mut v = ValidationResult::new("nucleus-integration");
        let mut ctx = CompositionContext::discover();
        run_nucleus_integration(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "nucleus-integration should evaluate at least one check"
        );
    }
}
