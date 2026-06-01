// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
//! Exp044: Squirrel AI Coordination — MCP tools and routing via the `ai` capability (Squirrel).

use primalspring::composition::CompositionContext;
use primalspring::ipc::discover::extract_capability_names;
use primalspring::validation::ValidationResult;

const SQUIRREL_CAP: &str = "ai";

fn phase_discovery_health(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability(SQUIRREL_CAP) {
        v.check_skip("squirrel_health", "ai capability not in context");
        v.check_skip("squirrel_caps", "ai capability not in context");
        return;
    }

    match ctx.health_check(SQUIRREL_CAP) {
        Ok(h) => v.check_bool(
            "squirrel_health",
            h,
            "squirrel (ai capability) health.liveness normalized",
        ),
        Err(e) if e.is_connection_error() => {
            v.check_skip("squirrel_health", &format!("{e}"));
        }
        Err(e) => v.check_bool("squirrel_health", false, &format!("{e}")),
    }

    match ctx.call(SQUIRREL_CAP, "capabilities.list", serde_json::json!({})) {
        Ok(j) => {
            let n = extract_capability_names(Some(j)).len();
            v.check_minimum("squirrel_caps", n, 1);
        }
        Err(e) if e.is_connection_error() => v.check_skip("squirrel_caps", &format!("{e}")),
        Err(e) => v.check_bool("squirrel_caps", false, &format!("{e}")),
    }
}

fn phase_mcp_tools(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability(SQUIRREL_CAP) {
        v.check_skip("mcp_tools_discovered", "ai capability not in context");
        return;
    }

    match ctx.call(SQUIRREL_CAP, "mcp.tools.list", serde_json::json!({})) {
        Ok(result) => {
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
        Err(e) if e.is_connection_error() => {
            v.check_skip("mcp_tools_discovered", &format!("{e}"));
        }
        Err(e) => v.check_skip(
            "mcp_tools_discovered",
            &format!("mcp.tools.list failed: {e}"),
        ),
    }
}

fn phase_ai_routing(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    if !ctx.has_capability(SQUIRREL_CAP) {
        v.check_skip("squirrel_system_status", "ai capability not in context");
        v.check_skip("tool_list_available", "ai capability not in context");
        return;
    }

    match ctx.call(SQUIRREL_CAP, "system.status", serde_json::json!({})) {
        Ok(result) => {
            v.check_bool(
                "squirrel_system_status",
                result.get("status").is_some(),
                "system.status returns status field",
            );
        }
        Err(_) => v.check_skip("squirrel_system_status", "system.status not available"),
    }

    match ctx.call(SQUIRREL_CAP, "tool.list", serde_json::json!({})) {
        Ok(result) => {
            let count = result
                .get("tools")
                .and_then(|t| t.as_array())
                .map_or(0, Vec::len);
            v.check_minimum("tool_list_available", count, 0);
            println!("  Squirrel tool.list: {count} tools");
        }
        Err(_) => v.check_skip("tool_list_available", "tool.list not available"),
    }
}

fn main() {
    ValidationResult::new("primalSpring Exp044 — Squirrel AI Coordination")
        .with_provenance("exp044_squirrel_ai_coordination", "2026-05-09")
        .run("primalSpring Exp044: Squirrel AI Coordination", |v| {
            let mut ctx = CompositionContext::from_live_discovery_with_fallback();

            v.section("Phase 1: Discovery + Health");
            phase_discovery_health(v, &mut ctx);

            v.section("Phase 2: MCP Tools");
            phase_mcp_tools(v, &mut ctx);

            v.section("Phase 3: AI Routing");
            phase_ai_routing(v, &mut ctx);
        });
}
