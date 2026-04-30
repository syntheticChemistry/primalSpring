# Downstream Composition Explorer Guide

**From:** primalSpring  
**For:** All spring teams  
**Date:** April 2026

## What This Is

primalSpring has built and validated a **reusable NUCLEUS composition library** that any spring can use to wire domain-specific applications through the full primal stack. The Tic-Tac-Toe composition proved the pattern end-to-end: capability discovery, BTSP-signed actions, DAG state tracking, loamSpine ledger sealing, sweetGrass braid provenance, and petalTongue interactive rendering with discrete event type isolation.

Your job is to **take this library and use it for your domain**. Each spring has a specific exploration lane (below) designed so your discoveries are complementary, not redundant.

## What You Get

### Files to pull from primalSpring

| File | What it does |
|------|-------------|
| `tools/nucleus_composition_lib.sh` | The reusable NUCLEUS composition library. Source this from your script. |
| `tools/composition_template.sh` | Minimal "hello world" composition. Copy this and fill in your domain hooks. |
| `tools/composition_nucleus.sh` | Parameterized NUCLEUS launcher. Starts primals from plasmidBin in dependency order. |
| `tools/ttt_composition.sh` | Reference implementation (Tic-Tac-Toe). Shows the full pattern in action. |
| `graphs/cells/<your_spring>_cell.toml` | Your cell graph for biomeOS deployment. |
| `graphs/downstream/downstream_manifest.toml` | Your validation capabilities list. |

### How to start

```bash
# 1. Set your composition identity
export COMPOSITION_NAME="hotspring-sim"   # your domain identifier
export FAMILY_ID="$COMPOSITION_NAME"

# 2. Launch NUCLEUS (uses plasmidBin binaries)
./tools/composition_nucleus.sh start

# 3. Verify
./tools/composition_nucleus.sh status

# 4. Copy the template and start building
cp tools/composition_template.sh tools/my_composition.sh
# Edit my_composition.sh: fill in domain_init, domain_render, domain_on_key, domain_on_click
bash tools/my_composition.sh
```

## The Library API

When you source `nucleus_composition_lib.sh`, you get:

### Discovery
- `discover_capabilities` — finds all sockets, respects REQUIRED_CAPS / OPTIONAL_CAPS
- `cap_available <name>` — returns 0 if capability is online
- `cap_socket <name>` — returns the socket path

### Transport
- `send_rpc <socket> <method> <params_json>` — JSON-RPC 2.0 over UDS
- `send_rpc_quiet <socket> <method> <params_json>` — same, discards output

### Visualization (petalTongue)
- `push_scene <session_id> <scene_json>` — render a scene graph
- `dismiss_scene <session_id>` — remove a scene
- `make_text_node <id> <x> <y> <text> <size> <r> <g> <b>` — scene node helper
- `motor_set_panel <name> <true|false>` — show/hide UI panels
- `motor_continuous <true|false>` — enable 60Hz rendering
- `motor_fit_to_view` — auto-zoom to content

### Interaction (three layers)
- `subscribe_interactions <event_type>` — L1: semantic events on scene primitives
- `poll_interaction` — poll L1 events
- `subscribe_sensor_stream` — L2: raw pointer_move, click, key_press, scroll
- `poll_sensor_stream` — poll L2 events
- `process_sensor_batch <json>` — isolate events by type, populate SENSOR_KEY, SENSOR_CLICK_CELL, SENSOR_HOVER_CHANGED
- `check_proprioception` — L3: throttled petalTongue health/fps check

### Hit Testing
Override `hit_test_fn(x, y)` to map pixel coordinates to your logical targets. Default returns -1.

### DAG (rhizoCrypt)
- `dag_create_session <domain> <genesis_metadata_json>` — create session + genesis vertex
- `dag_append_event <domain> <event_name> <state_snapshot> <metadata_json> <input_type> <hover_moves>` — append vertex
- `dag_get_children <vertex_id>` / `dag_get_frontier` — traverse
- `dag_merkle_root` — session integrity hash

### Ledger (loamSpine)
- `ledger_create_spine [name] [owner]` — create a new spine
- `ledger_append_entry <bond_id> <data_json> [committer]` — add entry
- `ledger_seal_spine` — seal the spine (immutable)

### Provenance (sweetGrass)
- `braid_init_session` — initialize session tag (auto-called by dag_create_session)
- `braid_record <event_name> <mime_type> <data_content> <custom_json> <input_type> <hover_moves>` — create braid
- `braid_query_recent [limit]` — query recent braids in session
- `braid_provenance_tree` — walk provenance graph from last braid

### Security
- `sign_payload <message>` — BTSP sign via beardog

### Lifecycle
- `composition_startup <title> <subtitle>` — startup splash with capability check
- `composition_summary` — print summary (DAG, braids, primals)
- `composition_teardown <scene_ids...>` — clean up subscriptions and scenes

## Per-Spring Exploration Lanes

Each spring explores a **discrete, complementary** aspect of the composition pattern. This is convergent evolution: you will each discover different things, and when you hand back, we absorb the cross-domain patterns into the library.

### ludoSpring — Interaction Fidelity and Real-Time Feedback

