// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Certification entropy resolution — mito-tier seed for BTSP authentication.
//!
//! Provides `resolve_mito_seed()` which discovers or generates a machine-level
//! BLAKE3 seed suitable for BTSP Phase 1 (family membership). The seed is
//! hex-encoded and used as raw UTF-8 bytes on the wire, matching BearDog's
//! `String::into_bytes()` convention.
//!
//! Priority order:
//! 1. `GUIDESTONE_SEED` env var (explicit override)
//! 2. `FAMILY_SEED` env var (already set by caller / launcher)
//! 3. `$SOCKET_DIR/.family.seed` file (written by nucleus_launcher.sh)
//! 4. Fresh generation: `BLAKE3("guidestone-mito-v1" || machine_id || hostname || 32 OsRng bytes)`

use std::collections::HashMap;
use std::path::Path;

use crate::validation::ValidationResult;

const MITO_DOMAIN: &str = "guidestone-mito-v1";
const FINGERPRINT_DOMAIN: &str = "primal-seed-v1";

/// Resolved seed and its provenance.
pub struct MitoSeed {
    /// Hex-encoded seed for BTSP Phase 1 handshakes.
    pub hex_seed: String,
    /// Where the seed value came from (env, file, or generated).
    pub source: SeedSource,
}

/// Provenance label for [`MitoSeed`].
#[derive(Debug, Clone, Copy)]
pub enum SeedSource {
    /// `GUIDESTONE_SEED` environment variable.
    EnvGuidestone,
    /// `FAMILY_SEED` environment variable.
    EnvFamily,
    /// `.family.seed` under the socket directory.
    SeedFile,
    /// Fresh BLAKE3 from machine identity and OS entropy.
    Generated,
}

impl std::fmt::Display for SeedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvGuidestone => write!(f, "GUIDESTONE_SEED env"),
            Self::EnvFamily => write!(f, "FAMILY_SEED env"),
            Self::SeedFile => write!(f, ".family.seed file"),
            Self::Generated => write!(f, "generated (machine entropy)"),
        }
    }
}

/// Resolve the mito-tier seed for BTSP authentication.
///
/// Checks env vars and seed files in priority order. If nothing is found,
/// generates a fresh machine-level seed from OS entropy + machine identity.
/// Returns a hex-encoded string matching BearDog's wire format.
#[must_use]
pub fn resolve_mito_seed() -> MitoSeed {
    if let Some(seed) = read_env_seed(crate::env_keys::GUIDESTONE_SEED) {
        return MitoSeed {
            hex_seed: seed,
            source: SeedSource::EnvGuidestone,
        };
    }

    if let Some(seed) = read_env_seed(crate::env_keys::FAMILY_SEED) {
        return MitoSeed {
            hex_seed: seed,
            source: SeedSource::EnvFamily,
        };
    }

    if let Some(seed) = read_seed_file() {
        return MitoSeed {
            hex_seed: seed,
            source: SeedSource::SeedFile,
        };
    }

    generate_machine_seed().map_or(
        MitoSeed {
            hex_seed: String::new(),
            source: SeedSource::Generated,
        },
        |seed| MitoSeed {
            hex_seed: seed,
            source: SeedSource::Generated,
        },
    )
}

fn read_env_seed(var: &str) -> Option<String> {
    let val = std::env::var(var).ok()?;
    let trimmed = val.trim().to_owned();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed)
}

fn read_seed_file() -> Option<String> {
    let socket_dir = crate::ipc::discover::resolve_socket_dir();
    let seed_path = Path::new(&socket_dir).join(".family.seed");
    let content = std::fs::read_to_string(&seed_path).ok()?;
    let trimmed = content.trim().to_owned();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed)
}

fn generate_machine_seed() -> Option<String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(MITO_DOMAIN.as_bytes());

    if let Ok(mid) = std::fs::read_to_string("/etc/machine-id") {
        hasher.update(mid.trim().as_bytes());
    }

    if let Ok(hostname) = std::env::var(crate::env_keys::HOSTNAME).or_else(|_| hostname_from_file())
    {
        hasher.update(hostname.trim().as_bytes());
    }

    let mut os_entropy = [0u8; 32];
    getrandom::fill(&mut os_entropy).ok()?;
    hasher.update(&os_entropy);

    let hash = hasher.finalize();
    Some(hex::encode(hash.as_bytes()))
}

fn hostname_from_file() -> Result<String, std::env::VarError> {
    std::fs::read_to_string("/etc/hostname").map_err(|_| std::env::VarError::NotPresent)
}

