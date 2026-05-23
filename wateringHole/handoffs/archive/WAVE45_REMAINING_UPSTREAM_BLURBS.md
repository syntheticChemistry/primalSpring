# Wave 45 — Remaining Upstream Work

> **Date**: 2026-05-23 (post-Wave 44 review, updated for biomeOS v3.70)
> **Status**: **ALL RESOLVED** — 12/13 primals with correct outbound announce (biomeOS exempt)
> **Ecosystem**: Full `primal.announce` compliance achieved across all announcing primals
> **biomeOS**: v3.70 — attestation verification via BearDog now wired, persistent weights on startup, weight_health introspection
> **primalSpring**: v0.9.26 — 784 tests, 458 methods, 49 scenarios (S47-S49 live Neural API validation)

---

## ~~songbird — Outbound Push + Capability Alignment~~

**RESOLVED** — commit `4a8f4cdc` (May 23) shipped both fixes:

1. **Outbound startup push**: `neural_announce::spawn_announce()` fires at line 118
   of `connection.rs` after socket bind. Uses WAVE42 tiered discovery
   (`$NEURAL_API_SOCKET` → XDG → `/tmp`).
2. **Capabilities aligned to routing domains**: `ROUTING_CAPABILITIES = ["relay", "communication", "presence"]`
   — matches `cost_hints` and `latency_estimates` keys. Routing weights now attach correctly.

---

## ~~squirrel — Neural API Socket Targeting~~

**RESOLVED** — `a7753bac` (Wave 44) shipped `resolve_neural_api_socket()` with
5-tier lookup, connect-probe liveness, independent announce call, 4 tests,
and `CURRENT_STATUS.md` updated. 7,093 tests passing.

---

## ~~bearDog — Attestation Field Name~~

**RESOLVED** — `2a94f2d6d` (Wave 111) already renamed `signed_attestation` →
`attestation` in `neural_registration.rs` (line 212). Verified: `primal.announce`
path sends the correct field. bearDog appears as verified in
`neural_api.weight_health` with biomeOS v3.70 attestation verification live.
The `capability.register` path retains its own schema — separate concern.
Confirmed by bearDog team: no further changes needed.

---

## ~~skunkBat — Expand Method Surface~~

**RESOLVED** — `c9f4146` (Wave 44) expanded to all 17 methods, fixed wire
identity (`primal_id` → `primal`), tests verify full payload, showcase
`README.md` and `RUN_ALL.sh` cleaned for tier-0-only state.

---

## ALL PRIMALS — STATUS

| Primal | Status |
|--------|--------|
| sweetGrass | Reference quality — 7 tests, full schema |
| loamSpine | Reference quality — 4+mock tests, full schema |
| rhizoCrypt | Reference quality — 4 tests, semantic mappings |
| nestgate | Working — 6 tests, minor cap name choice |
| bearDog | Fixed in Wave 111 — attestation renamed, verified by biomeOS v3.70 |
| songbird | Fixed May 23 — outbound push + capability alignment (4a8f4cdc) |
| toadStool | Fixed in S271 — expanded methods |
| barraCuda | Fixed in Wave 44 — full outbound 246-line module |
| coralReef | Fixed in Wave 44 — wire identity + methods |
| petalTongue | Fixed in Wave 44 — wire identity + 82 lines tests |
| skunkBat | Fixed in Wave 44 — all 17 methods, showcase cleaned |
| squirrel | Fixed in Wave 44 — neural-api socket, 4 tests, 7093 tests |
| biomeOS | Registration authority — no announce needed |
| sourDough | Scaffold generator — templates correct v3.68 shape |

---

## Ecosystem Convergence

All upstream items resolved. Current ecosystem state:

- **12/13 primals** with correct outbound `primal.announce` on startup (biomeOS exempt)
- **biomeOS v3.70** persistent routing weights survive restarts (redb-backed)
- **Attestation verification** live via BearDog `auth.verify_ionic` IPC
- **`neural_api.utilization`** tracks hot/cold methods ecosystem-wide
- **`neural_api.weight_health`** provides convergence diagnostics + circuit breaker status
- **`neural_api.routing_weights`** shows scored providers per capability
- The Neural API becomes a **functional adaptive routing layer** — not just a registry

### primalSpring Live Validation (S47-S49)

primalSpring now validates the Neural API stack end-to-end:
- **S47 Neural Dispatch Live**: `NeuralDispatcher.dispatch()` routes `crypto.hash` → bearDog, `storage.store` → nestgate through biomeOS
- **S48 Observatory Parity**: routing_weights, route_explain, composition_patterns, plan_tier, weight_health cross-referenced against local model
- **S49 Feedback Loop**: dispatch_instrumented 5x, verify metrics accumulate, utilization tracking, routing weight convergence

### Future Waves

- **Layer 5**: Learned routing (neural network weights from operational data)
- **Cross-gate routing**: FlockGate distributed dispatch via TURN relay
- **Primals re-announcing** on capability change at runtime
- **Node/Nucleus/Meta observatory elevation**: plan_tier validation for all tiers
- **Graph execution validation**: full `graph.execute` of named patterns (rootpulse_commit)
