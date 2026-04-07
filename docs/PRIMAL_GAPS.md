# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: Primal-only gaps relevant to primalSpring's upstream role. Downstream systems
> (gardens, springs) own their own debt and pick up patterns from `wateringHole/`.
>
> **Last updated**: 2026-04-06 — DEBT-02/05/06 resolved, capability registry published, barraCuda unblocked.
> **Compliance matrix**: `wateringHole/ECOSYSTEM_COMPLIANCE_MATRIX.md` v2.2.0 — 14 primals + 4 tools.
> **Grade distribution**: 3 A (rhizoCrypt, barraCuda, bingoCube), 7 B, 3 C, 1 D, 0 F.
> **Tool grades**: bingoCube A, benchScale A, agentReagents A, rustChip A.
> **Public-ready tools**: bingoCube (83.4% coverage), benchScale (61.9%), agentReagents (60.2%) — all >=60% gate, scrubbed, Grade A.
> **Deep debt resolved**: DEBT-02 (canonical capability registry — LOCAL/ROUTED split), DEBT-03 (Neural API routing — biomeOS v2.90), DEBT-05 (plasmidBin binary name verification), DEBT-06 (ionic protocol handlers — partial), probe_capability Tier 2 fix. Capability registry published to `wateringHole/capability_registry.toml`.
> **Top ecosystem gaps (primals)**: discovery debt (5 primals), license alignment (8 primals), `#[allow(` migration (4 primals), domain symlinks (8 primals).
> **barraCuda**: E0061 compile failure **FIXED** (Sprint 29). SIGSEGV remains concurrent-test driver debt. Musl-static rebuild pending.
> **ToadStool**: Heavy fmt debt (~1,899 lines of diff), clippy failures, discovery debt (2,998 refs).

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

**Compliance** (v2.87): clippy **CLEAN**, fmt **PASS**, all tests **PASS**, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, SPDX headers present. **Discovery compliance: COMPLETE** — identity-based routing (`discover_beardog_socket`, `beardog.health`) removed from all non-test code (v2.82 + v2.87). Only test fixtures retain primal names.

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

