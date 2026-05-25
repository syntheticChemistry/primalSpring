# primalSpring Architecture

## The Eukaryotic Cell Model

primalSpring has evolved from a prokaryotic ecosystem of individual
experiment binaries into a eukaryotic UniBin — a single executable that
contains all validation, certification, and coordination capabilities.

```
┌─────────────────────────────────────────────────────────────────┐
│                    primalspring UniBin                           │
│                     (the eukaryotic cell)                        │
│                                                                 │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────┐  │
│  │  Certification    │  │  Validation      │  │  IPC Server  │  │
│  │  Engine           │  │  Scenarios       │  │  (membrane)  │  │
│  │  (mitochondria)   │  │  (ribosomes)     │  │              │  │
│  │                   │  │                  │  │  JSON-RPC    │  │
│  │  L0: Bare         │  │  52 absorbed     │  │  2.0 over    │  │
│  │  L0.5: Seed       │  │  experiments     │  │  Unix socket │  │
│  │  L1: Discovery    │  │  across 10 tracks│  │              │  │
│  │  L1.5: BTSP       │  │                  │  │  MethodGate  │  │
│  │  L2: Health       │  │  Tier 1: Rust    │  │  (JH-0)      │  │
│  │  L3: Parity       │  │  Tier 2: Live/   │  │              │  │
│  │  L4: Pipeline     │  │                  │  │  capability  │  │
│  │  L5: Bonding      │  │  ScenarioMeta    │  │  discovery   │  │
│  │  L6: Crypto       │  │  + provenance    │  │  + routing   │  │
│  │  L7: Cellular     │  │  + track/tier    │  │              │  │
│  │  L8: Lifecycle    │  │  classification  │  │              │  │
│  └──────────────────┘  └──────────────────┘  └──────────────┘  │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    primalspring library                    │   │
│  │  composition · coordination · bonding · deploy · ipc      │   │
│  │  validation · certification · niche · tolerances          │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## CLI Surface

```
primalspring certify              # L0-L8 composition certification
primalspring certify --layer 3    # run up to layer 3
primalspring certify --bare       # L0 only, no primals needed

primalspring validate             # run all validation scenarios
primalspring validate --track atomic-composition
primalspring validate --scenario tower-atomic
primalspring validate --tier rust  # Tier 1 only (no IPC)
primalspring validate --tier live  # Tier 2 only (requires primals)
primalspring validate --list      # list all scenarios

primalspring serve                # JSON-RPC 2.0 IPC server
primalspring status               # composition health summary
primalspring version              # version info
```

## Two-Tier Validation Architecture

### Tier 1: Rust Validation (structural)

Pure Rust library code. No IPC, no running primals required. Tests
graph parsing, type systems, bonding policy, TOML manifests,
fragment resolution, seed provenance.

Runs in CI without a live NUCLEUS composition.

### Tier 2: Live NUCLEUS Validation (behavioral)

Requires deployed primals from plasmidBin. Exercises IPC calls via
`CompositionContext`, validates capability parity, cross-atomic
pipelines, BTSP authentication, composition lifecycle.

Runs with `biomeOS` orchestrating the full composition.

## Module Map

### Library (`ecoPrimal/src/`)

| Module | Purpose |
|--------|---------|
| `certification/` | Composition correctness engine (absorbed guidestone) |
| `certification/bare.rs` | L0: graph/fragment/manifest structural validation |
| `certification/health.rs` | L2-L4: atomic health, math parity, cross-atomic pipeline |
| `certification/bonding.rs` | L5: bonding model + live ionic bond |
| `certification/btsp.rs` | L1.5/L6: BTSP escalation, crypto, method gate |
| `certification/cellular.rs` | L7: per-spring deploy graph validation |
| `certification/lifecycle.rs` | L8: composition reload + rediscovery |
| `certification/entropy.rs` | Seed provenance, fingerprint verification |
| `validation/` | `ValidationResult` harness, check_bool/check_skip/section API |
| `validation/helpers.rs` | Shared graph parsing, Dark Forest invariants, capability cross-ref |
| `validation/scenarios/` | 49 absorbed experiment scenarios (10 tracks, 3 tiers: Rust/Live/Both) |
| `validation/scenarios/registry.rs` | `ScenarioMeta`, `ScenarioRegistry`, `Tier`, `Track` |
| `composition/` | `CompositionContext` — 5-tier discovery, IPC calls, BTSP |
| `coordination/` | `AtomicType`, composition validation (legacy probes removed Wave 32) |
| `bonding/` | `BondType`, `BondingPolicy`, `BtspEnforcer`, `TrustModel` |
| `deploy/` | Graph parsing, validation, structure |
| `ipc/` | JSON-RPC protocol, `PrimalClient`, `NeuralBridge`, discovery |
| `ipc/method_gate.rs` | MethodGate (JH-0) pre-dispatch authorization |
| `niche.rs` | Capability registration with biomeOS |
| `tolerances.rs` | Named, centralized tolerance constants |

### Binaries

| Binary | Purpose | Status |
|--------|---------|--------|
| `primalspring` | Eukaryotic UniBin (certify + validate + serve + status + version) | Active |
| `primalspring_primal` | Legacy RPC server | Transition (→ `primalspring serve`) |
| `nucleus_launcher` | Rust NUCLEUS launcher (`--federation-port` for LAN mesh) | Active |

### Validation Tracks

| Track | Description | Example Scenarios |
|-------|-------------|-------------------|
| atomic-composition | Tower/Node/Nest/Full NUCLEUS | tower-atomic, full-nucleus |
| graph-execution | Sequential/parallel/conditional DAG | sequential-graph |
| bonding | Covalent, ionic, metallic, weak | covalent-bond, ionic-bond |
| security | Bearer tokens, BTSP, method gate | bearer-token-auth, gate-failure |
| transport | Sockets, TCP, protocol escalation | socket-discovery, compute-triangle |
| cross-spring | Cross-spring data flow | cross-spring-data-flow |
| biomeos-deploy | biomeOS deployment, Neural API | biomeos-tower-deploy |
| infrastructure | Deployment matrix, cellular graphs | deployment-matrix |
| lifecycle | Composition reload, parity, federation | composition-lifecycle |
| sovereignty | Membrane composition, routing, content sovereignty | membrane-composition, sovereignty-parity |

## IPC Discovery

`CompositionContext::discover()` uses 5-tier escalation:

1. **Songbird routing** — `ipc.resolve` via the discovery primal
2. **Neural API** — `capability.call` via biomeOS (signal-tier calls transparently dispatch graph execution since v3.55)
3. **UDS convention** — `$XDG_RUNTIME_DIR/biomeos/{primal}-{fid}.sock`
4. **Socket registry scan** — enumerate known socket paths
5. **TCP probing** — opt-in, covalent mesh only

Atomic signals use `signal.dispatch` (biomeOS v3.55+) as the preferred path,
with `capability.call` fallback. `primal.announce` (v3.57) replaces separate
`lifecycle.register` + `capability.register` + `method.register` calls with
a single atomic RPC.

## Security Model

- **MethodGate (JH-0)**: Pre-dispatch capability authorization on all
  IPC endpoints. 13/13 primals adopted.
- **BTSP Phase 3 AEAD**: ChaCha20-Poly1305 for all cross-atomic connections.
  13/13 primals enforcing.
- **Ionic tokens**: BearDog Ed25519-signed capability scoped tokens.
- **Binding**: `--bind` defaults to `127.0.0.1` (PG-55, 13/13).

## Membrane Composition (VPS Sovereignty Boundary)

`graphs/membrane/tower_membrane.toml` defines the VPS inner membrane:

```
                    ┌── VPS Membrane ──────────────────────────┐
                    │                                          │
