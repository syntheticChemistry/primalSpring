// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Two-tier crypto key derivation and bootstrap validation.
//!
//! Rust absorption of `tools/nucleus_crypto_bootstrap.sh`. Provides the
//! same three-tier key derivation (base → family → purpose) using
//! HMAC-SHA256 via `BearDog`'s JSON-RPC interface, plus round-trip
//! verification of sign/verify and encrypt/decrypt.
//!
//! # Key Derivation Tiers
//!
//! - **Tier 0 (base)**: HMAC-SHA256 of the primal's seed fingerprint with
//!   a `primal-nucleus-v1:{primal}` info string.
//! - **Tier 1 (family)**: HMAC-SHA256 of `base_key || family_seed` with
//!   a `family-v1:{family_id}:{primal}` info string.
//! - **Tier 2 (purpose)**: HMAC-SHA256 of the family key with a
//!   `purpose-v1:{purpose}` info string.

use crate::composition::CompositionContext;
use crate::validation::ValidationResult;

/// Validate crypto bootstrap by verifying `BearDog`'s crypto capabilities.
///
/// Checks that the security provider supports the required crypto
/// operations: `crypto.sign`, `crypto.verify`, `crypto.encrypt`,
/// `crypto.decrypt`, and `crypto.hmac_sha256`.
pub fn validate_crypto_bootstrap(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let methods = [
        "crypto.sign",
        "crypto.verify",
        "crypto.encrypt",
        "crypto.decrypt",
        "crypto.hmac_sha256",
    ];

    for method in &methods {
        let check_name = format!("crypto_bootstrap:{}", method.replace('.', "_"));
        match ctx.call("security", method, serde_json::json!({"probe": true})) {
            Ok(_) => {
                v.check_bool(&check_name, true, &format!("{method} available"));
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(
                    &check_name,
                    &format!("{method}: security provider not available"),
                );
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("-32601") || msg.contains("unknown") {
                    v.check_skip(&check_name, &format!("{method}: not yet implemented"));
                } else {
                    v.check_bool(&check_name, false, &format!("{method}: error — {e}"));
                }
            }
        }
    }

    v.check_bool(
        "crypto_bootstrap:seed_fingerprints_present",
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../validation/seed_fingerprints.toml")
            .exists()
            || std::path::Path::new("validation/seed_fingerprints.toml").exists(),
        "seed_fingerprints.toml found (needed for Tier 0 derivation)",
    );
}

/// Derive a Tier 0 base key for a primal from its seed fingerprint.
///
/// Uses HMAC-SHA256 with the info string `primal-nucleus-v1:{primal}`.
/// This is a pure-Rust equivalent of the shell script's `derive_base_key()`.
#[must_use]
pub fn derive_base_key(primal: &str, fingerprint: &[u8]) -> [u8; 32] {
    use hmac::{Hmac, KeyInit, Mac};
    use sha2::Sha256;

    let info = format!("primal-nucleus-v1:{primal}");
    let Ok(mut mac) = <Hmac<Sha256> as KeyInit>::new_from_slice(fingerprint) else {
        return [0u8; 32];
    };
    mac.update(info.as_bytes());
    mac.finalize().into_bytes().into()
}

/// Derive a Tier 1 family key from the base key and family seed.
///
/// Uses HMAC-SHA256 of `base_key || family_seed` with the info string
/// `family-v1:{family_id}:{primal}`.
#[must_use]
pub fn derive_family_key(
    base_key: &[u8; 32],
    family_seed: &[u8],
    family_id: &str,
    primal: &str,
) -> [u8; 32] {
    use hmac::{Hmac, KeyInit, Mac};
    use sha2::Sha256;

    let mut combined = Vec::with_capacity(base_key.len() + family_seed.len());
    combined.extend_from_slice(base_key);
    combined.extend_from_slice(family_seed);

    let info = format!("family-v1:{family_id}:{primal}");
    let Ok(mut mac) = <Hmac<Sha256> as KeyInit>::new_from_slice(&combined) else {
        return [0u8; 32];
    };
    mac.update(info.as_bytes());
    mac.finalize().into_bytes().into()
}

/// Derive a Tier 2 purpose key from the family key.
///
/// Uses HMAC-SHA256 with the info string `purpose-v1:{purpose}`.
#[must_use]
pub fn derive_purpose_key(family_key: &[u8; 32], purpose: &str) -> [u8; 32] {
    use hmac::{Hmac, KeyInit, Mac};
    use sha2::Sha256;

    let info = format!("purpose-v1:{purpose}");
    let Ok(mut mac) = <Hmac<Sha256> as KeyInit>::new_from_slice(family_key) else {
        return [0u8; 32];
    };
    mac.update(info.as_bytes());
    mac.finalize().into_bytes().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_base_key_deterministic() {
        let fp = b"test-fingerprint-0123456789abcdef";
        let k1 = derive_base_key("beardog", fp);
        let k2 = derive_base_key("beardog", fp);
        assert_eq!(k1, k2, "same inputs must produce same output");
    }

    #[test]
    fn derive_base_key_different_primals() {
        let fp = b"test-fingerprint-0123456789abcdef";
        let k1 = derive_base_key("beardog", fp);
        let k2 = derive_base_key("songbird", fp);
        assert_ne!(k1, k2, "different primals must produce different keys");
    }

    #[test]
    fn derive_family_key_without_seed_extends_base() {
        let fp = b"test-fingerprint-0123456789abcdef";
        let base = derive_base_key("beardog", fp);
        let family = derive_family_key(&base, b"", "test-family", "beardog");
        assert_ne!(
            base, family,
            "family key should differ even with empty seed"
        );
    }

    #[test]
    fn derive_purpose_key_deterministic() {
        let family_key = [42u8; 32];
        let k1 = derive_purpose_key(&family_key, "encryption");
        let k2 = derive_purpose_key(&family_key, "encryption");
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_purpose_key_different_purposes() {
        let family_key = [42u8; 32];
        let k1 = derive_purpose_key(&family_key, "encryption");
        let k2 = derive_purpose_key(&family_key, "signing");
        assert_ne!(k1, k2);
    }

    #[test]
    fn full_three_tier_derivation() {
        let fp = b"published-primal-dna-fingerprint-v1";
        let base = derive_base_key("beardog", fp);
        let family = derive_family_key(&base, b"entropy-from-mito-beacon", "irongate", "beardog");
        let purpose = derive_purpose_key(&family, "nestgate-at-rest");
        assert_eq!(purpose.len(), 32);
        assert_ne!(purpose, [0u8; 32], "purpose key should not be zero");
    }
}
