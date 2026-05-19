// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scenario: Deployment Pipeline — structural validation of the plasmidBin
//! build → harvest → compose → deploy → verify pipeline.
//!
//! Validates the ecosystem deployment pipeline by reading the plasmidBin
//! manifest and primalSpring configuration structurally. No live deployment
//! is needed — all checks are Tier::Rust.
//!
//! Five stages:
//!   1. Build   — manifest lists all 13 primals with valid metadata
//!   2. Harvest — manifest declares checksum algorithm and binary matrix
//!   3. Compose — atomic compositions are structurally consistent
//!   4. Deploy  — deploy graphs reference valid primals and fragments
//!   5. Verify  — niche compositions cover expected primal sets

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

// ─── Stage 1: Build — manifest primal coverage ──────────────────────────────

fn stage_build(v: &mut ValidationResult, manifest: &toml::Value) {
    let primals_table = manifest.get("primals").and_then(|p| p.as_table());

    match primals_table {
        Some(table) => {
            v.check_bool(
                "build:primals_section_exists",
                true,
                &format!("{} primals declared in manifest", table.len()),
            );

            for expected in EXPECTED_DOMAIN_PRIMALS {
                let present = table.contains_key(*expected);
                v.check_bool(
                    &format!("build:primal:{expected}"),
                    present,
                    &format!("{expected} in manifest.primals"),
                );

                if let Some(entry) = table.get(*expected) {
                    let has_name = entry.get("name").and_then(|v| v.as_str()).is_some();
                    let has_desc = entry.get("description").and_then(|v| v.as_str()).is_some();
                    let has_version = entry.get("latest").and_then(|v| v.as_str()).is_some();
                    v.check_bool(
                        &format!("build:primal:{expected}:metadata"),
                        has_name && has_desc && has_version,
                        &format!("{expected} has name+description+latest"),
                    );
                }
            }
        }
        None => {
            v.check_bool(
                "build:primals_section_exists",
                false,
                "manifest missing [primals] section",
            );
        }
    }
}

// ─── Stage 2: Harvest — checksum and binary matrix ──────────────────────────

fn stage_harvest(v: &mut ValidationResult, manifest: &toml::Value) {
    let algo = manifest
        .get("manifest")
        .and_then(|m| m.get("checksum_algorithm"))
        .and_then(|a| a.as_str())
        .unwrap_or("");
    v.check_bool(
        "harvest:checksum_algorithm",
        algo == "blake3",
        &format!("checksum_algorithm = {algo} (expect blake3)"),
    );

    let format = manifest
        .get("manifest")
        .and_then(|m| m.get("format"))
        .and_then(|f| f.as_str())
        .unwrap_or("");
    v.check_bool(
        "harvest:format_genomeBin",
        format == "genomeBin",
        &format!("format = {format} (expect genomeBin)"),
    );

    let binaries = manifest.get("binaries").and_then(|b| b.as_table());
    match binaries {
        Some(table) => {
            v.check_bool(
                "harvest:binaries_section_exists",
                true,
                &format!("{} binaries in matrix", table.len()),
            );

            let musl_target = "x86_64-unknown-linux-musl";
            let mut musl_stripped_count = 0u32;
            let mut musl_static_count = 0u32;

            for (name, arches) in table {
                if let Some(arch_table) = arches.as_table() {
                    if let Some(musl) = arch_table.get(musl_target) {
                        let stripped = musl
                            .get("stripped")
                            .and_then(toml::Value::as_bool)
                            .unwrap_or(false);
                        let is_static = musl
                            .get("static")
                            .and_then(toml::Value::as_bool)
                            .unwrap_or(false);

                        if stripped {
                            musl_stripped_count += 1;
                        } else {
                            v.check_bool(
                                &format!("harvest:binary:{name}:stripped"),
                                false,
                                &format!("{name} x86_64-musl not stripped"),
                            );
                        }
                        if is_static {
                            musl_static_count += 1;
                        }
                    }
                }
            }

            v.check_bool(
                "harvest:all_musl_stripped",
                musl_stripped_count >= 13,
                &format!("{musl_stripped_count} binaries stripped for x86_64-musl (expect ≥13)"),
            );
            v.check_bool(
                "harvest:all_musl_static",
                musl_static_count >= 13,
                &format!("{musl_static_count} binaries static for x86_64-musl (expect ≥13)"),
            );
        }
        None => {
            v.check_bool(
                "harvest:binaries_section_exists",
                false,
                "manifest missing [binaries] section",
            );
        }
    }

    let primals_table = manifest.get("primals").and_then(|p| p.as_table());
    if let Some(table) = primals_table {
        let mut seed_count = 0u32;
        let skip_seed = ["sourdough", "skunkbat"];
        for (name, entry) in table {
            if entry
                .get("seed_fingerprint")
                .and_then(|v| v.as_str())
                .is_some()
            {
                seed_count += 1;
            } else if !skip_seed.contains(&name.as_str()) {
                let is_dev = entry
                    .get("latest")
                    .and_then(|v| v.as_str())
                    .is_some_and(|v| v.contains("dev"));
                if is_dev {
                    v.check_skip(
                        &format!("harvest:primal:{name}:seed_fingerprint"),
                        &format!("{name} is pre-release (dev), seed_fingerprint deferred"),
                    );
                } else {
                    v.check_bool(
                        &format!("harvest:primal:{name}:seed_fingerprint"),
                        false,
                        &format!("{name} missing seed_fingerprint (BLAKE3)"),
                    );
                }
            }
        }
        v.check_bool(
            "harvest:seed_fingerprint_coverage",
            seed_count >= 12,
            &format!("{seed_count} primals have seed_fingerprint (expect ≥12)"),
        );
    }
}

