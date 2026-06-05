// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: Deployment Pipeline — structural validation of the plasmidBin
//! build → harvest → compose → deploy → verify pipeline.
//!
//! Validates the ecosystem deployment pipeline by reading the plasmidBin
//! manifest and primalSpring configuration structurally. No live deployment
//! is needed — all checks are Tier::Rust.
//!
//! Six stages, each in its own submodule:
//!   1. Build      — manifest lists all 13 primals with valid metadata
//!   2. Harvest    — manifest declares checksum algorithm and binary matrix
//!   2.5 Provenance — composite fingerprint validation
//!   3. Compose    — atomic compositions are structurally consistent
//!   4. Deploy     — deploy graphs reference valid primals and fragments
//!   5. Verify     — niche compositions cover expected primal sets

mod stage_build;
mod stage_compose;
mod stage_deploy;
mod stage_harvest;
mod stage_provenance;
mod stage_verify;

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "deployment-pipeline",
        track: Track::Infrastructure,
        tier: Tier::Rust,
        provenance_crate: "primalspring_deployment_pipeline",
        provenance_date: "2026-05-14",
        description:
            "Deployment pipeline — build/harvest/compose/deploy/verify structural validation",
    },
    run,
};

const EXPECTED_DOMAIN_PRIMALS: &[&str] = &[
    "beardog",
    "songbird",
    "skunkbat",
    "toadstool",
    "barracuda",
    "coralreef",
    "nestgate",
    "rhizocrypt",
    "loamspine",
    "sweetgrass",
    "biomeos",
    "squirrel",
    "petaltongue",
];

const TOWER_PRIMALS: &[&str] = &["beardog", "songbird", "skunkbat"];
const NODE_ADDITIONS: &[&str] = &["toadstool", "barracuda", "coralreef"];
const NEST_ADDITIONS: &[&str] = &["nestgate", "rhizocrypt", "loamspine", "sweetgrass"];
const META_TIER: &[&str] = &["biomeos", "squirrel", "petaltongue"];

fn find_plasmidbin_file(filename: &str) -> Option<std::path::PathBuf> {
    let candidates = [
        std::env::current_dir()
            .ok()
            .map(|d| d.join("infra/plasmidBin").join(filename)),
        Some(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../infra/plasmidBin")
                .join(filename),
        ),
    ];

    for candidate in candidates.into_iter().flatten() {
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    let mut dir = std::env::current_dir().ok()?;
    for _ in 0..6 {
        let path = dir.join("infra/plasmidBin").join(filename);
        if path.is_file() {
            return Some(path);
        }
        if !dir.pop() {
            break;
        }
    }

    let xdg = std::env::var(crate::env_keys::ECOPRIMALS_PLASMID_BIN)
        .or_else(|_| {
            std::env::var(crate::env_keys::XDG_DATA_HOME)
                .or_else(|_| std::env::var(crate::env_keys::HOME).map(|h| format!("{h}/.local/share")))
                .map(|base| format!("{base}/ecoPrimals/plasmidBin"))
        })
        .ok()
        .map(|base| std::path::PathBuf::from(base).join(filename));

    xdg.filter(|p| p.is_file())
}

fn find_manifest() -> Option<toml::Value> {
    let candidates = [
        std::env::current_dir().ok().map(|d| d.join("infra/plasmidBin/manifest.toml")),
        Some(std::path::PathBuf::from(
            env!("CARGO_MANIFEST_DIR"),
        ).join("../../infra/plasmidBin/manifest.toml")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if candidate.is_file() {
            if let Ok(content) = std::fs::read_to_string(&candidate) {
                if let Ok(val) = toml::from_str(&content) {
                    return Some(val);
                }
            }
        }
    }

    let mut dir = std::env::current_dir().ok()?;
    for _ in 0..6 {
        let path = dir.join("infra/plasmidBin/manifest.toml");
        if path.is_file() {
            let content = std::fs::read_to_string(&path).ok()?;
            return toml::from_str(&content).ok();
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

/// Run the deployment pipeline validation across all stages.
pub fn run(v: &mut ValidationResult, _ctx: &mut CompositionContext) {
    let manifest = find_manifest();

    let Some(manifest) = manifest else {
        v.check_bool(
            "pipeline:manifest_found",
            false,
            "could not locate plasmidBin manifest.toml (tried infra/ and snapshot)",
        );
        return;
    };

    v.check_bool("pipeline:manifest_found", true, "plasmidBin manifest loaded");

    v.section("Stage 1: Build — primal metadata coverage");
    stage_build::stage_build(v, &manifest);

    v.section("Stage 2: Harvest — checksums and binary matrix");
    stage_harvest::stage_harvest(v, &manifest);

    v.section("Stage 2.5: Provenance — composite fingerprint validation");
    stage_provenance::stage_provenance(v);

    v.section("Stage 3: Compose — atomic model consistency");
    stage_compose::stage_compose(v, &manifest);

    v.section("Stage 4: Deploy — graph structure validation");
    stage_deploy::stage_deploy(v);

    v.section("Stage 5: Verify — niche composition coverage");
    stage_verify::stage_verify(v, &manifest);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn deployment_pipeline_structural() {
        let mut v = ValidationResult::new("deployment-pipeline");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
        assert!(
            v.failed == 0,
            "Deployment pipeline has {} failures (run with --nocapture to see summary)",
            v.failed,
        );
    }

    #[test]
    fn manifest_primals_cover_nucleus() {
        let manifest = find_manifest()
            .expect("must find manifest");

        let primals = manifest
            .get("primals")
            .and_then(|p| p.as_table())
            .expect("[primals] section");

        for expected in EXPECTED_DOMAIN_PRIMALS {
            assert!(
                primals.contains_key(*expected),
                "manifest missing primal: {expected}"
            );
        }
    }

    #[test]
    fn atomics_include_tower_base() {
        let manifest = find_manifest()
            .expect("must find manifest");

        let atomics = manifest
            .get("atomics")
            .and_then(|a| a.as_table())
            .expect("[atomics] section");

        for atomic_name in ["tower", "node", "nest", "nucleus"] {
            let declared: Vec<&str> = atomics
                .get(atomic_name)
                .and_then(|a| a.get("primals"))
                .and_then(|p| p.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            for tower_primal in TOWER_PRIMALS {
                assert!(
                    declared.contains(tower_primal),
                    "{atomic_name} atomic missing tower primal: {tower_primal}"
                );
            }
        }
    }
}
