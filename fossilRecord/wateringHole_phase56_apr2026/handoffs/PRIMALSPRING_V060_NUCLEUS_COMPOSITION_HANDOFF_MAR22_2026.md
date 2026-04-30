# primalSpring v0.6.0 — NUCLEUS Composition Handoff

**Date:** March 22, 2026
**From:** primalSpring v0.6.0
**To:** toadStool team, barraCuda team, nestgate team, all primal teams
**Supersedes:** PRIMALSPRING_V040_TOADSTOOL_BARRACUDA_EVOLUTION_HANDOFF_MAR21_2026.md
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring v0.6.0 has validated the full **NUCLEUS Composition** — Tower +
Nest + Node — with **58/58 stability gates passing**, **31 live integration
tests** running in parallel (~5s), and **7 experiments** all green. This
handoff documents the lessons learned integrating nestgate and toadstool,
what each team should absorb, and the evolution path forward.

The key finding: **both nestgate and toadstool required non-trivial harness
adaptations** to integrate. Neither binary follows the "standard" primal
launch pattern (server --socket PATH). The adaptations are documented here
so primal teams can evolve toward the ecosystem conventions.

---

## 1. What Was Done

### 1.1 Nest Atomic (nestgate integration)

**Composition**: beardog + songbird + biomeOS + nestgate

**What worked immediately**: nestgate has solid JSON-RPC 2.0 support, standard
`health`, `discover_capabilities`, and `storage.*` methods.

**What required adaptation**:

| Issue | Root Cause | Fix Applied |
|---|---|---|
| Subcommand | nestgate uses `daemon` not `server` | Added `subcommand` field to `LaunchProfile` |
| Socket path | nestgate ignores `--socket`; uses `NESTGATE_SOCKET` env var | Added `env_sockets` mapping in launch profile |
| JWT secret | nestgate requires `NESTGATE_JWT_SECRET` (min length) | Added to `extra_env` in launch profile |
| ZFS hard-fail | nestgate crashes if ZFS kernel module not loaded | Fixed `StorageState::new()` fallback to dev config (committed to nestgate repo) |
| `socket_only` pattern | Pre-existing compile error in `cli.rs` | Fixed `Commands::Daemon` destructure (committed to nestgate repo) |

**New gates validated** (8/8):
- nestgate starts via socket-only mode, responds to `health`
- `discover_capabilities` returns storage capabilities
- `storage.store` + `storage.retrieve` round-trip
- `storage.list` + `storage.exists` consistency
- `model.register` + `model.locate` cache operations
- Direct health check and capability discovery

### 1.2 Node Atomic (toadstool integration)

**Composition**: beardog + songbird + biomeOS + toadstool

**What required adaptation**:

| Issue | Root Cause | Fix Applied |
|---|---|---|
| Dual-protocol socket | toadstool creates tarpc primary + `.jsonrpc.sock` secondary | Added `jsonrpc_socket_suffix` to `LaunchProfile` |
| Socket path | toadstool ignores `--socket`; uses `TOADSTOOL_SOCKET` env var | Added `env_sockets` mapping |
| Socket remap | Harness needs to connect to `.jsonrpc.sock`, not primary | Added `SocketNucleation::remap()` |
| Method naming | Uses `toadstool.health`, not `health.liveness` | Extended health fallback chain to try `{primal}.health` |
| Capabilities | Uses `toadstool.query_capabilities`, not `capabilities.list` | Tests call prefixed methods directly |
| Security ack | Requires `TOADSTOOL_SECURITY_WARNING_ACKNOWLEDGED=1` | Added to `extra_env` |

**New gates validated** (5/5):
- toadstool starts, health check via `toadstool.health`
- `toadstool.query_capabilities` returns 4 workload types, 24 CPU cores
- `toadstool.version` reporting
- Node composition validation

### 1.3 NUCLEUS Composition

All 3 atomic layers compose together. exp068 starts Tower, then Nest, then Node
and validates cross-layer connectivity. **58/58 total gates passing.**

### 1.4 Harness Enhancements

| Enhancement | Purpose |
|---|---|
| `LaunchProfile.subcommand` | Override default `"server"` arg (nestgate uses `"daemon"`) |
| `LaunchProfile.jsonrpc_socket_suffix` | Handle dual-protocol primals (toadstool `.jsonrpc.sock`) |
| `LaunchProfile.env_sockets` | Pass socket paths via env vars instead of CLI flags |
| `SocketNucleation::remap()` | Update socket mapping post-spawn for rerouted endpoints |
| Health fallback chain | `health.liveness` -> `health.check` -> `health` -> `{primal}.health` |
| `socket_flag = "__skip__"` | Disable `--socket` CLI arg when primal uses env vars |

