// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Multi-Gate NUCLEUS — validates NUCLEUS deployment parity across gates.
//!
//! Phase 1 (Structural/Rust tier):
//! - Standard NUCLEUS composition is 13 primals (canonical roster order)
//! - Systemd unit template naming follows `membrane-nucleus@{primal}.service`
//! - Expected binary names in depot match the 13 primals
//!
//! Phase 2 (Live tier):
//! - Count local systemd units matching `membrane-nucleus@*` that are active
//! - Verify count >= 13 (full NUCLEUS)
//! - Each primal has a corresponding socket in `/run/user/{uid}/biomeos/`
//! - Socket liveness probe: connect + `health.liveness` JSON-RPC + verify `"result"`

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Multi-gate NUCLEUS deployment parity validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "multi-gate-nucleus",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-20",
        description: "NUCLEUS deployment parity across gates — 13/13 primals, units, sockets, liveness",
    },
    run: run_multi_gate_nucleus,
};

const NUCLEUS_UNIT_TEMPLATE: &str = "membrane-nucleus@.service";
const FULL_NUCLEUS_COUNT: usize = 13;

fn run_multi_gate_nucleus(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural/Rust tier");
    phase_structural(v);

    v.section("Phase 2: Live tier");
    phase_live(v);
}

fn phase_structural(v: &mut ValidationResult) {
    phase_nucleus_composition(v);
    phase_systemd_unit_naming(v);
    phase_depot_binaries(v);
}

fn phase_nucleus_composition(v: &mut ValidationResult) {
    let slugs: Vec<&str> = Primal::ALL.iter().map(|p| p.slug()).collect();

    v.check_count(
        "struct:nucleus_primal_count",
        slugs.len(),
        FULL_NUCLEUS_COUNT,
    );

    v.check_bool(
        "struct:nucleus_roster_complete",
        slugs.len() == FULL_NUCLEUS_COUNT,
        &format!(
            "NUCLEUS roster ({} primals): {}",
            slugs.len(),
            slugs.join(", ")
        ),
    );
}

fn phase_systemd_unit_naming(v: &mut ValidationResult) {
    for primal in Primal::ALL {
        let slug = primal.slug();
        let unit_name = format!("membrane-nucleus@{slug}.service");
        v.check_bool(
            &format!("struct:unit_name:{slug}"),
            unit_name.starts_with("membrane-nucleus@") && unit_name.ends_with(".service"),
            &format!("systemd unit template instance: {unit_name}"),
        );
    }

    let home = std::env::var("HOME").unwrap_or_default();
    let template_path = PathBuf::from(&home)
        .join(".config/systemd/user")
        .join(NUCLEUS_UNIT_TEMPLATE);

    if !template_path.exists() {
        v.check_skip(
            "struct:unit_template_exists",
            &format!("{NUCLEUS_UNIT_TEMPLATE} not at {}", template_path.display()),
        );
        return;
    }

    let content = std::fs::read_to_string(&template_path).unwrap_or_default();
    v.check_bool(
        "struct:unit_template_exists",
        true,
        &format!("{NUCLEUS_UNIT_TEMPLATE} present"),
    );
    v.check_bool(
        "struct:unit_template_specifier",
        content.contains("%i"),
        "template uses %i specifier for primal name",
    );
}

fn phase_depot_binaries(v: &mut ValidationResult) {
    let Some(depot_path) = resolve_depot_path() else {
        v.check_skip(
            "struct:depot_path",
            "plasmidBin depot not locatable (ECOPRIMALS_ROOT / ECOPRIMALS_PLASMID_BIN)",
        );
        for primal in Primal::ALL {
            v.check_skip(
                &format!("struct:depot_binary:{}", primal.slug()),
                "depot not locatable",
            );
        }
        return;
    };

    v.check_bool(
        "struct:depot_exists",
        depot_path.is_dir(),
        &format!("depot at {}", depot_path.display()),
    );

    if !depot_path.is_dir() {
        return;
    }

    let mut present = 0usize;
    let mut missing = Vec::new();

    for primal in Primal::ALL {
        let slug = primal.slug();
        let binary_path = depot_path.join(slug);
        if binary_path.is_file() {
            present += 1;
            v.check_bool(
                &format!("struct:depot_binary:{slug}"),
                true,
                &format!("{slug} binary in depot"),
            );
        } else {
            missing.push(slug);
            v.check_bool(
                &format!("struct:depot_binary:{slug}"),
                false,
                &format!("{slug} binary missing from depot"),
            );
        }
    }

    v.check_bool(
        "struct:depot_all_13",
        present == FULL_NUCLEUS_COUNT,
        &format!(
            "{present}/{FULL_NUCLEUS_COUNT} NUCLEUS binaries in depot{}",
            if missing.is_empty() {
                String::new()
            } else {
                format!(" (missing: {})", missing.join(", "))
            }
        ),
    );
}

fn phase_live(v: &mut ValidationResult) {
    phase_systemd_active_count(v);
    phase_socket_presence(v);
    phase_socket_liveness(v);
}

fn phase_systemd_active_count(v: &mut ValidationResult) {
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
        v.check_skip("live:systemd_available", "systemctl --user not available");
        return;
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let active_count = text.lines().filter(|l| l.contains("running")).count();

    v.check_bool(
        "live:nucleus_unit_count",
        active_count >= FULL_NUCLEUS_COUNT,
        &format!("{active_count} active membrane-nucleus@* units (need >= {FULL_NUCLEUS_COUNT})"),
    );
}

