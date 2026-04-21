// SPDX-License-Identifier: AGPL-3.0-or-later

//! Guidestone entropy resolution — mito-tier seed for BTSP authentication.
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

use primalspring::validation::ValidationResult;

const MITO_DOMAIN: &str = "guidestone-mito-v1";
const FINGERPRINT_DOMAIN: &str = "primal-seed-v1";

/// Resolved seed and its provenance.
pub struct MitoSeed {
    /// Hex-encoded seed for BTSP Phase 1 handshakes.
    pub hex_seed: String,
    pub source: SeedSource,
}

#[derive(Debug, Clone, Copy)]
pub enum SeedSource {
    EnvGuidestone,
    EnvFamily,
    SeedFile,
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
pub fn resolve_mito_seed() -> MitoSeed {
    if let Some(seed) = read_env_seed("GUIDESTONE_SEED") {
        return MitoSeed {
            hex_seed: seed,
            source: SeedSource::EnvGuidestone,
        };
    }

    if let Some(seed) = read_env_seed("FAMILY_SEED") {
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

    MitoSeed {
        hex_seed: generate_machine_seed(),
        source: SeedSource::Generated,
    }
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
    let socket_dir = std::env::var("SOCKET_DIR")
        .ok()
        .unwrap_or_else(|| "/tmp/ecoprimals".to_owned());
    let seed_path = Path::new(&socket_dir).join(".family.seed");
    let content = std::fs::read_to_string(&seed_path).ok()?;
    let trimmed = content.trim().to_owned();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed)
}

fn generate_machine_seed() -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(MITO_DOMAIN.as_bytes());

    if let Ok(mid) = std::fs::read_to_string("/etc/machine-id") {
        hasher.update(mid.trim().as_bytes());
    }

    if let Ok(hostname) = std::env::var("HOSTNAME")
        .or_else(|_| hostname_from_file())
    {
        hasher.update(hostname.trim().as_bytes());
    }

    let mut os_entropy = [0u8; 32];
    getrandom::fill(&mut os_entropy).expect("OS entropy unavailable");
    hasher.update(&os_entropy);

    let hash = hasher.finalize();
    hex::encode(hash.as_bytes())
}

fn hostname_from_file() -> Result<String, std::env::VarError> {
    std::fs::read_to_string("/etc/hostname")
        .map_err(|_| std::env::VarError::NotPresent)
}

/// Fingerprint verification result for a single primal.
#[derive(Debug)]
pub enum FingerprintStatus {
    Match,
    Mismatch { expected: String, computed: String },
    NoPublished,
    NoBinary,
}

/// Verify primal binaries against published seed fingerprints.
///
/// Reads `validation/seed_fingerprints.toml` and computes
/// `BLAKE3("primal-seed-v1" || name || version || binary_blake3)` for each
/// discovered binary, comparing against the published value.
pub fn verify_seed_fingerprints(
    fingerprints_path: &Path,
    plasmin_bin: &Path,
) -> HashMap<String, FingerprintStatus> {
    let mut results = HashMap::new();

    let content = match std::fs::read_to_string(fingerprints_path) {
        Ok(c) => c,
        Err(_) => return results,
    };

    let table: toml::Value = match content.parse() {
        Ok(t) => t,
        Err(_) => return results,
    };

    let fingerprints = match table.get("fingerprints").and_then(|f| f.as_table()) {
        Some(f) => f,
        None => return results,
    };

    let arch = "x86_64-unknown-linux-musl";
    let bin_dir = plasmin_bin.join("primals").join(arch);

    let manifest_path = plasmin_bin.join("manifest.toml");
    let versions = read_primal_versions(&manifest_path);

    for (name, expected_fp) in fingerprints {
        let expected = match expected_fp.as_str() {
            Some(s) => s,
            None => continue,
        };

        let version = match versions.get(name.as_str()) {
            Some(v) => v.clone(),
            None => {
                results.insert(name.clone(), FingerprintStatus::NoPublished);
                continue;
            }
        };

        let bin_path = bin_dir.join(name);
        if !bin_path.exists() {
            results.insert(name.clone(), FingerprintStatus::NoBinary);
            continue;
        }

        let binary_checksum = match compute_binary_blake3(&bin_path) {
            Some(c) => c,
            None => {
                results.insert(name.clone(), FingerprintStatus::NoBinary);
                continue;
            }
        };

        let computed = compute_seed_fingerprint(name, &version, &binary_checksum);

        if computed == expected {
            results.insert(name.clone(), FingerprintStatus::Match);
        } else {
            results.insert(
                name.clone(),
                FingerprintStatus::Mismatch {
                    expected: expected.to_owned(),
                    computed,
                },
            );
        }
    }

    results
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
    let content = match std::fs::read_to_string(manifest_path) {
        Ok(c) => c,
        Err(_) => return versions,
    };

    let table: toml::Value = match content.parse() {
        Ok(t) => t,
        Err(_) => return versions,
    };

    if let Some(primals) = table.get("primals").and_then(|p| p.as_table()) {
        for (name, entry) in primals {
            if let Some(ver) = entry.get("latest").and_then(|v| v.as_str()) {
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

    let mode = primalspring::btsp::security_mode_from_env();
    v.check_bool(
        "seed:btsp_mode",
        true,
        &format!("{mode:?} ({})",
            if matches!(mode, primalspring::btsp::SecurityMode::Production) {
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
    std::env::var("ECOPRIMALS_PLASMID_BIN").unwrap_or_else(|_| {
        "../../infra/plasmidBin".to_owned()
    })
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