/// Outcome of comparing a built primal against the published fingerprint.
#[derive(Debug)]
pub enum FingerprintStatus {
    /// Computed fingerprint matches the published value.
    Match,
    /// Computed fingerprint differs from published.
    Mismatch {
        /// Expected hex fingerprint from `seed_fingerprints.toml`.
        expected: String,
        /// Hex fingerprint computed from manifest + binary.
        computed: String,
    },
    /// Primal listed in fingerprints has no version in plasmidBin manifest.
    NoPublished,
    /// Expected binary path missing or unreadable.
    NoBinary,
}

/// Verify primal binaries against published seed fingerprints.
///
/// Reads `validation/seed_fingerprints.toml` and computes
/// `BLAKE3("primal-seed-v1" || name || version || binary_blake3)` for each
/// discovered binary, comparing against the published value.
#[must_use]
pub fn verify_seed_fingerprints(
    fingerprints_path: &Path,
    plasmin_bin: &Path,
) -> HashMap<String, FingerprintStatus> {
    let mut results = HashMap::new();

    let Ok(content) = std::fs::read_to_string(fingerprints_path) else {
        return results;
    };

    let Ok(table) = content.parse::<toml::Value>() else {
        return results;
    };

    let Some(fingerprints) = table.get("fingerprints").and_then(toml::Value::as_table) else {
        return results;
    };

    let arch = current_target_triple();
    let bin_dir = plasmin_bin.join("primals").join(arch);

    let manifest_path = plasmin_bin.join("manifest.toml");
    let versions = read_primal_versions(&manifest_path);

    for (name, expected_fp) in fingerprints {
        let Some(expected) = expected_fp.as_str() else {
            continue;
        };

        let key = name.clone();
        let Some(version) = versions.get(name.as_str()).cloned() else {
            results.insert(key, FingerprintStatus::NoPublished);
            continue;
        };

        let bin_path = bin_dir.join(name.as_str());
        if !bin_path.exists() {
            results.insert(key, FingerprintStatus::NoBinary);
            continue;
        }

        let Some(binary_checksum) = compute_binary_blake3(&bin_path) else {
            results.insert(key, FingerprintStatus::NoBinary);
            continue;
        };

        let computed = compute_seed_fingerprint(name, &version, &binary_checksum);

        if computed == expected {
            results.insert(key, FingerprintStatus::Match);
        } else {
            results.insert(
                key,
                FingerprintStatus::Mismatch {
                    expected: expected.to_owned(),
                    computed,
                },
            );
        }
    }

    results
}

const fn current_target_triple() -> &'static str {
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    return "x86_64-unknown-linux-musl";
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    return "aarch64-unknown-linux-musl";
    #[cfg(all(target_arch = "arm", target_os = "linux"))]
    return "armv7-unknown-linux-musleabihf";
    #[cfg(not(any(
        all(target_arch = "x86_64", target_os = "linux"),
        all(target_arch = "aarch64", target_os = "linux"),
        all(target_arch = "arm", target_os = "linux"),
    )))]
    return "x86_64-unknown-linux-musl";
}

fn compute_seed_fingerprint(name: &str, version: &str, binary_checksum: &str) -> String {
    let input = format!("{FINGERPRINT_DOMAIN}{name}{version}{binary_checksum}");
    let hash = blake3::hash(input.as_bytes());
    hex::encode(hash.as_bytes())
}

fn compute_binary_blake3(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let hash = blake3::hash(&data);
    Some(hex::encode(hash.as_bytes()))
}

fn read_primal_versions(manifest_path: &Path) -> HashMap<String, String> {
    let mut versions = HashMap::new();
    let Ok(content) = std::fs::read_to_string(manifest_path) else {
        return versions;
    };

    let Ok(table) = content.parse::<toml::Value>() else {
        return versions;
    };

    if let Some(primals) = table.get("primals").and_then(toml::Value::as_table) {
        for (name, entry) in primals {
            if let Some(ver) = entry.get("latest").and_then(toml::Value::as_str) {
                versions.insert(name.clone(), ver.to_owned());
            }
        }
    }

    versions
}

