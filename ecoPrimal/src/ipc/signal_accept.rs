// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Synchronous riboCipher signal classification (transitional — pre-primordial debt).
//!
//! # Deprecation Notice (Wave 114)
//!
//! This module is pre-primordial debt. Post-primordial primalSpring validates
//! against fully deployed NUCLEUS instances from the plasmidBin depot on VPS —
//! it does not implement or provide server accept patterns.
//!
//! Primals converge independently on `sourdough_core::transport::ribocipher`.
//! This is more robust than shared code: convergence means each primal evolves
//! its own accept loop to match the sourdough-defined standard.
//!
//! Retained for `detect_signal_buffered` (used by validation probes to
//! classify responses). Scheduled for removal once all validation runs
//! against live deployed NUCLEUS exclusively.

use std::io::{self, Read};

use crate::tolerances;

/// Classified transport signal tier from connection prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalTier {
    /// `[0xEC, 0x01]` — MitoBeacon clear tier. Proceed with plaintext JSON-RPC.
    Clear,
    /// `[0xED, version]` — MitoBeacon obfuscated. Tunnel negotiation required.
    MitoObfuscated,
    /// `[0xEE, version]` — Nuclear sealed. Per-session key exchange required.
    NuclearSealed,
    /// No recognized signal prefix. The byte is the first byte of the actual
    /// payload (likely `{` for JSON-RPC or a BTSP frame header).
    Legacy(u8),
}

impl SignalTier {
    /// Whether this tier allows immediate JSON-RPC processing.
    #[must_use]
    pub const fn is_json_rpc_ready(&self) -> bool {
        matches!(self, Self::Clear | Self::Legacy(_))
    }

    /// Whether this is a riboCipher-aware client (any signal tier).
    #[must_use]
    pub const fn is_ribocipher_client(&self) -> bool {
        matches!(self, Self::Clear | Self::MitoObfuscated | Self::NuclearSealed)
    }
}

use tolerances::{RIBOCIPHER_MITO_OBFUSCATED, RIBOCIPHER_NUCLEAR_SEALED};

/// Accept and classify the transport signal from a new connection.
///
/// Reads the first byte. If it's a recognized signal tier byte, reads the
/// second byte (version) and returns the classified tier. If not recognized,
/// returns `SignalTier::Legacy` with the byte value.
///
/// # Errors
///
/// Returns `io::Error` if the stream read fails (connection reset, etc).
pub fn accept_signal<R: Read>(stream: &mut R) -> io::Result<SignalTier> {
    let mut first = [0u8; 1];
    stream.read_exact(&mut first)?;

    match first[0] {
        tolerances::RIBOCIPHER_CLEAR => {
            let mut version = [0u8; 1];
            stream.read_exact(&mut version)?;
            Ok(SignalTier::Clear)
        }
        RIBOCIPHER_MITO_OBFUSCATED => {
            let mut version = [0u8; 1];
            stream.read_exact(&mut version)?;
            Ok(SignalTier::MitoObfuscated)
        }
        RIBOCIPHER_NUCLEAR_SEALED => {
            let mut version = [0u8; 1];
            stream.read_exact(&mut version)?;
            Ok(SignalTier::NuclearSealed)
        }
        byte => Ok(SignalTier::Legacy(byte)),
    }
}

/// Accept signal with automatic legacy JSON-RPC handling.
///
/// If the connection sends a riboCipher signal, consumes it and returns
/// `(SignalTier, None)`. If it's a legacy client (first byte is not a
/// signal tier), returns `(SignalTier::Legacy(byte), Some(byte))` so the
/// caller can prepend it to the JSON-RPC parser.
///
/// This is the recommended entry point for most primals:
/// ```ignore
/// let (tier, prepend) = accept_signal_or_json(&mut stream)?;
/// if let Some(first_byte) = prepend {
///     // Legacy client — first_byte is start of JSON-RPC payload
///     let full_line = format!("{}{}", first_byte as char, read_rest_of_line(&mut stream)?);
///     handle_jsonrpc(&full_line);
/// } else {
///     // riboCipher client — stream is positioned at JSON-RPC start
///     let line = read_line(&mut stream)?;
///     handle_jsonrpc(&line);
/// }
/// ```
///
/// # Errors
///
/// Returns `io::Error` on stream read failure.
pub fn accept_signal_or_json<R: Read>(stream: &mut R) -> io::Result<(SignalTier, Option<u8>)> {
    let tier = accept_signal(stream)?;
    match tier {
        SignalTier::Legacy(byte) => Ok((tier, Some(byte))),
        _ => Ok((tier, None)),
    }
}

