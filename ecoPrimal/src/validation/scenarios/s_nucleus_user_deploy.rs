// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: NUCLEUS User Deploy — validates the user-level systemd deployment
//! pattern that runs primals without root privileges.
//!
//! The user systemd pattern (`membrane-nucleus@.service` template) is the
//! standard for gate enrollment. It avoids sudo for NUCLEUS lifecycle while
//! keeping WireGuard and nftables as root-level infrastructure.
//!
//! This scenario validates:
//! 1. User systemd units are loaded and active
//! 2. Socket files exist in expected locations
//! 3. Live primals respond to health.liveness probes
//! 4. Songbird federation relay is running
//! 5. Known-degraded primals are correctly identified

use std::path::PathBuf;

use crate::composition::CompositionContext;
use crate::primal_names::{self, Primal};
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// NUCLEUS user deploy validation scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "nucleus-user-deploy",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-18",
        description: "User-level systemd NUCLEUS: unit state, sockets, liveness probes",
    },
    run: run_nucleus_user_deploy,
};

const KNOWN_MISSING: &[(&str, &str)] = &[(
    primal_names::SONGBIRD,
    "runs as songbird-federation.service, not nucleus template",
)];

fn run_nucleus_user_deploy(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    phase_systemd_units(v);
    phase_socket_presence(v);
    phase_liveness_probes(v, ctx);
    phase_degradation_tracking(v);
}

fn phase_systemd_units(v: &mut ValidationResult) {
    let output = std::process::Command::new("systemctl")
        .args([
            "--user",
            "list-units",
            "membrane-nucleus@*",
            "--no-pager",
            "--plain",
            "--no-legend",
        ])
        .output();

    let Ok(out) = output else {
        v.check_skip("systemd:available", "systemctl --user not available");
        return;
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let active_units: Vec<&str> = text.lines().filter(|l| l.contains("running")).collect();

    v.check_bool(
        "systemd:units_loaded",
        !active_units.is_empty(),
        &format!("{} membrane-nucleus@ units running", active_units.len()),
    );

    for primal in Primal::ALL_SLUGS {
        if *primal == primal_names::SONGBIRD {
            continue;
        }
        let unit_name = format!("membrane-nucleus@{primal}.service");
        let running = active_units.iter().any(|l| l.contains(&unit_name));
        v.check_bool(
            &format!("systemd:{primal}:running"),
            running,
            &format!(
                "{unit_name}: {}",
                if running { "active" } else { "NOT FOUND" }
            ),
        );
    }

    let songbird_out = std::process::Command::new("systemctl")
        .args(["--user", "is-active", "songbird-federation.service"])
        .output();

    let songbird_active = songbird_out
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default();

    v.check_bool(
        "systemd:songbird-federation:running",
        songbird_active == "active",
        &format!("songbird-federation: {songbird_active}"),
    );
}

fn phase_socket_presence(v: &mut ValidationResult) {
    let runtime_dir = socket_base_dir();

    let Some(base) = runtime_dir else {
        v.check_skip("sockets:base_dir", "biomeos socket directory not found");
        return;
    };

    v.check_bool(
        "sockets:base_dir",
        base.is_dir(),
        &format!("socket dir: {}", base.display()),
    );

    let socket_names = [
        "barracuda.sock",
        "beardog.sock",
        "coralreef.sock",
        "loamspine.sock",
        "petaltongue.sock",
        "rhizocrypt.sock",
        "skunkbat.sock",
        "songbird.sock",
        "squirrel.sock",
        "sweetgrass.sock",
        "toadstool.sock",
    ];

    let mut found = 0u32;
    for sock in &socket_names {
        let path = base.join(sock);
        if path.exists() {
            found += 1;
        }
    }

    v.check_bool(
        "sockets:count",
        found >= 8,
        &format!("{found}/{} expected sockets present", socket_names.len()),
    );

    let tmp_count: usize = socket_names
        .iter()
        .filter(|s| PathBuf::from("/tmp").join(s).exists())
        .count();

    if tmp_count > 0 {
        v.check_bool(
            "sockets:dual_path",
            true,
            &format!("{tmp_count} sockets also in /tmp (legacy dual-path)"),
        );
    }
}

fn phase_liveness_probes(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let probeable = ["security", "shader", "ai", "visualization", "ledger"];

    for cap in &probeable {
        let client = ctx.client_for(cap);
        match client {
            Some(c) => {
                let alive = c.call("health.liveness", serde_json::Value::Null).is_ok();
                v.check_bool(
                    &format!("liveness:{cap}"),
                    alive,
                    &format!("{cap}: {}", if alive { "ALIVE" } else { "NO RESPONSE" }),
                );
            }
            None => {
                v.check_skip(
                    &format!("liveness:{cap}"),
                    &format!("{cap} not discoverable via CompositionContext"),
                );
            }
        }
    }
}

fn phase_degradation_tracking(v: &mut ValidationResult) {
    for (primal, reason) in KNOWN_MISSING {
        let unit_name = format!("membrane-nucleus@{primal}.service");
        let output = std::process::Command::new("systemctl")
            .args(["--user", "is-active", &unit_name])
            .output();

        let status = match output {
            Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_owned(),
            Err(_) => "unknown".to_owned(),
        };

        if status == "active" {
            v.check_bool(
                &format!("degraded:{primal}:recovered"),
                true,
                &format!(
                    "{primal} was known-missing but is now ACTIVE (remove from KNOWN_MISSING)"
                ),
            );
        } else {
            v.check_skip(
                &format!("degraded:{primal}"),
                &format!("{primal}: {status} — {reason}"),
            );
        }
    }
}

fn socket_base_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let xdg_path = PathBuf::from(&xdg).join("biomeos");
        if xdg_path.is_dir() {
            return Some(xdg_path);
        }
    }
    let id_output = std::process::Command::new("id").arg("-u").output().ok()?;
    let uid = String::from_utf8_lossy(&id_output.stdout).trim().to_owned();
    let runtime = PathBuf::from(format!("/run/user/{uid}/biomeos"));
    if runtime.is_dir() {
        return Some(runtime);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nucleus_user_deploy_structural() {
        let mut v = ValidationResult::new("nucleus-user-deploy");
        let mut ctx = CompositionContext::discover();
        run_nucleus_user_deploy(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "nucleus-user-deploy should evaluate at least one check"
        );
    }
}
