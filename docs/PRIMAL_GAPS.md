# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Last updated**: 2026-03-31 — Phase 23f+ (ludoSpring V37.1 live gap matrix absorbed)

---

## petalTongue

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| PT-01 | `EguiCompiler` produces `ModalityOutput::Description` with JSON, not a dedicated `EguiShapes` variant | Medium | C1: Render | Add `EguiShapes` variant to `ModalityOutput` enum in `petal-tongue-scene`, update `egui_compiler.rs` to emit it |
| PT-02 | `web_mode.rs` has no WebSocket/SSE — cannot push live renders to browser, only poll via GET | **High** | C1: Render | Add WS endpoint to axum router in `web_mode.rs`, wire `DataService::subscribe` to push SVG/JSON on change |
| PT-03 | `motor_tx` not wired in server/web mode — `motor.*` IPC methods return error unless egui desktop app is running | Medium | C1: Render, C6: Proprioception | Pass motor channel from IPC server to app state when running in combined mode (server + web or server + egui) |
| PT-04 | No HTML export modality — `visualization.export` with `format: "json"` falls back to SVG | Low | C1: Render | Add `HtmlCompiler` or accept SVG-in-HTML as sufficient for dashboard use |
| PT-05 | `visualization.showing` requires `rendering_awareness` to be `Some` — returns `showing: false` when not wired | Medium | C6: Proprioception | Initialize `rendering_awareness` to a default `RenderingAwareness` on server startup |
| PT-06 | `callback_method` in interaction subscriptions is stored but never invoked — poll-only model | Medium | C6: Proprioception | Implement callback dispatch in `DataService::broadcast_interaction` when `callback_method` is set |
| PT-07 | No external event source wired in server mode — TUI `discover_primals` is separate from `DataService` refresh | Low | C6: Proprioception | Wire IPC capability discovery into `DataService` periodic refresh for server mode |

### petalTongue: Full Rust egui Gap

The current egui integration (`petal-tongue-ui` crate) compiles `SceneGraph` into `ModalityOutput::Description` (a JSON string) rather than native egui shapes. To reach **full Rust egui** as the primary interface:

1. Define `ModalityOutput::EguiShapes(Vec<EguiShape>)` with shape primitives mapped to `egui::Shape`
2. Update `EguiCompiler::compile()` to emit `EguiShapes` instead of `Description`
3. Wire the egui app (`app.rs`) to consume `EguiShapes` directly for rendering
4. Add `motor_tx` passthrough so server-mode interactions drive the egui event loop
5. This replaces the SVG-in-HTML dashboard with a native desktop GUI driven entirely by primals

---

## Squirrel

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| SQ-01 | `AiRouter` discovery has 3 paths (HTTP providers with API keys, `AI_PROVIDER_SOCKETS` UDS, biomeOS scan for ToadStool) — none natively support Ollama HTTP | **High** | C2: Narration | Add a "local" provider to `http_provider_config.rs` that wraps the OpenAI-compatible endpoint at `LOCAL_AI_ENDPOINT`, or create a tiny Ollama-to-UDS bridge primal |
| SQ-02 | `LOCAL_AI_ENDPOINT` env var feeds into `AIProviderConfig` and `DefaultEndpoints` but NOT into `AiRouter` discovery — config/router disconnect | Medium | C2: Narration | Wire `LOCAL_AI_ENDPOINT` env into provider initialization in `AiRouter::new()` or `discover_providers()` |
| SQ-03 | `deprecated-adapters` feature flag needed for most HTTP providers — gate not clearly documented | Low | C2: Narration | Document feature flags in Squirrel README; consider making local HTTP provider unconditional |

### Squirrel: Local AI Integration Path

For HuggingFace models via Ollama (the "lovely mid point"):

