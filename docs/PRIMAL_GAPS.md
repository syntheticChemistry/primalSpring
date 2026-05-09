# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.

> **Scope**: NUCLEUS primals only (13 core + compute/ecosystem primals).
> Downstream springs and gardens own their own debt and are NOT tracked here.
>
> **Current phase: INTERSTADIAL** — stadial gate cleared April 16, 2026.
> All 13 primals at modern async Rust parity: `async-trait` eliminated (13/13),
> enum dispatch (13/13), `cargo deny check bans` passes (13/13), Edition 2024 (13/13).
>
> **Last updated**: 2026-05-09
>
> **Full history**: archived in `fossilRecord/primal_gaps_phase60_may2026/PRIMAL_GAPS_FULL_HISTORY.md`

---

## Ecosystem Status (May 9, 2026)

**265 PASS, 0 FAIL, 0 KNOWN_GAP** — projectNUCLEUS Phase 60 validation complete.

| Primal | Tests | JH-0 | BTSP P3 | Wire Std | Debt Status |
|--------|------:|:----:|:-------:|:--------:|-------------|
| bearDog | 14,784+ | **ADOPTED** | FULL | L2 | **CLEAN** — HSM mock `#[cfg(test)]` (Wave 98) |
| songbird | 7,178+ | **ADOPTED** | FULL | L3 | **CLEAN** — DF-3 CallerContext wired (TCP transport-aware) |
| toadStool | 22,833+ | **ADOPTED** | FULL | L3 | **CLEAN** — DF-2 auth.mode env + eprintln→tracing (S233) |
| biomeOS | 7,919 | **ADOPTED** | FULL | consumer | **CLEAN** — test helpers gated `#[cfg(test)]` (v3.49) |
| nestgate | 8,915 | **ADOPTED** | FULL | L3 | **CLEAN** — NG-13/14/15 false positives confirmed (S59) |
| squirrel | 7,178 | **ADOPTED** | FULL | L2 | **CLEAN** — 1105L test split, inference dispatch (P7) |
| barraCuda | 4,422+ | **ADOPTED** | FULL | L2 | **CLEAN** — unwrap false positive confirmed, optional dep supported |
| petalTongue | varies | **ADOPTED** | FULL | L2/L3 | **CLEAN** — all `#[allow]` → `#[expect(reason)]` (P6) |
| rhizoCrypt | 1,602 | **ADOPTED** | FULL | L3 | **CLEAN** — canonical "dag" domain clarified (S64) |
| loamSpine | 1,442+ | **ADOPTED** | FULL | L3 | **CLEAN** |
| sweetGrass | 1,522 | **ADOPTED** | FULL | L3 | **CLEAN** — JH-0 gate + port 9850 canonical |
| coralReef | 4,506+ | **ADOPTED** | FULL | L2 | **CLEAN** — eprintln→tracing in 5 driver files (Iter 95) |
| skunkBat | 363+ | **ADOPTED** | FULL | L2 | **CLEAN** — JH-5 Phase 2 event instrumentation complete |

**13/13 at zero debt. 5 wave goals defined for next interstadial push (see below).**

---

## Next Interstadial Wave — Evolution Goals

These items are the active evolution targets for the next stadial push.
Delta springs and projectNUCLEUS should absorb the current interstadial
patterns (UniBin, two-tier validation, `CompositionContext`, deprecated
harness fossilization) while these goals mature upstream.

### Wave 1: JH-11 — Cross-Primal Token Federation

**Owner**: bearDog + biomeOS (ecosystem architecture)
**Priority**: HIGH — unlocks JH-5, cross-atomic auth, and Tier 4 rewiring for all springs
**Target**: Next stadial gate

Each primal's MethodGate validates ionic tokens independently. BearDog-issued tokens
cannot currently be verified by other primals without shared key distribution. biomeOS
composition forwarding (`_resource_envelope` in v3.48) is the production workaround.

