# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: Primal-only gaps relevant to primalSpring's upstream role. Downstream systems
> (gardens, springs) own their own debt and pick up patterns from `wateringHole/`.
>
> **Last updated**: 2026-04-07 — GAP-017 resolved, fmt debt cleared, trio witness harvest.
> License alignment COMPLETE. toadStool clippy CLEAN. bingoCube edition 2024.
> **Compliance matrix**: `wateringHole/ECOSYSTEM_COMPLIANCE_MATRIX.md` v2.3.0 — 15 primals + 4 tools.
> **Grade distribution**: 4 A (barraCuda, bingoCube, coralReef, skunkBat), 8 B, 3 C, 0 D, 0 F.
> **Tool grades**: bingoCube A, benchScale A, agentReagents A, rustChip A.
> **Public-ready tools**: bingoCube (83.4% coverage), benchScale (61.9%), agentReagents (60.2%) — all >=60% gate, scrubbed, Grade A.
> **Deep debt resolved**: DEBT-02 (canonical capability registry — LOCAL/ROUTED split), DEBT-03 (Neural API routing — biomeOS v2.90), DEBT-05 (plasmidBin binary name verification), DEBT-06 (ionic protocol handlers — partial), probe_capability Tier 2 fix. Capability registry published to `wateringHole/capability_registry.toml`.
> **Top ecosystem gaps (primals)**: discovery debt (toadStool D, Squirrel D), domain symlinks (8 primals).
> **Fmt debt CLEARED**: BearDog 0 diffs ↑, Songbird 0 diffs ↑, toadStool 0 diffs ↑ — all 15 primals now pass `cargo fmt --check`.
> **GAP-017 RESOLVED**: benchScale `deploy-ecoprimals.sh` now passes `--graphs-dir`, `--port`, `--family-id` to biomeOS. Health check upgraded to 15s grace + 3 retries.
> **barraCuda**: E0061 compile failure **FIXED** (Sprint 29). executor.rs split (max 845 LOC ↑). SIGSEGV remains concurrent-test driver debt. Musl-static rebuild pending.
> **ToadStool**: Clippy **CLEAN** ↑↑, fmt **CLEAN** ↑↑ (was 1,899), discovery debt (285 files).
> **License**: All 15 primals on `AGPL-3.0-or-later` — zero license debt.
>
> **Composition principle (April 6)**: Complex functions emerge from composing base
> primals via Neural API graphs. RootPulse is not a primal — it is a workflow graph
> (`rootpulse_commit.toml`) executed over the provenance trio. Cross-spring data
> exchange is a biomeOS graph design problem, not a missing primal. See
> `wateringHole/DEPLOYMENT_AND_COMPOSITION.md` §The Composition Principle.
>
> **Trio witness evolution (April 7)**: `WireAttestationRef` → `WireWitnessRef`.
> Attestation → Witness. `signature` → `evidence`. `attestations` → `witnesses`.
> Self-describing `kind`/`encoding`/`algorithm`/`tier`/`context` fields. Trio is now
> algo-agnostic and can track non-crypto events (checkpoints, markers, hashes).
> BearDog-to-witness bridge documented. Trio harvested to plasmidBin (glibc).
> See `wateringHole/handoffs/PRIMALSPRING_TRIO_WITNESS_HARVEST_HANDOFF_APR07_2026.md`.

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
| RC-02 | Witness wire evolution | **RESOLVED** (v0.14.0-dev — `WireWitnessRef`: kind/evidence/encoding/algorithm/tier/context) |

**Compliance**: clippy clean, fmt clean, `deny(unsafe_code)` + `forbid` in non-test builds via `cfg_attr`, `deny.toml` present, tests pass. Witness evolution harvested to plasmidBin (April 7).

---

## loamSpine

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| LS-03 | Panic on startup | **RESOLVED** (v0.9.15 — infant discovery fails gracefully) |
| LS-04 | Witness wire evolution | **RESOLVED** (v0.9.16 — `WireWitnessRef` in `trio_types.rs`, witnesses on wire summaries) |

**Compliance**: clippy clean, **fmt now PASSES** (previously failing — fixed), `forbid(unsafe_code)` at workspace level, `deny.toml` present, tests pass. Witness evolution harvested to plasmidBin (April 7).

---

