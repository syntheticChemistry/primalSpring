# primalSpring Phase 60 — Security Gate + Evolution Handoff for Downstream

**Date**: May 7, 2026  
**From**: primalSpring v0.9.25  
**For**: projectNUCLEUS, delta springs (hotSpring, wetSpring, airSpring, healthSpring, groundSpring, neuralSpring, ludoSpring, esotericWebb), sporeGarden  

---

## What Shipped in Phase 60 (cumulative)

### Security Gate (JH-0) — Critical

primalSpring now enforces a pre-dispatch capability authorization gate on its
JSON-RPC dispatcher. This is the ecosystem standard that all primals and
compositions should adopt.

**What it means for you:**
- When compositions flip to `Enforced` mode, only callers with valid capability
  tokens can invoke protected methods (anything beyond health/identity/capabilities)
- Your validation binaries should test that `auth.mode` responds and report the
  enforcement level
- Spring dispatchers that serve JSON-RPC (if any) should adopt the same pattern

**New methods available on primalSpring:**
- `auth.check` — is the caller authenticated?
- `auth.mode` — current enforcement mode (permissive/enforced)
- `auth.peer_info` — peer credential introspection (UID, PID, origin)

**New error codes to handle:**
- `-32001 PERMISSION_DENIED` — method requires a token you don't have
- `-32000 UNAUTHORIZED` — identity could not be established

Guide: `primalSpring/wateringHole/METHOD_GATE_STANDARD.md`

### Registry: 384 Methods Across 82 Domains

The capability registry now tracks 384 methods across 82 domain sections, owned
by 13 primals + primalSpring + 2 downstream products. New since last handoff:
- `auth.check`, `auth.mode`, `auth.peer_info` (primalSpring gate introspection)
- All existing domains fully reconciled with upstream primal evolution

### Zero Unsafe Code / Zero DEBT

- `SeedConfig` + `OnceLock` replaced all `unsafe { env::set_var }` calls
- Zero `DEBT`, `TODO`, `FIXME`, `HACK` markers in production code
- Library code should use `primalspring::env_keys::resolve_family_id()` and
  `resolve_family_seed()` instead of raw `std::env::var()`

### Upstream Gap Closures Since Last Downstream Handoff

| Gap | Resolution |
|-----|-----------|
| PT-09 | petalTongue BTSP Phase 2 — full handshake wired (v1.6.6) |
| PT-13 | petalTongue NestGate CAS backend integration (v1.6.6) |
| JH-0 | primalSpring `MethodGate` pattern — reference implementation complete |

### Metrics

- **666 tests** (618 passed + 48 ignored)
- **384 registered capability methods**
- **0 clippy errors**, 0 primal drift
- **211/211** source method strings validated
- **353** graph method references checked

---

## What Springs Should Do

### 1. Absorb the Method Gate Pattern (P1 — if you have a JSON-RPC dispatcher)

If your spring has a `server.rs` or equivalent that handles JSON-RPC requests:

```
handle_connection(stream)
  → CallerContext::from_unix_stream(stream)
  → for each request:
    → gate.check(method, caller)  // pre-dispatch
    → if Ok: dispatch_request(line)
    → if Err: return PERMISSION_DENIED
```

Start in `Permissive` mode. See `METHOD_GATE_STANDARD.md` for the full pattern.

### 2. Handle PERMISSION_DENIED in Your IPC Clients (P1)

If your spring calls primals via `PrimalClient`, you should now handle the
`PermissionDenied` error variant:

```rust
match client.call("compute.submit", params) {
    Err(e) if e.is_permission_denied() => {
        // Gate rejected — log and surface to user
    }
    // ... existing error handling
}
```

### 3. Use `resolve_family_id()` Instead of Raw Env Vars (P2)

If you depend on `primalspring` as a library and read `FAMILY_ID`:

```rust
// Before (may miss SeedConfig values):
let family = std::env::var("FAMILY_ID").unwrap_or_else(|_| "default".to_owned());

// After (checks OnceLock first, then env):
let family = primalspring::env_keys::resolve_family_id();
```

### 4. Update Your Validation Binaries (P2)

Add a check for the method gate in your guidestone or validation binary:

