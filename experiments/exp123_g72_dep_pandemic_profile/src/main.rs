//! Exp123: G72 Dependency Pandemic — Ecosystem Profile
//!
//! Validates the dependency hygiene of the primalSpring workspace and
//! profiles the broader ecosystem's conformance with G72 Tier 1 targets.
//!
//! Checks:
//! 1. primalSpring workspace has zero tokio deps (sync-only lab)
//! 2. No `tokio ["full"]` in workspace
//! 3. No reqwest/ureq/hyper-client in workspace
//! 4. No env_logger in workspace
//! 5. All deps use workspace-level version management
//! 6. Ecosystem audit: scan sibling primals for Tier 1 violations

use primalspring::validation::ValidationResult;
use std::path::{Path, PathBuf};

fn ecoprimals_root() -> Option<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .find(|p| p.join("primals").is_dir() && p.join("springs").is_dir())
        .map(|p| p.to_path_buf())
}

fn workspace_cargo_tomls() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let mut tomls = Vec::new();
    collect_cargo_tomls(root, &mut tomls, 3);
    tomls
}

fn collect_cargo_tomls(dir: &Path, out: &mut Vec<PathBuf>, depth: u8) {
    if depth == 0 {
        return;
    }
    let cargo = dir.join("Cargo.toml");
    if cargo.exists() {
        out.push(cargo);
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir()
                && !path.ends_with("target")
                && !path.ends_with(".git")
                && !path.ends_with("node_modules")
            {
                collect_cargo_tomls(&path, out, depth - 1);
            }
        }
    }
}

fn check_workspace_hygiene(v: &mut ValidationResult) {
    let tomls = workspace_cargo_tomls();

    v.check_bool(
        "workspace_cargo_tomls_found",
        !tomls.is_empty(),
        &format!("found {} Cargo.toml files in workspace", tomls.len()),
    );

    let mut tokio_count = 0;
    let mut tokio_full_count = 0;
    let mut reqwest_count = 0;
    let mut ureq_count = 0;
    let mut env_logger_count = 0;

    for toml_path in &tomls {
        let content = std::fs::read_to_string(toml_path).unwrap_or_default();

        if content.contains("tokio") && !content.contains("[workspace") {
            tokio_count += 1;
        }
        if content.contains("\"full\"") && content.contains("tokio") {
            tokio_full_count += 1;
        }
        if content.contains("reqwest") {
            reqwest_count += 1;
        }
        if content.contains("ureq") {
            ureq_count += 1;
        }
        if content.contains("env_logger") {
            env_logger_count += 1;
        }
    }

    v.check_bool(
        "zero_tokio",
        tokio_count == 0,
        &format!("{tokio_count} crates reference tokio (target: 0 — sync lab)"),
    );
    v.check_bool(
        "zero_tokio_full",
        tokio_full_count == 0,
        &format!("{tokio_full_count} crates use tokio [\"full\"] (target: 0)"),
    );
    v.check_bool(
        "zero_reqwest",
        reqwest_count == 0,
        &format!("{reqwest_count} crates use reqwest (target: 0 — use capability.call)"),
    );
    v.check_bool(
        "zero_ureq",
        ureq_count == 0,
        &format!("{ureq_count} crates use ureq (target: 0 — use capability.call)"),
    );
    v.check_bool(
        "zero_env_logger",
        env_logger_count == 0,
        &format!("{env_logger_count} crates use env_logger (target: 0 — use tracing)"),
    );
}

fn profile_ecosystem(v: &mut ValidationResult) {
    let Some(root) = ecoprimals_root() else {
        v.check_bool(
            "ecosystem_root",
            false,
            "ecoPrimals root not found — cannot profile ecosystem",
        );
        return;
    };

    v.check_bool("ecosystem_root", true, &format!("ecoPrimals root: {}", root.display()));

    let primals_dir = root.join("primals");
    let mut total_tokio_full = 0u32;
    let mut total_reqwest = 0u32;
    let mut total_ureq = 0u32;
    let mut total_env_logger = 0u32;
    let mut primal_count = 0u32;

    if let Ok(entries) = std::fs::read_dir(&primals_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            primal_count += 1;

            let mut local_tomls = Vec::new();
            collect_cargo_tomls(&path, &mut local_tomls, 4);

            let mut has_tokio_full = false;
            let mut has_reqwest = false;
            let mut has_ureq = false;
            let mut has_env_logger = false;

            for toml_path in &local_tomls {
                let content = std::fs::read_to_string(toml_path).unwrap_or_default();
                if content.contains("\"full\"") && content.contains("tokio") {
                    has_tokio_full = true;
                }
                if content.contains("reqwest") {
                    has_reqwest = true;
                }
                if content.contains("ureq") {
                    has_ureq = true;
                }
                if content.contains("env_logger") {
                    has_env_logger = true;
                }
            }

            if has_tokio_full {
                total_tokio_full += 1;
            }
            if has_reqwest {
                total_reqwest += 1;
            }
            if has_ureq {
                total_ureq += 1;
            }
            if has_env_logger {
                total_env_logger += 1;
            }

            let violations: Vec<&str> = [
                if has_tokio_full { Some("tokio[full]") } else { None },
                if has_reqwest { Some("reqwest") } else { None },
                if has_ureq { Some("ureq") } else { None },
                if has_env_logger { Some("env_logger") } else { None },
            ]
            .into_iter()
            .flatten()
            .collect();

            let clean = violations.is_empty();
            let msg = if clean {
                format!("{name}: G72 Tier 1 CLEAN")
            } else {
                format!("{name}: violations — {}", violations.join(", "))
            };
            v.check_bool(
                &format!("primal_{name}_tier1"),
                clean,
                &msg,
            );
        }
    }

    v.check_bool(
        "ecosystem_primals_scanned",
        primal_count >= 14,
        &format!("{primal_count} primals scanned"),
    );

    // G72 Tier 1 COMPLETE (Wave 157i): 9/9 teams responded.
    // Post-pandemic baselines reflect Tier 1 excision.
    // Tier 2 targets (HTTP→songBird, axum→0.8) are next.
    v.check_bool(
        "tier1_tokio_full_target",
        total_tokio_full == 0,
        &format!(
            "tokio[full] primals: {total_tokio_full} (post-pandemic target: 0)"
        ),
    );
    v.check_bool(
        "tier1_http_client_target",
        total_reqwest + total_ureq <= 5,
        &format!(
            "HTTP client primals: {} (reqwest={total_reqwest}, ureq={total_ureq}, Tier 2 target: 0)",
            total_reqwest + total_ureq
        ),
    );
    v.check_bool(
        "tier1_env_logger_target",
        total_env_logger <= 2,
        &format!("env_logger primals: {total_env_logger} (post-pandemic target: 0)"),
    );
}

fn main() {
    let mut v = ValidationResult::new("g72-dep-pandemic-profile");

    v.section("Phase 1: primalSpring workspace hygiene");
    check_workspace_hygiene(&mut v);

    v.section("Phase 2: Ecosystem Tier 1 profile");
    profile_ecosystem(&mut v);

    v.summary();
}
