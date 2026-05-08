# primalSpring Phase 60 — Ionic Tokens + GAP-11 Closure Handoff

**Date**: May 8, 2026  
**From**: primalSpring v0.9.25  
**For**: projectNUCLEUS, delta springs, sporeGarden  

---

## What Shipped (May 7–8 cumulative)

### JH-0: 13/13 Primals Adopted MethodGate — COMPLETE

Every primal in the ecosystem now ships a pre-dispatch capability gate with:
- `auth.check` / `auth.mode` / `auth.peer_info` introspection methods
- Permissive default (log + allow), Enforced mode via env var
- `-32001 PERMISSION_DENIED` / `-32000 UNAUTHORIZED` error codes
- 3 primals extract bearer tokens from params (barraCuda, biomeOS, petalTongue)

### JH-1: BearDog Ionic Tokens — RESOLVED (Wave 94)

BearDog ships primal-native identity and token infrastructure:

| Method | Purpose |
|--------|---------|
| `identity.create` | Generates ephemeral Ed25519 keypair, returns DID (`did:key:z6Mk...`) |
| `auth.issue_ionic` | Issues scoped token: `subject`, `scope` patterns (e.g. `["crypto.*"]`), `ttl_secs` |
| `auth.verify_ionic` | Verifies signature + expiry + optional scope check against a method |

**Token format**: JWT-like `base64(header).base64(payload).base64(signature)`
- **Header**: `{ alg: "EdDSA", typ: "ionic", ver: 1 }`
- **Payload**: `{ iss, sub, scope, iat, exp, jti }` — scope is glob patterns
- **Signature**: Ed25519 over `{header_b64}.{payload_b64}`

**Scope matching** (`scope_covers_method`): `*` matches all; `prefix.*` matches
dot-boundary prefix; exact string match. Enforced in MethodGate when mode is Enforced.

**What this means for you**: You can now issue capability-scoped tokens from BearDog,
pass them as `_bearer_token` in JSON-RPC params, and have MethodGate enforce scope.

### GAP-11: barraCuda 18/18 CLOSED (Sprint 55)

All 18 JSON-RPC surface gaps are now closed. New in Sprint 55:

| Method | Type | Purpose |
|--------|------|---------|
| `ml.mlp_train` | Path A (stateless) | SGD backpropagation: `layers`, `inputs`, `targets`, `learning_rate`, `epochs` → trained weights + MSE |
| `nautilus.create` | Path B (session) | Create server-side session: `pop_size`, `generations_per_train` → `session_id` |
| `nautilus.observe` | Path B | Feed observation data: `beta`, `plaquette`, `cg_iters`, etc. |
| `nautilus.train` | Path B | Train on accumulated observations → MSE, drift detection |
| `nautilus.predict` | Path B | Predict CG iterations for given `beta` |
| `nautilus.export` | Path B | Export brain state as JSON (for persistence/transfer) |
| `nautilus.import` | Path B | Import brain state → new session |

**Session pattern**: `nautilus.*` uses Path B (server-managed state) with process-global
`RwLock<HashMap<String, NautilusBrain>>`. Sessions persist until process restart.
`session_id` format: `nautilus-{hex_timestamp}`.

**barraCuda now has 71 JSON-RPC methods** including full `ode.step`, `ml.esn_predict`,
`ml.mlp_train`, and 6 `nautilus.*` session methods.

### Registry: 381 Methods Across 82 Domains

New since last handoff:
- `identity.create` (BearDog JH-1)
- `auth.issue_ionic`, `auth.verify_ionic` (BearDog JH-1)
- `ml.mlp_train` (barraCuda Sprint 55)
- 6 `nautilus.*` methods (barraCuda Sprint 55)
- `ode.step`, `ml.esn_predict` (barraCuda Sprint 54)

---

## What Springs Should Do

### 1. Wire Bearer Tokens into Your PrimalClient Calls (P1)

When calling protected methods, pass the ionic token:

```rust
let token = beardog_client.call("auth.issue_ionic", json!({
    "scope": ["compute.*", "storage.*"],
    "ttl_secs": 3600
}))?;

// Pass token in params for primals that extract it
let result = toadstool_client.call("compute.dispatch.submit", json!({
    "_bearer_token": token["token"],
    "workload": "my_job",
    // ... other params
}))?;
```

### 2. Handle Scope-Based Rejections (P1)

When `PRIMALSPRING_AUTH_MODE=enforced` (or the primal-specific env var):

```rust
match client.call("compute.submit", params) {
    Err(e) if e.is_permission_denied() => {
        // Token scope doesn't cover this method
        // Request a broader-scoped token or inform user
    }
    // ... existing handling
}
```

### 3. Consume barraCuda's New Methods (P2 — domain springs)

**For neuralSpring / airSpring / hotSpring**:

- `ml.mlp_train` — train MLPs over IPC instead of linking barraCuda as a library
- `nautilus.create/observe/train/predict` — server-session pattern for lattice QCD
- `ode.step` — RK4 integration without library coupling

### 4. Identity in Composition Graphs (P2)

For NUCLEUS graphs that need per-user identity:

```toml
[[step]]
name = "user_identity"
primal = "beardog"
method = "identity.create"

[[step]]
name = "scoped_token"
primal = "beardog"
method = "auth.issue_ionic"
params = { scope = ["compute.*", "storage.*"], ttl_secs = 7200 }
depends_on = ["user_identity"]
```

---

## Composition Patterns for NUCLEUS via Neural API

### Token Flow in Multi-User Deployments

```
User → JupyterHub spawner
     → BearDog: identity.create → DID
     → BearDog: auth.issue_ionic(scope=["compute.*"], ttl=session_ttl)
     → Token embedded in all subsequent PrimalClient calls
     → MethodGate on each primal: verify signature + scope + expiry
```

### Nautilus Session Lifecycle (barraCuda Path B)

```
Client → nautilus.create(pop_size=16) → session_id
       → nautilus.observe(session_id, beta=6.0, plaquette=0.5, ...)  (repeat)
       → nautilus.train(session_id) → { trained: true, mse: 0.003 }
       → nautilus.predict(session_id, beta=6.2) → { cg_iters: 42 }
       → nautilus.export(session_id) → brain_json (for persistence)
```

### Desktop NUCLEUS Stack

```
tools/desktop_nucleus.sh
  → 13 primals (all with MethodGate, permissive)
  → BearDog: ionic token issuer
  → barraCuda: 71 methods (ODE, ESN, MLP, Nautilus, tensor, stats, linalg)
  → biomeOS Neural API: 605+ capabilities, semantic routing
  → primalSpring guidestone: 9-layer validation + Layer 1.6 (method gate)
```

---

## Remaining Open Gaps

| Priority | Gap | Owner | Status |
|----------|-----|-------|--------|
| High | JH-2: Token-carried resource envelope | biomeOS + ToadStool | OPEN — now unblocked by JH-1 |
| Medium | JH-3: Composition hot-reload | biomeOS | OPEN |
| Medium | JH-4: Token issuance UX | BearDog + primalSpring | OPEN |
| Medium | JH-5: Log aggregation + provenance | skunkBat | OPEN |

---

## Pull Instructions

```bash
cd springs/primalSpring && git pull --rebase origin main
# Key files:
# - config/capability_registry.toml (381 methods)
# - wateringHole/METHOD_GATE_STANDARD.md (ecosystem standard)
# - wateringHole/DOWNSTREAM_HANDOFF_PHASE60_IONIC_TOKENS_MAY08_2026.md (this file)
# - docs/PRIMAL_GAPS.md (JH-0 13/13, JH-1 resolved, GAP-11 18/18)
```