1. Ensure `LOCAL_AI_ENDPOINT=http://localhost:11434` is set
2. Fix SQ-01/SQ-02 so `AiRouter` discovers Ollama as a provider
3. `ai.query` then routes to Ollama's `/api/chat` endpoint (OpenAI-compatible)
4. Any HuggingFace model pulled into Ollama (`ollama pull <model>`) becomes available
5. No API keys needed — fully local, fully primal-controlled

---

## NestGate

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| NG-01 | Isomorphic IPC adapter uses **in-memory KV** — `nestgate-rpc/src/rpc/isomorphic_ipc/unix_adapter.rs` stores data in `HashMap`, not on disk | Medium | C5: Persistence | Wire `nestgate-core` storage backend (RocksDB/sled) into the IPC adapter's `storage.*` handlers |
| NG-02 | No dedicated game session API — session persistence requires generic `storage.store` with session-keyed blobs | Low | C5: Persistence | Add `session.save` / `session.load` convenience methods that wrap `storage.store` with structured session schema |
| NG-03 | `data.*` handlers (NCBI, NOAA, IRIS) are live-data providers unrelated to game persistence — confusing capability surface | Low | C5: Persistence | Namespace clearly: `data.*` for live feeds, `storage.*` for persistence; document the distinction |

### NestGate: Persistence Reality

Current state: NestGate's IPC layer accepts `storage.store` and `storage.retrieve` but stores everything in a process-local `HashMap`. Data is lost on restart. The `nestgate-core` crate has actual storage backends but they are not wired into the RPC layer yet. This means C5 validation will pass for round-trip tests within a session but fail for persistence-across-restart tests.

---

## esotericWebb

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| EW-01 | `push_scene_to_ui` sends flat JSON `{node, description, npcs, turn, is_ending}` to petalTongue's `visualization.render.scene`, which expects `SceneGraph` struct — deserialization likely fails silently | **High** | C3: Session | Adapt `push_scene_to_ui` to construct `DataBinding::GameScene` or a proper `SceneGraph` with tagged `channel_type` |
| EW-02 | `poll_input` (petalTongue `interaction.poll`) exists on bridge but is NOT wired into the game loop — no UI-driven feedback into session | Medium | C3: Session | Add `interaction.poll` call in session tick/act cycle; map polled intents to session actions |
| EW-03 | `replay` command not implemented — `cmd_replay` returns error | Low | C3: Session | Wire provenance DAG to replay engine; low priority until NestGate persistence is real |
| EW-04 | V6 internal game science duplicates some ludoSpring capabilities — no composition-level dedup | Low | C4: Game Science | Define composition contract: when ludoSpring is present, esotericWebb defers to external `game.evaluate_flow`; otherwise uses internal |

---

## biomeOS

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| BM-01 | `graph.deploy` not in routing table — had to use `graph.execute` instead | **Fixed** | — | primalSpring now calls `graph.execute` |
| BM-02 | `health.liveness` not implemented on Neural API — fallback to `graph.list` needed | Low | All | Already handled in `NeuralBridge::health_check`; optional to add proper `health.liveness` to biomeOS |
| BM-03 | `capability.discover` returns `primary_endpoint` with `unix://` prefix — inconsistent with `primary_socket` used elsewhere | **Fixed** | — | primalSpring `strip_unix_uri` handles both formats |
| BM-04 | **Neural API capability registration: running primals not auto-registered** — `capability.list` only shows 5 biomeOS self-capabilities; `discover_and_register_primals` runs once at startup with 500ms timeouts, fails if primals aren't yet listening. No retry loop. | **High** | ludoSpring V37.1 (14 checks fail: exp084, exp087, exp088) | 3 options: (1) retry `discover_and_register_primals` periodically or on first `capability.call` miss, (2) primals self-register via `capability.register` on startup, (3) `topology.rescan` callable from outside |
| BM-05 | `probe_primal_capabilities` expects `capabilities.list` response format — primals returning different JSON shape or framing get empty list, silently skipped | Medium | ludoSpring V37.1 | Standardize expected response shape; add debug logging for mismatched formats |

