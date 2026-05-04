// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parity validation helpers — compare primal composition results against local baselines.

use crate::ipc::IpcError;
use crate::validation::ValidationResult;

use super::context::CompositionContext;
use super::routing::capability_to_primal;

/// Validate scalar parity between a local baseline and a primal composition.
///
/// This is the primary convenience function for springs. It:
/// 1. Calls `method` on the provider of `capability` via the composition
/// 2. Extracts a scalar `f64` from the result using `result_key`
/// 3. Compares against `expected` within `tolerance`
/// 4. Records the outcome on `v` (pass, fail, or skip if IPC unavailable)
///
/// ```rust,no_run
/// # use primalspring::composition::{CompositionContext, validate_parity};
/// # use primalspring::validation::ValidationResult;
/// # use primalspring::tolerances;
/// # let mut ctx = CompositionContext::from_live_discovery();
/// # let mut v = ValidationResult::new("test");
/// validate_parity(
///     &mut ctx, &mut v,
///     "sample_mean",
///     "tensor",              // capability — resolves to barraCuda
///     "stats.mean",
///     serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
///     "result",
///     3.0,
///     tolerances::CPU_GPU_PARITY_TOL,
/// );
/// ```
#[expect(
    clippy::too_many_arguments,
    reason = "domain-driven API: each parameter is semantically distinct"
)]
pub fn validate_parity(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
    result_key: &str,
    expected: f64,
    tolerance: f64,
) {
    match ctx.call_f64(capability, method, params, result_key) {
        Ok(actual) => {
            let diff = (actual - expected).abs();
            let ok = diff <= tolerance;
            let detail = format!(
                "composition={actual}, local={expected}, diff={diff:.2e}, tol={tolerance:.2e}"
            );
            v.check_bool(name, ok, &detail);
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(name, &format!("{capability} not available: {e}"));
        }
        Err(e) if e.is_transport_mismatch() => {
            v.check_skip(
                name,
                &format!("{capability} uses non-JSON-RPC transport: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(name, false, &format!("composition error: {e}"));
        }
    }
}

/// Like [`validate_parity`] but tries multiple candidate response keys.
#[expect(
    clippy::too_many_arguments,
    reason = "domain-driven API: each parameter is semantically distinct"
)]
pub fn validate_parity_flex(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
    result_keys: &[&str],
    expected: f64,
    tolerance: f64,
) {
    match ctx.call_f64_flex(capability, method, params, result_keys) {
        Ok(actual) => {
            let diff = (actual - expected).abs();
            let ok = diff <= tolerance;
            let detail = format!(
                "composition={actual}, local={expected}, diff={diff:.2e}, tol={tolerance:.2e}"
            );
            v.check_bool(name, ok, &detail);
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(name, &format!("{capability} not available: {e}"));
        }
        Err(e) if e.is_transport_mismatch() => {
            v.check_skip(
                name,
                &format!("{capability} uses non-JSON-RPC transport: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(name, false, &format!("composition error: {e}"));
        }
    }
}

/// Like [`validate_parity_vec`] but tries multiple candidate response keys.
#[expect(
    clippy::too_many_arguments,
    reason = "domain-driven API: each parameter is semantically distinct"
)]
pub fn validate_parity_vec_flex(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
    result_keys: &[&str],
    expected: &[f64],
    tolerance: f64,
) {
    let (resolved_key, arr) = match ctx.call_array_flex(capability, method, params, result_keys) {
        Ok(pair) => pair,
        Err(e) if e.is_connection_error() => {
            v.check_skip(name, &format!("{capability} not available: {e}"));
            return;
        }
        Err(e) if e.is_transport_mismatch() => {
            v.check_skip(
                name,
                &format!("{capability} uses non-JSON-RPC transport: {e}"),
            );
            return;
        }
        Err(e) => {
            v.check_bool(name, false, &format!("composition error: {e}"));
            return;
        }
    };

    let actual = flatten_numeric_json_values(&arr);
    if actual.len() != arr.len() && actual.is_empty() {
        v.check_bool(
            name,
            false,
            &format!(
                "{}/{} array elements are not numeric (null, string, or object) — \
                 check primal response schema (key: '{resolved_key}')",
                arr.len() - actual.len(),
                arr.len()
            ),
        );
        return;
    }
    if actual.len() != expected.len() {
        v.check_bool(
            name,
            false,
            &format!(
                "length mismatch: composition={}, local={} (key: '{resolved_key}')",
                actual.len(),
                expected.len()
            ),
        );
        return;
    }
    let max_diff = actual
        .iter()
        .zip(expected.iter())
        .map(|(a, e)| (a - e).abs())
        .fold(0.0_f64, f64::max);
    let ok = max_diff <= tolerance;
    let detail = format!(
        "max_diff={max_diff:.2e}, tol={tolerance:.2e}, len={} (key: '{resolved_key}')",
        actual.len()
    );
    v.check_bool(name, ok, &detail);
}

