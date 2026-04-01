# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Last updated**: 2026-04-01 — Phase 23h (rhizoCrypt RC-01 RESOLVED via source build; 12 gaps resolved, zero CRITICAL blockers)

---

## biomeOS

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| BM-01 | `graph.deploy` not in routing table | — | — | primalSpring calls `graph.execute` | **RESOLVED** (v2.79) |
| BM-02 | `health.liveness` not on Neural API | Low | All | `NeuralBridge::health_check` fallback works | **RESOLVED** (v2.81) |
| BM-03 | `capability.discover` returns `unix://` prefix | — | — | primalSpring `strip_unix_uri` handles both | **RESOLVED** (v2.79) |
| BM-04 | Neural API capability registration: primals starting after biomeOS invisible to `capability.list` | ~~**High**~~ | ludoSpring V37.1 | v2.81: `topology.rescan` + lazy discovery on miss + multi-shape probe | **RESOLVED** (v2.81) |
| BM-05 | `probe_primal_capabilities` silently skips unknown response shapes | ~~Medium~~ | ludoSpring V37.1 | v2.81: `extract_capabilities_from_response` accepts multiple shapes, warn! on unknown | **RESOLVED** (v2.81) |

biomeOS v2.81 also added: TCP-only CLI (`--tcp-only`), cross-gate `capability.call` routing via `GateRegistry`, fully concurrent tests (7,212 tests, 0 warnings).

---

## petalTongue

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| PT-01 | Socket at non-standard path, not discoverable by biomeOS | ~~Low~~ | C1 | Moved to `$XDG_RUNTIME_DIR/biomeos/petaltongue.sock` | **RESOLVED** |
| PT-02 | No live push to browser (was WebSocket/SSE) | ~~**High**~~ | C1: Render | SSE `/api/events` endpoint added in `web_mode.rs` | **RESOLVED** (SSE, not WS) |
| PT-03 | `motor_tx` not wired in server/web mode | ~~Medium~~ | C1, C6 | Drain channel wired so `motor.*` does not error | **RESOLVED** |
| PT-04 | No HTML export modality | Low | C1: Render | Accept SVG-in-HTML as sufficient | Open — low priority |
| PT-05 | `visualization.showing` returns false when not wired | Medium | C6: Proprioception | Initialize default `RenderingAwareness` | Open |
| PT-06 | `callback_method` stored but never invoked — poll-only | Medium | C6: Proprioception | Implement callback dispatch | Open |
| PT-07 | No external event source in server mode | Low | C6: Proprioception | Wire capability discovery into `DataService` | Open |

Also added: `--port` TCP JSON-RPC flag, `health.liveness`/`readiness`/`check` triad, `identity.get`, `lifecycle.status`, `capabilities.list`. New `graph_editor/ui_components/` for reasoning + status display.

### petalTongue: Full Rust egui Gap

EguiShapes variant is **deferred** — `EguiCompiler` still outputs `ModalityOutput::Description` (JSON string). The egui compiler module doc references `EguiShapes` but the enum variant does not exist yet. This remains the path to full native desktop UI driven by primals.

---

## barraCuda

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| BC-01 | Fitts formula: Welford `log2(D/W)` instead of Shannon `log2(2D/W+1)` | ~~**High**~~ | ludoSpring V37.1 (exp089) | `variant` parameter added, default `"shannon"` | **RESOLVED** (Sprint 25 / v0.3.11) |
| BC-02 | Hick formula: `log2(n+1)` vs standard `log2(n)` | ~~Medium~~ | ludoSpring V37.1 | `include_no_choice` parameter added, default `false` | **RESOLVED** (Sprint 25 / v0.3.11) |
| BC-03 | Perlin3D lattice: `perlin3d(0,0,0)` returns -0.11 | ~~Medium~~ | ludoSpring V37.1 (exp091) | Proper gradient vectors + trilinear interpolation + quintic fade | **RESOLVED** (Sprint 25 / v0.3.11) |
| BC-04 | No binary in plasmidBin | Medium | ludoSpring V37.1 | Binary ready (`barracuda.sock`, dual UDS+TCP), needs publishing to plasmidBin | **Mostly resolved** — binary exists, needs plasmidBin harvest |

Also: zero panics, modern idiomatic Rust, capability-based naming, WGSL-as-truth test architecture, NagaExecutor, 15-tier precision continuum.

---

## Squirrel

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| SQ-01 | Abstract-only socket (`@squirrel`), invisible to filesystem scan | ~~**High**~~ | C2: Narration | `UniversalListener` now prefers abstract → filesystem → TCP | **RESOLVED** (alpha.25b) |
| SQ-02 | `LOCAL_AI_ENDPOINT` not wired into `AiRouter` discovery | Medium | C2: Narration | Config exists but `AiRouter::new_with_discovery` doesn't read it | **Open** |
| SQ-03 | `deprecated-adapters` feature flag gate poorly documented | Low | C2: Narration | Document feature flags | Open |

