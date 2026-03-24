# primalSpring v0.7.0 — Ecosystem Absorption Handoff

**Date**: March 23, 2026
**From**: primalSpring (coordination spring)
**To**: All primal and spring teams
**Status**: 303 tests, 51 experiments, 22 deploy graphs, 87/87 gates, Phase 12

---

## What Happened

primalSpring performed a comprehensive review of all 7 sibling springs and all
primals (phase1, phase2, ecoPrimals top-level), absorbed the best-evolved
patterns into its core library, and verified everything compiles, passes clippy
pedantic+nursery, and passes 303 tests with zero warnings.

This handoff documents what was absorbed, what each team can leverage from
primalSpring's work, and what evolution opportunities we identified during the
review that are relevant to individual primal and spring teams.

---

## What primalSpring Absorbed (and teams can absorb back)

### 1. `deny.toml` Ban List Convergence

**Absorbed from**: groundSpring V120, wetSpring V132
**What**: Merged C-dependency ban lists into a single comprehensive `deny.toml`

```toml
# New bans (in addition to existing openssl-sys, openssl, ring, libssh2-sys, etc.)
{ crate = "aws-lc-sys", wrappers = [] }
{ crate = "aws-lc-rs", wrappers = [] }
{ crate = "cmake", wrappers = [] }
{ crate = "cc", wrappers = ["blake3"] }
{ crate = "pkg-config", wrappers = [] }
{ crate = "vcpkg", wrappers = [] }
```

**Action for teams**: If your `deny.toml` doesn't ban `aws-lc-sys`, `cmake`,
`cc`, `pkg-config`, or `vcpkg`, add them. The `cc` crate is allowed only as a
wrapper for `blake3` (which needs it for SIMD assembly). All other usage
indicates a C build dependency that violates ecoBin v3.0.

### 2. Cast Discipline Clippy Lints

**Absorbed from**: neuralSpring S170, airSpring V010
**What**: Workspace-level clippy cast lints that catch truncation and sign-loss bugs

```toml
[workspace.lints.clippy]
cast_lossless = "warn"
cast_precision_loss = "warn"
cast_possible_truncation = "warn"
cast_sign_loss = "warn"
cast_possible_wrap = "warn"
```

**Action for teams**: Add these to your `[workspace.lints.clippy]` in the
workspace `Cargo.toml`. Fix violations with explicit `saturating_cast` or
`try_from().unwrap_or()` rather than `#[allow]`. primalSpring's `cast.rs`
module provides safe cast helpers if needed.

### 3. `ValidationSink` Enrichment

**Absorbed from**: groundSpring V120
**What**: Two new methods on the `ValidationSink` trait

- `section(&self, name: &str)` — begin a named section of checks (e.g.
  "IPC health", "Graph validation")
- `write_summary(&self, passed: u32, failed: u32, skipped: u32)` — write a
  structured summary footer after all checks

Both have default no-op implementations so existing sinks continue to work
without changes.

**Action for teams**: If you use primalSpring's validation harness or have your
own `ValidationSink`, consider implementing `section()` for structured output
in CI logs.

### 4. Skip-Aware Exit Codes

**Absorbed from**: wetSpring V132
**What**: `ValidationResult::exit_code_skip_aware()` returns 3-way exit:

| Exit Code | Meaning |
|-----------|---------|
| 0 | At least one check passed, none failed |
| 1 | At least one check failed |
| 2 | All checks were skipped (no real passes or fails) |

**Action for teams**: Use `exit_code_skip_aware()` instead of `exit_code()` in
CI pipelines where "all skipped" should be distinguishable from "all passed".
This prevents false green in CI when no primals are actually running.

### 5. Cross-Cutting Property Tests (`proptest_ipc`)

**Absorbed from**: healthSpring V41
**What**: 7 new property tests that fuzz the entire IPC pipeline — discovery,
protocol formatting, extraction, dispatch classification, and multi-format
capability parsing. These tests run on every `cargo test` and have caught
edge cases that targeted unit tests missed.

**Action for teams**: If your spring has IPC handling code, consider adding
proptest for your extraction/dispatch path. The pattern is:

```rust
proptest! {
    #[test]
    fn your_extraction_is_consistent(val in arb_json_value()) {
        let result = your_extract(&val);
        let dispatch = your_dispatch(&val);
        // Assert consistency between paths
    }
}
```

### 6. Primal Display Names (`primal_names`)

**Absorbed from**: neuralSpring canonical naming pattern
**What**: A `primal_names` module with `display_name(slug) -> &str` and
`discovery_slug(display) -> &str` covering all 23 known primals and springs.

**Action for teams**: Use `primal_names::display_name()` when rendering primal
names in logs or UI. Use `primal_names::discovery_slug()` when constructing
socket paths or env var names. This eliminates hardcoded name mappings
scattered across codebases.

### 7. Provenance Trio Circuit Breaker

**Absorbed from**: healthSpring V41
**What**: Epoch-based circuit breaker with exponential backoff for all
provenance trio calls in `ipc::provenance`. After 3 consecutive failures,
the circuit opens and calls short-circuit to `None` without attempting IPC.

**Action for teams**: If your spring calls the provenance trio (rhizoCrypt,
loamSpine, sweetGrass), wrap your calls in a similar circuit breaker. The
trio's loamSpine has a known runtime panic that can cascade into timeout storms
without circuit-breaking.

