// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP client-side handshake for authenticated IPC.
//!
//! Implements the 4-step handshake that biomeOS's Neural API server expects
//! when `FAMILY_ID` is set (production mode). The wire format is newline-
//! delimited JSON matching `biomeos-core::btsp_client`:
//!
//! ```text
//! 1. Client → Server:  ClientHello  (protocol, version, client_ephemeral_pub)
//! 2. Server → Client:  ServerHello  (version, server_ephemeral_pub, challenge, session_id)
//! 3. Client → Server:  ChallengeResponse  (response HMAC, preferred_cipher)
//! 4. Server → Client:  HandshakeComplete  (cipher, session_id)
//! ```
//!
//! Key derivation matches `BearDog` `btsp_handshake/crypto.rs`:
//! - `handshake_key = HKDF-SHA256(ikm=family_seed, salt="btsp-v1", info="handshake")`
//! - `response = HMAC-SHA256(key=handshake_key, data=challenge || client_pub || server_pub)`

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tracing::{debug, warn};

use super::error::{IpcError, classify_io_error};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize)]
struct ClientHello {
    protocol: String,
    version: u8,
    client_ephemeral_pub: String,
}

#[derive(Debug, Deserialize)]
struct ServerHello {
    #[expect(
        dead_code,
        reason = "deserialized by serde but consumed only for protocol validation"
    )]
    version: u8,
    server_ephemeral_pub: String,
    challenge: String,
    #[serde(default)]
    session_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChallengeResponse {
    response: String,
    preferred_cipher: String,
}

#[derive(Debug, Deserialize)]
struct HandshakeComplete {
    cipher: String,
    session_id: String,
}

#[derive(Debug, Deserialize)]
struct HandshakeError {
    #[expect(
        dead_code,
        reason = "deserialized by serde; error code reserved for future classification"
    )]
    error: String,
    reason: String,
}

/// Result of a successful BTSP Phase 1 handshake.
///
/// Carries enough state to proceed to Phase 3 cipher negotiation
/// without re-deriving the handshake key.
#[derive(Debug)]
pub struct HandshakeResult {
    /// Session ID assigned by the server.
    pub session_id: String,
    /// Cipher the server selected (typically `"null"` for Phase 1).
    pub server_cipher: String,
    /// The HKDF-derived handshake key (retained for Phase 3 key derivation).
    pub handshake_key: [u8; 32],
}

fn derive_handshake_key(family_seed: &[u8]) -> Result<[u8; 32], IpcError> {
    let hk = Hkdf::<Sha256>::new(Some(b"btsp-v1"), family_seed);
    let mut okm = [0u8; 32];
    hk.expand(b"handshake", &mut okm)
        .map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP HKDF derivation failed: {e}"),
        })?;
    Ok(okm)
}

fn compute_challenge_hmac(
    handshake_key: &[u8; 32],
    challenge: &[u8],
    client_pub: &[u8],
    server_pub: &[u8],
) -> Result<[u8; 32], IpcError> {
    use hmac::KeyInit;
    let mut mac =
        HmacSha256::new_from_slice(handshake_key).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP HMAC init: {e}"),
        })?;
    mac.update(challenge);
    mac.update(client_pub);
    mac.update(server_pub);
    Ok(mac.finalize().into_bytes().into())
}

