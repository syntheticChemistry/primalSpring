# Live Desktop NUCLEUS — Deployment Gap Report (Phase 56)

**Date**: April 29, 2026 (refreshed — local debt pass)
**Deployment**: `desktop_nucleus.sh start` + biomeOS `neural-api` (family=desktop-nucleus)
**Primals deployed**: 12 spawned + biomeOS Neural API coordinator
**Health**: 10/10 JSON-RPC primals healthy (coralReef excluded — tarpc)
**biomeOS**: Online — 605 registered capabilities, 3078 auto-discovered from 36 sockets

---

## Critical Gaps (P0)

### GAP-01: petalTongue discovery heartbeat targets wrong socket [MITIGATED]

**Severity**: P0 → P1 (mitigated by symlink)
**Status**: Running stable after symlink fix

petalTongue hardcodes its heartbeat target to `/run/user/1000/biomeos/discovery-service.sock`.
The composition launcher provides `DISCOVERY_SOCKET=$SOCKET_DIR/songbird-{family}.sock`,
but petalTongue's `primal_registration` module ignores the env var and uses its
own compiled-in path.

**Mitigation applied**: Symlink `discovery-service.sock → songbird-desktop-nucleus.sock`.
After symlink, petalTongue runs stably at 60 FPS with no heartbeat failures.

**Root cause**: petalTongue's `primal_registration.rs:255` hardcodes the discovery
socket path instead of reading `DISCOVERY_SOCKET` from the environment.

**Fix needed (petalTongue team)**:
1. Read `DISCOVERY_SOCKET` env var (as all other primals do)
2. Fall back to `discovery-service.sock` only if env unset
3. Add exponential backoff on heartbeat failure (currently fails fast)
4. `desktop_nucleus.sh` should create the symlink automatically

### GAP-02: biomeOS Neural API now running [RESOLVED]

**Severity**: P0 → **RESOLVED** (manually started alongside composition)
**Status**: Online at `/run/user/1000/biomeos/neural-api-desktop-nucleus.sock`

biomeOS `neural-api` launched with `--graphs-dir` pointing at primalSpring graphs,
auto-discovered 3,078 capabilities from 36 sockets (primals + symlinks).

**Working Neural API features**:
- `capability.call` — semantic routing to primals (BearDog, Barracuda, rhizoCrypt, sweetGrass, Songbird)
- `graph.save` / `graph.list` — 8 graphs loaded (6 federation + 2 desktop)
- `graph.execute` — federation graph executed successfully (6 phases, 6ms)
- `graph.status` — execution tracking with phase/node completion
- `health.check` — shows 605 capabilities, coordinated mode
- `capability.list` — full route table with 25+ storage methods, provenance, etc.

**Remaining sub-gaps**:
- `graph.start_continuous` fails due to parser mismatch (DeploymentGraph vs ContinuousGraph)
- `graph.execute` for desktop graphs needs operation-to-capability dispatch (nodes execute as "unknown type")
- `app.*` methods not yet implemented
- `storage` domain routes to `compute` (ToadStool) not NestGate — capability registry bug
- biomeOS not auto-started by `desktop_nucleus.sh` — needs manual launch or script update

**Remaining fix (biomeOS team)**: `NucleusMode::Desktop` for native 12-primal launch,
consistent graph parser, `capability.call` dispatch in continuous nodes.

### GAP-03: Squirrel HTTP provider routing NOT implemented

**Severity**: P0 — AI narration, agentic loop, and chat are non-functional
**Status**: Provider registration succeeds but inference routing fails

Squirrel's `inference.register_provider` accepts HTTP URLs (e.g.
`http://127.0.0.1:11434`) but internally stores them with `transport="UDS"`.
When inference is requested, it tries to connect via Unix domain socket
to the HTTP URL string, which fails immediately.

The startup sequence:
1. `LOCAL_AI_ENDPOINT` is resolved and logged but **not auto-registered**
2. `AI_HTTP_PROVIDERS` env var is checked but **not consumed** (log: "No HTTP providers enabled")
3. `AI_PROVIDER_SOCKETS=""` causes a connect-to-empty-string crash
4. Runtime `inference.register_provider` registers metadata but routes as UDS

