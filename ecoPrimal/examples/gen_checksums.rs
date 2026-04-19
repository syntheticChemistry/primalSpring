// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate the CHECKSUMS manifest for `primalspring_guidestone` P3 verification.
//!
//! Usage: `cargo run --example gen_checksums > validation/CHECKSUMS`

fn main() {
    let files: &[&str] = &[
        "ecoPrimal/src/bin/primalspring_guidestone/main.rs",
        "ecoPrimal/src/composition/mod.rs",
        "ecoPrimal/src/validation/mod.rs",
        "ecoPrimal/src/tolerances/mod.rs",
        "ecoPrimal/src/coordination/mod.rs",
        "ecoPrimal/src/bonding/mod.rs",
        "ecoPrimal/src/btsp/mod.rs",
        "ecoPrimal/src/deploy/mod.rs",
        "ecoPrimal/src/checksums.rs",
        "ecoPrimal/Cargo.toml",
        "graphs/fragments/tower_atomic.toml",
        "graphs/fragments/node_atomic.toml",
        "graphs/fragments/nest_atomic.toml",
        "graphs/fragments/nucleus.toml",
        "graphs/fragments/meta_tier.toml",
        "graphs/fragments/provenance_trio.toml",
        "graphs/downstream/downstream_manifest.toml",
        "graphs/downstream/proto_nucleate_template.toml",
    ];

    let root = std::path::Path::new(".");
    let manifest = primalspring::checksums::generate_manifest(root, files);
    println!("# primalSpring guideStone CHECKSUMS — BLAKE3");
    println!("# Generated: {}", chrono_free_date());
    println!("# Files: {}", files.len());
    println!("#");
    println!("# Verify: primalspring::checksums::verify_manifest()");
    println!("{manifest}");
}

fn chrono_free_date() -> String {
    let output = std::process::Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".to_owned());
    output.trim().to_owned()
}