Your domain naturally exercises the tightest human feedback loop. You are the primary explorer of:

- **Multi-player input routing**: Can you split mouse and keyboard between two players? How does the sensor stream need to evolve for this? What does petalTongue need?
- **Continuous rendering**: When does 60Hz matter vs event-driven rendering? What tick rates work for different game modes?
- **DAG branching**: Game state time-travel, undo/redo trees, branching narratives — push rhizoCrypt hard.
- **Braid replay integrity**: Can you reconstruct a game from its braid chain? What metadata is missing?
- **petalTongue stress testing**: Complex scenes, fast updates, many interactive primitives.

**Your artifact**: interaction patterns, input device management patterns, petalTongue performance findings.

### hotSpring — Event-Driven Computation and DAG Memoization

Your domain exercises long-running compute with convergence-based (not time-based) progression. You explore:

- **Async tick models**: Your simulations converge, they don't tick at 60Hz. How should the main loop work? Event-driven? Polling with exponential backoff?
- **DAG for parameter sweeps**: Each lattice configuration is a vertex. Branching = different coupling constants. Memoization = don't recompute vertices you've visited.
- **Ledger sealing for reproducibility**: Each simulation run is a sealed spine. Can you reproduce results from the ledger alone?
- **Scientific provenance**: Braid chains that satisfy peer review audit requirements. What metadata does a physicist need?
- **Compute dispatch**: Push toadStool and barraCuda with real tensor workloads.

**Your artifact**: async computation patterns, DAG memoization patterns, scientific provenance schema.

### wetSpring — Data Exploration and Visualization

Your domain exercises large data navigation and rich visualization. You explore:

- **Genome/protein visualization**: Scene graphs for molecular structures. How complex can petalTongue scenes get? What primitives are missing?
- **DAG-linked data navigation**: Each gene/protein as a vertex, relationships as edges. Navigating a biological graph via DAG.
- **Large dataset handling**: If you need sequences from nestgate storage, how does storage integrate with the composition? What's the IPC pattern for streaming large data?
- **Braid lineage**: Data provenance — where did this sequence come from? What transformations were applied?

**Your artifact**: data visualization patterns, large-state DAG navigation, storage integration patterns.

### neuralSpring — Agent-Driven Composition and AI Feedback Loops

Your domain exercises AI-mediated interactions. You explore:

- **Squirrel-mediated composition**: Can Squirrel call NUCLEUS methods directly to compose a workflow? What's the agentic IPC pattern?
- **Inference pipeline**: Embedding, completion, reasoning through the primal stack. How does inference fit into DAG state?
- **Model decision audit**: Braid provenance for AI decisions. Can you trace why the model chose a particular path?
- **Agent feedback loops**: The AI acts, observes the result via proprioception/braids, and adjusts. What's the composition pattern for closed-loop agent behavior?

**Your artifact**: agentic composition patterns, inference integration, AI provenance schema.

### Other Springs (airSpring, groundSpring, healthSpring)

Start with the template. Focus on getting your NUCLEUS deployed, your validation capabilities exercised, and your first scene rendered. Document any primal behavior that differs from what primalSpring documented. Your exploration lane will crystallize as you engage with the system.

## What You Hand Back

When you've built your first composition, document and hand back:

1. **Discovered patterns**: What tick rates, interaction models, and visualization approaches worked for your domain?
2. **Primal gaps**: Any IPC method that misbehaves, returns unexpected formats, or is missing — add to your local `PRIMAL_GAPS.md` and hand back to primalSpring.
3. **Domain hooks that worked well**: If you built a reusable pattern (e.g., "DAG memoization wrapper" or "streaming data scene builder"), it's a candidate for promotion into the library.
4. **Primal behavior differences**: Anything that worked differently from what primalSpring documented (timeouts, error formats, BTSP behavior).

Hand back via your wateringHole handoff docs. primalSpring will absorb, abstract, and push refined patterns upstream to primal teams.

## Known Gaps (So You Don't Hit Them)

| Gap | What | Status |
|-----|------|--------|
| PG-45/52 | rhizoCrypt UDS empty responses | **RESOLVED** — rebuilt binary + FAMILY_SEED env var |
| PG-46 | toadStool slow on short timeouts | Use >=10s timeout (lib defaults to 5s) |
| PG-47 | barraCuda missing `stats.entropy` | Skip or compute locally |
| PG-48 | petalTongue musl + winit threading | **ADDRESSED** — `any_thread` fix in rebuilt binary |
| PG-51 | Songbird crypto provider discovery | **RESOLVED** — family-scoped BearDog fallback chain |
| PG-53 | petalTongue proprioception in server mode | **RESOLVED** — new handler returns complete JSON |
| PG-39 | Graph schema mismatch (primalSpring vs biomeOS) | Use shell compositions for now; graph alignment is upstream |

## Key References

- Composition guidance: `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md`
- Guidestone standard: `wateringHole/GUIDESTONE_COMPOSITION_STANDARD.md`
- PlasmidBin depot: `wateringHole/PLASMINBIN_DEPOT_PATTERN.md`
- Gap registry: `docs/PRIMAL_GAPS.md`
- Downstream manifest: `graphs/downstream/downstream_manifest.toml`