```
inference.register_provider — registered provider="ollama-local" socket=Some("http://127.0.0.1:11434") endpoint=None transport="UDS"
```

**Impact**: All AI-dependent features are inoperable even when Ollama is
running and reachable at `http://127.0.0.1:11434`.

**Fix needed (Squirrel team)**:
1. Auto-register `LOCAL_AI_ENDPOINT` / `OLLAMA_ENDPOINT` at startup
2. Implement HTTP transport in inference router (detect `http://` prefix)
3. Don't crash on empty `AI_PROVIDER_SOCKETS` string
4. `AI_HTTP_PROVIDERS` env should actually enable HTTP providers

---

## Known / Cosmetic Gaps (Expected)

### GAP-04: coralReef uses tarpc, not JSON-RPC

**Severity**: Cosmetic (known since Phase 55)
**Status**: PID alive, socket exists — no JSON-RPC health probe possible

coralReef uses tarpc binary protocol. Health checks via socat return empty
because socat sends JSON-RPC text over a tarpc-framed socket.

**Fix needed**: `desktop_nucleus.sh validate` should skip JSON-RPC health
checks for coralReef or add a tarpc-aware health probe.

### GAP-05: ToadStool registers method-level capabilities

**Severity**: Cosmetic (known since Phase 55)
**Status**: Registers `compute.dispatch` and `compute.capabilities`, not `compute`

ToadStool self-registers with Songbird using method-level capability names.
Domain-level `compute` resolution fails, but `compute.dispatch` works.

### GAP-06: Squirrel discovery.register rejected by Songbird

**Severity**: P2 — Falls back to standalone mode
**Status**: `"discovery.register rejected by discovery service: unknown JSON-RPC method"`

Squirrel tries `discovery.register` but Songbird expects `ipc.register`.
Squirrel falls back to standalone mode. The composition launcher's separate
registration step (`ipc.register`) succeeds, so this is only a first-party gap.

---

## biomeOS Neural API — Semantic Translation Status

### capability.call Routing Results

| Capability | Target Primal | Status | Notes |
|-----------|---------------|--------|-------|
| `crypto.blake3_hash` | BearDog | **PASS** | Returns BLAKE3 hash via semantic route |
| `dag.session.create` | rhizoCrypt | **PASS** | Must use `capability=dag, operation=dag.session.create` |
| `stats.mean` | Barracuda | **PASS** | `capability=stats, operation=stats.mean` |
| `ipc.list` | Songbird | **PASS** | 10 primals in service mesh |
| `braid.create` | sweetGrass | **PASS** | W3C PROV-O braid returned |
| `spectral.fft` | Barracuda | FAIL | Routes correctly but strips prefix → `fft` not found |
| `storage.store` | NestGate | FAIL | **Routes to ToadStool** (storage→compute registry bug) |
| `composition.nucleus_health` | sweetGrass | FAIL | Strips to `composition.health` (method name mismatch) |

**Routing pattern**: biomeOS uses `capability` to find the primal socket, then
forwards `operation` as the JSON-RPC method. The `operation` parameter MUST contain
the full dotted method name (e.g. `dag.session.create`, not just `session.create`).

### Graph Execution Results

| Method | Status | Notes |
|--------|--------|-------|
| `graph.list` | **PASS** | 8 graphs (6 federation + 2 desktop) |
| `graph.save` | **PASS** | Runtime injection works, persists to `runtime_graphs/` |
| `graph.get` | FAIL | Not fully implemented for runtime graphs |
| `graph.execute` | **PASS** | Federation graph: 6 phases, 6 nodes, 6ms. Nodes skip as "unknown type" |
| `graph.status` | **PASS** | Returns completed_nodes, duration, state |
| `graph.start_continuous` | FAIL | DeploymentGraph parser doesn't match continuous format |
| `graph.protocol_map` | **PASS** | Returns empty (no active protocols yet) |