---

## 2. What the toadStool Team Should Absorb

### P0: Adopt Standard RPC Methods (Ecosystem Alignment)

toadstool uses **prefixed** method names (`toadstool.health`, `toadstool.query_capabilities`).
This works but diverges from the ecosystem convention. Other primals can't discover
toadstool through generic `health.liveness` or `capabilities.list` calls without
special-casing.

**Recommendation**: register **aliases** for the standard methods:

| Standard Method | Current toadstool Method | Action |
|---|---|---|
| `health.liveness` | `toadstool.health` | Add alias (keep both) |
| `capabilities.list` | `toadstool.query_capabilities` | Add alias (keep both) |

primalSpring's health fallback chain works around this, but the 4-deep fallback is
a workaround, not a design choice.

### P0: Accept `--socket` CLI Argument

toadstool currently ignores `--socket` and only reads `TOADSTOOL_SOCKET` env var.
All other primals accept a `--socket PATH` CLI argument. Adding this would simplify
integration and eliminate the need for `env_sockets` + `socket_flag = "__skip__"`.

### P1: Single JSON-RPC Socket (or Document Dual-Protocol)

toadstool creates two sockets:
1. **Primary**: tarpc binary RPC (for internal high-performance calls)
2. **Secondary**: `.jsonrpc.sock` suffix (for ecosystem JSON-RPC 2.0)

This is a valid architecture for performance, but it creates friction:
- Harness must know about the suffix to wait for the right socket
- `SocketNucleation::remap()` was added solely for this pattern
- Other primals can't discover the JSON-RPC endpoint without convention knowledge

**Options**:
1. Document the dual-protocol pattern formally so other primals can adopt it
2. Have toadstool emit both on a single socket (multiplex)
3. Have the JSON-RPC socket be the "primary" and tarpc the "secondary"

### P1: GPU/Compute Environment Passthrough

The launch profile already forwards GPU-related env vars. toadstool should document
which variables control backend selection:

```toml
[profiles.toadstool.passthrough_env]
CUDA_VISIBLE_DEVICES = true
WGPU_BACKEND = true
WGPU_ADAPTER_NAME = true
VK_ICD_FILENAMES = true
```

### P2: MCP Tool Surface

The Squirrel AI composition pattern (exp061) proves that MCP tool discovery works
end-to-end. toadstool's 4 workload types should be surfaced as MCP tools:

| Workload Type | MCP Tool Name | Description |
|---|---|---|
| (from query_capabilities) | `compute.dispatch` | Submit workload for execution |
| (from query_capabilities) | `compute.status` | Check workload status |
| (from query_capabilities) | `compute.capabilities` | Query available compute |
| (from query_capabilities) | `compute.version` | Runtime version info |

---

## 3. What the barraCuda Team Should Absorb

### 3.1 Still Zero Direct Dependency

primalSpring has zero barraCuda imports and will not contribute math or shaders.
The relationship is indirect: primalSpring validates coordination, toadStool
dispatches to barraCuda for compute.

### 3.2 The Compute Chain Is Now Live

With Node Atomic validated, the compute dispatch chain is operational:

```
primalSpring (coordination) → AtomicHarness → toadStool (compute runtime)
                                                  ↓
                                            barraCuda (math kernels)
```

exp050 (compute triangle) models the full `coralReef → toadStool → barraCuda`
pipeline. Now that toadStool is live-validated in the harness, this experiment
can be promoted from capability-based scaffolding to live validation.

### 3.3 Provenance for Compute Results

The `Provenance { source, baseline_date, description }` struct is available for
tagging compute results. When barraCuda produces results, they should carry:
- Which shader was executed
- Which backend (WGPU, CUDA, CPU fallback)
- Precision mode (f32, f64, mixed)
- Device identity

---

## 4. What the nestgate Team Should Absorb

### P0: ZFS Graceful Degradation (Already Fixed)

We committed a fix to nestgate that falls back to development config (filesystem
mode) when ZFS kernel module is not loaded. This is correct behavior for non-ZFS
systems. **Commit**: `29f3997d` on nestgate main.

### P0: Accept `--socket` CLI Argument

Same recommendation as toadstool: nestgate reads `NESTGATE_SOCKET` env var but
doesn't accept `--socket PATH` on the CLI. Adding this would simplify harness
integration.

