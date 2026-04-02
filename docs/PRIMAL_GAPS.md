# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: Primal-only gaps relevant to primalSpring's upstream role. Downstream systems
> (gardens, springs) own their own debt and pick up patterns from `wateringHole/`.
>
> **Last updated**: 2026-04-02 — Re-audit cycle: gap re-verification, overstep scan, compliance recheck.
> 19 gaps resolved, 8 open (zero critical, zero high). 3 gaps improved this cycle (NG-01, SB-02, SB-03).
> Overstep scan confirms PRIMAL_RESPONSIBILITY_MATRIX — no new boundary violations found.

---

## biomeOS

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| BM-01 | `graph.deploy` routing | **RESOLVED** (v2.79 — `graph.execute`) |
| BM-02 | `health.liveness` on Neural API | **RESOLVED** (v2.81) |
| BM-03 | `unix://` prefix on `capability.discover` | **RESOLVED** (v2.79 — `strip_unix_uri`) |
| BM-04 | Late primal registration invisible | **RESOLVED** (v2.81 — `topology.rescan` + lazy discovery) |
| BM-05 | Multi-shape probe response | **RESOLVED** (v2.81) |

**Compliance**: clippy 1 warning (unused imports in `sweet-grass-service` test), fmt clean, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, tests pass, SPDX headers present.

---

## petalTongue

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| PT-01 | Socket at non-standard path | **RESOLVED** — `biomeos/petaltongue.sock` |
| PT-02 | No live push to browser | **RESOLVED** — SSE `/api/events` |
| PT-03 | `motor_tx` not wired in server mode | **RESOLVED** — drain channel wired |
| PT-04 | No `ExportFormat::Html` in headless CLI | Low | Partially — IPC `compile_html` wraps SVG-in-HTML (PT-04 tag), but `ExportFormat` enum lacks `Html` variant |
| PT-05 | `visualization.showing` returns false | **RESOLVED** — `RenderingAwareness` auto-init in `UnixSocketServer` |
| PT-06 | `callback_method` poll-only dispatch | Low | Open — `CallbackDispatch` struct + tests exist, but server `apply_interaction` discards broadcast results; push delivery not wired |
| PT-07 | No external event source in server mode | **RESOLVED** — periodic discovery refresh wired |

**Compliance**: clippy **3 warnings**, fmt clean, `forbid(unsafe_code)` per-crate (not workspace-level), `deny.toml` present, SPDX headers present. **Tests PASS** (previously 1 failure resolved). `#[expect]` migration (commit 158c21a) in progress.

---

## barraCuda

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| BC-01 | Fitts formula variant | **RESOLVED** (Sprint 25 — `variant` param, Shannon default) |
| BC-02 | Hick formula off-by-one | **RESOLVED** (Sprint 25 — `include_no_choice` param) |
| BC-03 | Perlin3D lattice | **RESOLVED** (Sprint 25 — proper gradients + quintic fade) |
| BC-04 | No plasmidBin binary | **RESOLVED** (April 1 harvest, 4.5M, requires GPU) |

---

## Squirrel

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SQ-01 | Abstract-only socket | **RESOLVED** (alpha.25b — `UniversalListener`) |
| SQ-02 | `LOCAL_AI_ENDPOINT` not wired into `AiRouter` | **RESOLVED** (alpha.27 — step 1.5 discovery, `resolve_local_ai_endpoint()`) |
| SQ-03 | `deprecated-adapters` feature flag docs | **RESOLVED** — documented in `CURRENT_STATUS.md` feature-gates table; intentional retention until v0.3.0 with migration path to `UniversalAiAdapter` + `LOCAL_AI_ENDPOINT` |

**Compliance**: clippy **2 warnings** (minor unfulfilled lint expectations), fmt clean, `deny.toml` present, tests pass. **Note**: no `forbid(unsafe_code)` at workspace manifest level (uses clippy groups instead). SPDX headers present. `deprecated-adapters` feature still present (empty, documented in CURRENT_STATUS.md).

---

