# Storytelling Evolution — ludoSpring + esotericWebb

**Status**: EVOLUTION GUIDANCE (updated Phase 56)  
**Date**: April 28, 2026 (originally March 28, 2026)  
**Source**: primalSpring substrate validation, leverage guides, experiment gaps, EVOLUTION_GAPS.md  
**Scope**: Evolution recommendations to get ludoSpring and esotericWebb online, visualized, and interactable with an AI DM for storytelling  
**Related specs**: `DESKTOP_SESSION_MODEL.md`, `LIVE_GUI_COMPOSITION_PATTERN.md`

---

## The Storytelling Stack

| Layer | Component | Role |
|-------|-----------|------|
| **Narrative engine** | esotericWebb | DAG-traced CRPG: scene resolution, NPCs, abilities, bounded endings |
| **Game science** | ludoSpring | Flow, DDA, engagement, Fitts, WFC, tick budget, sessions |
| **AI Dungeon Master** | Squirrel | ai.chat with context windows, narrative generation, tool execution |
| **Scene rendering** | petalTongue | visualization.render.scene, interaction.poll, TUI/egui/web |
| **Orchestration** | biomeOS | Neural API routing, graph execution (Continuous @ 60Hz) |
| **Session integrity** | Provenance trio | rhizoCrypt DAG, loamSpine lineage, sweetGrass attribution |

The storytelling loop: **player input → scene resolution → game science → AI narration → provenance stamp → scene render → repeat @ 60Hz**

---

## ludoSpring Evolution Guidance

### Current State (V53, Phase 55c)

- **Pure composition model**: 12-node cell graph, 30 capabilities, BTSP-enforced
- 60Hz tick budget with `TickBudget` and `SessionPhase` state machine
- 817 tests, zero clippy
- IPC surface: `game.evaluate_flow`, `game.fitts_cost`, `game.engagement`, `game.analyze_ui`, `game.accessibility`, `game.wfc_step`, `game.difficulty_adjustment`, `game.generate_noise`
- petalTongue push client (runtime, no crate dependency)
- barraCuda math dependency, optional wgpu GPU
- RPGPT architecture sketch in specs/
- Discovery self-registration via `DISCOVERY_SOCKET`
- esotericWebb bridge wiring present

### P0: Expand IPC Surface for esotericWebb

**Gap**: esotericWebb's `ludospring.rs` bridge calls `game.narrate_action`, `game.npc_dialogue`, `game.voice_check`, `game.push_scene`, `game.begin_session`, `game.complete_session` — none of which exist in the current `ludospring` binary's handler dispatch.

**Evolution**:
- Add handlers for Webb's required methods in `barracuda/src/ipc/handlers.rs`:
  - `game.narrate_action` — compose flow state + DDA context into a narration prompt, delegate to Squirrel via `capability.call("ai", "chat", ...)` or return structured narration data
  - `game.npc_dialogue` — evaluate NPC state against ruleset, return dialogue options with flow scores
  - `game.voice_check` — Fitts + engagement metrics for voice/ability interaction assessment
  - `game.push_scene` — forward scene data to petalTongue via `visualization.render.scene`
  - `game.begin_session` / `game.complete_session` — session lifecycle with optional rhizoCrypt DAG integration
- Register all new methods in the `CAPABILITIES` array in `ludospring.rs`

**Validation**: exp023 `actual_session` stops being skipped; gen4_storytelling_full graph connects end-to-end

### P0: plasmidBin Deployment

**Gap**: GEN4_BRIDGE.md lists `ludospring` as NOT YET DEPLOYED to plasmidBin. This blocks all substrate validation where benchScale needs to fetch the binary.

**Evolution**:
- Build `ludospring` for x86_64-unknown-linux-gnu and aarch64-unknown-linux-gnu
- Strip + package as ecoBin-compliant artifacts
- Register in `plasmidBin/sources.toml` with version and hash
- Ensure `primal_launch_profiles.toml` entry matches binary name

**Validation**: deployment matrix storytelling cells can provision ludoSpring

### P1: Graph Capability Alignment

**Gap**: `graphs/sketches/validation/ludospring_validate.toml` expects `game.engine`, `game.physics`, `game.flow_state`, `game.tick_health` — none match actual IPC method names.

**Evolution**:
- Align graph `capabilities` arrays with actual IPC surface
- Add `game.tick_health` as a health check method (returns tick budget headroom)
- Consider `game.engine` as an alias for the full game science surface
- Update all primalSpring graphs referencing ludoSpring

### P1: Session State Streaming

**Gap**: `ludospring_live_session` binary exists (120-tick stream) but is separate from the main `ludospring` server. esotericWebb needs real-time game state as part of the session loop, not a separate process.