Also added: `health.liveness`/`health.readiness` canonical names, blake3 crypto, ecosystem absorption.

### Squirrel: Local AI Integration Path

SQ-02 remains the last blocker for Ollama routing. `LOCAL_AI_ENDPOINT` env var is read by `AIProviderConfig::from_env()` but the `AiRouter` only discovers providers from `AI_HTTP_PROVIDERS`, `AI_PROVIDER_SOCKETS`, and biomeOS/toadStool socket probe. Once SQ-02 is wired, `ai.query` routes to Ollama at `localhost:11434` without API keys.

---

## songBird

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| SB-01 | `health.liveness` not exposed by canonical name | ~~Low~~ | IPC compliance | Canonical normalization added in `json_rpc_method.rs`, handlers wired | **RESOLVED** (wave89-90) |
| SB-02 | Local crypto deps (`sha2`, `hmac`, `ed25519-dalek`) — should delegate to bearDog | Low | Overstep | QUIC crate now uses `BearDogQuicCrypto` provider; `ring` eliminated from `songbird-quic`. CLI still has optional `ring-crypto` feature flag | **Partially resolved** |
| SB-03 | Embedded `sled` persistence | Low | Overstep | Still present in `songbird-orchestrator`, `songbird-sovereign-onion` | Open |

---

## NestGate

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| NG-01 | IPC adapter uses in-memory `HashMap`, not real storage backend | Medium | C5: Persistence | Wire `nestgate-core` storage into RPC handlers | Open |
| NG-02 | No dedicated game session API | Low | C5: Persistence | Add `session.save`/`session.load` convenience methods | Open |
| NG-03 | `data.*` handlers conflate live feeds with storage | Low | C5: Persistence | Namespace clearly | Open |
| NG-04 | `aws-lc-rs` C dependency still present via `nestgate-installer` → `reqwest` → `rustls` | Medium | ecoBin | `ring` removed, but replaced with `aws-lc-rs` (still C/ASM). Needs pure Rust TLS or songBird delegation | Open |
| NG-05 | `CryptoDelegate` pattern started but crypto crates not fully shed | Medium | Overstep | `nestgate-security` still has full crypto stack; delegation is WIP | Open |

Also: ~2,300 lines deprecated trait excision, flaky test fixes, service name centralization.

---

## esotericWebb

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| EW-01 | `push_scene_to_ui` sends flat JSON, petalTongue expects `SceneGraph` | **High** | C3: Session | Construct `DataBinding::GameScene` or `SceneGraph` | Open |
| EW-02 | `poll_input` exists but not wired into game loop | Medium | C3: Session | Wire into session tick/act cycle | Open |
| EW-03 | `replay` command not implemented | Low | C3: Session | Wire provenance DAG to replay engine | Open |
| EW-04 | V6 internal game science duplicates ludoSpring | Low | C4: Game Science | Composition contract for capability precedence | Open |

---

## rhizoCrypt

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| RC-01 | ~~**TCP-only transport**~~ | ~~**Critical**~~ | ludoSpring V37.1 | Session 23: `--unix [PATH]` CLI flag with default `$XDG_RUNTIME_DIR/biomeos/rhizocrypt.sock`. `UdsJsonRpcServer` in `jsonrpc/uds.rs`. Path migrated from `ecoPrimals/` to `biomeos/`. | **RESOLVED** (v0.14.0-dev s23) |

**Live validation (April 1)**: rhizoCrypt v0.14.0-dev (session 24, source-built) starts with `--unix`, binds UDS at `/run/user/1000/biomeos/rhizocrypt.sock` **AND** TCP on ports 9400 (tarpc) + 9401 (JSON-RPC dual-mode). Full health triad via UDS: `health.liveness` ✅ (`{"status":"alive"}`), `health.readiness` ✅ (`primal: rhizocrypt, version: 0.14.0-dev`), `health.check` ✅ (session counts, vertices, uptime). `dag.session.create` + `dag.session.list` via UDS ✅. 1,423 tests, lock-free CircuitBreaker, zero-sleep testing. **RC-01 RESOLVED — zero CRITICAL blockers remain.**

Note: The plasmidBin binary is still the old TCP-only version. Needs harvest of the source-built binary.

---

## loamSpine

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| LS-03 | ~~**Panic on startup**~~ | ~~**Critical**~~ | ludoSpring V37.1 | Infant discovery now fails gracefully ("No discovery service found") instead of panicking. Continues without discovery. | **RESOLVED** (v0.9.15) |

