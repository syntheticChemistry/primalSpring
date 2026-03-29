# Agentic Trio Evolution — biomeOS, petalTongue, Squirrel

**Status**: EVOLUTION GUIDANCE  
**Date**: March 28, 2026  
**Source**: primalSpring substrate validation, leverage guides, experiment gaps  
**Scope**: Specific evolution recommendations to make biomeOS, petalTongue, and Squirrel more leverageable as the agentic coordination layer for the ecoPrimals ecosystem

---

## The Three Layers

| Layer | Primal | Metaphor | Role |
|-------|--------|----------|------|
| **Nervous system** | biomeOS | Routes signals, executes graphs | Substrate orchestrator, Neural API, capability routing, lifecycle |
| **Brain** | Squirrel | Makes decisions, remembers context | AI inference, MCP tool hub, context management, peer discovery |
| **Senses** | petalTongue | Observes + acts | UI (TUI/egui/web/headless), SSE events, dashboards, human intent |

The loop: **petalTongue observes → human/agent decides → Squirrel acts → biomeOS routes → springs execute → petalTongue renders result**

---

## biomeOS Evolution Guidance

### Current State (v2.70+)

- 285+ semantic methods across 26 domains
- 5 graph coordination patterns (Sequential, Parallel, ConditionalDag, Pipeline, Continuous)
- Neural API routing via `capability.call`, `capability.discover`
- `TransportEndpoint` abstraction (UDS, TCP, HTTP, filesystem)
- Chimera codegen (stub → capability-based IPC forwarding)

### P0: TCP-Only API Mode

**Gap**: `biomeos neural-api --port <port>` is documented but the `api` mode ignores `--port` and forces UDS. This blocks all TCP-first cells in the deployment matrix and prevents mobile/Pixel deployment.

**Evolution**: 
- Honor `--port` flag: bind TCP listener alongside (or instead of) UDS
- Respect `BIOMEOS_TCP_ONLY=true` env var from deployment matrix
- When `PRIMAL_TRANSPORT=tcp`, child primal orchestration must use TCP endpoints

**Validation**: cells `tower-x86-homelan-tcp`, `tower-aarch64-homelan-tcp` will move from `blocked` to `untested`

### P0: Cross-Gate Routing (`gate` parameter)

**Gap**: `capability.call` does not honor a `gate` parameter for cross-gate routing. This blocks federation cells where capabilities live on remote gates.

**Evolution**:
- `capability.call({ capability, operation, args, gate })` → look up gate endpoint via Songbird mesh → forward request
- Document `route.register` as the federation pattern for inter-gate capability advertisement

**Validation**: cells `federation-x86-mixed-uds`, `wan-x86-satellite-uds`

### P1: Standalone Lab Mode

**Gap**: `biomeos neural-api` exits without `biome.yaml` or graphs — prevents use in benchScale Docker labs where we just need health and routing.

**Evolution**:
- `biomeos neural-api --standalone` or `--health-only` flag
- Starts with empty graph list, routes `capability.call` to discovered primals, responds to `health.liveness`
- No `biome.yaml` required

**Validation**: all benchScale topologies that include biomeOS

### P1: Health Method Alignment

**Gap**: `health.check` returns "Method not found" on Neural API. Clients expect `health.check` or `health.liveness` but biomeOS only responds to one.

**Evolution**:
- Register both `health.check` and `health.liveness` as aliases
- Return consistent `{ status: "alive" }` JSON for both

### P2: Abstract Socket Routing

**Gap**: biomeOS routes to filesystem sockets (`/tmp/biomeos/*.sock`). Squirrel listens on abstract socket `@squirrel`. Forward fails silently.

**Evolution**:
- `TransportEndpoint` should handle `abstract://squirrel` alongside `unix:///path/to/sock`
- Socket discovery should probe abstract namespace when filesystem socket not found
- Or: Squirrel should prefer filesystem sockets when `XDG_RUNTIME_DIR` is set

