// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! RPC call helpers and convenience methods for [`CompositionContext`].
//!
//! Split from `context.rs` to keep both modules under the 800-line
//! ecosystem guideline. This module holds the typed call wrappers,
//! health/crypto/resolve utilities, and visualization convenience surface.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use crate::ipc::IpcError;
use super::routing::capability_to_primal;
use super::CompositionContext;

impl CompositionContext {
    /// Call a method on the provider of `capability`.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client or the call fails.
    pub fn call(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        let client = self
            .clients
            .get_mut(capability)
            .ok_or_else(|| IpcError::SocketNotFound {
                primal: format!("capability:{capability}"),
            })?;
        let response = client.call(method, params)?;
        response.result.ok_or_else(|| IpcError::ProtocolError {
            detail: response
                .error
                .as_ref()
                .map_or_else(|| "no result".to_owned(), |e| e.message.clone()),
        })
    }

    /// Call a method with the context's bearer token injected.
    ///
    /// If a bearer token is set (via [`set_bearer_token`](Self::set_bearer_token)),
    /// it is added as `_bearer_token` in the JSON-RPC params. This enables
    /// scope-checked dispatch on the receiving primal's MethodGate.
    ///
    /// Falls back to a bare call if no bearer token is set.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client or the call fails.
    pub fn call_authenticated(
        &mut self,
        capability: &str,
        method: &str,
        mut params: serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        if let Some(ref token) = self.bearer_token {
            if let Some(obj) = params.as_object_mut() {
                obj.insert("_bearer_token".to_owned(), serde_json::json!(token));
            }
        }
        self.call(capability, method, params)
    }

    /// Call a method and extract a single `f64` from the result by key.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client, the call fails,
    /// or the key is missing/not numeric.
    pub fn call_f64(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
        key: &str,
    ) -> Result<f64, IpcError> {
        self.call_f64_flex(capability, method, params, &[key])
    }

    /// Like [`call_f64`](Self::call_f64) but tries multiple candidate keys
    /// in order, returning the first that resolves to a number.
    ///
    /// Handles cross-primal response schema divergence (e.g. barraCuda
    /// returns `{"mean": N}` while the canonical key is `"result"`).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails or no key resolves to a number.
    pub fn call_f64_flex(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
        keys: &[&str],
    ) -> Result<f64, IpcError> {
        let result = self.call(capability, method, params)?;
        for key in keys {
            if let Some(v) = result.get(*key).and_then(serde_json::Value::as_f64) {
                return Ok(v);
            }
        }
        Err(IpcError::SerializationError {
            detail: format!("none of keys {keys:?} found as number in {result}"),
        })
    }

    /// Like [`call`](Self::call) but tries multiple candidate keys for an
    /// array result, returning the first that resolves.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the call fails or no key resolves to an array.
    pub fn call_array_flex(
        &mut self,
        capability: &str,
        method: &str,
        params: serde_json::Value,
        keys: &[&str],
    ) -> Result<(String, Vec<serde_json::Value>), IpcError> {
        let result = self.call(capability, method, params)?;
        for key in keys {
            if let Some(arr) = result.get(*key).and_then(serde_json::Value::as_array) {
                return Ok(((*key).to_owned(), arr.clone()));
            }
        }
        Err(IpcError::SerializationError {
            detail: format!("none of keys {keys:?} found as array in {result}"),
        })
    }

    /// All capability names that have live clients in this context.
    #[must_use]
    pub fn available_capabilities(&self) -> Vec<&str> {
        self.clients.keys().map(String::as_str).collect()
    }

    /// Whether this context has a live client for the given capability.
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.clients.contains_key(capability)
    }

    /// Normalized health check that handles all primal response formats.
    ///
    /// Primals return different health shapes: `{"alive":true}`,
    /// `{"status":"alive"}`, `{"status":"ok"}`. This method normalizes
    /// all of them into a single boolean.
    ///
    /// loamSpine requires `{"include_details": true}` — this method sends
    /// that param for the `ledger`/`spine`/`merkle` capabilities automatically.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the capability has no client or the call fails.
    pub fn health_check(&mut self, capability: &str) -> Result<bool, IpcError> {
        use crate::primal_names;
        let params = match capability_to_primal(capability) {
            primal_names::LOAMSPINE => serde_json::json!({"include_details": true}),
            _ => serde_json::json!({}),
        };
        let result = self.call(capability, "health.liveness", params)?;
        Ok(result
            .get("alive")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
            || result
                .get("status")
                .and_then(|s| s.as_str())
                .is_some_and(|s| s == "ok" || s == "alive"))
    }

    /// Hash arbitrary bytes via the security primal (`crypto.hash`).
    ///
    /// `BearDog` expects the `data` param as base64-encoded bytes and returns
    /// the hash as base64. This method handles the encoding round-trip.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if security is unavailable or the call fails.
    pub fn hash_bytes(&mut self, data: &[u8], algorithm: &str) -> Result<String, IpcError> {
        let encoded = BASE64.encode(data);
        let result = self.call(
            "security",
            "crypto.hash",
            serde_json::json!({"data": encoded, "algorithm": algorithm}),
        )?;
        result
            .get("hash")
            .and_then(|h| h.as_str())
            .map(String::from)
            .ok_or_else(|| IpcError::SerializationError {
                detail: format!("'hash' key not found in {result}"),
            })
    }

    /// Resolve a capability to its provider primal name using Songbird.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if discovery is unavailable or resolution fails.
    pub fn resolve_capability(&mut self, capability: &str) -> Result<serde_json::Value, IpcError> {
        let primal_id = capability_to_primal(capability);
        self.call(
            "discovery",
            "ipc.resolve",
            serde_json::json!({"primal_id": primal_id}),
        )
    }

    // ── Visualization convenience (petalTongue live pipeline) ──────────

    /// Push a scene to petalTongue for live rendering.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `visualization` capability is absent or
    /// the render call fails.
    pub fn push_scene(
        &mut self,
        session_id: &str,
        title: &str,
        data: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        self.call(
            "visualization",
            crate::ipc::methods::visualization::RENDER_SCENE,
            serde_json::json!({
                "session_id": session_id,
                "title": title,
                "domain": "game",
                "scene": data,
            }),
        )
    }

    /// Push a streaming metric update to petalTongue.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `visualization` capability is absent or
    /// the render call fails.
    pub fn push_stream(
        &mut self,
        session_id: &str,
        action: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, IpcError> {
        self.call(
            "visualization",
            crate::ipc::methods::visualization::RENDER_STREAM,
            serde_json::json!({
                "session_id": session_id,
                "action": action,
                "data": payload,
            }),
        )
    }

    /// Poll petalTongue for pending user interaction events.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] if the `visualization` capability is absent or
    /// the poll call fails.
    pub fn poll_interactions(&mut self, session_id: &str) -> Result<serde_json::Value, IpcError> {
        self.call(
            "visualization",
            crate::ipc::methods::interaction::POLL,
            serde_json::json!({ "session_id": session_id }),
        )
    }
}
