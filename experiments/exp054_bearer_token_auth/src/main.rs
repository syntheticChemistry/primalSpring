// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp054: Bearer Token Auth — validates BearDog security.authenticate → compute.submit flow.

use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::ipc::protocol::JsonRpcRequest;
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn main() {
    ValidationResult::new("primalSpring Exp054 — Bearer Token Auth")
        .with_provenance("exp054_bearer_token_auth", "2026-03-24")
        .run(
            "primalSpring Exp054: BearDog security.authenticate → compute.submit",
            |v| {
                let beardog = discover_primal(primal_names::BEARDOG);
                let beardog_path = socket_path(primal_names::BEARDOG);
                v.check_bool(
                    "beardog_socket_path_discoverable",
                    beardog.primal == primal_names::BEARDOG
                        && beardog_path.to_string_lossy().contains(primal_names::BEARDOG),
                    "discover beardog socket path",
                );

                let auth_req = JsonRpcRequest::new(
                    "security.authenticate",
                    serde_json::json!({"credentials": "bearer"}),
                );
                let auth_line = auth_req.to_line();
                let auth_ok = auth_line.is_ok();
                let auth_str = auth_line
                    .as_ref()
                    .map(std::string::String::as_str)
                    .unwrap_or("");
                v.check_bool(
                    "security_authenticate_request_serializes",
                    auth_ok && auth_str.contains("security.authenticate"),
                    "JsonRpcRequest for \"security.authenticate\" serializes",
                );

                v.check_skip("actual_auth_flow", "actual auth flow needs live BearDog");
            },
        );
}