### Graph Parser Inconsistency (P1)

biomeOS has three distinct graph parsers:
1. **Neural API parser** (`graph.save`/`graph.list`): Accepts both `id`+`name` with `by_capability`
2. **DeploymentGraph parser** (`graph.execute`): Requires `id` on nodes + `[graph.nodes.operation]` tables
3. **ContinuousGraph parser** (`biomeos continuous`): Requires `name` + lowercase `coordination = "continuous"`

Desktop graphs must include both `id` and `name` on every node and use lowercase
`coordination = "continuous"` to pass all three parsers.

---

## IPC Parameter Documentation Gaps (P1)

### GAP-07: rhizoCrypt `dag.event.append` — Custom needs `event_name` + `domain`

**Resolved schema**:
```json
{
  "session_id": "<uuid from dag.session.create>",
  "event_type": {
    "AgentAction": {"agent_id": "...", "action": "..."}
  },
  "data": {"key": "value"}
}
```

For `Custom` variant, all three fields required:
```json
{"Custom": {"label": "...", "event_name": "...", "domain": "..."}}
```

Valid event types: `SessionStart`, `SessionEnd`, `AgentJoin`, `AgentLeave`,
`AgentAction`, `DataCreate`, `DataModify`, `DataDelete`, `DataTransfer`,
`SliceCheckout`, `SliceOperation`, `SliceResolve`, `GameEvent`, `Custom`.

### GAP-08: loamSpine `entry.append` — payload INSIDE Custom variant + committer required

**Resolved schema**:
```json
{
  "spine_id": "<uuid from spine.create>",
  "committer": "agent-name",
  "entry_type": {
    "Custom": {
      "label": "...",
      "type_uri": "urn:eco:...",
      "domain": "...",
      "payload": [104, 101, 108, 108, 111]
    }
  }
}
```

Key findings:
- `payload` is a **byte array** (sequence of u8), NOT a JSON object
- `payload` goes **inside** the `entry_type` variant, NOT at the top level
- `committer` is required at the top level
- Entries are Tower-signed automatically (`tower_signature` in metadata)

Valid entry types: `Genesis`, `MetadataUpdate`, `SpineSealed`, `SessionCommit`,
`SliceCheckout`, `SliceReturn`, `DataAnchor`, `BraidCommit`, `CertificateMint`,
`CertificateTransfer`, `CertificateLoan`, `CertificateReturn`, `SliceAnchor`,
`SliceOperation`, `SliceDeparture`, `TemporalMoment`, `PublicChainAnchor`,
`BondLedgerRecord`, `Custom`.

### GAP-09: sweetGrass `braid.create` requires `data_hash` + `mime_type` + `size`

**Resolved schema**:
```json
{
  "name": "...",
  "data_hash": "content-hash-hex",
  "mime_type": "application/json",
  "size": 42,
  "metadata": {"key": "value"}
}
```

Returns W3C PROV-O formatted JSON-LD with `@id = urn:braid:{data_hash}`.

### GAP-10: sweetGrass `contribution.record` requires `agent` + `role` enum + `content_hash`

**Resolved schema**:
```json
{
  "braid_id": "urn:braid:{data_hash}",
  "agent": "contributor-name",
  "role": "Validator",
  "content_hash": "hash-of-contributed-content",
  "description": "..."
}
```

Valid roles: `Creator`, `Contributor`, `Publisher`, `Validator`, `DataProvider`,
`ComputeProvider`, `StorageProvider`, `Orchestrator`, `Curator`, `Transformer`,
`Owner`, `Custom`.

### GAP-11: sweetGrass `attribution.chain` expects `hash` not `braid_id`

`attribution.chain` params: `{"hash": "content-hash"}` (not `braid_id`).

### GAP-12: petalTongue `visualization.render.dashboard` requires `session_id` + `bindings`

Dashboard rendering needs more params than documented:
```json
{"session_id": "...", "title": "...", "bindings": {...}, "panels": [...]}
```

