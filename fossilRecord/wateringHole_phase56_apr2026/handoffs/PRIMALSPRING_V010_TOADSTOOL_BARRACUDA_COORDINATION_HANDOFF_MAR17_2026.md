# Handoff: primalSpring v0.1.0 → toadStool / barraCuda — Coordination Validation Intelligence

**Date:** March 17, 2026  
**From:** primalSpring (coordination validation spring)  
**To:** toadStool (hardware dispatch), barraCuda (math primitives), coralReef (shader compiler)  
**License:** AGPL-3.0-or-later  
**Covers:** primalSpring v0.1.0 Phase 0→1

---

## Executive Summary

- primalSpring validates the coordination layer — it does NOT consume barraCuda math
- Zero direct dependency on barraCuda, toadStool, or coralReef crates
- All interaction is via IPC: `discover_primal()` + `PrimalClient` over Unix sockets
- 10 experiments directly exercise toadStool/barraCuda/coralReef coordination patterns
- The compute triangle (exp050) is the canonical integration test for the trio
- primalSpring's IPC patterns can inform toadStool/barraCuda JSON-RPC evolution

---

## What primalSpring Tests About toadStool/barraCuda/coralReef

### Direct Experiments

| Exp | What | Primals Exercised |
|-----|------|-------------------|
| 002 | Node Atomic bootstrap | Tower + toadStool: GPU dispatch via capability routing |
| 010 | Sequential graph | biomeOS coordinates toadStool in a dependency chain |
| 011 | Parallel graph | Concurrent toadStool capability calls |
| 012 | ConditionalDag graph | GPU dispatch with CPU fallback when toadStool unavailable |
| 014 | Continuous 60Hz tick | toadStool under sustained compute load |
| 025 | coralForge pipeline | coralReef compile → toadStool dispatch → barraCuda execute |
| 050 | Compute triangle | **Canonical test**: coralReef → toadStool → barraCuda live pipeline |
| 051 | Socket discovery sweep | Enumerate all primal sockets including toadStool/barraCuda |
| 052 | Protocol escalation | HTTP → JSON-RPC → tarpc negotiation (toadStool/NestGate) |
| 055 | Wait-for-health | Health probe pattern applied to toadStool startup ordering |

### IPC Conventions Validated

1. Socket discovery: `$TOADSTOOL_SOCKET` → `$XDG_RUNTIME_DIR/biomeos/toadstool-default.sock` → temp_dir fallback
2. JSON-RPC 2.0 with `AtomicU64` unique request IDs
3. `health.check` / `compute.dispatch` / `discovery.topology` methods
4. Graceful degradation when toadStool unavailable (exp012 ConditionalDag)

### What Other Springs Tell Us

| Spring | toadStool/barraCuda Pattern |
|--------|----------------------------|
| wetSpring V125 | 354 binaries, `compute.dispatch.*` direct dispatch |
| hotSpring v0.6.31 | coralReef sovereign compile 46/46, VFIO PBDMA |
| airSpring v0.8.7 | 6 local WGSL → upstream absorption pipeline |
| groundSpring V110 | 102 delegations (61 CPU + 41 GPU) |
| ludoSpring V22 | `compute.dispatch.*` direct dispatch, game real-time |
| healthSpring V30 | 6 WGSL shaders + 3 ODE→WGSL codegen |
| neuralSpring S163 | `DispatchOutcome`, circuit breaker |

See ecosystem-level copy at `wateringHole/handoffs/PRIMALSPRING_V010_TOADSTOOL_BARRACUDA_COORDINATION_HANDOFF_MAR17_2026.md` for full details.

---

*primalSpring: zero math, zero shaders, pure IPC coordination validation.*
