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
//! | Method                              | Tier    | Description                                  |
//! |-------------------------------------|---------|----------------------------------------------|
//! | `genetic.derive_lineage_beacon_key` | Mito    | Derive a beacon key for dark forest discovery |
//! | `genetic.derive_lineage_key`        | Nuclear | Derive a lineage key for auth/permissions     |
//! | `genetic.mix_entropy`               | Nuclear | Three-tier entropy mixing                     |
//! | `genetic.generate_lineage_proof`    | Nuclear | Generate a cryptographic lineage proof        |
//! | `genetic.verify_lineage`            | Nuclear | Verify a lineage proof                        |
//!
//! # Encoding
//!
//! BearDog uses **base64** for `lineage_seed`, `key`, `entropy`, `proof`
//! parameters and responses. Beacon keys are returned as **hex**. This
//! module handles the encoding/decoding transparently.
//!
//! # Generation Tracking
//!
//! BearDog's JSON-RPC API does not track generation numbers or parent
//! hashes — that provenance chain is maintained locally in
//! [`NuclearGenetics`]. Different `context` values produce distinct keys
//! from the same lineage seed, enabling the spawn-not-copy model.

use serde::{Deserialize, Serialize};

use crate::ipc::client::PrimalClient;
use crate::ipc::error::IpcError;

use super::mito_beacon::MitoBeacon;
use super::nuclear::NuclearGenetics;

// ── Param types (match BearDog's actual Deserialize structs) ─────────────

/// Parameters for `genetic.derive_lineage_beacon_key`.
///
/// BearDog extracts `lineage_seed` directly from the JSON params object.
#[derive(Debug, Serialize)]
struct DeriveBeaconKeyParams {
    lineage_seed: String,
}

/// Parameters for `genetic.derive_lineage_key`.
///
/// Matches `DeriveLineageKeyRequest` in beardog-tunnel.
#[derive(Debug, Serialize)]
struct DeriveLineageKeyParams {
    our_family_id: String,
    peer_family_id: String,
    context: String,
    lineage_seed: String,
}

/// Parameters for `genetic.mix_entropy`.
///
/// Matches `MixEntropyRequest` — all tiers optional (base64-encoded).
#[derive(Debug, Serialize)]
struct MixEntropyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    tier3_human: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier2_supervised: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier1_machine: Option<String>,
}

/// Parameters for `genetic.generate_lineage_proof`.
///
/// Matches `GenerateLineageProofRequest` in beardog-tunnel.
#[derive(Debug, Serialize)]
struct GenerateProofParams {
    our_family_id: String,
    peer_family_id: String,
    lineage_seed: String,
}

/// Parameters for `genetic.verify_lineage`.
///
/// Matches `VerifyLineageRequest` in beardog-tunnel.
#[derive(Debug, Serialize)]
struct VerifyLineageParams {
    our_family_id: String,
    peer_family_id: String,
    lineage_proof: String,
    lineage_seed: String,
}

// ── Response types (match BearDog's actual Serialize structs) ────────────

/// Result from `genetic.derive_lineage_beacon_key`.
///
/// BearDog returns `beacon_key` as hex-encoded 32 bytes.
#[derive(Debug, Deserialize)]
struct BeaconKeyResult {
    beacon_key: String,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    algorithm: Option<String>,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    domain: Option<String>,
}

/// Result from `genetic.derive_lineage_key`.
///
/// Matches `DeriveLineageKeyResponse` — `key` is base64-encoded 32 bytes.
#[derive(Debug, Deserialize)]
struct LineageKeyResult {
    key: String,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    method: Option<String>,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    quality_score: Option<f64>,
}

/// Result from `genetic.mix_entropy`.
///
/// Matches `MixEntropyResponse` — `entropy` is base64-encoded.
#[derive(Debug, Deserialize)]
struct MixEntropyResult {
    entropy: String,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    quality_score: Option<f64>,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    tiers_used: Option<u8>,
}

/// Result from `genetic.generate_lineage_proof`.
///
/// Matches `GenerateLineageProofResponse` — `proof` is base64-encoded.
#[derive(Debug, Deserialize)]
struct ProofResult {
    proof: String,
    #[expect(dead_code, reason = "protocol field: deserialized for completeness")]
    timestamp: Option<u64>,
}

