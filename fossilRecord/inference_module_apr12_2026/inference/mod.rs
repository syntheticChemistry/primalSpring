// SPDX-License-Identifier: AGPL-3.0-or-later

//! Vendor-agnostic inference provider abstraction.
//!
//! Defines the ecosystem-level contract for AI inference: any primal or
//! spring that exposes `inference.complete`, `inference.embed`, and
//! `inference.models` on its JSON-RPC socket is an inference provider.
//!
//! This decouples consumers (biomeOS, experiments, springs) from specific
//! vendors (Ollama, OpenAI, Anthropic). neuralSpring's native Rust models
//! slot in as just another provider — no code changes for consumers.
//!
//! # Wire Protocol
//!
//! Providers expose three JSON-RPC methods:
//!
//! | Method              | Request            | Response            |
//! |---------------------|--------------------|---------------------|
//! | `inference.complete` | [`CompleteRequest`] | [`CompleteResponse`] |
//! | `inference.embed`    | [`EmbedRequest`]    | [`EmbedResponse`]    |
//! | `inference.models`   | `{}`               | [`ModelsResponse`]   |
//!
//! # Discovery
//!
//! Providers register under the `"inference"` capability domain.
//! Consumers discover them via `capability.discover("inference")` through
//! biomeOS, or directly via the [`InferenceClient`] helper.

pub mod types;

pub use types::{
    CompleteRequest, CompleteResponse, EmbedInput, EmbedRequest, EmbedResponse, Message,
    MessageRole, ModelInfo, ModelsResponse, ProviderInfo, ProviderLocality, TokenUsage,
};

use crate::ipc::client::PrimalClient;
use crate::ipc::error::IpcError;
use std::path::{Path, PathBuf};

/// Capability domain for inference providers.
pub const CAPABILITY_DOMAIN: &str = "inference";

/// Environment variable to explicitly select an inference provider.
///
/// Values: `"ollama"`, `"neuralspring"`, `"openai"`, or a socket path.
/// When unset, discovery scans for any socket advertising inference
/// capabilities.
pub const PROVIDER_ENV: &str = "INFERENCE_PROVIDER";

/// The biomeOS socket directory (`$XDG_RUNTIME_DIR/biomeos/` or
/// `$TMPDIR/biomeos/`).
fn socket_dir() -> PathBuf {
    let base =
        std::env::var("XDG_RUNTIME_DIR").map_or_else(|_| std::env::temp_dir(), PathBuf::from);
    base.join("biomeos")
}

/// Client for calling an inference provider over JSON-RPC / UDS.
///
/// Wraps [`PrimalClient`] with typed methods matching the inference
/// wire protocol. Consumers don't need to know whether the backend
/// is Ollama, neuralSpring, or a remote API adapter.
pub struct InferenceClient {
    socket_path: PathBuf,
}