**Evolution**:
- Integrate live session streaming into the main `ludospring server` process
- JSON-RPC notification pattern: `game.session.tick` → push to subscribers
- petalTongue subscribes via `interaction.subscribe` for real-time dashboard updates

### P2: Ruleset Certification

**Gap**: `game/ruleset.rs` has abstract ruleset types but `game.ruleset_validate` (esotericWebb GAP-009) doesn't exist.

**Evolution**:
- `game.ruleset_validate` — validate a YAML ruleset against structural rules (die types, stat blocks, ability interactions)
- Return `RulesetCert` that loamSpine can certify and store
- Enables community-created rulesets with provenance

---

## esotericWebb Evolution Guidance

### Current State (gardens/esotericWebb, V7)

- 342 tests, ~91% coverage
- CRPG substrate: YAML content (worlds, NPCs, abilities, scenes, narrative graphs)
- Runtime loop: input → scene → predicates/effects → provenance → narration → viz → flow/DDA
- IPC bridges: ludospring.rs, squirrel.rs, petaltongue.rs, provenance.rs
- **PrimalBridge**: 7 primal domains consumed via capability discovery, all degrading gracefully
- 4 degradation patterns: `call_or_default`, `call_fire`, `call_extract_id`, `call_passthrough`
- Deploy graphs compose from NUCLEUS fragments (`tower_atomic`, `node_atomic`, `nest_atomic`, `meta_tier`)
- Game science absorbed locally — no spring runtime dependencies
- Neural API fallback provides transparent AI evolution
- TCP-first transport (vs biomeOS UDS-first) — UDS fallback when `PRIMAL_TRANSPORT=uds`
- EVOLUTION_GAPS.md with gaps (several resolved by Phase 55c upstream work)

### P0: ludoSpring IPC Alignment

**Gap**: Webb calls `game.narrate_action`, `game.npc_dialogue`, etc. that don't exist in ludoSpring yet. Squirrel `ai.chat` fallback works, but game-science-enriched narration is the differentiator.

**Evolution**:
- Coordinate with ludoSpring P0 (IPC surface expansion)
- Once ludoSpring exposes the methods, Webb's bridge code is already wired
- Validate with gen4_storytelling_full graph

### P0: Transport Negotiation

**Gap**: esotericWebb is TCP-first (`webb/src/ipc/mod.rs`), biomeOS routes via UDS-first. This creates a transport mismatch that biomeOS can't bridge until TCP-only API mode (biomeOS P0) is resolved.

**Evolution**:
- Short-term: esotericWebb falls back to UDS when `PRIMAL_TRANSPORT=uds` is set
- Long-term: biomeOS TCP-only mode resolves this at the orchestrator level
- Test both paths in deployment matrix: `storytelling-x86-homelan-uds` and `storytelling-x86-homelan-tcp`

### P1: petalTongue Scene Types (GAP-002)

**Gap**: petalTongue doesn't have a dialogue-tree scene type. Webb's `visualization.render.scene` sends scene data, but petalTongue needs to understand narrative-specific rendering.

**Evolution**:
- Define `SceneType::DialogueTree` in petalTongue's scene graph
- NPC portrait rendering, dialogue options, ability check indicators
- Branch visualization (which DAG path the player is on)
- Grammar of Graphics for game state dashboards (health bars, stat blocks)

**Validation**: gen4_storytelling_full with petalTongue rendering complete scenes

### P1: Squirrel Mechanical Constraints (GAP-003)

**Gap**: Squirrel `ai.chat` generates free-form narration but doesn't enforce game mechanical constraints (dice results, stat checks, ability cooldowns). NPC dialogue might violate game rules.

**Evolution**:
- esotericWebb passes mechanical context (resolved predicates, ability costs, dice results) as structured `context.create` data to Squirrel
- Squirrel's AI prompt includes these constraints
- Post-narration validation: esotericWebb checks Squirrel's output against game state for consistency
- ludoSpring `game.voice_check` provides the mechanical validation

### P1: Provenance Trio Live E2E (GAP-004)

**Gap**: rhizoCrypt wiring exists but the full trio (rhizoCrypt + loamSpine + sweetGrass) hasn't been validated end-to-end in a real session.

**Evolution**:
- Session start → `rhizocrypt.dag.session.create`
- Each scene → `rhizocrypt.dag.event.append` with full event payload
- Player choices → `loamspine.lineage.branch`
- AI narrations → `sweetgrass.provenance.create_braid` for attribution
- Session end → `loamspine.lineage.certify`
- Replay: load DAG, verify chain, re-render via petalTongue

**Validation**: rpgpt_session_provenance graph with provenance trio active

### P2: Content Pack Format (GAP-008)