**Live validation (March 31)**: loamSpine v0.9.15 starts cleanly, binds UDS at `/run/user/1000/biomeos/loamspine.sock`, responds to `health.liveness`, reports 19 capabilities across 4 domains. `entry.append` requires `spine_id` parameter. Socket path is conformant (`biomeos/loamspine.sock`). Infant discovery failure is now a `WARN` not a panic.

---

## toadStool

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| TS-01 | coralReef discovery hardcoded | Medium | ludoSpring V37.1 | Socket scan or `capability.discover` | Open |

S169 cleanup completed (30+ methods removed, -10,659 lines). On disk: S168. S169 was handed off.

---

## plasmidBin

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| PB-01 | NestGate JWT secret too short (25 bytes, needs 32+) | Low | ludoSpring V37.1 | `openssl rand -base64 48` | Open |

---

## ludoSpring

| ID | Gap | Severity | Exposed By | Fix Path | Status |
|----|-----|----------|------------|----------|--------|
| LS-01 | Gateway hardcodes flow params | Medium | C4: Game Science | Derive from session state | Open |
| LS-02 | No composition contract for esotericWebb vs ludoSpring dedup | Low | C4: Game Science | Capability precedence in graph metadata | Open |

---

## Cross-Cutting Gaps

| ID | Gap | Severity | Fix Path | Status |
|----|-----|----------|----------|--------|
| XC-01 | No standard `DataBinding` construction library | Medium | `ecoPrimal::databinding` module | Open |
| XC-02 | Gateway knows primal socket paths directly | ~~Medium~~ | Use `capability.discover` exclusively — now viable with BM-04 resolved | **Unblocked** |
| XC-03 | No composition health aggregator | Medium | `composition.health` in C7 graph | Open |

---

## Priority Order (revised post-full-validation)

**ZERO CRITICAL BLOCKERS** — RC-01 and LS-03 both RESOLVED.

**High** (blocks interactive product or major capability):
1. **EW-01** — esotericWebb scene format (enables primal-driven rendering)
2. **SQ-02** — Squirrel `LOCAL_AI_ENDPOINT` → `AiRouter` wiring (last blocker for local AI)

**Medium** (improves composition quality):
5. NG-01 — NestGate real persistence
6. NG-04 — NestGate `aws-lc-rs` C dependency
7. PT-05 — petalTongue awareness initialization
8. EW-02 — esotericWebb poll_input wiring
9. LS-01 — ludoSpring dynamic flow params
10. TS-01 — toadStool↔coralReef discovery

**Low** (polish):
11. PT-04, PT-06, PT-07
12. SQ-03, SB-03
13. NG-02, NG-03, NG-05
14. EW-03, EW-04, LS-02
15. PB-01, BC-04 (harvest)
16. XC-01, XC-03

---

## Resolved Gaps Summary (this cycle)

| ID | Primal | What Was Fixed | Resolved In |
|----|--------|---------------|-------------|
| BM-04 | biomeOS | `topology.rescan` + lazy discovery on miss + multi-shape probe | v2.81 |
| BM-05 | biomeOS | Multi-shape `extract_capabilities_from_response` | v2.81 |
| BC-01 | barraCuda | Fitts `variant` param (Shannon default) | Sprint 25 / v0.3.11 |
| BC-02 | barraCuda | Hick `include_no_choice` param | Sprint 25 / v0.3.11 |
| BC-03 | barraCuda | Perlin3D lattice fix (proper gradients + quintic fade) | Sprint 25 / v0.3.11 |
| PT-01 | petalTongue | Socket → `$XDG_RUNTIME_DIR/biomeos/petaltongue.sock` | IPC compliance evolution |
| PT-02 | petalTongue | SSE `/api/events` push (+ `--port` TCP flag) | IPC compliance evolution |
| PT-03 | petalTongue | `motor_tx` drain channel wired | IPC compliance evolution |
| SQ-01 | Squirrel | Filesystem socket via `UniversalListener` | alpha.25b |
| SB-01 | songBird | `health.liveness` canonical name | wave89-90 |
| LS-03 | loamSpine | Startup panic fixed — infant discovery fails gracefully | v0.9.15 |
| RC-01 | rhizoCrypt | UDS transport + `biomeos/` path migration + dual-mode TCP | v0.14.0-dev s23 |

**12 gaps resolved** this cycle. Gap count: **32 → 20 open** (12 resolved, 2 newly identified: NG-04, NG-05). **Zero CRITICAL blockers remaining.**

---

## Live Validation Results (March 31, 2026 — post-evolution)

```
  C1: Render                           6/6  PASS
  C2: Narration                        3/4  PARTIAL (ai.query fails: no Ollama provider — SQ-02)
  C3: Session                          8/8  PASS
  C4: Game Science                     6/6  PASS
  C5: Persistence                      4/5  PARTIAL (storage.list fails — NestGate method gap)
  C6: Proprioception                   5/5  PASS
  C7: Full Interactive                 9/10 PARTIAL (Squirrel AI cross-subsystem probe)

  TOTAL                                41/44  (93%)
```

