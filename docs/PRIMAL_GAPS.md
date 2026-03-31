# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Last updated**: 2026-03-28 — Phase 23f (Composition Decomposition)

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

**Immediate** (blocks interactive product):
1. PT-02 — petalTongue WebSocket push (enables live dashboard without polling)
2. SQ-01 — Squirrel Ollama routing (enables AI narration via primals)
3. EW-01 — esotericWebb scene format (enables primal-driven rendering)

**Next** (improves composition quality):
4. NG-01 — NestGate real persistence
5. PT-05 — petalTongue awareness initialization
6. EW-02 — esotericWebb poll_input wiring
7. LS-01 — ludoSpring dynamic flow params

**Later** (polish):
8. PT-01, PT-03, PT-04, PT-06, PT-07
9. SQ-02, SQ-03
10. NG-02, NG-03
11. EW-03, EW-04, LS-02
12. XC-01, XC-02, XC-03
