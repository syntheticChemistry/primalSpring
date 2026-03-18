// SPDX-License-Identifier: AGPL-3.0-or-later

//! MCP tool definitions for Squirrel AI coordination tool discovery.
//!
//! Exposes primalSpring's coordination capabilities as typed MCP tools
//! with JSON Schema input definitions. Squirrel discovers these via
//! `mcp.tools.list` and can route AI requests to the correct
//! coordination method.
//!
//! Pattern absorbed from airSpring v0.10.0, wetSpring V128, and
//! healthSpring V37 — all expose domain-specific MCP tools.

use serde::Serialize;

/// A single MCP tool definition with typed JSON Schema.
#[derive(Debug, Clone, Serialize)]
pub struct McpTool {
    /// Tool name matching the JSON-RPC method.
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// JSON Schema for the tool's input parameters.
    pub input_schema: serde_json::Value,
}

/// All MCP tools exposed by primalSpring.
///
/// These map 1:1 to JSON-RPC methods and are returned by `mcp.tools.list`.
#[must_use]
pub fn list_tools() -> Vec<McpTool> {
    vec![
        McpTool {
            name: "coordination.validate_composition",
            description: "Validate an atomic composition (Tower, Node, Nest, or FullNucleus). \
                           Probes all required primals for health and capabilities.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "atomic": {
                        "type": "string",
                        "enum": ["Tower", "Node", "Nest", "FullNucleus"],
                        "description": "Atomic composition layer to validate"
                    }
                },
                "required": ["atomic"]
            }),
        },
        McpTool {
            name: "coordination.discovery_sweep",
            description: "Enumerate primals in a composition and report their discovery status \
                           (socket found, discovery source).",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "atomic": {
                        "type": "string",
                        "enum": ["Tower", "Node", "Nest", "FullNucleus"],
                        "description": "Composition to sweep for primals"
                    }
                }
            }),
        },
        McpTool {
            name: "coordination.neural_api_status",
            description: "Check whether the biomeOS Neural API is reachable.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        McpTool {
            name: "graph.validate",
            description: "Validate a deploy graph TOML file. Structural check by default; \
                           set live=true to probe each node's primal.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to a deploy graph TOML file"
                    },
                    "live": {
                        "type": "boolean",
                        "description": "If true, probe each node's primal for live health",
                        "default": false
                    }
                },
                "required": ["path"]
            }),
        },
        McpTool {
            name: "graph.list",
            description: "Structurally validate all deploy graphs in the graphs directory.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        McpTool {
            name: "health.check",
            description: "Check primalSpring's own health status.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        McpTool {
            name: "health.readiness",
            description: "Readiness probe — reports Neural API reachability and \
                           number of discovered primals.",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        McpTool {
            name: "lifecycle.status",
            description: "Report primalSpring's current status (version, domain, running state).",
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
    ]
}

/// Map an MCP tool name to its corresponding JSON-RPC method name.
///
/// Returns `None` for unknown tool names.
#[must_use]
pub fn tool_to_method(tool_name: &str) -> Option<&'static str> {
    let tools = list_tools();
    tools.iter().find(|t| t.name == tool_name).map(|t| t.name)
}

/// Discover MCP tools from a remote primal via `mcp.tools.list`.
///
/// Connects to the given socket, calls `mcp.tools.list`, and returns
/// the raw JSON array of tool definitions. Returns an empty `Vec` on
/// any failure (connection, protocol, missing method).
///
/// Pattern absorbed from Squirrel alpha.13 `spring_tools.rs`.
#[must_use]
pub fn discover_remote_tools(socket: &std::path::Path, primal: &str) -> Vec<serde_json::Value> {
    let Ok(mut client) = super::client::PrimalClient::connect(socket, primal) else {
        return Vec::new();
    };
    let Ok(resp) = client.call("mcp.tools.list", serde_json::json!({})) else {
        return Vec::new();
    };
    resp.result
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_tools_is_not_empty() {
        assert!(!list_tools().is_empty());
    }

    #[test]
    fn all_tools_have_descriptions() {
        for tool in list_tools() {
            assert!(
                !tool.description.is_empty(),
                "tool {} missing description",
                tool.name
            );
        }
    }

    #[test]
    fn all_tools_have_input_schema() {
        for tool in list_tools() {
            assert!(
                tool.input_schema.is_object(),
                "tool {} has non-object input_schema",
                tool.name
            );
        }
    }

    #[test]
    fn all_tool_names_follow_domain_verb() {
        for tool in list_tools() {
            assert!(
                tool.name.contains('.'),
                "tool name '{}' should follow domain.verb format",
                tool.name
            );
        }
    }

    #[test]
    fn tool_to_method_known_tool() {
        assert_eq!(tool_to_method("health.check"), Some("health.check"));
    }

    #[test]
    fn tool_to_method_unknown_tool() {
        assert!(tool_to_method("nonexistent.tool").is_none());
    }

    #[test]
    fn tool_count_matches_capability() {
        let tools = list_tools();
        assert_eq!(tools.len(), 8, "expected 8 MCP tools");
    }

    #[test]
    fn discover_remote_tools_returns_empty_on_missing_socket() {
        let nonexistent = std::path::Path::new("/tmp/nonexistent_primalspring_test.sock");
        let tools = discover_remote_tools(nonexistent, "test");
        assert!(tools.is_empty());
    }

    #[test]
    fn validate_composition_tool_has_required_atomic() {
        let tools = list_tools();
        let vc = tools
            .iter()
            .find(|t| t.name == "coordination.validate_composition")
            .unwrap();
        let required = vc.input_schema.get("required").unwrap();
        assert!(
            required
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("atomic"))
        );
    }
}