### biomeOS: Capability Registration Architecture

The root cause of BM-04 is that `discover_and_register_primals()` in `server_lifecycle.rs` runs **once** at startup. The flow:
1. Scans `$XDG_RUNTIME_DIR/biomeos/*.sock` for known primal names
2. Sends `capabilities.list` with 500ms timeout
3. Registers non-empty responses in `NeuralRouter.capability_registry`

If primals start **after** biomeOS (which is the normal graph startup order), they miss the window. Category routing (`discover_by_capability_category`) falls back to socket scanning, but `capability.call` reports the domain as unregistered.

**Fix paths:**
- **Quick**: Add `topology.rescan` JSON-RPC method that re-runs `discover_and_register_primals`
- **Medium**: Retry discovery on first `capability.call` miss for an unregistered domain
- **Full**: Primals call `capability.register` on their own startup (requires SDK change)

---

## rhizoCrypt

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| RC-01 | **TCP-only transport** — binds `0.0.0.0:9401` for JSON-RPC but creates no Unix domain socket | **Critical** | ludoSpring V37.1 (blocks exp094, exp095, exp096, exp098 = +9 checks) | Add `--unix [PATH]` CLI flag, default to `$XDG_RUNTIME_DIR/biomeos/rhizocrypt.sock`. Follow BearDog/NestGate/sweetGrass/barraCuda pattern |

### rhizoCrypt: UDS is Prerequisite for All Composition

Every other ecoBin primal follows `$XDG_RUNTIME_DIR/biomeos/{primal}.sock`. rhizoCrypt being TCP-only means it cannot participate in any composition graph that expects UDS discovery. This is the #1 blocker for provenance trio compositions (rhizoCrypt + loamSpine + sweetGrass).

---

## loamSpine

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| LS-03 | **Panic on startup**: "Cannot start a runtime from within a runtime" at `infant_discovery.rs:233` — Tokio `block_on()` called inside existing async runtime | **Critical** | ludoSpring V37.1 (blocks exp095 = +6 checks) | Replace `block_on()` with `spawn` or restructure infant discovery to avoid nesting runtimes |

---

## barraCuda

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| BC-01 | **Fitts formula mismatch**: `activation.fitts(d=256, w=32, a=200, b=150)` → 800 (Welford `log2(D/W)`) vs Python 708.85 (Shannon `log2(2D/W+1)`) | **High** | ludoSpring V37.1 (exp089: -4 checks) | Add `variant` parameter to select Shannon vs Welford; default to Shannon (most-cited) |
| BC-02 | **Hick formula variant**: `activation.hick(n=8, a=200, b=150)` → 675.49 (`log2(n+1)`) vs Python 650 (`log2(n)`) | Medium | ludoSpring V37.1 (exp089) | Add `include_no_choice` parameter; default to standard `log2(n)` |
| BC-03 | **Perlin3D lattice invariant broken**: `noise.perlin3d(0,0,0)` → -0.11 instead of 0. Gradient noise must be zero at integer lattice points | Medium | ludoSpring V37.1 (exp091: -1 check) | Fix gradient interpolation at lattice boundaries in 3D implementation |
| BC-04 | **No binary in plasmidBin** — must be started from source build | Medium | ludoSpring V37.1 | Publish ecoBin to `infra/plasmidBin/barracuda/` |

---

## toadStool

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| TS-01 | `compute.dispatch` cannot find coralReef despite socket at `/run/user/1000/biomeos/coralreef-core-default.sock` — hardcoded discovery | Medium | ludoSpring V37.1 (exp085: -1 check) | Discover coralReef via socket directory scan or Songbird query; aligns with S169 overstep cleanup |

**Note**: toadStool S169 just completed major overstep cleanup (-10,659 lines): removed 30+ JSON-RPC methods belonging to Squirrel, coralReef, biomeOS, songBird. Shifted to pure JSON-RPC over UDS. Discovery should now use capability-based resolution.