**Compliance** (wave 99 — b650c7c): clippy **CLEAN**, fmt **PASS**, `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX present. Zero `todo!`/`unimplemented!`/`FIXME` in non-test code. Tests **ALL PASS** (168 unit + 8 e2e, 0 failures — was 1 failure, now fixed). **Discovery compliance: IMPROVED** — `SongbirdClient` and `barracuda.compute.dispatch` removed (wave 97). 24 env-var refs across 10 files (`SONGBIRD_`, `TOADSTOOL_`, `BARRACUDA_`, `BEARDOG_` in `petal-tongue-ipc`, `petal-tongue-core/constants`, `petal-tongue-ui` backends). Primal names in ~20 non-test source files (discovery, IPC, UI backends, sandbox mock).

---

## barraCuda

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| BC-01 | Fitts formula variant | **RESOLVED** (Sprint 25 — `variant` param, Shannon default) |
| BC-02 | Hick formula off-by-one | **RESOLVED** (Sprint 25 — `include_no_choice` param) |
| BC-03 | Perlin3D lattice | **RESOLVED** (Sprint 25 — proper gradients + quintic fade) |
| BC-04 | No plasmidBin binary | **RESOLVED** (April 1 harvest, 4.5M, requires GPU) |

**Compliance** (Sprint 29 — 318a6e6c): **E0061 compile failure FIXED** — `eval_math` decomposition added `c: Option<&Value>` for 3-operand naga math (`Clamp`, `Mix`, `SmoothStep`, `Fma`). clippy **CLEAN** (1 `unfulfilled_lint_expectations` warning on test-only `large_stack_arrays`), fmt **PASS**, `deny.toml` present, zero `todo!`/`unimplemented!`/`FIXME`. **`fault_injection` integration test SIGSEGV** — signal 11 crash after 2 of 12 tests pass (concurrent-test driver debt). Musl-static rebuild needed to refresh plasmidBin binary + checksums.

---

## Squirrel

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SQ-01 | Abstract-only socket | **RESOLVED** (alpha.25b — `UniversalListener`) |
| SQ-02 | `LOCAL_AI_ENDPOINT` not wired into `AiRouter` | **RESOLVED** (alpha.27 — step 1.5 discovery, `resolve_local_ai_endpoint()`) |
| SQ-03 | `deprecated-adapters` feature flag docs | **RESOLVED** — documented in `CURRENT_STATUS.md` feature-gates table; intentional retention until v0.3.0 with migration path to `UniversalAiAdapter` + `LOCAL_AI_ENDPOINT` |

**Compliance** (alpha.31 — 56d5bed0): Zero `todo!`/`unimplemented!`/`FIXME` in non-test code. fmt **PASS**. clippy/tests **STILL FAIL** (`squirrel-ai-tools` integration test — `MockAIClient` cfg gate issue blocks entire workspace test). `deny.toml` present. Workspace `forbid(unsafe_code)`. **Discovery compliance: PARTIAL** — audit finds **322 primal-name refs across 96 non-test files** and **10 env-var refs across 4 files** (`SONGBIRD_HOST`, `SONGBIRD_PORT`, `SONGBIRD_URL`). Alpha.30 commit targets capability-based discovery but most refs remain.

---

## songBird

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SB-01 | `health.liveness` canonical name | **RESOLVED** (wave89-90) |
| SB-02 | CLI `ring-crypto` opt-in feature | Low | Near-resolved — `rcgen` removed from lockfile (wave93); `ring` still in `Cargo.lock` but **not compiled** in default build; `ring-crypto` is opt-in CLI feature with single `cfg`-gated call. Default uses `rustls_rustcrypto`. Lockfile refresh would remove stale `ring` stanza |
| SB-03 | `sled` in orchestrator/sovereign-onion/tor | Low | Improved — wave93 feature-gated sled (`optional = true` + `dep:sled`) in all 3 crates. `sled-storage` default-on in orchestrator + sovereign-onion; opt-in `persistent-cache` for tor. Pending NestGate storage API |

**Compliance** (wave 99 — 1493ceaa9): clippy **CLEAN**, fmt **FAILS** (widespread — needs `cargo fmt`), all tests **PASS** (0 failures). `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX present. **Discovery compliance: POOR** — audit reveals **2558 primal-name refs across 321 non-test files** and **143 env-var refs across 50 files** (`BEARDOG_*`, `SONGBIRD_SECURITY`, `BEARDOG_SOCKET`, `BEARDOG_URL`). Wave 97 renamed `discover_beardog→discover_security_provider` but the migration is shallow — the vast majority of production code still hardcodes BearDog by name. This is the highest discovery debt in the ecosystem.

---

## NestGate

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| NG-01 | IPC storage backend inconsistency | Low | Improved — `StorageBackend` trait injection wired for tarpc object storage (`with_backend`/`with_backend_arc`). Semantic router delegates to `NestGateRpcClient`. **Residual**: `InMemoryMetadataBackend` is default for metadata axis; callers must inject filesystem-backed `MetadataBackend` |
| NG-02 | Session API inconsistency | **RESOLVED** — `semantic_router/session.rs` added (~489 lines); `SemanticRouter::call_method` dispatches `session.save`/`load`/`list`/`delete`. Full parity across unix-socket, isomorphic, and semantic paths |
| NG-03 | `data.*` handlers delegation | Low | Reframed — `data.*` removed from advertised capabilities (honest delegation story); `data_handlers.rs` stubs remain as explicit `not_implemented` for external live feeds; `model_cache_handlers.rs` also stubs |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated, TLS delegated to system `curl` |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

**Compliance** (a75e9f2a): Major modularization — 373 files, smart refactoring (max file ~500 LOC), placeholder evolution, test coverage push. Clippy **CLEAN** (0 warnings, improved from ~2). fmt **PASS**. 1449+ tests **ALL PASS**. `forbid(unsafe_code)` per-crate + workspace `deny`. `deny.toml` present. SPDX present. **Discovery compliance: STRONG** — only 7 non-test files have primal-name refs (all in config/discovery layers: `nestgate-config/services.rs`, `nestgate-discovery/` capability modules, `nestgate-rpc/atomic.rs`). Zero `NESTGATE_BEARDOG`, `SONGBIRD_HOST`, `SONGBIRD_PORT` env literals — all primal-specific env vars eliminated from production code. Handoff claims full capability-based discovery compliance.

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

