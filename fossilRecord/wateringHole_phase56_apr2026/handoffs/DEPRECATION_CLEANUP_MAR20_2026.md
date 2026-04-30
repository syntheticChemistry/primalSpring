# Deprecation Cleanup — March 20, 2026

**Executed by**: primalSpring (coordination spring)
**Scope**: Safe dead code removal across BearDog, Songbird, biomeOS

## Summary

| Repo | Lines Removed | Key Changes |
|------|---------------|-------------|
| Songbird | ~2,150 | 5 dead files, sqlite URL fix |
| BearDog | ~726 | 1 orphan file, 9 unused workspace deps |
| biomeOS | ~3,602 | deprecated-tui feature + module, cargo cruft |
| **Total** | **~6,478** | |

All three repos compile clean and are pushed to origin.

## Songbird Changes (commit 864d0e845)

### Dead Files Removed (1,865 lines)

| File | Lines | Why Dead |
|------|-------|----------|
| `persistence/production_registry.rs` | 719 | Broken syntax (invalid struct), never compiled, commented out of mod.rs |
| `task_lifecycle/storage.rs` | 510 | sqlx-based, superseded by `storage_sled.rs` (Jan 27 migration) |
| `consent_management/storage.rs` | 441 | sqlx-based, superseded by `storage_sled.rs` (Jan 27 migration) |
| `discovery_refactored/mod.rs` | 195 | Not declared in `lib.rs`, orphaned parallel module |
| `federation_aware_discovery_tests.rs.future` | 285 | Renamed to avoid build, uses non-existent feature |

### Bug Fix — sled Storage URL

`http_server.rs` was passing `"sqlite:/tmp/songbird-data/task_lifecycle.db"` to `sled::open()`.
The `sqlite:` prefix was a leftover from the sqlx→sled migration. sled was interpreting the entire
string as a directory path, causing startup failures in harness environments.

Fixed: plain path string, no protocol prefix.

### Remaining Debt (team action needed)

- **Triple RPC stack**: tarpc + JSON-RPC + REST/Axum — needs product decision on single transport
- **BearDog direct crypto**: `beardog_crypto_client.rs` — route through Neural API
- **`tests-incomplete` feature**: large disabled test suites — enable/fix or trim
- **`dead_code = "allow"` lint**: masks unused items in orchestrator

## BearDog Changes (commit 6f780c5d5)

### Orphan File Removed (726 lines)

`beardog-types/src/canonical/network/universal_endpoints.rs` — used `reqwest` (not a dep),
not in the module graph (`network.rs` is a flat file, not `network/mod.rs`).

### Unused Workspace Dependencies Pruned (9 entries)

| Dependency | Why Unused |
|------------|-----------|
| `sqlx` | No crate references it |
| `quinn` | No crate references it |
| `rustls` | No crate references it |
| `rustls-pemfile` | No crate references it |
| `tokio-rustls` | No crate references it |
| `prometheus` | No crate references it |
| `opentelemetry` | No crate references it |
| `opentelemetry-prometheus` | No crate references it |
| `metrics` | No crate references it |

### Remaining Debt (team action needed)

- **`beardog-integration` crate**: excluded from workspace, tests reference `reqwest` (not a dep) — delete or rewrite
- **`tarpc` stack in `beardog-ipc`**: ~2,500 LOC optional — aligns with TARPC_REMOVAL_RATIONALE doc
- **Dual `beardog` packages**: root lib vs `crates/beardog` — consolidate or rename
- **gRPC fields in types**: `grpc_port`, `BEARDOG_GRPC_*` — phase out or rename to capability metadata

## biomeOS Changes (commit 5f78c0e0)

### Deprecated TUI Removed (3,602 lines)

Entire `crates/biomeos-cli/src/tui/` directory deleted. Was behind `deprecated-tui` feature,
superseded by petalTongue universal UI primal.

- Removed `deprecated-tui` feature flag
- Removed `ratatui` and `crossterm` optional dependencies
- Simplified `handle_dashboard()` to single redirect (no cfg branching)
- Removed cfg gate from dashboard test
- Cleaned commented `[[bin]]`/`[[example]]` entries from root Cargo.toml

### Remaining Debt (team action needed)

- **`DirectBeardogCaller` + `discover_beardog_socket`**: identity-based discovery — migrate to Neural API
- **`biomeos-deploy`**: no Rust importers in workspace — quarantine or document
- **`biomeos-compute`**: no dependents — quarantine if not on roadmap
- **Root `biomeos` lib package**: slim facade with many unused deps in Cargo.toml
- **Shell scripts vs `biomeos nucleus`**: platform scripts still needed for mobile/USB

## Verification

All three repos: `cargo check` passes with zero errors, zero warnings (excluding Cargo.toml key warnings).
