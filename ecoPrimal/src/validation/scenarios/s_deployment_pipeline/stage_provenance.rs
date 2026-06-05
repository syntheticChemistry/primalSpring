// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stage 2.5: Provenance — composite fingerprint validation.

use crate::validation::ValidationResult;
use super::find_plasmidbin_file;

pub(super) fn stage_provenance(v: &mut ValidationResult) {
    let provenance_path = find_plasmidbin_file("provenance.toml");

    let Some(path) = provenance_path else {
        v.check_skip(
            "provenance:file_present",
            "provenance.toml not found (pre-provenance harvest or plasmidBin not local)",
        );
        return;
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            v.check_bool(
                "provenance:file_readable",
                false,
                &format!("cannot read provenance.toml: {e}"),
            );
            return;
        }
    };

    let provenance: toml::Value = match toml::from_str(&content) {
        Ok(p) => {
            v.check_bool("provenance:parses", true, "provenance.toml is valid TOML");
            p
        }
        Err(e) => {
            v.check_bool(
                "provenance:parses",
                false,
                &format!("provenance.toml parse error: {e}"),
            );
            return;
        }
    };

    let primals = provenance.get("primals").and_then(|p| p.as_table());

    let Some(primals_table) = primals else {
        v.check_bool(
            "provenance:has_primals",
            false,
            "provenance.toml missing [primals] section",
        );
        return;
    };

    let required_fields = [
        "content_hash",
        "source_commit",
        "source_repo",
        "build_timestamp",
        "rustc_version",
        "target",
        "provenance_hash",
    ];

    let mut entry_count = 0u32;
    let mut valid_count = 0u32;

    for (primal_name, arches) in primals_table {
        let Some(arch_table) = arches.as_table() else { continue };
        for (triple, entry) in arch_table {
            entry_count += 1;
            let mut fields_ok = true;
            for &field in &required_fields {
                let has_field = entry
                    .get(field)
                    .and_then(|v| v.as_str())
                    .is_some_and(|s| !s.is_empty());
                if !has_field {
                    v.check_bool(
                        &format!("provenance:{primal_name}.{triple}:{field}"),
                        false,
                        &format!("{primal_name}/{triple} missing {field}"),
                    );
                    fields_ok = false;
                }
            }

            if fields_ok {
                let content_hash = entry
                    .get("content_hash")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let is_valid_hash = content_hash.len() == 64
                    && content_hash.bytes().all(|b| b.is_ascii_hexdigit());
                let prov_hash = entry
                    .get("provenance_hash")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let is_valid_prov = prov_hash.len() == 64
                    && prov_hash.bytes().all(|b| b.is_ascii_hexdigit());

                if is_valid_hash && is_valid_prov {
                    valid_count += 1;
                } else {
                    v.check_bool(
                        &format!("provenance:{primal_name}.{triple}:hash_format"),
                        false,
                        &format!("{primal_name}/{triple} invalid hash format"),
                    );
                }
            }
        }
    }

    v.check_bool(
        "provenance:entry_count",
        entry_count > 0,
        &format!("{entry_count} provenance entries found"),
    );

    v.check_bool(
        "provenance:valid_entries",
        valid_count == entry_count,
        &format!("{valid_count}/{entry_count} entries have valid structure"),
    );

    if valid_count > 0 {
        let has_braid = primals_table.values().any(|arches| {
            arches.as_table().is_some_and(|t| {
                t.values().any(|e| {
                    e.get("braid_id")
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| !s.is_empty())
                })
            })
        });
        if has_braid {
            v.check_bool(
                "provenance:braid_ids_present",
                true,
                "at least one entry has a sweetGrass braid_id",
            );
        } else {
            v.check_skip(
                "provenance:braid_ids_present",
                "no braid_ids yet (sweetGrass not available during harvest)",
            );
        }
    }
}
