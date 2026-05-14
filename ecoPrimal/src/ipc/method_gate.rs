// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pre-dispatch capability gate for JSON-RPC methods (JH-0 + JH-11 prep).
//!
//! Every incoming RPC call passes through [`MethodGate::check`] *before*
//! reaching the dispatch table. The gate classifies methods into
//! [`MethodAccessLevel::Public`] (allowed without any token — health probes,
//! identity, capability advertisement) and [`MethodAccessLevel::Protected`]
//! (require a valid capability token once enforcement is activated).
//!
//! Two enforcement modes control behavior:
//! - **Permissive** (default): protected methods are logged but allowed,
//!   preserving backward compatibility during ecosystem rollout.
//! - **Enforced**: protected methods without a valid token are rejected
//!   with `PERMISSION_DENIED` (-32001).
//!
//! ## Token Verification (JH-11 preparation)
//!
//! The [`TokenVerifier`] trait abstracts ionic token verification. Two
//! implementations are provided:
//! - [`NoopVerifier`] — always accepts (for permissive / test deployments).
//! - [`BearDogVerifier`] — delegates to BearDog's `auth.verify_ionic` via IPC.
//!
//! The gate performs **scope validation**: a token's scope pattern (e.g.
//! `"stats.*"`) is matched against the requested method. Wildcard `"*"`
//! grants access to all methods.

use super::protocol::{JsonRpcError, error_codes};

/// Access level for a JSON-RPC method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodAccessLevel {
    /// Health probes, identity, capability advertisement — always allowed.
    Public,
    /// Requires a valid capability token when enforcement is active.
    Protected,
}

/// Methods that are always public. Matches NestGate PG-56's exempt
/// whitelist pattern: only introspection and liveness are open.
const PUBLIC_METHOD_PREFIXES: &[&str] = &["health."];

const PUBLIC_METHODS: &[&str] = &[
    "identity.get",
    "capabilities.list",
    "capability.list",
    "lifecycle.status",
    "auth.check",
    "auth.mode",
    "auth.peer_info",
];

/// Classify a method string into its access level.
#[must_use]
pub fn classify_method(method: &str) -> MethodAccessLevel {
    if PUBLIC_METHODS.contains(&method) {
        return MethodAccessLevel::Public;
    }
    for prefix in PUBLIC_METHOD_PREFIXES {
        if method.starts_with(prefix) {
            return MethodAccessLevel::Public;
        }
    }
    MethodAccessLevel::Protected
}

// ── Token verification ─────────────────────────────────────────────────

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
/// BearDog owns the cryptographic verification (`auth.verify_ionic`).
/// primalSpring defines the contract so it can validate the pattern
/// before BearDog/biomeOS fully wire cross-primal federation (JH-11).
pub trait TokenVerifier: std::fmt::Debug + Send + Sync {
    /// Verify a bearer token string and return its claims.
    ///
    /// Returns `None` if the token is invalid, expired, or tampered.
    fn verify(&self, token: &str) -> Option<VerifiedToken>;
}

/// Always-accept verifier for permissive mode and tests.
#[derive(Debug)]
pub struct NoopVerifier;

impl TokenVerifier for NoopVerifier {
    fn verify(&self, _token: &str) -> Option<VerifiedToken> {
        Some(VerifiedToken {
            scopes: vec!["*".to_owned()],
            subject: None,
            expires_in: None,
        })
    }
}

/// Verifier that delegates to BearDog's `auth.verify_ionic` via IPC.
///
/// When BearDog is unreachable, verification fails (returns `None`).
#[derive(Debug)]
pub struct BearDogVerifier {
    beardog_socket: std::path::PathBuf,
}

impl BearDogVerifier {
    /// Create a verifier targeting the given BearDog socket path.
    #[must_use]
    pub const fn new(beardog_socket: std::path::PathBuf) -> Self {
        Self { beardog_socket }
    }

