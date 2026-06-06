// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! primalSpring UniBin — the eukaryotic cell.
//!
//! Single binary consolidating certification (guidestone organelle),
//! validation scenarios (experiment ribosomes), and IPC server (cell membrane).
//!
//! Evolved from the prokaryotic era of separate binaries during the
//! interstadial transition.

#![forbid(unsafe_code)]

mod cli;
mod registry_lint;
mod serve;

use clap::Parser;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let parsed = cli::Cli::parse();

    match parsed.command {
        cli::Commands::Certify { layer, bare, format: _ } => cmd_certify(layer, bare),
        cli::Commands::Validate {
            ref track,
            ref scenario,
            ref tier,
            list,
            format,
            ref provenance_dir,
        } => cmd_validate(
            track.as_deref(),
            scenario.as_deref(),
            tier.as_deref(),
            list,
            matches!(format, cli::OutputFormat::Json),
            provenance_dir.as_deref(),
        ),
        cli::Commands::Serve => serve::run(),
        cli::Commands::Status => cmd_status(),
        cli::Commands::Checksums { ref output } => cmd_checksums(output),
        cli::Commands::Registry { ref check } => registry_lint::run(check),
        cli::Commands::Version => cmd_version(),
    }
}

fn cmd_certify(layer: Option<u8>, bare: bool) {
    let max_layer = if bare {
        0
    } else {
        layer.unwrap_or(primalspring::certification::MAX_LAYER)
    };

    let result = primalspring::certification::certify(max_layer);
    std::process::exit(result.exit_code());
}

fn cmd_validate(
    track: Option<&str>,
    scenario_id: Option<&str>,
    tier: Option<&str>,
    list: bool,
    json: bool,
    provenance_dir: Option<&str>,
) {
    use primalspring::validation::scenarios::{Tier, Track, build_registry};

    let registry = build_registry();

    if list {
        println!(
            "primalSpring Validation Scenarios ({} registered)\n",
            registry.len()
        );
        let hdr_scenario = "SCENARIO";
        let hdr_track = "TRACK";
        let hdr_tier = "TIER";
        let hdr_provenance = "PROVENANCE";
        println!("{hdr_scenario:<30} {hdr_track:<25} {hdr_tier:<6} {hdr_provenance}");
        println!("{}", "-".repeat(90));
        for s in registry.all() {
            println!(
                "{:<30} {:<25} {:<6} {}",
                s.meta.id, s.meta.track, s.meta.tier, s.meta.provenance_crate
            );
        }
        return;
    }

    let tier_filter: Option<Tier> = tier.map(|t| {
        Tier::from_str_loose(t).unwrap_or_else(|| {
            eprintln!("unknown tier: {t} (expected: rust, live, both)");
            std::process::exit(1);
        })
    });

    let track_filter: Option<Track> = track.and_then(|t| {
        Track::from_str_loose(t).or_else(|| {
            eprintln!("unknown track: {t}");
            std::process::exit(1);
        })
    });

    let mut v = primalspring::validation::ValidationResult::new(
        "primalSpring Validation — Scenario Runner",
    );
    let mut ctx = primalspring::composition::CompositionContext::discover();

    primalspring::validation::ValidationResult::print_banner(
        "primalSpring Validation — Scenario Runner",
    );

    let mut ran = 0usize;
    for s in registry.all() {
        if let Some(id) = scenario_id {
            if s.meta.id != id {
                continue;
            }
        }
        if let Some(track_f) = track_filter {
            if s.meta.track != track_f {
                continue;
            }
        }
        if let Some(tier_f) = tier_filter {
            if tier_f != Tier::Both && s.meta.tier != tier_f && s.meta.tier != Tier::Both {
                continue;
            }
        }

        v.section(&format!(
            "Scenario: {} [{}] ({})",
            s.meta.id, s.meta.track, s.meta.tier
        ));
        (s.run)(&mut v, &mut ctx);
        ran += 1;
    }

    if ran == 0 {
        eprintln!("no scenarios matched the filter criteria");
        std::process::exit(1);
    }

    if let Some(dir) = provenance_dir {
        write_provenance(&v, dir, ran);
    }

    if json {
        if let Ok(j) = v.to_json() {
            println!("{j}");
        } else {
            v.finish();
        }
    } else {
        v.finish();
    }
    std::process::exit(v.exit_code());
}

