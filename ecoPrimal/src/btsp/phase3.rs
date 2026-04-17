// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP Phase 3 — Encrypted post-handshake channel.
//!
//! After Phase 1 (mito-beacon tunnel) and Phase 2 (nuclear session), Phase 3
//! establishes an encrypted transport for the JSON-RPC channel. This replaces
//! the current `cipher = "null"` plaintext fallback with real encryption.
//!
//! # Protocol Flow
//!
//! After BTSP Phase 1 handshake completes (session_id established):
//!
//! ```text
//! 1. Client → Server:  btsp.negotiate { session_id, ciphers: ["chacha20-poly1305"] }
//! 2. Server → Client:  { cipher: "chacha20-poly1305", server_nonce: <base64> }
//! 3. Both sides derive session keys via HKDF from handshake_key + nonces
//! 4. All subsequent frames are encrypted + authenticated
//! ```
//!
//! # Session Key Derivation
//!
//! ```text
//! session_key = HKDF-SHA256(
//!     ikm = handshake_key,
//!     salt = client_nonce || server_nonce,
//!     info = "btsp-session-v1"
//! )
//! ```
//!
//! # Wire Format (encrypted channel)
//!
//! Each frame is a length-prefixed encrypted blob:
//! ```text
//! [4 bytes: length (big-endian u32)] [12 bytes: nonce] [length bytes: ciphertext + tag]
//! ```

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::ipc::error::IpcError;

/// Cipher suites supported in Phase 3 negotiation.
///
/// These map 1:1 to [`super::BtspCipherSuite`] but use wire-format string names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase3Cipher {
    /// ChaCha20-Poly1305 AEAD (encrypted + authenticated).
    #[serde(rename = "chacha20-poly1305")]
    ChaCha20Poly1305,
    /// HMAC-SHA256 integrity without encryption.
    #[serde(rename = "hmac-plain")]
    HmacPlain,
    /// Plaintext (no encryption, no integrity).
    #[serde(rename = "null")]
    Null,
}

impl Phase3Cipher {
    /// The wire-format name used in JSON negotiation messages.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::ChaCha20Poly1305 => "chacha20-poly1305",
            Self::HmacPlain => "hmac-plain",
            Self::Null => "null",
        }
    }

    /// Convert to the canonical [`BtspCipherSuite`](super::BtspCipherSuite).
    #[must_use]
    pub const fn to_suite(self) -> super::BtspCipherSuite {
        match self {
            Self::ChaCha20Poly1305 => super::BtspCipherSuite::ChaCha20Poly1305,
            Self::HmacPlain => super::BtspCipherSuite::HmacPlain,
            Self::Null => super::BtspCipherSuite::Null,
        }
    }

    /// Convert from the canonical [`BtspCipherSuite`](super::BtspCipherSuite).
    #[must_use]
    pub const fn from_suite(suite: super::BtspCipherSuite) -> Self {
        match suite {
            super::BtspCipherSuite::ChaCha20Poly1305 => Self::ChaCha20Poly1305,
            super::BtspCipherSuite::HmacPlain => Self::HmacPlain,
            super::BtspCipherSuite::Null => Self::Null,
        }
    }
}

/// Client → Server: cipher negotiation request.
#[derive(Debug, Serialize)]
pub struct NegotiateRequest {
    /// Session ID from the Phase 1 handshake.
    pub session_id: String,
    /// Ciphers the client supports, ordered by preference.
    pub ciphers: Vec<Phase3Cipher>,
    /// Client-generated random nonce for session key derivation.
    pub client_nonce: String,
}

/// Server → Client: cipher negotiation response.
#[derive(Debug, Deserialize)]
pub struct NegotiateResponse {
    /// The cipher selected by the server (from the client's offered list).
    pub cipher: Phase3Cipher,
    /// Server-generated random nonce for session key derivation.
    pub server_nonce: String,
}

/// Derived session keys for the encrypted channel.
///
/// Both sides derive the same keys from the handshake key + nonces.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SessionKeys {
    encrypt_key: [u8; 32],
    decrypt_key: [u8; 32],
}

impl std::fmt::Debug for SessionKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionKeys")
            .field("encrypt_key", &"[redacted]")
            .field("decrypt_key", &"[redacted]")
            .finish()
    }
}

impl SessionKeys {
    /// Derive session keys from the Phase 1 handshake key and both nonces.
    ///
    /// The client and server derive mirrored keys: the client's encrypt key
    /// is the server's decrypt key, and vice versa.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if HKDF expansion fails (should not happen for 32-byte output).
    pub fn derive(
        handshake_key: &[u8; 32],
        client_nonce: &[u8],
        server_nonce: &[u8],
        is_client: bool,
    ) -> Result<Self, IpcError> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let mut salt = Vec::with_capacity(client_nonce.len() + server_nonce.len());
        salt.extend_from_slice(client_nonce);
        salt.extend_from_slice(server_nonce);

