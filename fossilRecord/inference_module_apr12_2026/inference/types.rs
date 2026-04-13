// SPDX-License-Identifier: AGPL-3.0-or-later

//! Vendor-agnostic wire types for the `inference` capability domain.
//!
//! These types define the JSON-RPC request/response contract for
//! `inference.complete`, `inference.embed`, and `inference.models`.
//! Any primal or spring exposing these methods on its socket is an
//! inference provider — no vendor SDK, no HTTP assumption.

use serde::{Deserialize, Serialize};

// ─── inference.complete ──────────────────────────────────────────

/// Role of a message in a conversation (chat-style completions).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// System-level instructions or context.
    System,
    /// User input.
    User,
    /// Model-generated response.
    Assistant,
}

/// A single message in a conversation history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Who sent this message.
    pub role: MessageRole,
    /// Message text.
    pub content: String,
}

/// Request for `inference.complete`.
///
/// Supports both single-prompt and multi-turn chat. If `messages` is
/// non-empty it takes precedence over `prompt`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteRequest {
    /// Plain text prompt (single-turn).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Chat-style conversation history (multi-turn).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    /// Model identifier. Provider-specific (e.g. `"tinyllama"`,
    /// `"gpt-4"`, or a neuralSpring model tag).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Sampling temperature (0.0 = deterministic, higher = more random).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Maximum tokens to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Stop sequences.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
}

/// Response from `inference.complete`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteResponse {
    /// Generated text.
    pub text: String,

    /// Model that produced the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Token usage statistics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,

    /// Provider that fulfilled the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

/// Token consumption breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Tokens consumed by the input prompt/messages.
    pub prompt_tokens: u32,
    /// Tokens generated in the response.
    pub completion_tokens: u32,
    /// Sum of prompt + completion tokens.
    pub total_tokens: u32,
}

// ─── inference.embed ─────────────────────────────────────────────

/// Request for `inference.embed`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedRequest {
    /// Text(s) to embed. A single string is treated as a one-element batch.
    pub input: EmbedInput,

    /// Model identifier for the embedding model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Either a single string or a batch of strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbedInput {
    /// A single string to embed.
    Single(String),
    /// Multiple strings to embed as a batch.
    Batch(Vec<String>),
}

/// Response from `inference.embed`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedResponse {
    /// One embedding vector per input string.
    pub embeddings: Vec<Vec<f32>>,

    /// Model that produced the embeddings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

// ─── inference.models ────────────────────────────────────────────

/// A model available from an inference provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier (used in `CompleteRequest.model`).
    pub id: String,

    /// Human-readable name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether the model supports chat/completion.
    #[serde(default)]
    pub supports_completion: bool,

    /// Whether the model supports embedding.
    #[serde(default)]
    pub supports_embedding: bool,

    /// Parameter count (approximate, for routing decisions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter_count: Option<u64>,

    /// Context window size in tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u32>,
}

/// Response from `inference.models`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsResponse {
    /// Available models from this provider.
    pub models: Vec<ModelInfo>,
}

// ─── Provider metadata ───────────────────────────────────────────

/// Whether a provider runs locally or reaches an external service.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderLocality {
    /// In-process or same-host UDS (neuralSpring, local Ollama).
    Local,
    /// Remote HTTP API (OpenAI, Anthropic, etc.).
    Remote,
}

/// Metadata about an inference provider, used for routing decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier (e.g. `"ollama"`, `"neuralspring"`, `"openai"`).
    pub id: String,

    /// Human-readable name.
    pub name: String,

    /// Local vs remote.
    pub locality: ProviderLocality,

    /// Approximate latency in milliseconds (0 = unknown).
    #[serde(default)]
    pub avg_latency_ms: u64,

    /// Cost per 1K tokens in USD (None = free / unknown).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_per_1k_tokens: Option<f64>,
}

// ─── JSON-RPC wire method constants ──────────────────────────────

/// Standard JSON-RPC method names for the inference domain.
pub mod wire_methods {
    /// `inference.complete` — text/chat completion.
    pub const COMPLETE: &str = "inference.complete";
    /// `inference.embed` — embedding generation.
    pub const EMBED: &str = "inference.embed";
    /// `inference.models` — list available models.
    pub const MODELS: &str = "inference.models";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_request_round_trip() {
        let req = CompleteRequest {
            prompt: Some("Hello".into()),
            messages: vec![],
            model: Some("tinyllama".into()),
            temperature: Some(0.7),
            max_tokens: Some(128),
            stop: vec![],
        };
        let json = serde_json::to_string(&req).unwrap();
        let back: CompleteRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.prompt.as_deref(), Some("Hello"));
        assert_eq!(back.model.as_deref(), Some("tinyllama"));
    }

    #[test]
    fn complete_request_chat_style() {
        let req = CompleteRequest {
            prompt: None,
            messages: vec![
                Message {
                    role: MessageRole::System,
                    content: "You are helpful.".into(),
                },
                Message {
                    role: MessageRole::User,
                    content: "Hi".into(),
                },
            ],
            model: None,
            temperature: None,
            max_tokens: None,
            stop: vec![],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("system"));
        assert!(json.contains("user"));
    }

    #[test]
    fn embed_input_single_vs_batch() {
        let single: EmbedInput = serde_json::from_str("\"hello\"").unwrap();
        assert!(matches!(single, EmbedInput::Single(s) if s == "hello"));

        let batch: EmbedInput = serde_json::from_str("[\"a\", \"b\"]").unwrap();
        assert!(matches!(batch, EmbedInput::Batch(v) if v.len() == 2));
    }

    #[test]
    fn model_info_defaults() {
        let json = r#"{"id": "test-model"}"#;
        let info: ModelInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "test-model");
        assert!(!info.supports_completion);
        assert!(!info.supports_embedding);
    }

    #[test]
    fn provider_info_round_trip() {
        let info = ProviderInfo {
            id: "neuralspring".into(),
            name: "neuralSpring Native".into(),
            locality: ProviderLocality::Local,
            avg_latency_ms: 50,
            cost_per_1k_tokens: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        let back: ProviderInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(back.locality, ProviderLocality::Local);
    }
}