---

## Evolution Opportunities Identified Per Team

### BearDog Team
- **Abstract socket regression**: Android SELinux blocks filesystem sockets.
  BearDog v0.9.0 creates abstract sockets that fail on Pixel. Fix: fall back
  to `XDG_RUNTIME_DIR/biomeos/` filesystem sockets when abstract sockets fail.
- **primalSpring integration**: 5/5 Squirrel Discovery gates pass. BearDog is
  fully wired for capability-based composition.

### Songbird Team
- **11/12 subsystems UP**: The `monitoring.primal_status` subsystem returned
  `UnsupportedMethod` during live probing. All other subsystems healthy.
- **JSON-RPC subsystem naming**: Consider standardizing to
  `{domain}.{subsystem}.{operation}` triple.

### ToadStool + barraCuda Teams
- **Dual-protocol validated**: Both HTTP REST and JSON-RPC work via
  primalSpring's Node Atomic composition.
- **Compute triangle**: exp050 validates the full coralReef→toadStool→barraCuda
  pipeline end-to-end via capability discovery.

### NestGate Team
- **Storage fully validated**: 13/13 Nest Atomic checks pass (store, retrieve,
  list, model cache, ZFS fallback).
- **Cross-site replication**: exp072 structurally validates the 7-phase data
  federation pipeline. Next step: live multi-machine test.

### Provenance Trio (rhizoCrypt, loamSpine, sweetGrass)
- **sweetGrass LIVE**: All IPC calls succeed.
- **rhizoCrypt LIVE**: TCP socket reachable, session creation works.
- **loamSpine BROKEN**: Runtime panic on `ledger.commit`. Needs team fix.
- **4 gaps documented**: See `PROVENANCE_TRIO_LIVE_PROBING_MAR23_2026.md`

### Squirrel Team
- **Cross-primal discovery validated**: 5/5 gates. Squirrel discovers
  NestGate, ToadStool, Songbird, BearDog via `env_sockets`.
- **MCP tool integration**: 8 primalSpring tools discoverable via
  `mcp.tools.list` for AI coordination.

### biomeOS Team
- **Composition validated E2E**: Tower, Nest, Node, NUCLEUS, Graph Overlays
  all work via biomeOS orchestration.
- **Graph metadata parsing**: `graph_metadata.rs` validates `[graph.metadata]`
  and `[graph.bonding_policy]` sections. biomeOS should adopt this parsing for
  runtime policy enforcement.
- **STUN tier validation**: `stun_tiers.rs` validates sovereignty-first
  escalation. biomeOS should enforce "never skip to public before lineage"
  at orchestration time.

### petalTongue Team
- **Visualization validated**: exp065 confirms dashboard + Grammar of Graphics
  rendering via `visualization.render.dashboard` and `visualization.render.grammar`.
- **v1.6.6 integrated**: primalSpring's overlay compositions include
  petalTongue at any atomic tier.

### Spring Teams (hotSpring, wetSpring, groundSpring, neuralSpring, airSpring, healthSpring, ludoSpring)
- **Pattern absorption is bidirectional**: primalSpring absorbed patterns from
  all 7 springs. Springs should consider absorbing back:
  - `exit_code_skip_aware()` for CI exit codes
  - `primal_names` for display name consistency
  - `proptest_ipc` for IPC fuzz testing
  - The merged `deny.toml` ban list for ecoBin compliance
  - Cast discipline lints in `[workspace.lints.clippy]`

---

## Library Patterns Available for Absorption

Any crate depending on `ecoPrimal` (primalSpring's library crate) gets:

| Pattern | Module | What It Provides |
|---------|--------|------------------|
| Safe casts | `cast.rs` | `saturating_as_*` functions for numeric casts |
| Primal names | `primal_names` | `display_name()` / `discovery_slug()` for all 23 primals |
| Validation harness | `validation/` | `check_bool`, `check_skip`, `check_or_skip`, `OrExit`, `section()`, `exit_code_skip_aware()` |
| IPC resilience | `ipc/resilience` | `IpcError`, `CircuitBreaker`, `RetryPolicy`, `resilient_call()`, `DispatchOutcome<T>` |
| Discovery | `ipc/discover` | `discover_primal()`, `discover_by_capability()`, 5-tier fallback |
| IPC extraction | `ipc/extract` | `extract_rpc_result()`, `extract_rpc_dispatch()`, 4-format capability parsing |
| Provenance | `ipc/provenance` | `begin_experiment_session()`, `record_experiment_step()`, `complete_experiment()`, `rootpulse_*()` |
| Named tolerances | `tolerances/` | `HEALTH_TIMEOUT`, `DISCOVERY_TIMEOUT`, `IPC_RETRY_BASE`, etc. |
| Deploy graphs | `deploy/` | TOML parsing, structural validation, topological sort, live probing |
| Bonding | `bonding/` | `BondType`, `TrustModel`, `BondingPolicy`, `BondingConstraint`, `StunConfig` |
| Niche | `niche.rs` | Full BYOB self-knowledge: capabilities, mappings, costs, registration |

---

## Verification

All changes verified with:

```
cargo fmt --check          → clean
cargo clippy --workspace -- -D warnings  → 0 warnings
cargo deny check           → pass (all ecoBin bans enforced)
cargo test --workspace     → 303 tests pass, 0 failures
```

---

**License**: AGPL-3.0-or-later
