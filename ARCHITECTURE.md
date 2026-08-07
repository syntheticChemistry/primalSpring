# primalSpring Architecture

## NUCLEUS Evolution Arena

primalSpring is a **NUCLEUS evolution arena** — it validates how primals
compose, not domain science. It is NOT a primal: it does not serve on a
socket, does not register with biomeOS, and does not appear in NUCLEUS
compositions. It is a pure CLI tool + IPC client that probes live
NUCLEUS deployments from the outside.

```
┌─────────────────────────────────────────────────────────────────┐
│                    primalspring UniBin                           │
│                     (NUCLEUS evolution arena)                    │
│                                                                 │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────┐  │
│  │  Certification    │  │  Validation      │  │  IPC Client  │  │
│  │  Engine           │  │  Scenarios       │  │  (probing)   │  │
│  │                   │  │                  │  │              │  │
│  │  L0: Bare         │  │  197 scenarios   │  │  JSON-RPC    │  │
│  │  L0.5: Seed       │  │  across 14 tracks│  │  2.0 client  │  │
│  │  L1: Discovery    │  │  (3 tiers)       │  │              │  │
│  │  L1.5: BTSP       │  │                  │  │  Composition │  │
│  │  L2: Health       │  │  Tier 1: Rust    │  │  Context     │  │
│  │  L3: Parity       │  │  Tier 2: Live    │  │              │  │
│  │  L4: Pipeline     │  │                  │  │  capability  │  │
│  │  L5: Bonding      │  │  ScenarioMeta    │  │  discovery   │  │
│  │  L6: Crypto       │  │  + provenance    │  │  + probing   │  │
│  │  L7: Cellular     │  │  + track/tier    │  │              │  │
│  │  L8: Lifecycle    │  │  classification  │  │              │  │
│  └──────────────────┘  └──────────────────┘  └──────────────┘  │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    primalspring library                    │   │
│  │  composition · coordination · bonding · deploy · ipc      │   │
│  │  validation · certification · tolerances                  │   │
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
| `validation/scenarios/` | 197 validation scenarios (14 tracks, 3 tiers: Rust/Live/Both) |
| `validation/scenarios/registry.rs` | `ScenarioMeta`, `ScenarioRegistry`, `Tier`, `Track` |
| `composition/` | `CompositionContext` — 5-tier discovery, IPC calls, BTSP |
| `coordination/` | `AtomicType`, composition validation (legacy probes removed Wave 32) |
| `bonding/` | `BondType`, `BondingPolicy`, `BtspEnforcer`, `TrustModel` |
| `deploy/` | Graph parsing, validation, structure |
| `ipc/` | JSON-RPC protocol, `PrimalClient`, `NeuralBridge`, discovery |
| `ipc/method_gate.rs` | MethodGate (JH-0) validation (validates primals have auth wired) |
| `tolerances.rs` | Named, centralized tolerance constants |
| `evolution/` | Silicon-agnostic evolution: `Target`, `ArchFitness`, `GateMatrix`, `CytoplasmZone`, `EcosystemConvergence` |
| `evolution/gate.rs` | Gate readiness tracking, zone model, enrollment status, `local_assessment()` |
| `evolution/convergence.rs` | Drift detection (`DriftSignal`), severity, ecosystem convergence scoring |

### Binaries

| Binary | Purpose | Status |
|--------|---------|--------|
| `primalspring_unibin` | Arena CLI (certify + validate + status + version + checksums + registry + release + nucleus) | Active |
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

`CompositionContext::discover()` uses 5-tier escalation — **capability-first**:

1. **Songbird routing** — `ipc.resolve({"capability": cap})` with `primal_id` fallback
2. **Neural API** — `NeuralBridge::capability_call()` via biomeOS (the canonical post-primordial consumer API)
3. **UDS convention** — `$XDG_RUNTIME_DIR/biomeos/{primal}-{fid}.sock`
4. **Socket registry scan** — enumerate known socket paths
5. **TCP probing** — opt-in debug-only (`PRIMALSPRING_TCP_TIER5=1`), disabled in release builds

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

## Cytoplasm Zone Model (K-Derm Topology)

The K-Derm cytoplasm is segmented into physical zones by switching fabric.
Gates in the same zone share L2 connectivity; cross-zone traffic traverses
backbone links or WireGuard overlay.

```
         Hub 1 (Backbone)            Target: three-hub triangle
        CRS310 + sporeGate           with redundant paths.
       eastGate, northGate,          Any single leg failure
       ironGate (10G fabric)         routes through other two.
          /           \
    leg A/             \leg B (LIVE, 80m AOC 10G)
        /               \
   Hub 3 (Garage)----Hub 2 (House2)
   planned          leg C    Omada SX3008F (standalone L2)
                    planned  + GL.iNet Flint 2 (OpenWrt WiFi)
                             strandGate, southGate, swiftGate, fieldGate
