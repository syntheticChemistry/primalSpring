# Wave 44 — Neural API Announce Fix Blurbs

> **Date**: 2026-05-23 (post-Wave 43 audit)
> **Status**: **SUPERSEDED** (May 23, 2026) — All fixes deployed. 12/12 announcing primals compliant. Resolved in Wave 45.
> **Context**: 12/13 primals shipped `primal.announce` code in Wave 43.
> 3 are broken at wire level, 2 are inbound-only, 7 are working.
> sweetGrass, loamSpine, rhizoCrypt are reference-quality — use them as patterns.

---

## CRITICAL — Wire Identity Fix (biomeOS rejects payload)

### coralReef

**Bug**: `ecosystem.rs` sends `"name"` instead of `"primal"` in the JSON-RPC params.
biomeOS `PrimalAnnouncement` struct requires the field be called `primal`.
Current payload is silently rejected.

**Fix**:
```rust
// In send_primal_announce():
// BEFORE:
"name": "coralreef",
// AFTER:
"primal": "coralreef",
```

Also add a `methods` array — currently missing, so `methods_registered` will be 0.
Pull method names from your dispatch table constants.

**Reference**: rhizoCrypt `niche.rs` `announce_payload()` for clean pattern.

**Test**: Assert the payload contains `"primal"` (not `"name"`) and `"methods"` is non-empty.

---

### petalTongue

**Bug**: Same as coralReef — `announce_to_neural_api()` in `main.rs` sends `"name"` instead of `"primal"`.

**Fix**:
```rust
// BEFORE:
"name": "petaltongue",
// AFTER:
"primal": "petaltongue",
```

Also add a unit test for the payload shape — currently no test coverage on announce.

**Reference**: sweetGrass `neural_announce.rs` for test patterns (7 tests).

---

### skunkBat

**Bug**: `registration.rs` sends `"primal_id"` and `"name"` instead of `"primal"`.

**Fix**:
```rust
// BEFORE:
"primal_id": skunk_bat_core::PRIMAL_ID,
"name": skunk_bat_core::PRIMAL_NAME,
// AFTER:
"primal": skunk_bat_core::PRIMAL_ID,
```

Also:
- Expand method list from 5 to full 17 methods from `dispatch.rs` constants
- Add `pid` and `version` fields
- Add payload structure test (currently none)
- Fix `showcase/RUN_ALL.sh` — still references deleted tier 1-3 directories
- Update `showcase/README.md` to reflect tier-0-only state post-fossilization

---

## HIGH — Add Outbound Startup Announce

### songbird

**Current state**: Only has an **inbound** handler (`primal.announce` → builds response).
No outbound push to biomeOS neural-api on startup.

**What to add**:
1. After socket bind, send `primal.announce` to biomeOS neural-api socket
2. Use WAVE42 socket discovery: `$NEURAL_API_SOCKET` → `$XDG_RUNTIME_DIR/biomeos/neural-api-{family}.sock` → `/tmp` fallback
3. Fix capabilities/hints mismatch: `capabilities` currently contains 15 `network.*` tokens but `cost_hints` keys are `relay`/`communication`/`presence`. Either:
   - (a) Change `capabilities` to `["relay", "communication", "presence"]` to match hints, or
   - (b) Re-key hints to `network.relay`/`network.discovery`/etc. to match capabilities

**Reference**: sweetGrass `neural_announce.rs` — startup push + tiered socket discovery.

---

### barraCuda

**Current state**: `primal.announce` handler returns correct response, but no outbound startup push.

**What to add**:
1. After socket bind, push `primal.announce` to biomeOS neural-api
2. Reuse `discovery_socket_path()` for own socket; add neural-api socket resolution
3. Include full method list (87 methods) — currently handler includes them but startup doesn't push

**Reference**: bearDog `neural_registration.rs` — outbound wiring on both server paths.

---

## MEDIUM — Socket Discovery and Method Surface

### squirrel

