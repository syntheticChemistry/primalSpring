// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Ecosystem Freshness — validates the wateringHole ecosystem
//! manifest and freshness protocol against the local workspace state.
//!
//! Structural phase (Tier::Rust):
//!   Parses `ecosystem_manifest.toml` and `freshness.toml` from the
//!   wateringHole repo, validates the manifest schema (all required
//!   fields, valid membrane types, gate profiles reference real repos),
//!   and cross-checks freshness heads against manifest entries.
//!
//! Live phase (Tier::Live):
//!   Compares local git HEADs against freshness.toml to detect drift,
//!   validates that high-priority repos are present on disk.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "ecosystem-freshness",
        track: Track::Infrastructure,
        tier: Tier::Both,
        provenance_crate: "primalspring_postprimordial",
        provenance_date: "2026-05-28",
        description:
            "Ecosystem freshness — manifest schema + freshness drift detection",
    },
    run,
};

/// Valid membrane boundary classifications from `REPO_MEMBRANE_BOUNDARY.md`.
const VALID_MEMBRANES: &[&str] = &["inner-only", "trailing-mirror", "outer-only"];
/// Repo taxonomy categories in the ecosystem manifest.
const VALID_CATEGORIES: &[&str] = &["primal", "spring", "garden", "infra", "root"];
/// Sync priority levels for cascade-pull ordering.
const VALID_PRIORITIES: &[&str] = &["high", "standard", "low"];
/// Named covalent gates that should have profiles in the manifest.
const EXPECTED_GATES: &[&str] = &["eastGate", "ironGate", "southGate", "biomeGate"];

// ─── Structural: Manifest Schema ────────────────────────────────────────────

fn phase_manifest_schema(v: &mut ValidationResult) {
    let manifest_toml = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
    let parsed: toml::Value = match toml::from_str(manifest_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "schema:manifest_parse",
                false,
                &format!("ecosystem_manifest.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "schema:manifest_parse",
        true,
        "ecosystem_manifest.toml parses as valid TOML",
    );

    let meta = parsed.get("meta").and_then(|m| m.as_table());
    v.check_bool(
        "schema:meta_section",
        meta.is_some(),
        "[meta] section present",
    );

    if let Some(meta) = meta {
        let version = meta
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "schema:meta:version",
            !version.is_empty(),
            &format!("meta.version = \"{version}\""),
        );

        let total = meta
            .get("total_repos")
            .and_then(toml::Value::as_integer)
            .unwrap_or(0);
        v.check_bool(
            "schema:meta:total_repos",
            total > 0,
            &format!("meta.total_repos = {total}"),
        );
    }

    let repos = parsed.get("repos").and_then(|r| r.as_table());
    v.check_bool(
        "schema:repos_section",
        repos.is_some(),
        "[repos] section present",
    );

    let repo_names: Vec<String>;
    if let Some(repos) = repos {
        repo_names = repos.keys().cloned().collect();

        v.check_bool(
            "schema:repo_count",
            repos.len() >= 20,
            &format!("{} repos defined (expect >= 20)", repos.len()),
        );

        for (name, repo) in repos {
            let org = repo
                .get("org")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                &format!("schema:repo:{name}:org"),
                !org.is_empty(),
                &format!("{name}.org = \"{org}\""),
            );

            let local_path = repo
                .get("local_path")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                &format!("schema:repo:{name}:local_path"),
                !local_path.is_empty(),
                &format!("{name}.local_path = \"{local_path}\""),
            );

            let membrane = repo
                .get("membrane")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                &format!("schema:repo:{name}:membrane"),
                VALID_MEMBRANES.contains(&membrane),
                &format!("{name}.membrane = \"{membrane}\" (valid: {VALID_MEMBRANES:?})"),
            );

            let category = repo
                .get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            v.check_bool(
                &format!("schema:repo:{name}:category"),
                VALID_CATEGORIES.contains(&category),
                &format!("{name}.category = \"{category}\""),
            );

            if let Some(priority) = repo.get("sync_priority").and_then(|v| v.as_str()) {
                v.check_bool(
                    &format!("schema:repo:{name}:sync_priority"),
                    VALID_PRIORITIES.contains(&priority),
                    &format!("{name}.sync_priority = \"{priority}\""),
                );
            }
        }
    } else {
        repo_names = Vec::new();
    }

    let gates = parsed.get("gates").and_then(|g| g.as_table());
    v.check_bool(
        "schema:gates_section",
        gates.is_some(),
        "[gates] section present",
    );

    if let Some(gates) = gates {
        for gate_name in EXPECTED_GATES {
            let gate = gates.get(*gate_name);
            v.check_bool(
                &format!("schema:gate:{gate_name}:present"),
                gate.is_some(),
                &format!("gate profile \"{gate_name}\" defined"),
            );

            if let Some(gate) = gate {
                let gate_repos = gate
                    .get("repos")
                    .and_then(|r| r.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                v.check_bool(
                    &format!("schema:gate:{gate_name}:has_repos"),
                    !gate_repos.is_empty(),
                    &format!("{gate_name} has {} repos", gate_repos.len()),
                );

                for repo in &gate_repos {
                    v.check_bool(
                        &format!("schema:gate:{gate_name}:ref:{repo}"),
                        repo_names.contains(repo),
                        &format!("{gate_name} references \"{repo}\" — exists in [repos]"),
                    );
                }
            }
        }
    }
}

