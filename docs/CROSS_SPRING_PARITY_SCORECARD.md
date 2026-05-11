# Cross-Spring Composition Parity Scorecard

> papers → Python/R → Rust → primals (IPC) → NUCLEUS composition

**Last updated**: May 11, 2026 — Phase 60+ (v0.9.25, zero upstream gaps, JH-11 resolved, post-interstadial targets all green)
**Audited by**: primalSpring composition audit
**Method**: Pulled all 8 springs to HEAD, assessed each across 9 axes

## Legend

- **G** = Green (fully implemented / present)
- **Y** = Yellow (partial / in-progress)
- **R** = Red (absent / not started)

## Scorecard

| Spring | Tests | barraCuda Coupling | primalSpring Dep | Guidestone Level | Capability Registry | Deploy Graphs | Composition Experiments | Paper Notebooks | deny.toml |
|--------|------:|-------------------|-----------------|-----------------|-------------------|--------------|----------------------|----------------|-----------|
| **primalSpring** | 680 | None (validates, doesn't consume) | N/A (is primalSpring) | L5 (6 layers) | **G** 413 methods, sync-tested, zero drift | **G** 74 graphs | **G** 89 exp crates, CompositionContext throughout | **Y** 5 (frozen JSON, not live paper) | **G** bans ring/openssl |
| **hotSpring** | 1,025 | **Y** path dep + IPC | **G** unconditional | **G** L6 (certified) | **G** local TOML + sync test | **G** 5 graphs | **Y** exp bins (not crates), CompositionContext in guidestone | **G** 17 (paper-linked) | **G** bans ring/openssl/aws-lc-sys |
| **healthSpring** | 999 | **Y** path dep + IPC feature (barracuda-lib) | **Y** feature-gated | **G** L5 (Tier 1-3) | **G** 118 methods in TOML + CI cross-sync vs canonical 413 | **G** 7 graphs, skunkBat node | **G** 95 exp crates (exp123 NUCLEUS parity) | **G** 53 .ipynb (all controls converted) | **G** bans ring/openssl/aws-lc-sys |
| **wetSpring** | 1,962 | **Y** path dep + IPC feature (barracuda-lib) + barraCuda IPC routing | **Y** feature-gated | **G** L4 (38/38 NUCLEUS) | **G** TOML + cross-sync 413 | **G** 7 graphs | **G** 1 exp crate (exp400 NUCLEUS composition parity) | **G** 19 + Kachkovskiy | **G** bans ring + openssl |
| **neuralSpring** | 1,450 | **Y** path dep + IpcMathClient | **Y** feature-gated | **Y** L3 (L4-L5 pending) | **G** 34 capabilities, TOML + sync test | **G** 4 graphs (3 new Phase 60) | **G** exp094 parity crate, IPC in playGround | **G** 10 (paper-linked, DOI) | **G** bans ring/openssl/rustls |
| **ludoSpring** | 854 | **G** optional=true, IPC-first default | **Y** feature-gated | **G** L4 (Tier 1-3, 3-tier certification) | **G** 28 game.* + cross-sync 413 | **G** 12 graphs, skunkBat node | **G** 100 exp fossilized, 8 scenarios | **R** 0 .ipynb (Python baselines in baselines/) | **G** bans ring/openssl |
| **groundSpring** | 1,101 | **Y** path dep (optional, default) | **Y** feature-gated | **G** L4 (modularized 5-layer guidestone) | **G** 16 MCP tools + 6 registry sync tests | **G** 6 graphs | **G** 2 exp crates (exp094/exp095 use CompositionContext) | **G** 34 (paper-linked) | **G** bans ring/openssl |
| **airSpring** | 1,327 | **Y** path dep + some IPC wiring | **Y** feature-gated (guidestone) | **Y** L2+ (IPC-wired, 9 scenarios, method.register) | **G** 46 capabilities in TOML + cross-sync 413 | **Y** 4 graphs | **G** 3 exp crates (exp001-003) | **G** 25 (paper-linked) | **G** bans ring/openssl/aws-lc-sys |

## Summary by Axis

### 1. barraCuda Coupling

**Evolving**: ludoSpring leads with `barracuda` as `optional = true` and IPC-first default — the Tier 4 exemplar. wetSpring added `barraCuda IPC routing` (V159) with handler-level `#[cfg(feature = "primal-proof")]` wiring. Other springs still use barraCuda as a library path dependency. This is the primary Tier 4 evolution target.

**Evolution target**: Springs should evolve toward `barracuda` as an **optional** dependency (ludoSpring pattern), with IPC-first paths as the default for NUCLEUS deployment. Now **UNBLOCKED** by JH-11 resolution.

### 2. primalSpring Integration

- **hotSpring**: Only spring with **unconditional** primalSpring dependency (reference implementation pattern)
- **6 springs**: Feature-gated via `guidestone` feature (healthSpring, wetSpring, neuralSpring, ludoSpring, groundSpring, airSpring)

### 3. Guidestone Level

| Level | Springs |
|-------|---------|
| L5 | primalSpring, hotSpring, healthSpring, ludoSpring |
| L4 | wetSpring |
| L4 | groundSpring |
| L3 | neuralSpring |
| L2 | airSpring |

### 4. Capability Registry

- **Sync-tested** (highest maturity): primalSpring (canonical 413), hotSpring, neuralSpring (34 caps), groundSpring (16 MCP + 6 sync tests), healthSpring (118 methods + CI cross-sync), ludoSpring (28 game.* + cross-sync 413), wetSpring (cross-sync 413), airSpring (46 caps + cross-sync 413)
- All 8 springs CI-validated against canonical 413 (May 11)

### 5. Deploy Graphs

Total across ecosystem: **74** (primalSpring) + **5** (hotSpring) + **7** (healthSpring) + **7** (wetSpring) + **4** (neuralSpring) + **12** (ludoSpring) + **6** (groundSpring) + **4** (airSpring) = **119 deploy graphs**

### 6. Composition Experiments

| Tier | Springs |
|------|---------|
| Deep (50+ exp crates) | primalSpring (89), ludoSpring (100), healthSpring (94) |
| Moderate (exp bins/crates) | hotSpring (src/bin exp files), groundSpring (2: exp094/095 w/ CompositionContext), airSpring (3: exp001-003), wetSpring (1: exp400) |
| Minimal (guidestone/playground only) | neuralSpring (IPC in playGround) |

### 7. Paper Baselines

| Tier | Springs |
|------|---------|
| Rich (15+ notebooks) | healthSpring (53), groundSpring (34), airSpring (25), wetSpring (20), hotSpring (17) |
| Moderate (5-15) | neuralSpring (10), primalSpring (5) |
| Scripts only | ludoSpring (baselines/python/) |

### 8. Security Posture (deny.toml)

All 8 springs have workspace-root `deny.toml`. All ban `openssl`/`openssl-sys` and `ring`. hotSpring, healthSpring, and airSpring also ban `aws-lc-sys`/`aws-lc-rs`. airSpring's previous "missing workspace deny.toml" was a **false gap** — it exists and bans aws-lc-sys.

### 9. skunkBat IPC (NEW — May 11)

**8/8 springs wired** — all springs now have Rust-native skunkBat IPC modules
or skunkBat nodes in deploy graphs. The primary gap (ludoSpring, groundSpring,
airSpring had "graphs only") is now **RESOLVED**:
- **Rust IPC module**: wetSpring, neuralSpring, ludoSpring, groundSpring, airSpring, hotSpring
- **Deploy graph node**: healthSpring, ludoSpring (both)
- All modules follow the exemplar pattern: `emit_audit_event()`, socket discovery cascade,
  fire-and-forget semantics, graceful degradation when skunkBat is absent

## Primordial Extinction Wave — COMPLETED (May 9, 2026)

All 8 springs have completed the interstadial eukaryotic evolution:

- **UniBin consolidation**: 8/8 springs have single unified binaries
- **Guidestone absorption**: 8/8 springs have certification organelles
- **Scenario registry**: 8/8 springs have `validation/scenarios/` with `ScenarioMeta`
- **Fossil record**: 8/8 springs have `fossilRecord/` with dated provenance
- **Zero debt markers**: 8/8 springs at zero TODO/FIXME/HACK, zero clippy warnings
- **primalSpring v0.9.25 pin**: 7/8 (healthSpring upgraded; ludoSpring pinned)

### Remaining Coordination Targets (Next Stadial Gate)

1. **barraCuda IPC migration (Tier 4)**: Springs need `barracuda` as `optional = true`
   with IPC-first defaults. ludoSpring exemplar (V61). wetSpring has handler-level
   `primal-proof` wiring (V159). **UNBLOCKED** by JH-11. Owner: **spring teams (L3)**.
2. **`CompositionContext` full migration**: `PrimalClient` still encapsulated inside
   `CompositionContext` in primalSpring (by design). Owner: **primalSpring (L2)**.
3. ~~**airSpring aws-lc-sys ban**~~: **FALSE GAP** — workspace-root `deny.toml` exists
   and bans `aws-lc-sys`. Confirmed May 11.
4. ~~**Registry cross-sync CI**~~: **COMPLETED** — 8/8 springs CI-validated against 413.
5. ~~**GAP-12**~~: **RESOLVED** — 28 `game.*` methods (413 total, zero drift).
6. **barraCuda version alignment**: Springs show mild skew (air v0.3.7, ludo v0.3.11,
   health v0.3.13). Low priority. Owner: **spring teams (L3)**.
7. **Notebook gap**: ludoSpring has Python baselines as scripts, not notebooks —
   functionally equivalent but different artifact form. Owner: **ludoSpring (L3)**.
8. **Guidestone level convergence**: airSpring (L2+) and neuralSpring (L3) below
   ecosystem median (L4). Owner: **spring teams (L3)**.
9. **Foundation seeding**: airSpring (Thread 6 validated), hotSpring (Thread 2 seeded),
   neuralSpring (Threads 5+7 documented). Remaining springs to contribute. Owner: **spring teams (L3) + foundation (L5)**.

## Aggregate Metrics

| Metric | Value |
|--------|-------|
| Total tests across 8 springs | **12,900+** (primalSpring 680 + hotSpring 1,025 + healthSpring 999 + wetSpring 1,962 + neuralSpring 1,450 + ludoSpring 854 + groundSpring 1,101 + airSpring 1,327 + metalForge/integration/Python suites) |
| Total deploy graphs | **119** (primalSpring 74 + hotSpring 5 + healthSpring 7 + wetSpring 7 + neuralSpring 4 + ludoSpring 12 + groundSpring 6 + airSpring 4) |
| Total experiment crates | **293** (primalSpring 89 + ludoSpring 100 + healthSpring 95 + groundSpring 2 + airSpring 3 + wetSpring 1 + hotSpring exp bins + neuralSpring playGround) |
| Total paper notebooks | **198+** (healthSpring 53 + groundSpring 34 + airSpring 25 + wetSpring 20 + hotSpring 17 + neuralSpring 10 + primalSpring 5 + ludoSpring baselines) |
| Registered capability methods | **413** (primalSpring canonical, zero drift) |
| Primals in plasmidBin release | **13/13** (all architectures) |