## songBird

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SB-01 | `health.liveness` canonical name | **RESOLVED** (wave89-90) |
| SB-02 | CLI `ring-crypto` opt-in feature | Low | Near-resolved — `rcgen` removed from lockfile (wave93); `ring` still in `Cargo.lock` but **not compiled** in default build; `ring-crypto` is opt-in CLI feature with single `cfg`-gated call. Default uses `rustls_rustcrypto`. Lockfile refresh would remove stale `ring` stanza |
| SB-03 | `sled` in orchestrator/sovereign-onion/tor | Low | Improved — wave93 feature-gated sled (`optional = true` + `dep:sled`) in all 3 crates. `sled-storage` default-on in orchestrator + sovereign-onion; opt-in `persistent-cache` for tor. Pending NestGate storage API |

**Compliance**: clippy **8 warnings** (down from 395 — massive improvement, wave93 ring elimination + concurrency fix), fmt clean, `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX headers present. Tests pass.

---

## NestGate

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| NG-01 | IPC storage backend inconsistency | Low | Improved — `StorageBackend` trait injection wired for tarpc object storage (`with_backend`/`with_backend_arc`). Semantic router delegates to `NestGateRpcClient`. **Residual**: `InMemoryMetadataBackend` is default for metadata axis; callers must inject filesystem-backed `MetadataBackend` |
| NG-02 | Session API inconsistency | Low | Improved — `session.save`/`session.load` work on unix-socket + isomorphic paths; `session.list`/`session.delete` on isomorphic only. **Inconsistency**: `capabilities.rs` advertises all `session.*` but `SemanticRouter::call_method` has no `session.*` arms — semantic callers hit "unknown method" |
| NG-03 | `data.*` handlers still stubs | Low | Open — both `unix_socket_server/data_handlers.rs` and `semantic_router/data.rs` return `not_implemented`. Message improved ("configure NESTGATE_EXTERNAL_*") but no nestgate-core wiring |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated, TLS delegated to system `curl` |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

**Compliance**: clippy **16 warnings** + test compile errors (regression — `nestgate-zfs` unresolved imports, `nestgate_core::constants` missing fields). fmt clean. `forbid(unsafe_code)` per-crate + workspace `deny`. **`deny.toml` now PRESENT** (was missing). SPDX headers present. Tests: **RED** — compile errors in test targets block full suite.

---

## rhizoCrypt

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| RC-01 | TCP-only transport | **RESOLVED** (v0.14.0-dev s23 — `--unix`, `UdsJsonRpcServer`, `biomeos/` path) |

**Compliance**: clippy clean, fmt clean, `deny(unsafe_code)` + `forbid` in non-test builds via `cfg_attr`, `deny.toml` present, tests pass.

---

## loamSpine

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| LS-03 | Panic on startup | **RESOLVED** (v0.9.15 — infant discovery fails gracefully) |

**Compliance**: clippy clean, **fmt now PASSES** (previously failing — fixed), `forbid(unsafe_code)` at workspace level, `deny.toml` present, tests pass.

---

## toadStool

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| TS-01 | coralReef discovery not pure capability-based | Low | Improved — S166 capability-based discovery added (`discover_by_capability` in `service_discovery.rs`), but `coral_reef_client` still uses explicit 6-step ordered discovery, not unified `capability.discover` RPC |

**Compliance**: clippy **25 warnings**, **fmt STILL FAILS** (persistent formatting debt), no `forbid(unsafe_code)` at workspace level, `deny.toml` present. **Tests PASS** (previously 1 failure resolved; full suite ~19.6 min).

---

## sweetGrass

All gaps **RESOLVED**. TCP JSON-RPC added, `cargo-deny`, `forbid(unsafe)`.

**Compliance**: clippy 1 warning (unused imports in `tcp_jsonrpc.rs` test), fmt clean, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, tests pass.

---

## coralReef

No gaps identified.

---

## bearDog

No gaps identified.

**Compliance**: clippy clean, fmt clean, `forbid(unsafe_code)` at workspace level, `deny.toml` present, SPDX headers present, tests pass (14,201+). Gold standard for ecosystem compliance.

---

## Priority Order

**ZERO CRITICAL / HIGH / MEDIUM BLOCKERS.**

All 8 open gaps are **Low** severity — polish items owned by primal teams.

**Low** (polish, owned by primal teams):
1. **NG-01** — NestGate metadata backend injection (object storage wired; metadata axis still in-memory default)
2. **NG-02** — Session API inconsistency (capabilities advertise `session.*` but semantic router doesn't dispatch)
3. **NG-03** — `data.*` handler stubs (documented separation, no nestgate-core wiring)
4. **PT-04** — `ExportFormat::Html` headless variant (IPC modality already works)
5. **PT-06** — callback push dispatch (struct exists, wiring needed)
6. **SB-02** — `ring` lockfile ghost (not compiled in default build; lockfile refresh clears it)
7. **SB-03** — `sled` feature-gated but default-on in orchestrator/sovereign-onion (pending NestGate API)
8. **TS-01** — coralReef pure `capability.discover` (6-step heuristic works, not unified)

---

## Guideline Compliance Matrix

| Primal | Clippy | Fmt | `forbid(unsafe)` | `deny.toml` | SPDX | Tests |
|--------|--------|-----|-------------------|-------------|------|-------|
| biomeOS | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS |
| BearDog | CLEAN | PASS | workspace forbid | YES | YES | PASS (14K+) |
| Songbird | **8 warn** ↓ | PASS | per-crate forbid | YES | YES | PASS |
| NestGate | **16 warn** ↑ | PASS | deny + per-crate forbid | **YES** ↑ | YES | **RED** ↓ |
| petalTongue | **3 warn** | PASS | per-crate forbid | YES | YES | **PASS** ↑ |
| Squirrel | **2 warn** | PASS | **absent** | YES | YES | PASS |
| toadStool | **25 warn** | **FAIL** | **absent** | YES | n/c | **PASS** ↑ |
| sweetGrass | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS |
| rhizoCrypt | CLEAN | PASS | deny + cfg_attr forbid | YES | n/c | PASS |
| loamSpine | CLEAN | **PASS** ↑ | workspace forbid | YES | n/c | PASS |
| barraCuda | n/c | n/c | n/c | n/c | n/c | n/c |
| coralReef | n/c | n/c | n/c | n/c | n/c | n/c |

**Legend**: n/c = not checked this cycle; ↑ = improved since last audit; ↓ = regressed

### Compliance Evolution (April 2 re-audit)

1. **BearDog** remains gold standard: workspace `forbid(unsafe_code)`, clippy clean, fmt clean, deny.toml, all tests pass
2. **Songbird** massive improvement: 395 → 8 clippy warnings (wave93 ring elimination, concurrency fixes)
3. **loamSpine** fmt now passes (was failing — fixed in v0.9.16 deep debt evolution)
4. **NestGate** regressed: was clippy-clean, now 16 warnings + test compile errors (`nestgate-zfs`, `nestgate_core::constants`). **BUT** `deny.toml` now present (was the only primal without one)
5. **petalTongue** tests now pass (was 1 failure); `#[expect]` migration in progress
6. **toadStool** tests now pass (was 1 failure), but **fmt still fails** — persistent formatting debt
7. **Squirrel** and **toadStool** still lack `forbid(unsafe_code)` at workspace level

