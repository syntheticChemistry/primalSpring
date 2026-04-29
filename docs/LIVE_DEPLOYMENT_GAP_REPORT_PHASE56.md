# Live Desktop NUCLEUS — Deployment Gap Report (Phase 56)

**Date**: April 29, 2026 (reharvest pass — local debt resolution)
**Deployment**: `desktop_nucleus.sh start` + biomeOS `neural-api` (family=desktop-nucleus)
**Primals deployed**: 11 running + biomeOS Neural API coordinator (petalTongue not started)
**Health**: 11/12 healthy (petalTongue process absent, coralReef tarpc-only)
**biomeOS**: Online — 605 registered capabilities from 36 sockets
**Experiments**: exp101 ALL PASS, exp105 ALL PASS (17/17), exp106 11/14 (3 failures = stale biomeOS binary)

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
| GAP-01 | petalTongue | P1 | **RESOLVED UPSTREAM** | petalTongue reads `DISCOVERY_SOCKET` env var + exponential backoff (commit `4afeb84`) |
| GAP-02 | biomeOS | P1 | **Resolved** | Neural API online; `NucleusMode::Desktop` still needed for native launch |
| GAP-03 | Squirrel | P0 | **RESOLVED LOCAL** | Squirrel pushed locally — HTTP transport in inference router |
| GAP-04 | primalSpring | Cosmetic | Known | Skip coralReef in JSON-RPC validation |
| GAP-05 | ToadStool | Cosmetic | **RESOLVED UPSTREAM** | ToadStool S207: self-registration via `DISCOVERY_SOCKET` + `ipc.register` at startup |
| GAP-06 | Squirrel | P2 | Known | `discovery.register` → `ipc.register` |
| GAP-07 | primalSpring docs | P1 | **Resolved** | rhizoCrypt Custom event_type documented above |
| GAP-08 | primalSpring docs | P1 | **Resolved** | loamSpine entry.append schema documented above |
| GAP-09 | primalSpring docs | P1 | **Resolved** | sweetGrass braid.create schema documented above |
| GAP-10 | primalSpring docs | P1 | **Resolved** | sweetGrass contribution.record schema documented above |
| GAP-11 | primalSpring docs | P1 | **Resolved** | sweetGrass attribution.chain param name documented |
| GAP-12 | primalSpring docs | P1 | Open | petalTongue dashboard full param schema needed |
| GAP-13 | biomeOS | P1 | **RESOLVED UPSTREAM** | biomeOS v3.31: `capability.call` prefers translation registry provider (commit `d2280b29`) |
| GAP-14 | biomeOS | P1 | **RESOLVED UPSTREAM** | biomeOS v3.31: unified dual-parse loader for all graph paths (commit `d2280b29`) |
| GAP-15 | biomeOS | P1 | **RESOLVED UPSTREAM** | biomeOS v3.31: `graph.start_continuous` works for runtime-injected graphs (commit `d2280b29`) |
| GAP-16 | biomeOS | P2 | **RESOLVED UPSTREAM** | biomeOS v3.31: `register_only` + capability-bearing fallback in executor dispatch (commit `d2280b29`) |
| GAP-17 | petalTongue | P1 | **RESOLVED UPSTREAM** | petalTongue creates `visualization-{family}.sock` symlink at startup (confirmed in btsp/types.rs) |
| GAP-18 | Discovery | P1 | **Mitigated** | biomeOS `capability.resolve` is authoritative per v3.31; local symlink + exp106 fallback remain |
| GAP-19 | Discovery | P1 | **Mitigated** | ludoSpring not discoverable via `game_science` capability — symlink in `desktop_nucleus.sh` |
| GAP-20 | Discovery | P2 | **Mitigated** | `FAMILY_ID` exported in `desktop_nucleus.sh`; experiments read env |
| GAP-21 | NestGate | P2 | **RESOLVED UPSTREAM** | NestGate S49: `family_id` optional — falls back to server's `NESTGATE_FAMILY_ID` (commit `7a4b9556a`) |
| GAP-22 | primalSpring | P2 | **RESOLVED LOCAL** | Root cause: `dag.session.create` returns bare UUID, not `{"session_id":"..."}`. Fixed response parsing in experiments. |
| GAP-23 | primalSpring | P2 | **RESOLVED LOCAL** | Root cause: BearDog `crypto.blake3_hash` expects base64-encoded `data`. Fixed to base64-encode in experiments. |
| GAP-24 | Barracuda | P2 | **RESOLVED LOCAL** | `noise.perlin2d` params changed: `width`/`height` → `x`/`y`. Response changed: `{"data":[...]}` → `{"result":0.0}`. Fixed in experiments. |
| GAP-25 | loamSpine | P2 | **RESOLVED LOCAL** | `spine.create` now requires `owner` field. Added to experiments. |
| GAP-26 | sweetGrass | P2 | **RESOLVED LOCAL** | `contribution.record` with `content_hash` matching existing braid `data_hash` returns "Braid already exists". Use unique contribution hash. |
| GAP-27 | biomeOS | P1 | **Stale binary** | biomeOS binary in plasmidBin is pre-v3.31. `graph.list`/`graph.status`/`graph.save` return 0/error. `capability.discover("storage")` misroutes to ToadStool. Rebuild needed. |

