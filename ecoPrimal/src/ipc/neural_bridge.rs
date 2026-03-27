// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bridge to biomeOS Neural API mode — zero compile-time coupling.
//!
//! biomeOS is the ecosystem's **substrate primal**: it orchestrates
//! deployments, routes capabilities, and coordinates primals. The Neural
//! API is one of its UniBin modes (`biomeos neural-api`), providing
//! graph execution and capability routing over JSON-RPC 2.0.
//!
//! This module provides a thin wrapper around [`PrimalClient`] for
//! communicating with biomeOS's neural-api mode. No special client
//! library is needed — biomeOS is just another primal speaking JSON-RPC
//! 2.0 over a Unix socket.
//!
//! Socket discovery uses the standard tiered strategy:
//! 1. `$NEURAL_API_SOCKET` env override
//! 2. `$XDG_RUNTIME_DIR/biomeos/neural-api-{family}.sock`
//! 3. `/tmp/biomeos/neural-api-{family}.sock`
//!
//! # Ecosystem Compliance
//!
//! Per `PRIMAL_IPC_PROTOCOL.md` v3.1 and `STANDARDS_AND_EXPECTATIONS.md`:
//! primals never import other primals as Rust crates. All cross-primal
//! communication is JSON-RPC 2.0 at runtime.

use std::path::{Path, PathBuf};

use super::client::PrimalClient;
use super::error::IpcError;

/// Result of a `capability.call` invocation via the Neural API.
#[derive(Debug, Clone)]
pub struct CapabilityCallResult {
    /// The JSON value returned by the capability provider.
    pub value: serde_json::Value,
}

/// Bridge to biomeOS's neural-api mode (graph orchestration + capability routing).
///
/// biomeOS is the substrate primal — the ecosystem's composition and
/// deployment orchestrator. Its `neural-api` mode exposes graph execution
/// and capability routing. This struct wraps [`PrimalClient`] with the
/// neural-api method names and socket discovery conventions. All
/// communication is JSON-RPC 2.0 over a Unix domain socket — no
/// compile-time dependency on biomeOS.
#[derive(Debug)]
pub struct NeuralBridge {
    socket_path: PathBuf,
}

impl NeuralBridge {
    /// Discover the biomeOS neural-api socket using standard tiered lookup.
    ///
    /// Returns `None` if no socket is found (biomeOS not running).
    #[must_use]
    pub fn discover() -> Option<Self> {
        Self::discover_with(None, None)
    }

    /// Discover the biomeOS neural-api with optional explicit socket path and family ID.
    ///
    /// When `socket_hint` is `Some`, that path is tried first. Otherwise
    /// the standard `{NEURAL_API_SOCKET}` → XDG → `/tmp` tiers are walked.
    #[must_use]
    pub fn discover_with(socket_hint: Option<&str>, family_hint: Option<&str>) -> Option<Self> {
        if let Some(hint) = socket_hint {
            let path = PathBuf::from(hint);
            if path.exists() {
                return Some(Self { socket_path: path });
            }
        }

        if let Ok(explicit) = std::env::var("NEURAL_API_SOCKET") {
            let path = PathBuf::from(&explicit);
            if path.exists() {
                return Some(Self { socket_path: path });
            }
        }

        let family = family_hint
            .map(String::from)
            .or_else(|| std::env::var("FAMILY_ID").ok())
            .unwrap_or_else(|| "default".to_owned());

        let socket_name = format!("neural-api-{family}.sock");

        if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
            let path = PathBuf::from(xdg).join("biomeos").join(&socket_name);
            if path.exists() {
                return Some(Self { socket_path: path });
            }
        }

        let tmp_path = std::env::temp_dir().join("biomeos").join(&socket_name);
        if tmp_path.exists() {
            return Some(Self {
                socket_path: tmp_path,
            });
        }

        None
    }

    /// The resolved socket path.
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Check whether the biomeOS neural-api is healthy.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the socket is unreachable or the health
    /// check call fails.
    pub fn health_check(&self) -> Result<bool, IpcError> {
        let mut client = PrimalClient::connect(&self.socket_path, "neural-api")?;
        client.health_check()
    }

    /// Discover what capabilities are registered for a domain.
    ///
    /// Calls `capability.discover` on biomeOS's neural-api.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn discover_capability(&self, capability: &str) -> Result<serde_json::Value, IpcError> {
        let mut client = PrimalClient::connect(&self.socket_path, "neural-api")?;
        let params = serde_json::json!({ "capability": capability });
        let resp = client.call("capability.discover", params)?;
        if let Some(err) = resp.error {
            return Err(IpcError::from(err));
        }
        Ok(resp.result.unwrap_or(serde_json::Value::Null))
    }

    /// Invoke a semantic capability via the biomeOS capability router.
    ///
    /// biomeOS resolves `{capability}.{operation}` to the correct provider
    /// primal, translates the method, and returns the result.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on transport or application failure.
    pub fn capability_call(
        &self,
        capability: &str,
        operation: &str,
        args: &serde_json::Value,
    ) -> Result<CapabilityCallResult, IpcError> {
        let mut client = PrimalClient::connect(&self.socket_path, "neural-api")?;
        let params = serde_json::json!({
            "capability": capability,
            "operation": operation,
            "args": args,
        });
        let resp = client.call("capability.call", params)?;
        if let Some(err) = resp.error {
            return Err(IpcError::from(err));
        }
        Ok(CapabilityCallResult {
            value: resp.result.unwrap_or(serde_json::Value::Null),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_returns_none_when_no_biomeos() {
        assert!(NeuralBridge::discover().is_none());
    }

    #[test]
    fn discover_with_bad_hint_falls_through() {
        let bridge = NeuralBridge::discover_with(Some("/nonexistent/socket.sock"), None);
        assert!(bridge.is_none());
    }

    #[test]
    fn discover_with_none_hints_returns_none_when_no_sockets() {
        assert!(NeuralBridge::discover_with(None, Some("test-family")).is_none());
    }

    #[test]
    fn socket_path_accessor() {
        let bridge = NeuralBridge {
            socket_path: PathBuf::from("/tmp/test.sock"),
        };
        assert_eq!(bridge.socket_path(), Path::new("/tmp/test.sock"));
    }

    #[test]
    fn health_check_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge {
            socket_path: PathBuf::from("/nonexistent/neural-api.sock"),
        };
        assert!(bridge.health_check().is_err());
    }

    #[test]
    fn discover_capability_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge {
            socket_path: PathBuf::from("/nonexistent/neural-api.sock"),
        };
        assert!(bridge.discover_capability("crypto").is_err());
    }

    #[test]
    fn capability_call_fails_for_nonexistent_socket() {
        let bridge = NeuralBridge {
            socket_path: PathBuf::from("/nonexistent/neural-api.sock"),
        };
        let args = serde_json::json!({});
        assert!(bridge.capability_call("crypto", "sign", &args).is_err());
    }
}