```

| Zone | Hub | Fabric | Gates |
|------|-----|--------|-------|
| `Backbone` | 1 | CRS310 (10G) | sporeGate, eastGate, northGate, ironGate |
| `House2` | 2 | Omada SX3008F | strandGate, southGate, swiftGate, fieldGate |
| `Garage` | 3 | planned | (future compute) |
| `Wan` | — | Internet/VPS | golgi, pepti, flockGate |

`CytoplasmZone::for_gate()` auto-derives zone from gate name.

## Deprecated Patterns (Fossilized)

These patterns are deprecated. Removal is deferred until stadial entry to
preserve backward compatibility for springs still referencing the harness API.

| Pattern | Replacement | Retention note |
|---------|-------------|----------------|
| `AtomicHarness` / `RunningAtomic` | plasmidBin ecoBin deployment via biomeOS | ~600 LOC in `ecoPrimal/src/harness/`. Compiled and tested. Wave 49: retained for compat; removal deferred to stadial entry. |
| `spawn_primal` / `spawn_biomeos` | plasmidBin ecoBin deployment | Same module — coupled to `AtomicHarness` lifecycle. |
| `probe_primal` / `check_capability_health` | `CompositionContext.health_check()` | Still referenced in `s_coordination_api.rs` scenario + `handlers.rs`. Remove at stadial after scenario migration. |
| `validate_composition` | `CompositionContext.call()` | No direct callers remain. Safe to remove at stadial. |
| `PrimalClient::connect` (direct) | `CompositionContext.client_for()` | Still used in `ipc/neural_bridge.rs`, `composition/btsp.rs`, harness, tests. Migrate callers at stadial. |
| `CompositionContext::from_running` | `CompositionContext::discover()` | Called from `harness/mod.rs`. Remove with harness at stadial. |

## Fossil Record

Historical snapshots are preserved in the [fossilRecord repository](https://github.com/ecoPrimals/fossilRecord) under `springs/primalSpring/`:

| Snapshot | Contents |
|----------|----------|
| `experiments_pre_interstadial_may2026/` | 89 experiment sources before modern rewire |
| `harness_launcher_pre_interstadial_may2026/` | Harness + launcher before deprecation |
| `experiments_prokaryotic_may2026/` | 20 absorbed experiment sources before UniBin |
| `primal_gaps_phase60_may2026/` | Gap registry at Phase 60 ship |

## Neural API Dispatch Architecture (Stage 2)

G64+G65+G66 COMPLETE. G67 Neural API activation in progress (N1 DONE).

```
Consumer / experiment
    │
    ├── CompositionContext.call(domain, method, params)
    │       │
    │       ├── signal.dispatch (preferred — atomic composition)
    │       └── capability.call (fallback — single method)
    │               │
    │               └── NeuralBridge → biomeos-neural.sock
    │                       │
    │                       └── Neural API Router
    │                           ├── Routing table (490+ methods)
    │                           ├── Translation registry
    │                           └── Graph executor (for composition signals)
    │                                   │
    │                                   └── Primal sockets (UDS)
    │
    └── NeuralDispatcher.dispatch(method, params)
            │
            ├── Tier-aware routing (Tower/Node/Nest/Nucleus/Meta)
            ├── Metrics collection (latency, success, route path)
            └── Bridge outcome ingestion → adaptive routing weights
```

See `specs/STAGE2_ACTIVATION.md` for the N1-N6 validation contract.

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
                  → G64-G66 cephalization trilogy (COMPLETE)
                    → G67 Stage 2 activation (ACTIVE — N1 DONE)
```
