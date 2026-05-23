# Wave 43 — Neural API `primal.announce` Adoption Blurbs

> **Date**: 2026-05-22 (post-Wave 42)
> **Status**: **SUPERSEDED** (May 23, 2026) — All 12/12 announcing primals compliant. Resolved in Waves 44–45.
> **Reference**: `WAVE42_NEURAL_API_DEPLOYMENT_GUIDE.md` for full wire schema
> **biomeOS**: v3.69 (persistent weights, utilization tracking)
> **primalSpring**: v0.9.26 (779 tests, 457 methods, 46 scenarios)

---

## songbird — Fix Schema + Add Hints

**Priority**: HIGH — handler exists but schema mismatch blocks routing data

**What exists**: `primal.announce` handler using `provided_capabilities` key

**What to do**:
1. Rename `provided_capabilities` → `capabilities` (biomeOS expects this exact key)
2. Add `socket` field (required) — full path to your UDS, e.g. `$XDG_RUNTIME_DIR/biomeos/songbird-ecoPrimal.sock`
3. Add `cost_hints`: `{ "relay": 15.0, "communication": 10.0, "presence": 5.0 }`
4. Add `latency_estimates`: `{ "relay": 20, "communication": 10, "presence": 5 }`
5. Add `signal_tiers`: `["tower"]`

**Validation**: After fix, `neural_api.routing_weights` should show entries for `relay.*` and `communication.*` with non-default affinity.

**Wire example** — see `WAVE42_NEURAL_API_DEPLOYMENT_GUIDE.md` § Wire Schema

---

## toadStool — Wire Existing Stub

**Priority**: HIGH — stub code exists in `identity.rs`, just needs wiring

**What exists**: `primal_announce` function marked `dead_code` in identity module

**What to do**:
1. Wire `primal_announce()` into your JSON-RPC dispatch table
2. On startup (after socket is listening), call biomeOS with:
   - `capabilities`: `["compute", "science", "inference"]`
   - `methods`: all `compute.*`, `science.*`, `inference.*` from your IPC surface
   - `signal_tiers`: `["node"]`
   - `cost_hints`: `{ "compute": 100.0, "science": 50.0, "inference": 80.0 }` (compute is expensive)
   - `latency_estimates`: `{ "compute": 200, "science": 100, "inference": 150 }`
3. Remove `#[allow(dead_code)]` from the announce function

**Validation**: `neural_api.utilization` should show `compute.*` methods after a few `capability.call` dispatches through toadStool.

---

## bearDog — Add primal.announce Handler

**Priority**: HIGH — foundation primal, tower tier, no handler exists

**What exists**: mDNS-style discovery stubs (not JSON-RPC announce)

**What to do**:
1. Add a `primal.announce` self-announcement on startup
2. Discover biomeOS socket via tiered lookup (see guide § Socket Discovery)
3. Send JSON-RPC `primal.announce` with:
   - `capabilities`: `["crypto", "security"]`
   - `methods`: all `crypto.*` and `security.*` from your IPC surface
   - `signal_tiers`: `["tower"]`
   - `cost_hints`: `{ "crypto": 5.0, "security": 10.0 }` (crypto is fast and cheap)
   - `latency_estimates`: `{ "crypto": 2, "security": 15 }`
4. Re-announce on version upgrade or capability change

**Validation**: `neural_api.routing_weights` should show bearDog as a provider for `crypto.*` calls.

---

## nestgate — Wire announce_self() to JSON-RPC

**Priority**: MEDIUM — internal announce exists, needs wire format conversion

**What exists**: `announce_self()` with discovery stubs

**What to do**:
1. Convert `announce_self()` output to proper JSON-RPC `primal.announce` payload
2. Add `socket` field (required by biomeOS v3.68+)
3. Fields:
   - `capabilities`: `["storage", "dag", "replication"]`
   - `methods`: all `storage.*`, `dag.*`, `content.*` from your 676 RPC methods
   - `signal_tiers`: `["nest"]`
   - `cost_hints`: `{ "storage": 10.0, "dag": 15.0, "replication": 25.0 }`
   - `latency_estimates`: `{ "storage": 50, "dag": 20, "replication": 100 }`

**Validation**: After announce, `capability.call` for `storage.put` should route through nestgate with tracked latency.

---

## barraCuda — Add primal.announce

**Priority**: MEDIUM — compute tier, important for node atomic routing

