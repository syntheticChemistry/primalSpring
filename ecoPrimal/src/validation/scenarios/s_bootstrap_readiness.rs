// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Bootstrap Readiness — validates that deployment infrastructure
//! is correctly configured before attempting a NUCLEUS launch.
//!
//! Directly addresses the 8 hurdles discovered during fieldGate first-ant
//! deployment (Wave 114). This scenario runs purely structural checks: it
//! does not require live primals but verifies the prerequisites that must
//! be met before `nucleus_launcher start` can succeed.
//!
//! Phase 1: Binary discovery (all 13 primals resolvable via 4-tier search)
//! Phase 2: Runtime directory structure (biomeOS socket dir, PID dir)
//! Phase 3: Port registry sanity (no collisions in active configuration)
//! Phase 4: Environment configuration (required env vars, secret generation)

use crate::composition::CompositionContext;
use crate::tolerances;
use crate::validation::ValidationResult;

use super::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Bootstrap readiness scenario — pre-deployment infrastructure validation.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "bootstrap-readiness",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "nucleus_launcher_preflight",
        provenance_date: "2026-06-15",
        description: "Pre-deployment infrastructure validation (fieldGate first-ant lessons)",
    },
    run,
};

/// Run bootstrap readiness validation.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    v.section("Phase 1: Binary discovery (4-tier search)");
    phase_binary_discovery(v);

    v.section("Phase 2: Runtime directory structure");
    phase_runtime_dirs(v);

    v.section("Phase 3: Port registry sanity");
    phase_port_sanity(v);

    v.section("Phase 4: Environment configuration");
    phase_env_config(v);
}

fn phase_binary_discovery(v: &mut ValidationResult) {
    use crate::launcher::discover_binary;

    let has_depot = std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN).is_ok()
        || std::env::var(crate::env_keys::BIOMEOS_PLASMID_BIN_DIR).is_ok();
    let has_workspace_local = std::env::var(crate::env_keys::ECOPRIMALS_ROOT).is_ok();

    if has_depot {
        v.check_bool(
            "binary:depot_path_configured",
            true,
            "depot binary path set (post-primordial: fetched from VPS)",
        );
    } else if has_workspace_local {
        v.check_bool(
            "binary:depot_path_configured",
            false,
            "ECOPRIMALS_ROOT only (pre-primordial local workspace — use plasmidbin sync from VPS depot)",
        );
    } else {
        v.check_skip(
            "binary:depot_path_configured",
            "no depot path set (XDG fallback only — run plasmidbin sync)",
        );
        v.check_skip(
            "binary:discovery_complete",
            "skipped — no depot path configured",
        );
        return;
    }

    let primals = tolerances::all_primal_slugs();
    let mut found = 0u32;
    let mut missing: Vec<&str> = Vec::new();

    for slug in &primals {
        if discover_binary(slug).is_ok() {
            found += 1;
        } else {
            missing.push(slug);
        }
    }

    let total = u32::try_from(primals.len()).unwrap_or(u32::MAX);
    v.check_bool(
        "binary:discovery_complete",
        missing.is_empty(),
        &format!(
            "{found}/{total} primals discoverable{}",
            if missing.is_empty() {
                String::new()
            } else {
                format!(" — missing: {}", missing.join(", "))
            }
        ),
    );
}

fn phase_runtime_dirs(v: &mut ValidationResult) {
    let runtime_dir =
        std::path::PathBuf::from(tolerances::runtime_dir()).join(crate::env_keys::BIOMEOS_SUBDIR);

    let runtime_exists = runtime_dir.is_dir();
    if runtime_exists {
        v.check_bool(
            "runtime:biome_dir_exists",
            true,
            &format!("biomeOS runtime dir: {}", runtime_dir.display()),
        );

        let test_path = runtime_dir.join(".primalspring_write_test");
        let writable = std::fs::write(&test_path, b"test").is_ok();
        let _ = std::fs::remove_file(&test_path);
        v.check_bool(
            "runtime:writable",
            writable,
            &format!("runtime dir writable: {}", runtime_dir.display()),
        );
    } else {
        v.check_skip(
            "runtime:biome_dir_exists",
            &format!("biomeOS runtime dir absent: {}", runtime_dir.display()),
        );
        v.check_skip("runtime:writable", "runtime dir absent");
    }

    let pid_dir = std::path::PathBuf::from(tolerances::runtime_dir())
        .join(crate::env_keys::BIOMEOS_SUBDIR)
        .join("pids");

    let pid_exists = pid_dir.is_dir();
    if pid_exists {
        v.check_bool(
            "runtime:pid_dir_exists",
            true,
            &format!("PID directory: {}", pid_dir.display()),
        );
    } else {
        v.check_skip(
            "runtime:pid_dir_exists",
            &format!("PID directory absent: {}", pid_dir.display()),
        );
    }
}

fn phase_port_sanity(v: &mut ValidationResult) {
    let slugs = tolerances::ports::all_primal_slugs();
    let mut port_map: std::collections::HashMap<u16, Vec<&str>> = std::collections::HashMap::new();

    for slug in &slugs {
        let port = tolerances::ports::default_port_for(slug);
        port_map.entry(port).or_default().push(slug);
    }

    let collisions: Vec<_> = port_map
        .iter()
        .filter(|(_, slugs)| slugs.len() > 1)
        .map(|(port, slugs)| format!("{port}: {}", slugs.join(", ")))
        .collect();

    v.check_bool(
        "port:no_registry_collisions",
        collisions.is_empty(),
        &format!(
            "port registry: {} entries, {} collisions{}",
            slugs.len(),
            collisions.len(),
            if collisions.is_empty() {
                String::new()
            } else {
                format!(" — {}", collisions.join("; "))
            }
        ),
    );

    v.check_bool(
        "port:federation_defined",
        tolerances::FEDERATION_PORT > 0,
        &format!("federation port: {}", tolerances::FEDERATION_PORT),
    );
}

fn phase_env_config(v: &mut ValidationResult) {
    let advisory_vars: &[(&str, &str)] = &[
        (crate::env_keys::FAMILY_ID, "family/composition ID"),
        (crate::env_keys::ECOPRIMALS_PLASMID_BIN, "binary depot path"),
    ];

    for (var, purpose) in advisory_vars {
        let set = std::env::var(var).is_ok();
        if set {
            v.check_bool(
                &format!("env:{var}"),
                true,
                &format!("{var} set ({purpose})"),
            );
        } else {
            v.check_skip(
                &format!("env:{var}"),
                &format!("{var} not set — {purpose} (will use defaults)"),
            );
        }
    }

    let nestgate_secret_var = "NESTGATE_JWT_SECRET";
    let nestgate_secret_set = std::env::var(nestgate_secret_var).is_ok();
    if nestgate_secret_set {
        let val = std::env::var(nestgate_secret_var).unwrap_or_default();
        v.check_bool(
            "env:nestgate_jwt_secret_length",
            val.len() >= 32,
            &format!("NESTGATE_JWT_SECRET: {} chars (minimum 32)", val.len()),
        );
    } else {
        v.check_skip(
            "env:nestgate_jwt_secret",
            "NESTGATE_JWT_SECRET not set — nestgate will fail auth",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_runs_without_panic() {
        let mut v = ValidationResult::new("bootstrap-readiness");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.evaluated() > 0 || v.skipped > 0,
            "scenario must produce at least one check"
        );
    }
}
