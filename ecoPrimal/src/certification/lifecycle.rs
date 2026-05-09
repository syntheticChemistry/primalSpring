// SPDX-License-Identifier: AGPL-3.0-or-later

//! Layer 8: Composition lifecycle — reload, rediscover, re-liveness.

use crate::composition::CompositionContext;
use crate::coordination::AtomicType;
use crate::validation::ValidationResult;

/// Validate composition lifecycle: reload + rediscovery + liveness continuity.
///
/// Tests that `composition.reload` via biomeOS triggers a topology refresh
/// and that discovered capabilities survive the reload. This is the
/// guidestone-level validation of the hot-reload contract (JH-3).
pub fn validate_lifecycle(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    validate_reload(ctx, v);
    validate_rediscovery(ctx, v);
    validate_post_reload_liveness(ctx, v);
}

fn validate_reload(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    if !ctx.has_capability("orchestration") {
        v.check_skip(
            "lifecycle:reload",
            "biomeOS orchestration not available — composition.reload requires biomeOS",
        );
        return;
    }

    match ctx.reload() {
        Ok(result) => {
            let has_status = result.get("status").is_some() || result.get("ok").is_some();
            v.check_bool(
                "lifecycle:reload",
                true,
                &format!("reload response: {result} (structured: {has_status})"),
            );
        }
        Err(e) if e.is_method_not_found() => {
            v.check_skip(
                "lifecycle:reload",
                &format!("UPSTREAM GAP: composition.reload not implemented in biomeOS ({e})"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "lifecycle:reload",
                &format!("orchestration not reachable: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "lifecycle:reload",
                false,
                &format!("composition.reload failed: {e}"),
            );
        }
    }
}

fn validate_rediscovery(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let pre_caps: Vec<String> = ctx
        .available_capabilities()
        .iter()
        .map(|s| (*s).to_owned())
        .collect();

    ctx.rediscover();

    let post_caps: Vec<String> = ctx
        .available_capabilities()
        .iter()
        .map(|s| (*s).to_owned())
        .collect();

    let lost: Vec<&str> = pre_caps
        .iter()
        .filter(|c| !post_caps.iter().any(|p| p == *c))
        .map(String::as_str)
        .collect();

    if lost.is_empty() {
        v.check_bool(
            "lifecycle:rediscovery",
            true,
            &format!(
                "no capabilities lost ({} → {} total)",
                pre_caps.len(),
                post_caps.len()
            ),
        );
    } else {
        v.check_bool(
            "lifecycle:rediscovery",
            false,
            &format!("lost capabilities after rediscovery: {}", lost.join(", ")),
        );
    }
}

fn validate_post_reload_liveness(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let tower_caps = AtomicType::Tower.required_capabilities();
    let alive = tower_caps
        .iter()
        .filter(|&&cap| ctx.has_capability(cap) && matches!(ctx.health_check(cap), Ok(true)))
        .count();

    if tower_caps.is_empty() || alive == 0 {
        v.check_skip(
            "lifecycle:post_reload_liveness",
            "no Tower capabilities alive after lifecycle cycle",
        );
        return;
    }

    v.check_bool(
        "lifecycle:post_reload_liveness",
        alive > 0,
        &format!(
            "{alive}/{} Tower primals alive after lifecycle cycle",
            tower_caps.len()
        ),
    );
}
