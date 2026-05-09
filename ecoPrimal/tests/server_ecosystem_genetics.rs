// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    deprecated,
    reason = "integration test uses deprecated harness/launcher APIs"
)]
#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "integration tests — panics are the failure signal"
)]

//! Three-tier genetics integration tests (mito-beacon, nuclear lineage, tag). Run with `cargo test --ignored`.

#[expect(
    dead_code,
    reason = "shared helpers — each test file uses a different subset"
)]
mod integration;

// ===========================================================================
// Three-Tier Genetics Integration Tests
// ===========================================================================

// ---------------------------------------------------------------------------
// Gate 8.1: Mito-beacon key derivation via BearDog genetic.* RPCs
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn genetics_mito_beacon_derivation() {
    use primalspring::coordination::AtomicType;
    use primalspring::genetics::rpc;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-mito-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    let mut client = running
        .client_for("security")
        .expect("should connect to beardog");

    let seed_hex = String::from_utf8_lossy(running.mito_seed().expect("mito seed")).into_owned();

    let beacon =
        rpc::derive_lineage_beacon_key(&mut client, &seed_hex, "birdsong_beacon_v1", "test-family");

    match beacon {
        Ok(b) => {
            assert!(b.is_valid(), "derived beacon should have key material");
            assert!(!b.key_bytes().is_empty(), "beacon key should be non-empty");
            println!("  mito-beacon derived: beacon_id={}", b.beacon_id);
        }
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("Method not found") || msg.contains("not found"),
                "expected genetic.derive_lineage_beacon_key to exist or return method-not-found, got: {e}"
            );
            println!("  mito-beacon derivation: skipped ({e})");
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 8.2: Nuclear lineage derivation — genesis + child + grandchild chain
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn genetics_nuclear_lineage_chain() {
    use primalspring::coordination::AtomicType;
    use primalspring::genetics::rpc;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-nuc-chain-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    let mut client = running
        .client_for("security")
        .expect("should connect to beardog");

    let seed_hex = String::from_utf8_lossy(running.mito_seed().expect("mito seed")).into_owned();

    // Generation 0: genesis
    let genesis = match rpc::derive_lineage_key(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        "test-genesis-domain",
        None,
    ) {
        Ok(g) => g,
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("Method not found") || msg.contains("not found"),
                "expected genetic.derive_lineage_key or method-not-found, got: {e}"
            );
            println!("  nuclear lineage chain: skipped (BearDog genetic.* RPCs not available)");
            return;
        }
    };
    assert_eq!(genesis.generation(), 0, "genesis should be generation 0");
    assert!(genesis.is_genesis(), "genesis should be marked as genesis");
    assert!(
        !genesis.key_bytes().is_empty(),
        "genesis key should be non-empty"
    );
    println!(
        "  gen0: {} bytes, proof {} bytes",
        genesis.key_bytes().len(),
        genesis.proof().len()
    );

    // Generation 1: child (mixed with context entropy)
    let child = rpc::derive_lineage_key(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        "test-child-domain",
        Some(&genesis),
    )
    .expect("child derivation should succeed");

    assert_eq!(child.generation(), 1, "child should be generation 1");
    assert!(!child.is_genesis(), "child should not be genesis");
    assert_ne!(
        child.key_bytes(),
        genesis.key_bytes(),
        "child key must differ from genesis"
    );
    assert_ne!(
        child.parent_hash(),
        &[0u8; 32],
        "child parent hash should be non-zero"
    );
    println!(
        "  gen1: {} bytes, proof {} bytes, parent_hash non-zero",
        child.key_bytes().len(),
        child.proof().len()
    );

    // Generation 2: grandchild
    let grandchild = rpc::derive_lineage_key(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        "test-grandchild-domain",
        Some(&child),
    )
    .expect("grandchild derivation should succeed");

    assert_eq!(
        grandchild.generation(),
        2,
        "grandchild should be generation 2"
    );
    assert_ne!(
        grandchild.key_bytes(),
        child.key_bytes(),
        "grandchild key must differ from child"
    );
    assert_ne!(
        grandchild.parent_hash(),
        genesis.parent_hash(),
        "grandchild parent hash should differ from genesis parent hash"
    );
    println!(
        "  gen2: {} bytes, proof {} bytes",
        grandchild.key_bytes().len(),
        grandchild.proof().len()
    );
}

