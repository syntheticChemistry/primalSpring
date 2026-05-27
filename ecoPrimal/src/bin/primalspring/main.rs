// SPDX-License-Identifier: AGPL-3.0-or-later

//! primalSpring UniBin — the eukaryotic cell.
//!
//! Single binary consolidating certification (guidestone organelle),
//! validation scenarios (experiment ribosomes), and IPC server (cell membrane).
//!
//! Evolved from the prokaryotic era of separate binaries during the
//! interstadial transition.

#![deny(unsafe_code)]

mod cli;

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
        cli::Commands::Serve => cmd_serve(),
        cli::Commands::Status => cmd_status(),
        cli::Commands::Checksums { ref output } => cmd_checksums(output),
        cli::Commands::Registry { ref check } => cmd_registry(check),
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

    if let Ok(json) = v.to_json() {
        let path = dir.join("results.json");
        if let Ok(mut f) = std::fs::File::create(&path) {
            let _ = f.write_all(json.as_bytes());
            eprintln!("provenance: wrote {}", path.display());
        }
    }

    let today = {
        let d = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let days = d / 86400;
        let y = 1970 + (days * 400) / 146_097;
        format!("{y}-{:02}-{:02}", (days % 365) / 30 + 1, (days % 365) % 30 + 1)
    };

    let provenance_toml = format!(
        r#"[run]
spring = "primalSpring"
version = "{version}"
date = "{today}"
threads = ["10"]
tier = 2
scenarios_run = {scenarios_run}

[environment]
gate = "irongate"
nucleus_composition = "full"
host = "{host}"

[results]
total_checks = {total}
passed = {passed}
failed = {failed}
skipped = {skipped}
"#,
        version = env!("CARGO_PKG_VERSION"),
        today = today,
        scenarios_run = scenarios_run,
        host = std::env::var(primalspring::env_keys::HOSTNAME)
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "unknown".into()),
        total = v.evaluated(),
        passed = v.passed,
        failed = v.failed,
        skipped = v.skipped,
    );

    let path = dir.join("provenance.toml");
    if let Ok(mut f) = std::fs::File::create(&path) {
        let _ = f.write_all(provenance_toml.as_bytes());
        eprintln!("provenance: wrote {}", path.display());
    }
}

fn cmd_serve() {
    serve::run();
}

mod serve {
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::UnixListener;

    use primalspring::ipc::method_gate::{CallerContext, MethodGate};
    use primalspring::ipc::protocol::{
        JSONRPC_VERSION, JsonRpcError, JsonRpcRequest, JsonRpcResponse,
    };
    use primalspring::{PRIMAL_DOMAIN, PRIMAL_NAME};

