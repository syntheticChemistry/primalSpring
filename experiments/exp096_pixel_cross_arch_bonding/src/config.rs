// SPDX-License-Identifier: AGPL-3.0-or-later

use primalspring::ipc::tcp::tcp_rpc_multi_protocol;

pub(crate) fn pixel_host() -> String {
    std::env::var("PIXEL_HOST").unwrap_or_else(|_| "127.0.0.1".into())
}

pub(crate) fn pixel_beardog_port() -> u16 {
    std::env::var("PIXEL_BEARDOG_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9900)
}

pub(crate) fn pixel_songbird_port() -> u16 {
    std::env::var("PIXEL_SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9901)
}

pub(crate) fn pixel_nestgate_port() -> u16 {
    std::env::var("PIXEL_NESTGATE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9902)
}

pub(crate) fn family_id() -> String {
    std::env::var("FAMILY_ID").unwrap_or_else(|_| "pixel-cross-arch-test".into())
}

pub(crate) fn tcp_rpc_value(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, primalspring::ipc::IpcError> {
    tcp_rpc_multi_protocol(host, port, method, params).map(|(v, _)| v)
}
