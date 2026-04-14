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

## Gap 1: TCP Endpoint Propagation in NeuralRouter

**Symptom**: `capability.call("crypto.sign_ed25519")` fails even though BearDog is alive
on TCP 127.0.0.1:9900. Neural API returns an error because it routes to a Unix socket
path that doesn't exist on Android.

**Root cause**: When biomeOS spawns primals in `--tcp-only` mode, the `NeuralRouter`
still constructs a Unix socket endpoint for capability routing. It doesn't detect or
register the TCP endpoint that the primal actually binds.

**Fix path**: After primal spawn, NeuralRouter should:
1. Check if the primal bound a TCP port (from launch profile or process introspection)
2. Register TCP endpoint `host:port` instead of (or alongside) UDS path
3. `capability.call` routing should use the registered transport, not assume UDS

**Impact**: Blocks all `capability.call` flows on Android, Windows, and any `--tcp-only` deployment.

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

Gap 1 (TCP endpoint propagation) is the critical path. Once NeuralRouter registers
TCP endpoints, all 3 failing checks should pass. Gap 2 (env substitution) is important
for correctness but doesn't block basic capability routing. Gap 4 is quality-of-life
for operators deploying on non-UDS platforms.

---

**License**: AGPL-3.0-or-later
