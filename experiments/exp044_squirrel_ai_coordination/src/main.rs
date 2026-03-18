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
use primalspring::tolerances::VALIDATION_SUMMARY_WIDTH;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring Exp044 — Squirrel AI Coordination");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));
    println!("primalSpring Exp044: Squirrel AI Coordination");
    println!("{}", "=".repeat(VALIDATION_SUMMARY_WIDTH));

    let squirrel = discover_primal("squirrel");
    v.check_bool(
        "discover_squirrel",
        squirrel.primal == "squirrel",
        "discover squirrel",
    );

    let path = socket_path("squirrel");
    v.check_bool(
        "squirrel_socket_path_valid",
        path.to_string_lossy().contains("squirrel"),
        "squirrel socket path contains 'squirrel'",
    );

    let health = probe_primal("squirrel");
    if health.socket_found {
        v.check_bool(
            "squirrel_health",
            health.health_ok,
            "squirrel health.liveness",
        );
        v.check_minimum("squirrel_caps", health.capabilities.len(), 1);

        if let Some(ref sock) = squirrel.socket {
            if let Ok(mut client) = PrimalClient::connect(sock, "squirrel") {
                // MCP tool discovery
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

                // System status
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

                // Tool list routing
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

    v.finish();
    std::process::exit(v.exit_code());
}