    /// Attempt to create a verifier by discovering the security capability
    /// provider at runtime, falling back to the BearDog socket convention.
    #[must_use]
    pub fn discover() -> Option<Self> {
        let disc = crate::ipc::discover::discover_by_capability("security");
        if let Some(socket) = disc.socket {
            return Some(Self::new(socket));
        }
        let path = crate::ipc::discover::socket_path(crate::primal_names::BEARDOG);
        if path.exists() {
            Some(Self::new(path))
        } else {
            None
        }
    }
}

impl TokenVerifier for BearDogVerifier {
    fn verify(&self, token: &str) -> Option<VerifiedToken> {
        let label = crate::primal_names::BEARDOG;
        let mut client =
            crate::ipc::client::PrimalClient::connect(&self.beardog_socket, label).ok()?;
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
            .map_or_else(
                || vec!["*".to_owned()],
                |arr| {
                    arr.iter()
                        .filter_map(serde_json::Value::as_str)
                        .map(String::from)
                        .collect()
                },
            );

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

/// Check whether a token's scopes permit access to a method.
///
/// Scope patterns use domain-prefix matching:
/// - `"*"` matches everything
/// - `"stats.*"` matches `"stats.mean"`, `"stats.variance"`, etc.
/// - `"stats.mean"` matches only `"stats.mean"` exactly
#[must_use]
pub fn scope_permits_method(scopes: &[String], method: &str) -> bool {
    scopes.iter().any(|scope| {
        scope == "*"
            || scope == method
            || (scope.ends_with(".*")
                && method.starts_with(scope.trim_end_matches(".*"))
                && method.as_bytes().get(scope.len() - 2) == Some(&b'.'))
    })
}

// ── Peer credentials ────────────────────────────────────────────────────

/// Peer credentials extracted from `SO_PEERCRED` on Unix sockets.
///
/// Uses only the stable subset of `std::os::unix::net::UCred`:
/// `uid` (stable since 1.75) and `pid` (stable `Option<i32>`).
/// GID is deferred until `peer_credentials_unix_socket` stabilizes.
#[derive(Debug, Clone)]
pub struct PeerCredentials {
    /// Process ID of the caller (if available).
    pub pid: Option<u32>,
    /// User ID of the caller.
    pub uid: u32,
}

// ── Caller context ──────────────────────────────────────────────────────

/// Identity and authorization context for an incoming RPC call.
#[derive(Debug, Clone)]
pub struct CallerContext {
    /// Optional bearer / capability token sent in the request.
    pub bearer_token: Option<String>,
    /// Verified token claims (populated when a verifier accepts the token).
    pub verified: Option<VerifiedToken>,
    /// Peer credentials from `SO_PEERCRED` (Unix socket only).
    pub peer: Option<PeerCredentials>,
    /// Where the connection came from (`unix`, `tcp`, `loopback`).
    pub origin: ConnectionOrigin,
}

/// How the caller connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionOrigin {
    /// Local Unix domain socket.
    Unix,
    /// TCP loopback (127.0.0.1 / ::1).
    Loopback,
    /// Remote TCP connection.
    Remote,
}

impl CallerContext {
    /// Create a caller context for a Unix domain socket connection.
    ///
    /// Peer credentials (`SO_PEERCRED`) are not extracted here because
    /// `std::os::unix::net::UnixStream::peer_cred()` is still behind the
    /// unstable `peer_credentials_unix_socket` feature gate and the crate
    /// uses `#![forbid(unsafe_code)]`. Once the API stabilizes (or a safe
    /// wrapper like `rustix` is adopted), this method will populate
    /// `PeerCredentials` automatically. Until then, the gate operates
    /// on bearer tokens and connection origin.
    #[must_use]
    #[expect(
        clippy::missing_const_for_fn,
        reason = "will extract peer_cred() when the API stabilizes — not const then"
    )]
    pub fn from_unix_stream(_stream: &std::os::unix::net::UnixStream) -> Self {
        Self {
            bearer_token: None,
            verified: None,
            peer: None,
            origin: ConnectionOrigin::Unix,
        }
    }

    /// Build a caller context for loopback TCP with no peer credentials.
    #[must_use]
    pub const fn loopback() -> Self {
        Self {
            bearer_token: None,
            verified: None,
            peer: None,
            origin: ConnectionOrigin::Loopback,
        }
    }

    /// Extract `_bearer_token` from JSON-RPC params and optionally verify it.
    ///
    /// Ecosystem convention: callers attach bearer tokens as `_bearer_token`
    /// in the JSON-RPC params object (underscore prefix = transport metadata).
    #[must_use]
    pub fn with_params_token(
        mut self,
        params: &serde_json::Value,
        verifier: &dyn TokenVerifier,
    ) -> Self {
        if let Some(token) = params
            .get("_bearer_token")
            .and_then(serde_json::Value::as_str)
        {
            self.bearer_token = Some(token.to_owned());
            self.verified = verifier.verify(token);
        }
        self
    }
}

