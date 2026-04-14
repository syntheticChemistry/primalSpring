# biomeOS Composition Gaps — From primalSpring Phase 43

**Date**: April 14, 2026
**From**: primalSpring v0.9.14
**Context**: Cross-architecture Pixel deployment via biomeOS Neural API
**License**: AGPL-3.0-or-later

---

## Summary

primalSpring validated biomeOS as the composition substrate for cross-architecture
NUCLEUS deployment. A biomeOS-managed Tower was bootstrapped on a Pixel phone
(aarch64-unknown-linux-musl + GrapheneOS) using `biomeos neural-api --tcp-only`.
6/9 exp096 cross-arch checks pass. The remaining 3 failures expose upstream gaps
in biomeOS that affect any TCP-only or cross-platform deployment.

## What Works

- `biomeos neural-api --tcp-only --port 9000 --graphs-dir ./graphs` starts and bootstraps
- `tower_atomic_bootstrap.toml` (biomeOS's canonical format) parses and executes
- BearDog and Songbird spawn as child processes with correct PIDs
- Neural API responds to `health.liveness`, `capabilities.list` via raw TCP JSON-RPC
- `primal.health("beardog")` and `primal.health("songbird")` return ALIVE through proxy
- BearDog advertises 33 capabilities through Neural API
- `FAMILY_ID` verification passes cross-architecture

## Gap 1: TCP Endpoint Propagation in NeuralRouter — RESOLVED (v3.14 + translation_loader patch)

**Original symptom**: NeuralRouter registered UDS paths for primals even in `--tcp-only` mode.

**v3.14 fix**: `discovery_init.rs` gained TCP port scanning (9900–9919). However, the scan
uses plain JSON-RPC which BearDog rejects (BTSP enforcement). The graph-based registration
in `translation_loader.rs` also used `register_capability_unix` exclusively.

**Supplemental patch**: `translation_loader.rs` now branches on `self.tcp_only` and registers
`TransportEndpoint::TcpSocket` with deterministic port assignment (beardog→9900, songbird→9901,
etc.) during graph translation loading. TCP endpoints are now correctly registered.

**Current state**: Registration works. Forwarding fails (see Gap 5).

## Gap 5: BTSP-Aware TCP Forwarding in NeuralRouter (NEW)

**Symptom**: `capability.call("crypto.sign_ed25519")` routes to `tcp://127.0.0.1:9900`
(correct) but BearDog rejects the connection because biomeOS sends plain JSON-RPC text
over TCP while BearDog enforces BTSP binary framing on all TCP connections.

**Root cause**: biomeOS's TCP forwarding path (`Calling method X on tcp://host:port`)
opens a raw TCP connection and sends newline-delimited JSON-RPC. BearDog interprets
the first 4 bytes as a BTSP frame length header (`{"js` = 0x7B226A73 = 2065853043 bytes),
which exceeds the 16MB max frame size.

**BearDog is correct** — BTSP enforcement on TCP is the intended security posture. The
fix belongs in biomeOS's forwarding path, not in BearDog.

**Fix path**: biomeOS's TCP capability forwarding should:
1. Perform BTSP Phase 1 handshake before sending JSON-RPC payloads
2. Use `FAMILY_SEED` from the ExecutionContext for HMAC-SHA256 challenge response
3. After BTSP auth, send JSON-RPC frames within the authenticated channel
4. Cache authenticated connections per endpoint for subsequent calls

**Alternative**: BearDog also binds an abstract namespace socket (`@biomeos_beardog_default`)
which may accept plain JSON-RPC. If biomeOS can connect to abstract namespace sockets
(Linux-only, same machine), this bypasses the BTSP requirement for local forwarding.

**Impact**: Blocks all `capability.call` forwarding to BearDog via TCP. Songbird (HTTP)
may work since it doesn't require BTSP on its HTTP endpoint.

## Gap 2: Graph Environment Variable Substitution

**Symptom**: BearDog receives literal `${FAMILY_ID}` as its FAMILY_ID instead of the
actual value like `pixel-cross-arch-lab`.

**Root cause**: `[nodes.operation.environment]` sections in TOML graphs contain shell-style
variable references (`${FAMILY_ID}`, `${XDG_RUNTIME_DIR}`). The graph executor passes
these literally to `Command::env()` without substitution.

**Fix path**: Before spawning child processes, the graph executor should:
1. Scan `operation.environment` values for `${VAR_NAME}` patterns
2. Substitute from: explicit graph context > process env > empty string
3. Log warnings for unresolved variables

**Impact**: Primals get wrong FAMILY_ID, wrong socket paths, and wrong runtime dirs.
This affects genetics authentication (BTSP handshake fails with wrong FAMILY_ID).

## Gap 3: bootstrap.rs Environment Inheritance (PATCHED)

**What was patched**: `biomeos-atomic-deploy/src/bootstrap.rs` now inherits
`BIOMEOS_PLASMID_BIN_DIR`, `ECOPRIMALS_PLASMID_BIN`, `XDG_RUNTIME_DIR`, and `FAMILY_SEED`
from the process environment into the `ExecutionContext` for the graph executor.

**Why**: Without this, the graph executor couldn't find primal binaries on the Pixel
(`Binary not found for: beardog`). The fix is minimal — just read these 4 env vars
and inject them into the `ExecutionContext.env` HashMap.

**Status**: Patched locally, needs review and merge upstream.

## Gap 4: --tcp-only Cascade to Child Primals

**Current behavior**: `biomeos neural-api --tcp-only` binds its own API on TCP, but
child primals spawned from graphs still default to UDS binding unless their launch
profile explicitly sets TCP args.

**Desired behavior**: When biomeOS runs with `--tcp-only`, this mode should cascade
to all spawned primals automatically (e.g., inject `--port <auto>` into spawn args
or set `PRIMAL_TRANSPORT=tcp` in child env).

**Impact**: Currently requires manual launch profile configuration per primal for
TCP-only deployments. Should be automatic.

## Validation Evidence

```
exp096 results against biomeOS Neural API (forwarded port 19000 → Pixel 9000):

[PASS] Neural API alive (health.liveness)
[PASS] BearDog alive through proxy (primal.health)
[PASS] Songbird alive through proxy (primal.health)
[PASS] NestGate recognized (primal.health returns registered)
[PASS] BearDog capabilities (33 found via capabilities.list proxy)
[PASS] FAMILY_ID matches (pixel-cross-arch-lab verified cross-arch)
[FAIL] Genetics RPC via proxy (capability.call routing fails → Gap 1)
[FAIL] BTSP Phase 3 via proxy (capability.call routing fails → Gap 1)
[FAIL] HSM probe via proxy (capability.call routing fails → Gap 1)
```

## Recommendation

Gap 5 (BTSP-aware TCP forwarding) is now the critical path. Registration is solved —
biomeOS correctly routes `capability.call` to `tcp://127.0.0.1:9900`. But the forwarding
code sends plain JSON-RPC which BearDog's BTSP enforcement rejects. The fix requires
biomeOS to act as a BTSP client when forwarding to security-enforcing primals.

Gap 2 (env substitution) is landed in v3.14 via two-pass resolution. Gap 3 (bootstrap env
inheritance) is landed. Gap 4 (`--tcp-only` cascade) is landed. Only Gap 5 remains.

---

**License**: AGPL-3.0-or-later
