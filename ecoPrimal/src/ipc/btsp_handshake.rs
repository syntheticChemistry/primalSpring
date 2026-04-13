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
//! Key derivation matches BearDog `btsp_handshake/crypto.rs`:
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
    #[expect(dead_code, reason = "deserialized by serde but consumed only for protocol validation")]
    version: u8,
    server_ephemeral_pub: String,
    challenge: String,
    session_id: String,
}

#[derive(Debug, Serialize)]
struct ChallengeResponse {
    response: String,
    preferred_cipher: String,
}

#[derive(Debug, Deserialize)]
struct HandshakeComplete {
    #[expect(dead_code, reason = "deserialized by serde; cipher selection logged but not branched on yet")]
    cipher: String,
    session_id: String,
}

#[derive(Debug, Deserialize)]
struct HandshakeError {
    #[expect(dead_code, reason = "deserialized by serde; error code reserved for future BTSP Phase 3")]
    error: String,
    reason: String,
}

fn derive_handshake_key(family_seed: &[u8]) -> Result<[u8; 32], IpcError> {
    let hk = Hkdf::<Sha256>::new(Some(b"btsp-v1"), family_seed);
    let mut okm = [0u8; 32];
    hk.expand(b"handshake", &mut okm).map_err(|e| {
        IpcError::ProtocolError {
            detail: format!("BTSP HKDF derivation failed: {e}"),
        }
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
    let mut mac = HmacSha256::new_from_slice(handshake_key).map_err(|e| {
        IpcError::ProtocolError {
            detail: format!("BTSP HMAC init: {e}"),
        }
    })?;
    mac.update(challenge);
    mac.update(client_pub);
    mac.update(server_pub);
    Ok(mac.finalize().into_bytes().into())
}

/// Perform the BTSP client handshake on an already-connected Unix stream.
///
/// On success the stream is ready for normal JSON-RPC request/response.
///
/// # Errors
///
/// Returns `IpcError` on I/O failure, protocol mismatch, or HMAC computation
/// error.
pub fn client_handshake(
    stream: &mut UnixStream,
    family_seed: &[u8],
) -> Result<String, IpcError> {
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

    let server_hello: ServerHello = serde_json::from_str(server_hello_line.trim()).map_err(
        |e| IpcError::ProtocolError {
            detail: format!("BTSP ServerHello parse: {e}"),
        },
    )?;

    debug!(session_id = %server_hello.session_id, "BTSP: ServerHello received");

    // Decode server's ephemeral public key and challenge
    let server_pub_bytes = BASE64
        .decode(&server_hello.server_ephemeral_pub)
        .map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP server_ephemeral_pub decode: {e}"),
        })?;
    let challenge_bytes = BASE64
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

    debug!(session_id = %complete.session_id, "BTSP: handshake complete");
    Ok(complete.session_id)
}

/// Read the family seed from `FAMILY_SEED` environment variable (base64-encoded).
///
/// Returns `None` if the variable is not set.
pub fn family_seed_from_env() -> Option<Vec<u8>> {
    let encoded = std::env::var("FAMILY_SEED").ok()?;
    match BASE64.decode(encoded.trim()) {
        Ok(seed) if !seed.is_empty() => Some(seed),
        Ok(_) => {
            warn!("FAMILY_SEED is empty after base64 decode");
            None
        }
        Err(e) => {
            warn!("FAMILY_SEED base64 decode failed: {e}");
            None
        }
    }
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
        // In test env, FAMILY_SEED is typically not set
        // We can't mutate env safely in parallel tests, so just check it doesn't panic
        let _ = family_seed_from_env();
    }
}
