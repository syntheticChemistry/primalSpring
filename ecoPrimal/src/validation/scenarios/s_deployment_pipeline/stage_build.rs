// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stage 1: Build — manifest primal coverage.

use crate::validation::ValidationResult;
use super::EXPECTED_DOMAIN_PRIMALS;

pub(super) fn stage_build(v: &mut ValidationResult, manifest: &toml::Value) {
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
