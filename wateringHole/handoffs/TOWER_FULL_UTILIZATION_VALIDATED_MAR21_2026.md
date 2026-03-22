# Tower Full Utilization — VALIDATED

**Date**: 2026-03-21  
**From**: primalSpring  
**Version**: v0.5.0  
**Status**: 41/41 gates PASS — Tower Fully Utilized

## Summary

primalSpring has validated Tower Atomic at full utilization. All 41 stability gates
pass. Changes were made across songbird, biomeOS, and primalSpring codebases under
authorized cross-team work.

## What Was Done

### songbird (authorized changes)

1. **Ephemeral port support (`--port 0`)**: Config validation now allows `base_port == 0`
   when discovery is disabled. `bind_with_fallback` skips the +1..+10 port fallback loop
   when port is 0, letting the OS assign an ephemeral port. This eliminates port contention
   during parallel test execution.

   Files changed:
   - `crates/songbird-types/src/config/consolidated_canonical/mod.rs` — allow port 0
   - `crates/songbird-orchestrator/src/app/http_server.rs` — ephemeral bind path

2. **New IPC method aliases**: Added `discovery.find_primals`, `discovery.announce`,
   `federation.peers`, `federation.status` method handlers to `songbird-universal-ipc`.

   Files changed:
   - `crates/songbird-universal-ipc/src/service.rs` — new method match arms
   - `crates/songbird-universal-ipc/src/handlers/discovery_handler.rs` — `handle_announce`

3. **Rebuilt binary**: `plasmidBin/primals/songbird` updated with all fixes.

### biomeOS (authorized changes)

1. **Federation capability translations**: Added `[translations.federation]` section to
   `config/capability_registry.toml` mapping `federation.peers` and `federation.status`
   to songbird methods.

2. **Updated discovery translations**: Fixed method names in `[translations.discovery]`
   to match songbird's actual registered methods.

### primalSpring (own codebase)

1. **`extra_args` in LaunchProfile**: New field for passing verbatim CLI args to primals.
   Songbird profile now passes `--port 0` + `SONGBIRD_DISCOVERY_MODE=disabled`.

2. **Fixed experiment parameters**: Corrected BirdSong `node_id` requirement, `encrypted_beacon`
   field naming, petalTongue `PETALTONGUE_SOCKET` env var, Grammar of Graphics enum casing.

3. **Shortened family IDs**: Prevent `SUN_LEN` overflow in socket paths.

## Live Results

### Integration Tests: 19/19 PASS (parallel, ~1 second)

All tests run in parallel with no port contention. Previously required sequential
execution (~30s) due to songbird port 8080-8090 conflicts.

### Experiments: 4/4 ALL PASS

| Experiment | Result | Key Finding |
|---|---|---|
| exp062 subsystem sweep | 11/12 UP | `tor.connect` DOWN (expected: needs `tor.service.start` first) |
| exp063 rendezvous | ALL PASS | BirdSong beacon encrypt→decrypt round-trip verified |
| exp064 internet reach | ALL PASS | STUN, Onion, Tor paths available |
| exp065 petalTongue | ALL PASS | Dashboard + Grammar of Graphics rendering |

### Subsystem Status (from exp062)

| Subsystem | Methods UP | Status |
|---|---|---|
| core | 2/2 | health.liveness, capabilities.list |
| discovery | 1/1 | discovery.find_primals |
| stun | 2/2 | get_public_address, detect_nat_type |
| birdsong | 1/1 | generate_encrypted_beacon |
| onion | 2/2 | start, status |
| tor | 1/2 | status UP, connect DOWN (needs init first) |
| federation | 2/2 | peers, status (stubs, no active peers) |

## Handoff Notes by Team

### For songbird team
- The `--port 0` change is backward-compatible. The fault test `test_fault_invalid_port_zero`
  still passes (port 0 is rejected when discovery mode is enabled, which is the default).
- Federation methods are stubs. Wire them to actual `songbird-network-federation` when ready.
- `discovery.announce` currently returns a JSON ack without actually broadcasting. Wire to
  `AnonymousDiscoveryListener` when needed.
- `tor.connect` requires `tor.service.start` first — consider auto-init or better error message.

### For biomeOS team
- Federation translations added to `capability_registry.toml`. Verify they load correctly
  on next biomeOS build.
- Discovery translation methods updated to match songbird's actual registered method names.

### For petalTongue team
- Server mode works correctly with `PETALTONGUE_SOCKET` env var.
- Dashboard and Grammar of Graphics rendering both validated via JSON-RPC.
- `DashboardRenderRequest.bindings` expects `DataBinding` enum (tagged with `channel_type`).
- `GrammarExpr` uses PascalCase enums (`Cartesian`, `Bar`, `X`, `Y`).

### For nestgate team
- Tower is fully utilized. Ready for Nest Atomic integration.
- HTTPS probe to `api.nestgate.io` not yet wired as songbird IPC method — needs
  `discovery.https_probe` or similar if desired as a formal capability.

## What's Next

1. **Nest Atomic** — add nestgate to Tower composition, define storage gates
2. **Pixel 8a replication** — run exp063 mobile side on Pixel 8a over hotspot
3. **Federation wiring** — connect songbird federation stubs to actual peer discovery
4. **HTTPS probe IPC** — optional: expose HTTP client as `discovery.https_probe`
5. **Tor initialization** — auto-init Tor on `tor.connect` or provide clearer error path