**primalSpring readiness**: Full scope-checked token validation reference implementation
(`TokenVerifier` trait, `BearDogVerifier` calls `auth.verify_ionic` via IPC,
`scope_permits_method()` pattern matching). `CompositionContext::call_authenticated()`
threads bearer tokens through multi-capability graphs. UniBin scenarios
`s_bearer_token`, `s_gate_failure`, `s_gate_routing` pressure-test the federation
contract. Experiments exp108-exp111 cover the full auth surface.

**Remaining blockers**:
- bearDog: key distribution API for cross-primal shared verification
- biomeOS: forwarding-path token propagation through composition routing
- primalSpring: ready to validate once bearDog + biomeOS ship

**Delta spring impact**: Springs at Tier 3+ rewiring will need authenticated IPC
for Tier 4 (binary-only). JH-11 is the gate.

---

### Wave 2: JH-5 — Cross-Primal Audit Log Forwarding

**Owner**: skunkBat + rhizoCrypt + sweetGrass
**Priority**: MEDIUM — blocked by JH-11
**Target**: Post JH-11 delivery

skunkBat Phase 2 (local event instrumentation) is complete — all 7 event kinds emit
from live code paths. Phase 3 (forwarding security events to rhizoCrypt DAG +
sweetGrass braid) requires cross-primal IPC with authenticated tokens (JH-11).

**Delta spring impact**: Once JH-5 ships, every spring gains cross-primal audit
logging for free via biomeOS composition routing. Springs should prepare by
wiring skunkBat into their deploy graphs now.

---

### Wave 3: Primordial Extinction — Delta Spring Pattern Evolution

**Owner**: All delta springs (hotSpring, wetSpring, neuralSpring, healthSpring,
ludoSpring, groundSpring, airSpring)
**Priority**: HIGH — the primary interstadial work for delta teams
**Target**: Before next stadial gate

Delta springs must undergo the primordial extinction that primalSpring completed:

1. **UniBin consolidation**: Absorb experiment bins into a single unibin per spring.
   primalSpring's `ScenarioRegistry` + `ScenarioMeta` + `Tier`/`Track` taxonomy
   is the template. Fossilize prokaryotic binaries.
2. **Guidestone absorption**: Absorb guidestone layers as library organelles
   (certification engine pattern from primalSpring `certification/` module).
3. **CompositionContext migration**: Replace `PrimalClient`, `AtomicHarness`,
   `discover_primal()`, `spawn_primal()` with `CompositionContext` throughout.
4. **Deprecated API cleanup**: All `#[allow(deprecated)]` must carry `reason = "..."`.
   Zero bare suppressions.
5. **primalSpring v0.9.25 pin**: All springs must pin to v0.9.25 for UniBin,
   `CompositionContext`, and two-tier validation support.
6. **Fossil record**: Snapshot pre-extinction patterns to `fossilRecord/` with
   dated provenance READMEs.

| Spring | Current State | Target |
|--------|--------------|--------|
| healthSpring | Tier 3, gS L5, pinned to v0.9.17 | Pin v0.9.25, UniBin, Tier 4 |
| ludoSpring | Tier 3, gS L4, 100 exp crates | UniBin, Tier 4, make `ipc` default |
| hotSpring | Tier 2, gS L5, no `src/ipc/` | Create `src/ipc/`, UniBin, Tier 3 |
| wetSpring | Tier 2, gS L4+, 15 IPC handlers | Route compute through ecobin, UniBin |
| airSpring | Tier 2, gS L2, pre-delta | Unpause delta, UniBin, gS L4+ |
| neuralSpring | Tier 2, gS L3, no `src/ipc/` | Create `src/ipc/`, UniBin, latency-aware |
| groundSpring | Tier 1, gS L4, minimal IPC | Create `src/ipc/`, UniBin, Tier 2 |

---

### Wave 4: PG-63 — Matplotlib Agg Guidance Reconciliation

**Owner**: sporePrint / wateringHole docs
**Priority**: LOW (documentation inconsistency)
**Target**: Next docs pass

