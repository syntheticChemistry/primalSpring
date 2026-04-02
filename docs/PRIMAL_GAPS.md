# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: Primal-only gaps relevant to primalSpring's upstream role. Downstream systems
> (gardens, springs) own their own debt and pick up patterns from `wateringHole/`.
>
> **Last updated**: 2026-04-02 (PM) — Evolution pull + re-audit. Major progress across all tiers.
> 20 gaps resolved (+NG-02), 7 open (zero critical, zero high, zero medium).
> BearDog AI tree feature-gated. Squirrel 49K orphan lines removed, 0 todo!/unimplemented!.
> NestGate compile fixed + NG-02 resolved. toadStool fmt+clippy clean. petalTongue CHANGELOG added.

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
| PT-04 | No `ExportFormat::Html` in headless CLI | Low | Partially — `ExportFormat::Html` exists in headless path + IPC; needs product validation |
| PT-05 | `visualization.showing` returns false | **RESOLVED** — `RenderingAwareness` auto-init in `UnixSocketServer` |
| PT-06 | `callback_method` poll-only dispatch | Low | Code-complete — `push_delivery.rs` module, `broadcast()`, `CallbackDispatch` wired; **not enabled on live server** (`callback_tx` = `None` at startup; server wiring needed to activate push) |
| PT-07 | No external event source in server mode | **RESOLVED** — periodic discovery refresh wired |

**Compliance**: clippy status pending (disk space issue on audit host), fmt clean, `forbid(unsafe_code)` per-crate (not workspace-level), `deny.toml` present, SPDX headers present. Tests PASS. Sensory capability matrix + accessibility adapters added (in-domain UUI). **CHANGELOG.md added** (was missing). Zero stubs.

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

**Compliance** (alpha.29): 50+ unused deps removed across 13 crates. 112 orphan files (42K lines) deleted from uncompiled module trees. **Zero `todo!()` / `unimplemented!()`** (was 14 + 4). All hardcoded localhost/ports → discovery-first patterns. MockAIClient isolated to test builds. fmt clean, `deny.toml` present, tests pass. Workspace `forbid(unsafe_code)` added in alpha.28.

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
| NG-02 | Session API inconsistency | **RESOLVED** — `semantic_router/session.rs` added (~489 lines); `SemanticRouter::call_method` dispatches `session.save`/`load`/`list`/`delete`. Full parity across unix-socket, isomorphic, and semantic paths |
| NG-03 | `data.*` handlers delegation | Low | Reframed — `data.*` removed from advertised capabilities (honest delegation story); `data_handlers.rs` stubs remain as explicit `not_implemented` for external live feeds; `model_cache_handlers.rs` also stubs |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated, TLS delegated to system `curl` |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

**Compliance** (d7a0716b): Compile errors **FIXED** (constants re-exported, ZFS test gated). Clippy **clean exit** on `--workspace --all-targets` (~2 minor warnings: observe test assert, zfs test docs). fmt clean. `forbid(unsafe_code)` per-crate + workspace `deny`. `deny.toml` present. SPDX present. Discovery overstep modules **deprecated**. `src/cli/mod.rs` removed (444 lines). Session 15: TCP/RPC parity, health triad, `FileMetadataBackend`.

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

**Compliance** (S172-4): **clippy CLEAN** (was 25 warnings — all resolved). **fmt PASSES** (was 18 files failing — all resolved). `deny.toml` present. Tests pass (~19.6 min). VFIO DMA sharing module added (in-domain hw-safe). TS-01 partially advanced (async hygiene in discovery, but not full `capability.discover` migration).

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

**Compliance** (Wave 26): clippy clean, fmt clean, `forbid(unsafe_code)` at workspace level, `deny.toml` present (skip-list 30 → 15), SPDX present, **14,366+ tests, 0 failures**. Gold standard. **AI tree (11.9K LOC) feature-gated** behind `ai` feature per responsibility matrix. Flaky `production_ready` test stabilized. `handle_key_info` + client JSON-RPC dispatch evolved from stubs to real implementations.

---

## Priority Order

**ZERO CRITICAL / HIGH / MEDIUM BLOCKERS.**

All 7 open gaps are **Low** severity — polish items owned by primal teams.

**Low** (polish, owned by primal teams):
1. **NG-01** — NestGate metadata backend injection (`FileMetadataBackend` available; needs default wiring)
2. **NG-03** — `data.*` handler stubs (removed from capabilities; honest delegation story; stubs remain)
3. **PT-04** — `ExportFormat::Html` headless variant (exists; needs product validation)
4. **PT-06** — callback push dispatch (code-complete; server `callback_tx` wiring needed to activate)
5. **SB-02** — `ring` lockfile ghost (not compiled in default build; lockfile refresh clears it)
6. **SB-03** — `sled` feature-gated but default-on in orchestrator/sovereign-onion (pending NestGate API)
7. **TS-01** — coralReef pure `capability.discover` (discovery async-improved in S172-4, not fully migrated)

---

## Guideline Compliance Matrix

| Primal | Clippy | Fmt | `forbid(unsafe)` | `deny.toml` | SPDX | Tests |
|--------|--------|-----|-------------------|-------------|------|-------|
| biomeOS | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS |
| BearDog | CLEAN | PASS | workspace forbid | YES (skip 30→15) | YES | **PASS (14.4K+)** |
| Songbird | 8 warn | PASS | per-crate forbid | YES | YES | PASS |
| NestGate | **~2 warn** ↑↑ | PASS | deny + per-crate forbid | YES | YES | **PASS** ↑↑ |
| petalTongue | pending | PASS | per-crate forbid | YES | YES | PASS |
| Squirrel | pending | PASS | **workspace forbid** ↑ | YES | YES | PASS |
| toadStool | **CLEAN** ↑↑ | **PASS** ↑↑ | **absent** | YES | n/c | PASS |
| sweetGrass | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS |
| rhizoCrypt | CLEAN | PASS | deny + cfg_attr forbid | YES | n/c | PASS |
| loamSpine | CLEAN | PASS | workspace forbid | YES | n/c | PASS |
| barraCuda | n/c | n/c | n/c | n/c | n/c | n/c |
| coralReef | **RED** | n/c | n/c | n/c | n/c | **RED** |

**Legend**: n/c = not checked this cycle; ↑↑ = major improvement

### Compliance Evolution (April 2 PM — post-pull)

1. **BearDog** (Wave 26): AI tree feature-gated, flaky test stabilized, deny.toml skip-list halved (30→15), stubs→implementations
2. **NestGate** (d7a0716b): **compile fixed**, clippy ~2 warnings (was 16+RED), NG-02 resolved (session.rs), discovery overstep deprecated
3. **toadStool** (S172-4): **fmt fixed** (18 files), **clippy clean** (was 25 warnings), VFIO DMA in-domain
4. **Squirrel** (alpha.29): **49K orphan lines removed**, 0 `todo!`/`unimplemented!` (was 14+4), 50+ unused deps removed, workspace `forbid(unsafe_code)` added
5. **petalTongue** (9ce7a97): CHANGELOG.md added, sensory matrix + accessibility adapters (in-domain), PT-06 code-complete
6. **Songbird** + **biomeOS**: stable, no new commits

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

**20 gaps resolved** across the full cycle (+NG-02 this pull). **7 open** (all low). Zero critical, zero medium.

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
