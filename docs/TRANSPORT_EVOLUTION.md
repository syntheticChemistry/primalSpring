# Transport Evolution — UDS Convergence to Capability-Addressed Routing

**Date**: 2026-06-05 | **Wave**: 79 | **Author**: eastGate overwatch
**Status**: FORMALIZED — Phase 1 (UDS stadial gate) complete; Phase 2 in progress

---

## Problem Statement

Hardcoded TCP ports are the primary source of deployment friction in the
ecoPrimals ecosystem:

- **Port collisions** — 13 primals × N gates × M topologies = combinatorial explosion
- **Metadata leak** — open ports expose internal topology to the network
- **Non-isomorphic** — a graph that works on VPS breaks on desktop, ARM, container
- **Non-fractal** — nested NUCLEUS compositions (graph-defined products) collide
- **Platform-bound** — `0.0.0.0:9100` means nothing inside WASM or on a phone
- **O(n²) configuration** — every new gate multiplies the port coordination burden

The ecosystem needs a transport abstraction that makes primals **transport-ignorant**.
A primal calls `capability.call("security", "crypto.sign", payload)` and the
infrastructure delivers it — regardless of whether bearDog is on the same machine,
across a LAN, behind a WAN, or compiled to WASM.

---

## Evolution Phases

### Phase 0: Hardcoded TCP (Pre-Wave 77) — ELIMINATED

Every primal started with `--port <N> --bind 0.0.0.0`. Port numbers were
scattered across systemd units, launch scripts, env vars, and experiment
fixtures. Discovery probed TCP ports directly.

```
primal → --port 9100 --bind 0.0.0.0 → TCP listener → direct TCP connect
```

**Debt**: 13 `TCP_FALLBACK_*` constants, `PORT_REGISTRY`, per-primal `--bind`
flags, firewall rules for each port, `ss -tlnp` auditing.

### Phase 1: UDS Convergence (Wave 79) — COMPLETE ✓

Primals listen on Unix domain sockets. TCP is opt-in via explicit flag.
The `nucleus_launcher` defaults to `--tcp` opt-in (not `--uds-only` opt-out).
All composition graphs declare `transport = "uds_only"`. Deploy profiles
suppress port environment variables for UDS-only graphs. Discovery prefers
UDS socket probing over TCP.

```
primal → /run/membrane/{primal}.sock → UDS listener → ipc::discover
```

**What this proved**: Primals can function without TCP ports. The port
registry becomes historical metadata, not runtime configuration.

**Limits of UDS**:
- Unix-only (no Windows, different paths on macOS)
- Single-machine (cannot cross network boundaries)
- Path length limits (108 bytes Linux, 104 macOS)
- Not isomorphic (paths differ per platform/user/deployment)
- Naming convention fragility (nested NUCLEUS still needs path discipline)
- Songbird federation still requires TCP :7700 for cross-gate mesh

### Phase 2: Songbird-Routed IPC (Current) — IN PROGRESS

Primals register capabilities with Songbird at startup. All IPC flows
through `capability.call()` which Songbird resolves to the appropriate
transport. The primal never knows or chooses the transport.

```
primal → capability.call("security", "crypto.sign", payload)
           → Songbird resolves "security" → bearDog
           → Tower Atomic selects transport:
               same-process? → channel
               same-machine? → UDS / shared memory
               same-LAN?     → Songbird mesh relay
               cross-WAN?    → Songbird federation + BTSP tunnel
               WASM?         → message passing / postMessage
```

**Key contract**: Songbird's `ipc.register` + `ipc.resolve` become the
universal routing table. The `capability_registry.toml` is the compile-time
SSOT; Songbird is the runtime SSOT. They must agree.

**What changes**:
- `ipc::discover` evolves from "probe sockets/ports" to "ask Songbird"
- `CompositionContext::from_live_discovery()` becomes a Songbird query
- Deploy profiles declare capabilities, not sockets or ports
- Graph metadata drops `transport` field entirely (Songbird decides)
- ecoBin binaries work on any platform (no UDS path assumptions)

### Phase 3: Tower Transport Abstraction (Target)

The Tower Atomic composition model fully owns transport selection.
A primal is a pure function: capabilities in, capabilities out. The
Tower handles:

