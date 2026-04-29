# Desktop NUCLEUS Deployment

**Status**: Design Spec (Phase 56)
**Date**: April 28, 2026
**Origin**: primalSpring v0.9.22 Phase 55c — all 12 primals resolved
**Scope**: Evolve `biomeos nucleus` from a 5-primal coordinator to the full 12-primal desktop substrate

---

## Background

Today the Desktop NUCLEUS deploys via two paths:

1. **`composition_nucleus.sh`** (primary): Shell script that launches 11 primals
   in phased dependency order, creates capability symlinks, registers primals
   with Songbird, persists the family seed, and optionally starts petalTongue
   in `live` mode. This is the production path — validated at 28/30 checks.

2. **`biomeos nucleus --mode full`** (secondary): Rust binary that launches
   5 primals (BearDog, Songbird, NestGate, ToadStool, Squirrel) + Neural API.
   Has richer lifecycle management (`LifecycleManager`, health monitoring,
   pre-existing cluster detection) but cannot deploy the full Desktop NUCLEUS.

This spec defines `biomeos nucleus --mode desktop` — a new mode that absorbs
the composition launcher's capabilities into the biomeOS binary, making the
shell scripts a convenience rather than a requirement.

---

## Target Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    biomeos nucleus --mode desktop                │
│                                                                  │
│  ┌──────────────────┐  ┌──────────────────────────────────────┐ │
│  │ LifecycleManager │  │           Neural API Server          │ │
│  │                  │  │  (graph.deploy, capability.call,     │ │
│  │  health monitor  │  │   app.launch, composition.health)    │ │
│  │  process resurrect│  │                                     │ │
│  │  ordered shutdown │  └──────────────────────────────────────┘ │
│  └────────┬─────────┘                                           │
│           │ spawns + monitors                                    │
│  ┌────────┴──────────────────────────────────────────────────┐  │
│  │                  Primal Process Table                      │  │
│  │                                                           │  │
│  │  Phase 1 — Tower (electron)                               │  │
│  │    [1] BearDog    (security, crypto, btsp, secrets)       │  │
│  │    [2] Songbird   (discovery, ipc, http, mesh)            │  │
│  │                                                           │  │
│  │  Phase 2 — Nest (neutron)                                 │  │
│  │    [3] NestGate   (storage, encrypt-at-rest)              │  │
│  │    [4] Squirrel   (ai, inference, context, tool)          │  │
│  │                                                           │  │
│  │  Phase 3 — Node (proton)                                  │  │
│  │    [5] ToadStool  (compute, dispatch)                     │  │
│  │    [6] barraCuda  (tensor, math, stats)                   │  │
│  │    [7] coralReef  (shader, gpu_compile)                   │  │
│  │                                                           │  │
│  │  Phase 4 — Provenance (rootpulse)                         │  │
│  │    [8] rhizoCrypt (dag, merkle)                           │  │
│  │    [9] loamSpine  (ledger, certificate)                   │  │
│  │   [10] sweetGrass (attribution, braid, anchoring)         │  │
│  │                                                           │  │
│  │  Phase 5 — Meta (visualization)                           │  │
│  │   [11] petalTongue (visualization, motor, sensor)         │  │
│  │                                                           │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              Songbird Registration Loop                    │  │
│  │  After each primal is healthy, biomeOS calls               │  │
│  │  ipc.register with capabilities + socket endpoint          │  │
│  └───────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## `NucleusMode::Desktop`

### CLI

```
biomeos nucleus --mode desktop [--node-id <id>] [--family-id <id>] [--port <n>] [--tcp-only]
                               [--petaltongue-live] [--no-petaltongue]
                               [--graphs-dir <path>]
```

### Primal List (12 primals, ordered)

The `Desktop` mode extends `Full` by adding Compute, Provenance, and Meta
tier primals. The full launch list:

| Order | Primal | Tier | Required | Depends On |
|-------|--------|------|----------|------------|
| 1 | BearDog | Tower | yes | -- |
| 2 | Songbird | Tower | yes | BearDog |
| 3 | NestGate | Nest | yes | BearDog, Songbird |
| 4 | Squirrel | Nest | no | Songbird |
| 5 | ToadStool | Node | yes | BearDog, Songbird |
| 6 | barraCuda | Node | yes | BearDog |
| 7 | coralReef | Node | yes | ToadStool |
| 8 | rhizoCrypt | Provenance | no | BearDog |
| 9 | loamSpine | Provenance | no | BearDog, rhizoCrypt |
| 10 | sweetGrass | Provenance | no | BearDog, loamSpine |
| 11 | petalTongue | Meta | yes | Songbird |

biomeOS itself (the Neural API) runs as the 12th component in-process,
not as a spawned child.

### Mode Mapping