### P1: `health.liveness` Alias

nestgate uses `health` (not `health.liveness`). Adding `health.liveness` as an
alias would align with the ecosystem convention without breaking existing callers.

### P1: JWT Secret Configuration

nestgate requires `NESTGATE_JWT_SECRET` and enforces minimum length. This is good
security practice. Document the minimum length requirement (currently appears to be
~32 characters) so automated systems can generate compliant secrets.

---

## 5. Learnings Relevant to All Teams

### 5.1 Environment-Based Socket Discovery > CLI Flags

Both nestgate and toadstool ignore `--socket` CLI arguments and read socket paths
from environment variables. This is the emerging pattern, and it works well with
the harness's `env_sockets` feature. But it diverges from the `--socket PATH`
convention used by beardog, songbird, and squirrel.

**Recommendation**: support BOTH. Accept `--socket PATH` as CLI override, fall
back to `{PRIMAL}_SOCKET` env var, then fall back to the 5-tier discovery chain.

### 5.2 Dual-Protocol Primals Need a Convention

toadstool's tarpc + JSON-RPC dual-socket approach is a valid performance pattern.
If more primals adopt it, the ecosystem needs a convention:
- Naming: `{primal}.sock` (primary) + `{primal}.jsonrpc.sock` (JSON-RPC)
- Discovery: the JSON-RPC endpoint should be discoverable through standard means
- Health: the JSON-RPC endpoint should respond to `health.liveness`

### 5.3 Parallel Test Execution is Non-Negotiable

All 31 integration tests run in parallel in ~5 seconds. This is only possible
because:
- Family IDs include `std::process::id()` for uniqueness
- Socket paths are per-family (no conflicts)
- All primals use `--port 0` or don't bind TCP at all
- No global state mutation (`set_var` is unsafe in Rust 2024)

### 5.4 Honest Degradation Over Fake Passes

nestgate's ZFS fallback is a good example. On systems without ZFS, nestgate now
degrades to filesystem storage and reports this honestly. primalSpring's tests
validate storage.store/retrieve works regardless of backend.

---

## 6. Evolution Path

```
Tower Atomic (beardog + songbird + biomeOS)     FULLY UTILIZED (41/41)
    ↓
Nest Atomic (Tower + nestgate)                  VALIDATED (8/8)
    ↓
Node Atomic (Tower + toadstool)                 VALIDATED (5/5)
    ↓
NUCLEUS Composition (Tower + Nest + Node)       VALIDATED (58/58) ← YOU ARE HERE
    ↓
Full NUCLEUS (+ Squirrel + provenance trio)     NEXT
    Requires: Squirrel AI + sourDough + sweetGrass + fieldMouse
    Validates: Full composition with AI coordination + provenance
```

---

## 7. Metrics

| Metric | Value |
|--------|-------|
| primalSpring version | v0.6.0 |
| Total tests | 282 (239 unit + 31 integration + 2 doc + 10 ignored) |
| Integration tests (live) | 31 (19 Tower + 8 Nest + 4 Node) |
| Experiments | 47 (8 tracks) |
| Stability gates | 58/58 (Tower 41 + Nest 8 + Node 5 + NUCLEUS 4) |
| Deploy graphs with toadStool | 7 |
| Deploy graphs with nestgate | 4 |
| Registered capabilities | 37 |
| Harness adaptations for nestgate | 5 (subcommand, env socket, JWT, ZFS fix, compile fix) |
| Harness adaptations for toadstool | 6 (dual socket, env socket, security ack, method prefix, suffix, remap) |

---

## 8. References

- `experiments/exp066_nest_atomic/` — Nest Atomic storage validation (13/13 PASS)
- `experiments/exp067_node_atomic/` — Node Atomic compute validation (13/13 PASS)
- `experiments/exp068_full_nucleus/` — NUCLEUS composition validation (16/16 PASS)
- `experiments/exp050_compute_triangle/` — compute coordination template
- `config/primal_launch_profiles.toml` — nestgate and toadstool profiles
- `ecoPrimal/src/launcher/mod.rs` — `spawn_primal` with subcommand, env_sockets, remap
- `ecoPrimal/src/ipc/client.rs` — health fallback chain
- `specs/TOWER_STABILITY.md` — 58/58 gate definitions and evidence
- `specs/BARRACUDA_REQUIREMENTS.md` — confirms zero direct dependency

---

**License**: AGPL-3.0-or-later