## toadStool

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| TS-01 | coralReef discovery not pure capability-based | Low | Improved — S166 capability-based discovery added (`discover_by_capability` in `service_discovery.rs`), but `coral_reef_client` still uses explicit 6-step ordered discovery, not unified `capability.discover` RPC |

**Compliance** (S172-5 — 8af2244b0): Clippy **2 warnings** (unfulfilled lint expectations in `executor/types.rs` — minor). fmt **PASS**. 21,537 tests **ALL PASS**. `deny.toml` present. **Discovery compliance: PARTIAL** — S172-5 commit targets capability-based discovery + root doc cleanup, but audit finds ~30 non-test files still reference primal names. `SONGBIRD_*` env vars in fallback discovery (`infant_discovery`, `dns_discovery`, `runtime_defaults`), `BEARDOG_SOCKET` in primal socket modules. wateringHole IPC matrix updated to X→C but our scan shows residual coupling in compatibility/fallback paths. TS-01 partially advanced.

---

## sweetGrass

All gaps **RESOLVED**. TCP JSON-RPC added, `cargo-deny`, `forbid(unsafe)`.

| ID | Gap | Status |
|----|-----|--------|
| SG-01 | Witness wire evolution | **RESOLVED** (v0.7.27 — `Witness` type, `EcoPrimalsAttributes.witnesses`, kind/evidence/encoding) |

**Compliance**: clippy clean, fmt clean, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, tests pass. Witness evolution harvested to plasmidBin (April 7).

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

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| BD-01 | `crypto.verify_ed25519` does not accept `encoding` hint | Low | Open — caller must decode hex→raw→base64 before calling verify when witness has `encoding: "hex"`. Future evolution should accept encoding on verify wire. |

**Compliance** (Wave 26): clippy clean, fmt clean, `forbid(unsafe_code)` at workspace level, `deny.toml` present (skip-list 30 → 15), SPDX present, **14,366+ tests, 0 failures**. Gold standard. **AI tree (11.9K LOC) feature-gated** behind `ai` feature per responsibility matrix. Flaky `production_ready` test stabilized. `handle_key_info` + client JSON-RPC dispatch evolved from stubs to real implementations.

---

## Priority Order

**ZERO CRITICAL / HIGH / MEDIUM BLOCKERS.**

All 5 open gaps are **Low** severity — polish items owned by primal teams.

**Low** (polish, owned by primal teams):
1. **BD-01** — BearDog `crypto.verify_ed25519` encoding hint (caller decodes for now; future BearDog wire evolution)
2. **NG-01** — NestGate metadata backend injection (`FileMetadataBackend` available; needs default wiring)
3. **NG-03** — `data.*` handler stubs (removed from capabilities; honest delegation story; stubs remain)
4. **SB-02** — `ring` lockfile ghost (not compiled in default build; lockfile refresh clears it)
5. **SB-03** — `sled` feature-gated but default-on in orchestrator/sovereign-onion (pending NestGate API)

---

## Guideline Compliance Matrix

| Primal | Clippy | Fmt | `deny.toml` | License | Edition | Tests | Discovery |
|--------|--------|-----|-------------|---------|---------|-------|-----------|
| biomeOS | **CLEAN** | **PASS** | YES | **`-or-later`** ↑ | 2024 | **PASS (7,638)** ↑ | **P** |
| BearDog | **CLEAN** | **PASS** ↑ | YES | **`-or-later`** ↑ | 2024 | **PASS (14,366)** | **P** |
| Songbird | **CLEAN** | **PASS** ↑ | YES | **`-or-later`** ↑ | 2024 | TIMEOUT (large suite) | **P** |
| NestGate | **CLEAN** | **PASS** | YES | **`-or-later`** ↑ | 2024 | **PASS (11,661)** ↑ | **P** |
| petalTongue | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS** (1 flaky) | **P** |
| Squirrel | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (6,868)** | **P** |
| toadStool | **CLEAN** ↑↑ | **PASS** ↑↑ | YES | **`-or-later`** ↑ | 2024 | TIMEOUT (large suite) | **D** |
| sweetGrass | **CLEAN** ↑ | **PASS** | YES | **`-or-later`** ↑ | 2024 | PASS | **P** |
| rhizoCrypt | **CLEAN** ↑ | **PASS** | YES | `-or-later` | 2024 | PASS | **P** |
| loamSpine | **CLEAN** | **PASS** ↑ | YES | `-or-later` | 2024 | **PASS (29)** | **P** |
| barraCuda | **CLEAN** ↑ | **PASS** | YES | `-or-later` | 2024 | **PASS (3,899)** | **P** |
| sourDough | **CLEAN** | **PASS** | **YES** ↑ | `-or-later` | 2024 | **PASS (3)** | **P→C** |
| coralReef | **CLEAN** ↑ | **PASS** | YES | **`-or-later`** ↑ | 2024 | **PASS (7)** | **P→C** |
| bingoCube | **CLEAN** ↑ | **PASS** | **YES** ↑ | **`-or-later`** ↑ | **2024** ↑ | PASS | **P→C** |
| skunkBat | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS (3 ignored) | **P** |

