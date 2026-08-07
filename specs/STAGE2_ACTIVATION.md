# Stage 2 Activation — primalSpring Validation Contract

**Owner**: primalSpring (validation) + biomeOS (runtime)
**Status**: ACTIVE — N1 DONE, N2-N6 pending
**Wave**: 157a (Aug 7, 2026)
**Depends on**: G64 COMPLETE, G65 COMPLETE, G66 COMPLETE

---

## Prerequisites: The Cephalization Trilogy

Three glacial goals form the foundation Stage 2 builds on. All are COMPLETE
across all 15 primals as of Wave 156:

| Goal | What | Status | primalSpring validation |
|------|------|--------|----------------------|
| **G64** | Cephalization — tarpc convergent evolution | **COMPLETE 15/15** | exp113 baseline audit, `s_cephalization_audit` |
| **G65** | Protocol negotiation — single-socket dual-protocol | **COMPLETE 15/15** | exp052 structural, `s_protocol_escalation` |
| **G66** | Transport abstraction — silicon-agnostic IPC | **COMPLETE 15/15** cross-arch | exp094 parity, `s_cross_arch_bonding` |

These gave every primal: tarpc service traits (G64), single-socket JSON-RPC +
tarpc negotiation (G65), and `TransportEndpoint`/`TransportStream` abstraction
that works across UDS, named pipes, and TCP (G66).

---

## G67: Neural API Activation

**Goal**: biomeOS Neural API becomes the sole routing substrate for all primal
communication within a gate. Consumers call capabilities, not sockets.

### Three Stages

```
STAGE 1 — PRIMORDIAL (bootstrap only after Stage 2 activation)
  Consumer → UDS socket path → Primal
  Manual wiring. Per-gate config. Jelly string prone.

STAGE 2 — NEURAL API ROUTING (current activation target)
  Consumer → neural-api-default.sock → capability.call → Primal
  Capability discovery. Graph execution. Isomorphic deployment.

STAGE 3 — BIOMEOS AS OS (glacial)
  biomeOS IS the operating system.
  Primals are processes. Capability registry is the process table.
```

### Stage 2 Properties

**Isomorphic deployment**: A single `graph.execute(nucleus_complete)` deploys
identically on any gate. Same graph, different hardware, same composition
semantics.

**Fractal self-similarity**: Each gate is a self-similar NUCLEUS. A Steam Deck
runs the same graphs as a 128 GB Threadripper — the Neural API routes to
available capabilities.

**Jelly string elimination**: No consumer ever imports a socket path.
`capability.call("crypto", "sign_ed25519", {...})` resolves at runtime.

**Deprecation boundary**: Stage 1 is fully primordial when no consumer on any
gate calls a primal socket directly. Direct socket paths exist only in:
- Neural API's own discovery layer (scans sockets to build the registry)
- Bootstrap mode (before Neural API is alive)

---

## Activation Tasks — primalSpring Perspective

| ID | Task | Validates | Experiment anchors | Status |
|----|------|-----------|-------------------|--------|
| **N1** | Forwarding fix (tarpc/BTSP fast-fail) | `capability.call` dispatch uses pooled JSON-RPC, skips heavy escalation | biomeOS commit `ffed2c5b` | **DONE** |
| **N2** | `capability.call` routes to bearDog | Security domain → bearDog via Neural API socket | exp075, exp087, exp091 | **NEXT** |
| **N3** | Tower Atomic routing | crypto + mesh + defense domains compose through Neural API | exp001, exp060, exp094, exp112 | PENDING |
| **N4** | Provenance Trio routing (rootPulse) | dag + spine + braid domains route to rhizoCrypt + loamSpine + sweetGrass | exp020, exp041, exp094, trio_ops | PENDING |
| **N5** | Node Atomic routing | compute + shader + tensor domains route to toadStool + barraCuda + coralReef | exp117 (new), exp050 | PENDING |
| **N6** | Deploy to production gates | Stage 2 live on sporeGate, westGate, strandGate | gate team handoffs | BLOCKED on N2-N5 |

### N2 Verification Protocol

The first live test. Proves `capability.call` forwards through the Neural API
to a real primal and returns a real response (not a stub).

```
1. Neural API server running on biomeos-neural.sock
2. bearDog running on beardog-default.sock
3. Call: capability.call("crypto", "sign_ed25519", {"data": "dGVzdA=="})
4. Neural API resolves "crypto" → bearDog via routing table
5. Forwards JSON-RPC to beardog-default.sock
6. Returns Ed25519 signature (real crypto, not stub)
7. Latency: <50ms for local UDS round-trip
```

Validated by:
- `exp091_primal_routing_matrix` — L0 matrix across 10 domains
- `exp087_neural_api_routing_e2e` — full domain sweep with latency checks
- `exp075_biomeos_neural_api_live` — NeuralBridge smoke test
- `biomeOS/scripts/neural-api-test.sh` — shell-level socat harness

### N3-N5 Verification Protocol

Same pattern as N2, extended to atomic compositions:

- **N3 (Tower)**: bearDog + songBird + skunkBat. Three domains (crypto,
  mesh, defense) compose through Neural API. Tower quorum = 3.
- **N4 (Provenance Trio)**: rhizoCrypt + loamSpine + sweetGrass. Session-scoped
  commit: DAG events → `session.commit` → braid. Trio quorum = 3 + bearDog signing.
- **N5 (Node)**: toadStool + barraCuda + coralReef. Compute trio IPC:
  `shader.compile.wgsl` → `compute.dispatch.submit` → result + BLAKE3 witness.

---

## primalSpring Primordial Debt Boundary

Stage 2 activation defines a clear boundary for primalSpring's own code:

### Must be post-primordial (no direct sockets)

- All experiments targeting N2-N6 validation
- `CompositionContext::discover()` — already post-primordial (5-tier escalation)
- `NeuralBridge` — already post-primordial (runtime JSON-RPC)
- `NeuralDispatcher` — already post-primordial (tier-aware dispatch)

### Gated behind `primordial-compat` feature

- `AtomicHarness` / `RunningAtomic` (direct process spawn)
- `spawn_primal` / `spawn_biomeos` (launcher module)
- `signal_accept.rs` (pre-primordial signal handling)
- `probe_primal` / `probe_primal_at_socket` (direct socket probes)
- Experiments using direct `tcp_rpc` to hardcoded ports

### Retained for bootstrap

- `CompositionContext::from_live_discovery_with_fallback()` — structural-only
  fallback when NUCLEUS is not running (returns capability domains from TOML,
  not from live discovery)
- Tier 5 TCP probing (gated behind `PRIMALSPRING_TCP_TIER5=1`, debug only)

---

## Cross-References

| Document | Relationship |
|----------|-------------|
| `infra/wateringHole/specs/NEURAL_API_ACTIVATION_SPEC.md` | Canonical G67 spec (ecosystem-wide) |
| `specs/NEURAL_API_EVOLUTION.md` | primalSpring's evolutionary model (L0-L5) |
| `specs/COMPOSITION_BROKER.md` | subGen evidence: 704 caps, COORDINATED mode |
| `specs/MIXED_COMPOSITION_PATTERNS.md` | Particle model + L0-L3 validation layers |
| `specs/NUCLEUS_LAB_INTEGRATION.md` | benchScale/agentReagents for isolated testing |
