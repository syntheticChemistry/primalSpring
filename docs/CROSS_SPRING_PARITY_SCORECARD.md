# Cross-Spring Composition Parity Scorecard

> papers → Python/R → Rust → primals (IPC) → NUCLEUS composition

**Last updated**: May 8, 2026 — Phase 60 (v0.9.25)
**Audited by**: primalSpring composition audit
**Method**: Pulled all 8 springs to HEAD, assessed each across 8 axes

## Legend

- **G** = Green (fully implemented / present)
- **Y** = Yellow (partial / in-progress)
- **R** = Red (absent / not started)

## Scorecard

| Spring | Tests | barraCuda Coupling | primalSpring Dep | Guidestone Level | Capability Registry | Deploy Graphs | Composition Experiments | Paper Notebooks | deny.toml |
|--------|------:|-------------------|-----------------|-----------------|-------------------|--------------|----------------------|----------------|-----------|
| **primalSpring** | 666 | None (validates, doesn't consume) | N/A (is primalSpring) | L5 (6 layers) | **G** 389 methods, sync-tested | **G** 74 graphs | **G** 85 exp crates, 4 use CompositionContext | **Y** 5 (frozen JSON, not live paper) | **G** bans ring/openssl |
| **hotSpring** | 993 | **Y** path dep + IPC | **G** unconditional | **G** L5 (reference impl) | **G** local TOML + sync test | **Y** 1 graph | **Y** exp bins (not crates), CompositionContext in guidestone | **G** 17 (paper-linked) | **G** bans openssl, ring via rustls |
| **healthSpring** | 948+ | **Y** path dep + typed IPC clients | **Y** feature-gated | **G** L5 (Tier 1-3) | **Y** Rust constants, no TOML | **G** 7 graphs | **G** 94 exp crates, live IPC in exp117-122 | **R** 0 .ipynb (54 Python scripts) | **G** bans openssl, ring via rustls |
| **wetSpring** | 1,594 | **Y** path dep + IPC feature | **Y** feature-gated | **G** L4 (38/38 NUCLEUS) | **Y** TOML at root, no cross-sync | **G** 7 graphs | **R** 0 exp crates (CompositionContext in guidestone) | **G** 19 (tiered) | **Y** bans openssl, ring not banned |
| **neuralSpring** | 1,387 | **Y** path dep + IpcMathClient | **Y** feature-gated | **Y** L3 (L4-L5 pending) | **G** local TOML + sync test | **Y** 1 graph | **R** 0 exp crates (IPC in playGround) | **G** 10 (paper-linked, DOI) | **G** bans ring/openssl/rustls |
| **ludoSpring** | 820 | **Y** path dep + IPC feature | **Y** feature-gated | **G** L5 (Tier 1-3) | **Y** TOML + internal sync, no cross-sync | **G** 12 graphs | **G** 100 exp crates, many with IPC | **R** 0 .ipynb (Python baselines in baselines/) | **G** bans ring/openssl |
| **groundSpring** | 965+ | **Y** path dep (optional, default) | **Y** feature-gated | **Y** L3 | **Y** TOML at root, no sync test | **G** 6 graphs | **R** 0 exp crates (CompositionContext in guidestone) | **G** 34 (paper-linked) | **G** bans ring/openssl |
| **airSpring** | 1,364 | **Y** path dep + some IPC wiring | **R** absent | **R** L1 (P1-P5 only) | **R** no registry file | **Y** 4 graphs | **R** 0 exp crates | **G** 25 (paper-linked) | **Y** sub-crate deny, bans ring/openssl |

## Summary by Axis

### 1. barraCuda Coupling

**Universal gap**: Every spring still uses barraCuda as a library **path dependency**. Some springs also have IPC client paths (healthSpring's `BarraCudaClient`, neuralSpring's `IpcMathClient`, hotSpring's `send_jsonrpc`), but none have fully replaced the path dep with pure IPC. This is the primary sovereign composition gap — the path dep means springs cannot run without the barraCuda source tree present at build time.

**Evolution target**: Springs should evolve toward `barracuda` as an **optional** dependency (like ludoSpring already does with `default-features = false`), with IPC-first paths as the default for NUCLEUS deployment.

### 2. primalSpring Integration

- **hotSpring**: Only spring with **unconditional** primalSpring dependency (reference implementation pattern)
- **5 springs**: Feature-gated via `guidestone` feature (healthSpring, wetSpring, neuralSpring, ludoSpring, groundSpring)
- **airSpring**: No Rust dependency at all (filesystem path references only)

### 3. Guidestone Level

| Level | Springs |
|-------|---------|
| L5 | primalSpring, hotSpring, healthSpring, ludoSpring |
| L4 | wetSpring |
| L3 | neuralSpring, groundSpring |
| L1 | airSpring |

### 4. Capability Registry

- **Sync-tested** (highest maturity): primalSpring (canonical), hotSpring, neuralSpring
- **TOML present, internal sync**: ludoSpring, wetSpring, groundSpring
- **Rust constants only**: healthSpring
- **Absent**: airSpring

### 5. Deploy Graphs

Total across ecosystem: **74** (primalSpring) + **1** (hotSpring) + **7** (healthSpring) + **7** (wetSpring) + **1** (neuralSpring) + **12** (ludoSpring) + **6** (groundSpring) + **4** (airSpring) = **112 deploy graphs**

### 6. Composition Experiments

| Tier | Springs |
|------|---------|
| Deep (50+ exp crates) | primalSpring (85), ludoSpring (100), healthSpring (94) |
| Moderate (exp bins/IPC) | hotSpring (src/bin exp files) |
| Minimal (guidestone only) | wetSpring, neuralSpring, groundSpring, airSpring |

### 7. Paper Baselines

| Tier | Springs |
|------|---------|
| Rich (15+ notebooks) | groundSpring (34), airSpring (25), wetSpring (19), hotSpring (17) |
| Moderate (5-15) | neuralSpring (10), primalSpring (5) |
| Scripts only | healthSpring (54 .py), ludoSpring (baselines/python/) |

### 8. Security Posture (deny.toml)

All 8 springs have `deny.toml` (or sub-crate deny). All ban `openssl`/`openssl-sys`. Most ban `ring` outright; healthSpring and hotSpring allow `ring` only as a transitive via `rustls`. wetSpring does not explicitly ban `ring`.

## Critical Path to Full Parity

1. **barraCuda IPC migration**: All springs need `barracuda` as `optional = true` with IPC-first defaults
2. **airSpring bootstrap**: Needs primalSpring feature-gated dep, registry TOML, guidestone upgrade to L3+
3. **Registry cross-sync**: Only hotSpring and neuralSpring auto-test against their local TOML; no spring tests against primalSpring's canonical 389-method registry
4. **wetSpring ring ban**: Should align with ecosystem posture
5. **Notebook gap**: healthSpring and ludoSpring have Python baselines as scripts, not notebooks — functionally equivalent but different artifact form

## Aggregate Metrics

| Metric | Value |
|--------|-------|
| Total tests across 8 springs | **8,737+** |
| Total deploy graphs | **112** |
| Total experiment crates | **279** (primalSpring 85 + ludoSpring 100 + healthSpring 94) |
| Total paper notebooks | **110** |
| Registered capability methods | **389** (primalSpring canonical) |
| Primals in plasmidBin release | **13/13** (all architectures) |
