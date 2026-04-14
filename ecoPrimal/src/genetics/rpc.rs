// SPDX-License-Identifier: AGPL-3.0-or-later

//! BearDog genetic RPC client.
//!
//! Typed wrappers around BearDog's `genetic.*` JSON-RPC methods,
//! translating raw RPC responses into the three-tier genetics types.
//!
//! All cryptographic operations are delegated to BearDog — primalSpring
//! never derives keys itself. This module only marshals parameters and
//! deserializes results.
//!
//! # RPC Methods
//!
//! | Method                            | Tier    | Description                              |
//! |-----------------------------------|---------|------------------------------------------|
//! | `genetic.derive_lineage_beacon_key` | Mito    | Derive a beacon key for dark forest discovery |
//! | `genetic.derive_lineage_key`      | Nuclear | Derive a lineage key with generational mixing |
//! | `genetic.mix_entropy`             | Nuclear | Three-tier entropy mixing (human, supervised, machine) |
//! | `genetic.generate_lineage_proof`  | Nuclear | Generate a cryptographic lineage proof    |
//! | `genetic.verify_lineage`          | Nuclear | Verify a lineage proof chain              |

use serde::{Deserialize, Serialize};

use crate::ipc::client::PrimalClient;
use crate::ipc::error::IpcError;

use super::mito_beacon::MitoBeacon;
use super::nuclear::NuclearGenetics;

/// Parameters for `genetic.derive_lineage_beacon_key`.
#[derive(Debug, Serialize)]
struct DeriveBeaconKeyParams {
    lineage_seed: String,
    domain: String,
}

/// Parameters for `genetic.derive_lineage_key`.
#[derive(Debug, Serialize)]
struct DeriveLineageKeyParams {
    lineage_seed: String,
    domain: String,
    generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_key_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_entropy: Option<String>,
}

/// Parameters for `genetic.mix_entropy`.
#[derive(Debug, Serialize)]
struct MixEntropyParams {
    tiers: Vec<String>,
}

/// Parameters for `genetic.generate_lineage_proof`.
#[derive(Debug, Serialize)]
struct GenerateProofParams {
    lineage_seed: String,
    generation: u64,
    parent_key_hash: Option<String>,
    context: String,
}

/// Parameters for `genetic.verify_lineage`.
#[derive(Debug, Serialize)]
struct VerifyLineageParams {
    proof: String,
    claimed_generation: u64,
    claimed_parent_hash: Option<String>,
    context: String,
}

/// Result from `genetic.derive_lineage_beacon_key`.
#[derive(Debug, Deserialize)]
struct BeaconKeyResult {
    beacon_key: String,
    beacon_id: String,
}

/// Result from `genetic.derive_lineage_key`.
#[derive(Debug, Deserialize)]
struct LineageKeyResult {
    lineage_key: String,
    generation: u64,
    parent_hash: Option<String>,
}

/// Result from `genetic.mix_entropy`.
#[derive(Debug, Deserialize)]
struct MixEntropyResult {
    mixed: String,
}

/// Result from `genetic.generate_lineage_proof`.
#[derive(Debug, Deserialize)]
struct ProofResult {
    proof: String,
}

/// Result from `genetic.verify_lineage`.
#[derive(Debug, Deserialize)]
struct VerifyResult {
    valid: bool,
}

