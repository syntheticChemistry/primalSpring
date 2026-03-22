# Tower Full Utilization Handoff — primalSpring v0.5.0

**Date**: 2026-03-21  
**Sprint**: Tower Full Utilization  
**Status**: Tests written, live validation pending

## Summary

primalSpring v0.5.0 expands Tower validation from core stability (24/24 gates)
to full utilization (24/41 gates, 17 pending live sprint). This adds songbird
subsystem coverage, Pixel 8a rendezvous replication, internet reach validation,
and petalTongue visualization integration.

## What Was Delivered

### New Integration Tests (6)

All `#[ignore]`, require plasmidBin + optional network:

| Test | Gate | Songbird Method |
|------|------|-----------------|
| `tower_discovery_announce_find` | 7.1 | `discovery.announce` + `discovery.find_primals` |
| `tower_stun_public_address` | 7.2 | `stun.get_public_address` |
| `tower_birdsong_beacon` | 7.3 | `birdsong.generate_encrypted_beacon` + `decrypt` |
| `tower_onion_service` | 7.4 | `onion.start` + `onion.status` |
| `tower_tor_status` | 7.5 | `tor.status` |
| `tower_federation_status` | 7.6 | `songbird.federation.peers` |

### New Experiments (4)

| # | Name | Validates |
|---|------|-----------|
| 062 | `tower_subsystem_sweep` | All songbird JSON-RPC methods (UP/DEGRADED/DOWN) |
| 063 | `pixel_tower_rendezvous` | BirdSong beacon + onion + STUN (Pixel deployment path) |
| 064 | `nestgate_internet_reach` | HTTPS, STUN, NAT, onion, Tor internet paths |
| 065 | `petaltongue_tower_dashboard` | visualization.render.dashboard + grammar via petalTongue |

### Infrastructure

- **petalTongue v1.6.6** rebuilt and deployed to `plasmidBin/primals/petaltongue`
- **`[profiles.petaltongue]`** added to `primal_launch_profiles.toml`
- **12 new capabilities** added to capability registry and `niche.rs`
- **`tower_full_capability.toml`** deploy graph with all subsystem nodes
- **TOWER_STABILITY.md** expanded: gates 7-11 (subsystem health, beacon, rendezvous, internet, visualization)

### Test & Experiment Count

| Metric | v0.4.0 | v0.5.0 |
|--------|--------|--------|
| Unit tests | 239 | 239 |
| Integration tests | 23 | 29 |
| Doc tests | 2 | 2 |
| **Total tests** | **264** | **270** |
| Experiments | 40 | 44 |
| Capabilities | 25 | 37 |
| Tower gates | 24/24 | 24/41 |

## Architecture Pattern: `direct_rpc_call`

The new songbird subsystem tests use a `direct_rpc_call` helper that sends
JSON-RPC 2.0 requests directly to a primal's Unix socket. This bypasses
Neural API routing for subsystem-specific methods that may not have capability
translations registered yet. This is a pragmatic testing pattern while the
Neural API capability translation table is being expanded.

## What Blocks Gate 7-11 Completion

1. **Live validation sprint**: Tests are written but need to be run against
   the real plasmidBin binaries with network access
2. **Songbird method registration**: Some subsystem methods (e.g., `birdsong.*`,
   `onion.*`) may return "Method not found" if songbird doesn't register them
   in its current build — the tests handle this gracefully
3. **petalTongue IPC wiring**: petalTongue `server` subcommand needs to bind
   the socket path from `--socket` CLI argument — verify this works
4. **Network access**: Gates 10.1-10.3 require internet access to nestgate.io

## Cross-Team Notes

### For songbird team
- Tests probe: `discovery.*`, `stun.*`, `birdsong.*`, `onion.*`, `tor.*`, `federation.*`
- If any method returns "Method not found", the tests pass gracefully but gates
  are marked DEGRADED — team should ensure all methods are registered

### For biomeOS team
- `tower_full_capability.toml` graph adds petalTongue and Squirrel as optional nodes
- Pixel rendezvous experiment (exp063) documents the mobile-side steps for
  full replication with `start_nucleus_mobile.sh`

### For petalTongue team
- petalTongue v1.6.6 built with musl target and deployed to plasmidBin
- exp065 tests `visualization.render.dashboard` and `visualization.render.grammar`
- Confirm `server` subcommand binds to `--socket` path for IPC

## Next Steps

1. **Run live validation sprint** — execute all ignored tests with plasmidBin
2. **Fix any DEGRADED subsystems** — work with songbird team on method registration
3. **Validate Pixel rendezvous** — run exp063 + mobile side with Pixel 8a
4. **Proceed to Nest Atomic** — add nestgate, inherit all Tower gates