        let hk = Hkdf::<Sha256>::new(Some(&salt), handshake_key);

        let mut client_to_server = [0u8; 32];
        hk.expand(b"btsp-session-v1-c2s", &mut client_to_server)
            .map_err(|e| IpcError::ProtocolError {
                detail: format!("BTSP Phase 3 HKDF c2s: {e}"),
            })?;

        let mut server_to_client = [0u8; 32];
        hk.expand(b"btsp-session-v1-s2c", &mut server_to_client)
            .map_err(|e| IpcError::ProtocolError {
                detail: format!("BTSP Phase 3 HKDF s2c: {e}"),
            })?;

        if is_client {
            Ok(Self {
                encrypt_key: client_to_server,
                decrypt_key: server_to_client,
            })
        } else {
            Ok(Self {
                encrypt_key: server_to_client,
                decrypt_key: client_to_server,
            })
        }
    }

    /// Encrypt a plaintext message for transmission.
    ///
    /// Returns `nonce || ciphertext` (12 + plaintext.len() + 16 bytes).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if encryption fails or nonce generation fails.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, IpcError> {
        use chacha20poly1305::aead::{Aead, KeyInit};
        use chacha20poly1305::{ChaCha20Poly1305, Nonce};

        let cipher = ChaCha20Poly1305::new((&self.encrypt_key).into());

        let mut nonce_bytes = [0u8; 12];
        getrandom::fill(&mut nonce_bytes).map_err(|e| IpcError::ProtocolError {
            detail: format!("BTSP Phase 3 nonce generation: {e}"),
        })?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| IpcError::ProtocolError {
                detail: format!("BTSP Phase 3 encrypt: {e}"),
            })?;

        let mut frame = Vec::with_capacity(12 + ciphertext.len());
        frame.extend_from_slice(&nonce_bytes);
        frame.extend_from_slice(&ciphertext);
        Ok(frame)
    }

    /// Decrypt a received frame (`nonce || ciphertext`).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if decryption fails (tampered data, wrong key, etc.).
    pub fn decrypt(&self, frame: &[u8]) -> Result<Vec<u8>, IpcError> {
        use chacha20poly1305::aead::{Aead, KeyInit};
        use chacha20poly1305::{ChaCha20Poly1305, Nonce};

        if frame.len() < 12 + 16 {
            return Err(IpcError::ProtocolError {
                detail: format!(
                    "BTSP Phase 3 frame too short: {} bytes (need >= 28)",
                    frame.len()
                ),
            });
        }

        let (nonce_bytes, ciphertext) = frame.split_at(12);
        let cipher = ChaCha20Poly1305::new((&self.decrypt_key).into());
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| IpcError::ProtocolError {
                detail: format!("BTSP Phase 3 decrypt: {e}"),
            })
    }
}