The `motor.panel.update` method does NOT exist — confirming the motor channel
P0 bug from `PETALTONGUE_DESKTOP_SHELL_PHASE56_APR28_2026.md`.

---

## Working Features Confirmed (Full Refresh)

| Feature | Status | Evidence |
|---------|--------|----------|
| BearDog ChaCha20-Poly1305 encrypt | **PASS** | Returns ciphertext + tag + nonce |
| BearDog BLAKE3 hash | **PASS** | Returns base64 hash |
| BearDog BTSP server status | **PASS** | `active_sessions: 0`, `max_active: 1024` |
| BearDog capabilities (196 methods) | **PASS** | Full cost estimates + signed announcement |
| Songbird discovery (10 primals) | **PASS** | `ipc.list` shows all 10 with capability lists |
| NestGate storage round-trip | **PASS** | `store` + `get` with family-scoped namespacing |
| NestGate encrypt-at-rest | **PASS** | `family_id` in responses confirms family scoping |
| Squirrel tool aggregation (33 tools) | **PASS** | Includes AI, context, graph, discovery, lifecycle domains |
| ToadStool compute (115 methods) | **PASS** | Includes ecology, ollama, GPU, WASM, science domains |
| Barracuda stats.mean | **PASS** | Returns correct `3.0` for `[1,2,3,4,5]` |
| Barracuda tensor.create | **PASS** | 2x3 tensor created on CPU backend |
| Barracuda spectral.fft | **PASS** | Correct FFT of square wave |
| rhizoCrypt DAG session lifecycle | **PASS** | create → append × 2 → merkle root (chain grows) |
| rhizoCrypt vertex hash chain | **PASS** | Merkle root changes with each append |
| loamSpine spine lifecycle | **PASS** | create → append → get_tip (Tower-signed entries) |
| sweetGrass braid lifecycle | **PASS** | create → get → get_by_hash (W3C PROV-O) |
| sweetGrass composition health | **PASS** | `composition.nucleus_health` responds |
| petalTongue 60 FPS | **PASS** | `proprioception.get` shows 60 FPS, `loop_complete: true` |
| petalTongue health (post-symlink) | **PASS** | Running stably with virtual display |
| Seed fingerprints | **PASS** | 12/12 present |
| Capability domain symlinks | **PASS** | All domain aliases created |

---

## Provenance Trio E2E Status (Fully Resolved)

| Step | Primal | Status | Notes |
|------|--------|--------|-------|
| Create DAG session | rhizoCrypt | **PASS** | Returns UUID, `session_type` auto-maps to General |
| Append AgentAction | rhizoCrypt | **PASS** | Returns vertex hash, session_id must be UUID from create |
| Append Custom event | rhizoCrypt | **PASS** | Needs `event_name` + `domain` + `label` inside Custom |
| Merkle root | rhizoCrypt | **PASS** | Root changes with each append |
| Create spine | loamSpine | **PASS** | Returns spine_id UUID + genesis_hash |
| Append entry | loamSpine | **PASS** | `payload` (byte array) goes INSIDE entry_type variant |
| Get tip | loamSpine | **PASS** | Tower-signed, hash-chained entries |
| Create braid | sweetGrass | **PASS** | W3C PROV-O with Ed25519 Tower witness |
| Get braid | sweetGrass | **PASS** | Full JSON-LD with `@context`, `@type` |
| Record contribution | sweetGrass | **PASS** | Needs `agent`, `role` (enum), `content_hash` |

**Verdict**: Provenance trio is **100% functional E2E**. All parameter formats
now documented above. The main gap was documentation — the wire format uses
Rust serde struct variants that differ from the simplified spec format.

---

## Summary of Actions

