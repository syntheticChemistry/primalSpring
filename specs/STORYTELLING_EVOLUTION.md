# Storytelling Evolution — ludoSpring + esotericWebb

**Status**: EVOLUTION GUIDANCE  
**Date**: March 28, 2026  
**Source**: primalSpring substrate validation, leverage guides, experiment gaps, EVOLUTION_GAPS.md  
**Scope**: Evolution recommendations to get ludoSpring and esotericWebb online, visualized, and interactable with an AI DM for storytelling

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

### Current State (v14, workspace 82 members)

- `ludospring-barracuda` core: game science library + 8 IPC methods
- IPC surface: `game.evaluate_flow`, `game.fitts_cost`, `game.engagement`, `game.analyze_ui`, `game.accessibility`, `game.wfc_step`, `game.difficulty_adjustment`, `game.generate_noise`
- 60Hz tick budget with `TickBudget` and `SessionPhase` state machine
- petalTongue push client (runtime, no crate dependency)
- barraCuda math dependency, optional wgpu GPU
- RPGPT architecture sketch in specs/

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

**Gap**: `graphs/spring_validation/ludospring_validate.toml` expects `game.engine`, `game.physics`, `game.flow_state`, `game.tick_health` — none match actual IPC method names.

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

### Current State (gardens/esotericWebb, V4)

- CRPG substrate: YAML content (worlds, NPCs, abilities, scenes, narrative graphs)
- Runtime loop: input → scene → predicates/effects → provenance → narration → viz → flow/DDA
- IPC bridges: ludospring.rs, squirrel.rs, petaltongue.rs, provenance.rs
- TCP-first transport (vs biomeOS UDS-first)
- 5 internal experiments (narrative reachability → autoplay coverage)
- EVOLUTION_GAPS.md with 10 open gaps

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

### What Works Today

1. esotericWebb has the runtime loop coded: input → scene → narration → viz → flow
2. ludoSpring has 8 game science methods working over IPC
3. Squirrel ai.chat provides AI narration with context windows
4. petalTongue renders dashboards and basic scenes
5. rhizoCrypt DAG session wiring exists in esotericWebb

### What Doesn't Work Yet

1. **ludoSpring missing 6 methods** that esotericWebb calls (game.narrate_action, etc.)
2. **Transport mismatch**: Webb TCP-first vs biomeOS UDS-first
3. **petalTongue lacks dialogue-tree scene type** — scenes render but aren't narrative-specific
4. **Squirrel narration ignores game mechanics** — free-form text vs constrained by rules
5. **Provenance trio untested E2E** in a real session
6. **ludoSpring not in plasmidBin** — can't be provisioned by benchScale

### Recommended New Experiments

| ID | Name | What It Validates |
|----|------|-------------------|
| exp088 | `storytelling_session_loop` | Full loop: Webb → ludoSpring → Squirrel → petalTongue → provenance |
| exp089 | `ludospring_expanded_ipc` | New ludoSpring IPC methods (narrate_action, npc_dialogue, etc.) |
| exp090 | `rpgpt_provenance_replay` | Session record → DAG → replay with verification |

---

## Priority Summary

| Priority | ludoSpring | esotericWebb |
|----------|-----------|--------------|
| **P0** | Expand IPC (6 methods for Webb), plasmidBin deploy | ludoSpring IPC alignment, transport negotiation |
| **P1** | Graph capability alignment, session streaming | petalTongue scene types, Squirrel constraints, provenance E2E |
| **P2** | Ruleset certification | Content pack format, Songbird filtered discovery |

---

## Deployment Readiness

| Component | Ready? | Blocker |
|-----------|--------|---------|
| esotericWebb binary | Yes (CLI `serve`) | Transport mismatch with biomeOS |
| ludoSpring binary | Partial (8 of 14 methods) | Missing Webb-required methods |
| Squirrel AI DM | Yes (`ai.chat` + context) | Abstract socket routing (biomeOS P2) |
| petalTongue scenes | Partial (dashboards work) | Missing dialogue-tree scene type |
| biomeOS orchestration | Partial | TCP-only mode not implemented |
| Provenance trio | Wired but untested E2E | Needs live trio + session |
| plasmidBin ludoSpring | Not deployed | Build + register + strip |
