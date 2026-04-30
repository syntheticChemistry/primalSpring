// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp100 — MCP Ecosystem Tools
//!
//! Validates Squirrel `tool.list` aggregation with multiple springs
//! announcing tools via MCP.
//!
//! Phase 56 — Desktop Substrate (AGENTIC_TRIO_EVOLUTION.md)

use primalspring::ipc::client::PrimalClient;
use primalspring::ipc::discover::discover_primal;
use primalspring::validation::ValidationResult;

fn phase_primalspring_tools(v: &mut ValidationResult) {
    v.section("primalSpring MCP Tools");

    let ps = discover_primal("primalspring_primal");
    let Some(ps_sock) = ps.socket.as_ref() else {
        v.check_skip("ps_tools", "primalSpring primal not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(ps_sock, "primalspring") else {
        v.check_skip("ps_tools", "primalSpring connection failed");
        return;
    };

    let resp = client.call("mcp.tools.list", serde_json::json!({}));
    match resp {
        Ok(r) => {
            let tools = r
                .result
                .as_ref()
                .and_then(|r| r.get("tools"))
                .and_then(|t| t.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "ps_tools",
                tools >= 8,
                &format!("primalSpring exposes {tools} MCP tools (expected >= 8)"),
            );
        }
        Err(e) => {
            v.check_skip("ps_tools", &format!("mcp.tools.list failed: {e}"));
        }
    }
}

fn phase_squirrel_aggregation(v: &mut ValidationResult) {
    v.section("Squirrel Tool Aggregation");

    let sq = discover_primal("squirrel");
    let Some(sq_sock) = sq.socket.as_ref() else {
        v.check_skip("sq_tools", "Squirrel not discovered");
        return;
    };

    let Ok(mut client) = PrimalClient::connect(sq_sock, "squirrel") else {
        v.check_skip("sq_tools", "Squirrel connection failed");
        return;
    };

    let resp = client.call("tool.list", serde_json::json!({}));
    match resp {
        Ok(r) => {
            let tools = r
                .result
                .as_ref()
                .and_then(|r| r.get("tools"))
                .and_then(|t| t.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "sq_tool_count",
                tools > 0,
                &format!("Squirrel aggregates {tools} tools from ecosystem"),
            );
        }
        Err(e) => {
            v.check_skip("sq_tools", &format!("tool.list failed: {e}"));
        }
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp100 — MCP Ecosystem Tools")
        .with_provenance("exp100_mcp_ecosystem_tools", "2026-04-28")
        .run(
            "Exp100: Squirrel tool aggregation from ecosystem primals",
            |v| {
                phase_primalspring_tools(v);
                phase_squirrel_aggregation(v);
            },
        );
}