**Legend**: ↑ = improved since last audit; ↑↑ = major improvement

### Compliance Evolution (April 6 — full ecosystem re-audit)

**Ecosystem-wide license alignment COMPLETE.** All 15 primals now on `AGPL-3.0-or-later`. Previously 8 were on `-only`.

1. **toadStool**: **Massive turnaround.** Clippy **CLEAN** (was FAIL — 2 errors). Fmt from ~1,899 diffs to **1 diff**. License updated. `tar` dep updated.
2. **bingoCube**: Edition **2024** (was 2021). Clippy **CLEAN** (was 15 errors). `deny.toml` added. License updated.
3. **coralReef**: Clippy **CLEAN** (was 7 errors in `coral-gpu` tests). License updated.
4. **rhizoCrypt**: Clippy **CLEAN** (was 5 `doc_markdown` warnings). 39 warnings resolved.
5. **sweetGrass**: Clippy **CLEAN** (was 1 warning). License updated.
6. **sourDough**: `deny.toml` **added** (was missing).
7. **loamSpine**: Fmt **PASS** (was failing). v0.9.16 public chain anchor.
8. **biomeOS**: v2.91 — large file refactoring, 27 new tests, license `-or-later`. 7,638 tests.
9. **beardog**: 344 clippy warnings resolved (pedantic+nursery clean). License updated. 1 fmt diff.
10. **songbird**: 48 clippy warnings resolved. License updated. 2 fmt diffs.
11. **barraCuda**: E0061 compile failure **FIXED** (Sprint 29). SIGSEGV test thread cap. 3,899 tests pass.
12. **NestGate**: 11,661 tests. Doc alignment. License updated.
13. **Squirrel**, **petalTongue**, **skunkBat**: Stable, clean.

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
| LS-04 | loamSpine | Witness wire evolution (`WireWitnessRef` in `trio_types.rs`) | v0.9.16 |
| RC-02 | rhizoCrypt | Witness wire evolution (`WireWitnessRef`, evidence, kind) | v0.14.0-dev |
| SG-01 | sweetGrass | Witness wire evolution (`Witness`, `EcoPrimalsAttributes.witnesses`) | v0.7.27 |

| TS-01 | toadStool | coralReef `capability.discover` | S173-2 |
| PT-04 | petalTongue | HTML graph export | deep debt evolution |
| PT-06 | petalTongue | callback_tx push notifications | deep debt evolution |

**26 gaps resolved** across the full cycle. **5 open** (all low). Zero critical, zero medium.
10 build/test debt items resolved (April 6): license alignment complete, toadStool clippy+fmt,
bingoCube clippy+edition, coralReef clippy, rhizoCrypt clippy, sweetGrass clippy, NestGate fmt,
barraCuda compile. 3 trio witness wire gaps resolved (April 7): RC-02, LS-04, SG-01.
BD-01 added (BearDog encoding hint on verify — low, forward-compatible).

---

## Capability-Based Discovery Compliance (April 6, 2026)