Channel 3 (Surface) │  ┌──────────┐    ┌──────────┐           │
TLS public HTTPS ───┤  │ Songbird │────│ BearDog  │           │
                    │  │ (network)│    │ (crypto) │           │
                    │  └─────┬────┘    └─────┬────┘           │
                    │        │               │                │
                    │  ┌─────┴────┐    ┌─────┴────┐           │
                    │  │ SkunkBat │    │ NestGate │           │
                    │  │ (defense)│    │ (cache)  │           │
                    │  └──────────┘    └──────────┘           │
                    │                                          │
Channel 2 (Relay)   │  BTSP tunnel ─────────── gate hardware  │
Channel 1 (Signal)  │  UDS ─── primal-to-primal IPC           │
                    └──────────────────────────────────────────┘
```

Content-aware routing (`config/routing_config_reference.toml`) decides per-request:
gate (btsp_tunnel) vs VPS cache (local_filesystem) vs peer (songbird_p2p) vs
fallback (http_proxy), scoped by bonding trust tier.

## Deprecated Patterns (Fossilized)

These patterns are deprecated and will be removed in the next stadial:

| Pattern | Replacement |
|---------|-------------|
| `AtomicHarness` / `RunningAtomic` | plasmidBin ecoBin deployment via biomeOS |
| `spawn_primal` / `spawn_biomeos` | plasmidBin ecoBin deployment |
| `probe_primal` / `check_capability_health` | `CompositionContext.health_check()` |
| `validate_composition` | `CompositionContext.call()` |
| `PrimalClient::connect` (direct) | `CompositionContext.client_for()` |
| `CompositionContext::from_running` | `CompositionContext::discover()` |

## Fossil Record

Historical snapshots are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:

| Snapshot | Contents |
|----------|----------|
| `experiments_pre_interstadial_may2026/` | 89 experiment sources before modern rewire |
| `harness_launcher_pre_interstadial_may2026/` | Harness + launcher before deprecation |
| `experiments_prokaryotic_may2026/` | 20 absorbed experiment sources before UniBin |
| `primal_gaps_phase60_may2026/` | Gap registry at Phase 60 ship |

## Evolution Path

```
Python baseline
  → Rust validation (Tier 1)
    → barraCuda CPU math
      → barraCuda GPU compute
        → fused TensorSession pipeline
          → sovereign dispatch (coralReef)
            → primal composition (proto-nucleate graph)
              → NUCLEUS deployment (biomeOS Neural API)
              → composition collapse (signal.dispatch + primal.announce)
                → sovereignty layer (membrane composition + content routing)
```