**Gap**: World content is YAML files in `content/`. No standardized pack format for distribution.

**Evolution**:
- Define `.webbpack` format: signed archive (BearDog) of YAML content
- NestGate stores packs; loamSpine certifies authorship
- Squirrel can discover and recommend packs via `tool.list`
- petalTongue renders a pack browser

### P2: Songbird Capability-Filtered Discovery (GAP-006)

**Gap**: Songbird's `discovery.query` doesn't support filtering by capability. Webb must discover all primals then filter locally.

**Evolution**:
- Songbird adds `discovery.query({ capabilities: ["game.*"] })` → returns only matching primals
- Reduces discovery overhead in large meshes
- Benefits all primals, not just Webb

---

## Cross-Cutting: The Storytelling Loop End-to-End

### What Works Today (Phase 55c)

1. esotericWebb V7 has the full runtime loop with PrimalBridge + graceful degradation
2. ludoSpring V53 has 8 game science methods + pure composition model + 60Hz tick
3. Squirrel `ai.chat` provides AI narration with context windows + HTTP provider support
4. petalTongue renders dashboards and basic scenes, `live` mode validated
5. rhizoCrypt DAG session wiring exists in esotericWebb
6. All 12 NUCLEUS primals resolved — full Desktop NUCLEUS operational
7. Provenance trio: loamSpine ships Tower-signed entries, sweetGrass ships braid+anchor signing delegation
8. Two-tier crypto architecture operational (seed fingerprints -> HKDF -> purpose keys)

### What Doesn't Work Yet

1. **ludoSpring missing 6 methods** that esotericWebb calls (game.narrate_action, etc.) — see `LUDOSPRING_IPC_EXPANSION_PHASE56_APR28_2026.md`
2. **Transport mismatch**: Webb TCP-first vs biomeOS UDS-first (partially resolved: Webb has UDS fallback)
3. **petalTongue lacks dialogue-tree scene type** — scenes render but aren't narrative-specific
4. **Squirrel narration ignores game mechanics** — free-form text vs constrained by rules
5. **Provenance trio untested E2E** in a real storytelling session (individual primals validated)
6. **No desktop application session model** — esotericWebb runs standalone, not as a biomeOS-managed session (see `DESKTOP_SESSION_MODEL.md`)

### Recommended New Experiments

| ID | Name | What It Validates |
|----|------|-------------------|
| exp102 | `storytelling_session_loop` | Full loop: Webb → ludoSpring → Squirrel → petalTongue → provenance |
| exp103 | `ludospring_expanded_ipc` | New ludoSpring IPC methods (narrate_action, npc_dialogue, etc.) |
| exp104 | `rpgpt_provenance_replay` | Session record → DAG → replay with verification |

---

## Priority Summary

| Priority | ludoSpring | esotericWebb |
|----------|-----------|--------------|
| **P0** | Expand IPC (6 methods for Webb), plasmidBin deploy | ludoSpring IPC alignment, transport negotiation |
| **P1** | Graph capability alignment, session streaming | petalTongue scene types, Squirrel constraints, provenance E2E |
| **P2** | Ruleset certification | Content pack format, Songbird filtered discovery |

---

## Deployment Readiness (Phase 55c / Phase 56)

| Component | Ready? | Blocker |
|-----------|--------|---------|
| esotericWebb binary | Yes (CLI `serve`, PrimalBridge V7) | Transport mismatch partially resolved (UDS fallback) |
| ludoSpring binary | Partial (8 of 14 methods) | Missing 6 Webb-required methods |
| Squirrel AI DM | Yes (`ai.chat` + context + HTTP providers) | Socket alignment partially resolved (DISCOVERY_SOCKET) |
| petalTongue scenes | Partial (dashboards + live mode work) | Missing dialogue-tree scene type |
| biomeOS orchestration | **Improved** (Desktop NUCLEUS validated) | `--mode desktop` native launch pending (Phase 56) |
| Provenance trio | **Improved** (all 3 validated individually, Tower-signed) | Untested E2E in real storytelling session |
| plasmidBin ludoSpring | **RESOLVED** — pure composition model | No spring binary needed (composes NUCLEUS primals) |
| Desktop session model | **NEW** (Phase 56) | `app.launch` API not yet implemented in biomeOS |

### Phase 56 Desktop Application Target

esotericWebb becomes the **first desktop application** running on the
biomeOS substrate via `app.launch`. The application graph
(`app_esotericwebb.toml`) composes esotericWebb + ludoSpring on top of
the running 12-primal NUCLEUS. A `ContinuousSession` at 60Hz drives the
game loop, with petalTongue providing the display surface.

See `DESKTOP_SESSION_MODEL.md` for the full application lifecycle spec.