    pub fn run() {
        let sock_path = primalspring::ipc::discover::socket_path(PRIMAL_NAME);
        let gate = MethodGate::from_env();

        tracing::info!("{PRIMAL_NAME} server starting...");
        tracing::info!(domain = PRIMAL_DOMAIN);
        tracing::info!(socket = %sock_path.display());
        tracing::info!(auth_mode = gate.mode().as_str(), "method gate initialized");

        if let Some(parent) = sock_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                tracing::error!(error = %e, "failed to create socket directory");
                std::process::exit(1);
            }
        }

        let _ = std::fs::remove_file(&sock_path);
        let listener = match UnixListener::bind(&sock_path) {
            Ok(l) => l,
            Err(e) => {
                tracing::error!(error = %e, "failed to bind Unix socket");
                std::process::exit(1);
            }
        };

        tracing::info!("listening for JSON-RPC 2.0 connections");

        std::thread::spawn(move || {
            primalspring::niche::register_with_target(&sock_path);
        });

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    tracing::debug!("client connected");
                    if let Err(e) = handle_connection(&stream, &gate) {
                        tracing::warn!(error = %e, "connection error");
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, "accept failed");
                }
            }
        }
    }

    fn handle_connection(
        stream: &std::os::unix::net::UnixStream,
        gate: &MethodGate,
    ) -> std::io::Result<()> {
        let caller = CallerContext::from_unix_stream(stream);
        let mut writer = stream;
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        while reader.read_line(&mut line)? > 0 {
            let response = dispatch_gated(&line, &caller, gate);
            let response_json = match serde_json::to_string(&response) {
                Ok(json) => json,
                Err(e) => {
                    tracing::error!(error = %e, "failed to serialize JSON-RPC response");
                    r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"internal serialization error"},"id":0}"#.to_owned()
                }
            };
            writer.write_all(response_json.as_bytes())?;
            writer.write_all(b"\n")?;
            line.clear();
        }

        Ok(())
    }

    fn dispatch_gated(
        line: &str,
        base_caller: &CallerContext,
        gate: &MethodGate,
    ) -> JsonRpcResponse {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(line.trim());
        let method = parsed
            .as_ref()
            .ok()
            .and_then(|v| v["method"].as_str())
            .unwrap_or("");
        let normalized = primalspring::ipc::normalize_method(method);
        let id = parsed
            .as_ref()
            .ok()
            .and_then(|v| v["id"].as_u64())
            .unwrap_or(0);

        let params = parsed
            .as_ref()
            .ok()
            .and_then(|v| v.get("params"))
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let caller = base_caller
            .clone()
            .with_params_token(&params, gate.verifier());

        if let Err(err) = gate.check(normalized, &caller) {
            return JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_owned(),
                result: None,
                error: Some(err),
                id,
            };
        }

        match normalized {
            "auth.mode" | "auth.check" | "auth.peer_info" => {
                dispatch_auth(normalized, &caller, gate, id)
            }
            _ => dispatch_request(line),
        }
    }

    fn dispatch_auth(
        method: &str,
        caller: &CallerContext,
        gate: &MethodGate,
        id: u64,
    ) -> JsonRpcResponse {
        let result = match method {
            "auth.mode" => serde_json::json!({ "mode": gate.mode().as_str() }),
            "auth.check" => {
                let has_token = caller.bearer_token.is_some();
                let verified = caller.verified.is_some();
                let mut r = serde_json::json!({
                    "authenticated": has_token,
                    "verified": verified,
                    "enforcement": gate.mode().as_str(),
                });
                if let Some(ref v) = caller.verified {
                    r["scopes"] = serde_json::json!(v.scopes);
                    if let Some(ref sub) = v.subject {
                        r["subject"] = serde_json::json!(sub);
                    }
                    if let Some(exp) = v.expires_in {
                        r["expires_in"] = serde_json::json!(exp);
                    }
                }
                r
            }
            "auth.peer_info" => serde_json::json!({
                "origin": format!("{:?}", caller.origin),
                "peer": caller.peer.as_ref().map(|p| serde_json::json!({
                    "uid": p.uid,
                    "pid": p.pid,
                })),
            }),
            _ => unreachable!(),
        };

        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(result),
            error: None,
            id,
        }
    }

    fn dispatch_request(raw_request: &str) -> JsonRpcResponse {
        let request: JsonRpcRequest = match serde_json::from_str(raw_request.trim()) {
            Ok(r) => r,
            Err(_) => {
                return JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_owned(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "parse error".to_owned(),
                        data: None,
                    }),
                    id: 0,
                };
            }
        };

        let method = primalspring::ipc::normalize_method(&request.method);
        let id = request.id;

        let result: serde_json::Value = match method {
            "health.check" | "health.liveness" => {
                serde_json::json!({"status": "ok", "primal": "primalspring"})
            }
            "health.readiness" => {
                serde_json::json!({"status": "ok", "primal": "primalspring", "ready": true})
            }
            "capabilities.list" | "capability.list" => {
                let caps = primalspring::niche::all_capabilities();
                serde_json::json!({
                    "capabilities": caps,
                    "count": caps.len(),
                    "primal": primalspring::PRIMAL_NAME,
                })
            }
            "coordination.status" => {
                serde_json::json!({
                    "primal": "primalspring",
                    "version": env!("CARGO_PKG_VERSION"),
                    "domain": primalspring::PRIMAL_DOMAIN,
                })
            }
            _ => {
                return JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_owned(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32601,
                        message: format!("method not found: {method}"),
                        data: None,
                    }),
                    id,
                };
            }
        };

        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(result),
            error: None,
            id,
        }
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
        "ecoPrimal/src/bin/primalspring_primal/main.rs",
        "ecoPrimal/src/certification/mod.rs",
        "ecoPrimal/src/composition/mod.rs",
        "ecoPrimal/src/validation/mod.rs",
        "ecoPrimal/src/validation/scenarios/mod.rs",
        "ecoPrimal/src/validation/scenarios/registry.rs",
        "ecoPrimal/src/validation/scenarios/s_cephalization.rs",
        "ecoPrimal/src/validation/scenarios/s_tower_cns.rs",
        "ecoPrimal/src/validation/scenarios/s_kderm_boundary.rs",
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

fn cmd_registry(check: &str) {
    let registry_path = "config/capability_registry.toml";
    let registry_content = match std::fs::read_to_string(registry_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FAIL: cannot read {registry_path}: {e}");
            std::process::exit(1);
        }
    };

    let registered: std::collections::HashSet<String> = registry_content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('"') && trimmed.contains('.') {
                let method = trimmed.trim_matches(|c: char| c == '"' || c == ',' || c.is_whitespace());
                if method.contains('.') && method.chars().all(|c| c.is_ascii_lowercase() || c == '.' || c == '_' || c.is_ascii_digit()) {
                    return Some(method.to_owned());
                }
            }
            None
        })
        .collect();

    let run_source = check == "all" || check == "source";
    let run_graphs = check == "all" || check == "graphs";
    let run_coverage = check == "all" || check == "coverage";

    let mut errors = 0u32;

    if run_source {
        errors += check_source_methods(&registered);
    }
    if run_graphs {
        errors += check_graph_methods(&registered);
    }
    if run_coverage {
        check_coverage(&registered);
    }

    if errors > 0 {
        eprintln!("\n{errors} registry drift issue(s) found");
        std::process::exit(1);
    }
    println!("\nRegistry lint: PASS ({} methods registered)", registered.len());
}

