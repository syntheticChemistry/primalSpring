# Wave 45 — Remaining Upstream Work

> **Date**: 2026-05-23 (post-Wave 44 review)
> **Status**: Active — only teams with remaining work
> **Ecosystem**: 10/13 primals have working outbound announce

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

## squirrel — Neural API Socket Targeting

**Priority**: MEDIUM — announce payload is correct but delivered to wrong socket

**Issue**: `announce_to_neural_api()` reuses `find_biomeos_socket()` which targets
the **orchestrator** socket (`biomeos.sock`), not the neural-api socket
(`neural-api-{family}.sock`). In a split-socket deployment, the announce
reaches the orchestrator but not the routing layer.

squirrel's own `discovery.rs` already checks `$NEURAL_API_SOCKET` for inbound
queries — the same logic should be used for outbound announce.

**Fix**: Add `resolve_neural_api_socket()` following WAVE42 tiers and call it
from the announce path instead of `find_biomeos_socket()`:

```rust
fn resolve_neural_api_socket() -> Option<PathBuf> {
    // 1. Explicit override
    if let Ok(p) = std::env::var("NEURAL_API_SOCKET") {
        let path = PathBuf::from(&p);
        if path.exists() { return Some(path); }
    }
    // 2. XDG runtime
    let family = std::env::var("ECOPRIMALS_FAMILY_ID")
        .unwrap_or_else(|_| "ecoPrimal".into());
    let xdg = dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let candidate = xdg.join("biomeos").join(format!("neural-api-{family}.sock"));
    if candidate.exists() { return Some(candidate); }
    // 3. /tmp fallback
    let fallback = PathBuf::from(format!("/tmp/biomeos/neural-api-{family}.sock"));
    if fallback.exists() { return Some(fallback); }
    None
}
```

Also update `CURRENT_STATUS.md` to document the outbound `primal.announce`.

---

## bearDog — Attestation Field Name (Minor)

**Priority**: LOW — field is currently null/unused, becomes relevant when
biomeOS wires attestation verification

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

## skunkBat — Expand Method Surface (Minor)

**Priority**: LOW — announce works, but only 5 of 17 methods are listed

**Issue**: `neural_announce` lists 5 `security.*` methods while dispatch exposes 17.
Routing weights and utilization tracking only cover the announced subset.

**Fix**: Pull method names from dispatch constants instead of hardcoding:
```rust
"methods": crate::dispatch::ALL_METHODS,  // full 17-method surface
```

Also: update `showcase/README.md` and `showcase/RUN_ALL.sh` to reflect
tier-0-only state after the Wave 34 fossilization.

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
| biomeOS | Registration authority — no announce needed |
| sourDough | Scaffold generator — templates correct v3.68 shape |

---

## Ecosystem Convergence

After songbird (HIGH) and squirrel (MEDIUM) fix:

- **12/13 primals** with correct outbound `primal.announce` on startup
- **biomeOS v3.69** persistent routing weights accumulate across restarts
- **`neural_api.utilization`** tracks hot/cold methods ecosystem-wide
- **`neural_api.routing_weights`** shows scored providers per capability
- The Neural API becomes a **functional routing layer** — not just a registry

Future waves can then focus on:
- **Layer 5**: Learned routing (neural network weights from operational data)
- **Cross-gate routing**: FlockGate distributed dispatch
- **Primals re-announcing** on capability change at runtime
- **Attestation verification** via bearDog signed announcements