// ── Enforcement mode ────────────────────────────────────────────────────

/// Enforcement mode for the method gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementMode {
    /// Log violations but allow all calls (backward-compatible default).
    Permissive,
    /// Reject unauthenticated calls to protected methods.
    Enforced,
}

impl EnforcementMode {
    /// Resolve from `PRIMALSPRING_AUTH_MODE` env var.
    /// Defaults to `Permissive` if unset or unrecognized.
    #[must_use]
    pub fn from_env() -> Self {
        match std::env::var("PRIMALSPRING_AUTH_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "enforced" | "enforce" | "strict" => Self::Enforced,
            _ => Self::Permissive,
        }
    }

    /// Human-readable label for diagnostics and `auth.mode` responses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permissive => "permissive",
            Self::Enforced => "enforced",
        }
    }
}

// ── Method gate ─────────────────────────────────────────────────────────

/// Pre-dispatch gate that checks caller authorization before method execution.
#[derive(Debug)]
pub struct MethodGate {
    mode: EnforcementMode,
    verifier: Box<dyn TokenVerifier>,
}

impl MethodGate {
    /// Create a gate with the given enforcement mode and a noop verifier.
    #[must_use]
    pub fn new(mode: EnforcementMode) -> Self {
        Self {
            mode,
            verifier: Box::new(NoopVerifier),
        }
    }

    /// Create a gate with a specific token verifier.
    #[must_use]
    pub fn with_verifier(mode: EnforcementMode, verifier: Box<dyn TokenVerifier>) -> Self {
        Self { mode, verifier }
    }

    /// Create a gate from the environment (`PRIMALSPRING_AUTH_MODE`).
    ///
    /// In enforced mode, attempts to discover BearDog for real verification.
    /// Falls back to noop if BearDog is unreachable.
    #[must_use]
    pub fn from_env() -> Self {
        let mode = EnforcementMode::from_env();
        let verifier: Box<dyn TokenVerifier> = if mode == EnforcementMode::Enforced {
            match BearDogVerifier::discover() {
                Some(v) => Box::new(v),
                None => Box::new(NoopVerifier),
            }
        } else {
            Box::new(NoopVerifier)
        };
        Self { mode, verifier }
    }

    /// Current enforcement mode.
    #[must_use]
    pub const fn mode(&self) -> EnforcementMode {
        self.mode
    }

    /// Reference to the active token verifier.
    #[must_use]
    pub fn verifier(&self) -> &dyn TokenVerifier {
        &*self.verifier
    }

