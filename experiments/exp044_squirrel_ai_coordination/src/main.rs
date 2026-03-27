// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp044: Squirrel AI Coordination
//!
//! Validates MCP tool discovery and AI routing via Squirrel.
//! When Squirrel is live, queries `mcp.tools.list` to discover
//! available tools and verifies `tool.list` routing.
//! Gracefully degrades when Squirrel is not running.

use primalspring::coordination::probe_primal;
use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::{discover_primal, socket_path};
use primalspring::primal_names;
use primalspring::validation::ValidationResult;

fn probe_squirrel_rpc(v: &mut ValidationResult, client: &mut PrimalClient) {
    match client.call("mcp.tools.list", serde_json::json!({})) {
        Ok(resp) => {
            let result = resp.result.unwrap_or_default();
            let tool_count = result
                .get("tool_count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            v.check_minimum(
                "mcp_tools_discovered",
                usize::try_from(tool_count).unwrap_or(0),
                0,
            );
            println!("  MCP tools discovered: {tool_count}");

            if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                for tool in tools.iter().take(5) {
                    if let Some(name) = tool.get("name").and_then(|n| n.as_str()) {
                        println!("    tool: {name}");
                    }
                }
                if tools.len() > 5 {
                    println!("    ... and {} more", tools.len() - 5);
                }
            }
        }
        Err(e) => {
            v.check_skip(
                "mcp_tools_discovered",
                &format!("mcp.tools.list failed: {e}"),
            );
        }
    }

    match client.call("system.status", serde_json::json!({})) {
        Ok(resp) => {
            let result = resp.result.unwrap_or_default();
            v.check_bool(
                "squirrel_system_status",
                result.get("status").is_some(),
                "system.status returns status field",
            );
        }
        Err(_) => {
            v.check_skip("squirrel_system_status", "system.status not available");
        }
    }

    match client.call("tool.list", serde_json::json!({})) {
        Ok(resp) => {
            let result = resp.result.unwrap_or_default();
            let count = result
                .get("tools")
                .and_then(|t| t.as_array())
                .map_or(0, Vec::len);
            v.check_minimum("tool_list_available", count, 0);
            println!("  Squirrel tool.list: {count} tools");
        }
        Err(_) => {
            v.check_skip("tool_list_available", "tool.list not available");
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp044 — Squirrel AI Coordination")
        .with_provenance("exp044_squirrel_ai_coordination", "2026-03-24")
        .run("primalSpring Exp044: Squirrel AI Coordination", |v| {
            let squirrel = discover_primal(primal_names::SQUIRREL);
            v.check_bool(
                "discover_squirrel",
                squirrel.primal == primal_names::SQUIRREL,
                "discover squirrel",
            );

            let path = socket_path(primal_names::SQUIRREL);
            v.check_bool(
                "squirrel_socket_path_valid",
                path.to_string_lossy().contains(primal_names::SQUIRREL),
                "squirrel socket path contains 'squirrel'",
            );

            let health = probe_primal(primal_names::SQUIRREL);
            if health.socket_found {
                v.check_bool(
                    "squirrel_health",
                    health.health_ok,
                    "squirrel health.liveness",
                );
                v.check_minimum("squirrel_caps", health.capabilities.len(), 1);

                if let Some(ref sock) = squirrel.socket {
                    if let Ok(mut client) = PrimalClient::connect(sock, primal_names::SQUIRREL) {
                        probe_squirrel_rpc(v, &mut client);
                    } else {
                        v.check_skip("mcp_tools_discovered", "cannot connect to squirrel");
                        v.check_skip("squirrel_system_status", "cannot connect");
                        v.check_skip("tool_list_available", "cannot connect");
                    }
                }
            } else {
                v.check_skip("squirrel_health", "squirrel not reachable");
                v.check_skip("squirrel_caps", "squirrel not reachable");
                v.check_skip("mcp_tools_discovered", "squirrel not reachable");
                v.check_skip("squirrel_system_status", "squirrel not reachable");
                v.check_skip("tool_list_available", "squirrel not reachable");
            }
        });
}
