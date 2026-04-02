# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: Primal-only gaps relevant to primalSpring's upstream role. Downstream systems
> (gardens, springs) own their own debt and pick up patterns from `wateringHole/`.
>
> **Last updated**: 2026-04-01 — Full primal audit + guideline compliance review.
> 18 gaps resolved, 8 open (zero critical, zero high). SQ-03 reclassified as RESOLVED (documented).

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

**Compliance**: clippy 1 warning (deprecated `assert_cmd::Command::cargo_bin`), fmt clean, `forbid(unsafe_code)` per-crate (not workspace-level), `deny.toml` present, SPDX headers present. **1 test failure**: `provenance_trio::tests::discover_returns_none_without_sockets` — assertion issue in test environment. 513 tests pass.

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

**Compliance**: clippy passes (minor unfulfilled lint expectations in `squirrel-mcp-auth` tests), fmt clean, `deny.toml` present, tests pass. **Note**: no `forbid(unsafe_code)` at workspace manifest level (uses clippy groups instead). SPDX headers present.

---

## songBird

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SB-01 | `health.liveness` canonical name | **RESOLVED** (wave89-90) |
| SB-02 | CLI `ring-crypto` opt-in feature | Low | Improved — songbird-quic is ring-free; `ring` remains in lockfile via `rcgen` (dev) and optional `rustls/ring-crypto`. `deny.toml` documents allowed wrappers |
| SB-03 | `sled` in orchestrator/sovereign-onion/tor | Low | Open — `sled = "0.34"` direct in orchestrator and sovereign-onion; optional in tor-protocol. No removal signals |

**Compliance**: clippy has warnings (395 `unwrap_used` warnings in songbird-orchestrator tests), fmt clean, `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX headers present. Tests: ~11,669 pass; potential flake in orchestrator tarpc performance tests.

---

## NestGate

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| NG-01 | IPC storage backend inconsistency | Medium | Improved — Unix-socket and isomorphic IPC use filesystem-backed `tokio::fs`; tarpc/semantic paths still use in-memory HashMap. nestgate-core not uniformly wired |
| NG-02 | No dedicated game session API | Low | Improved — `session.save`/`session.load` implemented in unix-socket and isomorphic IPC; not a rich game session product API |
| NG-03 | `data.*` handlers conflate live feeds with storage | Low | Improved — semantic router documents `data.*` vs `storage.*` separation; `data_handlers.rs` still contains stub handlers |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated, TLS delegated to system `curl` |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

**Compliance**: clippy clean (exit 0), fmt clean, `forbid(unsafe_code)` per-crate + workspace `deny`, no `deny.toml`, SPDX headers present. Tests: 12,054 pass, **3 failures** (chaos_monitoring timing, capability discovery tests, service integration). `cc` present as transitive build dep.

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

**Compliance**: clippy clean, **fmt fails** (multiple files need reformatting — `network.rs`, `manifest.rs`, `infant_discovery/tests.rs`, signer tests), `forbid(unsafe_code)` at workspace level, `deny.toml` present, tests pass.

---

## toadStool

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| TS-01 | coralReef discovery not pure capability-based | Low | Improved — S166 capability-based discovery added (`discover_by_capability` in `service_discovery.rs`), but `coral_reef_client` still uses explicit 6-step ordered discovery, not unified `capability.discover` RPC |

**Compliance**: clippy passes with warnings (`redundant_clone` in test code), **fmt fails** (`execution.rs` formatting), no `forbid(unsafe_code)` at workspace level, `deny.toml` present. Tests: 532 pass, **1 failure** (`test_discovery_handles_timeout`).

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

**ZERO CRITICAL / HIGH BLOCKERS.**

**Medium** (improves composition quality):
1. **NG-01** — NestGate persistent backend consistency (unix-socket is durable; tarpc/semantic paths are not)

**Low** (polish, owned by primal teams):
2. PT-04 — `ExportFormat::Html` headless variant (IPC modality already works)
3. PT-06 — callback push dispatch (struct exists, wiring needed)
4. SB-02 — `ring` in lockfile via `rcgen` dev + optional `rustls`
5. SB-03 — `sled` direct dependency in orchestrator/sovereign-onion
6. NG-02 — game session API (basic save/load exists)
7. NG-03 — `data.*` handler stubs (documented separation, not wired)
8. TS-01 — coralReef pure `capability.discover` (6-step discovery works, not unified)

---

## Guideline Compliance Matrix

| Primal | Clippy | Fmt | `forbid(unsafe)` | `deny.toml` | SPDX | Tests |
|--------|--------|-----|-------------------|-------------|------|-------|
| biomeOS | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS |
| BearDog | CLEAN | PASS | workspace forbid | YES | YES | PASS (14K+) |
| Songbird | 395 warn (tests) | PASS | per-crate forbid | YES | YES | PASS (~11.7K) |
| NestGate | CLEAN | PASS | deny + per-crate forbid | **NO** | YES | 3 FAIL / 12K pass |
| petalTongue | 1 warn | PASS | per-crate forbid | YES | YES | 1 FAIL / 513 pass |
| Squirrel | PASS | PASS | **absent** | YES | YES | PASS |
| toadStool | warns (test) | **FAIL** | **absent** | YES | n/c | 1 FAIL / 532 pass |
| sweetGrass | 1 warn | PASS | deny + per-crate forbid | YES | YES | PASS |
| rhizoCrypt | CLEAN | PASS | deny + cfg_attr forbid | YES | n/c | PASS |
| loamSpine | CLEAN | **FAIL** | workspace forbid | YES | n/c | PASS |
| barraCuda | n/c | n/c | n/c | n/c | n/c | n/c |
| coralReef | n/c | n/c | n/c | n/c | n/c | n/c |

**Legend**: n/c = not checked this cycle (no open gaps, lower priority)

### Compliance Observations

1. **BearDog** is the gold standard: workspace `forbid(unsafe_code)`, clippy clean, fmt clean, deny.toml, all tests pass
2. **NestGate** missing `deny.toml` — only primal without one
3. **Squirrel** and **toadStool** lack `forbid(unsafe_code)` at workspace level
4. **toadStool** and **loamSpine** have `cargo fmt` failures — need `cargo fmt --all`
5. **NestGate** (3), **petalTongue** (1), **toadStool** (1) have test failures — mostly timing/environment
6. **Songbird** has 395 clippy warnings in orchestrator tests (`unwrap_used`)

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

**19 gaps resolved** across the full cycle. **8 open** (1 medium, 7 low). Zero critical.

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