/// Result from `genetic.verify_lineage`.
///
/// Matches `VerifyLineageResponse`.
#[derive(Debug, Deserialize)]
struct VerifyResult {
    valid: bool,
}

// ── Public API ───────────────────────────────────────────────────────────

/// Derive a mito-beacon key from BearDog via `genetic.derive_lineage_beacon_key`.
///
/// The `lineage_seed` is the family's root seed (base64-encoded). BearDog
/// uses HKDF-SHA256 with domain `birdsong_beacon_v1` to derive a 32-byte
/// beacon key. All family members with the same seed derive the same key.
///
/// `beacon_id` and `group_name` are metadata for the returned [`MitoBeacon`]
/// — BearDog does not track these.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn derive_lineage_beacon_key(
    client: &mut PrimalClient,
    lineage_seed: &str,
    beacon_id: &str,
    group_name: &str,
) -> Result<MitoBeacon, IpcError> {
    let params = DeriveBeaconKeyParams {
        lineage_seed: lineage_seed.to_owned(),
    };
    let result: BeaconKeyResult = client.call_extract(
        "genetic.derive_lineage_beacon_key",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;

    let key_bytes = hex_decode(&result.beacon_key)?;
    Ok(MitoBeacon::new(
        beacon_id.to_owned(),
        group_name.to_owned(),
        key_bytes,
    ))
}

/// Derive a nuclear lineage key from BearDog via `genetic.derive_lineage_key`.
///
/// For genesis (generation 0), `parent` should be `None`.
/// For child generations, pass the parent's `NuclearGenetics` reference.
///
/// BearDog's key derivation uses Blake3 KDF seeded by the lineage_seed,
/// family IDs, and context. Different `context` values produce distinct
/// keys — this is how the "spawn not copy" model works: each generation
/// uses a unique context derived from the parent chain.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn derive_lineage_key(
    client: &mut PrimalClient,
    lineage_seed: &str,
    our_family_id: &str,
    peer_family_id: &str,
    context: &str,
    parent: Option<&NuclearGenetics>,
) -> Result<NuclearGenetics, IpcError> {
    let params = DeriveLineageKeyParams {
        lineage_seed: lineage_seed.to_owned(),
        our_family_id: our_family_id.to_owned(),
        peer_family_id: peer_family_id.to_owned(),
        context: context.to_owned(),
    };
    let result: LineageKeyResult = client.call_extract(
        "genetic.derive_lineage_key",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;

    let lineage_key = b64_decode(&result.key)?;

    let proof_bytes = generate_lineage_proof(client, lineage_seed, our_family_id, peer_family_id)?;

    if let Some(p) = parent {
        Ok(p.spawn_child(lineage_key, proof_bytes, context.to_owned()))
    } else {
        Ok(NuclearGenetics::genesis(
            lineage_key,
            proof_bytes,
            context.to_owned(),
        ))
    }
}

/// Mix entropy tiers via BearDog `genetic.mix_entropy`.
///
/// BearDog mixes up to three entropy tiers using internal PRNG:
/// - `tier3_human`: Human lived experience entropy (base64)
/// - `tier2_supervised`: Human supervised machine entropy (base64)
/// - `tier1_machine`: Store bought machine entropy (base64)
///
/// All tiers are optional. Even with no tiers, BearDog returns machine
/// entropy from its internal PRNG.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn mix_entropy(
    client: &mut PrimalClient,
    tier3_human: Option<&[u8]>,
    tier2_supervised: Option<&[u8]>,
    tier1_machine: Option<&[u8]>,
) -> Result<Vec<u8>, IpcError> {
    let params = MixEntropyParams {
        tier3_human: tier3_human.map(b64_encode),
        tier2_supervised: tier2_supervised.map(b64_encode),
        tier1_machine: tier1_machine.map(b64_encode),
    };
    let result: MixEntropyResult = client.call_extract(
        "genetic.mix_entropy",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;
    b64_decode(&result.entropy)
}

/// Generate a lineage proof via BearDog `genetic.generate_lineage_proof`.
///
/// The proof is a Blake3 + HMAC commitment binding the lineage_seed to the
/// family ID pair. BearDog returns a base64-encoded proof blob.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn generate_lineage_proof(
    client: &mut PrimalClient,
    lineage_seed: &str,
    our_family_id: &str,
    peer_family_id: &str,
) -> Result<Vec<u8>, IpcError> {
    let params = GenerateProofParams {
        lineage_seed: lineage_seed.to_owned(),
        our_family_id: our_family_id.to_owned(),
        peer_family_id: peer_family_id.to_owned(),
    };
    let result: ProofResult = client.call_extract(
        "genetic.generate_lineage_proof",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;
    b64_decode(&result.proof)
}

/// Verify a lineage proof via BearDog `genetic.verify_lineage`.
///
/// Returns `true` if the proof is valid for the given family pair and
/// lineage seed.
///
/// # Errors
///
/// Returns [`IpcError`] on transport failure or if BearDog reports an error.
pub fn verify_lineage(
    client: &mut PrimalClient,
    lineage_seed: &str,
    our_family_id: &str,
    peer_family_id: &str,
    proof: &[u8],
) -> Result<bool, IpcError> {
    let params = VerifyLineageParams {
        lineage_seed: lineage_seed.to_owned(),
        our_family_id: our_family_id.to_owned(),
        peer_family_id: peer_family_id.to_owned(),
        lineage_proof: b64_encode(proof),
    };
    let result: VerifyResult = client.call_extract(
        "genetic.verify_lineage",
        serde_json::to_value(&params).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?,
    )?;
    Ok(result.valid)
}

// ── Encoding helpers ─────────────────────────────────────────────────────

fn b64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn b64_decode(s: &str) -> Result<Vec<u8>, IpcError> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .map_err(|e| IpcError::SerializationError {
            detail: format!("base64 decode: {e}"),
        })
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
        let encoded = data.iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write;
            let _ = write!(s, "{b:02x}");
            s
        });
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
    fn b64_round_trip() {
        let data = b"test data for base64";
        let encoded = b64_encode(data);
        let decoded = b64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn derive_beacon_params_serialize() {
        let p = DeriveBeaconKeyParams {
            lineage_seed: "c2VlZA==".to_owned(),
        };
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["lineage_seed"], "c2VlZA==");
        assert!(v.get("domain").is_none());
    }

    #[test]
    fn derive_lineage_params_serialize() {
        let p = DeriveLineageKeyParams {
            lineage_seed: "c2VlZA==".to_owned(),
            our_family_id: "family-a".to_owned(),
            peer_family_id: "family-b".to_owned(),
            context: "test-ctx".to_owned(),
        };
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["our_family_id"], "family-a");
        assert_eq!(v["context"], "test-ctx");
    }

    #[test]
    fn mix_entropy_params_skip_none() {
        let p = MixEntropyParams {
            tier3_human: Some("aGVsbG8=".to_owned()),
            tier2_supervised: None,
            tier1_machine: None,
        };
        let v = serde_json::to_value(&p).unwrap();
        assert!(v.get("tier3_human").is_some());
        assert!(v.get("tier2_supervised").is_none());
        assert!(v.get("tier1_machine").is_none());
    }

    #[test]
    fn beacon_key_result_deserialize() {
        let json =
            r#"{"beacon_key":"aabb","algorithm":"HKDF-SHA256","domain":"birdsong_beacon_v1"}"#;
        let r: BeaconKeyResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.beacon_key, "aabb");
    }

    #[test]
    fn lineage_key_result_deserialize() {
        let json = r#"{"key":"dGVzdA==","method":"Blake3-Lineage-KDF","quality_score":0.8}"#;
        let r: LineageKeyResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.key, "dGVzdA==");
    }

    #[test]
    fn mix_entropy_result_deserialize() {
        let json = r#"{"entropy":"dGVzdA==","quality_score":0.7,"tiers_used":2}"#;
        let r: MixEntropyResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.entropy, "dGVzdA==");
    }

    #[test]
    fn verify_result_deserialize() {
        let json = r#"{"valid":true}"#;
        let r: VerifyResult = serde_json::from_str(json).unwrap();
        assert!(r.valid);
    }

    #[test]
    fn proof_result_deserialize() {
        let json = r#"{"proof":"cHJvb2Y=","timestamp":1713100000}"#;
        let r: ProofResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.proof, "cHJvb2Y=");
    }
}