fn flatten_numeric_json_values(arr: &[serde_json::Value]) -> Vec<f64> {
    let mut out = Vec::new();
    for val in arr {
        if let Some(n) = val.as_f64() {
            out.push(n);
        } else if let Some(inner) = val.as_array() {
            out.extend(flatten_numeric_json_values(inner));
        }
    }
    out
}

/// Validate vector parity between a local baseline and a primal composition.
///
/// Like [`validate_parity`] but for multi-element results (tensors, arrays).
/// All elements must match within tolerance for the check to pass.
#[expect(
    clippy::too_many_arguments,
    reason = "domain-driven API: each parameter is semantically distinct"
)]
pub fn validate_parity_vec(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
    result_key: &str,
    expected: &[f64],
    tolerance: f64,
) {
    let result = match ctx.call(capability, method, params) {
        Ok(r) => r,
        Err(e) if e.is_connection_error() => {
            v.check_skip(name, &format!("{capability} not available: {e}"));
            return;
        }
        Err(e) if e.is_transport_mismatch() => {
            v.check_skip(
                name,
                &format!("{capability} uses non-JSON-RPC transport: {e}"),
            );
            return;
        }
        Err(e) => {
            v.check_bool(name, false, &format!("composition error: {e}"));
            return;
        }
    };

    let arr = result.get(result_key).and_then(serde_json::Value::as_array);

    let Some(arr) = arr else {
        v.check_bool(
            name,
            false,
            &format!("key '{result_key}' not found or not an array"),
        );
        return;
    };

    let actual = flatten_numeric_json_values(arr);
    if actual.len() != arr.len() && actual.is_empty() {
        v.check_bool(
            name,
            false,
            &format!(
                "{}/{} array elements are not numeric (null, string, or object) — \
                 check primal response schema",
                arr.len() - actual.len(),
                arr.len()
            ),
        );
        return;
    }
    if actual.len() != expected.len() {
        v.check_bool(
            name,
            false,
            &format!(
                "length mismatch: composition={}, local={}",
                actual.len(),
                expected.len()
            ),
        );
        return;
    }

    let max_diff = actual
        .iter()
        .zip(expected.iter())
        .map(|(a, e)| (a - e).abs())
        .fold(0.0_f64, f64::max);
    let ok = max_diff <= tolerance;
    let detail = format!(
        "len={}, max_diff={max_diff:.2e}, tol={tolerance:.2e}",
        actual.len()
    );
    v.check_bool(name, ok, &detail);
}

/// Call a primal method, recording PASS on success, SKIP on absent/protocol
/// errors, and FAIL on unexpected errors.
///
/// Returns `Some(result)` on success so callers can chain pipeline steps.
pub fn call_or_skip(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    check_name: &str,
    capability: &str,
    method: &str,
    params: serde_json::Value,
) -> Option<serde_json::Value> {
    match ctx.call(capability, method, params) {
        Ok(result) => {
            v.check_bool(check_name, true, "ok");
            Some(result)
        }
        Err(e) if is_skip_error(&e) => {
            v.check_skip(check_name, &format!("{e}"));
            None
        }
        Err(e) => {
            v.check_bool(check_name, false, &format!("{e}"));
            None
        }
    }
}

/// Whether an IPC error should be treated as a graceful skip.
#[must_use]
pub fn is_skip_error(e: &IpcError) -> bool {
    e.is_connection_error() || e.is_protocol_error() || e.is_transport_mismatch()
}

/// Validate that a set of required capabilities are live in the composition.
///
/// Returns the count of capabilities that responded alive. Springs should
/// `exit(2)` when zero (no NUCLEUS deployed).
///
/// ```rust,no_run
/// # use primalspring::composition::{CompositionContext, validate_liveness};
/// # use primalspring::validation::ValidationResult;
/// # let mut ctx = CompositionContext::from_live_discovery();
/// # let mut v = ValidationResult::new("test");
/// let alive = validate_liveness(
///     &mut ctx, &mut v,
///     &["tensor", "security", "compute"],
/// );
/// if alive == 0 {
///     eprintln!("No NUCLEUS primals discovered.");
///     std::process::exit(2);
/// }
/// ```
pub fn validate_liveness(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    required_capabilities: &[&str],
) -> usize {
    let mut alive = 0;
    for &cap in required_capabilities {
        let primal = capability_to_primal(cap);
        let name = format!("{primal}.liveness");
        match ctx.health_check(cap) {
            Ok(true) => {
                v.check_bool(&name, true, &format!("{primal} alive via {cap}"));
                alive += 1;
            }
            Ok(false) => {
                v.check_bool(&name, false, &format!("{primal} responded but not alive"));
            }
            Err(e) if e.is_connection_error() => {
                v.check_skip(&name, &format!("{primal} not reachable: {e}"));
            }
            Err(e) if e.is_protocol_error() => {
                v.check_skip(
                    &name,
                    &format!("{primal} reachable but protocol mismatch (likely HTTP): {e}"),
                );
                alive += 1;
            }
            Err(e) => {
                v.check_bool(&name, false, &format!("{primal} health error: {e}"));
            }
        }
    }
    alive
}