| Gap | Owner | Priority | Status | Action |
|-----|-------|----------|--------|--------|
| GAP-01 | petalTongue team | P1 | **Mitigated** | Read `DISCOVERY_SOCKET` env; add heartbeat backoff |
| GAP-02 | biomeOS team | P1 | **Resolved** | Neural API online; `NucleusMode::Desktop` still needed for native launch |
| GAP-03 | Squirrel team | P0 | Open | HTTP transport in inference router |
| GAP-04 | primalSpring | Cosmetic | Known | Skip coralReef in JSON-RPC validation |
| GAP-05 | ToadStool | Cosmetic | Known | Register domain-level `compute` capability |
| GAP-06 | Squirrel team | P2 | Known | `discovery.register` → `ipc.register` |
| GAP-07 | primalSpring docs | P1 | **Resolved** | rhizoCrypt Custom event_type documented above |
| GAP-08 | primalSpring docs | P1 | **Resolved** | loamSpine entry.append schema documented above |
| GAP-09 | primalSpring docs | P1 | **Resolved** | sweetGrass braid.create schema documented above |
| GAP-10 | primalSpring docs | P1 | **Resolved** | sweetGrass contribution.record schema documented above |
| GAP-11 | primalSpring docs | P1 | **Resolved** | sweetGrass attribution.chain param name documented |
| GAP-12 | primalSpring docs | P1 | Open | petalTongue dashboard full param schema needed |
| GAP-13 | biomeOS team | P1 | New | `storage` capability routes to `compute` (ToadStool) not NestGate |
| GAP-14 | biomeOS team | P1 | New | Graph parser inconsistency (3 parsers with different schemas) |
| GAP-15 | biomeOS team | P1 | New | `graph.start_continuous` fails for runtime-injected graphs |
| GAP-16 | biomeOS team | P2 | New | `graph.execute` node dispatch: nodes skip as "unknown type" |
| GAP-17 | Discovery | P1 | **Mitigated** | petalTongue not discoverable via `visualization` capability — symlink in `desktop_nucleus.sh` |
| GAP-18 | Discovery | P1 | **Mitigated** | biomeOS not discoverable via primal name `biomeos` — symlink + exp106 multi-name fallback |
| GAP-19 | Discovery | P1 | **Mitigated** | ludoSpring not discoverable via `game_science` capability — symlink in `desktop_nucleus.sh` |
| GAP-20 | Discovery | P2 | **Mitigated** | `FAMILY_ID` exported in `desktop_nucleus.sh`; experiments read env |
| GAP-21 | NestGate | P2 | **Mitigated** | `storage.store` needs `family_id` param — added in exp094/exp105/exp106/exp101 |
| GAP-22 | rhizoCrypt | P2 | New | `dag.session.create` returns error response via capability socket (exp105) |
| GAP-23 | BearDog | P2 | New | `crypto.blake3_hash` returns error response via capability socket (exp105) |

---

## Experiment Results: Micro-Desktop + The Rhizome (Phase 56)

### exp105 — The Rhizome Micro-Game

**Run**: `FAMILY_ID=desktop-nucleus cargo run -p primalspring-exp105`
**Result**: 8/13 passed, 7 skipped, 5 failures

| Phase | Check | Result | Notes |
|-------|-------|--------|-------|
| World Gen | biome_noise (Barracuda) | **PASS** | Perlin noise → Rhizome Network biome |
| World Gen | wfc_floor (ludoSpring) | SKIP | ludoSpring not discoverable via `game_science` (GAP-19) |
| World Gen | creature_spawn | **PASS** | 2 creatures placed deterministically |
| World Gen | item_spawn | **PASS** | 5 items placed deterministically |
| Rendering | scene_render (petalTongue) | SKIP | petalTongue not discoverable via `visualization` (GAP-17) |
| Game Loop | turns_simulated | **PASS** | 10 turns completed, movement + combat working |
| Game Loop | flow_eval (ludoSpring) | SKIP | ludoSpring not discoverable (GAP-19) |
| Game Loop | damage_calc (Barracuda) | **PASS** | `stats.mean([5,1,8])` → 4.7 |
| Save | nestgate_store | FAIL | Connected but error response (GAP-21) |
| Save | dag_session (rhizoCrypt) | FAIL | Connected but error response (GAP-22) |
| Save | braid_create (sweetGrass) | **PASS** | W3C PROV-O braid created |
| Save | contribution_record | FAIL | Braid created but contribution record errored |
| Load | load_game (NestGate) | FAIL | Connected but error response |
| Narration | ai_narrate (Squirrel) | **PASS** | AI chat responded |
| Crypto | crypto_hash (BearDog) | FAIL | Connected but error response (GAP-23) |
| Discovery | discovery_list (Songbird) | **PASS** | 10 primals in service mesh |