/// Perform the BTSP client handshake on an already-connected Unix stream.
///
/// On success the stream is ready for JSON-RPC (if `cipher = "null"`) or
/// for Phase 3 cipher negotiation (if upgrading to encrypted channel).
///
/// Returns a [`HandshakeResult`] carrying the session ID, server-selected
/// cipher, and handshake key material needed for Phase 3 key derivation.
///
/// # Errors
///
/// Returns `IpcError` on I/O failure, protocol mismatch, or HMAC computation
/// error.
pub fn client_handshake(
    stream: &mut UnixStream,
    family_seed: &[u8],
) -> Result<HandshakeResult, IpcError> {
    let handshake_key = derive_handshake_key(family_seed)?;

    // Generate 32 random bytes as the client ephemeral "public key".
    // For `null` cipher negotiation, only the raw bytes matter as HMAC input;
    // real X25519 DH is only needed when negotiating encrypted session keys.
    let mut client_pub_bytes = [0u8; 32];
    getrandom::fill(&mut client_pub_bytes).map_err(|e| IpcError::ProtocolError {
        detail: format!("BTSP random generation failed: {e}"),
    })?;

    // Step 1: send ClientHello
    let hello = ClientHello {
        protocol: "btsp".to_owned(),
        version: 1,
        client_ephemeral_pub: BASE64.encode(client_pub_bytes),
    };
    let mut hello_line =
        serde_json::to_string(&hello).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?;
    hello_line.push('\n');
    stream
        .write_all(hello_line.as_bytes())
        .map_err(classify_io_error)?;

    debug!("BTSP: ClientHello sent");

    // Step 2: read ServerHello
    let mut reader = BufReader::new(stream.try_clone().map_err(classify_io_error)?);
    let mut server_hello_line = String::new();
    reader
        .read_line(&mut server_hello_line)
        .map_err(classify_io_error)?;

    if server_hello_line.is_empty() {
        return Err(IpcError::ProtocolError {
            detail: "BTSP: server closed connection (no ServerHello)".to_owned(),
        });
    }

    // Try parsing as HandshakeError first
    if let Ok(err) = serde_json::from_str::<HandshakeError>(server_hello_line.trim()) {
        return Err(IpcError::ProtocolError {
            detail: format!("BTSP handshake rejected: {}", err.reason),
        });
    }

    let server_hello: ServerHello =
        serde_json::from_str(server_hello_line.trim()).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP ServerHello parse: {e}"),
        })?;

    debug!(session_id = ?server_hello.session_id, "BTSP: ServerHello received");

    // Decode server's ephemeral public key and challenge
    let server_pub_bytes = BASE64
        .decode(&server_hello.server_ephemeral_pub)
        .map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP server_ephemeral_pub decode: {e}"),
        })?;
    let challenge_bytes =
        BASE64
            .decode(&server_hello.challenge)
            .map_err(|e| IpcError::ProtocolError {
                detail: format!("BTSP challenge decode: {e}"),
            })?;

    // Step 3: compute HMAC response and send ChallengeResponse
    let response_hmac = compute_challenge_hmac(
        &handshake_key,
        &challenge_bytes,
        &client_pub_bytes,
        &server_pub_bytes,
    )?;

    let challenge_resp = ChallengeResponse {
        response: BASE64.encode(response_hmac),
        preferred_cipher: "null".to_owned(),
    };
    let mut resp_line =
        serde_json::to_string(&challenge_resp).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?;
    resp_line.push('\n');
    stream
        .write_all(resp_line.as_bytes())
        .map_err(classify_io_error)?;

    debug!("BTSP: ChallengeResponse sent");

    // Step 4: read HandshakeComplete
    let mut complete_line = String::new();
    reader
        .read_line(&mut complete_line)
        .map_err(classify_io_error)?;

    if complete_line.is_empty() {
        return Err(IpcError::ProtocolError {
            detail: "BTSP: server closed connection (no HandshakeComplete)".to_owned(),
        });
    }

    // Could be HandshakeError if verification failed
    if let Ok(err) = serde_json::from_str::<HandshakeError>(complete_line.trim()) {
        return Err(IpcError::ProtocolError {
            detail: format!("BTSP verification failed: {}", err.reason),
        });
    }

    let complete: HandshakeComplete =
        serde_json::from_str(complete_line.trim()).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP HandshakeComplete parse: {e}"),
        })?;

    debug!(
        session_id = %complete.session_id,
        cipher = %complete.cipher,
        "BTSP: handshake complete"
    );
    Ok(HandshakeResult {
        session_id: complete.session_id,
        server_cipher: complete.cipher,
        handshake_key,
    })
}

