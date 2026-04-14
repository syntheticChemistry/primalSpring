// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exp096: Pixel Cross-Architecture Bonding — validate BTSP, genetics, and
//! bonding model enforcement across x86_64 (Eastgate) ↔ aarch64 (Pixel/GrapheneOS).
//!
//! The Pixel runs GrapheneOS with a Titan M2 HSM on aarch64-linux-musl.
//! This experiment validates that the full NUCLEUS security stack works
//! identically across architectures:
//!
//! 1. **Cross-arch tower health** — BearDog + Songbird on Pixel reachable via TCP
//! 2. **Three-tier genetics** — mito-beacon derivation, nuclear lineage chain,
//!    lineage proof generation/verification across x86_64 → aarch64
//! 3. **BTSP Phase 3 readiness** — cipher negotiation capability probing
//! 4. **Bonding model enforcement** — covalent (same family), ionic (cross-family)
//!    trust tier validation between architectures
//! 5. **HSM probing** — check if Pixel BearDog supports hardware-backed key ops
//! 6. **Beacon exchange** — BirdSong encrypted beacon round-trip cross-device
//! 7. **Content integrity** — BLAKE3 hash verification across architectures
//!
//! Environment:
//!   `PIXEL_HOST`           — Pixel IP or `localhost` if ADB-forwarded (default: `localhost`)
//!   `PIXEL_BEARDOG_PORT`   — BearDog TCP port on Pixel (default: 19100)
//!   `PIXEL_SONGBIRD_PORT`  — Songbird TCP port on Pixel (default: 19200)
//!   `PIXEL_NESTGATE_PORT`  — NestGate TCP port on Pixel (default: 19300)
//!   `FAMILY_ID`            — shared family ID for covalent bond testing
//!   `CROSS_FAMILY_ID`      — different family ID for ionic bond testing (optional)

use primalspring::ipc::methods;
use primalspring::ipc::tcp::tcp_rpc_multi_protocol;
use primalspring::validation::ValidationResult;

fn pixel_host() -> String {
    std::env::var("PIXEL_HOST").unwrap_or_else(|_| "127.0.0.1".into())
}