**Current state**: Payload is correct, but `announce_to_neural_api()` reuses `find_biomeos_socket()` which targets the **orchestrator** socket (`biomeos.sock`), not the neural-api socket.

**Fix**: Add a separate `resolve_neural_api_socket()` that follows WAVE42 tiers:
1. `$NEURAL_API_SOCKET`
2. `$XDG_RUNTIME_DIR/biomeos/neural-api-{family}.sock`
3. `/tmp/biomeos/neural-api-{family}.sock`

Your own `discovery.rs` already checks `NEURAL_API_SOCKET` for inbound queries — reuse that logic for outbound announce.

Also update `CURRENT_STATUS.md` to document the outbound `primal.announce` capability.

**Reference**: sweetGrass `resolve_neural_api_socket()`.

---

### toadStool

**Current state**: Outbound announce works, but `ipc_surface::ANNOUNCED_METHODS` only contains `compute.*` methods. Capabilities claim `["compute", "science", "inference"]` but announced methods don't cover science or inference.

**Fix**: Either:
- (a) Expand `ANNOUNCED_METHODS` to include `science.*` and `inference.*` methods, or
- (b) Narrow capabilities to `["compute"]` only

Option (a) is preferred — toadStool does serve science and inference workloads.

---

## LOW — Minor Field Corrections

### bearDog

**Minor**: Sends `"signed_attestation"` — biomeOS expects `"attestation"`. The field is optional and currently `null`/unused, but when attestation verification is wired, the rename will matter.

```rust
// BEFORE:
"signed_attestation": null,
// AFTER:
"attestation": null,
```

---

### nestgate

**Informational**: Uses `["storage", "content"]` as capabilities while WAVE43 blurb specified `["storage", "dag", "replication"]`. This is a product-model choice (nestgate's primary surface is content-addressed storage), not a bug. When `dag.*` and `replication.*` methods are exposed via the announce method list, routing will still work at the method level.

No action required unless you want the routing weight table to show dag/replication as first-class capability domains.

---

## NO ACTION REQUIRED

### sweetGrass, loamSpine, rhizoCrypt

Reference-quality implementations. Full v3.68 schema, correct tiers, outbound wiring, substantive tests. Other teams should use these as patterns.

### biomeOS

Registration authority — receives, does not self-announce. v3.69 persistent weights and utilization tracking are live.

---

## Validation After Fixes

```bash
# Start biomeOS neural-api, then your primal. Check registration:
echo '{"jsonrpc":"2.0","method":"neural_api.routing_weights","params":{},"id":1}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/neural-api-ecoPrimal.sock

# After a few capability.call dispatches, check utilization:
echo '{"jsonrpc":"2.0","method":"neural_api.utilization","params":{},"id":2}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/neural-api-ecoPrimal.sock
```

If your primal name appears in routing_weights with non-default affinity, the announce is working. If `neural_api.utilization` shows your methods, end-to-end dispatch is operational.

---

## sourDough Scaffold Note

The scaffold generator correctly produces v3.68-shaped payloads but with empty
stub values. Teams using `sourdough scaffold` should populate `capabilities()`,
`signal_tiers()`, `cost_hints()`, and `latency_estimates()` before shipping.
The inbound handler also conflates capabilities with methods — align with the
outbound pattern (domain names for capabilities, method names for methods).

---

## Wave 44 Priority

| Priority | Teams | Fix |
|----------|-------|-----|
| **P0 — wire broken** | coralReef, petalTongue, skunkBat | `name`/`primal_id` → `primal` |
| **P1 — no outbound** | songbird, barraCuda | Add startup push to neural-api |
| **P2 — socket/methods** | squirrel, toadStool | Neural-api socket, expand methods |
| **P3 — minor** | bearDog | `signed_attestation` → `attestation` |

Once P0 fixes land, **10/13 primals** will have working outbound announce.
After P1, all 12 announcers will push on startup.
The Neural API routing layer becomes functional at ecosystem scale.