// ─── Stage 3: Compose — atomic model consistency ────────────────────────────

fn stage_compose(v: &mut ValidationResult, manifest: &toml::Value) {
    let atomics = manifest.get("atomics").and_then(|a| a.as_table());
    match atomics {
        Some(table) => {
            v.check_bool(
                "compose:atomics_section_exists",
                true,
                &format!("{} atomics defined", table.len()),
            );

            check_atomic_primals(v, table, "tower", TOWER_PRIMALS);

            let node_expected: Vec<&str> = TOWER_PRIMALS
                .iter()
                .chain(NODE_ADDITIONS.iter())
                .copied()
                .collect();
            check_atomic_primals(v, table, "node", &node_expected);

            let nest_expected: Vec<&str> = TOWER_PRIMALS
                .iter()
                .chain(NEST_ADDITIONS.iter())
                .copied()
                .collect();
            check_atomic_primals(v, table, "nest", &nest_expected);

            let nucleus_expected: Vec<&str> = TOWER_PRIMALS
                .iter()
                .chain(NODE_ADDITIONS.iter())
                .chain(NEST_ADDITIONS.iter())
                .copied()
                .collect();
            check_atomic_primals(v, table, "nucleus", &nucleus_expected);
        }
        None => {
            v.check_bool(
                "compose:atomics_section_exists",
                false,
                "manifest missing [atomics] section",
            );
        }
    }

    let tower_toml = include_str!("../../../../graphs/fragments/tower_atomic.toml");
    let node_toml = include_str!("../../../../graphs/fragments/node_atomic.toml");
    let nest_toml = include_str!("../../../../graphs/fragments/nest_atomic.toml");
    let nucleus_toml = include_str!("../../../../graphs/fragments/nucleus.toml");

    for (name, toml_str) in [
        ("tower_atomic", tower_toml),
        ("node_atomic", node_toml),
        ("nest_atomic", nest_toml),
        ("nucleus", nucleus_toml),
    ] {
        let parsed: Result<toml::Value, _> = toml::from_str(toml_str);
        v.check_bool(
            &format!("compose:fragment:{name}:parses"),
            parsed.is_ok(),
            &format!("{name} fragment is valid TOML"),
        );

        if let Ok(val) = parsed {
            let node_count = val
                .get("fragment")
                .and_then(|f| f.get("nodes"))
                .and_then(|n| n.as_array())
                .map_or(0, std::vec::Vec::len);
            v.check_bool(
                &format!("compose:fragment:{name}:has_nodes"),
                node_count > 0,
                &format!("{name} has {node_count} nodes"),
            );
        }
    }
}

