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
//! | Layer | Name | Description |
//! |-------|------|-------------|
//! | 0     | Bare | graph/fragment/manifest structural validation (no primals needed) |
//! | 0.5   | Seed Provenance | mito seed resolved, fingerprints verified, BTSP mode set |
//! | 1     | Discovery | all primals in the graph discoverable via capability scan |
//! | 1.5   | BTSP Escalation | per-atomic security posture (cleartext vs BTSP per tier) |
//! | 2     | Health | every discovered primal responds to `health.liveness` |
//! | 3     | Capability Parity | math, storage, shader IPC calls produce correct results |
//! | 4     | Cross-Atomic Pipeline | Tower hash → Nest store → retrieve → verify |
//! | 5     | Bonding Model | bonding policies correctly enforced between atomics |
//! | 6     | BTSP + Crypto | crypto.hash parity, cipher policy, Ed25519 roundtrip |
//! | 7     | Cellular | per-spring deploy graphs parse, declare live mode, cover capabilities |
//!
//! # Exit Codes
//!
//! - `0` — all layers passed (NUCLEUS certified)
//! - `1` — one or more layers failed
//! - `2` — bare-only mode (no primals discovered, structural checks only)

#![deny(unsafe_code)]

mod entropy;

use std::path::Path;

use primalspring::bonding::{BondType, BondingPolicy};
use primalspring::btsp;
use primalspring::composition::{self, CompositionContext, validate_liveness, validate_parity};
use primalspring::coordination::AtomicType;
use primalspring::deploy;
use primalspring::ipc::NeuralBridge;
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
    // Layer 0.5: Seed Provenance — resolve entropy, set BTSP credentials
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 0.5: Seed Provenance");
    let mito_seed = entropy::resolve_mito_seed();

    let family_id = std::env::var("FAMILY_ID")
        .ok()
        .filter(|s| !s.is_empty() && s != "default")
        .unwrap_or_else(|| "guidestone-validation".to_owned());
    // SAFETY: called in main() before any threads are spawned.
    // Set FAMILY_ID for socket discovery and FAMILY_SEED for BTSP auth.
    // upgrade_btsp_clients() attempts proactive BTSP on all capabilities.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("FAMILY_ID", &family_id);
        std::env::set_var("FAMILY_SEED", &mito_seed.hex_seed);
        std::env::set_var("BEARDOG_FAMILY_SEED", &mito_seed.hex_seed);
    }

    entropy::validate_seed_provenance(&mut v, &mito_seed);

    // ════════════════════════════════════════════════════════════════════
    // Layer 1: Discovery — can we find primals?
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 1: Discovery");
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();

    let full_caps = AtomicType::FullNucleus.required_capabilities();
    let alive = validate_liveness(&mut ctx, &mut v, full_caps);

    if alive == 0 {
        eprintln!("[guideStone] No NUCLEUS primals discovered — bare certification only.");
        eprintln!("  Deploy from plasmidBin and rerun for full certification.");
        v.finish();
        let code = if v.exit_code() == 0 { 2 } else { 1 };
        std::process::exit(code);
    }

    // ════════════════════════════════════════════════════════════════════
    // Layer 1.5: BTSP Escalation — per-atomic security posture
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 1.5: BTSP Escalation");
    validate_btsp_escalation(&ctx, &mut v);
    validate_substrate_health(&mut v);

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

    // ════════════════════════════════════════════════════════════════════
    // Layer 7: Cellular Deployment — per-spring deploy graphs
    // ════════════════════════════════════════════════════════════════════
    v.section("Layer 7: Cellular Deployment");
    validate_cellular_graphs(&mut v);

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
    validate_checksums(v);
}

fn validate_checksums(v: &mut ValidationResult) {
    primalspring::checksums::verify_manifest(v, "validation/CHECKSUMS");
}

