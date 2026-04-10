// SPDX-License-Identifier: AGPL-3.0-or-later

//! BearDog Secure Tunnel Protocol (BTSP) — types and validation.
//!
//! BTSP is the local IPC encryption protocol for Unix domain sockets.
//! Every production connection authenticates via BTSP handshake first;
//! plaintext is a negotiated privilege after secure nucleation, never a default.
//!
//! This module provides the canonical type definitions used by primalSpring
//! validation, biomeOS routing, and primal socket listeners. The actual
//! crypto implementation lives in BearDog (`btsp.session.*` methods).

use serde::{Deserialize, Serialize};

use crate::bonding::BondType;

/// BTSP cipher suite negotiated post-authentication.
///
/// After the BTSP handshake proves family membership, both parties negotiate
/// a cipher suite. The negotiation itself is authenticated — you cannot forge
/// a downgrade request without the family seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BtspCipherSuite {
    /// Encrypted + authenticated. Default. Zero-knowledge.
    /// Uses ChaCha20-Poly1305 with HKDF-SHA256 derived session keys.
    ChaCha20Poly1305,

    /// Authenticated but not encrypted. Integrity without confidentiality.
    /// HMAC-SHA256 tag on every frame. Useful for high-throughput
    /// same-machine workloads where the OS is trusted but tamper detection
    /// is desired.
    HmacPlain,

    /// Raw plaintext frames. Session is still authenticated (family membership
    /// proven during handshake). Both parties must explicitly opt in AND the
    /// BondingPolicy must allow it. Same length-prefix framing — just unencrypted.
    Null,
}

impl BtspCipherSuite {
    /// Human-readable description.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::ChaCha20Poly1305 => "encrypted + authenticated (ChaCha20-Poly1305)",
            Self::HmacPlain => "authenticated plaintext (HMAC-SHA256 integrity)",
            Self::Null => "raw plaintext (authenticated session, no encryption)",
        }
    }

    /// Whether this suite provides confidentiality (payload encryption).
    #[must_use]
    pub const fn is_encrypted(self) -> bool {
        matches!(self, Self::ChaCha20Poly1305)
    }

    /// Whether this suite provides integrity verification.
    #[must_use]
    pub const fn has_integrity(self) -> bool {
        matches!(self, Self::ChaCha20Poly1305 | Self::HmacPlain)
    }
}

/// Security mode for a primal socket — determined at startup from environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityMode {
    /// Production: `FAMILY_ID` set. BTSP handshake mandatory on all connections.
    /// Cipher negotiated post-authentication per `BondingPolicy`.
    Production,

    /// Development: `BIOMEOS_INSECURE=1`, no `FAMILY_ID`. Raw cleartext
    /// JSON-RPC, no BTSP. Loud warnings on every connection.
    Development,
}

/// Determine the security mode from environment variables.
///
/// - `FAMILY_ID` set (and not `"default"`) → `Production`
/// - `BIOMEOS_INSECURE=1` → `Development`
/// - Neither → `Development` (standalone/default mode)
/// - Both → invalid (see [`validate_insecure_guard`])
#[must_use]
pub fn security_mode_from_env() -> SecurityMode {
    let has_family = std::env::var("FAMILY_ID")
        .map(|v| !v.is_empty() && v != "default")
        .unwrap_or(false);

    if has_family {
        SecurityMode::Production
    } else {
        SecurityMode::Development
    }
}

/// Validate that `FAMILY_ID` and `BIOMEOS_INSECURE` are not both set.
///
/// # Errors
///
/// Returns a human-readable error message when both are set.
pub fn validate_insecure_guard() -> Result<(), String> {
    let has_family = std::env::var("FAMILY_ID")
        .map(|v| !v.is_empty() && v != "default")
        .unwrap_or(false);
    let insecure = std::env::var("BIOMEOS_INSECURE")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    if has_family && insecure {
        return Err("FATAL: FAMILY_ID and BIOMEOS_INSECURE=1 cannot coexist. \
             Production mode (FAMILY_ID set) requires BTSP authentication. \
             Remove BIOMEOS_INSECURE to run in production, or unset FAMILY_ID for development."
            .to_owned());
    }
    Ok(())
}

/// Minimum cipher suite allowed for a given bond type.
///
/// This is the enforcement policy: a bond's `BondingPolicy` may allow
/// negotiation down to this floor but never below it.
#[must_use]
pub const fn min_cipher_for_bond(bond: BondType) -> BtspCipherSuite {
    match bond {
        BondType::Covalent => BtspCipherSuite::Null,
        BondType::Metallic | BondType::OrganoMetalSalt => BtspCipherSuite::HmacPlain,
        BondType::Ionic | BondType::Weak => BtspCipherSuite::ChaCha20Poly1305,
    }
}

