# Phase 56: Desktop Substrate + The Rhizome — Primal & Spring Team Handoff

**Date**: April 28, 2026
**Version**: 0.9.23
**From**: primalSpring (coordination and composition validation)
**To**: All primal teams + spring teams + garden teams

---

## Summary

Phase 56 brings primalSpring from "coordination validated" to "desktop applications
running." We now have a live 12-primal NUCLEUS serving desktop applications through
biomeOS and petalTongue, with The Rhizome roguelike game as the first real application.

This handoff documents what works, what gaps remain, and what each team should
absorb to make the desktop substrate production-ready.

---

## What's New

### Desktop NUCLEUS (12 primals live)
- `tools/desktop_nucleus.sh` starts all 12 primals with correct env wiring
- biomeOS Neural API coordinates via `neural-api` subcommand
- 605 registered capabilities, 3078 auto-discovered from 36 sockets
- 11/12 primals healthy on heartbeat (petalTongue discovery gap)

### The Rhizome (exp105)
- Roguelike micro-game running on NUCLEUS substrate
- Barracuda `noise.perlin2d` for biome generation (working)
- Barracuda `stats.mean` for damage calculation (working)
- petalTongue `visualization.render.scene` for tile grid (discovery gap — uses stdout fallback)
- NestGate `storage.store/get` for TOML saves (connected, param gap)
- Provenance trio for save sealing (sweetGrass braid works, others have param gaps)
- Squirrel `ai.chat` for narration (working)
- 10-turn game loop with movement, combat, item pickup (all working)

### Micro-Desktop Shell (exp106)
- biomeOS capability routing tested (crypto, dag, stats, discovery all route correctly)
- System health bar: 11/12 primals confirmed healthy via socket heartbeat
- Multi-session petalTongue rendering (attempted, blocked by discovery gap)
- Provenance sidebar with DAG event chain + merkle root

### 23 Gaps Documented
- `docs/LIVE_DEPLOYMENT_GAP_REPORT_PHASE56.md` — full inventory with severity, status, evidence

---

## Per-Primal Action Items

### biomeOS
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-13 | P1 | `storage` capability routes to ToadStool, not NestGate | Fix capability registry mapping |
| GAP-14 | P1 | Three graph parsers with different schema requirements | Unify `neural-api`, `deploy`, `continuous` parsers |
| GAP-15 | P1 | `graph.start_continuous` fails for runtime-injected graphs | Fix continuous graph executor |
| GAP-16 | P2 | `graph.execute` skips nodes as "unknown type" | Implement node dispatch for all node types |
| GAP-18 | P1 | biomeOS not discoverable as `biomeos` — socket is `neural-api-*` | Register `biomeos-{family}.sock` symlink/alias |

### petalTongue
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-01 | P1 | Hardcodes heartbeat to `discovery-service.sock` ignoring env | Read `DISCOVERY_SOCKET` env var |
| GAP-17 | P1 | Not discoverable via `visualization` capability | Register `visualization-{family}.sock` alongside `petaltongue-*` |
| motor P0 | P0 | `motor.panel.update` logged but not routed to GUI | Wire motor channel commands to egui |

### Squirrel
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-03 | P0 | HTTP inference provider URLs treated as UDS paths | Fix `inference.register_provider` HTTP routing |

### NestGate
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-21 | P2 | `storage.store` returns error on capability-named socket | Verify param schema matches on `storage-{family}.sock` |

### rhizoCrypt
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-22 | P2 | `dag.session.create` errors on `dag-{family}.sock` | Verify param schema on capability socket vs primal socket |

### BearDog
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-23 | P2 | `crypto.blake3_hash` errors on `crypto-{family}.sock` | Verify `data` param encoding (string vs bytes) |

### Songbird
- Working well. `ipc.list` returns 10+ primals. Discovery mesh is stable.
- Consider: Register capability-aliased sockets for primals that only register primal-name sockets.

### ludoSpring
| Gap | Priority | Description | Fix |
|-----|----------|-------------|-----|
| GAP-19 | P1 | Not discoverable via `game_science` capability | Register `game_science-{family}.sock` or expose socket under capability name |

---

## Composition Patterns for Downstream Springs

### The Python → Rust → Primal Pipeline

Every spring's baseCamp follows this validation pipeline:

1. **Python baseline**: Peer-reviewed, reproducible implementations (NumPy, SciPy)
2. **Rust port**: Matches Python within tolerance. Pure Rust, ecoBin compliant.
3. **Primal composition**: Matches Rust port via IPC through NUCLEUS. `validate_parity()` compares.

For primalSpring, this is recursive: the "science" IS coordination. For other springs:
- hotSpring validates physics → primal compositions match PyTorch baselines
- wetSpring validates biology → primal compositions match BioPython baselines
- neuralSpring validates ML → primal compositions match transformer baselines

### NUCLEUS Deployment via biomeOS Neural API

The standard deployment pattern for applications on NUCLEUS:

```
1. Start NUCLEUS:  desktop_nucleus.sh start
2. Start biomeOS:  biomeos neural-api --socket $SOCKET --graphs-dir $GRAPHS --family-id $FAMILY
3. Deploy graph:   biomeOS graph.execute / graph.save / graph.start_continuous
4. Route calls:    capability.call { capability, operation, params }
```

