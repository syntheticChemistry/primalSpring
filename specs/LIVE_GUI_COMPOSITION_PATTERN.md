# Live GUI Composition Pattern

**Status**: Design Spec (Phase 56)
**Date**: April 28, 2026
**Origin**: primalSpring Desktop NUCLEUS — petalTongue `live` mode
**Scope**: How petalTongue's egui desktop connects to the running NUCLEUS via IPC

---

## Overview

petalTongue's `live` mode merges a native egui/eframe desktop window with
a JSON-RPC IPC server in a single process. This is the canonical pattern
for Desktop NUCLEUS deployments — the NUCLEUS runs as a background
composition of 12 primals, and petalTongue `live` provides the human
interface.

This spec documents how the live GUI integrates with the NUCLEUS
substrate: shared state, motor commands, sensor events, and session
management.

---

## Architecture

```
┌─────────────────── petalTongue live ───────────────────┐
│                                                        │
│  Main thread (winit/egui)         Tokio runtime        │
│  ┌──────────────────────┐    ┌────────────────────┐   │
│  │  PetalTongueApp      │    │  IPC Server         │   │
│  │  (eframe::App)       │    │  (JSON-RPC/UDS)     │   │
│  │                      │    │                     │   │
│  │  panels / renderer   │    │  visualization.*    │   │
│  │  scene graph tick    │◀──▶│  motor.*            │   │
│  │  sensor collection   │    │  sensor.*           │   │
│  │  motor drain         │    │  interaction.*      │   │
│  └──────────────────────┘    │  proprioception.*   │   │
│           │                  └─────────┬───────────┘   │
│           │  Arc<RwLock<..>>           │               │
│           └───────────┬────────────────┘               │
│                       │                                │
│  ┌────────────────────┴───────────────────────────┐   │
│  │            Shared State                         │   │
│  │  VisualizationState: sessions, grammar_scenes   │   │
│  │  SensorStreamRegistry: sensor event broadcast   │   │
│  │  MotorChannel: mpsc sender/receiver             │   │
│  └─────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────┘
         │ UDS sockets                    │
    ┌────┴──────────────┐    ┌───────────┴─────────────┐
    │  NUCLEUS primals  │    │  biomeOS Neural API     │
    │  (11 processes)   │    │  (capability routing)   │
    └───────────────────┘    └─────────────────────────┘
```

---

## Shared State Model

### VisualizationState

The central bridge between IPC and the GUI. Wrapped in
`Arc<RwLock<VisualizationState>>` and passed to both the IPC server
and `PetalTongueApp`.

```rust
pub struct VisualizationState {
    pub sessions: HashMap<String, RenderSession>,
    pub grammar_scenes: HashMap<String, SceneGraph>,
    pub active_session: Option<String>,
    // ...
}
```

**IPC writes, GUI reads.** The IPC server updates `VisualizationState`
when external callers push visualization data via `visualization.render.*`
or `visualization.set_state`. The GUI reads the state every frame in
`PetalTongueApp::update()` to render the latest content.

### Sessions as Multi-Application Viewports

Each `RenderSession` represents an independent visualization stream — a
running application, a dashboard, or a sensor feed. Sessions are keyed by
string ID (e.g., `"esotericwebb-game-0"`, `"system-monitor"`,
`"fieldmouse-sensor-3"`).

The IPC contract for session management:

| Method | Direction | Effect |
|--------|-----------|--------|
| `visualization.session.create` | External -> petalTongue | Create new session, returns session ID |
| `visualization.session.destroy` | External -> petalTongue | Remove session |
| `visualization.session.list` | External -> petalTongue | List active sessions |
| `visualization.render.scene` | External -> petalTongue | Push scene graph to a session |
| `visualization.render.dashboard` | External -> petalTongue | Push dashboard layout to a session |
| `visualization.render.grammar` | External -> petalTongue | Push Grammar of Graphics spec |
| `visualization.render.stream` | External -> petalTongue | Push continuous render data |
| `visualization.export` | External -> petalTongue | Export session to SVG/PNG/JSON |

---

## Motor Commands (Efferent)

Motor commands flow **from the NUCLEUS into petalTongue** to control the
GUI: create panels, update layouts, navigate, trigger animations.

### Current Implementation (Known Gap)

In `live_mode.rs`, the IPC server receives `motor.*` commands via a
`motor_tx` channel. However, the **receiver** is a background thread
that only **logs** commands — it is **not** connected to the
`PetalTongueApp`'s `motor_rx` that drives the actual GUI.

```
IPC motor.* handler
    -> motor_tx (server)
    -> motor_rx (logging thread)   <-- NOT the GUI
    
PetalTongueApp
    -> motor_tx (app-internal)
    -> motor_rx (drain_motor_commands)  <-- this is the GUI
```

### Target State

The server's `motor_tx` and the app's `motor_rx` must be the **same
channel**. The fix is to pass the app's `motor_tx` into the server via
`with_motor_sender()` and remove the separate logging-only receiver:

```
IPC motor.* handler
    -> motor_tx (shared)
    -> motor_rx (PetalTongueApp::drain_motor_commands)  <-- GUI receives
```

### Motor Command Types