/// Negotiate a Phase 3 encrypted cipher on an already-authenticated stream.
///
/// After `client_handshake` returns a [`HandshakeResult`], this function
/// sends a `btsp.negotiate` JSON-RPC request offering the client's preferred
/// ciphers. The server selects a cipher and returns a nonce. Both sides then
/// derive [`SessionKeys`](crate::btsp::phase3::SessionKeys) via HKDF.
///
/// If the server does not support Phase 3 or rejects negotiation, this returns
/// `Ok(None)` and the stream continues in plaintext (NULL cipher fallback).
///
/// # Errors
///
/// Returns [`IpcError`] on I/O or serialization failure.
pub fn negotiate_phase3(
    stream: &mut UnixStream,
    handshake: &HandshakeResult,
) -> Result<Option<crate::btsp::phase3::SessionKeys>, IpcError> {
    use crate::btsp::phase3::{NegotiateResponse, Phase3Cipher, SessionKeys, generate_nonce};

    let client_nonce = generate_nonce()?;
    let client_nonce_b64 = BASE64.encode(client_nonce);

    let negotiate_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "btsp.negotiate",
        "params": {
            "session_id": handshake.session_id,
            "ciphers": [Phase3Cipher::ChaCha20Poly1305.wire_name()],
            "client_nonce": client_nonce_b64,
        },
        "id": 1
    });

    let mut line =
        serde_json::to_string(&negotiate_req).map_err(|e| IpcError::SerializationError {
            detail: e.to_string(),
        })?;
    line.push('\n');
    stream
        .write_all(line.as_bytes())
        .map_err(classify_io_error)?;

    debug!("BTSP Phase 3: negotiate request sent");

    let mut reader = BufReader::new(stream.try_clone().map_err(classify_io_error)?);
    let mut resp_line = String::new();
    reader
        .read_line(&mut resp_line)
        .map_err(classify_io_error)?;

    if resp_line.is_empty() {
        debug!("BTSP Phase 3: server closed connection during negotiate — falling back to null");
        return Ok(None);
    }

    let resp: serde_json::Value =
        serde_json::from_str(resp_line.trim()).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP Phase 3 negotiate response parse: {e}"),
        })?;

    if resp.get("error").is_some() {
        let msg = resp["error"]["message"]
            .as_str()
            .or_else(|| resp["error"].as_str())
            .unwrap_or("unknown");
        debug!(
            reason = msg,
            "BTSP Phase 3: server rejected negotiate — falling back to null"
        );
        return Ok(None);
    }

    let result = resp.get("result").ok_or_else(|| IpcError::ProtocolError {
        detail: "BTSP Phase 3: negotiate response missing 'result'".to_owned(),
    })?;

    let negotiate_resp: NegotiateResponse =
        serde_json::from_value(result.clone()).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP Phase 3 NegotiateResponse parse: {e}"),
        })?;

    if negotiate_resp.cipher == Phase3Cipher::Null {
        debug!("BTSP Phase 3: server selected null cipher — staying plaintext");
        return Ok(None);
    }

    let server_nonce =
        BASE64
            .decode(&negotiate_resp.server_nonce)
            .map_err(|e| IpcError::ProtocolError {
                detail: format!("BTSP Phase 3 server_nonce decode: {e}"),
            })?;

    let keys = SessionKeys::derive(&handshake.handshake_key, &client_nonce, &server_nonce, true)?;

    debug!(
        cipher = negotiate_resp.cipher.wire_name(),
        "BTSP Phase 3: encrypted channel established"
    );
    Ok(Some(keys))
}

/// Read the family seed from `FAMILY_SEED` environment variable (base64-encoded).
///
/// Returns `None` if the variable is not set.
///
/// # Deprecation
///
/// This function reads the legacy flat `FAMILY_SEED` which predates the
/// three-tier genetics model. Use [`mito_beacon_from_env`] for new code,
/// which wraps the same environment variable into a proper
/// [`MitoBeacon`](crate::genetics::MitoBeacon) at the discovery tier.
///
/// The current BTSP handshake is mito-tier (Phase 1). Nuclear escalation
/// (Phase 2) will use [`crate::genetics::rpc::derive_lineage_key`] to
/// spawn a child generation within the mito tunnel.
#[deprecated(
    since = "0.10.0",
    note = "Use mito_beacon_from_env() for the genetics-aware path. \
            FAMILY_SEED is being transitioned to the mito-beacon tier."
)]
#[must_use]
pub fn family_seed_from_env() -> Option<Vec<u8>> {
    raw_family_seed_from_env()
}

/// Internal: read `FAMILY_SEED` bytes from the environment.
///
/// Shared implementation between the deprecated [`family_seed_from_env`]
/// and the new [`mito_beacon_from_env`].
///
/// Supports two encodings (auto-detected):
/// - **Hex** (64 ASCII hex chars → 32 bytes): produced by `AtomicHarness`
///   and consumed by `BearDog` as raw UTF-8 bytes.
/// - **Base64**: legacy encoding from earlier BTSP drafts.
///
/// If the value is valid hex (even length, all hex digits), it is used as
/// **raw UTF-8 bytes** (not hex-decoded) for wire compatibility with
/// `BearDog`, which reads `FAMILY_SEED` as raw bytes. If it's not valid hex,
/// base64 decoding is attempted as fallback.
fn raw_family_seed_from_env() -> Option<Vec<u8>> {
    let value = std::env::var("FAMILY_SEED").ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        warn!("FAMILY_SEED is empty");
        return None;
    }

    if is_hex_string(trimmed) {
        debug!("FAMILY_SEED: using as raw UTF-8 bytes (hex-encoded seed)");
        return Some(trimmed.as_bytes().to_vec());
    }

    match BASE64.decode(trimmed) {
        Ok(seed) if !seed.is_empty() => {
            debug!("FAMILY_SEED: decoded from base64 ({} bytes)", seed.len());
            Some(seed)
        }
        Ok(_) => {
            warn!("FAMILY_SEED is empty after base64 decode");
            None
        }
        Err(_) => {
            debug!("FAMILY_SEED: using as raw UTF-8 bytes (not hex, not base64)");
            Some(trimmed.as_bytes().to_vec())
        }
    }
}