**Graph template** (`graphs/desktop/app_rhizome.toml`):
- Declare the app binary as `spawn = true`
- Declare NUCLEUS capabilities as `spawn = false` (consumed from substrate)
- Use `by_capability` for loose coupling
- Set `coordination = "continuous"` for game loops / UI
- Include `[graph.metadata.application]` with `tick_hz`, `display_mode`

### Capability Routing Rules

When using biomeOS `capability.call`:
- `operation` parameter MUST contain the full dotted method name (e.g., `dag.session.create` not `session.create`)
- `capability` determines which primal socket biomeOS routes to
- The route strips `capability` and forwards `operation` as JSON-RPC method

### Discovery Pattern

```rust
let disc = discover_by_capability("math");
let sock = disc.socket.expect("Barracuda not found");
let mut client = PrimalClient::connect(&sock, "barracuda")?;
let resp = client.call("stats.mean", json!({"data": [1.0, 2.0, 3.0]}))?;
```

Requires `FAMILY_ID=desktop-nucleus` (or your deployment family) in environment.

---

## What Springs Should Absorb

### For springs building desktop applications:
1. Use `graphs/desktop/app_rhizome.toml` as template for your app graph
2. Use `specs/DESKTOP_SESSION_MODEL.md` for session lifecycle patterns
3. Use `specs/MICRO_DESKTOP_COMPOSITION.md` for desktop layout conventions
4. Use exp105/exp106 as reference for `ValidationResult` gap-finding pattern

### For springs consuming NUCLEUS capabilities:
1. Set `FAMILY_ID` to your deployment family name
2. Use `discover_by_capability()` with the capability name (not primal name)
3. Implement graceful degradation (skip, not crash, when primal unavailable)
4. Read `docs/LIVE_DEPLOYMENT_GAP_REPORT_PHASE56.md` for known parameter schemas

### For springs validating their science:
1. Follow the Python → Rust → Primal pattern in `whitePaper/baseCamp/README.md`
2. Use `check_composition_parity()` / `check_composition_parity_vec()` from ValidationResult
3. Report gaps back to primalSpring via the gap report format

---

## Provenance Trio — Corrected Wire Schemas

These schemas were discovered through extensive live debugging and are critical
for any spring consuming the provenance trio:

### rhizoCrypt `dag.event.append`
```json
{
  "session_id": "<uuid from dag.session.create>",
  "event_type": {
    "Custom": {"label": "...", "event_name": "...", "domain": "..."}
  },
  "data": {"key": "value"}
}
```

### loamSpine `entry.append`
```json
{
  "spine_id": "<uuid from spine.create>",
  "committer": "agent-name",
  "entry_type": {
    "Custom": {
      "label": "...", "type_uri": "urn:eco:...",
      "domain": "...", "payload": [104, 101, 108, 108, 111]
    }
  }
}
```
Note: `payload` is byte array INSIDE the entry_type variant.

### sweetGrass `braid.create`
```json
{
  "name": "...",
  "data_hash": "hex-hash",
  "mime_type": "application/json",
  "size": 42,
  "metadata": {"key": "value"}
}
```

### sweetGrass `contribution.record`
```json
{
  "braid_id": "urn:braid:{data_hash}",
  "agent": "contributor-name",
  "role": "Creator",
  "content_hash": "hex-hash",
  "description": "..."
}
```
Valid roles: Creator, Contributor, Publisher, Validator, DataProvider, ComputeProvider,
StorageProvider, Orchestrator, Curator, Transformer, Owner, Custom.

---

## Files Created/Modified in Phase 56

### New Files
- `specs/RHIZOME_MICRO_GAME.md` — Game design spec
- `specs/MICRO_DESKTOP_COMPOSITION.md` — Desktop shell spec
- `specs/DESKTOP_NUCLEUS_DEPLOYMENT.md` — Deployment spec
- `specs/DESKTOP_SESSION_MODEL.md` — Session model spec
- `specs/LIVE_GUI_COMPOSITION_PATTERN.md` — GUI patterns
- `graphs/desktop/app_rhizome.toml` — The Rhizome deploy graph
- `graphs/desktop/app_esotericwebb.toml` — esotericWebb deploy graph
- `graphs/desktop/app_system_monitor.toml` — System monitor deploy graph
- `graphs/desktop/desktop_shell.toml` — Desktop shell deploy graph
- `experiments/exp099_agentic_loop_substrate/` through `experiments/exp106_micro_desktop_shell/`
- `docs/LIVE_DEPLOYMENT_GAP_REPORT_PHASE56.md` — Live gap report

### Modified Files
- `Cargo.toml` — workspace members updated (84 experiments)
- `tools/desktop_nucleus.sh` — auto-symlink for petalTongue discovery
- All root docs (README, CHANGELOG, CONTEXT, wateringHole, baseCamp, experiments)

---

## Next Steps

1. **P0 fixes**: Squirrel HTTP provider routing, petalTongue motor channel
2. **P1 fixes**: Socket naming (biomeOS, petalTongue, ludoSpring), biomeOS graph parsers, capability registry
3. **P2 investigation**: Parameter schema mismatches on capability-named sockets
4. **Downstream**: Springs adopt desktop composition patterns, build domain-specific apps
5. **The Rhizome**: Evolve from validation experiment to playable demo (interactive input, deeper game logic)