---

## Resolved Gaps Summary

| ID | Primal | What Was Fixed | Resolved In |
|----|--------|---------------|-------------|
| BM-01–05 | biomeOS | Graph routing, health, discovery, multi-shape | v2.79–v2.81 |
| BC-01–04 | barraCuda | Fitts/Hick/Perlin fixes, plasmidBin harvest | Sprint 25 |
| PT-01–03, PT-05, PT-07 | petalTongue | Socket, SSE, motor_tx, awareness init, server discovery | IPC compliance evolution |
| SQ-01–03 | Squirrel | Filesystem socket, `LOCAL_AI_ENDPOINT`, feature flag docs | alpha.25b–27 |
| SB-01 | songBird | `health.liveness` canonical | wave89-90 |
| NG-04–05 | NestGate | ring/aws-lc-rs eliminated, crypto delegated to BearDog | deep debt evolution |
| RC-01 | rhizoCrypt | UDS transport + biomeos/ path | v0.14.0-dev s23 |
| LS-03 | loamSpine | Startup panic → graceful degradation | v0.9.15 |

**19 gaps resolved** across the full cycle. **8 open** (all low). Zero critical, zero medium.

---

## Overstep Audit (April 2, 2026)

Cross-referenced against `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md`. No new boundary violations found.

| Primal | Overstep Status | Detail |
|--------|----------------|--------|
| biomeOS | **Known** | `redb` in `biomeos-graph` (metrics storage) — borderline operational state vs NestGate domain |
| BearDog | **Known** | `axum` in `beardog-integration` (HTTP); AI/neural tree in `beardog-core` (~36 files) |
| Songbird | **Known** | `sled` persistence in orchestrator/sovereign-onion (SB-03, now feature-gated) |
| NestGate | **Known** | Crypto, discovery, network, MCP, orchestration — all documented in matrix; `nestgate-security` crypto delegated to BearDog (NG-05 RESOLVED) |
| toadStool | **Cleaned (S169)** | 30+ methods + 10,659 lines removed; only TS-01 (coralReef discovery) + security sandbox remain |
| sweetGrass | **CLEAN** | Own storage crates (sled/redb) are in domain |
| rhizoCrypt | **CLEAN** | TCP listener is standard dual-mode IPC per protocol, not networking overstep |
| loamSpine | **CLEAN** | TCP/HTTP listeners are standard IPC dual-mode per protocol |
| petalTongue | **CLEAN** | axum serves visualization UI (in domain); no embedded compute/storage/AI |
| Squirrel | **Known** | `sled`/`sqlx` persistence, `ed25519-dalek`/TLS — documented; broader than "cache only" |

