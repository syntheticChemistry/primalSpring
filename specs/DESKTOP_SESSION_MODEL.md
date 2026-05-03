# Desktop Session Model

**Status**: Design Spec (Phase 56)
**Date**: April 28, 2026
**Origin**: primalSpring Desktop NUCLEUS — application lifecycle on NUCLEUS substrate
**Scope**: Defines how applications (gardens/compositions) run as sessions on the Desktop NUCLEUS

---

## Overview

A Desktop NUCLEUS deployment provides 13 primals as a capability substrate.
**Applications** are compositions that run **on top of** the substrate — they
are deploy graphs executed by biomeOS with petalTongue providing the display
surface.

This spec defines:
- What an "application" is in the NUCLEUS model
- How sessions bind applications to petalTongue surfaces
- The lifecycle API for launching, suspending, and terminating applications
- How biomeOS manages multiple concurrent applications

---

## Core Concepts

### Application

An application is a **deploy graph** with an `[graph.metadata.application]`
section. It declares what capabilities it needs, what entry point to call,
and how it presents itself on the desktop.

```toml
[graph]
name = "esotericwebb_game"
description = "Disco Elysium-style CRPG on NUCLEUS substrate"
version = "1.0.0"
coordination = "continuous"

[graph.metadata.application]
name = "Esoteric Webb"
category = "game"
entry_capability = "game.begin_session"
display_mode = "fullscreen_session"
icon = "esotericwebb.png"
tick_hz = 60

[[graph.nodes]]
name = "game_engine"
binary = "esotericwebb"
# ...
```

Applications do **not** include NUCLEUS primals in their graphs — they
consume capabilities from the already-running substrate. The `[[graph.nodes]]`
list contains only the application's own binaries and any springs it depends
on.

### Session

A session binds an application to a petalTongue visualization surface.

```
Application (deploy graph)
    + petalTongue Session (visualization surface)
    + biomeOS ContinuousSession (tick loop, if continuous)
    = Running Desktop Application
```

Each session has:

| Property | Type | Description |
|----------|------|-------------|
| `session_id` | string | Unique identifier (UUID or slug) |
| `app_graph` | string | Name of the application's deploy graph |
| `surface_id` | string | petalTongue session ID (visualization surface) |
| `continuous_id` | string? | biomeOS `ContinuousSession` ID (if tick-based) |
| `state` | enum | `launching`, `running`, `suspended`, `terminating` |
| `created_at` | timestamp | When the session was created |

### Display Modes

| Mode | Behavior |
|------|----------|
| `panel` | Renders in a panel within the petalTongue shell window |
| `fullscreen_session` | Takes over the main rendering area |
| `floating` | Renders in a floating egui::Window |
| `background` | No visualization surface (headless service) |
| `multi_viewport` | Gets its own native window (Phase A multi-surface) |

---

## Lifecycle API

biomeOS exposes these Neural API methods for application management:

### `app.launch`

Launch an application from a deploy graph.

**Request:**
```json
{
  "method": "app.launch",
  "params": {
    "graph": "app_esotericwebb",
    "display_mode": "fullscreen_session",
    "params": {
      "world": "disco_isles",
      "save_slot": 1
    }
  }
}
```

**Behavior:**
1. Load the application graph from `graphs/desktop/` or `graphs/cells/`
2. Validate that all required capabilities are available in the running NUCLEUS
3. Deploy the graph (start application binaries if any)
4. Create a petalTongue session via `visualization.session.create`
5. If `coordination = "continuous"`, start a `ContinuousSession` at the
   specified tick rate (default 60Hz)
6. Bind the continuous session to the petalTongue session (render output
   flows to the surface, sensor input flows from it)
7. Call the `entry_capability` to start the application

**Response:**
```json
{
  "result": {
    "session_id": "ewebb-01HQ3...",
    "surface_id": "viz-ewebb-01HQ3...",
    "continuous_id": "cont-ewebb-01HQ3...",
    "state": "running"
  }
}
```

### `app.list`

List all application sessions.

**Response:**
```json
{
  "result": {
    "sessions": [
      {
        "session_id": "ewebb-01HQ3...",
        "app_name": "Esoteric Webb",
        "state": "running",
        "tick_hz": 60,
        "uptime_secs": 1234
      },
      {
        "session_id": "monitor-default",
        "app_name": "System Monitor",
        "state": "running",
        "tick_hz": 1,
        "uptime_secs": 5678
      }
    ]
  }
}
```

### `app.suspend`

Pause an application session. The continuous tick loop pauses,
petalTongue freezes the last frame, application processes remain alive.

```json
{ "method": "app.suspend", "params": { "session_id": "ewebb-01HQ3..." } }
```

### `app.resume`

Resume a suspended session.

```json
{ "method": "app.resume", "params": { "session_id": "ewebb-01HQ3..." } }
```

### `app.terminate`

Stop an application session. Flushes provenance, destroys the petalTongue
session, stops the continuous loop, and cleans up application processes.

```json
{ "method": "app.terminate", "params": { "session_id": "ewebb-01HQ3..." } }
```

### `app.info`

Get detailed information about a running session.

```json
{
  "result": {
    "session_id": "ewebb-01HQ3...",
    "app_name": "Esoteric Webb",
    "graph": "app_esotericwebb",
    "state": "running",
    "display_mode": "fullscreen_session",
    "tick_hz": 60,
    "tick_budget_ms": 16.0,
    "avg_tick_ms": 8.3,
    "surface_id": "viz-ewebb-01HQ3...",
    "capabilities_consumed": ["game.*", "ai.*", "dag.*", "visualization.*"],
    "provenance_session": "rhizocrypt-session-abc123"
  }
}
```

---

## Continuous Session Binding

For tick-based applications (games, real-time dashboards), the continuous
session creates the bridge between computation and display:

