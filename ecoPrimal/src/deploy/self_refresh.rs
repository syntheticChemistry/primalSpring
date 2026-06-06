// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Self-refresh: fetch primal binaries from GitHub/Forgejo Releases.
//!
//! Provides the Rust-native capability for a deployed NUCLEUS to
//! update its own binaries from a release channel without manual
//! intervention. Replaces the `deploy_membrane.sh self-refresh` bash
//! flow with typed, provenance-aware binary fetching.
//!
//! # Source Priority
//!
//! 1. **plasmidBin local** — `$ECOPRIMALS_PLASMID_BIN/primals/{triple}/{name}`
//! 2. **VPS rsync** — `rsync user@vps:/srv/plasmidBin/primals/{triple}/{name}`
//! 3. **GitHub Releases** — `https://github.com/org/repo/releases/latest`
//! 4. **Forgejo Releases** — sovereign mirror (when available)
//!
//! The source is selected by the `PLASMIDBIN_SOURCE` env var or defaults
//! to `local` → `github` fallback.

use std::path::{Path, PathBuf};

/// Binary source for self-refresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshSource {
    /// Local plasmidBin directory.
    Local,
    /// GitHub Releases (outer membrane).
    GitHub,
    /// Forgejo Releases (inner membrane).
    Forgejo,
    /// VPS rsync (direct membrane).
    Vps,
}

impl RefreshSource {
    /// Parse from env var or string.
    pub fn from_str_or_default(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "github" | "gh" => Self::GitHub,
            "forgejo" | "fg" => Self::Forgejo,
            "vps" | "rsync" => Self::Vps,
            _ => Self::Local,
        }
    }

    /// Read the configured source from `PLASMIDBIN_SOURCE` env var.
    pub fn from_env() -> Self {
        std::env::var("PLASMIDBIN_SOURCE")
            .map(|s| Self::from_str_or_default(&s))
            .unwrap_or(Self::Local)
    }
}

/// Metadata for a binary refresh operation.
#[derive(Debug, Clone)]
pub struct RefreshTarget {
    /// Primal slug (e.g., `"beardog"`).
    pub slug: String,
    /// Target triple (e.g., `"x86_64-unknown-linux-musl"`).
    pub triple: String,
    /// Expected BLAKE3 checksum (hex), if known from manifest.
    pub expected_checksum: Option<String>,
}

/// Result of a refresh attempt.
#[derive(Debug)]
pub struct RefreshResult {
    /// Which binary was refreshed.
    pub target: RefreshTarget,
    /// Source used.
    pub source: RefreshSource,
    /// Where the binary was placed.
    pub destination: PathBuf,
    /// Whether checksum was verified.
    pub checksum_verified: bool,
}

/// Resolve the local plasmidBin binary path.
///
/// Checks `$ECOPRIMALS_PLASMID_BIN`, then `$XDG_DATA_HOME/ecoPrimals/plasmidBin`,
/// then `~/.local/share/ecoPrimals/plasmidBin`.
#[must_use]
pub fn resolve_plasmidbin_root() -> PathBuf {
    if let Ok(val) = std::env::var("ECOPRIMALS_PLASMID_BIN") {
        return PathBuf::from(val);
    }
    let data_home = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_owned());
            format!("{home}/.local/share")
        });
    PathBuf::from(data_home).join("ecoPrimals/plasmidBin")
}

/// Locate a primal binary in the local plasmidBin.
///
/// Returns `Some(path)` if found, `None` otherwise.
#[must_use]
pub fn locate_local_binary(slug: &str, triple: &str) -> Option<PathBuf> {
    let root = resolve_plasmidbin_root();
    let path = root.join("primals").join(triple).join(slug);
    path.exists().then_some(path)
}

/// Construct the GitHub Releases download URL for a primal binary.
///
/// URL pattern: `https://github.com/{org}/{repo}/releases/latest/download/{slug}-{triple}`
#[must_use]
pub fn github_release_url(org: &str, repo: &str, slug: &str, triple: &str) -> String {
    format!(
        "https://github.com/{org}/{repo}/releases/latest/download/{slug}-{triple}"
    )
}

/// Check if a binary at `path` matches the expected BLAKE3 checksum.
///
/// Returns `true` if the checksum matches, `false` if it doesn't or
/// if the file cannot be read.
pub fn verify_blake3(path: &Path, expected_hex: &str) -> bool {
    let Ok(data) = std::fs::read(path) else {
        return false;
    };
    let hash = blake3::hash(&data);
    hash.to_hex().as_str() == expected_hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_source_from_str() {
        assert_eq!(RefreshSource::from_str_or_default("github"), RefreshSource::GitHub);
        assert_eq!(RefreshSource::from_str_or_default("gh"), RefreshSource::GitHub);
        assert_eq!(RefreshSource::from_str_or_default("forgejo"), RefreshSource::Forgejo);
        assert_eq!(RefreshSource::from_str_or_default("vps"), RefreshSource::Vps);
        assert_eq!(RefreshSource::from_str_or_default("local"), RefreshSource::Local);
        assert_eq!(RefreshSource::from_str_or_default("unknown"), RefreshSource::Local);
    }

    #[test]
    fn resolve_plasmidbin_root_returns_path() {
        let root = resolve_plasmidbin_root();
        assert!(!root.as_os_str().is_empty());
    }

    #[test]
    fn github_release_url_format() {
        let url = github_release_url("ecoKitchen", "plasmidBin", "beardog", "x86_64-unknown-linux-musl");
        assert!(url.contains("github.com"));
        assert!(url.contains("beardog-x86_64-unknown-linux-musl"));
    }

    #[test]
    fn verify_blake3_nonexistent_file() {
        assert!(!verify_blake3(Path::new("/nonexistent/path"), "deadbeef"));
    }

    #[test]
    fn locate_local_binary_returns_none_for_unknown() {
        assert!(locate_local_binary("nonexistent_primal_xyz", "x86_64-unknown-linux-musl").is_none());
    }
}
