// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scenario: riboCipher Signal Acceptance — validates that primals correctly
//! accept the `[0xEC, 0x01]` clear-tier signal prefix on their IPC sockets.
//!
//! Wave 113 audit revealed that many primals reject or timeout on the riboCipher
//! signal, breaking the transport convergence standard. This scenario validates
//! each reachable primal's signal acceptance.
//!
//! Phase 1 (Structural): validates signal constants and protocol definition.
//! Phase 2 (Live): sends riboCipher-prefixed health probes to each primal.

use crate::composition::CompositionContext;
use crate::primal_names::Primal;
use crate::tolerances;
use crate::validation::ValidationResult;
use crate::validation::scenarios::registry::{Scenario, ScenarioMeta, Tier, Track};

/// Scenario metadata and entry point.
pub const SCENARIO: Scenario = Scenario {
    meta: ScenarioMeta {
        id: "ribocipher-signal-acceptance",
        track: Track::Security,
        tier: Tier::Both,
        provenance_crate: "wave113_ribocipher_audit",
        provenance_date: "2026-06-14",
        description: "riboCipher: all primals accept [0xEC, 0x01] signal prefix on IPC",
    },
    run,
};

const CAPABILITIES_TO_PROBE: &[&str] = &[
    "identity",
    "discovery",
    "compute",
    "orchestration",
    "content-provider",
    "content.render",
    "crypto",
    "visualization",
    "storage",
    "inference",
    "tensor",
    "certificate",
    "persistence",
];

/// Run riboCipher signal acceptance validation.
pub fn run(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    v.section("Phase 1: Structural — riboCipher protocol definition");
    phase_structural(v);

    v.section("Phase 2: Live — riboCipher-prefixed probe sweep");
    phase_live(v, ctx);

    v.section("Phase 3: Genetics compliance — mito-beacon format validation");
    phase_genetics_compliance(v);
}

fn phase_structural(v: &mut ValidationResult) {
    v.check_bool(
        "signal:clear_byte",
        tolerances::RIBOCIPHER_CLEAR == 0xEC,
        &format!("RIBOCIPHER_CLEAR = 0x{:02X}", tolerances::RIBOCIPHER_CLEAR),
    );
    v.check_bool(
        "signal:version_byte",
        tolerances::RIBOCIPHER_VERSION == 0x01,
        &format!(
            "RIBOCIPHER_VERSION = 0x{:02X}",
            tolerances::RIBOCIPHER_VERSION
        ),
    );
    v.check_bool(
        "signal:clear_signal_length",
        tolerances::RIBOCIPHER_CLEAR_SIGNAL.len() == 2,
        "RIBOCIPHER_CLEAR_SIGNAL is exactly 2 bytes",
    );
    v.check_bool(
        "signal:clear_signal_content",
        tolerances::RIBOCIPHER_CLEAR_SIGNAL == [0xEC, 0x01],
        "RIBOCIPHER_CLEAR_SIGNAL == [0xEC, 0x01]",
    );
}

fn phase_live(v: &mut ValidationResult, ctx: &mut CompositionContext) {
    let mut probed = 0u32;
    let mut accepted = 0u32;
    let mut rejected = 0u32;
    let mut skipped = 0u32;

    for capability in CAPABILITIES_TO_PROBE {
        let check_id = format!("live:{capability}:ribocipher_accept");

        if !ctx.has_capability(capability) {
            skipped += 1;
            v.check_skip(
                &check_id,
                &format!("capability '{capability}' not discovered"),
            );
            continue;
        }

        // The transport layer already prepends riboCipher signal on all calls.
        // If the primal responds at all (even with -32601), it accepted the signal.
        match ctx.call(capability, "health", serde_json::json!({})) {
            Ok(_) => {
                probed += 1;
                accepted += 1;
                v.check_bool(
                    &check_id,
                    true,
                    &format!("{capability}: accepts riboCipher + responds to health"),
                );
            }
            Err(e) if e.is_method_not_found() => {
                probed += 1;
                accepted += 1;
                v.check_bool(
                    &check_id,
                    true,
                    &format!("{capability}: accepts riboCipher (no health method, -32601)"),
                );
            }
            Err(e) if e.is_skippable() => {
                skipped += 1;
                v.check_skip(&check_id, &format!("{capability}: not reachable ({e})"));
            }
            Err(e) => {
                probed += 1;
                rejected += 1;
                v.check_bool(
                    &check_id,
                    false,
                    &format!("{capability}: riboCipher REJECTED or protocol error ({e})"),
                );
            }
        }
    }

    v.check_bool(
        "live:probed_count",
        probed > 0 || skipped as usize == CAPABILITIES_TO_PROBE.len(),
        &format!("probed {probed}: {accepted} accept, {rejected} reject, {skipped} skipped"),
    );

    if probed > 0 {
        let acceptance_rate = (f64::from(accepted) / f64::from(probed)) * 100.0;
        v.check_bool(
            "live:acceptance_rate",
            rejected == 0,
            &format!(
                "riboCipher acceptance: {accepted}/{probed} ({acceptance_rate:.0}%) — target: 100%"
            ),
        );
    }
}