```
Each tick (e.g., 60Hz):
    1. biomeOS ContinuousExecutor ticks the graph
    2. Graph nodes execute in topological order:
       a. Input node: poll petalTongue sensor events
       b. Logic nodes: game/AI/compute processing
       c. Render node: push scene to petalTongue session
    3. petalTongue renders the scene in the next egui frame
    4. Sensor events from egui are buffered for the next tick
```

### Tick Budget

Each application declares a tick rate. biomeOS's `ContinuousExecutor`
manages the tick clock:

| Tick Rate | Budget | Use Case |
|-----------|--------|----------|
| 60 Hz | 16.6ms | Games, real-time visualization |
| 30 Hz | 33.3ms | Interactive dashboards |
| 10 Hz | 100ms | Monitoring, low-frequency updates |
| 1 Hz | 1000ms | System health, background telemetry |

If a tick exceeds its budget, the executor logs a warning and skips
frames rather than accumulating tick debt beyond 2x the budget.

---

## Desktop Shell

The desktop shell is itself an application — a special "always-on"
session that provides the chrome around other applications:

### Shell Components

| Component | IPC Source | petalTongue Rendering |
|-----------|-----------|----------------------|
| Status bar | `composition.health`, `topology.primals` | `TopBottomPanel::top` |
| App launcher | `app.list`, graph metadata | `SidePanel` or floating window |
| System tray | `lifecycle.status` per primal | Icon bar with health indicators |
| Notifications | Motor commands from biomeOS | Toast overlay |
| Session switcher | `app.list` | Tab bar or task bar |

The shell graph (`desktop_shell.toml`) runs as a background continuous
session at 1Hz tick rate, updating system status and rendering the chrome.

### Application Focus

Only one application session has "focus" at a time. The focused session:
- Receives sensor events from petalTongue
- Gets priority rendering in the central area
- Has its tick loop running at full speed

Non-focused sessions:
- Do not receive sensor events (unless subscribed to `background` events)
- Render at reduced rate or freeze their last frame
- May have their tick rate reduced to save resources

Focus is managed via:
```json
{ "method": "app.focus", "params": { "session_id": "ewebb-01HQ3..." } }
```

---

## Provenance Integration

When an application session starts, biomeOS optionally creates a
provenance chain:

1. `rhizocrypt.dag.session.create` — DAG session for the application run
2. Each significant event (scene transition, player choice, AI narration)
   is appended via `rhizocrypt.dag.event.append`
3. `loamspine.entry.append` — ledger entries for certification
4. `sweetgrass.braid.create` — attribution for multi-agent contributions
5. On termination: `loamspine.session.commit` seals the provenance chain

This enables replay, auditing, and attestation of application sessions.

---

## Application Discovery

Applications are discovered from deploy graph files with
`[graph.metadata.application]` sections. biomeOS scans:

1. `graphs/desktop/` — desktop application graphs
2. `graphs/cells/` — cellular deployment graphs with application metadata
3. `{XDG_DATA_HOME}/ecoPrimals/apps/` — user-installed application graphs

Each discovered graph is validated:
- Required capabilities are satisfiable by the running NUCLEUS
- Binary dependencies exist in `plasmidBin` or `PATH`
- The graph is structurally valid (no cycles, capabilities declared)

Invalid graphs are listed but marked as `unavailable` with a reason.

---

## Example: esotericWebb as Desktop Application

```toml
[graph]
name = "esotericwebb_game"
description = "Disco Elysium-style CRPG — DAG-traced narrative"
version = "1.0.0"
coordination = "continuous"

[graph.metadata]
cell_type = "garden"
domain = "storytelling"
security_model = "btsp_enforced"

[graph.metadata.application]
name = "Esoteric Webb"
category = "game"
entry_capability = "game.begin_session"
display_mode = "fullscreen_session"
tick_hz = 60

[graph.environment]
WEBB_CONTENT_DIR = "${ESOTERICWEBB_CONTENT:-content}"

[[graph.nodes]]
name = "esotericwebb"
binary = "esotericwebb"
order = 1
required = true
spawn = true
health_method = "health.liveness"
by_capability = "game"
capabilities = [
    "game.begin_session", "game.complete_session",
    "game.narrate_action", "game.npc_dialogue",
    "game.voice_check", "game.push_scene",
]

[[graph.nodes]]
name = "ludospring"
binary = "ludospring"
order = 2
required = false
spawn = true
health_method = "health.liveness"
by_capability = "game_science"
capabilities = [
    "game.evaluate_flow", "game.fitts_cost",
    "game.engagement", "game.difficulty_adjustment",
]
```

Launch:
```json
{ "method": "app.launch", "params": { "graph": "esotericwebb_game" } }
```

This deploys the esotericWebb binary and ludoSpring as application-level
processes. They connect to the NUCLEUS substrate's capabilities (BearDog
for signing, Songbird for discovery, NestGate for storage, Squirrel for
AI narration, petalTongue for rendering, Provenance trio for session
integrity) without those primals appearing in the application graph.

---

## Relationship to Other Specs

| Spec | Relationship |
|------|-------------|
| `DESKTOP_NUCLEUS_DEPLOYMENT.md` | Defines the substrate this model runs on |
| `LIVE_GUI_COMPOSITION_PATTERN.md` | Defines the petalTongue integration layer |
| `AGENTIC_TRIO_EVOLUTION.md` | Defines the biomeOS<->Squirrel<->petalTongue loop |
| `STORYTELLING_EVOLUTION.md` | Defines esotericWebb as the first application |
| `nucleus_desktop_cell.toml` | The NUCLEUS substrate graph |
| `BIOMEOS_NUCLEUS_EVOLUTION.md` | biomeOS substrate evolution roadmap |
