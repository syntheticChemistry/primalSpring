// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stage 2: Harvest — checksum algorithm and binary matrix validation.

use crate::validation::ValidationResult;

pub(super) fn stage_harvest(v: &mut ValidationResult, manifest: &toml::Value) {
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