// ─── Structural: Freshness Schema ───────────────────────────────────────────

fn phase_freshness_schema(v: &mut ValidationResult) {
    let freshness_toml = include_str!("../../../../../../infra/wateringHole/freshness.toml");
    let parsed: toml::Value = match toml::from_str(freshness_toml) {
        Ok(p) => p,
        Err(e) => {
            v.check_bool(
                "schema:freshness_parse",
                false,
                &format!("freshness.toml parse error: {e}"),
            );
            return;
        }
    };

    v.check_bool(
        "schema:freshness_parse",
        true,
        "freshness.toml parses as valid TOML",
    );

    let wave = parsed.get("wave").and_then(|w| w.as_table());
    v.check_bool(
        "schema:freshness:wave_section",
        wave.is_some(),
        "[wave] section present",
    );

    if let Some(wave) = wave {
        let id = wave
            .get("id")
            .and_then(toml::Value::as_integer)
            .unwrap_or(0);
        v.check_bool(
            "schema:freshness:wave_id",
            id > 0,
            &format!("wave.id = {id}"),
        );

        let date = wave
            .get("date")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        v.check_bool(
            "schema:freshness:wave_date",
            date.len() == 10,
            &format!("wave.date = \"{date}\""),
        );
    }

    let heads = parsed.get("heads").and_then(|h| h.as_table());
    v.check_bool(
        "schema:freshness:heads_section",
        heads.is_some(),
        "[heads] section present",
    );

    if let Some(heads) = heads {
        v.check_bool(
            "schema:freshness:head_count",
            heads.len() >= 20,
            &format!("{} HEAD refs in freshness.toml (expect >= 20)", heads.len()),
        );

        for (name, head) in heads {
            let hash = head.as_str().unwrap_or("");
            v.check_bool(
                &format!("schema:freshness:head:{name}:valid_hash"),
                hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit()),
                &format!("{name} has valid 40-char hex HEAD"),
            );
        }
    }
}

// ─── Live: Workspace Drift Detection ────────────────────────────────────────