fn phase_socket_presence(v: &mut ValidationResult) {
    let Some(base) = biomeos_socket_dir() else {
        v.check_skip("live:socket_dir", "biomeOS socket directory not found");
        for primal in Primal::ALL {
            v.check_skip(
                &format!("live:socket:{}", primal.slug()),
                "socket dir not found",
            );
        }
        return;
    };

    v.check_bool(
        "live:socket_dir",
        base.is_dir(),
        &format!("socket dir: {}", base.display()),
    );

    if !base.is_dir() {
        return;
    }

    let mut found = 0usize;
    for primal in Primal::ALL {
        let slug = primal.slug();
        let path = resolve_primal_socket(&base, slug);
        let exists = path.exists();
        if exists {
            found += 1;
        }
        v.check_bool(
            &format!("live:socket:{slug}"),
            exists,
            &format!(
                "{slug}: {}",
                if exists {
                    path.display().to_string()
                } else {
                    format!("no socket at {}", path.display())
                }
            ),
        );
    }

    v.check_bool(
        "live:socket_count",
        found >= FULL_NUCLEUS_COUNT,
        &format!(
            "{found}/{FULL_NUCLEUS_COUNT} primal sockets in {}",
            base.display()
        ),
    );
}

fn phase_socket_liveness(v: &mut ValidationResult) {
    let Some(base) = biomeos_socket_dir() else {
        v.check_skip("live:liveness", "biomeOS socket directory not found");
        return;
    };

    for primal in Primal::ALL {
        let slug = primal.slug();
        let path = resolve_primal_socket(&base, slug);
        if !path.exists() {
            v.check_skip(
                &format!("live:liveness:{slug}"),
                &format!("socket not present: {}", path.display()),
            );
            continue;
        }

        let alive = probe_socket_liveness(&path);
        v.check_bool(
            &format!("live:liveness:{slug}"),
            alive,
            &format!(
                "{slug} health.liveness via {}: {}",
                path.display(),
                if alive { "ALIVE" } else { "NO RESPONSE" }
            ),
        );
    }
}

fn resolve_primal_socket(base: &Path, primal: &str) -> PathBuf {
    let plain = base.join(format!("{primal}.sock"));
    if plain.exists() {
        return plain;
    }

    let family = crate::env_keys::resolve_family_id();
    let family_sock = base.join(format!("{primal}-{family}.sock"));
    if family_sock.exists() {
        return family_sock;
    }

    plain
}

fn probe_socket_liveness(socket: &Path) -> bool {
    let timeout = Duration::from_secs(tolerances::IPC_SOCKET_TIMEOUT_SECS);
    let Ok(mut stream) = UnixStream::connect(socket) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));

    if stream
        .write_all(&tolerances::RIBOCIPHER_CLEAR_SIGNAL)
        .is_err()
    {
        return false;
    }

    let payload = r#"{"jsonrpc":"2.0","id":1,"method":"health.liveness","params":{}}"#;
    if stream.write_all(payload.as_bytes()).is_err() {
        return false;
    }
    if stream.write_all(b"\n").is_err() {
        return false;
    }

    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            let resp = String::from_utf8_lossy(&buf[..n]);
            resp.contains("\"result\"")
        }
        _ => false,
    }
}

fn biomeos_socket_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(xdg).join("biomeos");
        if path.is_dir() {
            return Some(path);
        }
    }

    let id_output = std::process::Command::new("id").arg("-u").output().ok()?;
    let uid = String::from_utf8_lossy(&id_output.stdout).trim().to_owned();
    let runtime = PathBuf::from(format!("/run/user/{uid}/biomeos"));
    if runtime.is_dir() {
        Some(runtime)
    } else {
        None
    }
}

fn resolve_depot_path() -> Option<PathBuf> {
    let triple = tolerances::current_target_triple();
    let depot_root = PathBuf::from(tolerances::plasmidbin_depot_root());
    let arch_depot = depot_root.join("primals").join(&triple);
    if arch_depot.is_dir() {
        return Some(arch_depot);
    }

    let flat = depot_root.join("primals");
    if flat.is_dir() {
        return Some(flat);
    }

    ecoprimals_root().map(|r| r.join("infra/plasmidBin/primals").join(&triple))
}

fn ecoprimals_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("ECOPRIMALS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest_dir.join("../../..");
    if candidate.join("infra").is_dir() {
        Some(candidate.canonicalize().unwrap_or(candidate))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_gate_nucleus_structural() {
        let mut v = ValidationResult::new("multi-gate-nucleus");
        phase_structural(&mut v);
        assert_eq!(
            v.failed, 0,
            "multi-gate-nucleus structural phase had {} failures",
            v.failed
        );
    }

    #[test]
    fn multi_gate_nucleus_runs() {
        let mut v = ValidationResult::new("multi-gate-nucleus");
        let mut ctx = CompositionContext::discover();
        run_multi_gate_nucleus(&mut v, &mut ctx);
        assert!(
            v.passed + v.failed + v.skipped > 0,
            "multi-gate-nucleus should evaluate at least one check"
        );
    }
}
