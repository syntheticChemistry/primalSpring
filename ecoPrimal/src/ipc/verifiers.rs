// SPDX-License-Identifier: AGPL-3.0-or-later

//! Token verification implementations for the method gate.
//!
//! [`PermissiveVerifier`] — always accepts (permissive / test deployments).
//! [`SecurityVerifier`] — discovers the security capability provider via IPC.
//! [`DenyVerifier`] — rejects all tokens (enforced mode when provider is down).

/// Result of verifying an ionic capability token.
#[derive(Debug, Clone)]
pub struct VerifiedToken {
    /// Scope patterns the token grants (e.g. `["stats.*", "tensor.*"]`).
    pub scopes: Vec<String>,
    /// Subject / principal identifier from the token.
    pub subject: Option<String>,
    /// Seconds until expiry (`None` if the verifier doesn't report it).
    pub expires_in: Option<u64>,
}

/// Abstraction over ionic token verification.
///
/// The security capability provider owns cryptographic verification
/// (`auth.verify_ionic`). primalSpring defines the contract so it can
/// validate the pattern regardless of which primal serves security.
pub trait TokenVerifier: std::fmt::Debug + Send + Sync {
    /// Verify a bearer token string and return its claims.
    ///
    /// Returns `None` if the token is invalid, expired, or tampered.
    fn verify(&self, token: &str) -> Option<VerifiedToken>;
}

/// Always-accept verifier for permissive mode and tests.
///
/// Returns wildcard scopes for any token (including empty). This is the
/// correct behavior when `EnforcementMode::Permissive` or `Off` — the gate
/// logs but does not block. In `Enforced` mode, use [`SecurityVerifier`].
#[derive(Debug)]
pub struct PermissiveVerifier;

impl TokenVerifier for PermissiveVerifier {
    fn verify(&self, _token: &str) -> Option<VerifiedToken> {
        Some(VerifiedToken {
            scopes: vec!["*".to_owned()],
            subject: None,
            expires_in: None,
        })
    }
}

/// Verifier that delegates to the security capability provider via IPC.
///
/// Discovers the security provider at runtime through capability-based
/// discovery only — no hardcoded primal name fallback. When the security
/// provider is unreachable, verification fails (returns `None`), making
/// enforced mode deny-by-default.
#[derive(Debug)]
pub struct SecurityVerifier {
    security_socket: std::path::PathBuf,
}

impl SecurityVerifier {
    /// Create a verifier targeting a specific socket path.
    #[must_use]
    pub const fn new(security_socket: std::path::PathBuf) -> Self {
        Self { security_socket }
    }

    /// Discover the security capability provider at runtime.
    ///
    /// Uses only capability-based discovery (`"security"` domain). No
    /// hardcoded primal names — the provider is whoever advertises the
    /// `security` capability on this host.
    #[must_use]
    pub fn discover() -> Option<Self> {
        let disc = crate::ipc::discover::discover_by_capability("security");
        disc.socket.map(Self::new)
    }
}

impl TokenVerifier for SecurityVerifier {
    fn verify(&self, token: &str) -> Option<VerifiedToken> {
        let label = "security";
        let mut client =
            crate::ipc::client::PrimalClient::connect(&self.security_socket, label).ok()?;
        let response = client
            .call("auth.verify_ionic", serde_json::json!({ "token": token }))
            .ok()?;
        let result = response.result?;

        if !result
            .get("valid")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            return None;
        }

        let scopes = result
            .get("scopes")
            .and_then(serde_json::Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .filter(|s| !s.is_empty())?;

        let subject = result
            .get("subject")
            .and_then(serde_json::Value::as_str)
            .map(String::from);

        let expires_in = result.get("expires_in").and_then(serde_json::Value::as_u64);

        Some(VerifiedToken {
            scopes,
            subject,
            expires_in,
        })
    }
}

/// Deny-all verifier: rejects every token. Used as enforced-mode fallback
/// when the security capability provider is unreachable — ensures the gate
/// defaults to deny rather than silently accepting.
#[derive(Debug)]
pub struct DenyVerifier;

impl TokenVerifier for DenyVerifier {
    fn verify(&self, _token: &str) -> Option<VerifiedToken> {
        None
    }
}

/// Parse a `VerifiedToken` from bearDog's `auth.verify_ionic` JSON response.
///
/// bearDog w131+ returns scopes as a top-level array in the result:
/// ```json
/// { "valid": true, "scopes": ["crypto.*", "security.*"], "subject": "east-gate", "expires_in": 3600 }
/// ```
///
/// Returns `None` if the response indicates an invalid token or is malformed.
#[must_use]
pub fn parse_verify_ionic_response(result: &serde_json::Value) -> Option<VerifiedToken> {
    if !result
        .get("valid")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        return None;
    }

    let scopes = result
        .get("scopes")
        .and_then(serde_json::Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(serde_json::Value::as_str)
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .filter(|s| !s.is_empty())?;

    let subject = result
        .get("subject")
        .and_then(serde_json::Value::as_str)
        .map(String::from);

    let expires_in = result.get("expires_in").and_then(serde_json::Value::as_u64);

    Some(VerifiedToken {
        scopes,
        subject,
        expires_in,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissive_verifier_always_accepts() {
        let v = PermissiveVerifier;
        let result = v.verify("anything");
        assert!(result.is_some());
        assert_eq!(result.unwrap().scopes, vec!["*"]);
    }

    #[test]
    fn deny_verifier_always_rejects() {
        let v = DenyVerifier;
        assert!(v.verify("anything").is_none());
    }

    #[test]
    fn parse_beardog_w131_valid_response() {
        let response = serde_json::json!({
            "valid": true,
            "scopes": ["crypto.*", "security.*"],
            "subject": "east-gate",
            "expires_in": 3600
        });
        let token = parse_verify_ionic_response(&response).unwrap();
        assert_eq!(token.scopes, vec!["crypto.*", "security.*"]);
        assert_eq!(token.subject.as_deref(), Some("east-gate"));
        assert_eq!(token.expires_in, Some(3600));
    }

    #[test]
    fn parse_beardog_invalid_token_response() {
        let response = serde_json::json!({
            "valid": false,
            "error": "token_expired"
        });
        assert!(parse_verify_ionic_response(&response).is_none());
    }

    #[test]
    fn parse_beardog_missing_scopes_rejects() {
        let response = serde_json::json!({
            "valid": true,
            "subject": "east-gate"
        });
        assert!(parse_verify_ionic_response(&response).is_none());
    }

    #[test]
    fn parse_beardog_empty_scopes_rejects() {
        let response = serde_json::json!({
            "valid": true,
            "scopes": [],
            "subject": "east-gate"
        });
        assert!(parse_verify_ionic_response(&response).is_none());
    }

    #[test]
    fn parse_beardog_wildcard_scope() {
        let response = serde_json::json!({
            "valid": true,
            "scopes": ["*"],
            "subject": "admin"
        });
        let token = parse_verify_ionic_response(&response).unwrap();
        assert_eq!(token.scopes, vec!["*"]);
    }
}
