# Wave 22 Composition Patterns — Primal + Spring Team Handoff

**Date:** May 18, 2026
**From:** primalSpring (coordination spring)
**To:** All primal teams, all spring teams
**Status:** Stadial gate cleared — patterns ready for absorption
**License:** AGPL-3.0-or-later

---

## Summary

Wave 22 is complete. All 13 primals are at zero debt, 14/14 are
stale-socket-clean, and the composition patterns are mature enough for
downstream absorption. This handoff documents what we learned, what
patterns to absorb, and what each team should evolve toward.

---

## 1. The Python → Rust → Primal Validation Pipeline

This is the canonical science validation pattern for the ecosystem.
Every spring should follow it:

```
Stage 1: Python baseline (peer-reviewed, reproducible)
    ↓ validate_parity()
Stage 2: Rust port (ecoBin compliant, zero C deps)
    ↓ capability_call() via IPC
Stage 3: Primal composition (NUCLEUS, live deployment)
```

**For primalSpring**, this is recursive: the "science" is coordination itself.
The Python baseline is the spec. The Rust port is the implementation.
The Primal composition is the live ecosystem (biomeOS + 13 primals).

**Key patterns to absorb:**
- `CompositionContext::discover()` — runtime capability discovery
- `validate_parity()` — Python vs Rust numerical agreement
- `capability_call()` — JSON-RPC 2.0 over UDS with BTSP encryption
- `ValidationResult` builder with `.with_provenance()` metadata

---

## 2. NUCLEUS Composition and Deployment via NeuralAPI

### Atomic Model (Phase 32)

```
Tower (3):  bearDog + songbird + skunkBat     [trust boundary]
Node  (6):  Tower + toadStool + barraCuda + coralReef  [compute]
Nest  (7):  Tower + nestGate + rhizoCrypt + loamSpine + sweetGrass  [storage + provenance]
NUCLEUS (13): Node ∪ Nest + squirrel + biomeOS + petalTongue  [full composition]
```

### biomeOS NeuralAPI Deployment

The canonical deployment path through biomeOS:

1. **`primal.announce`** — single-RPC atomic registration
   (lifecycle + capabilities + translations + signal tiers).
   Falls back to legacy 3-call pattern when biomeOS < v3.57.

2. **`composition.deploy(graph)`** — deploy graph execution.
   biomeOS resolves fragment references, discovers primals by capability,
   and orchestrates the composition.

3. **`signal.dispatch`** — signal-tier interception on `capability.call`.
   biomeOS v3.56+ intercepts capability calls and routes through signal
   tiers when available.

4. **`capability.call`** — the fundamental composition primitive.
   Consumer calls capability; biomeOS routes to the primal that provides it.

### Atomic Instantiation

Deploy graphs use fragment-first composition with `resolve = true`:

```toml
[[graph.fragments]]
ref = "fragments/tower.toml"
resolve = true

[[graph.nodes]]
primal = "biomeos"
by_capability = "orchestration"
```

biomeOS resolves fragment references to concrete primal binaries at deploy
time, based on capability discovery. This means graphs are portable across
different hardware and network topologies.

**94 deploy graphs** are defined in primalSpring, serving as the canonical
templates for ecosystem deployment (80 deploy + 14 atomic signal graphs).

---

## 3. Socket Hygiene — What We Learned

### The Problem

wetSpring observed 50+ stale biomeOS sockets and 100+ stale songbird sockets
from prior crashes. Discovery succeeded (file exists) but connections failed
(ECONNREFUSED), costing ~100ms per attempt.

### The Solution (3 layers)

**Layer 1 — Server side (all primals):**
- `unlink()` before `bind()` at every UDS bind site
- Shutdown cleanup: remove socket on SIGINT/SIGTERM and Drop
- **14/14 primals confirmed clean** after ecosystem sweep

**Layer 2 — Consumer side (primalSpring):**
- `socket_is_alive(path)` — connect-probe with 50ms timeout
- `DEAD_SOCKET_CACHE` — process-level negative cache
- Replaces `path.exists()` across all 6 discovery tiers

**Layer 3 — Infrastructure (plasmidBin):**
- `doctor.sh` — stale socket detection (fuser + python3 fallback)
- `stop_gate.sh` — post-kill socket cleanup
- `start_primal.sh` — pre-start stale socket removal

### Pattern for Springs

```python
def socket_is_alive(path: str, timeout: float = 0.05) -> bool:
    if not os.path.exists(path):
        return False
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(timeout)
        s.connect(path)
        s.close()
        return True
    except (ConnectionRefusedError, FileNotFoundError, OSError):
        return False
```

