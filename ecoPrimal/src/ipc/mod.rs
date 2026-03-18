// SPDX-License-Identifier: AGPL-3.0-or-later

//! IPC client for primalSpring coordination validation.
//!
//! Provides JSON-RPC 2.0 over Unix sockets for communicating with primals
//! during composition and coordination validation. Discovery is
//! capability-based: primalSpring has only self-knowledge and discovers
//! peers at runtime via environment overrides, XDG socket convention,
//! or the Neural API.

pub mod client;
pub mod discover;
pub mod protocol;

pub use neural_api_client_sync::NeuralBridge;