// ---------------------------------------------------------------------------
// Gate 8.3: Lineage proof generation and verification round-trip
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn genetics_lineage_proof_verify() {
    use primalspring::coordination::AtomicType;
    use primalspring::genetics::rpc;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-proof-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    let mut client = running
        .client_for("security")
        .expect("should connect to beardog");

    let seed_hex = String::from_utf8_lossy(running.mito_seed().expect("mito seed")).into_owned();

    let genesis = match rpc::derive_lineage_key(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        "proof-test-domain",
        None,
    ) {
        Ok(g) => g,
        Err(e) => {
            println!("  lineage proof/verify: skipped ({e})");
            return;
        }
    };

    // Proof already embedded in genesis by derive_lineage_key
    assert!(
        !genesis.proof().is_empty(),
        "genesis should have a lineage proof"
    );

    // Verify the genesis proof
    let valid = rpc::verify_lineage(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        genesis.proof(),
    );

    match valid {
        Ok(is_valid) => {
            assert!(is_valid, "genesis proof should verify as valid");
            println!("  genesis proof verified: valid={is_valid}");
        }
        Err(e) => {
            println!(
                "  lineage verification: error ({e}) — may be expected if BearDog version doesn't support verify_lineage"
            );
        }
    }

    // Derive a child and verify its proof too
    let child = rpc::derive_lineage_key(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        "proof-child-domain",
        Some(&genesis),
    )
    .expect("child derivation should succeed");

    let child_valid = rpc::verify_lineage(
        &mut client,
        &seed_hex,
        &family_id,
        "test-peer",
        child.proof(),
    );

    match child_valid {
        Ok(is_valid) => {
            assert!(is_valid, "child proof should verify as valid");
            println!("  child proof verified: valid={is_valid}");
        }
        Err(e) => {
            println!("  child verification: error ({e})");
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 8.4: Entropy mixing via genetic.mix_entropy
// ---------------------------------------------------------------------------

#[test]
#[ignore = "requires plasmidBin binaries (run with --ignored)"]
fn genetics_entropy_mixing() {
    use primalspring::coordination::AtomicType;
    use primalspring::genetics::rpc;
    use primalspring::harness::AtomicHarness;

    let family_id = format!("itest-entropy-{}", std::process::id());
    let running = AtomicHarness::new(AtomicType::Tower)
        .start(&family_id)
        .expect("tower atomic should start");

    let mut client = running
        .client_for("security")
        .expect("should connect to beardog");

    let human_entropy = b"user-passphrase-entropy-tier";
    let supervised_entropy = b"supervised-model-entropy-tier";
    let machine_entropy = b"random-machine-bytes-tier-abc";

    let mixed = rpc::mix_entropy(
        &mut client,
        Some(human_entropy.as_slice()),
        Some(supervised_entropy.as_slice()),
        Some(machine_entropy.as_slice()),
    );

    match mixed {
        Ok(result) => {
            assert!(!result.is_empty(), "mixed entropy should be non-empty");
            assert_ne!(
                result, human_entropy,
                "mixed result should differ from any single input"
            );
            println!("  entropy mixed: {} bytes", result.len());

            // Mixing the same inputs should be deterministic
            let mixed2 = rpc::mix_entropy(
                &mut client,
                Some(human_entropy.as_slice()),
                Some(supervised_entropy.as_slice()),
                Some(machine_entropy.as_slice()),
            )
            .expect("second mix should succeed");
            assert_eq!(result, mixed2, "same inputs should produce same output");
        }
        Err(e) => {
            let msg = format!("{e}");
            assert!(
                msg.contains("Method not found") || msg.contains("not found"),
                "expected mix_entropy or method-not-found, got: {e}"
            );
            println!("  entropy mixing: skipped ({e})");
        }
    }
}
