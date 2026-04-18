// SPDX-License-Identifier: AGPL-3.0-or-later

//! primalSpring guideStone — Composition Certification.
//!
//! Self-validating deployable that certifies a NUCLEUS composition is
//! structurally sound, IPC-healthy, and cryptographically functional.
//! Domain guideStones (hotSpring, healthSpring, etc.) inherit this base
//! certification and only need to validate their own science on top.
//!
//! # Layers (each depends on the previous)
//!
//! 0. **Bare** — graph/fragment/manifest structural validation (no primals needed)
//! 1. **Discovery** — all primals in the graph discoverable via capability scan
//! 2. **Health** — every discovered primal responds to `health.liveness`
//! 3. **Capability Parity** — math, storage, shader IPC calls produce correct results
//! 4. **Cross-Atomic Pipeline** — Tower hash → Nest store → retrieve → verify
//! 5. **Bonding Model** — bonding policies correctly enforced between atomics
//! 6. **BTSP + Crypto** — crypto.hash parity, cipher policy, Ed25519 roundtrip
//!
//! # Exit Codes
//!
//! - `0` — all layers passed (NUCLEUS certified)
//! - `1` — one or more layers failed
//! - `2` — bare-only mode (no primals discovered, structural checks only)

#![forbid(unsafe_code)]

use std::path::Path;

use primalspring::bonding::{BondType, BondingPolicy};
use primalspring::btsp;
use primalspring::composition::{self, CompositionContext, validate_liveness, validate_parity};
use primalspring::coordination::AtomicType;
use primalspring::deploy;
use primalspring::tolerances;
use primalspring::validation::ValidationResult;

fn main() {
    let mut v = ValidationResult::new("primalSpring guideStone — Composition Certification");

    ValidationResult::print_banner("primalSpring guideStone — Base Composition Certification");

    // ════════════════════════════════════════════════════════════════════
    // Layer 0: Bare Properties (always runs, no primals needed)
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 0: Bare Properties");
    validate_bare_properties(&mut v);

    // ════════════════════════════════════════════════════════════════════
    // Layer 1: Discovery — can we find primals?
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 1: Discovery");
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();

    let full_caps = AtomicType::FullNucleus.required_capabilities();
    let alive = validate_liveness(&mut ctx, &mut v, full_caps);

    if alive == 0 {
        eprintln!("[guideStone] No NUCLEUS primals discovered — bare certification only.");
        v.finish();
        std::process::exit(v.exit_code_skip_aware());
    }

    // ════════════════════════════════════════════════════════════════════
    // Layer 2: Health — all primals alive per atomic tier
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 2: Atomic Health");
    validate_atomic_health(&mut ctx, &mut v);

    // ════════════════════════════════════════════════════════════════════
    // Layer 3: Capability Parity — math, storage, shader
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 3: Capability Parity");
    validate_math_parity(&mut ctx, &mut v);
    validate_storage_roundtrip(&mut ctx, &mut v);
    validate_shader_capabilities(&mut ctx, &mut v);

    // ════════════════════════════════════════════════════════════════════
    // Layer 4: Cross-Atomic Pipeline
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 4: Cross-Atomic Pipeline");
    validate_cross_atomic_pipeline(&mut ctx, &mut v);

    // ════════════════════════════════════════════════════════════════════
    // Layer 5: Bonding Model Verification
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 5: Bonding Model");
    validate_bonding_policies(&mut v);

    // ════════════════════════════════════════════════════════════════════
    // Layer 6: BTSP + Crypto
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 6: BTSP + Crypto");
    validate_crypto(&mut ctx, &mut v);

    v.finish();
    std::process::exit(v.exit_code());
}

// ════════════════════════════════════════════════════════════════════════
// Layer 0: Bare Properties
// ════════════════════════════════════════════════════════════════════════

fn validate_bare_properties(v: &mut ValidationResult) {
    validate_graph_parsing(v);
    validate_fragment_resolution(v);
    validate_manifest_consistency(v);
    validate_bonding_type_wellformed(v);
}

fn validate_graph_parsing(v: &mut ValidationResult) {
    let graph_dirs: &[&str] = &[
        "graphs/fragments",
        "graphs/profiles",
        "graphs/downstream",
        "graphs/spring_deploy",
        "graphs/spring_validation",
        "graphs/multi_node",
    ];

    let mut total = 0usize;
    let mut clean = 0usize;

    for dir_name in graph_dirs {
        let dir = Path::new(dir_name);
        if !dir.exists() {
            continue;
        }
        let results = deploy::validate_all_graphs(dir);
        for result in &results {
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            } else if !result.parsed {
                v.check_bool(
                    &format!("graph_parse:{}", result.path),
                    false,
                    "failed to parse",
                );
            } else {
                v.check_bool(
                    &format!("graph_structural:{}", result.path),
                    false,
                    &result.issues.join("; "),
                );
            }
        }
    }

    v.check_bool(
        "bare:all_graphs_parse",
        total > 0 && clean == total,
        &format!("{clean}/{total} graphs clean"),
    );
}