`CONTENT_GUIDE.md` says "don't set Agg" while `SPRING_EVOLUTION_TARGETS.md` says
"use Agg" for notebook rendering. Reconcile during next documentation wave.

---

### Wave 5: PG-54 — Adaptive Composition Tick Model

**Owner**: primalSpring composition library + biomeOS
**Priority**: LOW (deferred by design)
**Target**: Post Tier 4 rewiring

Fixed `POLL_INTERVAL` (0.5s) in `nucleus_composition_lib.sh` doesn't suit all domains.
Evolution: allow domain hooks to specify tick mode (fixed, adaptive, event-driven).
ludoSpring's 60Hz tick-budget constraint (0.6ms game.tick) is the stress test —
once ludoSpring achieves Tier 4 with acceptable IPC latency, the tick model can
generalize.

---

## Compliance Summary

All 13 primals share these invariants (regressions are rejected):

| Invariant | Status |
|-----------|--------|
| `async-trait` eliminated | **13/13** |
| Enum dispatch (finite implementors) | **13/13** |
| `cargo deny check bans` (ring/openssl/aws-lc-sys banned) | **13/13** |
| Edition 2024 | **13/13** |
| JH-0 MethodGate pre-dispatch authorization | **13/13** |
| BTSP Phase 3 (ChaCha20-Poly1305 AEAD) | **13/13** |
| Capability Wire Standard L2+ | **13/13** |
| `--bind` / localhost-default (PG-55) | **13/13** |
| plasmidBin musl-static ecoBin | **13/13** |
| `forbid(unsafe_code)` or justified opt-out | **13/13** |

---

## Portability Posture

| Class | Issue | Status |
|-------|-------|--------|
| C Crypto (`ring`) | BearDog pure-Rust delegation, `deny.toml` bans | **RESOLVED** (13/13) |
| GPU/Vulkan (`wgpu`) | barraCuda 4-tier fallback (GPU→CPU→IPC→scalar) | **RESOLVED** |
| Remaining C surfaces | All feature-gated or target-gated | **ACCEPTABLE** |
| `ring` lockfile ghost | Cargo v4 artifact, never compiled | **NOT ACTIONABLE** |

---

## Per-Primal Quick Reference

Detailed per-primal gap tables, BTSP compliance matrices, capability wire standard
levels, plasmidBin binary inventory, and historical resolution logs are archived in:

`fossilRecord/primal_gaps_phase60_may2026/PRIMAL_GAPS_FULL_HISTORY.md`

Key per-primal handoffs in `infra/wateringHole/handoffs/`:

| Primal | Latest Handoff |
|--------|---------------|
| toadStool | `TOADSTOOL_S233_DF2_AUTH_MODE_ENV_EPRINTLN_MIGRATION_MAY08_2026.md` |
| biomeOS | `BIOMEOS_V349_TEST_HELPER_ISOLATION_HANDOFF_MAY08_2026.md` |
| bearDog | `BEARDOG_V090_WAVE97_CROSS_FAMILY_CONTRACTS_SESSION_UX_HANDOFF_MAY08_2026.md` |
| squirrel | `SQUIRREL_V010_PRIMALSPRING_P7_CODE_QUALITY_HANDOFF_MAY08_2026.md` |
| barraCuda | `BARRACUDA_V0313_SPRINT56_AUDIT_TRIAGE_MAY08_2026.md` |
| rhizoCrypt | (S64 in wateringHole) |
| sweetGrass | `SWEETGRASS_V0732_JH0_METHOD_GATE_HANDOFF_MAY08_2026.md` |
| nestgate | `NESTGATE_V470_SESSION59_JH0_METHOD_GATE_MAY08_2026.md` |
| petalTongue | `PETALTONGUE_V166_JH0_METHOD_GATE_HANDOFF_MAY08_2026.md` |
| skunkBat | `SKUNKBAT_V020DEV_JH5_AUDIT_LOG_HANDOFF_MAY08_2026.md` |
