# primalSpring V0.7.0 → Phase 22: ludoSpring game.* Method Gap Handoff

**Date**: 2026-03-29  
**From**: primalSpring (coordination/composition validation)  
**To**: ludoSpring, esotericWebb  
**Priority**: P0 (blocks storytelling composition deployment)

## Context

primalSpring Phase 22 (Track 14) built E2E composition experiments (exp085–exp088) to validate functional deployments. **exp088 (Storytelling Composition)** validates the full ludoSpring + esotericWebb + Squirrel + petalTongue stack via biomeOS Neural API routing.

During this work, we confirmed a critical method gap: **esotericWebb expects 3 `game.*` RPC methods that ludoSpring does not yet implement**.

## The Gap

### Methods esotericWebb Calls (from `webb/src/ipc/ludospring.rs`)

| Method | Purpose | Status |
|--------|---------|--------|
| `game.npc_dialogue` | NPC dialogue generation for RPGPT AI DM | **NOT IMPLEMENTED** |
| `game.narrate_action` | Action narration for AI DM storytelling | **NOT IMPLEMENTED** |
| `game.begin_session` | Session lifecycle management | **NOT IMPLEMENTED** |

### Methods ludoSpring Implements (from `barracuda/src/ipc/handlers.rs`)

| Method | Purpose | Status |
|--------|---------|--------|
| `game.evaluate_flow` | Csikszentmihalyi flow state evaluation | Implemented |
| `game.difficulty_adjustment` | Dynamic difficulty adjustment | Implemented |
| `game.wfc_step` | Wave Function Collapse step | Implemented |
| `game.fitts_cost` | Fitts's law cost analysis | Implemented |
| `game.engagement` | Engagement metrics | Implemented |
| `game.health` | Health check | Implemented |
| `game.session_state` | Session state query | Implemented |
| `game.configure` | Runtime configuration | Implemented |

## Impact

- The `ecoprimals-storytelling-tower.yaml` benchScale topology cannot run end-to-end until these 3 methods are implemented.
- exp088 marks these as `check_skip` GAP items — they will auto-promote to `check_bool` once ludoSpring implements them.
- Squirrel's AI DM cannot invoke narration through the standard `game.*` RPC path and must fall back to `ai.chat`.

## Recommended Actions

### ludoSpring

1. **Implement `game.npc_dialogue`** — accept `{ "npc_id": string, "context": object }`, return dialogue text. Can delegate to Squirrel `ai.chat` internally for AI-generated dialogue.
2. **Implement `game.narrate_action`** — accept `{ "action": string, "scene_context": object }`, return narration text. Same AI delegation pattern.
3. **Implement `game.begin_session`** — accept `{ "session_id": string, "config": object }`, return session state. Wire to existing `game.session_state` and `game.configure` internals.

### esotericWebb

1. Once ludoSpring implements these methods, remove fallback paths in `webb/src/ipc/ludospring.rs`.
2. The NPC dialogue flow should be: `esotericWebb → game.npc_dialogue → ludoSpring → ai.chat → Squirrel` (not direct `ai.chat`).

## primalSpring Tracking

- **exp088** (`exp088_storytelling_composition`) — validates all 8+ game.* methods including the 3 gap methods.
- **`ipc::methods::game`** — canonical constants for all game.* method names.
- **`ecoprimals-storytelling-tower.yaml`** — benchScale topology for the full stack.
- **Method constants**: `primalspring::ipc::methods::game::*` — all springs and primals should use these constants.

## Metrics

| Metric | Phase 21 | Phase 22 |
|--------|----------|----------|
| Experiments | 63 | 67 |
| Tests | 411 | 413 |
| IPC method modules | 10 | 16 |
| Composition experiments | 0 | 4 |
| benchScale topologies referenced | 2 | 4 |
