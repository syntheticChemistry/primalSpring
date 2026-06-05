// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stage 3: Compose — atomic model consistency.

use crate::validation::ValidationResult;
use super::{TOWER_PRIMALS, NODE_ADDITIONS, NEST_ADDITIONS};

pub(super) fn stage_compose(v: &mut ValidationResult, manifest: &toml::Value) {
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

    let tower_toml = include_str!("../../../../../graphs/fragments/tower_atomic.toml");
    let node_toml = include_str!("../../../../../graphs/fragments/node_atomic.toml");
    let nest_toml = include_str!("../../../../../graphs/fragments/nest_atomic.toml");
    let nucleus_toml = include_str!("../../../../../graphs/fragments/nucleus.toml");

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