---

## Experiment Results: Micro-Desktop + The Rhizome (Phase 56, Reharvest)

### exp101 — fieldMouse AI Triage

**Run**: `FAMILY_ID=desktop-nucleus cargo run -p primalspring-exp101`
**Result**: **ALL PASS** — 2/2 passed, 1 skipped

| Phase | Check | Result | Notes |
|-------|-------|--------|-------|
| Storage | storage_ingest (NestGate) | **PASS** | Via primal-name fallback (biomeOS misroutes to ToadStool — GAP-27) |
| AI | ai_models_available (Squirrel) | **PASS** | Inference models available |
| Rendering | alert_render (petalTongue) | SKIP | petalTongue process not running |

### exp105 — The Rhizome Micro-Game

**Run**: `FAMILY_ID=desktop-nucleus cargo run -p primalspring-exp105`
**Result**: **ALL PASS** — 17/17 passed, 4 skipped

| Phase | Check | Result | Notes |
|-------|-------|--------|-------|
| World Gen | biome_noise (Barracuda) | **PASS** | Perlin noise via `x`/`y` params (GAP-24 fixed) → Rhizome Network |
| World Gen | wfc_floor (ludoSpring) | SKIP | ludoSpring not deployed |
| World Gen | creature_spawn | **PASS** | 2 creatures placed deterministically |
| World Gen | item_spawn | **PASS** | 5 items placed deterministically |
| Rendering | scene_render (petalTongue) | SKIP | petalTongue process not running |
| Game Loop | turns_simulated | **PASS** | 10 turns, movement + combat |
| Game Loop | flow_eval (ludoSpring) | SKIP | ludoSpring not deployed |
| Game Loop | damage_calc (Barracuda) | **PASS** | `stats.mean([5,1,8])` → 4.7 |
| Save | nestgate_store (NestGate) | **PASS** | Via primal-name fallback (biomeOS misroute — GAP-27) |
| Save | dag_session (rhizoCrypt) | **PASS** | Bare UUID parsed (GAP-22 fixed) |
| Save | dag_seal (rhizoCrypt) | **PASS** | Custom event sealed in DAG |
| Save | ledger_entry (loamSpine) | **PASS** | With `owner` param (GAP-25 fixed) |
| Save | braid_create (sweetGrass) | **PASS** | W3C PROV-O braid, unique hash per run |
| Save | contribution_record (sweetGrass) | **PASS** | Unique contribution hash (GAP-26 fixed) |
| Save | save_complete | **PASS** | Full save pipeline E2E |
| Load | load_game (NestGate) | **PASS** | Round-trip via primal-name fallback |
| Load | merkle_verify (rhizoCrypt) | **PASS** | Merkle root verified (bare string parsed) |
| Narration | ai_narrate (Squirrel) | **PASS** | AI chat responded |
| Crypto | crypto_hash (BearDog) | **PASS** | BLAKE3 hash with base64 data (GAP-23 fixed) |
| Discovery | discovery_list (Songbird) | **PASS** | 10 primals in service mesh |

