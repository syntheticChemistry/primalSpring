# Cross-Spring Composition Parity Scorecard

> papers → Python/R → Rust → primals (IPC) → NUCLEUS composition

**Last updated**: May 9, 2026 — Phase 60+ (v0.9.25)
**Audited by**: primalSpring composition audit
**Method**: Pulled all 8 springs to HEAD, assessed each across 8 axes

## Legend

- **G** = Green (fully implemented / present)
- **Y** = Yellow (partial / in-progress)
- **R** = Red (absent / not started)

## Scorecard

| Spring | Tests | barraCuda Coupling | primalSpring Dep | Guidestone Level | Capability Registry | Deploy Graphs | Composition Experiments | Paper Notebooks | deny.toml |
|--------|------:|-------------------|-----------------|-----------------|-------------------|--------------|----------------------|----------------|-----------|
| **primalSpring** | 680 | None (validates, doesn't consume) | N/A (is primalSpring) | L5 (6 layers) | **G** 389 methods, sync-tested | **G** 74 graphs | **G** 89 exp crates, CompositionContext throughout | **Y** 5 (frozen JSON, not live paper) | **G** bans ring/openssl |
| **hotSpring** | 1,002 | **Y** path dep + IPC | **G** unconditional | **G** L5 (reference impl) | **G** local TOML + sync test | **G** 5 graphs | **Y** exp bins (not crates), CompositionContext in guidestone | **G** 17 (paper-linked) | **G** bans ring/openssl/aws-lc-sys |
| **healthSpring** | 1,002 | **Y** path dep + IPC feature (barracuda-lib) | **Y** feature-gated | **G** L5 (Tier 1-3) | **G** 118 methods in TOML + CI cross-sync vs canonical 389 | **G** 7 graphs | **G** 95 exp crates (exp123 NUCLEUS parity) | **G** 53 .ipynb (all controls converted) | **G** bans ring/openssl/aws-lc-sys |
| **wetSpring** | 1,209 | **Y** path dep + IPC feature (barracuda-lib) | **Y** feature-gated | **G** L4 (38/38 NUCLEUS) | **Y** TOML at root + cross-sync script | **G** 7 graphs | **G** 1 exp crate (exp400 NUCLEUS composition parity) | **G** 19 + Kachkovskiy | **G** bans ring + openssl (Phase 60 absorption) |
| **neuralSpring** | 1,432 | **Y** path dep + IpcMathClient | **Y** feature-gated | **Y** L3 (L4-L5 pending) | **G** local TOML + sync test | **G** 4 graphs (3 new Phase 60) | **G** exp094 parity crate, IPC in playGround | **G** 10 (paper-linked, DOI) | **G** bans ring/openssl/rustls |
| **ludoSpring** | 665+ | **Y** path dep + IPC feature (default) | **Y** feature-gated | **G** L4 (Tier 1-3, 3-tier certification) | **Y** TOML + 15 domain modules, internal sync | **G** 12 graphs | **G** 100 exp fossilized, 5 scenarios absorbed | **R** 0 .ipynb (Python baselines in baselines/) | **G** bans ring/openssl |
| **groundSpring** | 965+ | **Y** path dep (optional, default) | **Y** feature-gated | **G** L4 (guidestone + composition crates) | **G** 16 MCP tools + 6 registry sync tests | **G** 6 graphs | **G** 2 exp crates (exp094/exp095 use CompositionContext) | **G** 34 (paper-linked) | **G** bans ring/openssl |
| **airSpring** | 1,364 | **Y** path dep + some IPC wiring | **Y** feature-gated (guidestone) | **Y** L2 (IPC-wired, 3 composition crates) | **G** 44 capabilities in TOML | **Y** 4 graphs | **G** 3 exp crates (exp001-003) | **G** 25 (paper-linked) | **Y** sub-crate deny, bans ring/openssl (aws-lc-sys not banned) |

## Summary by Axis

### 1. barraCuda Coupling

**Universal gap**: Every spring still uses barraCuda as a library **path dependency**. Some springs also have IPC client paths (healthSpring's `BarraCudaClient`, neuralSpring's `IpcMathClient`, hotSpring's `send_jsonrpc`), but none have fully replaced the path dep with pure IPC. This is the primary sovereign composition gap — the path dep means springs cannot run without the barraCuda source tree present at build time.

**Evolution target**: Springs should evolve toward `barracuda` as an **optional** dependency (like ludoSpring already does with `default-features = false`), with IPC-first paths as the default for NUCLEUS deployment.

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

- **Sync-tested** (highest maturity): primalSpring (canonical 389), hotSpring, neuralSpring, groundSpring (16 MCP + 6 sync tests), healthSpring (118 methods + CI cross-sync vs canonical)
- **TOML present, internal sync**: ludoSpring, wetSpring, airSpring (44 caps)

### 5. Deploy Graphs

Total across ecosystem: **74** (primalSpring) + **1** (hotSpring) + **7** (healthSpring) + **7** (wetSpring) + **1** (neuralSpring) + **12** (ludoSpring) + **6** (groundSpring) + **4** (airSpring) = **112 deploy graphs**

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

All 8 springs have `deny.toml` (or sub-crate deny). All ban `openssl`/`openssl-sys`. All now ban `ring` (wetSpring added in Phase 60 absorption). hotSpring and healthSpring also ban `aws-lc-sys`/`aws-lc-rs`. airSpring's bans are in sub-crate deny files (no workspace-root `deny.toml`); `aws-lc-sys` not explicitly banned there.

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
   with IPC-first defaults. Blocked by JH-11 token federation.
2. **`CompositionContext` full migration**: `PrimalClient` still encapsulated inside
   `CompositionContext` in primalSpring (by design). Full elimination blocked by Tier 4.
3. **airSpring aws-lc-sys ban**: Workspace-root `deny.toml` missing (only sub-crate
   deny files); `aws-lc-sys` not explicitly banned — minor alignment target.
4. **Registry cross-sync CI**: All springs should CI-test local capability methods
   against primalSpring canonical 389 (`config/capability_registry.toml`).
5. **GAP-12**: 15 ludoSpring IPC methods need canonical registration in primalSpring.
6. **barraCuda version alignment**: Springs show mild skew (air v0.3.7, ludo v0.3.11,
   health v0.3.13). Worth aligning in next coordination pass.
7. **Notebook gap**: ludoSpring has Python baselines as scripts, not notebooks —
   functionally equivalent but different artifact form.

## Aggregate Metrics

| Metric | Value |
|--------|-------|
| Total tests across 8 springs | **9,317+** (primalSpring 680 + hotSpring 1,002 + healthSpring 999 + wetSpring 1,209 + neuralSpring 1,432 + ludoSpring 665 + groundSpring 965 + airSpring 1,364 + metalForge/integration suites) |
| Total deploy graphs | **119** (primalSpring 74 + hotSpring 5 + healthSpring 7 + wetSpring 7 + neuralSpring 4 + ludoSpring 12 + groundSpring 6 + airSpring 4) |
| Total experiment crates | **293** (primalSpring 89 + ludoSpring 100 + healthSpring 95 + groundSpring 2 + airSpring 3 + wetSpring 1 + hotSpring exp bins + neuralSpring playGround) |
| Total paper notebooks | **198+** (healthSpring 53 + groundSpring 34 + airSpring 25 + wetSpring 20 + hotSpring 17 + neuralSpring 10 + primalSpring 5 + ludoSpring baselines) |
| Registered capability methods | **389** (primalSpring canonical) |
| Primals in plasmidBin release | **13/13** (all architectures) |