```rust
// Call auth.mode on primalspring — verify gate is wired
match client.call("auth.mode", serde_json::Value::Null) {
    Ok(resp) if resp.is_success() => { /* gate wired */ }
    _ => { /* gate not available */ }
}
```

### 5. Fill `sporeprint/validation-summary.md` (P2 — sporePrint pipeline)

Update your spring's validation summary with current metrics. The sporePrint
pipeline auto-renders these to primals.eco. Follow the pattern in
`wetSpring/sporeprint/validation-summary.md`.

### 6. Create/Update Notebooks (P3 — sporePrint pipeline)

If you haven't yet, create a `notebooks/` directory following
`wetSpring/notebooks/NOTEBOOK_PATTERN.md`. Recommended 5-notebook set:
01-domain-validation, 02-benchmark-comparison, 03-paper-reproductions,
04-cross-spring-connections, 05-domain-deep-dive.

---

## What projectNUCLEUS Should Do

### 1. Test the Gate (P1)

With primalSpring running, call `auth.mode` and `auth.check` to verify
the gate is wired. In your pentest scripts, verify that:
- Public methods (health.check, identity.get) work without a token
- Protected methods log warnings in permissive mode
- When `PRIMALSPRING_AUTH_MODE=enforced`, protected methods return `-32001`

### 2. Absorb PT-09/PT-13 Closures

petalTongue v1.6.6 resolved both. Your `validation/PETALTONGUE_GAPS_HANDBACK.md`
can be updated to reflect these closures.

### 3. Begin JH-1 Coordination with BearDog

JH-0 (gate pattern) is the prerequisite for JH-1 (BearDog token issuance).
With the gate wired, the next step is BearDog shipping `auth.issue_ionic` and
`auth.verify_ionic`. The gate's token verification is a trait interface ready
to plug in.

---

## Composition Patterns for NUCLEUS Deployment via Neural API

### Desktop NUCLEUS (13-primal live substrate)

```
tools/desktop_nucleus.sh → biomeOS Neural API → 13 primals
                         → graphs/desktop/*.toml → composition validation
                         → primalspring_primal server → auth.mode (gate status)
```

### Multi-User (JupyterHub / ironGate)

```
JupyterHub → spawn per-user compositions
           → each user gets PRIMALSPRING_AUTH_MODE=enforced
           → MethodGate rejects cross-user calls without ionic token
           → biomeOS composition.reload for per-primal hot-swap (JH-3, pending)
```

### Spring Validation via biomeOS

```
spring_guidestone → CompositionContext::discover()
                  → capability-based routing via Neural API
                  → validate_method_gate() → Layer 1.6 security check
                  → full 9-layer certification
```

---

## Remaining Open Gaps (for reference)

| Priority | Gap | Owner | Status |
|----------|-----|-------|--------|
| **Critical** | JH-0: RPC dispatcher capability check | All primals | **IN PROGRESS** — primalSpring reference complete, ecosystem adoption pending |
| High | JH-1: BearDog identity management (`auth.issue_ionic`) | BearDog | OPEN |
| High | JH-2: Token-carried resource envelope | biomeOS + ToadStool | OPEN |
| Medium | JH-3: Composition hot-reload | biomeOS | OPEN |
| Medium | JH-4: BearDog + primalSpring token UX | BearDog + primalSpring | OPEN |
| Medium | JH-5: skunkBat log aggregation | skunkBat | OPEN |
| P3 | barraCuda 4 stateful API gaps | barraCuda | UNBLOCKED |
| P3 | coralReef transitive libc | coralReef | OPEN |

---

## Pull Instructions

```bash
# primalSpring (the source of truth)
cd springs/primalSpring && git pull --rebase origin main

# infra wateringHole (handoffs)
cd infra && git pull --rebase origin main

# Key files to review:
# - wateringHole/METHOD_GATE_STANDARD.md (ecosystem standard)
# - wateringHole/DOWNSTREAM_HANDOFF_PHASE60_SECURITY_GATE_MAY07_2026.md (this file)
# - ecoPrimal/src/ipc/method_gate.rs (reference implementation)
# - config/capability_registry.toml (384 methods, auth.* + nautilus.* sections)
```