**Compliance** (S172-5 — 8af2244b0): Clippy **2 warnings** (unfulfilled lint expectations in `executor/types.rs` — minor). fmt **PASS**. 21,537 tests **ALL PASS**. `deny.toml` present. **Discovery compliance: PARTIAL** — S172-5 commit targets capability-based discovery + root doc cleanup, but audit finds ~30 non-test files still reference primal names. `SONGBIRD_*` env vars in fallback discovery (`infant_discovery`, `dns_discovery`, `runtime_defaults`), `BEARDOG_SOCKET` in primal socket modules. wateringHole IPC matrix updated to X→C but our scan shows residual coupling in compatibility/fallback paths. TS-01 partially advanced.

---

## sweetGrass

All gaps **RESOLVED**. TCP JSON-RPC added, `cargo-deny`, `forbid(unsafe)`.

**Compliance**: clippy 1 warning (unused imports in `tcp_jsonrpc.rs` test), fmt clean, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, tests pass.

---

## sourDough

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SD-01 | Missing `deny.toml` | Low | Open — cargo-deny not configured |
| SD-02 | musl cross-compilation | Low | Open — binary builds not yet wired for ecoBin |
| SD-03 | genomeBin signing | Low | Open — sequoia-openpgp not implemented |

**Compliance** (v0.1.0 — f1cc802): clippy **CLEAN** (`all` + `pedantic` + `nursery`), fmt **PASS**, `forbid(unsafe_code)` at workspace level, `deny.toml` **MISSING**, SPDX AGPL-3.0-or-later in Cargo.toml. **239 tests, 0 failures** (unit + integration + e2e + doctests), coverage 96%+. Edition 2024, workspace lints centralized. Zero `TODO`/`FIXME`/`HACK`/`unimplemented!` in source. **Discovery compliance: NEAR-CLEAN** — 1 BearDog string in CLI genomebin.rs (cosmetic). Scaffold independence confirmed: generated primals have no runtime dependency on sourDough.

---

## coralReef

No gaps identified.

**Compliance** (Iter 72 — 5a6ca52): clippy **CLEAN**, fmt **PASS**, `forbid(unsafe_code)` on coralreef-core + nak-ir-proc + stubs, `deny.toml` present. **4,257 tests, 0 failures**. SPDX present. **Discovery compliance: CLEAN** — 28 `biomeos` refs (ecosystem-standard env vars: `BIOMEOS_FAMILY_ID`, `BIOMEOS_ECOSYSTEM_NAMESPACE`, `BIOMEOS_ECOSYSTEM_REGISTRY`), 1 "Songbird" doc comment, 1 "BarraCUDA" attribution comment. Zero routing violations. Socket at `$XDG_RUNTIME_DIR/biomeos/coralreef-core-{family}.sock` with `shader.sock` + `device.sock` domain symlinks. Dispatch boundary with toadStool documented (`DISPATCH_BOUNDARY.md`).

---

## bearDog

No gaps identified.

**Compliance** (Wave 26): clippy clean, fmt clean, `forbid(unsafe_code)` at workspace level, `deny.toml` present (skip-list 30 → 15), SPDX present, **14,366+ tests, 0 failures**. Gold standard. **AI tree (11.9K LOC) feature-gated** behind `ai` feature per responsibility matrix. Flaky `production_ready` test stabilized. `handle_key_info` + client JSON-RPC dispatch evolved from stubs to real implementations.

---

## Priority Order

**ZERO CRITICAL / HIGH / MEDIUM BLOCKERS.**

All 4 open gaps are **Low** severity — polish items owned by primal teams.

**Low** (polish, owned by primal teams):
1. **NG-01** — NestGate metadata backend injection (`FileMetadataBackend` available; needs default wiring)
2. **NG-03** — `data.*` handler stubs (removed from capabilities; honest delegation story; stubs remain)
3. **SB-02** — `ring` lockfile ghost (not compiled in default build; lockfile refresh clears it)
4. **SB-03** — `sled` feature-gated but default-on in orchestrator/sovereign-onion (pending NestGate API)

---

## Guideline Compliance Matrix