**Validation**: exp077 `squirrel_neural_api_bridge` will stop skipping

---

## Squirrel Evolution Guidance

### Current State (v0.1.0-alpha.25)

- 452k LOC, 22 workspace crates, 6,839 tests, 86.5% coverage
- AI inference: `ai.query`, `ai.complete`, `ai.chat` with vendor-agnostic routing
- MCP: `tool.execute`, `tool.list` with JSON Schema
- Context: `context.create`, `context.update`, `context.summarize` (DashMap, NestGate persistence)
- Peer discovery: `discovery.peers`, `capabilities.list`
- DignityEvaluator on all AI operations

### P0: Socket Transport Alignment

**Gap**: Squirrel defaults to abstract UDS `@squirrel` which biomeOS cannot route to.

**Evolution**:
- Default to filesystem socket at `$XDG_RUNTIME_DIR/ecoPrimals/squirrel.sock` (matching biomeOS discovery)
- Fall back to abstract socket only when env `SQUIRREL_ABSTRACT_SOCKET=true`
- Register `socket_env_key = "SQUIRREL_SOCKET"` in ecosystem manifest

**Validation**: exp077 passes without skip, exp061/exp070 routing through biomeOS works

### P1: Capability String Canonicalization

**Gap**: Graph TOMLs use inconsistent names: `ai.coordinate` vs `ai.query` vs `ai.execute_tool` vs `tool.execute`. This creates drift between declared graph capabilities and actual RPC methods.

**Evolution**:
- Canonical methods per leverage guide: `ai.query`, `ai.complete`, `ai.chat`, `tool.execute`, `tool.list`, `context.*`, `discovery.peers`
- Update all graph TOMLs to use canonical names
- `capability_registry.toml` should register the full surface (not just `ai.query` and `ai.health`)
- Graph `by_capability` labels can differ from RPC methods, but documentation must map them

### P1: MCP Tool Ecosystem Expansion

**Gap**: `tool.list` returns primalSpring's 8 tools, but ToadStool, barraCuda, NestGate don't announce tools to Squirrel yet.

**Evolution**:
- ToadStool: announce `compute.submit`, `compute.status` as MCP tools
- NestGate: announce `storage.store`, `storage.retrieve` as MCP tools
- Each primal that exposes `capability.announce` should surface its methods as MCP tools
- Squirrel aggregates: local tools + announced remote tools → unified catalog

**Validation**: exp044 `mcp.tools.list` returns > 8 tools when springs are live

### P2: Agentic Graph Integration

**Gap**: Squirrel can route AI queries but doesn't participate in biomeOS graph execution feedback loops.

**Evolution**:
- `graph.suggest_optimizations` → Squirrel analyzes graph execution telemetry
- `ai.evaluate_composition` → Squirrel assesses whether a primal composition is healthy/optimal
- Feed petalTongue composition dashboards with Squirrel's assessment

---

## petalTongue Evolution Guidance

### Current State (v1.6.6)

- Grammar of Graphics, declarative scene graph, dashboard layout engine
- Multi-modal: egui, TUI (ratatui), web (axum), headless, server (JSON-RPC IPC)
- SSE client for biomeOS ecosystem events
- Songbird discovery and topology visualization wired
- Squirrel adapter in frame loop

### P0: biomeOS SSE Robustness

**Gap**: SSE client exists (`sse.rs`) but validation only runs when biomeOS is live. No reconnection logic or graceful degradation documented.

**Evolution**:
- SSE client with exponential backoff reconnection
- Fallback to polling `health.liveness` when SSE unavailable
- Cache last-known ecosystem state for offline rendering

**Validation**: exp078 with intermittent biomeOS availability

### P1: ToadStool Frame Path

**Gap**: Window lifecycle wired but `display.present` / frame submission not connected.

**Evolution**:
- Complete the `display.present` → ToadStool GPU render → petalTongue frame path
- This enables real-time GPU-rendered visualizations (not just Grammar of Graphics SVG)
- Required for gaming mesh chimera (60Hz tick budget)