    /// Pre-dispatch authorization check with scope validation.
    ///
    /// Returns `Ok(())` if the call should proceed.
    ///
    /// # Errors
    ///
    /// Returns `JsonRpcError` with `PERMISSION_DENIED` when:
    /// - A protected method is called without a token (enforced mode), or
    /// - A token is present but its scopes don't cover the requested method.
    pub fn check(&self, method: &str, caller: &CallerContext) -> Result<(), JsonRpcError> {
        let level = classify_method(method);

        if level == MethodAccessLevel::Public {
            return Ok(());
        }

        if let Some(ref verified) = caller.verified {
            if scope_permits_method(&verified.scopes, method) {
                return Ok(());
            }
            tracing::warn!(
                method,
                scopes = ?verified.scopes,
                "method gate: token scope does not cover requested method"
            );
            return Err(JsonRpcError {
                code: error_codes::PERMISSION_DENIED,
                message: format!("permission denied: token scope does not cover '{method}'"),
                data: Some(serde_json::json!({
                    "method": method,
                    "reason": "scope_mismatch",
                })),
            });
        }

        let has_token = caller.bearer_token.is_some();
        if has_token {
            tracing::warn!(
                method,
                "method gate: bearer token present but verification failed"
            );
        }

        match self.mode {
            EnforcementMode::Permissive => {
                tracing::warn!(
                    method,
                    caller_uid = caller.peer.as_ref().map(|p| p.uid),
                    caller_pid = caller.peer.as_ref().and_then(|p| p.pid),
                    "method gate: unauthenticated call to protected method (permissive mode — allowing)"
                );
                Ok(())
            }
            EnforcementMode::Enforced => {
                tracing::warn!(
                    method,
                    caller_uid = caller.peer.as_ref().map(|p| p.uid),
                    caller_pid = caller.peer.as_ref().and_then(|p| p.pid),
                    "method gate: REJECTED unauthenticated call to protected method"
                );
                Err(JsonRpcError {
                    code: error_codes::PERMISSION_DENIED,
                    message: format!(
                        "permission denied: method '{method}' requires a capability token"
                    ),
                    data: Some(serde_json::json!({ "method": method })),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── classify_method ──

    #[test]
    fn health_methods_are_public() {
        assert_eq!(classify_method("health.check"), MethodAccessLevel::Public);
        assert_eq!(
            classify_method("health.liveness"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("health.readiness"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn identity_is_public() {
        assert_eq!(classify_method("identity.get"), MethodAccessLevel::Public);
    }

    #[test]
    fn capabilities_list_is_public() {
        assert_eq!(
            classify_method("capabilities.list"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("capability.list"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn auth_introspection_is_public() {
        assert_eq!(classify_method("auth.check"), MethodAccessLevel::Public);
        assert_eq!(classify_method("auth.mode"), MethodAccessLevel::Public);
        assert_eq!(classify_method("auth.peer_info"), MethodAccessLevel::Public);
    }

    #[test]
    fn lifecycle_status_is_public() {
        assert_eq!(
            classify_method("lifecycle.status"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn coordination_methods_are_protected() {
        assert_eq!(
            classify_method("coordination.validate_composition"),
            MethodAccessLevel::Protected
        );
        assert_eq!(
            classify_method("coordination.deploy_atomic"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn graph_methods_are_protected() {
        assert_eq!(
            classify_method("graph.validate"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn unregistered_methods_are_protected() {
        assert_eq!(
            classify_method("bonding.propose"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn empty_method_is_protected() {
        assert_eq!(classify_method(""), MethodAccessLevel::Protected);
    }

    // ── scope_permits_method ──

    #[test]
    fn wildcard_scope_permits_all() {
        assert!(scope_permits_method(&["*".to_owned()], "stats.mean"));
        assert!(scope_permits_method(&["*".to_owned()], "storage.store"));
    }

    #[test]
    fn domain_wildcard_permits_domain_methods() {
        let scopes = vec!["stats.*".to_owned()];
        assert!(scope_permits_method(&scopes, "stats.mean"));
        assert!(scope_permits_method(&scopes, "stats.variance"));
        assert!(!scope_permits_method(&scopes, "storage.store"));
        assert!(!scope_permits_method(&scopes, "statistics.mean"));
    }

    #[test]
    fn exact_scope_permits_only_exact_match() {
        let scopes = vec!["stats.mean".to_owned()];
        assert!(scope_permits_method(&scopes, "stats.mean"));
        assert!(!scope_permits_method(&scopes, "stats.variance"));
    }

    #[test]
    fn multiple_scopes_union() {
        let scopes = vec!["stats.*".to_owned(), "tensor.*".to_owned()];
        assert!(scope_permits_method(&scopes, "stats.mean"));
        assert!(scope_permits_method(&scopes, "tensor.matmul"));
        assert!(!scope_permits_method(&scopes, "storage.store"));
    }

    #[test]
    fn empty_scopes_deny_all() {
        let scopes: Vec<String> = vec![];
        assert!(!scope_permits_method(&scopes, "stats.mean"));
    }

    // ── CallerContext ──

    #[test]
    fn loopback_context_has_no_peer() {
        let ctx = CallerContext::loopback();
        assert!(ctx.peer.is_none());
        assert!(ctx.bearer_token.is_none());
        assert!(ctx.verified.is_none());
        assert_eq!(ctx.origin, ConnectionOrigin::Loopback);
    }

    #[test]
    fn with_params_token_extracts_bearer() {
        let ctx = CallerContext::loopback();
        let params = serde_json::json!({
            "_bearer_token": "test-token-123",
            "values": [1, 2, 3],
        });
        let ctx = ctx.with_params_token(&params, &NoopVerifier);
        assert_eq!(ctx.bearer_token.as_deref(), Some("test-token-123"));
        assert!(ctx.verified.is_some());
        assert_eq!(ctx.verified.as_ref().unwrap().scopes, vec!["*"]);
    }

    #[test]
    fn with_params_token_absent_leaves_none() {
        let ctx = CallerContext::loopback();
        let params = serde_json::json!({ "values": [1, 2, 3] });
        let ctx = ctx.with_params_token(&params, &NoopVerifier);
        assert!(ctx.bearer_token.is_none());
        assert!(ctx.verified.is_none());
    }

    // ── EnforcementMode ──

    #[test]
    fn enforcement_mode_as_str() {
        assert_eq!(EnforcementMode::Permissive.as_str(), "permissive");
        assert_eq!(EnforcementMode::Enforced.as_str(), "enforced");
    }

    // ── MethodGate::check ──

    #[test]
    fn public_method_always_passes() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext::loopback();
        assert!(gate.check("health.check", &caller).is_ok());
        assert!(gate.check("identity.get", &caller).is_ok());
        assert!(gate.check("capabilities.list", &caller).is_ok());
    }

    #[test]
    fn protected_method_passes_in_permissive_mode() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext::loopback();
        assert!(gate.check("coordination.deploy_atomic", &caller).is_ok());
    }

    #[test]
    fn protected_method_rejected_in_enforced_mode_without_token() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext::loopback();
        let result = gate.check("coordination.deploy_atomic", &caller);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, error_codes::PERMISSION_DENIED);
        assert!(err.message.contains("coordination.deploy_atomic"));
    }

    #[test]
    fn protected_method_passes_with_verified_token_and_matching_scope() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext {
            bearer_token: Some("valid-token".to_owned()),
            verified: Some(VerifiedToken {
                scopes: vec!["coordination.*".to_owned()],
                subject: Some("test-user".to_owned()),
                expires_in: Some(3600),
            }),
            peer: None,
            origin: ConnectionOrigin::Unix,
        };
        assert!(gate.check("coordination.deploy_atomic", &caller).is_ok());
    }

    #[test]
    fn protected_method_rejected_with_wrong_scope() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext {
            bearer_token: Some("valid-token".to_owned()),
            verified: Some(VerifiedToken {
                scopes: vec!["stats.*".to_owned()],
                subject: None,
                expires_in: None,
            }),
            peer: None,
            origin: ConnectionOrigin::Unix,
        };
        let result = gate.check("coordination.deploy_atomic", &caller);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.data
                .as_ref()
                .and_then(|d| d.get("reason"))
                .and_then(serde_json::Value::as_str)
                == Some("scope_mismatch")
        );
    }

    #[test]
    fn gate_error_includes_method_in_data() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext::loopback();
        let err = gate.check("graph.validate", &caller).unwrap_err();
        let method_in_data = err
            .data
            .as_ref()
            .and_then(|d| d.get("method"))
            .and_then(serde_json::Value::as_str);
        assert_eq!(method_in_data, Some("graph.validate"));
    }
}