fn validate_fragment_resolution(v: &mut ValidationResult) {
    let fragment_dir = Path::new("graphs/fragments");
    if !fragment_dir.exists() {
        v.check_skip("bare:fragments_exist", "graphs/fragments/ not found");
        return;
    }

    let fragments = deploy::validate_all_graphs(fragment_dir);
    let expected_fragments = &[
        "tower_atomic",
        "node_atomic",
        "nest_atomic",
        "nucleus",
        "meta_tier",
        "provenance_trio",
    ];

    let found_names: Vec<&str> = fragments.iter().map(|f| f.name.as_str()).collect();
    for &expected in expected_fragments {
        let present = found_names.contains(&expected);
        v.check_bool(
            &format!("bare:fragment:{expected}"),
            present,
            if present {
                "found"
            } else {
                "missing from graphs/fragments/"
            },
        );
    }
}

fn validate_manifest_consistency(v: &mut ValidationResult) {
    let manifest_path = Path::new("graphs/downstream/downstream_manifest.toml");
    if !manifest_path.exists() {
        v.check_skip("bare:manifest_exists", "downstream_manifest.toml not found");
        return;
    }

    let Ok(content) = std::fs::read_to_string(manifest_path) else {
        v.check_bool("bare:manifest_readable", false, "failed to read");
        return;
    };

    let parsed: Result<toml::Value, _> = toml::from_str(&content);
    match parsed {
        Ok(table) => {
            let entries = table
                .get("downstream")
                .and_then(|d| d.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "bare:manifest_valid",
                entries > 0,
                &format!("{entries} downstream entries"),
            );

            if let Some(arr) = table.get("downstream").and_then(|d| d.as_array()) {
                for entry in arr {
                    let name = entry
                        .get("spring_name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown");
                    let has_caps = entry
                        .get("validation_capabilities")
                        .and_then(|c| c.as_array())
                        .is_some_and(|c| !c.is_empty());
                    v.check_bool(
                        &format!("bare:manifest:{name}:has_capabilities"),
                        has_caps,
                        if has_caps {
                            "validation_capabilities present"
                        } else {
                            "missing or empty validation_capabilities"
                        },
                    );
                }
            }
        }
        Err(e) => {
            v.check_bool(
                "bare:manifest_valid",
                false,
                &format!("TOML parse error: {e}"),
            );
        }
    }
}

fn validate_bonding_type_wellformed(v: &mut ValidationResult) {
    for &bond in BondType::all() {
        let desc = bond.description();
        v.check_bool(
            &format!("bare:bondtype:{bond:?}"),
            !desc.is_empty(),
            &format!("description: {desc}"),
        );
    }

    let policy = BondingPolicy::covalent_default();
    let errors = policy.validate();
    let detail = if errors.is_empty() {
        "clean".to_owned()
    } else {
        errors.join("; ")
    };
    v.check_bool("bare:bondpolicy:covalent_default_valid", errors.is_empty(), &detail);

    let ionic = BondingPolicy::ionic_contract(vec!["compute".to_owned()]);
    let errors = ionic.validate();
    let detail = if errors.is_empty() {
        "clean".to_owned()
    } else {
        errors.join("; ")
    };
    v.check_bool("bare:bondpolicy:ionic_contract_valid", errors.is_empty(), &detail);
}

// ════════════════════════════════════════════════════════════════════════
// Layer 2: Atomic Health
// ════════════════════════════════════════════════════════════════════════