```rust
impl NucleusMode {
    fn primals(self) -> Vec<&'static str> {
        match self {
            NucleusMode::Tower   => vec![BEARDOG, SONGBIRD],
            NucleusMode::Node    => vec![BEARDOG, SONGBIRD, TOADSTOOL],
            NucleusMode::Nest    => vec![BEARDOG, SONGBIRD, NESTGATE, SQUIRREL],
            NucleusMode::Full    => vec![BEARDOG, SONGBIRD, NESTGATE, TOADSTOOL, SQUIRREL],
            NucleusMode::Desktop => vec![
                BEARDOG, SONGBIRD,                           // Tower
                NESTGATE, SQUIRREL,                          // Nest
                TOADSTOOL, BARRACUDA, CORALREEF,             // Node
                RHIZOCRYPT, LOAMSPINE, SWEETGRASS,           // Provenance
                PETALTONGUE,                                 // Meta
            ],
        }
    }
}
```

---

## Launch Phases

### Phase 1: Tower Atomic

BearDog and Songbird form the trust boundary. BearDog must be healthy
before any other primal starts — it provides BTSP handshake, signing,
and secret management for the entire NUCLEUS.

**BearDog environment:**

| Variable | Value | Source |
|----------|-------|--------|
| `BEARDOG_FAMILY_SEED` | 64-char hex | env or generated from `/dev/urandom` |
| `FAMILY_ID` | string | CLI `--family-id` or env |
| `BEARDOG_NODE_ID` | hostname | CLI `--node-id` or env |

**Songbird environment:**

| Variable | Value | Source |
|----------|-------|--------|
| `SONGBIRD_SECURITY_PROVIDER` | BearDog socket path | derived from socket naming |
| `BTSP_PROVIDER_SOCKET` | BearDog socket path | derived |
| `SONGBIRD_DISCOVERY_MODE` | `"disabled"` | hardcoded for local NUCLEUS |

### Phase 2: Nest Atomic

NestGate and Squirrel. NestGate delegates auth to BearDog via
`NESTGATE_AUTH_MODE=beardog`. Squirrel discovers services via
`DISCOVERY_SOCKET` pointing at Songbird.

### Phase 3: Node Atomic

ToadStool (GPU compute), barraCuda (math/tensor), coralReef (shader).
All receive BearDog and Songbird socket paths. barraCuda may create
a `math-{family}.sock` alias; biomeOS should handle both socket names.

### Phase 4: Provenance Trio

rhizoCrypt, loamSpine, sweetGrass. Launched in dependency order since
loamSpine consumes rhizoCrypt DAGs, and sweetGrass consumes loamSpine
ledger entries. All delegate signing to BearDog via `crypto.sign_ed25519`.

### Phase 5: Meta Visualization

petalTongue launches last. If `--petaltongue-live` is set (default for
`Desktop` mode), petalTongue runs in `live` mode (egui desktop window)
**without `setsid`** — the GUI needs the controlling terminal's display.

If `--no-petaltongue` is set, petalTongue is skipped (headless NUCLEUS).

**petalTongue environment:**

| Variable | Value |
|----------|-------|
| `DISPLAY` | inherited or `:1` |
| `PETALTONGUE_SOCKET` | `{socket_dir}/petaltongue-{family}.sock` |
| `AWAKENING_ENABLED` | `false` (startup animation disabled) |

---

## Family Seed Lifecycle

1. **Generation**: If `BEARDOG_FAMILY_SEED` is not set, biomeOS generates
   a 32-byte random seed from `/dev/urandom` and hex-encodes it (64 chars).

2. **Persistence**: biomeOS writes the seed to `{SOCKET_DIR}/.family.seed`
   with mode `0600`. This allows stop/restart without losing the seed.

3. **Detection**: On startup, if `BEARDOG_FAMILY_SEED` is unset but
   `{SOCKET_DIR}/.family.seed` exists, biomeOS reads it. This enables
   the `detect_ecosystem` codepath to reuse an existing deployment's seed.

4. **Propagation**: biomeOS exports `BEARDOG_FAMILY_SEED` and `FAMILY_SEED`
   to all child processes via `build_primal_command_with()`.

---

## Songbird Registration Contract

After each primal starts and passes its health check, biomeOS sends an
`ipc.register` JSON-RPC call to Songbird with the primal's capabilities
and socket endpoint.

### Registration Payload

```json
{
  "jsonrpc": "2.0",
  "method": "ipc.register",
  "params": {
    "primal_id": "<name>",
    "capabilities": ["<cap1>", "<cap2>", ...],
    "endpoint": "unix://<socket_path>"
  },
  "id": 1
}
```

### Capability Registry

The capability arrays come from `nucleus_launch_profiles.toml` or are
hardcoded per `NucleusMode::Desktop`. The canonical mapping:

