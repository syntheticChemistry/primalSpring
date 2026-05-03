# Micro-Desktop Composition Specification

**Date**: April 28, 2026
**Status**: Phase 56 -- Desktop Substrate
**Related**: `RHIZOME_MICRO_GAME.md`, `DESKTOP_SESSION_MODEL.md`

---

## Overview

The micro-desktop is a biomeOS-managed shell that composes the NUCLEUS into a
functioning desktop environment. It wraps **The Rhizome** game as its primary
application, with a system health monitor bar and a provenance sidebar, all
rendered through petalTongue.

The desktop validates biomeOS's graph orchestration, multi-session petalTongue
rendering, and capability routing in a real composition scenario.

---

## Desktop Layout

```
+---------------------------------------------------------------+
|  [System Monitor Bar]            HP:13/13  Session:active  |F1|
+---------------------------------------------------------------+
|                                          |  Provenance      |
|                                          |  Sidebar         |
|         Game Viewport                    |                  |
|         (40x25 tile grid)                |  [DAG chain]     |
|                                          |  save@turn247    |
|                                          |  save@turn180    |
|                                          |  session_start   |
|                                          |                  |
|                                          |  [Merkle Root]   |
|                                          |  a3f8c2...       |
+---------------------------------------------------------------+
|  [Message Bar]  You found a Tensor Shard!                     |
+---------------------------------------------------------------+
```

### Zones

| Zone | Size (cols x rows) | petalTongue Session | Content |
|------|-------------------|-------------------|---------|
| System Monitor Bar | 60x1 | `desktop-shell` | Primal health indicators, session status |
| Game Viewport | 40x25 | `rhizome-game` | The Rhizome tile grid |
| Provenance Sidebar | 20x25 | `desktop-shell` | DAG event list, merkle root |
| Message Bar | 60x1 | `rhizome-game` | Game messages, narration |

---

## Session Model

### biomeOS Graph Sessions

The desktop composes two biomeOS graph sessions:

1. **`desktop-shell`** (continuous, 1Hz tick)
   - Polls primal health via `capability.call` -> Songbird
   - Updates system monitor bar
   - Polls rhizoCrypt for DAG updates -> provenance sidebar
   - Manages game session lifecycle (start, pause, save-quit)

2. **`rhizome-game`** (continuous, 10Hz tick)
   - Runs the game loop (see RHIZOME_MICRO_GAME.md)
   - Pushes scene to its own petalTongue viewport
   - Fires save events to provenance trio

### Session Lifecycle

```
desktop_nucleus.sh start
  └─> biomeOS starts desktop-shell graph
        └─> shell health-polls primals (1Hz)
        └─> shell launches rhizome-game graph
              └─> game initializes world (Barracuda + ludoSpring)
              └─> game begins turn loop (10Hz)
              └─> game saves trigger provenance events
        └─> shell updates provenance sidebar from DAG
```

### petalTongue Multi-Session

Both sessions render to the same petalTongue instance but use separate
scene hierarchies:

- `desktop-shell` owns the chrome (system bar, sidebar, message bar)
- `rhizome-game` owns the game viewport

This exercises petalTongue's multi-session rendering capability (a potential gap).

---

## System Monitor Bar

### Health Indicators

The system bar polls Songbird `ipc.list` every second and displays a per-primal
health grid:

```
[Bio✓] [Song✓] [Nest✓] [Squi✗] [Bear✓] [Toad✓] [Barr✓] [Coral✓] [Rz✓] [Loam✓] [Swt✓] [Petal✓]
```

Each indicator:
- `✓` (green) = primal socket exists and responds to heartbeat
- `✗` (red) = primal unreachable or method errors
- `?` (yellow) = primal reachable but degraded (e.g., Squirrel with no providers)

### Session Status

Right-aligned text showing:
- Current game session ID
- Turn number
- Floor depth

---

## Provenance Sidebar

### DAG Event List

Fetches the most recent N events from the rhizoCrypt DAG session:

```
rhizoCrypt dag.merkle.root { session_id }
  -> display root hash

rhizoCrypt dag.event.list { session_id, limit: 10 }
  -> display event labels with timestamps
```

Each event entry rendered as:
```
[turn 247] game_save     a3f8c2
[turn 180] game_save     91b4e0
[turn 001] session_start d82f31
```

### Integrity Status

Shows the current merkle root and whether the chain is intact (root matches
recomputation from event hashes).

---

## biomeOS Integration

### Capability Routing

The desktop shell uses `biomeOS capability.call` for cross-primal operations:

| Operation | Capability | Method | Target |
|-----------|-----------|--------|--------|
| Health check | `composition` | `composition.nucleus_health` | Songbird |
| DAG status | `dag` | `dag.merkle.root` | rhizoCrypt |
| Game launch | `graph` | `graph.execute` | biomeOS |
| Save storage | `storage` | `storage.store` | NestGate |

### Known Gaps

- `capability.call` with `storage` routes to ToadStool instead of NestGate (GAP-13)
- `graph.start_continuous` fails for desktop graphs (GAP-15)
- `motor.panel.update` commands not routed to GUI (motor P0 bug)

Workaround: Direct IPC calls to primal sockets when biomeOS routing fails.

---

## Graph Templates

### desktop-shell graph (`desktop_shell.toml`)

Already exists at `graphs/desktop/desktop_shell.toml`. Needs:
- Health poll node (Songbird heartbeat at 1Hz)
- Provenance poll node (rhizoCrypt DAG query at 1Hz)
- Scene update node (petalTongue push at 1Hz)

### rhizome-game graph (`app_rhizome.toml`)

New graph at `graphs/desktop/app_rhizome.toml`. Needs:
- Game tick node (10Hz loop: input -> update -> render)
- World gen node (Barracuda + ludoSpring on session start)
- Save node (NestGate + provenance trio on save events)
- Narration node (Squirrel on encounter events)

---

## Experiment Structure

The desktop is implemented as `exp106_micro_desktop_shell`, a primalSpring
experiment using the `ValidationResult` framework:

1. Connect to biomeOS Neural API
2. Launch desktop-shell graph
3. Poll system health (validate Songbird discovery)
4. Launch rhizome-game graph (validate biomeOS graph execution)
5. Read provenance sidebar data (validate rhizoCrypt DAG queries)
6. Push combined scene to petalTongue (validate multi-session rendering)
7. Report gaps

This validates biomeOS orchestration, multi-session rendering, and the
composition model without requiring interactive desktop input.