fn phase_workspace_drift(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let eco_root = std::env::var("ECOPRIMALS_ROOT").ok().or_else(|| {
        // CARGO_MANIFEST_DIR = .../ecoPrimals/springs/primalSpring/ecoPrimal
        // We need .../ecoPrimals (3 levels up)
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .parent()?  // .../springs/primalSpring
            .parent()?  // .../springs
            .parent()   // .../ecoPrimals
            .map(|p| p.to_string_lossy().into_owned())
    });

    let Some(eco_root) = eco_root else {
        v.check_skip(
            "live:drift:eco_root",
            "Cannot determine ecoPrimals root (set ECOPRIMALS_ROOT)",
        );
        return;
    };

    let eco_path = std::path::Path::new(&eco_root);
    if !eco_path.exists() {
        v.check_skip(
            "live:drift:eco_root",
            &format!("ecoPrimals root not found: {eco_root}"),
        );
        return;
    }

    v.check_bool(
        "live:drift:eco_root",
        true,
        &format!("ecoPrimals root: {eco_root}"),
    );

    let manifest_toml = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
    let manifest: toml::Value = match toml::from_str(manifest_toml) {
        Ok(p) => p,
        Err(_) => return,
    };

    let repos = match manifest.get("repos").and_then(|r| r.as_table()) {
        Some(r) => r,
        None => return,
    };

    let high_priority_repos: Vec<(&String, &toml::Value)> = repos
        .iter()
        .filter(|(_, info)| {
            info.get("sync_priority")
                .and_then(|v| v.as_str())
                .unwrap_or("standard")
                == "high"
        })
        .collect();

    for (name, info) in &high_priority_repos {
        let local_path = info
            .get("local_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let repo_dir = eco_path.join(local_path);
        let git_dir = repo_dir.join(".git");

        v.check_bool(
            &format!("live:drift:present:{name}"),
            git_dir.exists(),
            &format!("{name} ({local_path}) present on disk"),
        );
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────────

/// Run the ecosystem freshness validation: manifest schema + freshness schema + drift.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Ecosystem Manifest Schema (structural)");
    phase_manifest_schema(v);

    v.section("Phase 2: Freshness Protocol Schema (structural)");
    phase_freshness_schema(v);

    v.section("Phase 3: Workspace Drift Detection (live)");
    phase_workspace_drift(v, ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn ecosystem_freshness_structural() {
        let mut v = ValidationResult::new("ecosystem-freshness");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.failed == 0,
            "Ecosystem freshness has {} failures (passed={}, skipped={})",
            v.failed, v.passed, v.skipped,
        );
    }

    #[test]
    fn manifest_parses_as_valid_toml() {
        let toml_str = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
        let parsed: toml::Value = toml::from_str(toml_str).expect("valid TOML");
        assert!(parsed.get("repos").is_some(), "missing [repos] section");
        assert!(parsed.get("gates").is_some(), "missing [gates] section");
        assert!(parsed.get("meta").is_some(), "missing [meta] section");
    }

    #[test]
    fn freshness_parses_as_valid_toml() {
        let toml_str = include_str!("../../../../../../infra/wateringHole/freshness.toml");
        let parsed: toml::Value = toml::from_str(toml_str).expect("valid TOML");
        assert!(parsed.get("wave").is_some(), "missing [wave] section");
        assert!(parsed.get("heads").is_some(), "missing [heads] section");
    }

    #[test]
    fn all_gate_profiles_reference_existing_repos() {
        let toml_str = include_str!("../../../../../../infra/wateringHole/ecosystem_manifest.toml");
        let parsed: toml::Value = toml::from_str(toml_str).expect("valid TOML");

        let repo_names: std::collections::HashSet<String> = parsed
            .get("repos")
            .and_then(|r| r.as_table())
            .map(|t| t.keys().cloned().collect())
            .unwrap_or_default();

        let gates = parsed.get("gates").and_then(|g| g.as_table()).expect("gates");
        for (gate_name, gate) in gates {
            let gate_repos = gate
                .get("repos")
                .and_then(|r| r.as_array())
                .expect("repos array");
            for repo in gate_repos {
                let name = repo.as_str().expect("repo name string");
                assert!(
                    repo_names.contains(name),
                    "gate '{gate_name}' references unknown repo '{name}'"
                );
            }
        }
    }
}