fn write_provenance(v: &primalspring::validation::ValidationResult, dir: &str, scenarios_run: usize) {
    use std::io::Write;
    let dir = std::path::Path::new(dir);
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("warning: could not create provenance dir {}: {e}", dir.display());
        return;
    }

    if let Ok(json_str) = v.to_json() {
        let path = dir.join("results.json");
        if let Ok(mut f) = std::fs::File::create(&path) {
            let _ = f.write_all(json_str.as_bytes());
            eprintln!("provenance: wrote {}", path.display());
        }
    }

    let today = chrono_free_today();
    let provenance = format!(
        "[provenance]\nprimal = \"primalspring\"\nversion = \"{}\"\ndate = \"{today}\"\nscenarios = {scenarios_run}\npassed = {}\nfailed = {}\nskipped = {}\n",
        env!("CARGO_PKG_VERSION"),
        v.passed,
        v.failed,
        v.skipped,
    );
    let path = dir.join("provenance.toml");
    if let Err(e) = std::fs::write(&path, provenance) {
        eprintln!("warning: could not write {}: {e}", path.display());
    } else {
        eprintln!("provenance: wrote {}", path.display());
    }
}

fn cmd_status() {
    use primalspring::coordination::AtomicType;
    use primalspring::ipc::discover::{discover_capabilities_for, neural_api_healthy};
    use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

    println!("{PRIMAL_NAME} v{}", env!("CARGO_PKG_VERSION"));
    println!("domain: {PRIMAL_DOMAIN}");
    println!(
        "local methods: {} | routed: {}",
        primalspring::niche::LOCAL_CAPABILITIES.len(),
        primalspring::niche::ROUTED_CAPABILITIES.len(),
    );

    let neural_ok = neural_api_healthy();
    println!(
        "neural_api: {}",
        if neural_ok { "reachable" } else { "not found" }
    );

    let capabilities = AtomicType::FullNucleus.required_capabilities();
    let discovered = discover_capabilities_for(capabilities);
    let found = discovered.iter().filter(|d| d.socket.is_some()).count();
    println!("capabilities: {found}/{} discovered", capabilities.len());

    for d in &discovered {
        let status = if d.socket.is_some() { "UP" } else { "DOWN" };
        let provider = d.resolved_primal.as_deref().unwrap_or("unresolved");
        println!("  [{status}] {} (via {provider})", d.capability);
    }
}

fn cmd_checksums(output: &str) {
    use primalspring::checksums;
    use std::path::Path;

    const TRACKED_FILES: &[&str] = &[
        "ecoPrimal/src/bin/primalspring/main.rs",
        "ecoPrimal/src/bin/primalspring/cli.rs",
        "ecoPrimal/src/bin/primalspring/serve.rs",
        "ecoPrimal/src/bin/primalspring/registry_lint.rs",
        "ecoPrimal/src/bin/primalspring_primal/main.rs",
        "ecoPrimal/src/certification/mod.rs",
        "ecoPrimal/src/composition/mod.rs",
        "ecoPrimal/src/validation/mod.rs",
        "ecoPrimal/src/validation/scenarios/mod.rs",
        "ecoPrimal/src/validation/scenarios/registry.rs",
        "ecoPrimal/src/validation/scenarios/s_cephalization.rs",
        "ecoPrimal/src/validation/scenarios/s_tower_cns.rs",
        "ecoPrimal/src/tolerances/mod.rs",
        "ecoPrimal/src/coordination/mod.rs",
        "ecoPrimal/src/bonding/mod.rs",
        "ecoPrimal/src/btsp/mod.rs",
        "ecoPrimal/src/deploy/mod.rs",
        "ecoPrimal/src/checksums.rs",
        "ecoPrimal/Cargo.toml",
        "config/capability_registry.toml",
        "graphs/fragments/tower_atomic.toml",
        "graphs/fragments/node_atomic.toml",
        "graphs/fragments/nest_atomic.toml",
        "graphs/fragments/nucleus.toml",
        "graphs/fragments/meta_tier.toml",
        "graphs/fragments/provenance_trio.toml",
        "graphs/downstream/downstream_manifest.toml",
        "graphs/downstream/proto_nucleate_template.toml",
    ];

    let root = Path::new(".");
    let manifest = checksums::generate_manifest(root, TRACKED_FILES);
    let today = chrono_free_today();

    let header = format!(
        "# primalSpring guideStone CHECKSUMS — BLAKE3\n# Generated: {today}\n# Files: {}\n#\n# Verify: primalspring::checksums::verify_manifest()\n",
        TRACKED_FILES.len()
    );

    let content = format!("{header}{manifest}\n");

    if let Err(e) = std::fs::write(output, &content) {
        eprintln!("error writing {output}: {e}");
        std::process::exit(1);
    }
    println!("Regenerated {output} ({} files)", TRACKED_FILES.len());
}

fn chrono_free_today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86400;
    let mut y = 1970i64;
    let mut remaining = days;
    loop {
        let year_days: u64 = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 366 } else { 365 };
        if remaining < year_days { break; }
        remaining -= year_days;
        y += 1;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let month_days: &[u64] = if leap {
        &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0;
    for (i, &md) in month_days.iter().enumerate() {
        if remaining < md { m = i; break; }
        remaining -= md;
    }
    format!("{y}-{:02}-{:02}", m + 1, remaining + 1)
}

fn cmd_version() {
    println!("primalspring {}", env!("CARGO_PKG_VERSION"));
}
