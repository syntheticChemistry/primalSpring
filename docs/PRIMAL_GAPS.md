# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: Primal-only gaps relevant to primalSpring's upstream role. Downstream systems
> (gardens, springs) own their own debt and pick up patterns from `wateringHole/`.
>
> **Last updated**: 2026-04-01 — Post-pull re-evaluation. 18 gaps resolved, 8 open (zero critical, zero high).

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

---

## petalTongue

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| PT-01 | Socket at non-standard path | **RESOLVED** — `biomeos/petaltongue.sock` |
| PT-02 | No live push to browser | **RESOLVED** — SSE `/api/events` |
| PT-03 | `motor_tx` not wired in server mode | **RESOLVED** — drain channel wired |
| PT-04 | No HTML export modality | Low | Open — SVG-in-HTML is sufficient |
| PT-05 | `visualization.showing` returns false | **RESOLVED** — `RenderingAwareness` auto-init in `UnixSocketServer` |
| PT-06 | `callback_method` poll-only dispatch | Low | Partially resolved — `PendingCallback` struct + tests exist |
| PT-07 | No external event source in server mode | **RESOLVED** — periodic discovery refresh wired (explicit `PT-07` tag) |

**EguiShapes variant** deferred — `EguiCompiler` still outputs `ModalityOutput::Description`. Tracked by petalTongue team.

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
| SQ-03 | `deprecated-adapters` feature flag docs | Low | Open |

---

## songBird

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SB-01 | `health.liveness` canonical name | **RESOLVED** (wave89-90) |
| SB-02 | CLI `ring-crypto` opt-in feature | Low | Partially — `songbird-quic` is `ring`-free via `BearDogQuicCrypto`; CLI feature flag remains opt-in |
| SB-03 | `sled` in orchestrator/sovereign-onion/tor | Low | Open — persistence overstep |

---

## NestGate

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| NG-01 | IPC adapter uses in-memory HashMap | Medium | Open — `nestgate-core` storage not wired into RPC handlers |
| NG-02 | No dedicated game session API | Low | Open |
| NG-03 | `data.*` handlers conflate live feeds with storage | Low | Open |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated, TLS delegated to system `curl` |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

---

## rhizoCrypt

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| RC-01 | TCP-only transport | **RESOLVED** (v0.14.0-dev s23 — `--unix`, `UdsJsonRpcServer`, `biomeos/` path) |

---

## loamSpine

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| LS-03 | Panic on startup | **RESOLVED** (v0.9.15 — infant discovery fails gracefully) |

---

## toadStool

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| TS-01 | coralReef discovery not pure capability-based | Low | Improved — 6-step discovery (env vars, manifest, biomeos scan), but not fully `capability.discover` |

S171 on disk. `shader.compile.*` removed (coralReef domain). `ember.list`/`ember.status` added. Health triad wired.

---

## sweetGrass

All gaps **RESOLVED**. TCP JSON-RPC added, `cargo-deny`, `forbid(unsafe)`.

---

## coralReef

No gaps identified.

---

## bearDog

No gaps identified.

---

## Priority Order

**ZERO CRITICAL / HIGH BLOCKERS.**

**Medium** (improves composition quality):
1. **NG-01** — NestGate real persistence backend

**Low** (polish, owned by primal teams):
2. PT-04 — HTML export
3. PT-06 — callback push dispatch
4. SQ-03 — feature flag docs
5. SB-02 — CLI ring-crypto opt-in
6. SB-03 — sled persistence overstep
7. NG-02, NG-03 — NestGate session API / namespace
8. TS-01 — coralReef pure capability discovery

---

## Resolved Gaps Summary

| ID | Primal | What Was Fixed | Resolved In |
|----|--------|---------------|-------------|
| BM-01–05 | biomeOS | Graph routing, health, discovery, multi-shape | v2.79–v2.81 |
| BC-01–04 | barraCuda | Fitts/Hick/Perlin fixes, plasmidBin harvest | Sprint 25 |
| PT-01–03, PT-05, PT-07 | petalTongue | Socket, SSE, motor_tx, awareness init, server discovery | IPC compliance evolution |
| SQ-01–02 | Squirrel | Filesystem socket, `LOCAL_AI_ENDPOINT` wiring | alpha.25b–27 |
| SB-01 | songBird | `health.liveness` canonical | wave89-90 |
| NG-04–05 | NestGate | ring/aws-lc-rs eliminated, crypto delegated to BearDog | deep debt evolution |
| RC-01 | rhizoCrypt | UDS transport + biomeos/ path | v0.14.0-dev s23 |
| LS-03 | loamSpine | Startup panic → graceful degradation | v0.9.15 |

**18 gaps resolved** across the full cycle. **8 open** (1 medium, 7 low). Zero critical.

---

## plasmidBin Inventory

| Binary | Size | Source | UDS | Notes |
|--------|------|--------|-----|-------|
| beardog | 7.1M | musl-static | ✅ | Mar 27 |
| biomeos | 12M | musl-static | ✅ | Mar 28 |
| songbird | 16M | musl-static | ✅ | Mar 27 |
| squirrel | 5.8M | musl-static | ✅ | Mar 27 |
| petaltongue | 30M | musl-static | ✅ | Mar 28 |
| nestgate | 4.9M | musl-static | ✅ | Mar 28 |
| toadstool | 16M | musl-static | ✅ | Mar 27 (S168 binary — S171 needs rebuild) |
| rhizocrypt | 5.4M | glibc | ✅ | April 1 — RC-01 fix |
| loamspine | 6.9M | glibc | ✅ | April 1 — LS-03 fix |
| sweetgrass | 8.8M | glibc | ✅ | April 1 |
| barracuda | 4.5M | glibc | N/A | April 1 — requires GPU |

**Note**: rhizoCrypt/loamSpine/sweetGrass/barraCuda are glibc dynamic — musl-static cross-compile needed for containers.

---

## primalSpring Rewiring Status (April 1, 2026)

| Area | Status |
|------|--------|
| `methods.rs` | ✅ Aligned — `graph.execute`, `topology.rescan`, `ember.*`, `shader.compile` removed, `ai.*`, `visualization.*`, `interaction.*` added |
| `NeuralBridge` | ✅ Aligned — `topology_rescan()` added, `graph.execute` call correct |
| `discover.rs` | ✅ Aligned — plain socket name discovery (`{name}.sock`, `{name}-ipc.sock`) added |
| `capability.rs` | ✅ Aligned — 4-format parsing, `strip_unix_uri`, multi-shape |
| `validate_compositions.py` | ✅ Aligned — SQ-02 messaging updated, NestGate `family_id`, C7 Squirrel check live |
| Composition graphs (C1–C7) | ✅ Clean — no stale references |
| Cargo.toml | ✅ `edition = "2024"`, `rust-version = "1.87"` |
| Tests | ✅ 10/10 pass, 4/4 doc-tests pass |

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