**Full roguelike pipeline validated**: world gen → render (stdout fallback) → 10-turn game loop
→ save (NestGate + provenance trio) → load → merkle verify → AI narrate → crypto hash.

### exp106 — Micro-Desktop Shell

**Run**: `FAMILY_ID=desktop-nucleus cargo run -p primalspring-exp106`
**Result**: 11/14 passed, 2 skipped, 3 failures (all biomeOS stale binary — GAP-27)

| Phase | Check | Result | Notes |
|-------|-------|--------|-------|
| biomeOS | biomeos_connect | **PASS** | Multi-name fallback: biomeos → neural-api → orchestration |
| Health | healthy_primals | **PASS** | 11/12 healthy (petalTongue not running) |
| Health | health_bar_format | **PASS** | `[Bio✓] [Song✓] [Nest✓] ... [Petal✗]` |
| Routing | route_crypto (biomeOS) | **PASS** | blake3_hash with base64 via capability.call |
| Routing | route_dag (biomeOS) | **PASS** | dag.session.create via capability.call |
| Routing | route_stats (biomeOS) | **PASS** | stats.mean via capability.call |
| Routing | route_discovery (biomeOS) | **PASS** | ipc.list via capability.call |
| Routing | route_storage (biomeOS) | SKIP | Misroutes to ToadStool (GAP-27 — stale binary) |
| Graphs | graph_list | **FAIL** | Returns 0 graphs — stale biomeOS binary (GAP-27) |
| Graphs | graph_status | **FAIL** | Not accessible — stale biomeOS binary (GAP-27) |
| Graphs | graph_save | **FAIL** | Not functional — stale biomeOS binary (GAP-27) |
| Provenance | prov_session (rhizoCrypt) | **PASS** | DAG session created |
| Provenance | prov_merkle (rhizoCrypt) | **PASS** | Merkle root computed |
| Rendering | multi_render (petalTongue) | SKIP | petalTongue not running |
| Fallback | direct_nestgate | **PASS** | Direct NestGate storage via primal name |
| Fallback | direct_barracuda | **PASS** | `noise.perlin2d` with `x`/`y` params |

### Upstream Handoff Notes

**P1 — Stale biomeOS Binary (GAP-27)** — **BLOCKING 3 exp106 checks**:
The `plasmidBin/primals/x86_64-unknown-linux-musl/biomeos` binary is pre-v3.31.
v3.31 (pulled, in source tree) fixes:
- `capability.discover("storage")` misrouting to ToadStool instead of NestGate
- `graph.list` / `graph.status` / `graph.save` returning empty/error
- Unified dual-parse TOML loader for all graph paths

**Action**: Rebuild biomeOS from pulled source and restart NUCLEUS. All 3 exp106
failures and the `route_storage` skip should resolve.

**P1 — Socket Naming Gaps (GAP-17, 18, 19)** — **MITIGATED LOCAL**:
`desktop_nucleus.sh` creates 13 capability-aliased symlinks. petalTongue (upstream)
now creates `visualization-{family}.sock` at startup. biomeOS found via multi-name
fallback in exp106. ludoSpring binary not deployed.

**P2 — Resolved Local Debt (GAP-22 through GAP-26)**:
All five gaps were caller-side issues in primalSpring experiments:
- GAP-22: `dag.session.create` returns bare UUID, not `{"session_id":"..."}`
- GAP-23: `crypto.blake3_hash` expects base64-encoded `data`
- GAP-24: `noise.perlin2d` params changed `width`/`height` → `x`/`y`
- GAP-25: `spine.create` now requires `owner` field
- GAP-26: `contribution.record` `content_hash` must differ from braid's `data_hash`

These represent **upstream API evolution** — primalSpring experiments were calling
stale schemas. All fixed in this reharvest pass.

**P2 — biomeOS Tier 1 Discovery Workaround**:
Even with biomeOS running, `discover_by_capability("storage")` routes to ToadStool.
Experiments now use `discover_primal("nestgate")` as fallback when Tier 1 returns
a misrouted socket. This workaround should be removable once biomeOS binary is rebuilt.