---

## plasmidBin Inventory

| Binary | Size | Source | UDS | Notes |
|--------|------|--------|-----|-------|
| beardog | 7.1M | musl-static | yes | Mar 27 |
| biomeos | 12M | musl-static | yes | Mar 28 |
| songbird | 16M | musl-static | yes | Mar 27 |
| squirrel | 5.8M | musl-static | yes | Mar 27 |
| petaltongue | 30M | musl-static | yes | Mar 28 |
| nestgate | 4.9M | musl-static | yes | Mar 28 |
| toadstool | 16M | musl-static | yes | Mar 27 (S168 binary — S171 needs rebuild) |
| rhizocrypt | 5.4M | glibc | yes | April 1 — RC-01 fix |
| loamspine | 6.9M | glibc | yes | April 1 — LS-03 fix |
| sweetgrass | 8.8M | glibc | yes | April 1 |
| barracuda | 4.5M | glibc | N/A | April 1 — requires GPU |

**Note**: rhizoCrypt/loamSpine/sweetGrass/barraCuda are glibc dynamic — musl-static cross-compile needed for containers.

---

## primalSpring Rewiring Status (April 1, 2026)

| Area | Status |
|------|--------|
| `methods.rs` | Aligned — `graph.execute`, `topology.rescan`, `ember.*`, `shader.compile` removed, `ai.*`, `visualization.*`, `interaction.*` added |
| `NeuralBridge` | Aligned — `topology_rescan()` added, `graph.execute` call correct |
| `discover.rs` | Aligned — plain socket name discovery (`{name}.sock`, `{name}-ipc.sock`) added |
| `capability.rs` | Aligned — 4-format parsing, `strip_unix_uri`, multi-shape |
| `validate_compositions.py` | Aligned — SQ-02 messaging updated, NestGate `family_id`, C7 Squirrel check live |
| Composition graphs (C1–C7) | Clean — no stale references |
| Cargo.toml | `edition = "2024"`, `rust-version = "1.87"` |
| Tests | 403 pass (10/10 unit, 4/4 doc-tests) |

---

## Live Validation Results (April 1, 2026 — post-rewiring)

```
  C1: Render                           6/6  PASS
  C2: Narration                        3/4  PARTIAL (ai.query — no local Ollama running)
  C3: Session                          8/8  PASS
  C4: Game Science                     6/6  PASS
  C5: Persistence                      5/5  PASS
  C6: Proprioception                   5/5  PASS
  C7: Full Interactive                 10/10 PASS

  TOTAL                                43/44  (98%)
```

Previous: 41/44 (93%) → **43/44 (98%)** after rewiring and pull.

The single remaining failure (`ai.query`) is an **environment dependency** — Squirrel's `AiRouter` is now correctly wired (SQ-02 resolved), but no local Ollama/llama.cpp instance is running. With Ollama serving a model at `localhost:11434`, C2 would reach 4/4.