| Primal | Clippy | Fmt | `forbid(unsafe)` | `deny.toml` | SPDX | Tests | Discovery |
|--------|--------|-----|-------------------|-------------|------|-------|-----------|
| biomeOS | CLEAN | PASS | deny + per-crate forbid | YES | YES | **PASS** | **C** |
| BearDog | CLEAN | PASS | workspace forbid | YES (skip 30→15) | YES | **PASS (14.4K+)** | **C** |
| Songbird | CLEAN | PASS | per-crate forbid | YES | YES | **PASS (8.9K+)** | **P→C** ↑ |
| NestGate | CLEAN | PASS | deny + per-crate forbid | YES | YES | **PASS (11.3K)** ↑ | **P→C** |
| petalTongue | CLEAN | PASS | per-crate forbid | YES | YES | **PASS (6K)** ↑ | **P→C** ↑ |
| Squirrel | **CLEAN** ↑ | PASS | workspace forbid | YES | YES | **PASS (6.9K)** ↑ | **P** |
| toadStool | **FAIL** ↓ | PASS | deny (workspace) ↑ | YES | n/c | **PASS (6.5K)** | **P** ↑ |
| sweetGrass | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS | **C** |
| rhizoCrypt | CLEAN | PASS | deny + cfg_attr forbid | YES | n/c | PASS | **C** |
| loamSpine | CLEAN | PASS | workspace forbid | YES | n/c | PASS | **C** |
| barraCuda | **1 lint** | PASS | n/c | YES | n/c | **PASS (3.8K)** ↑ | n/a |
| sourDough | CLEAN | PASS | workspace forbid | **NO** | YES | **PASS (239)** | **C** |
| coralReef | CLEAN | PASS | forbid (core/stubs) | YES | YES | **PASS (4.3K)** | **C** |

**Legend**: n/c = not checked this cycle; ↑ = improved since last audit

### Compliance Evolution (April 3 — full ecosystem pull + re-audit)

1. **Squirrel** (alpha.36): **Major turnaround.** Clippy **CLEAN** (was FAIL), fmt PASS, **6,856 tests PASS, 0 failures** (was build-broken). alpha.33 removed 65,910 lines dead code. alpha.36: stub evolution, self-ref cleanup, zero-copy. Build fully unblocked. Discovery: 215 files / 1789 refs, env vars 38 files / 225 refs (broader scan than previous — full crates/).
2. **NestGate** (3dc0044b): **Overstep shedding accelerating.** Deleted `discovery_mechanism` module (-2K lines, zero in-tree consumers). Deprecated `nestgate-network` crate (`#![deprecated]`, zero workspace dependents). Removed dead features from automation/zfs. Documented security delegation model. Clippy CLEAN, fmt PASS, 6,607 tests (2 flaky discovery tests — pass on retry). Discovery: 22 files / 192 refs, env vars 9 files / 32 refs.
3. **toadStool** (S174-S175): S173-2 was a direct primalSpring audit response — **TS-01 RESOLVED**, `deny(unsafe_code)` documented (43/43 crates). S174-S175: unsafe reduction -80% in consumer blocks, doc cleanup. Clippy **FAIL** (2 errors: `v4l2` private `_pad` fields in `toadstool-display`, plus deprecated `aes_gcm::from_slice` in `toadstool-distributed`). fmt PASS. Discovery: 393 files / 3239 refs (full scan).
4. **Songbird** (wave 102): fmt **PASS** (was FAIL). Discovery 2558→1472 refs (42% reduction). 7,020+ tests. Status X→P.
5. **barraCuda**: No new commits. `fault_injection` SIGSEGV unresolved.
6. **biomeOS**, **BearDog**, **rhizoCrypt**, **loamSpine**, **sweetGrass**, **petalTongue**, **sourDough**: No new commits. Status stable.

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

| TS-01 | toadStool | coralReef `capability.discover` | S173-2 |
| PT-04 | petalTongue | HTML graph export | deep debt evolution |
| PT-06 | petalTongue | callback_tx push notifications | deep debt evolution |

**23 gaps resolved** across the full cycle. **4 open** (all low). Zero critical, zero medium.

---

## Capability-Based Discovery Compliance (April 3, 2026)

