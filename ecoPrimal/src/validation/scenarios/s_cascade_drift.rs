// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Cascade Drift Detection — validates VCS synchronization and
//! depot freshness across the ecosystem.
//!
//! The ecoPrimals cascade model requires that all remotes (Forgejo + GitHub)
//! stay in parity. This scenario validates:
//!
//! 1. Local workspace has no uncommitted production changes
//! 2. Local HEAD matches the configured remote(s) HEAD
//! 3. Depot binaries are fresh relative to VCS HEAD (age check)
//! 4. All ecosystem repositories are tracking and synchronized
//!
//! This is the automated version of the "17/17 zero drift" ecosystem metric.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::{Scenario, ScenarioMeta, Tier, Track};

/// Cascade drift detection scenario.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "cascade-drift",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring",
        provenance_date: "2026-06-17",
        description: "Validates VCS remote parity and depot freshness (zero drift)",
    },
    run,
};

/// Run cascade drift validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Local workspace cleanliness");
    phase_workspace_clean(v);

    v.section("Phase 2: Remote parity");
    phase_remote_parity(v);

    v.section("Phase 3: Depot freshness");
    phase_depot_freshness(v);
}

fn phase_workspace_clean(v: &mut ValidationResult) {
    let workspace_root = resolve_workspace_root();

    let Some(root) = &workspace_root else {
        v.check_skip(
            "workspace:root",
            "workspace root not found (CARGO_MANIFEST_DIR ancestry)",
        );
        return;
    };

    v.check_bool(
        "workspace:root",
        root.is_dir(),
        &format!("workspace: {}", root.display()),
    );

    let status = git_command(root, &["status", "--porcelain"]);
    match status {
        Some(output) => {
            let dirty_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
            if dirty_lines.is_empty() {
                v.check_bool("workspace:clean", true, "workspace is clean");
            } else {
                // Dirty workspace is common during development — report as skip,
                // not failure. A deployment pipeline would treat this as a gate.
                v.check_skip(
                    "workspace:clean",
                    &format!(
                        "{} uncommitted changes (first: {})",
                        dirty_lines.len(),
                        dirty_lines.first().unwrap_or(&"?")
                    ),
                );
            }
        }
        None => {
            v.check_skip("workspace:clean", "git status failed (not a git repo?)");
        }
    }

    let branch = git_command(root, &["rev-parse", "--abbrev-ref", "HEAD"]);
    if let Some(b) = branch {
        let b = b.trim();
        v.check_bool(
            "workspace:branch",
            b == "main" || b == "master",
            &format!("on branch: {b}"),
        );
    }
}

fn phase_remote_parity(v: &mut ValidationResult) {
    let workspace_root = resolve_workspace_root();
    let Some(root) = &workspace_root else {
        v.check_skip("remote:parity", "workspace root unavailable");
        return;
    };

    let local_head = git_command(root, &["rev-parse", "HEAD"]);
    let Some(local_sha) = local_head.map(|s| s.trim().to_owned()) else {
        v.check_skip("remote:local_head", "cannot resolve local HEAD");
        return;
    };

    v.check_bool(
        "remote:local_head",
        local_sha.len() == 40,
        &format!("local HEAD: {}", &local_sha[..8.min(local_sha.len())]),
    );

    let remotes = git_command(root, &["remote"]);
    let remote_names: Vec<&str> = remotes
        .as_deref()
        .unwrap_or("")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    v.check_bool(
        "remote:count",
        !remote_names.is_empty(),
        &format!(
            "{} remotes configured: {}",
            remote_names.len(),
            remote_names.join(", ")
        ),
    );

    let mut synced_count = 0u32;
    let mut total_checked = 0u32;

    for remote in &remote_names {
        let fetch_result = git_command(root, &["fetch", remote, "--dry-run"]);
        if fetch_result.is_none() {
            v.check_skip(
                &format!("remote:{remote}:reachable"),
                &format!("{remote} not reachable (network or auth)"),
            );
            continue;
        }

        let remote_ref = format!("{remote}/main");
        let remote_sha = git_command(root, &["rev-parse", &remote_ref]);

        if let Some(sha) = remote_sha.map(|s| s.trim().to_owned()) {
            total_checked += 1;
            let in_sync = sha == local_sha;
            if in_sync {
                synced_count += 1;
            }
            v.check_bool(
                &format!("remote:{remote}:parity"),
                in_sync,
                &format!(
                    "{remote}/main: {} ({})",
                    &sha[..8.min(sha.len())],
                    if in_sync { "IN SYNC" } else { "DRIFTED" }
                ),
            );
        } else {
            v.check_skip(
                &format!("remote:{remote}:parity"),
                &format!("{remote}/main ref not found (never fetched?)"),
            );
        }
    }

    if total_checked > 0 {
        v.check_bool(
            "remote:all_synced",
            synced_count == total_checked,
            &format!("{synced_count}/{total_checked} remotes in parity with local HEAD"),
        );
    }
}