Previous: 34/43 (79%) → **41/44 (93%)** after evolution cycle.

### Remaining 3 Failures (C1-C7 suite)

| Failure | Composition | Root Cause | Gap ID |
|---------|------------|------------|--------|
| `ai.query` | C2 | No Ollama provider wired into `AiRouter` | SQ-02 |
| `storage.list` | C5 | NestGate method returns error on empty prefix | NG-01 |
| Squirrel AI cross-subsystem | C7 | Squirrel socket at non-biomeos path, C7 probe uses biomeos/ only | SQ-01 (partial — socket at `/run/user/1000/squirrel/squirrel.sock`, not `biomeos/`) |

### Deep Validation — Per-Primal Findings (March 31)

| Primal | Socket | Health Triad | Capabilities | Notable |
|--------|--------|-------------|-------------|---------|
| **biomeOS** | `/biomeos/neural-api-*.sock` | `graph.list` (no `health.liveness` method) | 173+ capability domains, routing works for crypto/storage/viz/ai/game | `topology.rescan` → "Method not found" (running old v2.80 binary, v2.81 not deployed yet) |
| **bearDog** | `/biomeos/beardog-*.sock` | liveness ✅ readiness ✅ | 0 methods in `capabilities.list` (empty response) | `crypto.hash` base64 input required |
| **songBird** | `/biomeos/songbird-*.sock` | liveness ✅ health(short) ✅ | 14 capabilities | Uptime 338K+ seconds, stable |
| **petalTongue** | `/biomeos/petaltongue-ipc.sock` | liveness ✅ readiness ✅ check ✅ | 0 in `capabilities.list` (returns empty) | `identity.get` → not found, `lifecycle.status` → not found (old binary) |
| **NestGate** | `/biomeos/nestgate.sock` | health ✅ | 25 capabilities | `storage.list` works WITH `family_id` param, store/retrieve round-trip ✅ |
| **Squirrel** | `/squirrel/squirrel.sock` (not biomeos!) | liveness ✅ readiness ✅ | 25 capabilities | `ai_router: false` in readiness, 0 providers |
| **ludoSpring** | `/biomeos/ludospring-*.sock` | health.check ✅ | game.* methods work | All game science checks pass |
| **esotericWebb** | `/biomeos/esotericwebb-*.sock` | webb.liveness ✅ | Full session lifecycle | 12 actions, graph, state all working |
| **toadStool** | `/biomeos/toadstool.jsonrpc.sock` | "Method not found" | 0 capabilities | S168 binary — outdated, needs S169 rebuild |
| **rhizoCrypt** | `/biomeos/rhizocrypt.sock` ✅ | liveness ✅ readiness ✅ check ✅ | 4 domains, 26 methods | **RC-01 RESOLVED** (source-built). UDS + dual-mode TCP. plasmidBin needs harvest. |
| **loamSpine** | `/biomeos/loamspine.sock` ✅ | liveness ✅ | 19 capabilities | **LS-03 RESOLVED** — starts cleanly, infant discovery fails gracefully |
| **sweetGrass** | not tested (no binary) | — | — | No plasmidBin or local binary available |
| **barraCuda** | not tested (no binary) | — | — | No plasmidBin or local binary available |

### Projected Impact With Remaining Fixes

| Fix | Result |
|-----|--------|
| SQ-02 (wire `LOCAL_AI_ENDPOINT` into `AiRouter`) | C2 → 4/4 PASS |
| NG-01 (`storage.list` fix) | C5 → 5/5 PASS |
| Squirrel socket in `biomeos/` | C7 → 10/10 PASS |
| **All C1-C7 fixes** | **44/44 (100%)** |

### ludoSpring 141-Check Broader Matrix

The C1-C7 suite covers 44 checks. ludoSpring's full 141-check matrix (exp084-098) includes provenance trio experiments. Both RC-01 and LS-03 are now **RESOLVED**. Projected:

| Fix | Running Total | % |
|-----|---------------|---|
| Previous (pre-evolution) | ~95/141 | 67.4% |
| + LS-03 resolved (v0.9.15) | ~124/141 | 87.9% |
| + RC-01 resolved (v0.14.0-dev s23) | ~133/141 | 94.3% |
| + TS-01 (toadStool S169 rebuild) | ~134/141 | 95.0% |
| + SQ-02 + remaining | ~138/141 | 97.9% |

**Note**: rhizoCrypt's source-built binary has the fix. The plasmidBin binary is still the old TCP-only version and needs harvesting. Once deployed, the provenance trio (rhizoCrypt + loamSpine + sweetGrass) compositions are unblocked.
