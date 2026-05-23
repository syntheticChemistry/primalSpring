# Wave 45 — Remaining Upstream Work

> **Date**: 2026-05-23 (post-Wave 44 review, updated for biomeOS v3.70)
> **Status**: Active — only teams with remaining work
> **Ecosystem**: 11/13 primals have working outbound announce (squirrel + skunkBat fixed pre-blurb)
> **biomeOS**: v3.70 — attestation verification via BearDog now wired (was stub), persistent weights on startup, weight_health introspection
> **primalSpring**: v0.9.26 — 784 tests, 458 methods, 49 scenarios (S47-S49 live Neural API validation)

---

## songbird — Outbound Push + Capability Alignment

**Priority**: HIGH — last foundation primal without outbound startup announce

**Two issues remain**:

1. **No outbound startup push.** songbird has an inbound `primal.announce` handler
   (responds when queried) but never pushes to biomeOS neural-api on startup.
   Every other tower-tier primal (bearDog, skunkBat) now pushes on startup.

2. **Capabilities/hints key mismatch.** `capabilities` array contains 15 `network.*`
   tokens (`network.relay`, `network.discovery`, etc.) but `cost_hints` and
   `latency_estimates` use domain keys (`relay`, `communication`, `presence`).
   biomeOS seeds routing weights per capability domain — mismatched keys mean
   hints never attach to the right weights.

**Fix**:
```rust
// Option A (preferred): align capabilities to routing domains
"capabilities": ["relay", "communication", "presence"],
// cost_hints/latency_estimates already use these keys — done

// Option B: re-key hints to match capabilities
"cost_hints": { "network.relay": 15.0, "network.discovery": 10.0, ... }
// more verbose, less clean for routing
```

Then add outbound push after socket bind:
```rust
// After server.listen():
if let Some(neural_socket) = resolve_neural_api_socket() {
    announce_to_neural_api(&neural_socket, &own_socket).await;
}
```

**Socket discovery** — use the WAVE42 tiered lookup:
1. `$NEURAL_API_SOCKET`
2. `$XDG_RUNTIME_DIR/biomeos/neural-api-{family}.sock`
3. `/tmp/biomeos/neural-api-{family}.sock`

**Reference**: sweetGrass `neural_announce.rs` — cleanest outbound + discovery.

---

## ~~squirrel — Neural API Socket Targeting~~

**RESOLVED** — `a7753bac` (Wave 44) shipped `resolve_neural_api_socket()` with
5-tier lookup, connect-probe liveness, independent announce call, 4 tests,
and `CURRENT_STATUS.md` updated. 7,093 tests passing.

---

## bearDog — Attestation Field Name (Minor)

**Priority**: MEDIUM (elevated) — biomeOS v3.70 now delegates Ed25519
attestation verification to BearDog via `auth.verify_ionic` IPC. The field
mismatch means bearDog's own attestation won't verify when it self-announces.

**Issue**: Sends `"signed_attestation"` — biomeOS expects `"attestation"`.

**Fix**:
```rust
// BEFORE:
"signed_attestation": attestation_value,
// AFTER:
"attestation": attestation_value,
```

No urgency until the attestation verification path is wired in biomeOS.

---

## ~~skunkBat — Expand Method Surface~~

**RESOLVED** — `c9f4146` (Wave 44) expanded to all 17 methods, fixed wire
identity (`primal_id` → `primal`), tests verify full payload, showcase
`README.md` and `RUN_ALL.sh` cleaned for tier-0-only state.

---

## NO ACTION REQUIRED

| Primal | Status |
|--------|--------|
| sweetGrass | Reference quality — 7 tests, full schema |
| loamSpine | Reference quality — 4+mock tests, full schema |
| rhizoCrypt | Reference quality — 4 tests, semantic mappings |
| nestgate | Working — 6 tests, minor cap name choice |
| bearDog | Working — 3 tests, minor attestation field |
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

After songbird (HIGH) and bearDog attestation rename (MEDIUM) ship:

- **12/13 primals** with correct outbound `primal.announce` on startup
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