/// Generate a random 32-byte nonce for Phase 3 negotiation.
///
/// # Errors
///
/// Returns [`IpcError`] if random generation fails.
pub fn generate_nonce() -> Result<[u8; 32], IpcError> {
    let mut nonce = [0u8; 32];
    getrandom::fill(&mut nonce).map_err(|e| IpcError::ProtocolError {
        detail: format!("BTSP Phase 3 nonce generation: {e}"),
    })?;
    Ok(nonce)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase3_cipher_wire_names() {
        assert_eq!(
            Phase3Cipher::ChaCha20Poly1305.wire_name(),
            "chacha20-poly1305"
        );
        assert_eq!(Phase3Cipher::HmacPlain.wire_name(), "hmac-plain");
        assert_eq!(Phase3Cipher::Null.wire_name(), "null");
    }

    #[test]
    fn phase3_cipher_suite_round_trip() {
        for cipher in [
            Phase3Cipher::ChaCha20Poly1305,
            Phase3Cipher::HmacPlain,
            Phase3Cipher::Null,
        ] {
            assert_eq!(Phase3Cipher::from_suite(cipher.to_suite()), cipher);
        }
    }

    #[test]
    fn phase3_cipher_serde_round_trip() {
        for cipher in [
            Phase3Cipher::ChaCha20Poly1305,
            Phase3Cipher::HmacPlain,
            Phase3Cipher::Null,
        ] {
            let json = serde_json::to_string(&cipher).unwrap();
            let back: Phase3Cipher = serde_json::from_str(&json).unwrap();
            assert_eq!(cipher, back);
        }
    }

    #[test]
    fn phase3_cipher_json_format() {
        let json = serde_json::to_string(&Phase3Cipher::ChaCha20Poly1305).unwrap();
        assert_eq!(json, "\"chacha20-poly1305\"");
    }

    #[test]
    fn session_keys_derive_deterministic() {
        let hk = [0xAA; 32];
        let cn = [0xBB; 32];
        let sn = [0xCC; 32];

        let k1 = SessionKeys::derive(&hk, &cn, &sn, true).unwrap();
        let k2 = SessionKeys::derive(&hk, &cn, &sn, true).unwrap();
        assert_eq!(k1.encrypt_key, k2.encrypt_key);
        assert_eq!(k1.decrypt_key, k2.decrypt_key);
    }

    #[test]
    fn session_keys_client_server_mirror() {
        let hk = [0xAA; 32];
        let cn = [0xBB; 32];
        let sn = [0xCC; 32];

        let client_keys = SessionKeys::derive(&hk, &cn, &sn, true).unwrap();
        let server_keys = SessionKeys::derive(&hk, &cn, &sn, false).unwrap();

        assert_eq!(
            client_keys.encrypt_key, server_keys.decrypt_key,
            "client encrypt = server decrypt"
        );
        assert_eq!(
            client_keys.decrypt_key, server_keys.encrypt_key,
            "client decrypt = server encrypt"
        );
    }

    #[test]
    fn encrypt_decrypt_round_trip() {
        let hk = [0x42; 32];
        let cn = [0x01; 32];
        let sn = [0x02; 32];

        let client = SessionKeys::derive(&hk, &cn, &sn, true).unwrap();
        let server = SessionKeys::derive(&hk, &cn, &sn, false).unwrap();

        let plaintext = b"hello from BTSP Phase 3";
        let frame = client.encrypt(plaintext).unwrap();
        let decrypted = server.decrypt(&frame).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_decrypt_large_payload() {
        let hk = [0x42; 32];
        let cn = [0x01; 32];
        let sn = [0x02; 32];

        let client = SessionKeys::derive(&hk, &cn, &sn, true).unwrap();
        let server = SessionKeys::derive(&hk, &cn, &sn, false).unwrap();

        let plaintext = vec![0xAB; 64 * 1024];
        let frame = client.encrypt(&plaintext).unwrap();
        let decrypted = server.decrypt(&frame).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn decrypt_rejects_tampered_frame() {
        let hk = [0x42; 32];
        let cn = [0x01; 32];
        let sn = [0x02; 32];

        let client = SessionKeys::derive(&hk, &cn, &sn, true).unwrap();
        let server = SessionKeys::derive(&hk, &cn, &sn, false).unwrap();

        let mut frame = client.encrypt(b"authentic data").unwrap();
        let last = frame.len() - 1;
        frame[last] ^= 0xFF;
        assert!(server.decrypt(&frame).is_err());
    }

    #[test]
    fn decrypt_rejects_short_frame() {
        let hk = [0x42; 32];
        let cn = [0x01; 32];
        let sn = [0x02; 32];
        let server = SessionKeys::derive(&hk, &cn, &sn, false).unwrap();
        assert!(server.decrypt(&[0u8; 10]).is_err());
    }

    #[test]
    fn decrypt_rejects_wrong_key() {
        let cn = [0x01; 32];
        let sn = [0x02; 32];

        let client = SessionKeys::derive(&[0xAA; 32], &cn, &sn, true).unwrap();
        let wrong_server = SessionKeys::derive(&[0xBB; 32], &cn, &sn, false).unwrap();

        let frame = client.encrypt(b"secret").unwrap();
        assert!(wrong_server.decrypt(&frame).is_err());
    }

    #[test]
    fn generate_nonce_produces_32_bytes() {
        let nonce = generate_nonce().unwrap();
        assert_eq!(nonce.len(), 32);
    }

    #[test]
    fn generate_nonce_is_random() {
        let n1 = generate_nonce().unwrap();
        let n2 = generate_nonce().unwrap();
        assert_ne!(n1, n2, "nonces should be different");
    }

    #[test]
    fn negotiate_request_serialize() {
        let req = NegotiateRequest {
            session_id: "test-session".to_owned(),
            ciphers: vec![Phase3Cipher::ChaCha20Poly1305],
            client_nonce: "YWJjZA==".to_owned(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["session_id"], "test-session");
        assert_eq!(json["ciphers"][0], "chacha20-poly1305");
    }

    #[test]
    fn negotiate_response_deserialize() {
        let json = r#"{"cipher":"chacha20-poly1305","server_nonce":"eHl6"}"#;
        let resp: NegotiateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.cipher, Phase3Cipher::ChaCha20Poly1305);
        assert_eq!(resp.server_nonce, "eHl6");
    }
}