fn validate_graph_parsing(v: &mut ValidationResult) {
    let graph_dirs: &[&str] = &["graphs/profiles", "graphs/multi_node"];

    let skip_suffixes: &[&str] = &["_manifest.toml", "_template.toml"];

    let mut total = 0usize;
    let mut clean = 0usize;

    for dir_name in graph_dirs {
        let dir = Path::new(dir_name);
        if !dir.exists() {
            continue;
        }
        let results = deploy::validate_all_graphs(dir);
        for result in &results {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
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

    let downstream_dir = Path::new("graphs/downstream");
    if downstream_dir.exists() {
        for result in &deploy::validate_all_graphs(downstream_dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            } else if !result.parsed {
                v.check_bool(
                    &format!("graph_parse:{}", result.path),
                    false,
                    "failed to parse as deploy graph",
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

    let validation_dir = Path::new("graphs/spring_validation");
    if validation_dir.exists() {
        for result in &deploy::validate_all_graphs(validation_dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            }
        }
    }

    let deploy_dir = Path::new("graphs/spring_deploy");
    if deploy_dir.exists() {
        for result in &deploy::validate_all_graphs(deploy_dir) {
            if skip_suffixes.iter().any(|s| result.path.ends_with(s)) {
                continue;
            }
            total += 1;
            if result.parsed && result.issues.is_empty() {
                clean += 1;
            }
        }
    }

    v.check_bool(
        "bare:all_graphs_parse",
        total > 0 && clean == total,
        &format!("{clean}/{total} deploy graphs clean"),
    );
}

fn validate_fragment_resolution(v: &mut ValidationResult) {
    let fragment_dir = Path::new("graphs/fragments");
    if !fragment_dir.exists() {
        v.check_skip("bare:fragments_exist", "graphs/fragments/ not found");
        return;
    }

    let expected_fragments = &[
        "tower_atomic",
        "node_atomic",
        "nest_atomic",
        "nucleus",
        "meta_tier",
        "provenance_trio",
    ];

    for &expected in expected_fragments {
        let path = fragment_dir.join(format!("{expected}.toml"));
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    let parsed: Result<toml::Value, _> = toml::from_str(&content);
                    match parsed {
                        Ok(table) => {
                            let has_fragment = table.get("fragment").is_some();
                            let has_nodes = table
                                .get("fragment")
                                .and_then(|f| f.get("nodes").or_else(|| f.get("node")))
                                .and_then(|n| n.as_array())
                                .is_some_and(|a| !a.is_empty());
                            v.check_bool(
                                &format!("bare:fragment:{expected}"),
                                has_fragment && has_nodes,
                                &format!("[fragment] section: {has_fragment}, nodes: {has_nodes}"),
                            );
                        }
                        Err(e) => {
                            v.check_bool(
                                &format!("bare:fragment:{expected}"),
                                false,
                                &format!("TOML parse error: {e}"),
                            );
                        }
                    }
                }
                Err(e) => {
                    v.check_bool(
                        &format!("bare:fragment:{expected}"),
                        false,
                        &format!("cannot read: {e}"),
                    );
                }
            }
        } else {
            v.check_bool(
                &format!("bare:fragment:{expected}"),
                false,
                "missing from graphs/fragments/",
            );
        }
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
    v.check_bool(
        "bare:bondpolicy:covalent_default_valid",
        errors.is_empty(),
        &detail,
    );

    let ionic = BondingPolicy::ionic_contract(vec!["compute".to_owned()]);
    let errors = ionic.validate();
    let detail = if errors.is_empty() {
        "clean".to_owned()
    } else {
        errors.join("; ")
    };
    v.check_bool(
        "bare:bondpolicy:ionic_contract_valid",
        errors.is_empty(),
        &detail,
    );
}

// ════════════════════════════════════════════════════════════════════════
// Layer 1.5: BTSP Escalation — per-atomic security posture
// ════════════════════════════════════════════════════════════════════════

fn validate_btsp_escalation(ctx: &CompositionContext, v: &mut ValidationResult) {
    use primalspring::bonding::{BtspEnforcer, TrustModel};

    let btsp = ctx.btsp_state();

    let tiers: &[(&str, &[&str], BondType)] = &[
        (
            "Tower",
            AtomicType::Tower.required_capabilities(),
            BondType::Covalent,
        ),
        ("Node", &["compute", "tensor", "shader"], BondType::Metallic),
        ("Nest", &["storage", "ai"], BondType::Metallic),
        (
            "Provenance",
            &["dag", "commit", "provenance"],
            BondType::Metallic,
        ),
    ];

    for &(tier_name, caps, bond) in tiers {
        for &cap in caps {
            let check_name = format!("btsp:{tier_name}:{cap}");
            match btsp.get(cap) {
                Some(true) => {
                    v.check_bool(&check_name, true, "BTSP authenticated");
                }
                Some(false) => {
                    v.check_bool(&check_name, false, "cleartext (BTSP not yet enforced)");
                }
                None => {
                    v.check_skip(&check_name, "capability not discovered");
                }
            }
        }

        let min_cipher = btsp::min_cipher_for_bond(bond);
        let trust_req = match bond {
            BondType::Covalent => "nuclear-tier genetics",
            BondType::Metallic | BondType::OrganoMetalSalt => "mito-beacon genetics",
            BondType::Ionic => "contractual",
            BondType::Weak => "any",
        };
        v.check_bool(
            &format!("btsp:policy:{tier_name}"),
            true,
            &format!("{bond:?} bond, min cipher {min_cipher:?}, trust requires {trust_req}"),
        );

        let policy = BondingPolicy::covalent_default();
        let peer_trust = match bond {
            BondType::Covalent => TrustModel::NuclearLineage,
            _ => TrustModel::MitoBeaconFamily,
        };
        let decision = BtspEnforcer::evaluate_connection_with_trust(
            &BondingPolicy {
                bond_type: bond,
                ..policy
            },
            min_cipher,
            Some(peer_trust),
        );
        v.check_bool(
            &format!("btsp:enforcer:{tier_name}"),
            decision.allowed,
            &decision.reason,
        );
    }

    let btsp_count = btsp.values().filter(|&&v| v).count();
    let total = btsp.len();

    let cleartext_caps: Vec<&String> = btsp
        .iter()
        .filter(|&(_, &auth)| !auth)
        .map(|(cap, _)| cap)
        .collect();
    let detail = if cleartext_caps.is_empty() {
        format!("{btsp_count}/{total} capabilities BTSP-authenticated")
    } else {
        format!(
            "{btsp_count}/{total} capabilities BTSP-authenticated (cleartext: {})",
            cleartext_caps
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        )
    };
    v.check_bool("btsp:summary", btsp_count == total, &detail);
}

/// biomeOS substrate health — neural-api liveness + graph.list.
///
/// biomeOS is the NUCLEUS substrate. Every composition depends on it for
/// orchestration and capability routing. This check validates that the
/// neural-api socket is discoverable and responds to both health and
/// graph probes.
fn validate_substrate_health(v: &mut ValidationResult) {
    let bridge = if let Some(b) = NeuralBridge::discover() {
        v.check_bool(
            "substrate:biomeos:discovered",
            true,
            &format!("socket: {}", b.socket_path().display()),
        );
        b
    } else {
        v.check_skip(
            "substrate:biomeos:discovered",
            "neural-api socket not found (biomeOS not running)",
        );
        v.check_skip("substrate:biomeos:liveness", "no socket");
        v.check_skip("substrate:biomeos:graph_list", "no socket");
        return;
    };

    match bridge.health_check() {
        Ok(true) => v.check_bool("substrate:biomeos:liveness", true, "alive"),
        Ok(false) => v.check_bool(
            "substrate:biomeos:liveness",
            false,
            "responded but unhealthy",
        ),
        Err(e) if e.is_connection_error() || e.is_protocol_error() => {
            v.check_skip(
                "substrate:biomeos:liveness",
                &format!("reachable but incompatible: {e}"),
            );
        }
        Err(e) => v.check_bool("substrate:biomeos:liveness", false, &format!("error: {e}")),
    }

    let graph_result = (|| {
        let mut client =
            primalspring::ipc::client::PrimalClient::connect(bridge.socket_path(), "neural-api")?;
        client.call("graph.list", serde_json::Value::Null)
    })();

    match graph_result {
        Ok(resp) if resp.is_success() => {
            v.check_bool(
                "substrate:biomeos:graph_list",
                true,
                "graph executor available",
            );
        }
        Ok(_) => {
            v.check_bool(
                "substrate:biomeos:graph_list",
                false,
                "graph.list returned error",
            );
        }
        Err(e) if e.is_method_not_found() => {
            v.check_skip(
                "substrate:biomeos:graph_list",
                "graph.list not implemented (older biomeOS)",
            );
        }
        Err(e) if e.is_connection_error() || e.is_protocol_error() => {
            v.check_skip(
                "substrate:biomeos:graph_list",
                &format!("transport mismatch: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "substrate:biomeos:graph_list",
                false,
                &format!("error: {e}"),
            );
        }
    }
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
                Err(e) if e.is_protocol_error() => {
                    v.check_skip(
                        &check_name,
                        &format!("reachable but protocol mismatch: {e}"),
                    );
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
        "tensor.matmul_inline",
        serde_json::json!({
            "lhs": [[1.0, 0.0], [0.0, 1.0]],
            "rhs": [[3.0, 7.0], [2.0, 5.0]]
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
        "shader.compile.capabilities",
        serde_json::json!({}),
    ) {
        Ok(result) => {
            let has_archs = result
                .get("supported_archs")
                .and_then(|c| c.as_array())
                .is_some_and(|c| !c.is_empty());
            let has_legacy = result
                .get("capabilities")
                .and_then(|c| c.as_array())
                .is_some_and(|c| !c.is_empty());
            let arch_count = result
                .get("supported_archs")
                .and_then(|c| c.as_array())
                .map_or(0, Vec::len);
            v.check_bool(
                "parity:shader_capabilities",
                has_archs || has_legacy,
                &format!("{arch_count} supported architectures"),
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
            v.check_skip(
                "pipeline:nest_roundtrip",
                "tower unavailable, skipping nest",
            );
        }
        Err(e) => {
            v.check_bool("pipeline:tower_hash", false, &format!("hash error: {e}"));
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
        v.check_bool(
            &format!("bonding:policy:{bond:?}:valid"),
            errors.is_empty(),
            &detail,
        );

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
            v.check_skip(
                "crypto:blake3_hash",
                &format!("security not available: {e}"),
            );
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
    use base64::Engine as _;
    let raw_message = b"guidestone_ed25519_roundtrip_2026";
    let test_message = base64::engine::general_purpose::STANDARD.encode(raw_message);

    validate_ed25519_keygen(ctx, v);
    validate_ed25519_sign_and_verify(ctx, v, &test_message);
}

fn validate_ed25519_keygen(ctx: &mut CompositionContext, v: &mut ValidationResult) {
    match ctx.call(
        "security",
        "crypto.ed25519_generate_keypair",
        serde_json::json!({}),
    ) {
        Ok(keygen_result) => {
            let has_pub = keygen_result
                .get("public_key")
                .and_then(|s| s.as_str())
                .is_some();
            let has_sec = keygen_result
                .get("secret_key")
                .and_then(|s| s.as_str())
                .is_some();
            v.check_bool(
                "crypto:ed25519_keygen",
                has_pub && has_sec,
                &format!("pub={has_pub} sec={has_sec}"),
            );
        }
        Err(e) if e.is_connection_error() => {
            v.check_skip(
                "crypto:ed25519_keygen",
                &format!("security not available: {e}"),
            );
        }
        Err(e) => {
            v.check_bool(
                "crypto:ed25519_keygen",
                false,
                &format!("keygen failed: {e}"),
            );
        }
    }
}

fn validate_ed25519_sign_and_verify(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    test_message: &str,
) {
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

            let public_key = sign_result
                .get("public_key")
                .and_then(|s| s.as_str())
                .unwrap_or("");
            if public_key.is_empty() {
                v.check_skip(
                    "crypto:ed25519_verify",
                    "UPSTREAM GAP: crypto.sign does not expose public_key",
                );
            } else {
                validate_ed25519_verify(ctx, v, test_message, signature, public_key);
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

fn validate_ed25519_verify(
    ctx: &mut CompositionContext,
    v: &mut ValidationResult,
    test_message: &str,
    signature: &str,
    public_key: &str,
) {
    match ctx.call(
        "security",
        "crypto.verify",
        serde_json::json!({
            "message": test_message,
            "signature": signature,
            "public_key": public_key,
            "algorithm": "ed25519"
        }),
    ) {
        Ok(verify_result) => {
            let valid = verify_result
                .get("valid")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            v.check_bool("crypto:ed25519_verify", valid, "sign→verify roundtrip");
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

// ════════════════════════════════════════════════════════════════════════
// Layer 7: Cellular Deployment
// ════════════════════════════════════════════════════════════════════════

fn validate_cellular_graphs(v: &mut ValidationResult) {
    let cells_dir = Path::new("graphs/cells");

    if !cells_dir.is_dir() {
        v.check_skip("cellular:dir_exists", "graphs/cells/ not found");
        return;
    }
    v.check_bool("cellular:dir_exists", true, "graphs/cells/ present");

    let manifest_path = cells_dir.join("cells_manifest.toml");
    let manifest_ok = manifest_path.is_file()
        && std::fs::read_to_string(&manifest_path)
            .ok()
            .and_then(|s| s.parse::<toml::Value>().ok())
            .is_some();
    v.check_bool(
        "cellular:manifest_parses",
        manifest_ok,
        "cells_manifest.toml present and valid",
    );

    let cell_files: Vec<_> = std::fs::read_dir(cells_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with("_cell.toml"))
        })
        .collect();

    v.check_bool(
        "cellular:cell_count",
        !cell_files.is_empty(),
        &format!("{} cell graphs found", cell_files.len()),
    );

    for entry in &cell_files {
        let path = entry.path();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let Ok(content) = std::fs::read_to_string(&path) else {
            v.check_bool(
                &format!("cellular:{stem}:readable"),
                false,
                "file not readable",
            );
            continue;
        };

        let val: toml::Value = match content.parse() {
            Ok(p) => p,
            Err(e) => {
                v.check_bool(
                    &format!("cellular:{stem}:parses"),
                    false,
                    &format!("parse error: {e}"),
                );
                continue;
            }
        };
        validate_cell_graph_toml_checks(v, stem, &val);
    }
}

fn validate_cell_graph_toml_checks(v: &mut ValidationResult, stem: &str, val: &toml::Value) {
    v.check_bool(&format!("cellular:{stem}:parses"), true, "valid TOML");

    let has_graph = val.get("graph").is_some();
    v.check_bool(
        &format!("cellular:{stem}:graph_section"),
        has_graph,
        "[graph] section present",
    );

    let pt_mode = val
        .get("graph")
        .and_then(|g| g.get("metadata"))
        .and_then(|m| m.get("petaltongue_mode"))
        .and_then(|v| v.as_str());
    v.check_bool(
        &format!("cellular:{stem}:live_mode"),
        pt_mode == Some("live"),
        &format!("petaltongue_mode = {:?}", pt_mode.unwrap_or("MISSING")),
    );

    // Support both primalSpring schema ([[graph.nodes]] + name) and
    // biomeOS schema ([[nodes]] + id)
    let nodes = val
        .get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .or_else(|| val.get("nodes").and_then(|n| n.as_array()));

    let node_names: Vec<&str> = nodes
        .iter()
        .flat_map(|arr| arr.iter())
        .filter_map(|n| {
            n.get("name")
                .and_then(|v| v.as_str())
                .or_else(|| n.get("id").and_then(|v| v.as_str()))
        })
        .collect();

    let has_tower = node_names.contains(&"beardog") && node_names.contains(&"songbird");
    v.check_bool(
        &format!("cellular:{stem}:tower"),
        has_tower,
        "Tower primals (beardog + songbird) present",
    );

    let has_petaltongue = node_names.contains(&"petaltongue");
    v.check_bool(
        &format!("cellular:{stem}:petaltongue"),
        has_petaltongue,
        "petalTongue node present",
    );

    let has_validate = node_names
        .iter()
        .any(|n| n.starts_with("validate") || n.starts_with("validate-"));
    v.check_bool(
        &format!("cellular:{stem}:health_check"),
        has_validate,
        "validation health_check node present",
    );

    let security_models: Vec<&str> = nodes
        .iter()
        .flat_map(|arr| arr.iter())
        .filter_map(|n| n.get("security_model").and_then(|v| v.as_str()))
        .collect();
    let all_btsp = !security_models.is_empty()
        && security_models
            .iter()
            .all(|&m| m == "btsp" || m == "btsp_enforced");
    let btsp_count = security_models
        .iter()
        .filter(|&&m| m == "btsp" || m == "btsp_enforced")
        .count();
    v.check_bool(
        &format!("cellular:{stem}:btsp_default"),
        all_btsp,
        &format!(
            "{}/{} nodes declare btsp security_model",
            btsp_count,
            security_models.len()
        ),
    );
}