Per `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.2: primals MUST discover each other
by capability domain via Neural API, not by hardcoded primal names in routing code.

Scan methodology: `rg -l` for other-primal names in non-test, non-archive, non-target source
files. Self-references excluded. biomeOS excluded from grading (orchestrator — primal-name
awareness is in-domain for routing).

| Primal | Compliance | Other-Primal Files | Env-Var Refs | Trend |
|--------|-----------|-------------------|--------------|-------|
| sourDough | **P→C** | 5 | 2 | Stable |
| bingoCube | **P→C** | 9 | 0 | Stable |
| coralReef | **P→C** | 31 | 17 | Stable |
| NestGate | **P** | 63 | 97 | Improving ↑ |
| loamSpine | **P** | 72 | 155 | Stable |
| sweetGrass | **P** | 93 | 196 | Stable |
| skunkBat | **P** | 66 | 9 | New entry |
| BearDog | **P** | 131 | 147 | New methodology (broader scan) |
| barraCuda | **P** | 136 | 17 | New entry |
| petalTongue | **P** | 168 | 106 | Stable |
| rhizoCrypt | **P** | 127 | 204 | Broader scan vs previous |
| Songbird | **P** | 193 | 329 | Improving ↑ |
| Squirrel | **D** | 205 | 232 | Stable |
| toadStool | **D** | 285 | 203 | Improving ↑ |
| biomeOS | *(orchestrator)* | 458 | 733 | In-domain routing refs |

**Note on methodology change**: This scan uses a broader regex than previous audits
(includes all 15 primal tokens, case-insensitive, across all non-test source files).
Previous audits used a narrower scope, producing lower absolute numbers. Relative
rankings and trends are consistent.

### Discovery Compliance Priority

1. **toadStool** — 285 files (was 384 previous scan). Improving but still highest non-orchestrator.
2. **Squirrel** — 205 files. Many refs are acceptable (logging, aliases, serde compat).
3. **Songbird** — 193 files. Strongest improvement trajectory across audit cycles.
4. **petalTongue** — 168 files. UI backends reference primal names for display/discovery.
5. **NestGate** — 63 files. Near-compliant. Overstep shedding continues.

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
| rhizocrypt | 5.4M | glibc | yes | April 7 — witness evolution harvest |
| loamspine | 8.5M | glibc | yes | April 7 — witness evolution harvest |
| sweetgrass | 11.9M | glibc | yes | April 7 — witness evolution harvest |
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

## Ecosystem Audit Debt (April 6, 2026)

### License Alignment — **COMPLETE**

All 15 primals now on `AGPL-3.0-or-later`. Zero license debt remaining.

### Build/Test Debt

| Primal | Category | Issue | Status |
|--------|----------|-------|--------|
| barraCuda | ~~compile~~ | ~~E0061~~ `eval_math` decomposition | **FIXED** (Sprint 29) |
| barraCuda | ~~file size~~ | ~~`executor.rs` 1,097 lines~~ → split (max 845 LOC) | **FIXED** ↑ |
| barraCuda | **test** | `fault_injection` SIGSEGV (thread cap added Sprint 29) | Open |
| BearDog | ~~fmt~~ | ~~1 file diff~~ | **FIXED** ↑ |
| Songbird | ~~fmt~~ | ~~2 file diffs~~ | **FIXED** ↑ |
| toadStool | ~~fmt~~ | ~~1,899 diffs~~ → ~~1 diff~~ → **0 diffs** | **FIXED** ↑↑ |
| toadStool | ~~clippy~~ | ~~`manual_let_else`, deprecated `GenericArray`~~ | **FIXED** ↑↑ |
| NestGate | ~~fmt~~ | ~~`migration.rs:189`~~ | **FIXED** |
| coralReef | ~~clippy~~ | ~~7 errors in `coral-gpu` tests~~ | **FIXED** ↑ |
| bingoCube | ~~clippy~~ | ~~15 cast errors~~ | **FIXED** ↑ |
| bingoCube | ~~edition~~ | ~~2021~~ → **2024** | **FIXED** ↑ |
| rhizoCrypt | ~~clippy~~ | ~~5 `doc_markdown` warnings~~ | **FIXED** (39 warnings resolved) ↑ |
| sweetGrass | ~~clippy~~ | ~~1 unused import~~ | **FIXED** ↑ |
| sweetGrass | ~~config~~ | ~~`.cargo/config.toml` target-dir points to `/home/southgate/`~~ | **FIXED** (already cleaned) |
| petalTongue | **test** | 1 flaky test (`test_resolve_instance_id_error_message_invalid`) | Open (passes on retry) |

**Resolved this cycle:** 14 build/test debt items (+4 this push: fmt×3, executor split). **Remaining:** 2 (barraCuda SIGSEGV test, petalTongue flaky test).
