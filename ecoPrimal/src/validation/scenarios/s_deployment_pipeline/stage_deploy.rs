// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stage 4: Deploy — deploy graph structure validation.

use crate::validation::ValidationResult;
use super::META_TIER;

pub(super) fn stage_deploy(v: &mut ValidationResult) {
    let bootstrap_toml = include_str!("../../../../../graphs/tower_atomic_bootstrap.toml");
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

    let meta_toml = include_str!("../../../../../graphs/fragments/meta_tier.toml");
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