/// Peek-based signal detection for buffered readers.
///
/// Uses `fill_buf` to non-destructively inspect the first bytes without
/// consuming them if they're not a signal. This is the zero-copy path
/// for primals using `BufReader`.
///
/// Returns the tier and the number of bytes to `consume()` from the buffer.
#[must_use]
pub fn detect_signal_buffered(buf: &[u8]) -> (SignalTier, usize) {
    if buf.len() >= 2 {
        match buf[0] {
            tolerances::RIBOCIPHER_CLEAR => (SignalTier::Clear, 2),
            RIBOCIPHER_MITO_OBFUSCATED => (SignalTier::MitoObfuscated, 2),
            RIBOCIPHER_NUCLEAR_SEALED => (SignalTier::NuclearSealed, 2),
            byte => (SignalTier::Legacy(byte), 0),
        }
    } else if buf.len() == 1 {
        match buf[0] {
            tolerances::RIBOCIPHER_CLEAR
            | RIBOCIPHER_MITO_OBFUSCATED
            | RIBOCIPHER_NUCLEAR_SEALED => {
                (SignalTier::Legacy(buf[0]), 0)
            }
            byte => (SignalTier::Legacy(byte), 0),
        }
    } else {
        (SignalTier::Legacy(0), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn accept_clear_signal() {
        let data = [0xEC, 0x01, b'{', b'"'];
        let mut cursor = Cursor::new(&data);
        let tier = accept_signal(&mut cursor).unwrap();
        assert_eq!(tier, SignalTier::Clear);
        assert!(tier.is_json_rpc_ready());
        assert!(tier.is_ribocipher_client());
        // Stream should be positioned after the 2-byte signal
        let mut rest = Vec::new();
        cursor.read_to_end(&mut rest).unwrap();
        assert_eq!(rest, b"{\"");
    }

    #[test]
    fn accept_legacy_json() {
        let data = b"{\"jsonrpc\":\"2.0\"}";
        let mut cursor = Cursor::new(data.as_slice());
        let tier = accept_signal(&mut cursor).unwrap();
        assert_eq!(tier, SignalTier::Legacy(b'{'));
        assert!(tier.is_json_rpc_ready());
        assert!(!tier.is_ribocipher_client());
    }

    #[test]
    fn accept_mito_obfuscated() {
        let data = [0xED, 0x01, 0x00, 0x00];
        let mut cursor = Cursor::new(&data);
        let tier = accept_signal(&mut cursor).unwrap();
        assert_eq!(tier, SignalTier::MitoObfuscated);
        assert!(!tier.is_json_rpc_ready());
        assert!(tier.is_ribocipher_client());
    }

    #[test]
    fn accept_nuclear_sealed() {
        let data = [0xEE, 0x01, 0x00];
        let mut cursor = Cursor::new(&data);
        let tier = accept_signal(&mut cursor).unwrap();
        assert_eq!(tier, SignalTier::NuclearSealed);
        assert!(!tier.is_json_rpc_ready());
        assert!(tier.is_ribocipher_client());
    }

    #[test]
    fn accept_signal_or_json_ribocipher() {
        let data = [0xEC, 0x01, b'{'];
        let mut cursor = Cursor::new(&data);
        let (tier, prepend) = accept_signal_or_json(&mut cursor).unwrap();
        assert_eq!(tier, SignalTier::Clear);
        assert_eq!(prepend, None);
    }

    #[test]
    fn accept_signal_or_json_legacy() {
        let data = b"{\"method\":\"health\"}";
        let mut cursor = Cursor::new(data.as_slice());
        let (tier, prepend) = accept_signal_or_json(&mut cursor).unwrap();
        assert_eq!(tier, SignalTier::Legacy(b'{'));
        assert_eq!(prepend, Some(b'{'));
    }

    #[test]
    fn detect_buffered_clear() {
        let buf = [0xEC, 0x01, b'{'];
        let (tier, consume) = detect_signal_buffered(&buf);
        assert_eq!(tier, SignalTier::Clear);
        assert_eq!(consume, 2);
    }

    #[test]
    fn detect_buffered_legacy() {
        let buf = b"{\"jsonrpc\"";
        let (tier, consume) = detect_signal_buffered(buf);
        assert_eq!(tier, SignalTier::Legacy(b'{'));
        assert_eq!(consume, 0);
    }

    #[test]
    fn detect_buffered_btsp_frame() {
        // BTSP frames start with length prefix (not 0xEC/0xED/0xEE)
        let buf = [0x00, 0x00, 0x01, 0x00];
        let (tier, consume) = detect_signal_buffered(&buf);
        assert_eq!(tier, SignalTier::Legacy(0x00));
        assert_eq!(consume, 0);
    }
}