**Working end-to-end**: World generation (Barracuda noise), game loop (10 turns with combat),
Squirrel AI narration, sweetGrass braid creation, Songbird discovery.

**stdout fallback render** produces correct ASCII roguelike map with rooms, corridors,
creatures, items, and player movement.

### exp106 — Micro-Desktop Shell

**Run**: `FAMILY_ID=desktop-nucleus cargo run -p primalspring-exp106`
**Result**: 2/4 passed, 5 skipped, 2 failures

| Phase | Check | Result | Notes |
|-------|-------|--------|-------|
| biomeOS | biomeos_connect | SKIP | biomeOS not discoverable (GAP-18) |
| Health | healthy_primals | **PASS** | 11/12 primals healthy (petalTongue missing) |
| Health | health_bar_format | **PASS** | System bar shown with per-primal indicators |
| Routing | capability.call tests | SKIP | biomeOS not connected (GAP-18) |
| Graphs | graph management | SKIP | biomeOS not connected (GAP-18) |
| Provenance | sidebar DAG | SKIP | dag.session.create failed (GAP-22) |
| Rendering | multi-session | SKIP | petalTongue not discoverable (GAP-17) |
| Fallback | direct_nestgate | FAIL | Connected but error response (GAP-21) |
| Fallback | direct_barracuda | FAIL | Connected but error response |

**System health bar** confirms 11/12 primals responsive to heartbeat on `desktop-nucleus`
family sockets. Only petalTongue missing (no `visualization-*` socket alias).

### Upstream Handoff Notes

**P1 — Socket Naming Gaps (GAP-17, 18, 19)** — **MITIGATED LOCAL**:
The capability-based discovery (`discover_by_capability`) finds sockets named
`{capability}-{family}.sock`. Three primals register sockets by primal name instead
of capability name:
- petalTongue → `petaltongue-desktop-nucleus.sock` (not `visualization-*`)
- biomeOS → `neural-api-desktop-nucleus.sock` (not `biomeos-*` or `orchestration-*`)
- ludoSpring → no socket found (possibly not started or registered as different cap)

**Local mitigation (April 29)**: `desktop_nucleus.sh` now creates 13 capability-aliased
symlinks via `create_capability_symlinks()` after primal startup. exp106 also tries
`neural-api` and `orchestration` as fallback discovery names for biomeOS.

**Upstream fix**: Each primal should register capability-aliased sockets alongside
primal-named ones at startup (`visualization-{family}.sock` etc.).

**P2 — FAMILY_ID Default (GAP-20)** — **MITIGATED LOCAL**:
`discover_by_capability` defaults `FAMILY_ID` to `"default"`, but the running NUCLEUS uses
family `desktop-nucleus`. `desktop_nucleus.sh` already exports `FAMILY_ID` (line 30).
Experiments read `FAMILY_ID` from env and thread it into IPC calls.

**Upstream fix**: `discover_by_capability()` should read `FAMILY_ID` from a primal
manifest or biomeOS runtime state, not just env vars.

**P2 — IPC Error Responses (GAP-21, 22, 23)** — **GAP-21 MITIGATED LOCAL**:
NestGate `storage.store` requires `family_id` parameter — confirmed by exp094 pattern.
Added `family_id` to all NestGate calls in exp101, exp105, exp106.

Remaining:
- rhizoCrypt `dag.session.create` may have a different schema on the `dag-*` capability socket vs. the `rhizocrypt-*` primal socket (GAP-22)
- BearDog `crypto.blake3_hash` may need different parameter encoding (`data` as bytes vs. string) (GAP-23)

These require parameter fuzzing against each primal's actual wire format on the capability-named sockets.
