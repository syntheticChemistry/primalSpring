// SPDX-License-Identifier: AGPL-3.0-or-later

use primalspring::ipc::NeuralBridge;
use primalspring::ipc::tcp::tcp_rpc_multi_protocol;

pub mod domain {
    pub const SECURITY: &str = "security";
    pub const DISCOVERY: &str = "discovery";
    pub const STORAGE: &str = "storage";
}

pub fn pixel_host() -> String {
    std::env::var("PIXEL_HOST").unwrap_or_else(|_| "127.0.0.1".into())
}

pub fn pixel_beardog_port() -> u16 {
    std::env::var("PIXEL_BEARDOG_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9900)
}

pub fn pixel_songbird_port() -> u16 {
    std::env::var("PIXEL_SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9901)
}

pub fn pixel_nestgate_port() -> u16 {
    std::env::var("PIXEL_NESTGATE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9902)
}

pub fn family_id() -> String {
    std::env::var("FAMILY_ID").unwrap_or_else(|_| "pixel-cross-arch-test".into())
}

/// Direct TCP JSON-RPC (legacy cross-gate transport).
#[cfg_attr(not(feature = "primordial-compat"), expect(dead_code, reason = "TCP fallback only with primordial-compat"))]
pub fn tcp_rpc_value(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, primalspring::ipc::IpcError> {
    tcp_rpc_multi_protocol(host, port, method, params).map(|(v, _)| v)
}

/// Primary RPC path: `NeuralBridge::capability_call()`, with optional direct TCP fallback.
#[cfg_attr(
    not(feature = "primordial-compat"),
    expect(unused_variables, reason = "tcp_host/tcp_port used only with primordial-compat")
)]
pub fn rpc_value(
    bridge: Option<&NeuralBridge>,
    capability_domain: &str,
    method: &str,
    params: &serde_json::Value,
    tcp_host: &str,
    tcp_port: u16,
) -> Result<serde_json::Value, primalspring::ipc::IpcError> {
    if let Some(bridge) = bridge {
        match bridge.capability_call(capability_domain, method, params) {
            Ok(resp) => return Ok(resp.value),
            Err(e) => {
                #[cfg(feature = "primordial-compat")]
                {
                    let _ = e;
                }
                #[cfg(not(feature = "primordial-compat"))]
                return Err(e);
            }
        }
    }

    #[cfg(feature = "primordial-compat")]
    {
        tcp_rpc_value(tcp_host, tcp_port, method, params)
    }

    #[cfg(not(feature = "primordial-compat"))]
    {
        Err(primalspring::ipc::IpcError::SocketNotFound {
            primal: "neural-bridge".into(),
        })
    }
}