---

## plasmidBin

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| PB-01 | NestGate JWT secret in `start_primal.sh` generates 25-byte key; NestGate requires 32+ bytes | Low | ludoSpring V37.1 | Use `openssl rand -base64 48` in the NestGate case block |

---

## ludoSpring

| ID | Gap | Severity | Exposed By | Fix Path |
|----|-----|----------|------------|----------|
| LS-01 | Gateway hardcodes flow params (`skill: 0.5, challenge: 0.5, time_pressure: 0.3`) instead of deriving from game state | Medium | C4: Game Science | Define mapping: `esotericWebb session.state.trust` → `skill`, `scene difficulty` → `challenge` |
| LS-02 | No composition-level contract for when esotericWebb should call ludoSpring vs use internal game science | Low | C4: Game Science | See EW-04 — define capability precedence in composition graph metadata |

---

## Cross-Cutting Gaps

| ID | Gap | Severity | Fix Path |
|----|-----|----------|----------|
| XC-01 | No standard `DataBinding` construction library — gateway, esotericWebb, and any future consumer all build bindings ad-hoc | Medium | Create `ecoPrimal::databinding` module with typed constructors for gauge, timeseries, GameScene, etc. |
| XC-02 | Thin gateway still knows primal socket paths — should use `capability.discover` exclusively | Medium | Refactor gateway to resolve all primals via biomeOS Neural API `capability.discover` |
| XC-03 | No composition health aggregator — each subsystem validated independently but no "is the whole product healthy?" endpoint | Medium | Add `composition.health` to C7 graph that probes all C1-C6 health in sequence |

---

## Priority Order

**Critical** (primals cannot participate in composition):
1. RC-01 — rhizoCrypt UDS transport (blocks all provenance trio compositions, +9 checks)
2. LS-03 — loamSpine startup panic (blocks provenance trio, +6 checks)

**High** (blocks interactive product or major capability routing):
3. BM-04 — biomeOS capability registration (running primals invisible to `capability.call`, +14 checks)
4. BC-01 — barraCuda Fitts formula mismatch (wrong HCI results, +4 checks)
5. PT-02 — petalTongue WebSocket push (enables live dashboard without polling)
6. SQ-01 — Squirrel Ollama routing (enables AI narration via primals)
7. EW-01 — esotericWebb scene format (enables primal-driven rendering)

**Medium** (improves composition quality):
8. NG-01 — NestGate real persistence
9. BC-02 — barraCuda Hick formula variant
10. BC-03 — barraCuda Perlin3D lattice invariant
11. BC-04 — barraCuda plasmidBin binary
12. BM-05 — biomeOS probe response format standardization
13. TS-01 — toadStool↔coralReef discovery
14. PT-05 — petalTongue awareness initialization
15. EW-02 — esotericWebb poll_input wiring
16. LS-01 — ludoSpring dynamic flow params

**Low** (polish):
17. PT-01, PT-03, PT-04, PT-06, PT-07
18. SQ-02, SQ-03
19. NG-02, NG-03
20. EW-03, EW-04, LS-02
21. PB-01 — plasmidBin NestGate JWT secret length
22. XC-01, XC-02, XC-03

## Projected Impact (from ludoSpring V37.1 gap matrix)

| Fix | Checks Gained | Running Total | % |
|-----|--------------|---------------|---|
| Current | — | 95/141 | 67.4% |
| + RC-01 (rhizoCrypt UDS) | +9 | 104/141 | 73.8% |
| + LS-03 (loamSpine panic) | +6 | 110/141 | 78.0% |
| + BC-01/02/03 (barraCuda formulas) | +5 | 115/141 | 81.6% |
| + BM-04 (biomeOS capability reg) | +14 | 129/141 | 91.5% |
| + TS-01 (toadStool↔coralReef) | +1 | 130/141 | 92.2% |
| **All fixes** | **+35** | **130/141** | **92.2%** |
