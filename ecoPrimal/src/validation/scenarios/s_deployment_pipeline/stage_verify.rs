// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stage 5: Verify — niche composition coverage.

use crate::validation::ValidationResult;
use super::TOWER_PRIMALS;

pub(super) fn stage_verify(v: &mut ValidationResult, manifest: &toml::Value) {
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