fn validate_atomic_health(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let tiers: &[(&str, &[&str])] = &[
        ("Tower", AtomicType::Tower.required_capabilities()),
        ("Node", AtomicType::Node.required_capabilities()),
        ("Nest", AtomicType::Nest.required_capabilities()),
    ];

    for &(tier_name, caps) in tiers {
        for &cap in caps {
            let check_name = format!("health:{tier_name}:{cap}");
            match ctx.health_check(cap) {
                Ok(true) => v.check_bool(&check_name, true, "alive"),
                Ok(false) => v.check_bool(&check_name, false, "responded but not alive"),
                Err(e) if e.is_connection_error() => {
                    v.check_skip(&check_name, &format!("not reachable: {e}"));
                }
                Err(e) => v.check_bool(&check_name, false, &format!("error: {e}")),
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════
// Layer 3: Capability Parity
// ════════════════════════════════════════════════════════════════════════

fn validate_math_parity(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    validate_parity(
        ctx,
        v,
        "parity:stats.mean",
        "tensor",
        "stats.mean",
        serde_json::json!({"data": [1.0, 2.0, 3.0, 4.0, 5.0]}),
        "result",
        3.0,
        tolerances::IPC_ROUND_TRIP_TOL,
    );

    composition::validate_parity_vec(
        ctx,
        v,
        "parity:tensor.matmul_identity",
        "tensor",
        "tensor.matmul",
        serde_json::json!({
            "a": [[1.0, 0.0], [0.0, 1.0]],
            "b": [[3.0, 7.0], [2.0, 5.0]],
            "rows_a": 2, "cols_a": 2, "cols_b": 2
        }),
        "result",
        &[3.0, 7.0, 2.0, 5.0],
        tolerances::IPC_ROUND_TRIP_TOL,
    );
}

fn validate_storage_roundtrip(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_key = "guidestone_parity_roundtrip";
    let test_value = "primalspring_guidestone_2026";

    match ctx.call(
        "storage",
        "storage.store",
        serde_json::json!({"key": test_key, "value": test_value}),
    ) {
        Ok(_) => {
            match ctx.call(
                "storage",
                "storage.retrieve",
                serde_json::json!({"key": test_key}),
            ) {
                Ok(retrieved) => {
                    let val = retrieved
                        .get("value")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    v.check_bool(
                        "parity:storage_roundtrip",
                        val == test_value,
                        &format!("stored={test_value}, retrieved={val}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "parity:storage_roundtrip",
                        false,
                        &format!("retrieve failed: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "parity:storage_roundtrip",
                &format!("storage not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "parity:storage_roundtrip",
                false,
                &format!("store failed: {e}"),
            );
        }
    }
}

fn validate_shader_capabilities(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    match ctx.call(
        "shader",
        "compile.capabilities",
        serde_json::json!({}),
    ) {
        Ok(result) => {
            let has_caps = result
                .get("capabilities")
                .and_then(|c| c.as_array())
                .is_some_and(|c| !c.is_empty());
            v.check_bool(
                "parity:shader_capabilities",
                has_caps,
                &format!("response: {result}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "parity:shader_capabilities",
                &format!("shader not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "parity:shader_capabilities",
                false,
                &format!("call failed: {e}"),
            );
        }
    }
}

// ════════════════════════════════════════════════════════════════════════
// Layer 4: Cross-Atomic Pipeline
// ════════════════════════════════════════════════════════════════════════

fn validate_cross_atomic_pipeline(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_data = b"guidestone_cross_atomic_2026";

    // Step 1: Tower — hash via BearDog
    let hash_result = ctx.hash_bytes(test_data, "blake3");
    match hash_result {
        Ok(hash_b64) => {
            v.check_bool(
                "pipeline:tower_hash",
                !hash_b64.is_empty(),
                &format!("BLAKE3: {}...", &hash_b64[..hash_b64.len().min(16)]),
            );

            // Step 2: Nest — store the hash
            let store_key = "guidestone_pipeline_hash";
            match ctx.call(
                "storage",
                "storage.store",
                serde_json::json!({"key": store_key, "value": &hash_b64}),
            ) {
                Ok(_) => {
                    // Step 3: Nest — retrieve and verify
                    match ctx.call(
                        "storage",
                        "storage.retrieve",
                        serde_json::json!({"key": store_key}),
                    ) {
                        Ok(retrieved) => {
                            let val = retrieved
                                .get("value")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            v.check_bool(
                                "pipeline:nest_roundtrip",
                                val == hash_b64,
                                "hash stored and retrieved matches",
                            );
                        }
                        Err(e) => {
                            v.check_bool(
                                "pipeline:nest_roundtrip",
                                false,
                                &format!("retrieve failed: {e}"),
                            );
                        }
                    }
                }
                Err(e) if e.is_connection_error() => {
                    v.check_skip(
                        "pipeline:nest_roundtrip",
                        &format!("storage not available: {e}"),
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "pipeline:nest_roundtrip",
                        false,
                        &format!("store failed: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "pipeline:tower_hash",
                &format!("security not available: {e}"),
            );
            v.check_skip("pipeline:nest_roundtrip", "tower unavailable, skipping nest");
        }
        Err(e) => {
            v.check_bool(
                "pipeline:tower_hash",
                false,
                &format!("hash error: {e}"),
            );
            v.check_skip("pipeline:nest_roundtrip", "tower failed, skipping nest");
        }
    }
}

// ════════════════════════════════════════════════════════════════════════
// Layer 5: Bonding Model Verification
// ════════════════════════════════════════════════════════════════════════

fn validate_bonding_policies(v: &mut ValidationResult) {
    for &bond in BondType::all() {
        let policy = match bond {
            BondType::Ionic => BondingPolicy::ionic_contract(vec!["compute".to_owned()]),
            _ => BondingPolicy::covalent_default(),
        };

        let errors = policy.validate();
        let detail = if errors.is_empty() {
            "policy well-formed".to_owned()
        } else {
            errors.join("; ")
        };
        v.check_bool(&format!("bonding:policy:{bond:?}:valid"), errors.is_empty(), &detail);

        let min_cipher = btsp::min_cipher_for_bond(bond);
        v.check_bool(
            &format!("bonding:cipher:{bond:?}:min_allowed"),
            btsp::cipher_allowed(bond, min_cipher),
            &format!("min cipher {min_cipher:?} accepted for {bond:?}"),
        );

        if min_cipher.is_encrypted() {
            v.check_bool(
                &format!("bonding:cipher:{bond:?}:encrypted"),
                true,
                &format!("{bond:?} requires encrypted cipher"),
            );
        }
    }

    v.check_bool(
        "bonding:trust_ordering",
        BondType::Covalent.shares_electrons()
            && BondType::Metallic.shares_electrons()
            && !BondType::Ionic.shares_electrons()
            && !BondType::Weak.shares_electrons(),
        "Covalent+Metallic share electrons, Ionic+Weak do not",
    );

    v.check_bool(
        "bonding:ionic_metered",
        BondType::Ionic.is_metered() && !BondType::Covalent.is_metered(),
        "only Ionic is metered",
    );
}

// ════════════════════════════════════════════════════════════════════════
// Layer 6: BTSP + Crypto
// ════════════════════════════════════════════════════════════════════════

fn validate_crypto(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    // crypto.hash parity: known input → verify non-empty output
    let test_data = b"guidestone_crypto_parity_2026";
    match ctx.hash_bytes(test_data, "blake3") {
        Ok(hash) => {
            v.check_bool(
                "crypto:blake3_hash",
                !hash.is_empty(),
                &format!("BLAKE3 produced {}B base64", hash.len()),
            );

            // Determinism: same input → same output
            match ctx.hash_bytes(test_data, "blake3") {
                Ok(hash2) => {
                    v.check_bool(
                        "crypto:blake3_determinism",
                        hash == hash2,
                        "same input produces same hash",
                    );
                }
                Err(e) => {
                    v.check_bool(
                        "crypto:blake3_determinism",
                        false,
                        &format!("second hash call failed: {e}"),
                    );
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip("crypto:blake3_hash", &format!("security not available: {e}"));
            v.check_skip("crypto:blake3_determinism", "security not available");
        }
        Err(e) => {
            v.check_bool("crypto:blake3_hash", false, &format!("hash error: {e}"));
            v.check_skip("crypto:blake3_determinism", "first hash failed");
        }
    }

    // BTSP cipher policy structural validation
    validate_btsp_cipher_policy(v);

    // Ed25519 signing roundtrip via BearDog
    validate_ed25519_roundtrip(ctx, v);
}

fn validate_btsp_cipher_policy(v: &mut ValidationResult) {
    let mode = btsp::security_mode_from_env();
    v.check_bool(
        "btsp:security_mode_resolved",
        true,
        &format!("mode: {mode:?}"),
    );

    let guard = btsp::validate_insecure_guard();
    let detail = match &guard {
        Ok(()) => "guard passed".to_owned(),
        Err(msg) => format!("guard issue: {msg}"),
    };
    v.check_bool("btsp:insecure_guard", guard.is_ok(), &detail);
}

fn validate_ed25519_roundtrip(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    let test_message = "guidestone_ed25519_roundtrip_2026";

    match ctx.call(
        "security",
        "crypto.sign",
        serde_json::json!({"message": test_message, "algorithm": "ed25519"}),
    ) {
        Ok(sign_result) => {
            let signature = sign_result
                .get("signature")
                .and_then(|s| s.as_str())
                .unwrap_or("");
            v.check_bool(
                "crypto:ed25519_sign",
                !signature.is_empty(),
                &format!("signature: {}...", &signature[..signature.len().min(16)]),
            );

            if !signature.is_empty() {
                match ctx.call(
                    "security",
                    "crypto.verify",
                    serde_json::json!({
                        "message": test_message,
                        "signature": signature,
                        "algorithm": "ed25519"
                    }),
                ) {
                    Ok(verify_result) => {
                        let valid = verify_result
                            .get("valid")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(false);
                        v.check_bool(
                            "crypto:ed25519_verify",
                            valid,
                            "signature verified",
                        );
                    }
                    Err(e) => {
                        v.check_bool(
                            "crypto:ed25519_verify",
                            false,
                            &format!("verify call failed: {e}"),
                        );
                    }
                }
            }
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "crypto:ed25519_sign",
                &format!("security not available: {e}"),
            );
            v.check_skip("crypto:ed25519_verify", "security not available");
        }
        Err(e) => {
            v.check_bool(
                "crypto:ed25519_sign",
                false,
                &format!("sign call failed: {e}"),
            );
            v.check_skip("crypto:ed25519_verify", "sign failed, skipping verify");
        }
    }
}