Per `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.2: primals MUST discover each other
by capability domain via Neural API, not by hardcoded primal names in routing code.

| Primal | Compliance | Primal-Name Refs (files) | Env-Var Refs (files) | Trend |
|--------|-----------|--------------------------|----------------------|-------|
| BearDog | **C** | 0 (self-refs only) | 0 | Stable |
| biomeOS | **C** | 0 non-test | 0 non-test | Stable |
| rhizoCrypt | **C** | 0 | 0 | Stable |
| loamSpine | **C** | 0 | 0 | Stable |
| sweetGrass | **C** | 0 | 0 | Stable |
| NestGate | **P→C** | 22 files / 192 refs (full scan) | 9 files / 32 refs | **Improving** ↑ |
| petalTongue | **P→C** ↑ | 125 files / 982 refs (full scan) | 15 files / 77 refs | **Improving** ↑ |
| toadStool | **P** ↑ | 384 files / 2998 refs ↑ | 52 files / 168 refs | **Improving** ↑ |
| Squirrel | **P** | 215 files / 1789 refs (full scan) | 38 files / 225 refs | **Build fixed** ↑ |
| Songbird | **P→C** ↑ | 178 files / 935 refs ↑ | 68 files / 285 refs | **Improving** ↑↑↑ |
| coralReef | **C** | 2 (doc/attribution comments) | 0 (only `BIOMEOS_*` standard) | Stable |
| sourDough | **C** | 1 (cosmetic) | 0 | Stable |

### Discovery Compliance Priority

1. **toadStool** — 2998 refs / 384 files (was 3239/393 — 7% cut in S176-S178). `env_config` primal names → capability names. Trajectory improving.
2. **Squirrel** — 1789 refs / 215 files. Build FIXED (alpha.36). Bulk is acceptable (logging, aliases, serde compat).
3. **petalTongue** — 982 refs / 125 files. Major renames landed. Improving.
4. **Songbird** — **935 refs / 178 files** (was 2558→1472→1016→935 — **63% total reduction**). Strongest trajectory. Near-compliant.
5. **NestGate** — 195 refs / 24 files. Near-compliant. Overstep shedding is the real work.

Full per-primal details: `wateringHole/ECOSYSTEM_COMPLIANCE_MATRIX.md` §Tier 4: Discovery / Self-Knowledge.

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

---

## Ecosystem Audit Debt (April 4, 2026)

### License Alignment — `AGPL-3.0-or-later` is the standard

Pinning to `-only` means not trusting the nonprofit stewards who fight the legal battles. The scyBorg triple-copyleft is aggressive by design; `-or-later` is the correct SPDX identifier.

| Primal | Current | Needed | Debt |
|--------|---------|--------|------|
| BearDog | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| Songbird | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| NestGate | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| ToadStool | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| coralReef | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| biomeOS | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| sweetGrass | `AGPL-3.0-only` | `AGPL-3.0-or-later` | Update `Cargo.toml` |
| bingoCube | `AGPL-3.0` | `AGPL-3.0-or-later` | Update `Cargo.toml` |

Compliant (no change needed): barraCuda, Squirrel, petalTongue, rhizoCrypt, LoamSpine, sourDough.

### Build/Test Debt

| Primal | Category | Issue |
|--------|----------|-------|
| barraCuda | ~~compile~~ | ~~E0061~~ **FIXED** (Sprint 29 — `eval_math` decomposition). Musl-static rebuild pending. |
| barraCuda | **file size** | `executor.rs` 1,097 lines (limit 1,000) |
| ToadStool | **fmt** | ~1,899 lines of diff — `cargo fmt --all` never run cleanly |
| ToadStool | **clippy** | `manual_let_else` in GPU test code, deprecated `GenericArray::from_slice` |
| NestGate | **fmt** | 1 file: `migration.rs:189` line wrap |
| coralReef | **clippy** | 7 errors: `items_after_statements`, `doc_markdown` in `coral-gpu` tests |
| bingoCube | **clippy** | 15 cast errors in `core/src/lib.rs` |
| bingoCube | **edition** | 2021 (needs 2024) |
| bingoCube | **docs** | No `CHANGELOG.md` |
| rhizoCrypt | **clippy** | 5 `doc_markdown` in test file `store_redb_tests_query.rs` |
| sweetGrass | **clippy** | 1 unused import in `tcp_jsonrpc.rs:123` |
| sweetGrass | **config** | `.cargo/config.toml` target-dir points to `/home/southgate/` |
| BearDog | **test** | `dispatch_doctor_comprehensive_runs` fails when no live primals |
| biomeOS | **edition** | `tools/` sub-crate still on 2021 |