fn check_atomic_primals(
    v: &mut ValidationResult,
    atomics: &toml::map::Map<String, toml::Value>,
    atomic_name: &str,
    expected: &[&str],
) {
    if let Some(atomic) = atomics.get(atomic_name) {
        let declared: Vec<&str> = atomic
            .get("primals")
            .and_then(|p| p.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        for &expected_primal in expected {
            v.check_bool(
                &format!("compose:atomic:{atomic_name}:{expected_primal}"),
                declared.contains(&expected_primal),
                &format!("{expected_primal} in {atomic_name} atomic"),
            );
        }
    } else {
        v.check_bool(
            &format!("compose:atomic:{atomic_name}:exists"),
            false,
            &format!("{atomic_name} atomic not in manifest"),
        );
    }
}

// ─── Stage 4: Deploy — deploy graph structure ───────────────────────────────

fn stage_deploy(v: &mut ValidationResult) {
    let bootstrap_toml = include_str!("../../../../graphs/tower_atomic_bootstrap.toml");
    let parsed: Result<toml::Value, _> = toml::from_str(bootstrap_toml);

    match parsed {
        Ok(val) => {
            v.check_bool(
                "deploy:bootstrap_graph_parses",
                true,
                "tower_atomic_bootstrap.toml is valid TOML",
            );

            let secure = val
                .get("graph")
                .and_then(|g| g.get("metadata"))
                .and_then(|m| m.get("secure_by_default"))
                .and_then(toml::Value::as_bool)
                .unwrap_or(false);
            v.check_bool(
                "deploy:bootstrap_secure_by_default",
                secure,
                "bootstrap graph declares secure_by_default = true",
            );

            let has_fragments = val
                .get("graph")
                .and_then(|g| g.get("metadata"))
                .and_then(|m| m.get("fragments"))
                .and_then(|f| f.as_array())
                .is_some_and(|a| !a.is_empty());
            v.check_bool(
                "deploy:bootstrap_has_fragments",
                has_fragments,
                "bootstrap graph declares fragment references",
            );
        }
        Err(e) => {
            v.check_bool(
                "deploy:bootstrap_graph_parses",
                false,
                &format!("bootstrap graph parse error: {e}"),
            );
        }
    }

    let meta_toml = include_str!("../../../../graphs/fragments/meta_tier.toml");
    let meta_parsed: Result<toml::Value, _> = toml::from_str(meta_toml);
    v.check_bool(
        "deploy:meta_tier_fragment_parses",
        meta_parsed.is_ok(),
        "meta_tier fragment is valid TOML",
    );

    if let Ok(val) = meta_parsed {
        let meta_binaries: Vec<&str> = val
            .get("fragment")
            .and_then(|f| f.get("nodes"))
            .and_then(|n| n.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|n| n.get("binary").and_then(|v| v.as_str()))
                    .collect()
            })
            .unwrap_or_default();

        for &expected in META_TIER {
            v.check_bool(
                &format!("deploy:meta_tier:{expected}"),
                meta_binaries.contains(&expected),
                &format!("{expected} binary in meta_tier fragment"),
            );
        }
    }
}

// ─── Stage 5: Verify — niche composition coverage ───────────────────────────

fn stage_verify(v: &mut ValidationResult, manifest: &toml::Value) {
    let niches = manifest.get("niches").and_then(|n| n.as_table());
    match niches {
        Some(table) => {
            v.check_bool(
                "verify:niches_section_exists",
                true,
                &format!("{} niches defined", table.len()),
            );

            for (niche_name, niche) in table {
                let primals: Vec<&str> = niche
                    .get("primals")
                    .and_then(|p| p.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();

                let has_tower = TOWER_PRIMALS.iter().all(|t| primals.contains(t));
                v.check_bool(
                    &format!("verify:niche:{niche_name}:tower_base"),
                    has_tower,
                    &format!(
                        "{niche_name} includes full Phase 32 tower (BearDog+Songbird+skunkBat)"
                    ),
                );

                let has_desc = niche
                    .get("description")
                    .and_then(|d| d.as_str())
                    .is_some_and(|d| !d.is_empty());
                v.check_bool(
                    &format!("verify:niche:{niche_name}:description"),
                    has_desc,
                    &format!("{niche_name} has a description"),
                );
            }
        }
        None => {
            v.check_bool(
                "verify:niches_section_exists",
                false,
                "manifest missing [niches] section",
            );
        }
    }
}

// ─── Entry point ─────────────────────────────────────────────────────────────

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

/// Run the deployment pipeline validation across all five stages.
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
    stage_build(v, &manifest);

    v.section("Stage 2: Harvest — checksums and binary matrix");
    stage_harvest(v, &manifest);

    v.section("Stage 3: Compose — atomic model consistency");
    stage_compose(v, &manifest);

    v.section("Stage 4: Deploy — graph structure validation");
    stage_deploy(v);

    v.section("Stage 5: Verify — niche composition coverage");
    stage_verify(v, &manifest);
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