/// Derive a mito-beacon key from BearDog via `genetic.derive_lineage_beacon_key`.
///
/// The `lineage_seed` is the family's root seed (what was formerly the
/// plaintext `FAMILY_SEED`). The `domain` scopes the beacon derivation
/// (e.g. `"birdsong_beacon_v1"`).
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn derive_lineage_beacon_key(
    client: &mut PrimalClient,
    lineage_seed: &str,
    domain: &str,
    group_name: &str,
) -> Result<MitoBeacon, IpcError> {
    let params = DeriveBeaconKeyParams {
        lineage_seed: lineage_seed.to_owned(),
        domain: domain.to_owned(),
    };
    let result: BeaconKeyResult = client.call_extract(
        "genetic.derive_lineage_beacon_key",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;

    let key_bytes = hex_decode(&result.beacon_key)?;
    Ok(MitoBeacon::new(result.beacon_id, group_name.to_owned(), key_bytes))
}

/// Derive a nuclear lineage key from BearDog via `genetic.derive_lineage_key`.
///
/// For genesis (generation 0), `parent` should be `None`.
/// For child generations, pass the parent's `NuclearGenetics` reference.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn derive_lineage_key(
    client: &mut PrimalClient,
    lineage_seed: &str,
    domain: &str,
    parent: Option<&NuclearGenetics>,
    context_entropy: Option<&[u8]>,
) -> Result<NuclearGenetics, IpcError> {
    let (generation, parent_key_hash) = parent.map_or((0, None), |p| {
        (p.generation() + 1, Some(hex_encode(p.key_hash())))
    });

    let params = DeriveLineageKeyParams {
        lineage_seed: lineage_seed.to_owned(),
        domain: domain.to_owned(),
        generation,
        parent_key_hash,
        context_entropy: context_entropy.map(hex_encode_slice),
    };
    let result: LineageKeyResult = client.call_extract(
        "genetic.derive_lineage_key",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;

    let lineage_key = hex_decode(&result.lineage_key)?;

    let proof_bytes = generate_lineage_proof(
        client,
        lineage_seed,
        result.generation,
        result.parent_hash.as_deref(),
        domain,
    )?;

    if let Some(p) = parent {
        Ok(p.spawn_child(lineage_key, proof_bytes, domain.to_owned()))
    } else {
        Ok(NuclearGenetics::genesis(lineage_key, proof_bytes, domain.to_owned()))
    }
}

/// Mix entropy tiers via BearDog `genetic.mix_entropy`.
///
/// BearDog mixes human, supervised, and machine entropy tiers using
/// HKDF-SHA256 to produce a combined entropy blob suitable for key
/// derivation or nonce generation.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn mix_entropy(
    client: &mut PrimalClient,
    tiers: &[&[u8]],
) -> Result<Vec<u8>, IpcError> {
    let params = MixEntropyParams {
        tiers: tiers.iter().map(|t| hex_encode_slice(t)).collect(),
    };
    let result: MixEntropyResult = client.call_extract(
        "genetic.mix_entropy",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;
    hex_decode(&result.mixed)
}

/// Generate a lineage proof via BearDog `genetic.generate_lineage_proof`.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn generate_lineage_proof(
    client: &mut PrimalClient,
    lineage_seed: &str,
    generation: u64,
    parent_key_hash: Option<&str>,
    context: &str,
) -> Result<Vec<u8>, IpcError> {
    let params = GenerateProofParams {
        lineage_seed: lineage_seed.to_owned(),
        generation,
        parent_key_hash: parent_key_hash.map(ToOwned::to_owned),
        context: context.to_owned(),
    };
    let result: ProofResult = client.call_extract(
        "genetic.generate_lineage_proof",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;
    hex_decode(&result.proof)
}

/// Verify a lineage proof via BearDog `genetic.verify_lineage`.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn verify_lineage(
    client: &mut PrimalClient,
    proof: &[u8],
    claimed_generation: u64,
    claimed_parent_hash: Option<&[u8; 32]>,
    context: &str,
) -> Result<bool, IpcError> {
    let params = VerifyLineageParams {
        proof: hex_encode_slice(proof),
        claimed_generation,
        claimed_parent_hash: claimed_parent_hash.map(|h| hex_encode(*h)),
        context: context.to_owned(),
    };
    let result: VerifyResult = client.call_extract(
        "genetic.verify_lineage",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;
    Ok(result.valid)
}

fn hex_encode(data: [u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in data {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn hex_encode_slice(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for b in data {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, IpcError> {
    if !hex.len().is_multiple_of(2) {
        return Err(IpcError::SerializationError {
            detail: format!("odd hex length: {}", hex.len()),
        });
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| IpcError::SerializationError {
                detail: format!("invalid hex at {i}: {e}"),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trip() {
        let data = [0xDE, 0xAD, 0xBE, 0xEF];
        let encoded = hex_encode_slice(&data);
        assert_eq!(encoded, "deadbeef");
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn hex_decode_odd_length() {
        assert!(hex_decode("abc").is_err());
    }

    #[test]
    fn hex_decode_invalid_chars() {
        assert!(hex_decode("gg").is_err());
    }

    #[test]
    fn hex_encode_32_bytes() {
        let data = [0u8; 32];
        let s = hex_encode(data);
        assert_eq!(s.len(), 64);
        assert!(s.chars().all(|c| c == '0'));
    }

    #[test]
    fn derive_beacon_params_serialize() {
        let p = DeriveBeaconKeyParams {
            lineage_seed: "seed".to_owned(),
            domain: "birdsong_beacon_v1".to_owned(),
        };
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["domain"], "birdsong_beacon_v1");
    }

    #[test]
    fn derive_lineage_params_skip_none() {
        let p = DeriveLineageKeyParams {
            lineage_seed: "seed".to_owned(),
            domain: "test".to_owned(),
            generation: 0,
            parent_key_hash: None,
            context_entropy: None,
        };
        let v = serde_json::to_value(&p).unwrap();
        assert!(v.get("parent_key_hash").is_none());
        assert!(v.get("context_entropy").is_none());
    }

    #[test]
    fn beacon_key_result_deserialize() {
        let json = r#"{"beacon_key":"aabb","beacon_id":"test-beacon"}"#;
        let r: BeaconKeyResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.beacon_id, "test-beacon");
    }

    #[test]
    fn verify_result_deserialize() {
        let json = r#"{"valid":true}"#;
        let r: VerifyResult = serde_json::from_str(json).unwrap();
        assert!(r.valid);
    }
}
