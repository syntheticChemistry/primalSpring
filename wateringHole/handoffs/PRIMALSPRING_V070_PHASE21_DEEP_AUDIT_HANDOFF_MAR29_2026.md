# SPDX-License-Identifier: AGPL-3.0-or-later

# primalSpring v0.7.0 — Phase 21: Deep Ecosystem Audit + Library Consolidation

**Date**: March 29, 2026
**From**: primalSpring coordination team
**To**: All primal teams + all spring teams + biomeOS + provenance trio

---

## Summary

Phase 21 executed a comprehensive 8-axis audit of primalSpring against ecosystem
standards (`wateringHole/`), followed by full remediation. The audit covered:
completion status, code quality, validation fidelity, barraCuda dependency health,
GPU evolution readiness, test coverage, ecosystem standards, and primal coordination.

**Result**: Zero TODOs/FIXMEs/HACKs, zero clippy warnings (pedantic+nursery), zero
`#[allow()]` in production, zero unsafe, zero C deps, 411 tests passing, 63 experiments
with 100% provenance, 59 deploy graphs.

---

## What Changed

### New Library Modules

| Module | Purpose | Replaces |
|--------|---------|----------|
| `ipc::tcp` | Shared TCP RPC: `tcp_rpc()`, `tcp_rpc_with_timeout()`, `http_health_probe()`, `env_port()` | Per-experiment TCP boilerplate in exp063, exp073, exp074, exp076, exp081–084 |
| `ipc::methods` | Centralized JSON-RPC method name constants (nested modules: `health`, `capabilities`, `coordination`, `provenance`, `graph`, `composition`, `lifecycle`, `mcp`) | Hardcoded string literals across experiments |
| `ipc::capability` | Capability discovery + routing (extracted from `ipc/discover.rs`) | Monolithic discover module |

### Refactored Modules

| Module | Change | LOC Before → After |
|--------|--------|-------------------|
| `launcher/` | Split into `discovery.rs`, `profiles.rs`, `spawn.rs`, `biomeos.rs` | 1 file → 4 sub-modules, public API preserved |
| `ipc/discover.rs` | Capability logic extracted to `ipc/capability.rs` | Monolithic → focused |
| `ipc/provenance.rs` | Time-based half-open circuit breaker, graceful mutex poisoning | Threshold-only → epoch + probe token |
| `ipc/client.rs` | Uses `Transport` enum internally | Raw `UnixStream` → unified Unix+TCP |
| `harness/mod.rs` | `println!` → `tracing::info!`/`tracing::debug!` | Direct stdout → structured logging |
| `validation/or_exit.rs` | `eprintln!` → `tracing::error!` | Direct stderr → structured logging |

### Experiments Consolidated

8 experiments refactored to use library helpers instead of local implementations:

| Experiment | Removed Local Code | Now Uses |
|------------|-------------------|----------|
| exp063 | `tcp_rpc_call` | `ipc::tcp::tcp_rpc` |
| exp073 | `tcp_rpc_call`, `http_health_check` | `ipc::tcp::tcp_rpc`, `ipc::tcp::http_health_probe` |
| exp074 | `tcp_rpc`, `http_health` | `ipc::tcp::tcp_rpc`, `ipc::tcp::http_health_probe` |
| exp076 | `tcp_rpc` | `ipc::tcp::tcp_rpc` |
| exp081 | `tcp_rpc`, `http_health` | `ipc::tcp::tcp_rpc`, `ipc::tcp::http_health_probe` |
| exp082 | `tcp_rpc` | `ipc::tcp::tcp_rpc`, `ipc::tcp::tcp_rpc_with_timeout` |
| exp083 | `tcp_rpc` | `ipc::tcp::tcp_rpc` |
| exp084 | `tcp_rpc` | `ipc::tcp::tcp_rpc` |

All method name strings replaced with `ipc::methods::*` constants.
All primal name strings replaced with `primal_names::*` slug constants.

---

## Primal Team Actions

### biomeOS Team

**P0 — TCP transport gap**: `biomeOS api --port N` is ignored; the server always
binds to Unix socket. This blocks all TCP-only deployment matrix cells (Android,
cross-gate federation). primalSpring's `ipc::tcp` module is ready to consume TCP
endpoints the moment biomeOS supports them.

**P1 — Cross-gate `gate` field**: `capability.call` responses should include the
`gate` identifier so callers can route to specific gates. primalSpring's deploy
graphs already declare `gate` metadata on multi-node compositions.

**P1 — Standalone neural-api mode**: Allow `neural-api-server` to run without full
biomeOS daemon for lightweight testing and spring validation.

### Squirrel Team

**P0 — Abstract socket vs filesystem**: Squirrel uses Linux abstract sockets
(`@squirrel`), but primalSpring's 5-tier discovery expects filesystem sockets
under `$XDG_RUNTIME_DIR/biomeos/`. Either Squirrel should also listen on a
filesystem socket, or the discovery tiers need an abstract socket probe.

**P1 — MCP tool schema evolution**: primalSpring registers 8 MCP tools via
`mcp.tools.list`. As Squirrel evolves its tool discovery, the JSON Schema
definitions should converge on a shared schema version.

### Songbird Team