/// Check whether a requested cipher suite is allowed for a bond type.
///
/// The cipher is allowed if its security level is >= the minimum for the bond.
#[must_use]
pub const fn cipher_allowed(bond: BondType, requested: BtspCipherSuite) -> bool {
    let min = min_cipher_for_bond(bond);
    cipher_level(requested) >= cipher_level(min)
}

/// Numeric security level for ordering: higher = more secure.
const fn cipher_level(suite: BtspCipherSuite) -> u8 {
    match suite {
        BtspCipherSuite::Null => 0,
        BtspCipherSuite::HmacPlain => 1,
        BtspCipherSuite::ChaCha20Poly1305 => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bonding::BondType;

    #[test]
    fn covalent_allows_all_ciphers() {
        assert!(cipher_allowed(BondType::Covalent, BtspCipherSuite::Null));
        assert!(cipher_allowed(
            BondType::Covalent,
            BtspCipherSuite::HmacPlain
        ));
        assert!(cipher_allowed(
            BondType::Covalent,
            BtspCipherSuite::ChaCha20Poly1305
        ));
    }

    #[test]
    fn ionic_requires_encryption() {
        assert!(!cipher_allowed(BondType::Ionic, BtspCipherSuite::Null));
        assert!(!cipher_allowed(BondType::Ionic, BtspCipherSuite::HmacPlain));
        assert!(cipher_allowed(
            BondType::Ionic,
            BtspCipherSuite::ChaCha20Poly1305
        ));
    }

    #[test]
    fn weak_requires_encryption() {
        assert!(!cipher_allowed(BondType::Weak, BtspCipherSuite::Null));
        assert!(!cipher_allowed(BondType::Weak, BtspCipherSuite::HmacPlain));
        assert!(cipher_allowed(
            BondType::Weak,
            BtspCipherSuite::ChaCha20Poly1305
        ));
    }

    #[test]
    fn metallic_allows_hmac_and_above() {
        assert!(!cipher_allowed(BondType::Metallic, BtspCipherSuite::Null));
        assert!(cipher_allowed(
            BondType::Metallic,
            BtspCipherSuite::HmacPlain
        ));
        assert!(cipher_allowed(
            BondType::Metallic,
            BtspCipherSuite::ChaCha20Poly1305
        ));
    }

    #[test]
    fn organo_metal_salt_allows_hmac_and_above() {
        assert!(!cipher_allowed(
            BondType::OrganoMetalSalt,
            BtspCipherSuite::Null
        ));
        assert!(cipher_allowed(
            BondType::OrganoMetalSalt,
            BtspCipherSuite::HmacPlain
        ));
        assert!(cipher_allowed(
            BondType::OrganoMetalSalt,
            BtspCipherSuite::ChaCha20Poly1305
        ));
    }

    #[test]
    fn cipher_suite_descriptions() {
        assert!(!BtspCipherSuite::ChaCha20Poly1305.description().is_empty());
        assert!(!BtspCipherSuite::HmacPlain.description().is_empty());
        assert!(!BtspCipherSuite::Null.description().is_empty());
    }

    #[test]
    fn cipher_suite_properties() {
        assert!(BtspCipherSuite::ChaCha20Poly1305.is_encrypted());
        assert!(BtspCipherSuite::ChaCha20Poly1305.has_integrity());

        assert!(!BtspCipherSuite::HmacPlain.is_encrypted());
        assert!(BtspCipherSuite::HmacPlain.has_integrity());

        assert!(!BtspCipherSuite::Null.is_encrypted());
        assert!(!BtspCipherSuite::Null.has_integrity());
    }

    #[test]
    fn security_mode_serde_round_trip() {
        for mode in [SecurityMode::Production, SecurityMode::Development] {
            let json = serde_json::to_string(&mode).unwrap();
            let back: SecurityMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, back);
        }
    }

    #[test]
    fn cipher_suite_serde_round_trip() {
        for suite in [
            BtspCipherSuite::ChaCha20Poly1305,
            BtspCipherSuite::HmacPlain,
            BtspCipherSuite::Null,
        ] {
            let json = serde_json::to_string(&suite).unwrap();
            let back: BtspCipherSuite = serde_json::from_str(&json).unwrap();
            assert_eq!(suite, back);
        }
    }
}