| Method | Params | GUI Effect |
|--------|--------|------------|
| `motor.panel.create` | `{id, title, position, size}` | Create floating panel |
| `motor.panel.update` | `{id, content}` | Update panel content |
| `motor.panel.close` | `{id}` | Close panel |
| `motor.navigate` | `{target}` | Switch active view/session |
| `motor.notification` | `{level, message, duration}` | Show toast notification |
| `motor.theme` | `{name}` | Switch visual theme |

---

## Sensor Events (Afferent)

Sensor events flow **from petalTongue into the NUCLEUS** — human input
captured by egui is broadcast to interested consumers via IPC.

### Collection Pipeline

1. egui captures raw input events (mouse, keyboard, touch, scroll)
2. `collect_sensor_events()` in `PetalTongueApp::update()` processes
   raw input into semantic `SensorEvent` types
3. Events are pushed to the `SensorStreamRegistry` (shared state)
4. IPC consumers that called `interaction.sensor_stream.subscribe`
   receive events on their next `interaction.sensor_stream.poll`

### Sensor Event Types

| Event | Fields | Use Case |
|-------|--------|----------|
| `pointer.move` | `{x, y, session}` | Cursor tracking |
| `pointer.click` | `{x, y, button, session}` | Selection, interaction |
| `key.press` | `{key, modifiers}` | Keyboard input |
| `key.release` | `{key}` | Key up events |
| `scroll` | `{delta_x, delta_y}` | Scroll input |
| `text.input` | `{text}` | Text entry |
| `window.resize` | `{width, height}` | Layout changes |
| `window.focus` | `{focused}` | Focus state |
| `intent` | `{action, context}` | High-level user intent |

### Intent Events

The `intent` event type is a higher-level abstraction. petalTongue
recognizes interaction patterns (click a dialogue option, select an
ability, drag a file) and emits semantic intent events that Squirrel
or biomeOS can consume without understanding raw input:

```json
{
  "type": "intent",
  "action": "dialogue.choose",
  "context": {
    "session": "esotericwebb-game-0",
    "npc": "innkeeper",
    "option_id": 2,
    "option_text": "Tell me about the mountains."
  }
}
```

This is the afferent half of the agentic loop — human decisions flow
from petalTongue through biomeOS to Squirrel for AI processing.

---

## Integration with biomeOS

### Capability Registration

petalTongue registers with Songbird (via `DISCOVERY_SOCKET` or biomeOS
registration loop) with capabilities:

```json
["visualization", "motor", "sensor", "interaction", "modality", "audio"]
```

### biomeOS Graph Interaction

biomeOS can push visualization updates to petalTongue via
`capability.call`:

```json
{
  "capability": "visualization",
  "operation": "render.scene",
  "args": { "session": "monitor-0", "scene": {...} }
}
```

### Continuous Session Binding

When biomeOS runs a `ContinuousSession` graph (e.g., a 60Hz game loop),
the graph can include petalTongue as a render node. Each tick of the
continuous session may:

1. Execute compute/AI nodes (game logic, inference)
2. Push render data to petalTongue via `visualization.render.scene`
3. Poll sensor events from petalTongue via `interaction.sensor_stream.poll`

This creates the **real-time feedback loop** between the NUCLEUS substrate
and the desktop surface.

---

## Integration with Squirrel

### Sensor -> Squirrel

petalTongue's `intent` events are routed through biomeOS to Squirrel
for AI decision-making:

```
petalTongue intent event
    -> biomeOS capability.call("ai", "evaluate_intent", {intent})
    -> Squirrel processes intent, returns action
    -> biomeOS routes action to target primal
    -> Result flows back to petalTongue via visualization update
```

### Squirrel -> petalTongue

Squirrel can render AI-generated content by calling petalTongue's motor
commands through biomeOS:

```
Squirrel generates narrative
    -> biomeOS capability.call("visualization", "render.scene", {scene})
    -> petalTongue displays scene in the active session
```

---

## Startup Sequence

1. NUCLEUS launches 11 primals (Tower -> Nest -> Node -> Provenance)
2. petalTongue `live` starts as the final primal:
   a. Tokio runtime spawns IPC server on `petaltongue-{family}.sock`
   b. Shared state initialized (VisualizationState, SensorStreamRegistry)
   c. egui/eframe window opens on main thread (1400x900, min 800x600)
   d. IPC server registers with Songbird via `DISCOVERY_SOCKET`
3. biomeOS detects petalTongue via health check or Songbird discovery
4. The desktop is ready — biomeOS can push visualizations, applications
   can create sessions, and sensor events flow back through the loop

---

## Relationship to Multi-Surface Evolution

This spec documents the **single-window** `live` mode as it exists today.
The multi-surface evolution (see `DESKTOP_SESSION_MODEL.md`) extends this
pattern:

- **Phase A**: Multiple egui viewports (`ViewportId` per session)
- **Phase B**: `DisplayManager` routes sessions to viewports
- **Phase C**: ToadStool `display.*` IPC for compositor-level surfaces

The session model and IPC contract defined here remain valid for all phases.
Sessions are already keyed by ID — the only change is whether they render
into panels within one window or into independent native windows.

---

## Non-Goals

- Window manager / compositor semantics (see ToadStool `display.*`)
- Multi-monitor support (egui defers to the platform's display server)
- Audio routing details (delegated to petalTongue's audio subsystem)
- Scene graph compilation internals (see `petal-tongue-scene` crate docs)
