// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! exp100 — MCP Ecosystem Tools
//!
//! Validates Squirrel `tool.list` aggregation with multiple springs
//! announcing tools via MCP.
//!
//! Phase 56 — Desktop Substrate (AGENTIC_TRIO_EVOLUTION.md)

use primalspring::composition::CompositionContext;
use primalspring::ipc::IpcError;
use primalspring::validation::ValidationResult;

fn orchestration_route(
    ctx: &mut CompositionContext,
    capability: &str,
    operation: &str,
    args: &serde_json::Value,
) -> Result<serde_json::Value, IpcError> {
    ctx.call(
        "orchestration",
        "capability.call",
        serde_json::json!({
            "capability": capability,
            "operation": operation,
            "args": args,
        }),
    )
}

fn phase_primalspring_tools(v: &mut ValidationResult) {
    v.section("primalSpring MCP Tools");

    let mut ctx = CompositionContext::discover();

    let resp = if ctx.has_capability("orchestration") {
        orchestration_route(&mut ctx, "mcp", "mcp.tools.list", &serde_json::json!({})).or_else(
            |_| {
                orchestration_route(
                    &mut ctx,
                    "primalspring",
                    "mcp.tools.list",
                    &serde_json::json!({}),
                )
            },
        )
    } else {
        Err(IpcError::SocketNotFound {
            primal: "orchestration".to_owned(),
        })
    };

    match resp {
        Ok(r) => {
            let tools = r
                .get("tools")
                .or_else(|| r.get("result").and_then(|x| x.get("tools")))
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

    let mut ctx = CompositionContext::discover();

    if !ctx.has_capability("ai") {
        v.check_skip("sq_tools", "Squirrel not discovered");
        return;
    }

    match ctx.call("ai", "tool.list", serde_json::json!({})) {
        Ok(r) => {
            let tools = r
                .get("tools")
                .or_else(|| r.get("result").and_then(|x| x.get("tools")))
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
        .with_provenance("exp100_mcp_ecosystem_tools", "2026-05-09")
        .run(
            "Exp100: Squirrel tool aggregation from ecosystem primals",
            |v| {
                phase_primalspring_tools(v);
                phase_squirrel_aggregation(v);
            },
        );
}