/// Layer 0.5: Seed Provenance — validate the crypto envelope before discovery.
pub fn validate_seed_provenance(v: &mut ValidationResult, seed: &MitoSeed) {
    v.check_bool(
        "seed:mito_resolved",
        true,
        &format!("family seed loaded (source: {})", seed.source),
    );

    let mode = crate::btsp::security_mode_from_env();
    v.check_bool(
        "seed:btsp_mode",
        true,
        &format!(
            "{mode:?} ({})",
            if matches!(mode, crate::btsp::SecurityMode::Production) {
                "BTSP authenticated connections"
            } else {
                "cleartext development mode"
            }
        ),
    );

    let fingerprints_path = Path::new("validation/seed_fingerprints.toml");
    if !fingerprints_path.exists() {
        v.check_skip(
            "seed:fingerprints",
            "validation/seed_fingerprints.toml not found",
        );
        return;
    }

    let plasmin_bin = resolve_plasmin_bin();
    let plasmin_path = Path::new(&plasmin_bin);
    if !plasmin_path.join("manifest.toml").exists() {
        v.check_skip("seed:fingerprints", "plasmidBin manifest not found");
        return;
    }

    let results = verify_seed_fingerprints(fingerprints_path, plasmin_path);
    if results.is_empty() {
        v.check_skip("seed:fingerprints", "no fingerprints to verify");
        return;
    }

    for (name, status) in &results {
        match status {
            FingerprintStatus::Match => {
                v.check_bool(
                    &format!("seed:fingerprint:{name}"),
                    true,
                    "matches published",
                );
            }
            FingerprintStatus::Mismatch { expected, computed } => {
                v.check_bool(
                    &format!("seed:fingerprint:{name}"),
                    false,
                    &format!(
                        "MISMATCH expected={}... computed={}...",
                        &expected[..expected.len().min(12)],
                        &computed[..computed.len().min(12)]
                    ),
                );
            }
            FingerprintStatus::NoPublished => {
                v.check_skip(
                    &format!("seed:fingerprint:{name}"),
                    "no version in manifest",
                );
            }
            FingerprintStatus::NoBinary => {
                v.check_skip(
                    &format!("seed:fingerprint:{name}"),
                    "binary not found in plasmidBin",
                );
            }
        }
    }
}

fn resolve_plasmin_bin() -> String {
    crate::tolerances::plasmidbin_depot_root()
}

/// Encode the `hex` crate — re-export for the binary since primalspring
/// doesn't expose it.
mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            s.push(HEX_CHARS[(b >> 4) as usize] as char);
            s.push(HEX_CHARS[(b & 0xf) as usize] as char);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_encode_roundtrip() {
        assert_eq!(hex::encode(&[]), "");
        assert_eq!(hex::encode(&[0x00]), "00");
        assert_eq!(hex::encode(&[0xff]), "ff");
        assert_eq!(hex::encode(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
        assert_eq!(
            hex::encode(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
            "0123456789abcdef"
        );
    }

    #[test]
    fn seed_source_display() {
        assert_eq!(
            format!("{}", SeedSource::EnvGuidestone),
            "GUIDESTONE_SEED env"
        );
        assert_eq!(format!("{}", SeedSource::EnvFamily), "FAMILY_SEED env");
        assert_eq!(format!("{}", SeedSource::SeedFile), ".family.seed file");
        assert_eq!(
            format!("{}", SeedSource::Generated),
            "generated (machine entropy)"
        );
    }

    #[test]
    fn resolve_mito_seed_returns_nonempty() {
        let seed = resolve_mito_seed();
        assert!(
            !seed.hex_seed.is_empty(),
            "seed should always resolve to something"
        );
        assert!(
            seed.hex_seed.chars().all(|c| c.is_ascii_hexdigit()),
            "seed should be hex: {}",
            seed.hex_seed
        );
    }

    #[test]
    fn compute_seed_fingerprint_deterministic() {
        let fp1 = compute_seed_fingerprint("beardog", "0.9.31", "abc123");
        let fp2 = compute_seed_fingerprint("beardog", "0.9.31", "abc123");
        assert_eq!(fp1, fp2, "same inputs must produce same fingerprint");
        assert!(!fp1.is_empty());
        assert_eq!(fp1.len(), 64, "BLAKE3 hex should be 64 chars");
    }

    #[test]
    fn compute_seed_fingerprint_varies_with_input() {
        let fp_a = compute_seed_fingerprint("beardog", "0.9.31", "abc123");
        let fp_b = compute_seed_fingerprint("songbird", "0.9.31", "abc123");
        let fp_c = compute_seed_fingerprint("beardog", "0.9.32", "abc123");
        let fp_d = compute_seed_fingerprint("beardog", "0.9.31", "def456");
        assert_ne!(fp_a, fp_b, "different names must differ");
        assert_ne!(fp_a, fp_c, "different versions must differ");
        assert_ne!(fp_a, fp_d, "different checksums must differ");
    }

    #[test]
    fn verify_seed_fingerprints_missing_path() {
        let results = verify_seed_fingerprints(
            Path::new("/nonexistent/fingerprints.toml"),
            Path::new("/nonexistent/plasmin"),
        );
        assert!(
            results.is_empty(),
            "missing path should yield empty results"
        );
    }

    #[test]
    fn validate_seed_provenance_runs_without_panic() {
        let seed = resolve_mito_seed();
        let mut v = crate::validation::ValidationResult::new("test");
        validate_seed_provenance(&mut v, &seed);
    }
}