/// Check whether a string is a plausible hex-encoded seed.
///
/// Returns `true` if even length, at least 32 chars, and all ASCII hex digits.
fn is_hex_string(s: &str) -> bool {
    s.len() >= 32 && s.len().is_multiple_of(2) && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Read the family seed from `FAMILY_SEED` and wrap it as a mito-beacon.
///
/// This is the genetics-aware replacement for [`family_seed_from_env`].
/// The legacy `FAMILY_SEED` environment variable is interpreted as the
/// mito-beacon tier key material. The `FAMILY_ID` env var (if set) is
/// used as the beacon group name.
///
/// # Two-Phase Intent
///
/// The BTSP handshake that uses this beacon key is **Phase 1** (mito-tier):
/// it proves group membership and establishes a tunnel. Within that tunnel,
/// **Phase 2** (nuclear) spawns a fresh generation of nuclear genetics for
/// the actual session (permissions, auth, secure data).
///
/// Returns `None` if `FAMILY_SEED` is not set or cannot be decoded.
#[must_use]
pub fn mito_beacon_from_env() -> Option<crate::genetics::MitoBeacon> {
    let seed = raw_family_seed_from_env()?;
    let family_id = std::env::var("FAMILY_ID").unwrap_or_else(|_| "unknown".to_owned());
    Some(crate::genetics::MitoBeacon::new(
        family_id.clone(),
        family_id,
        seed,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_key_deterministic() {
        let seed = b"test-family-seed-32-bytes-long!!";
        let k1 = derive_handshake_key(seed).expect("k1");
        let k2 = derive_handshake_key(seed).expect("k2");
        assert_eq!(k1, k2);
    }

    #[test]
    fn different_seeds_different_keys() {
        let k1 = derive_handshake_key(b"seed-alpha").expect("alpha");
        let k2 = derive_handshake_key(b"seed-bravo").expect("bravo");
        assert_ne!(k1, k2);
    }

    #[test]
    fn hmac_roundtrip() {
        let hk = [0xAA; 32];
        let challenge = b"random-challenge-bytes";
        let client_pub = b"client-pub-placeholder-32-bytes!";
        let server_pub = b"server-pub-placeholder-32-bytes!";

        let mac = compute_challenge_hmac(&hk, challenge, client_pub, server_pub).expect("compute");
        assert_eq!(mac.len(), 32);

        // Same inputs produce same output
        let mac2 =
            compute_challenge_hmac(&hk, challenge, client_pub, server_pub).expect("compute2");
        assert_eq!(mac, mac2);
    }

    #[test]
    fn hmac_different_inputs_different_output() {
        let hk = [0xAA; 32];
        let mac1 = compute_challenge_hmac(&hk, b"challenge1", b"cpub", b"spub").expect("m1");
        let mac2 = compute_challenge_hmac(&hk, b"challenge2", b"cpub", b"spub").expect("m2");
        assert_ne!(mac1, mac2);
    }

    #[test]
    fn family_seed_from_env_none_when_unset() {
        let _ = raw_family_seed_from_env();
    }

    #[test]
    fn mito_beacon_from_env_none_when_unset() {
        let _ = mito_beacon_from_env();
    }

    #[test]
    fn is_hex_string_accepts_valid_hex() {
        let hex64 = "a1b2c3d4e5f6a7b8a1b2c3d4e5f6a7b8a1b2c3d4e5f6a7b8a1b2c3d4e5f6a7b8";
        assert!(is_hex_string(hex64));
    }

    #[test]
    fn is_hex_string_rejects_short() {
        assert!(!is_hex_string("abcd"));
    }

    #[test]
    fn is_hex_string_rejects_odd_length() {
        assert!(!is_hex_string(&"a".repeat(33)));
    }

    #[test]
    fn is_hex_string_rejects_non_hex() {
        assert!(!is_hex_string(&"zz".repeat(32)));
    }

    #[test]
    fn is_hex_string_accepts_uppercase() {
        let hex = "A1B2C3D4E5F6A7B8A1B2C3D4E5F6A7B8";
        assert!(is_hex_string(hex));
    }
}
