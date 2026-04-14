// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tier 3 — Genetic Tags (open participation channels).
//!
//! Tags are the evolution of the deprecated plaintext `FAMILY_SEED`. They
//! represent open participation channels within a mito-beacon family:
//! hashtags, chat rooms, subreddits — public or semi-public group
//! identifiers for broadcast participation.
//!
//! Tags are **freely copyable** (contrast with nuclear genetics which are
//! never copied). A large mito-beacon family can have open subgroups
//! within it via tags.
//!
//! # Migration
//!
//! The legacy `FAMILY_SEED` environment variable maps to a `GeneticTag`
//! via [`GeneticTag::from_legacy_family_seed`]. This provides backward
//! compatibility while the ecosystem transitions to the full three-tier
//! genetics model.

use serde::{Deserialize, Serialize};

/// A genetic tag: open participation channel within a mito-beacon family.
///
/// Freely `Clone` + `Copy`-able — tags are public/semi-public group
/// identifiers, not secrets. They allow broadcast participation without
/// granting permissions (those require nuclear genetics).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GeneticTag {
    /// The tag identifier (e.g. `"#build-lab"`, `"primalspring-dev"`, legacy family seed).
    pub tag: String,
    /// Optional parent mito-beacon this tag belongs to.
    pub beacon_id: Option<String>,
}

impl GeneticTag {
    /// Create a new tag.
    #[must_use]
    pub const fn new(tag: String) -> Self {
        Self {
            tag,
            beacon_id: None,
        }
    }

    /// Create a tag scoped to a specific mito-beacon group.
    #[must_use]
    pub const fn scoped(tag: String, beacon_id: String) -> Self {
        Self {
            tag,
            beacon_id: Some(beacon_id),
        }
    }

    /// Migrate a legacy `FAMILY_SEED` plaintext value to a genetic tag.
    ///
    /// This is the backward-compatibility bridge: existing `FAMILY_SEED`
    /// env values become tags in the open participation tier. They no
    /// longer carry authentication weight (that's nuclear genetics) or
    /// discovery capability (that's mito-beacons).
    #[must_use]
    pub fn from_legacy_family_seed(seed: &str) -> Self {
        Self {
            tag: seed.to_owned(),
            beacon_id: None,
        }
    }

    /// Whether this tag was migrated from a legacy family seed.
    ///
    /// Heuristic: tags from `FAMILY_SEED` are typically long hex/base64
    /// strings without a `#` prefix or beacon scope.
    #[must_use]
    pub fn is_legacy(&self) -> bool {
        self.beacon_id.is_none()
            && !self.tag.starts_with('#')
            && self.tag.len() > 16
    }
}

impl std::fmt::Display for GeneticTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref bid) = self.beacon_id {
            write!(f, "{}:{}", bid, self.tag)
        } else {
            write!(f, "{}", self.tag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tag() {
        let tag = GeneticTag::new("#build-lab".to_owned());
        assert_eq!(tag.tag, "#build-lab");
        assert!(tag.beacon_id.is_none());
        assert!(!tag.is_legacy());
    }

    #[test]
    fn scoped_tag() {
        let tag = GeneticTag::scoped(
            "#gpu-cluster".to_owned(),
            "guild-storm".to_owned(),
        );
        assert_eq!(tag.beacon_id.as_deref(), Some("guild-storm"));
        assert_eq!(tag.to_string(), "guild-storm:#gpu-cluster");
    }

    #[test]
    fn legacy_migration() {
        let tag = GeneticTag::from_legacy_family_seed(
            "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
        );
        assert!(tag.is_legacy());
    }

    #[test]
    fn short_tag_not_legacy() {
        let tag = GeneticTag::new("test".to_owned());
        assert!(!tag.is_legacy());
    }

    #[test]
    fn tag_equality() {
        let t1 = GeneticTag::new("#chat".to_owned());
        let t2 = GeneticTag::new("#chat".to_owned());
        assert_eq!(t1, t2);
    }

    #[test]
    fn tag_is_clone() {
        let original = GeneticTag::new("#dev".to_owned());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}