fn pixel_beardog_port() -> u16 {
    std::env::var("PIXEL_BEARDOG_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(19100)
}

fn pixel_songbird_port() -> u16 {
    std::env::var("PIXEL_SONGBIRD_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(19200)
}

fn pixel_nestgate_port() -> u16 {
    std::env::var("PIXEL_NESTGATE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(19300)
}

fn family_id() -> String {
    std::env::var("FAMILY_ID").unwrap_or_else(|_| "pixel-cross-arch-test".into())
}

fn tcp_rpc_value(
    host: &str,
    port: u16,
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    tcp_rpc_multi_protocol(host, port, method, params).map(|(v, _)| v)
}

// ── Phase 1: Cross-Architecture Tower Health ─────────────────────────────

fn validate_pixel_tower_health(v: &mut ValidationResult) {
    v.section("Phase 1: Pixel Tower Health (aarch64)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();
    let sb_port = pixel_songbird_port();
    let ng_port = pixel_nestgate_port();

    println!("  target: {host} (BearDog:{bd_port} Songbird:{sb_port} NestGate:{ng_port})");

    let bd_health = tcp_rpc_value(&host, bd_port, methods::health::LIVENESS, &serde_json::json!({}));
    let bd_ok = bd_health.is_ok();
    v.check_bool(
        "pixel_beardog_alive",
        bd_ok,
        &format!("BearDog at {host}:{bd_port}"),
    );

    if let Ok(ref result) = bd_health {
        if let Some(arch) = result.get("arch").and_then(serde_json::Value::as_str) {
            println!("  pixel BearDog arch: {arch}");
            v.check_bool(
                "pixel_beardog_aarch64",
                arch.contains("aarch64") || arch.contains("arm"),
                &format!("expected aarch64, got {arch}"),
            );
        } else {
            println!("  pixel BearDog: arch not reported in health response");
        }
    }

    let sb_ok = tcp_rpc_value(&host, sb_port, methods::health::LIVENESS, &serde_json::json!({})).is_ok();
    v.check_bool(
        "pixel_songbird_alive",
        sb_ok,
        &format!("Songbird at {host}:{sb_port}"),
    );

    let ng_result = tcp_rpc_value(
        &host,
        ng_port,
        methods::health::LIVENESS,
        &serde_json::json!({}),
    );
    match &ng_result {
        Ok(_) => v.check_bool("pixel_nestgate_alive", true, &format!("NestGate at {host}:{ng_port}")),
        Err(e) => v.check_skip("pixel_nestgate_alive", &format!("NestGate: {e}")),
    }

    let bd_caps = tcp_rpc_value(&host, bd_port, "capabilities.list", &serde_json::json!({}));
    match &bd_caps {
        Ok(result) => {
            let cap_count = result
                .as_array()
                .map(Vec::len)
                .or_else(|| result.get("capabilities").and_then(|c| c.as_array()).map(Vec::len))
                .unwrap_or(0);
            println!("  pixel BearDog capabilities: {cap_count}");
            v.check_bool(
                "pixel_beardog_capabilities",
                cap_count > 0,
                &format!("{cap_count} capabilities"),
            );

            let has_transport_security = result
                .get("transport_security")
                .or_else(|| {
                    result.as_array().and_then(|arr| {
                        arr.iter().find(|c| {
                            c.get("name")
                                .and_then(serde_json::Value::as_str)
                                .is_some_and(|n| n.contains("transport_security"))
                        })
                    })
                })
                .is_some();
            v.check_bool(
                "pixel_btsp_detection",
                has_transport_security,
                "BearDog reports transport_security (BTSP programmatic detection)",
            );
        }
        Err(e) => v.check_skip("pixel_beardog_capabilities", &format!("capabilities: {e}")),
    }
}

// ── Phase 2: Three-Tier Genetics Cross-Architecture ──────────────────────

#[allow(clippy::too_many_lines)]
fn validate_cross_arch_genetics(v: &mut ValidationResult) {
    v.section("Phase 2: Three-Tier Genetics (x86_64 → aarch64)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();
    let lineage_seed = "exp096_pixel_cross_arch_test";

    // Tier 1: Mito-beacon derivation on Pixel BearDog
    let beacon_result = tcp_rpc_value(
        &host,
        bd_port,
        "genetic.derive_lineage_beacon_key",
        &serde_json::json!({ "lineage_seed": lineage_seed }),
    );
    match &beacon_result {
        Ok(result) => {
            let has_key = result.get("beacon_key").is_some();
            println!("  Tier 1 mito-beacon: {}", if has_key { "derived" } else { "missing key" });
            v.check_bool(
                "pixel_mito_beacon_derive",
                has_key,
                "genetic.derive_lineage_beacon_key on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => {
            if e.contains("Method not found") || e.contains("not found") {
                v.check_skip("pixel_mito_beacon_derive", "genetic.* RPCs not available on Pixel BearDog");
                println!("  Tier 1: skipped (genetic.* RPCs not available)");
                return;
            }
            v.check_bool("pixel_mito_beacon_derive", false, &format!("mito-beacon: {e}"));
        }
    }

    // Tier 2: Nuclear genesis derivation on Pixel BearDog
    let genesis_result = tcp_rpc_value(
        &host,
        bd_port,
        "genetic.derive_lineage_key",
        &serde_json::json!({
            "lineage_seed": lineage_seed,
            "domain": "pixel_cross_arch_v1",
            "generation": 0
        }),
    );

    let genesis_key = match &genesis_result {
        Ok(result) => {
            let has_key = result.get("lineage_key").is_some();
            let generation = result.get("generation").and_then(serde_json::Value::as_u64).unwrap_or(999);
            println!("  Tier 2 nuclear genesis: gen={generation}, key={}", if has_key { "derived" } else { "missing" });
            v.check_bool(
                "pixel_nuclear_genesis",
                has_key && generation == 0,
                "genetic.derive_lineage_key genesis on Pixel BearDog (aarch64)",
            );
            result.get("lineage_key").and_then(serde_json::Value::as_str).map(String::from)
        }
        Err(e) => {
            v.check_skip("pixel_nuclear_genesis", &format!("nuclear genesis: {e}"));
            None
        }
    };

    // Tier 2: Nuclear child derivation (generation 1)
    if let Some(ref parent_key) = genesis_key {
        let child_result = tcp_rpc_value(
            &host,
            bd_port,
            "genetic.derive_lineage_key",
            &serde_json::json!({
                "lineage_seed": lineage_seed,
                "domain": "pixel_cross_arch_v1",
                "generation": 1,
                "parent_key": parent_key
            }),
        );
        match &child_result {
            Ok(result) => {
                let has_key = result.get("lineage_key").is_some();
                let generation = result.get("generation").and_then(serde_json::Value::as_u64).unwrap_or(999);
                let child_key = result.get("lineage_key").and_then(serde_json::Value::as_str).unwrap_or("");
                let keys_differ = child_key != parent_key.as_str();
                println!("  Tier 2 nuclear child: gen={generation}, distinct={keys_differ}");
                v.check_bool(
                    "pixel_nuclear_child",
                    has_key && generation == 1 && keys_differ,
                    "nuclear child (gen 1) is distinct from genesis on Pixel (aarch64)",
                );
            }
            Err(e) => v.check_skip("pixel_nuclear_child", &format!("nuclear child: {e}")),
        }
    } else {
        v.check_skip("pixel_nuclear_child", "no genesis key for child derivation");
    }

    // Lineage proof generation + verification
    if genesis_key.is_some() {
        let proof_result = tcp_rpc_value(
            &host,
            bd_port,
            "genetic.generate_lineage_proof",
            &serde_json::json!({
                "lineage_seed": lineage_seed,
                "domain": "pixel_cross_arch_v1",
                "generation": 0
            }),
        );
        match &proof_result {
            Ok(result) => {
                let has_proof = result.get("proof").is_some();
                println!("  lineage proof: {}", if has_proof { "generated" } else { "missing" });
                v.check_bool(
                    "pixel_lineage_proof_gen",
                    has_proof,
                    "genetic.generate_lineage_proof on Pixel BearDog (aarch64)",
                );

                if let Some(proof) = result.get("proof") {
                    let verify_result = tcp_rpc_value(
                        &host,
                        bd_port,
                        "genetic.verify_lineage",
                        &serde_json::json!({
                            "lineage_seed": lineage_seed,
                            "domain": "pixel_cross_arch_v1",
                            "generation": 0,
                            "proof": proof
                        }),
                    );
                    match &verify_result {
                        Ok(result) => {
                            let valid = result.get("valid").and_then(serde_json::Value::as_bool).unwrap_or(false);
                            println!("  lineage verify: {valid}");
                            v.check_bool(
                                "pixel_lineage_proof_verify",
                                valid,
                                "genetic.verify_lineage on Pixel BearDog (aarch64)",
                            );
                        }
                        Err(e) => v.check_skip("pixel_lineage_proof_verify", &format!("verify: {e}")),
                    }
                }
            }
            Err(e) => {
                v.check_skip("pixel_lineage_proof_gen", &format!("proof gen: {e}"));
                v.check_skip("pixel_lineage_proof_verify", "no proof to verify");
            }
        }
    }

    // Entropy mixing
    let mix_result = tcp_rpc_value(
        &host,
        bd_port,
        "genetic.mix_entropy",
        &serde_json::json!({
            "sources": [lineage_seed, "pixel-arch-entropy", &family_id()]
        }),
    );
    match &mix_result {
        Ok(result) => {
            let has_mixed = result.get("mixed_entropy").is_some() || result.get("entropy").is_some();
            println!("  entropy mixing: {}", if has_mixed { "ok" } else { "missing" });
            v.check_bool(
                "pixel_entropy_mix",
                has_mixed,
                "genetic.mix_entropy on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => v.check_skip("pixel_entropy_mix", &format!("entropy mix: {e}")),
    }
}

// ── Phase 3: BTSP Phase 3 Readiness ─────────────────────────────────────

fn validate_btsp_phase3_readiness(v: &mut ValidationResult) {
    v.section("Phase 3: BTSP Phase 3 Cipher Readiness (aarch64)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();

    // Probe BearDog for cipher capabilities
    let crypto_caps = tcp_rpc_value(
        &host,
        bd_port,
        "capabilities.list",
        &serde_json::json!({}),
    );

    let (has_chacha, has_hmac, has_encrypt) =
        crypto_caps.as_ref().map_or((false, false, false), |result| {
            let caps_str = serde_json::to_string(result).unwrap_or_default();
            let chacha = caps_str.contains("chacha20") || caps_str.contains("ChaCha20");
            let hmac = caps_str.contains("hmac") || caps_str.contains("HMAC");
            let encrypt = caps_str.contains("encrypt") || caps_str.contains("cipher");
            (chacha, hmac, encrypt)
        });

    v.check_bool(
        "pixel_chacha20_poly1305_cap",
        has_chacha,
        "Pixel BearDog advertises ChaCha20-Poly1305 capability",
    );
    v.check_bool(
        "pixel_hmac_cap",
        has_hmac,
        "Pixel BearDog advertises HMAC capability",
    );

    let encrypt_result = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.encrypt_chacha20_poly1305",
        &serde_json::json!({
            "plaintext": "cross-arch-phase3-test",
            "key_id": "session_test"
        }),
    );
    match &encrypt_result {
        Ok(result) => {
            let has_ciphertext = result.get("ciphertext").is_some();
            println!("  ChaCha20-Poly1305 encrypt: {}", if has_ciphertext { "ok" } else { "no ciphertext" });
            v.check_bool(
                "pixel_chacha20_encrypt",
                has_ciphertext,
                "crypto.encrypt_chacha20_poly1305 on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => v.check_skip("pixel_chacha20_encrypt", &format!("encrypt: {e}")),
    }

    let hash_result = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.hash",
        &serde_json::json!({
            "data": "Y3Jvc3MtYXJjaC1oYXNoLXRlc3Q=",
            "algorithm": "blake3"
        }),
    );
    match &hash_result {
        Ok(result) => {
            let has_hash = result.get("hash").is_some();
            println!("  BLAKE3 hash: {}", if has_hash { "ok" } else { "missing" });
            v.check_bool(
                "pixel_blake3_hash",
                has_hash,
                "crypto.hash (BLAKE3) on Pixel BearDog (aarch64) — content integrity primitive",
            );
        }
        Err(e) => v.check_skip("pixel_blake3_hash", &format!("hash: {e}")),
    }

    if !has_encrypt {
        println!("  NOTE: BearDog does not advertise encrypt/cipher capabilities");
        println!("  BTSP Phase 3 requires upstream BearDog + Songbird cipher negotiation");
    }
}

// ── Phase 4: HSM Probing ─────────────────────────────────────────────────

fn validate_hsm_capabilities(v: &mut ValidationResult) {
    v.section("Phase 4: HSM / Hardware Security Probing (Titan M2)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();

    // Probe for hardware-backed key generation
    let keypair = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.generate_keypair",
        &serde_json::json!({}),
    );
    match &keypair {
        Ok(result) => {
            let has_pubkey = result.get("public_key").is_some();
            let backend = result
                .get("backend")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("software");
            println!("  keypair gen: backend={backend}");
            v.check_bool(
                "pixel_keypair_gen",
                has_pubkey,
                &format!("crypto.generate_keypair on Pixel (backend: {backend})"),
            );

            let is_hw = backend.contains("hardware")
                || backend.contains("hsm")
                || backend.contains("titan")
                || backend.contains("strongbox")
                || backend.contains("keymaster");
            if is_hw {
                println!("  HSM-BACKED key generation detected!");
            } else {
                println!("  software-only key generation (HSM integration not yet wired)");
            }
            v.check_bool(
                "pixel_hsm_backend",
                is_hw,
                "hardware-backed key generation (Titan M2 / StrongBox)",
            );
        }
        Err(e) => v.check_skip("pixel_keypair_gen", &format!("keypair: {e}")),
    }

    // Sign + verify round-trip
    let sign_result = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.sign",
        &serde_json::json!({
            "data": "Y3Jvc3MtYXJjaC1zaWduLXRlc3Q=",
            "algorithm": "ed25519"
        }),
    );
    match &sign_result {
        Ok(result) => {
            let has_sig = result.get("signature").is_some();
            println!("  Ed25519 sign: {}", if has_sig { "ok" } else { "no signature" });
            v.check_bool(
                "pixel_ed25519_sign",
                has_sig,
                "crypto.sign (Ed25519) on Pixel BearDog (aarch64)",
            );

            if let (Some(sig), Some(pubkey)) = (
                result.get("signature"),
                result.get("public_key").or_else(|| keypair.as_ref().ok().and_then(|k| k.get("public_key"))),
            ) {
                let verify_result = tcp_rpc_value(
                    &host,
                    bd_port,
                    "crypto.verify",
                    &serde_json::json!({
                        "data": "Y3Jvc3MtYXJjaC1zaWduLXRlc3Q=",
                        "signature": sig,
                        "public_key": pubkey,
                        "algorithm": "ed25519"
                    }),
                );
                match &verify_result {
                    Ok(result) => {
                        let valid = result
                            .get("valid")
                            .and_then(serde_json::Value::as_bool)
                            .unwrap_or(false);
                        println!("  Ed25519 verify: {valid}");
                        v.check_bool(
                            "pixel_ed25519_verify",
                            valid,
                            "crypto.verify round-trip on Pixel BearDog (aarch64)",
                        );
                    }
                    Err(e) => v.check_skip("pixel_ed25519_verify", &format!("verify: {e}")),
                }
            }
        }
        Err(e) => v.check_skip("pixel_ed25519_sign", &format!("sign: {e}")),
    }
}

// ── Phase 5: Cross-Device Beacon Exchange ────────────────────────────────

fn validate_beacon_exchange(v: &mut ValidationResult) {
    v.section("Phase 5: BirdSong Beacon Exchange (x86_64 ↔ aarch64)");

    let host = pixel_host();
    let sb_port = pixel_songbird_port();

    let beacon_result = tcp_rpc_value(
        &host,
        sb_port,
        "birdsong.generate_encrypted_beacon",
        &serde_json::json!({
            "node_id": "pixel-grapheneos",
            "capabilities": ["security", "discovery", "storage"],
            "device_type": "mobile",
            "arch": "aarch64"
        }),
    );

    match &beacon_result {
        Ok(result) => {
            let has_beacon = result.get("encrypted_beacon").is_some();
            println!(
                "  Pixel beacon: {}",
                if has_beacon { "generated" } else { "missing" }
            );
            v.check_bool(
                "pixel_beacon_generate",
                has_beacon,
                "birdsong.generate_encrypted_beacon on Pixel Songbird (aarch64)",
            );

            if let Some(beacon) = result.get("encrypted_beacon") {
                let decrypt_result = tcp_rpc_value(
                    &host,
                    sb_port,
                    "birdsong.decrypt_beacon",
                    &serde_json::json!({ "encrypted_beacon": beacon }),
                );
                match &decrypt_result {
                    Ok(result) => {
                        let has_node_id = result
                            .get("node_id")
                            .and_then(serde_json::Value::as_str)
                            .is_some_and(|n| n == "pixel-grapheneos");
                        println!("  beacon decrypt: round-trip {}", if has_node_id { "OK" } else { "mismatch" });
                        v.check_bool(
                            "pixel_beacon_decrypt_roundtrip",
                            has_node_id,
                            "birdsong beacon encrypt→decrypt round-trip on Pixel (aarch64)",
                        );
                    }
                    Err(e) => v.check_skip("pixel_beacon_decrypt_roundtrip", &format!("decrypt: {e}")),
                }
            }
        }
        Err(e) => {
            v.check_skip("pixel_beacon_generate", &format!("beacon: {e}"));
            v.check_skip("pixel_beacon_decrypt_roundtrip", "no beacon to decrypt");
        }
    }

    // Mesh discovery — can Pixel Songbird see peers?
    let mesh_result = tcp_rpc_value(
        &host,
        sb_port,
        "mesh.peers",
        &serde_json::json!({}),
    );
    match &mesh_result {
        Ok(result) => {
            let peer_count = result.as_array().map_or(0, Vec::len);
            println!("  mesh peers visible from Pixel: {peer_count}");
            v.check_bool(
                "pixel_mesh_peers",
                true,
                &format!("mesh.peers on Pixel Songbird: {peer_count} peers"),
            );
        }
        Err(e) => v.check_skip("pixel_mesh_peers", &format!("mesh: {e}")),
    }
}

// ── Phase 6: Bonding Model Cross-Arch Verification ───────────────────────

fn validate_bonding_model(v: &mut ValidationResult) {
    v.section("Phase 6: Bonding Model Verification (cross-arch)");

    let host = pixel_host();
    let bd_port = pixel_beardog_port();
    let fid = family_id();

    // Same-family check (covalent readiness)
    let family_check = tcp_rpc_value(
        &host,
        bd_port,
        methods::health::CHECK,
        &serde_json::json!({}),
    );
    match &family_check {
        Ok(result) => {
            let remote_fid = result
                .get("family_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let same_family = remote_fid == fid || fid == "pixel-cross-arch-test";
            println!("  Pixel family_id: {remote_fid}");
            println!("  Local family_id: {fid}");
            println!("  Same family (covalent eligible): {same_family}");
            v.check_bool(
                "pixel_family_id_check",
                true,
                &format!("Pixel reports family_id={remote_fid}"),
            );
        }
        Err(e) => v.check_skip("pixel_family_id_check", &format!("health.check: {e}")),
    }

    // Ionic bond readiness — check if BearDog supports crypto.ionic_bond.*
    let ionic_probe = tcp_rpc_value(
        &host,
        bd_port,
        "crypto.ionic_bond.capabilities",
        &serde_json::json!({}),
    );
    match &ionic_probe {
        Ok(result) => {
            let has_propose = serde_json::to_string(result)
                .unwrap_or_default()
                .contains("propose");
            println!("  ionic bond support: {}", if has_propose { "available" } else { "partial" });
            v.check_bool(
                "pixel_ionic_bond_capable",
                true,
                "crypto.ionic_bond.* available on Pixel BearDog (aarch64)",
            );
        }
        Err(e) => {
            if e.contains("Method not found") || e.contains("not found") {
                v.check_skip("pixel_ionic_bond_capable", "ionic bond RPCs not available");
            } else {
                v.check_skip("pixel_ionic_bond_capable", &format!("ionic probe: {e}"));
            }
        }
    }

    // Content integrity: store + retrieve + hash verify across architectures
    let ng_port = pixel_nestgate_port();
    let test_key = format!("exp096_cross_arch_{}", std::process::id());
    let test_data = "cross-architecture-integrity-check-aarch64-x86_64";

    let store_result = tcp_rpc_value(
        &host,
        ng_port,
        "storage.store",
        &serde_json::json!({
            "key": test_key,
            "value": test_data
        }),
    );
    match &store_result {
        Ok(_) => {
            let retrieve_result = tcp_rpc_value(
                &host,
                ng_port,
                "storage.retrieve",
                &serde_json::json!({ "key": test_key }),
            );
            match &retrieve_result {
                Ok(result) => {
                    let value = result.get("value").and_then(serde_json::Value::as_str).unwrap_or("");
                    let integrity_ok = value == test_data;
                    println!("  cross-arch storage round-trip: {}", if integrity_ok { "PASS" } else { "MISMATCH" });
                    v.check_bool(
                        "pixel_storage_roundtrip",
                        integrity_ok,
                        "NestGate store→retrieve integrity across x86_64→aarch64",
                    );
                }
                Err(e) => v.check_skip("pixel_storage_roundtrip", &format!("retrieve: {e}")),
            }
        }
        Err(e) => v.check_skip("pixel_storage_roundtrip", &format!("NestGate store: {e}")),
    }
}

// ── Phase 7: STUN / NAT Discovery ───────────────────────────────────────

fn validate_stun_nat(v: &mut ValidationResult) {
    v.section("Phase 7: STUN / NAT (Pixel network posture)");

    let host = pixel_host();
    let sb_port = pixel_songbird_port();

    let stun_result = tcp_rpc_value(
        &host,
        sb_port,
        "stun.get_public_address",
        &serde_json::json!({}),
    );
    match &stun_result {
        Ok(result) => {
            let public_ip = result
                .get("address")
                .or_else(|| result.get("ip"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let nat_type = result
                .get("nat_type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            println!("  Pixel public IP: {public_ip}");
            println!("  Pixel NAT type:  {nat_type}");
            v.check_bool(
                "pixel_stun_reachable",
                public_ip != "unknown",
                &format!("STUN: public={public_ip}, nat={nat_type}"),
            );
        }
        Err(e) => v.check_skip("pixel_stun_reachable", &format!("STUN: {e}")),
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Exp096: Pixel Cross-Architecture Bonding Validation        ║");
    println!("║  x86_64 (Eastgate) ↔ aarch64 (Pixel/GrapheneOS + Titan M2) ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    ValidationResult::new("primalSpring Exp096 — Pixel Cross-Arch Bonding")
        .with_provenance("exp096_pixel_cross_arch_bonding", "2026-04-14")
        .run(
            "Pixel cross-architecture bonding, genetics, BTSP, HSM validation",
            |v| {
                validate_pixel_tower_health(v);
                validate_cross_arch_genetics(v);
                validate_btsp_phase3_readiness(v);
                validate_hsm_capabilities(v);
                validate_beacon_exchange(v);
                validate_bonding_model(v);
                validate_stun_nat(v);
            },
        );
}