fn check_source_methods(registered: &std::collections::HashSet<String>) -> u32 {
    println!("=== Source method strings vs registry ===");
    let mut errors = 0u32;
    let src_dirs = ["ecoPrimal/src", "experiments"];

    for dir in &src_dirs {
        let Ok(walker) = walk_rs_files(dir) else { continue };
        for path in walker {
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            for (line_no, line) in content.lines().enumerate() {
                for method in extract_method_strings(line) {
                    if !registered.contains(&method) && !is_known_non_method(&method) {
                        if errors == 0 {
                            println!("  DRIFT: method string(s) not in registry:");
                        }
                        println!("    {method}  ({}:{})", path.display(), line_no + 1);
                        errors += 1;
                    }
                }
            }
        }
    }

    if errors == 0 {
        println!("  OK: all source method strings found in registry");
    }
    errors
}

fn check_graph_methods(registered: &std::collections::HashSet<String>) -> u32 {
    println!("=== Graph TOML methods vs registry ===");
    let mut errors = 0u32;
    let graph_dirs = ["graphs/fragments", "graphs/cells", "graphs/downstream"];

    for dir in &graph_dirs {
        let Ok(walker) = walk_toml_files(dir) else { continue };
        for path in walker {
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            for (line_no, line) in content.lines().enumerate() {
                for method in extract_method_strings(line) {
                    if !registered.contains(&method) && !is_known_non_method(&method) {
                        if errors == 0 {
                            println!("  DRIFT: graph method(s) not in registry:");
                        }
                        println!("    {method}  ({}:{})", path.display(), line_no + 1);
                        errors += 1;
                    }
                }
            }
        }
    }

    if errors == 0 {
        println!("  OK: all graph methods found in registry");
    }
    errors
}

fn check_coverage(registered: &std::collections::HashSet<String>) {
    println!("=== Registry coverage (registered but never referenced) ===");
    let mut all_source = String::new();
    for dir in &["ecoPrimal/src", "experiments", "graphs"] {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                collect_content_recursive(&entry.path(), &mut all_source);
            }
        }
    }

    let mut unused = 0u32;
    for method in registered {
        if !all_source.contains(method.as_str()) {
            if unused == 0 {
                println!("  Advisory: registered methods with no source references:");
            }
            println!("    {method}");
            unused += 1;
        }
    }

    if unused == 0 {
        println!("  OK: all {} registered methods are referenced in source", registered.len());
    } else {
        println!("  {unused} registered method(s) have no source references (advisory)");
    }
}

fn collect_content_recursive(path: &std::path::Path, out: &mut String) {
    if path.is_file() {
        if let Ok(c) = std::fs::read_to_string(path) {
            out.push_str(&c);
        }
    } else if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                collect_content_recursive(&entry.path(), out);
            }
        }
    }
}

fn extract_method_strings(line: &str) -> Vec<String> {
    let mut methods = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('"') {
        rest = &rest[start + 1..];
        if let Some(end) = rest.find('"') {
            let candidate = &rest[..end];
            if candidate.contains('.')
                && candidate.len() >= 3
                && candidate.chars().all(|c| c.is_ascii_lowercase() || c == '.' || c == '_' || c.is_ascii_digit())
                && candidate.chars().next().is_some_and(|c| c.is_ascii_lowercase())
            {
                methods.push(candidate.to_owned());
            }
            rest = &rest[end + 1..];
        } else {
            break;
        }
    }
    methods
}

fn is_known_non_method(s: &str) -> bool {
    s.contains("..") || s.starts_with('.') || s.ends_with('.')
        || s == "prov.o" || s == "json.ld"
        || s.starts_with("v0.") || s.starts_with("v1.") || s.starts_with("v2.")
        || s.contains("_test.") || s.contains(".toml") || s.contains(".json")
        || s.contains(".rs") || s.contains(".sh") || s.contains(".py")
        || s.contains(".sock") || s.contains(".log") || s.contains(".pid")
        || s.contains(".seed") || s.contains(".txt") || s.contains(".md")
}

fn walk_rs_files(dir: &str) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    walk_files_with_ext(std::path::Path::new(dir), "rs", &mut files)?;
    Ok(files)
}

fn walk_toml_files(dir: &str) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    walk_files_with_ext(std::path::Path::new(dir), "toml", &mut files)?;
    Ok(files)
}

fn walk_files_with_ext(dir: &std::path::Path, ext: &str, out: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_files_with_ext(&path, ext, out)?;
        } else if path.extension().is_some_and(|e| e == ext) {
            out.push(path);
        }
    }
    Ok(())
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