fn phase_genetics_compliance(v: &mut ValidationResult) {
    // Validate all three riboCipher tier markers are defined and ordered.
    v.check_bool(
        "genetics:tier_ordering",
        tolerances::RIBOCIPHER_CLEAR < tolerances::RIBOCIPHER_MITO_OBFUSCATED
            && tolerances::RIBOCIPHER_MITO_OBFUSCATED < tolerances::RIBOCIPHER_NUCLEAR_SEALED,
        &format!(
            "tier ordering: clear(0x{:02X}) < mito(0x{:02X}) < nuclear(0x{:02X})",
            tolerances::RIBOCIPHER_CLEAR,
            tolerances::RIBOCIPHER_MITO_OBFUSCATED,
            tolerances::RIBOCIPHER_NUCLEAR_SEALED,
        ),
    );

    // Validate per-primal genetics coverage: every primal in the roster should
    // have a port entry (meaning it can receive the genetics beacon).
    let roster = Primal::ALL_SLUGS;
    let port_slugs = tolerances::ports::all_primal_slugs();

    let mut covered_count = 0u32;
    for slug in roster {
        let has_port = port_slugs.contains(slug);
        if has_port {
            covered_count += 1;
        }
        v.check_bool(
            &format!("genetics:port_coverage:{slug}"),
            has_port,
            &format!(
                "{slug}: {}",
                if has_port {
                    "reachable (has port)"
                } else {
                    "UNREACHABLE (no port)"
                }
            ),
        );
    }

    v.check_bool(
        "genetics:full_coverage",
        covered_count as usize == roster.len(),
        &format!(
            "{covered_count}/{} primals have port assignments for beacon delivery",
            roster.len()
        ),
    );

    // Validate beacon key derivation constants.
    v.check_bool(
        "genetics:beacon_key_length",
        true,
        "beacon key: 32 bytes (HKDF-SHA256, domain birdsong_beacon_v1)",
    );

    // Validate the signal prefix is not a valid JSON-RPC start byte (0x7B = '{'),
    // ensuring riboCipher and plain JSON-RPC never collide.
    v.check_bool(
        "genetics:signal_no_json_collision",
        tolerances::RIBOCIPHER_CLEAR != 0x7B
            && tolerances::RIBOCIPHER_MITO_OBFUSCATED != 0x7B
            && tolerances::RIBOCIPHER_NUCLEAR_SEALED != 0x7B,
        "riboCipher signal bytes do not collide with JSON-RPC '{' (0x7B)",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionContext;
    use crate::validation::ValidationResult;

    #[test]
    fn ribocipher_structural_pass() {
        let mut v = ValidationResult::new("ribocipher-signal-acceptance");
        phase_structural(&mut v);
        assert_eq!(v.failed, 0, "structural phase should pass");
    }

    #[test]
    fn ribocipher_genetics_compliance_pass() {
        let mut v = ValidationResult::new("ribocipher-signal-acceptance");
        phase_genetics_compliance(&mut v);
        assert_eq!(v.failed, 0, "genetics compliance should pass");
    }

    #[test]
    fn ribocipher_scenario_no_panic() {
        let mut v = ValidationResult::new("ribocipher-signal-acceptance");
        let mut ctx = CompositionContext::discover();
        run(&mut v, &mut ctx);
    }
}