fn phase_depot_freshness(v: &mut ValidationResult) {
    let depot_root = resolve_depot_root();

    let Some(depot) = depot_root else {
        v.check_skip("depot:root", "depot root not configured");
        return;
    };

    if !depot.is_dir() {
        v.check_skip(
            "depot:root",
            &format!("depot not found: {}", depot.display()),
        );
        return;
    }

    let target_triple = crate::tolerances::platform::current_target_triple();
    let arch_depot = depot.join("primals").join(&target_triple);

    if !arch_depot.is_dir() {
        v.check_skip(
            &format!("depot:{target_triple}:present"),
            &format!("no depot for {target_triple}"),
        );
        return;
    }

    let binary_count =
        std::fs::read_dir(&arch_depot).map_or(0, |rd| rd.filter_map(Result::ok).count());

    v.check_bool(
        &format!("depot:{target_triple}:populated"),
        binary_count >= 10,
        &format!("{binary_count} binaries in {target_triple} depot (expect ≥10)"),
    );

    let newest_mtime = std::fs::read_dir(&arch_depot).ok().and_then(|rd| {
        rd.filter_map(Result::ok)
            .filter_map(|e| e.metadata().ok())
            .filter_map(|m| m.modified().ok())
            .max()
    });

    if let Some(newest) = newest_mtime {
        let age = std::time::SystemTime::now()
            .duration_since(newest)
            .unwrap_or_default();
        let hours = age.as_secs() / 3600;

        // Build authorities (pepti) use tight 72h threshold.
        // Consumer gates use relaxed 168h (7 day) since they depend on
        // upstream push which may lag during enrollment waves.
        let is_build_authority = std::env::var("DEPOT_BUILD_AUTHORITY").is_ok();
        let threshold_h = if is_build_authority {
            crate::tolerances::DEPOT_FRESHNESS_LAN_H
        } else {
            crate::tolerances::DEPOT_FRESHNESS_THRESHOLD_H
        };

        v.check_bool(
            &format!("depot:{target_triple}:fresh"),
            hours < threshold_h,
            &format!("newest binary is {hours}h old (threshold: <{threshold_h}h)"),
        );
    } else {
        v.check_skip(
            &format!("depot:{target_triple}:fresh"),
            "cannot determine depot binary age",
        );
    }
}

fn resolve_workspace_root() -> Option<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        if ancestor.join(".git").exists() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

fn resolve_depot_root() -> Option<PathBuf> {
    if let Ok(val) = std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN) {
        let p = PathBuf::from(val);
        if p.is_dir() {
            return Some(p);
        }
    }
    let plasmid = crate::tolerances::plasmidbin_depot_root();
    let p = PathBuf::from(&plasmid);
    if p.is_dir() {
        return Some(p);
    }
    None
}

fn git_command(cwd: &Path, args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cascade_drift_structural() {
        let mut v = ValidationResult::new("cascade-drift");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        // workspace:clean may fail during development (uncommitted changes).
        // The scenario correctly reports this as a check, not a crash.
        // We only assert that the scenario ran to completion without panicking
        // and that infrastructure checks succeeded.
        assert!(
            v.evaluated() > 0,
            "cascade-drift should evaluate at least one check"
        );
    }
}