impl InferenceClient {
    /// Connect to an inference provider at the given socket path.
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    /// Discover and connect to an inference provider.
    ///
    /// Resolution order:
    /// 1. `INFERENCE_PROVIDER` env var (socket path or provider name)
    /// 2. `inference.sock` in the biomeOS socket directory
    /// 3. Family-suffixed sockets (`inference-{family}.sock`, `squirrel-{family}.sock`)
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::SocketNotFound`] if no inference provider socket
    /// is discovered at any of the candidate paths.
    pub fn discover() -> Result<Self, IpcError> {
        if let Ok(val) = std::env::var(PROVIDER_ENV) {
            let path = resolve_provider_value(&val);
            if path.exists() {
                return Ok(Self::new(path));
            }
            return Err(IpcError::SocketNotFound {
                primal: format!("inference (INFERENCE_PROVIDER={val})"),
            });
        }

        let dir = socket_dir();
        let candidates = [dir.join("inference.sock"), dir.join("squirrel.sock")];
        for candidate in &candidates {
            if candidate.exists() {
                return Ok(Self::new(candidate));
            }
        }

        let fid = std::env::var("BIOMEOS_FAMILY_ID")
            .or_else(|_| std::env::var("FAMILY_ID"))
            .unwrap_or_else(|_| "default".to_owned());
        let family_candidates = [
            dir.join(format!("inference-{fid}.sock")),
            dir.join(format!("squirrel-{fid}.sock")),
        ];
        for candidate in &family_candidates {
            if candidate.exists() {
                return Ok(Self::new(candidate));
            }
        }

        Err(IpcError::SocketNotFound {
            primal: "inference".into(),
        })
    }

    /// Text/chat completion.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure, RPC error, or
    /// deserialization failure.
    pub fn complete(&self, req: &CompleteRequest) -> Result<CompleteResponse, IpcError> {
        let mut client = PrimalClient::connect(&self.socket_path, "inference")?;
        let resp = client.call(
            types::wire_methods::COMPLETE,
            serde_json::to_value(req).map_err(|e| IpcError::SerializationError {
                detail: e.to_string(),
            })?,
        )?;
        if let Some(err) = resp.error {
            return Err(IpcError::from(err));
        }
        serde_json::from_value(resp.result.unwrap_or_default()).map_err(|e| {
            IpcError::SerializationError {
                detail: e.to_string(),
            }
        })
    }

    /// Embedding generation.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure, RPC error, or
    /// deserialization failure.
    pub fn embed(&self, req: &EmbedRequest) -> Result<EmbedResponse, IpcError> {
        let mut client = PrimalClient::connect(&self.socket_path, "inference")?;
        let resp = client.call(
            types::wire_methods::EMBED,
            serde_json::to_value(req).map_err(|e| IpcError::SerializationError {
                detail: e.to_string(),
            })?,
        )?;
        if let Some(err) = resp.error {
            return Err(IpcError::from(err));
        }
        serde_json::from_value(resp.result.unwrap_or_default()).map_err(|e| {
            IpcError::SerializationError {
                detail: e.to_string(),
            }
        })
    }

    /// List available models from the provider.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] on connection failure, RPC error, or
    /// deserialization failure.
    pub fn models(&self) -> Result<ModelsResponse, IpcError> {
        let mut client = PrimalClient::connect(&self.socket_path, "inference")?;
        let resp = client.call(types::wire_methods::MODELS, serde_json::json!({}))?;
        if let Some(err) = resp.error {
            return Err(IpcError::from(err));
        }
        serde_json::from_value(resp.result.unwrap_or_default()).map_err(|e| {
            IpcError::SerializationError {
                detail: e.to_string(),
            }
        })
    }

    /// The socket path this client is connected to.
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

/// Resolve a provider name or path to a socket path.
///
/// Known names map to conventional socket filenames:
/// - `"ollama"` → `squirrel.sock` (Ollama is accessed via Squirrel's adapter)
/// - `"neuralspring"` → `neuralspring.sock`
/// - `"openai"` → `squirrel.sock` (OpenAI is accessed via Squirrel's adapter)
/// - anything else → treated as a literal socket path
fn resolve_provider_value(val: &str) -> PathBuf {
    let dir = socket_dir();
    match val {
        "ollama" | "openai" | "anthropic" | "squirrel" => dir.join("squirrel.sock"),
        "neuralspring" => dir.join("neuralspring.sock"),
        other => {
            let path = PathBuf::from(other);
            if path.is_absolute() {
                path
            } else {
                dir.join(other)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_known_providers() {
        let ollama = resolve_provider_value("ollama");
        assert!(ollama.ends_with("squirrel.sock"));

        let neural = resolve_provider_value("neuralspring");
        assert!(neural.ends_with("neuralspring.sock"));

        let openai = resolve_provider_value("openai");
        assert!(openai.ends_with("squirrel.sock"));
    }

    #[test]
    fn resolve_absolute_path() {
        let path = resolve_provider_value("/tmp/custom.sock");
        assert_eq!(path, PathBuf::from("/tmp/custom.sock"));
    }

    #[test]
    fn resolve_relative_name() {
        let path = resolve_provider_value("my-provider.sock");
        assert!(path.ends_with("my-provider.sock"));
    }
}