**Validation**: gaming-x86-localhost-uds cell, exp065 with ToadStool active

### P1: Provenance Trio Integration

**Gap**: Clients wired but `sweetGrass.provenance.create_braid`, `LoamSpine.commit.session`, `rhizoCrypt.dag.create_session` not yet invoked in documented end-to-end flow.

**Evolution**:
- petalTongue should stamp rendered artifacts with provenance
- Export SVG/PNG with embedded Merkle root (rhizoCrypt session)
- Braid attribution for multi-agent dashboard contributions
- Commit rendered reports to LoamSpine ledger

**Validation**: nucleus-x86-basement-provenance cell with petalTongue active

### P2: Defensive Visualization (skunkBat)

**Gap**: skunkBat not wired to petalTongue.

**Evolution**:
- skunkBat `defense.*` events → petalTongue threat dashboard
- Topology view with color-coded trust boundaries (covalent=green, ionic=yellow, weak=red, hostile=red-flash)
- Real-time violation alerts in TUI and egui modes

**Validation**: skunkbat-x86-homelan-uds cell

### P2: fieldMouse Sensor Dashboards

**Gap**: petalTongue can render dashboards but no fieldMouse-specific templates exist.

**Evolution**:
- `visualization.render.dashboard` template for fieldMouse frame streams
- Time-series sensor data (pH, moisture, temp) with domain-aware palettes
- Anomaly highlighting based on Squirrel classification
- Works in TUI mode for headless field deployments

**Validation**: fieldmouse-x86-homelan-uds, agentic-fieldmouse cells

---

## Cross-Cutting: The Agentic Loop

### What Works Today

1. biomeOS discovers primals and routes `capability.call` to them
2. Squirrel responds to `ai.query` via biomeOS Neural API (exp061)
3. petalTongue renders dashboards and topology (exp043, exp065, exp078)
4. primalSpring exposes MCP tools to Squirrel (exp044)

### What Doesn't Work Yet

1. **biomeOS → Squirrel routing fails** due to abstract socket mismatch (exp077)
2. **petalTongue → Squirrel intent feedback** not validated (no experiment)
3. **Squirrel → petalTongue render request** not validated (no experiment)
4. **Full loop**: human clicks petalTongue → biomeOS routes intent → Squirrel decides → biomeOS executes graph → springs act → petalTongue renders — **no end-to-end experiment**

### Recommended New Experiments

| ID | Name | What It Validates |
|----|------|-------------------|
| exp085 | `agentic_loop_substrate` | Full three-way loop: petalTongue → biomeOS → Squirrel → biomeOS → springs → petalTongue |
| exp086 | `mcp_ecosystem_tools` | Squirrel `tool.list` with multiple springs announcing tools |
| exp087 | `fieldmouse_ai_triage` | fieldMouse frame → NestGate → Squirrel classify → petalTongue alert |

### Deployment Matrix Cells

These cells validate the agentic trio in different substrate conditions:

| Cell | Topology | What It Tests |
|------|----------|---------------|
| `agentic-x86-homelan-uds` | agentic_tower | Full loop on home network |
| `agentic-x86-basement-uds` | agentic_tower | Full loop on HPC (lowest latency) |
| `agentic-x86-homelan-tcp` | agentic_tower | TCP-only (biomeOS --port gap exposed) |
| `agentic-fm-x86-homelan-uds` | agentic_fieldmouse | AI-guided sensor orchestration |

---

## Priority Summary

| Priority | biomeOS | Squirrel | petalTongue |
|----------|---------|----------|-------------|
| **P0** | TCP-only `--port`, cross-gate `gate` param | Socket transport alignment | SSE reconnection robustness |
| **P1** | Standalone lab mode, health method aliases | Capability canonicalization, MCP tool expansion | ToadStool frame path, Provenance trio |
| **P2** | Abstract socket routing | Agentic graph integration | skunkBat defense viz, fieldMouse dashboards |