| Primal | Capabilities |
|--------|-------------|
| BearDog | `security`, `crypto`, `btsp`, `encryption`, `genetic`, `secrets`, `tls` |
| Songbird | `discovery`, `ipc`, `http`, `stun`, `igd`, `mesh`, `relay`, `onion`, `punch` |
| NestGate | `storage` |
| Squirrel | `ai`, `inference`, `context`, `tool`, `graph` |
| ToadStool | `compute` |
| barraCuda | `tensor`, `math`, `stats`, `linalg`, `spectral`, `activation`, `ml`, `fhe`, `noise` |
| coralReef | `shader`, `gpu_compile` |
| rhizoCrypt | `dag`, `merkle`, `provenance` |
| loamSpine | `ledger`, `certificate`, `bonding`, `anchor`, `proof` |
| sweetGrass | `attribution`, `braid`, `provenance`, `compression`, `contribution` |
| petalTongue | `visualization`, `motor`, `sensor`, `interaction`, `modality`, `audio` |

### Self-Registration (Preferred)

Primals that implement self-registration (via `DISCOVERY_SOCKET` env probe)
should not be registered by biomeOS. biomeOS checks whether the primal
already appears in Songbird's registry before registering it. The check:

```json
{"jsonrpc":"2.0","method":"ipc.resolve","params":{"capability":"<primary_cap>"},"id":1}
```

If the response contains the primal's socket path, skip registration.
Otherwise, register.

Primals with confirmed self-registration as of Phase 55c:
- barraCuda (Sprint 47)
- ToadStool (S207)
- sweetGrass (v0.7.28)

---

## Capability Domain Symlinks

biomeOS creates symbolic links in `{SOCKET_DIR}` for capability-based
discovery by filesystem probe (the fallback path when Songbird is unavailable):

```
security-{family}.sock  -> beardog-{family}.sock
crypto-{family}.sock    -> beardog-{family}.sock
compute-{family}.sock   -> toadstool-{family}.sock
tensor-{family}.sock    -> barracuda-{family}.sock
storage-{family}.sock   -> nestgate-{family}.sock
dag-{family}.sock       -> rhizocrypt-{family}.sock
ledger-{family}.sock    -> loamspine-{family}.sock
attribution-{family}.sock -> sweetgrass-{family}.sock
ai-{family}.sock        -> squirrel-{family}.sock
visualization-{family}.sock -> petaltongue-{family}.sock
shader-{family}.sock    -> coralreef-{family}.sock
```

---

## Health Monitoring

biomeOS's `LifecycleManager` polls each primal's `health.liveness` endpoint
at a configurable interval (default: 10s). On failure:

1. Log warning, increment failure counter
2. After 3 consecutive failures, attempt `lifecycle.resurrect` (restart process)
3. After 5 consecutive failures, mark primal as `degraded` in composition health
4. Non-required primals (Squirrel, Provenance trio) degrade gracefully
5. Required primals (Tower, Node core) trigger composition-level alerts

The health summary is available via `composition.health` and the per-tier
endpoints: `composition.tower_health`, `composition.nest_health`,
`composition.node_health`, `composition.nucleus_health`.

---

## Ordered Shutdown

Shutdown reverses the launch order:

1. petalTongue (close GUI window)
2. sweetGrass, loamSpine, rhizoCrypt (flush provenance)
3. coralReef, barraCuda, ToadStool (drain compute)
4. Squirrel (save context)
5. NestGate (flush storage)
6. Songbird (deregister mesh)
7. BearDog (wipe ephemeral keys)

Each primal receives `SIGTERM`, waits up to 5s, then `SIGKILL` if still
running. Socket files are cleaned after all processes exit.

---

## Relationship to Existing Components

### `composition_nucleus.sh`

Remains as a convenience launcher and fallback. Once `biomeos nucleus --mode
desktop` is feature-complete, the shell script becomes a compatibility shim
that delegates to biomeOS when available, or runs the legacy shell path.

### `desktop_nucleus.sh`

Already has `start_via_biomeos()` and `start_via_composition()` paths.
The `start_via_biomeos()` path should be updated to pass `--mode desktop`
instead of `--mode full` once the biomeOS evolution is complete.

### `nucleus_desktop_cell.toml`

The cell graph already defines the 12-primal desktop topology. biomeOS
`--mode desktop` should use this graph as its source of truth for primal
ordering, capabilities, and dependency relationships — either by loading
it directly or by encoding the same information in
`nucleus_launch_profiles.toml`.

### `BIOMEOS_NUCLEUS_EVOLUTION.md`

This spec complements Phase 1 (coordination key caching, implemented).
The `Desktop` mode is a lifecycle evolution, not a crypto embedding change.
Phase 2/3 (embedded BearDog library) can proceed independently.

---

## Non-Goals for This Spec

- Embedding BearDog as a library (see `BIOMEOS_NUCLEUS_EVOLUTION.md` Phase 2)
- TCP-only mode (`--port` without UDS) — separate evolution item
- Cross-gate federation routing — separate spec
- Application session management — see `DESKTOP_SESSION_MODEL.md`