**Healthy** — Songbird is the most mature primal from primalSpring's perspective.
TCP transport (`--listen`), BirdSong beacons, subsystem health, cross-gate
federation all validated. No blocking actions.

**Nice-to-have**: Expose `health.liveness` as a distinct lightweight probe
(currently aliases to `health.check` with full subsystem enumeration).

### BearDog Team

**P0 — TCP transport**: BearDog has no `--listen` mode. Cross-gate federation
(Pixel, remote gates) is blocked. primalSpring exp063 and exp076 fall back to
skip when BearDog isn't reachable over TCP.

### petalTongue Team

**P1 — Dialogue-tree scene type**: RPGPT storytelling stack expects a
`scene_type: "dialogue_tree"` rendering mode. Current petalTongue supports
`dashboard` and `grammar_of_graphics` but not branching dialogue.

### NestGate Team

**Healthy** — NestGate storage round-trips validated. ZFS graceful degradation
working. Family-scoped sockets working. No blocking actions.

### ToadStool Team

**Healthy** — Dual-protocol socket (tarpc + JSON-RPC) validated. Compute
capabilities probed. No blocking actions from primalSpring.

### Provenance Trio (rhizoCrypt, LoamSpine, sweetGrass)

**P1 — Circuit breaker alignment**: primalSpring's provenance module now uses
an epoch-based circuit breaker with half-open probing. Trio primals should
honor `health.liveness` for fast circuit-breaker probe responses (avoid
full session creation overhead on half-open probes).

**P1 — Session throughput**: exp084 provenance adversarial tests tampered DAG
detection and replay attacks. Trio primals should ensure these error paths
return structured JSON-RPC error codes (not connection resets).

---

## Spring Team Actions

### All Springs

The 8-axis audit methodology used in Phase 21 is documented in
`PRIMALSPRING_V070_ECOSYSTEM_AUDIT_GUIDANCE_HANDOFF_MAR27_2026.md` and is
reusable. Key patterns absorbed:

1. **`ipc::methods` pattern** — centralize all JSON-RPC method name strings as
   `pub const` in a `methods` module. Eliminates drift between server dispatch
   and client calls.

2. **`ipc::tcp` pattern** — if your experiments use TCP RPC to probe primals,
   extract the boilerplate into a library module with configurable timeouts
   from `tolerances/`.

3. **`primal_names` pattern** — all primal name references should use typed
   constants, not string literals. This catches typos at compile time.

4. **Launcher sub-module pattern** — when a module exceeds ~500 LOC, split by
   responsibility (discovery, profiles, spawn) rather than arbitrary line count.

5. **Tracing migration** — library code should use `tracing` crate macros, not
   `println!`/`eprintln!`. Validation harness banner/summary output is the
   exception (user-facing terminal output).

6. **Circuit breaker half-open** — for any circuit breaker protecting external
   IPC, implement time-based half-open with a single probe token. Avoids
   permanent open circuits when transient failures resolve.

### ludoSpring Team

**P0 — esotericWebb IPC surface**: esotericWebb expects 6 `game.*` methods
that ludoSpring doesn't yet implement. The deployment matrix storytelling
cells are blocked on this.

### airSpring Team

**P1 — Edition/MSRV alignment**: airSpring uses Rust edition 2021 / MSRV 1.92.
The ecosystem is converging on edition 2024. Consider upgrading when stable.

---

## Ecosystem Learnings

### What Worked

- **Capability-first discovery** eliminates cross-primal coupling. primalSpring
  never hardcodes which primal provides a capability — it discovers at runtime.
- **Deploy graphs as source of truth** — all 59 graphs have `by_capability` on
  every node. Graphs define compositions, not code.
- **Honest scaffolding** — `check_skip()` prevents false passes when providers
  are absent. Zero dishonest scaffolding across 63 experiments.
- **Named tolerances** — every latency/throughput bound is a named constant in
  `tolerances/` with calibration provenance.

### What Needs Evolution

- **Transport story fragmentation**: Unix sockets (primalSpring default), TCP
  (cross-gate, Android), abstract sockets (Squirrel) — needs ecosystem-wide
  transport negotiation protocol.
- **Method name convergence**: different primals use different prefixes for the
  same operation (`health.check` vs `{primal}.health`). `normalize_method()`
  handles this, but upstream convergence would be cleaner.
- **Spring primal binary maturity**: 5/6 spring primals built for plasmidBin,
  but none are production-hardened. Each spring team should own their primal
  binary and publish to plasmidBin.

---

## Metrics

| Metric | Phase 20 | Phase 21 | Delta |
|--------|----------|----------|-------|
| Tests | 385 | 411 | +26 |
| Experiments | 63 | 63 | — |
| Deploy graphs | 59 | 59 | — |
| Clippy warnings | 0 | 0 | — |
| TODOs/FIXMEs | 0 | 0 | — |
| `#[allow()]` | 0 | 0 | — |
| Library modules | ~15 | ~19 | +4 (tcp, methods, capability, launcher sub-modules) |
| Local TCP helpers | 8 copies | 0 | Consolidated to library |
| Hardcoded method strings | ~40 | 0 | Consolidated to ipc::methods |
| Hardcoded primal names | ~12 | 0 | Consolidated to primal_names |

---

**License**: AGPL-3.0-or-later