- **Discovery**: Songbird registry (runtime) + capability_registry.toml (compile)
- **Routing**: Capability → primal → physical location → transport
- **Security**: BTSP authentication on every hop (already done)
- **Failover**: If UDS is down, try mesh relay. If mesh is down, try federation.
- **Observability**: Transport choice is logged, not configured

```
[graph.nodes]
name = "beardog"
capabilities = ["security", "crypto.*", "btsp.*"]
# No port. No socket. No bind. No transport.
# Tower Atomic resolves everything.
```

**Fractal property**: A graph-defined product (ludoSpring, esotericWebb)
contains an inner NUCLEUS. The inner NUCLEUS's primals register with
the inner Songbird. The outer Songbird knows the inner product as a
single capability provider. Nesting is transparent.

**Isomorphic property**: The same graph TOML deploys identically on
VPS, desktop, Raspberry Pi, container, or WASM runtime. Only the
Tower's transport selector changes — the graph and primals are invariant.

---

## Transport Selection Matrix (Phase 3 Target)

| Topology | Transport | Selected By |
|----------|-----------|-------------|
| Same process (future: in-process primals) | Channel / fn call | Tower |
| Same machine, same NUCLEUS | UDS (Linux) / named pipe (Windows) / mach port (macOS) | Tower |
| Same machine, nested NUCLEUS | UDS with namespaced paths | Tower |
| Same LAN, bonded gate | Songbird mesh relay (mDNS discovered) | Songbird |
| Cross-WAN, federated gate | Songbird federation (:7700) + BTSP tunnel | Songbird |
| VPS peptidoglycan layer | Songbird federation (sovereign TLS) | Songbird |
| WASM / browser | postMessage / MessagePort | Tower (WASM adapter) |
| Mobile / Pixel | Platform IPC (Binder on Android, XPC on iOS) | Tower (platform adapter) |

**The O(n) property**: Adding gate N+1 requires only:
1. Start bearDog (identity)
2. Start Songbird (mesh)
3. `mesh.init` with one peer address
4. Songbird auto-discovers all other gates via gossip

No port configuration. No firewall rules. No transport selection.
The mesh self-organizes.

---

## Migration Path

### Already Done (Wave 79)
- [x] `nucleus_launcher` defaults UDS-only (`--tcp` opt-in)
- [x] `discover_with_fallback()` TCP gated behind `tcp_tier5_enabled()`
- [x] All graphs `transport = "uds_only"` + `tcp_ports = 0`
- [x] Deploy profiles suppress port env for UDS-only graphs
- [x] `show_status` UDS-first probing
- [x] VPS firewall closes standalone primal ports
- [x] BD-TRUST-01 resolved (zero-operator trust seeding)
- [x] Songbird deep debt: hardcoded ports → constants

### Next (Wave 80+)
- [ ] Songbird `ipc.resolve` returns transport-qualified endpoints
- [ ] `CompositionContext` queries Songbird instead of probing sockets
- [ ] `nucleus_launcher` registers capabilities with Songbird at spawn time
- [ ] Graph metadata evolves: `transport` field deprecated → capability-only
- [ ] `tolerances::PORT_REGISTRY` annotated as legacy/standalone-only
- [ ] Cross-platform transport adapter trait in ecoPrimal
- [ ] ecoBin compliance: single binary, no platform transport assumptions

### Stadial Gate (Required for Stadial Entry)
- [ ] 3+ gate Plasmodium collective via capability-addressed routing
- [ ] Zero port configuration in any gate's deployment
- [ ] VPS binary refresh with UDS-native nestgate/toadstool/skunkbat
- [ ] `s_transport_evolution` validation scenario (Phase 2 checks)

---

## Invariants

1. **Primals never choose transport.** A primal provides capabilities and
   consumes capabilities. How they connect is not its concern.
2. **Songbird is the runtime routing SSOT.** If Songbird is down, the
   composition is degraded — this is correct, not a bug.
3. **TCP is federation-only.** The only legitimate TCP port in Tower Atomic
   is Songbird's federation port (:7700). Everything else is UDS or
   Songbird-routed.
4. **ecoBin compliance requires transport ignorance.** A binary that
   hardcodes `TcpListener::bind` or `UnixListener::bind` is not ecoBin
   compliant. Transport is injected by the Tower, not chosen by the primal.
5. **Capability addressing is O(n).** Adding a gate adds one mesh peer,
   not N port configurations.

---

*"The primal doesn't know how it's connected. It only knows what it can do."*
