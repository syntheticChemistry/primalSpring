# Handoff: primalSpring v0.1.0 — Deep Debt Audit + Evolution

**Date:** March 17, 2026  
**From:** primalSpring (Phase 0→1 deep debt audit)  
**To:** biomeOS, all NUCLEUS primals, ecosystem  
**License:** AGPL-3.0-or-later  
**Covers:** primalSpring v0.1.0, all 38 experiments, 7 tracks

---

## Executive Summary

primalSpring underwent a comprehensive audit against all ecosystem standards
(wateringHole), sibling spring patterns, and deep debt criteria. Every finding
was immediately resolved. The codebase is now zero-warning across all five
toolchain checks (fmt, check, clippy pedantic+nursery, test, doc) with full
public API documentation and honest validation in every experiment.

---

## What Changed (This Session)

### Critical Fixes

| Finding | Before | After |
|---------|--------|-------|
| Dishonest scaffolding (exp041, exp042) | `check_bool("name", true, "scaffolded")` — always passed | Real discovery-based experiments with `discover_for()`, `probe_primal()`, `check_skip()` |
| Broken doc link (discover.rs) | `AtomicType::required_primals()` unresolvable | Fully qualified `crate::coordination::AtomicType::required_primals()` |

### Workspace-Level Lint Consolidation

| Lint | Before | After |
|------|--------|-------|
| `unsafe_code` | `#![forbid(unsafe_code)]` in 40 files | Single `unsafe_code = "forbid"` in `[workspace.lints.rust]` |
| `missing_docs` | Not enforced | `missing_docs = "warn"` in `[workspace.lints.rust]` |

Removed 40 redundant per-file `#![forbid(unsafe_code)]` directives. Single
source of truth in `Cargo.toml`.

### Documentation (98 lib + 25 experiment warnings → 0)

- Added doc comments to all public struct fields, enum variants, methods, and constants
- Added crate-level `//!` doc comments to all 25 experiment crates that lacked them
- Broken intra-doc link in `ipc::discover` fixed
- `cargo doc --no-deps` now produces **zero warnings**

### Server Hardening

- `unwrap_or_default()` on response serialization replaced with explicit
  error handling that logs via `tracing::error` and returns a valid JSON-RPC
  internal error response

### Metadata

- Added `repository` field to `[workspace.package]` in `Cargo.toml`

---

## Current State

| Metric | Value |
|--------|-------|
| `cargo fmt --check` | PASS |
| `cargo check --all-features` | 0 warnings |
| `cargo clippy --all-features -- -D warnings` | PASS |
| `cargo test --all-features` | 69 passed, 0 failed |
| `cargo doc --all-features --no-deps` | 0 warnings |
| Experiments | 38 (7 tracks) |
| Unit tests | 69 |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace `forbid` |
| missing_docs | Workspace `warn` — 0 warnings |
| C dependencies | 0 (ecoBin compliant) |
| Dishonest scaffolding | 0 |
| Files over 1000 LOC | 0 (max: 375) |

---

## Audit Results by Category

### Completion Status
- 38 experiments, all compile, all use honest validation
- exp041/exp042 evolved from fake passes to real discovery-based experiments
- All tolerance constants documented with provenance notes (pending calibration)
- No Python baselines needed (coordination domain, not numerical)

### Code Quality
- Zero clippy warnings (pedantic + nursery)
- Zero `#[allow()]` in production code
- Workspace-level `unsafe_code = "forbid"`
- Workspace-level `missing_docs = "warn"` with zero warnings
- All errors typed (`IpcError` enum with `Display` + `std::error::Error`)
- Pure Rust dependency chain confirmed via `cargo tree`

### Ecosystem Standards
- License: AGPL-3.0-or-later (SPDX headers on all source files)
- ecoBin: Pure Rust, zero C deps, cross-compile ready (Unix transport)
- UniBin: Single binary with Server/Status/Version subcommands
- IPC: JSON-RPC 2.0, `domain.verb` naming, XDG socket discovery
- Files: All under 1000 LOC
- Handoffs: Follow naming convention

### Primal Coordination
- Socket discovery for all NUCLEUS primals via env/XDG/temp fallback
- Neural API bridge via `neural-api-client-sync`
- `primalspring_primal` server exposes 6 JSON-RPC methods
- Capability registration with Songbird: not yet implemented (Phase 1)

---

## What Blocks Phase 1 (Live IPC)

1. **Live primals needed** — experiments need BearDog + Songbird running on
   Unix sockets to move from `check_skip` to real IPC validation
2. **biomeOS graph executor** — Track 2 experiments need biomeOS to parse
   and execute the graph TOML files
3. **Provenance Trio deployment** — Track 3 (RootPulse) needs rhizoCrypt +
   LoamSpine + sweetGrass running for the 6-phase commit flow
4. **Songbird registration** — `primalspring_primal` should call
   `ipc.register` at startup to advertise coordination capabilities

---

## What Each Consumer Should Know

### biomeOS
- `graphs/primalspring_deploy.toml` is ready for `biomeos deploy`
- `niches/primalspring-coordination.yaml` defines full deployment

### All Primals
- Discovery uses `$XDG_RUNTIME_DIR/biomeos/{primal}-{family}.sock` convention
  and `{PRIMAL}_SOCKET` env override — no hardcoded primal rosters
- primalSpring discovers peers at runtime via capability-based routing

### Springs
- primalSpring validates coordination patterns mined from all sibling springs
- Does not import other springs — coordinates through IPC and wateringHole handoffs

---

**License**: AGPL-3.0-or-later
