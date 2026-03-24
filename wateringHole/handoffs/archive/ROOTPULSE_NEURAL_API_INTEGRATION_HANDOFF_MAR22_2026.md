# RootPulse Neural API Integration Handoff

**Date**: March 22, 2026
**Version**: primalSpring v0.7.0, Phase 11
**Author**: primalSpring coordination team

---

## Summary

primalSpring now integrates with the provenance trio (rhizoCrypt, loamSpine,
sweetGrass) via biomeOS Neural API `capability.call`. Zero compile-time
coupling to trio crates. All interaction routes through semantic capability
domains, and all experiments degrade gracefully when the trio is unavailable.

## What Was Built

### `ecoPrimal/src/ipc/provenance.rs` (~310 LOC)

Reusable provenance integration module following the
`SPRING_PROVENANCE_TRIO_INTEGRATION_PATTERN.md`:

| Function | Capability Domain | Operation | Primal |
|----------|-------------------|-----------|--------|
| `begin_experiment_session()` | `dag` | `create_session` | rhizoCrypt |
| `record_experiment_step()` | `dag` | `event.append` | rhizoCrypt |
| `complete_experiment()` | `dag` + `commit` + `provenance` | dehydrate → session → create_braid | All three |
| `rootpulse_branch()` | `dag` | `branch` | rhizoCrypt |
| `rootpulse_merge()` | `dag` | `merge` | rhizoCrypt |
| `rootpulse_diff()` | `dag` | `diff` | rhizoCrypt |
| `rootpulse_federate()` | `dag` | `federate` | rhizoCrypt |
| `trio_available()` | `dag` + `commit` + `provenance` | `health` | All three |
| `trio_health()` | `dag` + `commit` + `provenance` | `health` | All three |

### Graceful Degradation Contract

| Condition | Behavior | Status |
|-----------|----------|--------|
| Neural API unreachable | Return `Unavailable` | Experiment passes (SKIP) |
| Dehydrate fails | Return `Unavailable` | Experiment passes (SKIP) |
| Commit fails | Return `Partial` | Dehydration preserved |
| Braid fails | Return `Complete` with empty `braid_id` | Commit preserved |

Domain logic never fails because provenance is unavailable.

### Experiments Evolved

| Experiment | Before | After |
|------------|--------|-------|
| **exp020** RootPulse Commit | 6x `check_skip` | 6-phase pipeline via `capability.call` (health → dehydrate → sign → store → commit → attribute) |
| **exp021** Branch/Merge | 1x `check_skip` | Branch creation + branch work + merge via `capability.call` |
| **exp022** Diff/Federate | 1x `check_skip` | Divergent sessions + Merkle diff + federation via `capability.call` |
| **exp041** Trio Science E2E | 1x `check_skip` | Full chain: begin → 3 steps → dehydrate → commit → attribute |

All experiments use `check_or_skip` gated on `trio_available()`: when trio
is running, they exercise real IPC; when not, they produce SKIP (not FAIL).

## What This Unblocks

- **Other springs** can adopt the same pattern by creating an `ipc/provenance.rs`
  module following `SPRING_PROVENANCE_TRIO_INTEGRATION_PATTERN.md` and using
  primalSpring's module as a reference implementation.
- **biomeOS** capability routing for `dag.*`, `commit.*`, `provenance.*` is
  already wired in `capability_domains.rs`. No biomeOS changes needed.
- **Provenance trio teams** can validate their IPC contract by running
  primalSpring's experiments against live trio primals.

## What's Needed for Live Validation

| Need | Impact | Owner |
|------|--------|-------|
| Trio release binaries in `plasmidBin/` | Experiments skip all live IPC checks until trio is running | Trio teams |
| `exp013` (Pipeline Streaming) | Needs sweetGrass binary for NestGate → primalSpring → sweetGrass pipeline | sweetGrass team |
| `exp014` (Continuous Tick) | Needs all trio for 8-node 60Hz health poll | Trio teams |

**Note**: The `provenance-trio-types` dependency has been **resolved** — all
trio teams inlined their types (March 22, 2026). primalSpring has zero
compile-time dependency on any trio crate; all integration is via Neural API
`capability.call`.

## How to Test

```bash
# Without trio (all provenance checks SKIP):
cargo run --bin exp020_rootpulse_commit
cargo run --bin exp041_provenance_trio_science

# With trio running (checks exercise real IPC):
# 1. Build and start provenance trio
# 2. Start biomeOS with Neural API
# 3. Run experiments:
cargo run --bin exp020_rootpulse_commit
cargo run --bin exp021_rootpulse_branch_merge
cargo run --bin exp022_rootpulse_diff_federate
cargo run --bin exp041_provenance_trio_science
```

## Architecture Reference

```text
┌─────────────────────┐
│    primalSpring      │
│  ipc::provenance     │
│                      │
│  begin_session()  ────→ capability.call("dag", "create_session", ...)
│  record_step()    ────→ capability.call("dag", "event.append", ...)
│  complete()       ────→ 1. capability.call("dag", "dehydrate", ...)
│                   │    2. capability.call("commit", "session", ...)
│                   │    3. capability.call("provenance", "create_braid", ...)
│  branch()         ────→ capability.call("dag", "branch", ...)
│  merge()          ────→ capability.call("dag", "merge", ...)
│  diff()           ────→ capability.call("dag", "diff", ...)
│  federate()       ────→ capability.call("dag", "federate", ...)
└────────┬──────────┘
         │ Unix socket (Neural API)
         ▼
┌─────────────────────┐    capability routing
│    biomeOS           │──→ dag.*        → rhizoCrypt
│   Neural API         │──→ commit.*     → loamSpine
│                      │──→ provenance.* → sweetGrass
└──────────────────────┘
```

## For Other Springs

To adopt this pattern in your spring:

1. Create `ipc/provenance.rs` following `SPRING_PROVENANCE_TRIO_INTEGRATION_PATTERN.md`
2. Replace `primalspring` with your spring name in session metadata
3. Use `check_or_skip` gating in experiments for graceful degradation
4. Do NOT add `provenance-trio-types` as a compile-time dependency (use Neural API)
5. Test with `NEURAL_API_SOCKET` unset (degradation) and with biomeOS running (live)