---

## 4. Standards Updated

| Standard | Version | What Changed |
|----------|---------|--------------|
| `CAPABILITY_BASED_DISCOVERY_STANDARD.md` | v1.3.0 | §5: connect-probe liveness, §6: startup cleanup |
| `DEPLOYMENT_VALIDATION_STANDARD.md` | — | Stale socket hygiene section added |
| `CAPABILITY_WIRE_STANDARD.md` | — | `{capabilities, count, primal}` envelope (Wave 22) |

---

## 5. Per-Primal Evolution Summary

| Primal | Version | Wave 22 Highlights |
|--------|---------|-------------------|
| bearDog | 0.9.0 | Wave 106 socket prevention, 14,784+ tests |
| songbird | 0.2.1 | Socket hygiene confirmed, 7,178+ tests |
| toadStool | 0.2.0 | S264: 6/6 bind sites, `compute.fan_out`, 9,028 tests |
| biomeOS | 0.1.0 | v3.61: signal dispatch, `primal.announce`, socket hygiene |
| nestgate | 0.1.0 | S64: `capabilities.list` aligned, 8,915+ tests |
| squirrel | 0.1.0 | Socket cleanup, deployment guide, 7,178 tests |
| barraCuda | 0.4.0 | Sprint 70 stadial + transport.rs socket fix, 4,422+ tests |
| petalTongue | 1.6.6 | Socket cleanup + PID file, backend=nestgate live |
| rhizoCrypt | 0.14.0 | S69: `dag.partial_dehydrate`, hex acceptance, 1,637+ tests |
| loamSpine | 0.9.16 | `normalize_method` aliases, hex hash acceptance, 1,442+ tests |
| sweetGrass | 0.7.37 | Socket hygiene, TCP/BTSP gap closed, 1,522 tests |
| coralReef | 0.2.0 | FECS stability proof, eprintln→tracing, 4,506+ tests |
| skunkBat | 0.2.0 | Phase 3 audit forwarding, ipc cleanup, 363+ tests |

---

## 6. What Springs Should Absorb

### All Springs

1. **Connect-probe pattern** for trio/primal discovery (see §3)
2. **Session-level negative cache** for dead sockets
3. **`primal.announce`** for biomeOS registration (with legacy fallback)
4. **`CompositionContext`** pattern for capability-based probing
5. **Structured `ValidationResult`** with provenance metadata

### Domain-Specific

| Spring | Absorption Target |
|--------|-------------------|
| wetSpring | Socket pattern for 264-clone pipelines, `compute.fan_out` for parallelism |
| hotSpring | Sovereign boot abstraction (toadStool/coralReef GPU pipeline) |
| neuralSpring | Squirrel inference pipeline, NestGate weight persistence |
| airSpring | LTEE E3, gS L5+ |
| ludoSpring | coralReef shader IPC for game rendering |
| groundSpring | lithoSpore integration (B3+B4 ingested) |
| healthSpring | NestComposition facade, Foundation T10 gap |

---

## 7. Escalation Items (for records)

| Item | Owner | Status |
|------|-------|--------|
| skunkBat `seed_fingerprint` in plasmidBin manifest | plasmidBin CI | GAP — harvest pipeline should auto-populate |
| sourDough `Cargo.toml` version 0.1.0 vs docs 0.3.0 | sourDough team | Version bump missed |
| Composition gap #3: GPU API alignment (`submit_and_map`) | barraCuda / wetSpring | Open — HMMA awaits coralReef codegen |
| Composition gap #8: Cross-gate dispatch via songBird | songBird / biomeOS | Open (Phase 2) |
| `exp052_protocol_escalation` naming drift | primalSpring | Minor — folder name vs content mismatch |
| `results/*.json` dates (May 6-9) | primalSpring | Moderate — regenerate before notebook deps |

---

## 8. Next Waves

| Wave | Focus | Owner |
|------|-------|-------|
| 23 | wetSpring E2E study completion (Barrick 2009 live, Tenaillon 264) | wetSpring |
| 24 | Shadow run execution (TLS/NAT/content/auth parity proofs) | projectNUCLEUS |
| 25 | Primal-spring pairing depth (compute trio niche) | All springs |
| 26 | Remaining composition gaps (#3 GPU, #8 cross-gate) | barraCuda, songBird |
| 27 | projectFOUNDATION thread saturation (Thread 4 gap) | All springs |