**What to do**:
1. Add `primal.announce` handler — send on startup after socket ready
2. Fields:
   - `capabilities`: `["math", "shader", "compute"]`
   - `methods`: all `math.*`, `shader.*` from your 87 IPC methods
   - `signal_tiers`: `["node"]`
   - `cost_hints`: `{ "math": 20.0, "shader": 50.0, "compute": 80.0 }`
   - `latency_estimates`: `{ "math": 10, "shader": 100, "compute": 200 }`

---

## coralReef — Add primal.announce

**Priority**: MEDIUM — shader compilation, node tier

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["compile", "shader_compile", "gpu"]`
   - `signal_tiers`: `["node"]`
   - `cost_hints`: `{ "compile": 60.0, "shader_compile": 80.0, "gpu": 100.0 }`
   - `latency_estimates`: `{ "compile": 500, "shader_compile": 800, "gpu": 50 }`

---

## rhizoCrypt — Add primal.announce

**Priority**: MEDIUM — DAG engine, nest tier

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["dag", "integrity", "merkle"]`
   - `signal_tiers`: `["nest"]`
   - `cost_hints`: `{ "dag": 10.0, "integrity": 5.0, "merkle": 8.0 }`
   - `latency_estimates`: `{ "dag": 15, "integrity": 5, "merkle": 10 }`

---

## loamSpine — Add primal.announce

**Priority**: MEDIUM — permanence layer, nest tier, anchoring

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["anchor", "ledger", "permanence"]`
   - `signal_tiers`: `["nest"]`
   - `cost_hints`: `{ "anchor": 20.0, "ledger": 15.0, "permanence": 30.0 }`
   - `latency_estimates`: `{ "anchor": 50, "ledger": 20, "permanence": 100 }`

---

## sweetGrass — Add primal.announce

**Priority**: LOW — attribution/provenance, nest tier

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["provenance", "attribution", "braid"]`
   - `signal_tiers`: `["nest"]`
   - `cost_hints`: `{ "provenance": 10.0, "attribution": 8.0, "braid": 12.0 }`
   - `latency_estimates`: `{ "provenance": 15, "attribution": 10, "braid": 20 }`

---

## skunkBat — Add primal.announce

**Priority**: LOW — defensive security, tower tier

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["defense", "threat_detection", "baseline"]`
   - `signal_tiers`: `["tower"]`
   - `cost_hints`: `{ "defense": 15.0, "threat_detection": 20.0, "baseline": 10.0 }`
   - `latency_estimates`: `{ "defense": 5, "threat_detection": 10, "baseline": 2 }`

---

## petalTongue — Add primal.announce

**Priority**: LOW — UI/rendering, meta tier

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["render", "ui", "accessibility"]`
   - `signal_tiers`: `["meta"]`
   - `cost_hints`: `{ "render": 30.0, "ui": 20.0, "accessibility": 10.0 }`
   - `latency_estimates`: `{ "render": 16, "ui": 10, "accessibility": 5 }`

---

## squirrel — Add primal.announce

**Priority**: LOW — AI coordination, meta tier

**What to do**:
1. Add `primal.announce` handler
2. Fields:
   - `capabilities`: `["inference", "mcp", "coordination"]`
   - `signal_tiers`: `["meta"]`
   - `cost_hints`: `{ "inference": 50.0, "mcp": 10.0, "coordination": 15.0 }`
   - `latency_estimates`: `{ "inference": 500, "mcp": 20, "coordination": 30 }`

---

## biomeOS — No Action Required

biomeOS is the registration authority. It receives `primal.announce` calls;
it does not self-announce. v3.69 persistent weights and utilization tracking
are live. No work needed.

---

## Timeline

- **Wave 43**: songbird + toadStool + bearDog (HIGH priority — foundation tier)
- **Wave 44**: nestgate + barraCuda + coralReef (MEDIUM — compute/storage tier)
- **Wave 45+**: remaining primals as they evolve (LOW — can batch)

Each team can self-validate using:
```bash
# After your primal announces, check routing weights:
echo '{"jsonrpc":"2.0","method":"neural_api.routing_weights","params":{},"id":1}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/neural-api-ecoPrimal.sock

# Check utilization after a few capability.call dispatches:
echo '{"jsonrpc":"2.0","method":"neural_api.utilization","params":{},"id":2}' | \
  socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/biomeos/neural-api-ecoPrimal.sock
```

Weight persistence means routing intelligence accumulates across restarts.
The more primals announce, the smarter the Neural API becomes.
