// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Registry drift detection — verifies capability method strings across
//! Rust source, TOML graphs, and the canonical `capability_registry.toml`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn run(check: &str) {
    let registry_path = "config/capability_registry.toml";
    let registry_content = match std::fs::read_to_string(registry_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FAIL: cannot read {registry_path}: {e}");
            std::process::exit(1);
        }
    };

    let registered: HashSet<String> = registry_content
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

fn check_source_methods(registered: &HashSet<String>) -> u32 {
    println!("=== Source method strings vs registry ===");
    let mut errors = 0u32;
    let src_dirs = ["ecoPrimal/src", "experiments"];

    for dir in &src_dirs {
        let Ok(walker) = walk_files_ext(dir, "rs") else { continue };
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

fn check_graph_methods(registered: &HashSet<String>) -> u32 {
    println!("=== Graph TOML methods vs registry ===");
    let mut errors = 0u32;
    let graph_dirs = ["graphs/fragments", "graphs/cells", "graphs/downstream"];

    for dir in &graph_dirs {
        let Ok(walker) = walk_files_ext(dir, "toml") else { continue };
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

fn check_coverage(registered: &HashSet<String>) {
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

fn collect_content_recursive(path: &Path, out: &mut String) {
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

fn walk_files_ext(dir: &str, ext: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    walk_recursive(Path::new(dir), ext, &mut files)?;
    Ok(files)
}

fn walk_recursive(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_recursive(&path, ext, out)?;
        } else if path.extension().is_some_and(|e| e == ext) {
            out.push(path);
        }
    }
    Ok(())
}
