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

## Upstream Gap Reconciliation (projectNUCLEUS May 9, 2026)

Post-deep-debt-sweep reconciliation from downstream `projectNUCLEUS`:

### Resolved

| ID | What | Resolution |
|----|------|------------|
| DF-2 | toadStool `TOADSTOOL_AUTH_MODE` env | toadStool S233 — `auth.mode` env + `eprintln` → `tracing` |
| DF-3 | songbird/squirrel silent on `auth.mode` TCP | songbird — `CallerContext` wired (TCP transport-aware) |
| U5 | sweetGrass port 39085 vs 9850 | sweetGrass v0.7.32 — port 9850 canonical |
| GAP-12 | 15 ludoSpring IPC methods need canonical registration | **RESOLVED** — 18 `game.*` methods in `config/capability_registry.toml` (403 total, zero drift) |
| U1 | CHECKSUMS stale after Phase 59 refactoring | **RESOLVED** — regenerated with 25 tracked files (UniBin, certification, scenarios, registry) |
| U2 | 5 deploy graphs missing `by_capability` | **FALSE POSITIVE** — only manifests (parameter tables, not node-bearing graphs) lack field; all actual `[[graph.nodes]]` graphs have `by_capability` |
| U3 | 8 profile graphs missing `bonding_policy` | **RESOLVED** — 9/9 profile graphs already have `bonding_policy` |

### Still Open (not primalSpring-owned)

| ID | Owner | Severity | What | primalSpring posture |
|----|-------|----------|------|---------------------|
| JH-11 | bearDog/biomeOS | **HIGH** | Cross-primal token federation | Ready — `TokenVerifier`, scenarios, exp108-111 |
| GAP-06 | rhizoCrypt | MEDIUM | No UDS transport | Blocks 4 ludoSpring provenance experiments |
| GAP-03 | biomeOS | MEDIUM | Cell graph live deploy not tested | Need `composition.deploy(graph)` E2E test |
| GAP-09 | biomeOS | MEDIUM | Neural API registration endpoint | New spring methods need registration path |

### Tier 3 Code Quality (primal team backlogs — coordination tracking)

| Priority | Primal | Issue | Status |
|----------|--------|-------|--------|
| 1 | coralReef | `eprintln!` → `tracing` | Done (Iter 95) |
| 2 | barraCuda | `unwrap()` → `?` in session/ops | Confirmed false positive (optional dep) |
| 3 | nestGate | `unwrap()` → `?` in rpc/discovery | Confirmed false positive (S59) |
| 4 | biomeOS | Mock helpers mixed with production code | Done (v3.49 `#[cfg(test)]` isolation) |
| 5 | bearDog | HSM mock not feature-gated | Done (Wave 98 `#[cfg(test)]`) |
| 6 | petalTongue | Bare `#[allow]` without reason | Done (P6 `#[expect(reason)]`) |
| 7 | squirrel | 1105-line test file | Done (P7 inference dispatch split) |

---

## Next Interstadial Wave — Evolution Goals

These items are the active evolution targets for the next stadial push.
Delta springs have completed the interstadial primordial extinction (8/8
eukaryotic UniBin, May 9, 2026). projectNUCLEUS and downstream products
should absorb the current patterns while these goals mature upstream.

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

**COMPLETED** (May 9, 2026) — All 8 springs have completed the primordial extinction:

1. **UniBin consolidation**: 8/8 — all springs have single unified binaries with
   `certify`/`validate`/`status`/`version` subcommands (most also have `serve`).
2. **Guidestone absorption**: 8/8 — certification engine absorbed as library organelle.
3. **Deprecated API cleanup**: 8/8 — zero bare `#[allow(deprecated)]` suppressions.
4. **primalSpring v0.9.25 pin**: 7/8 pinned (ludoSpring pinned, healthSpring upgraded).
5. **Fossil record**: 8/8 — `fossilRecord/` with dated provenance READMEs.
6. **Zero debt**: 8/8 — zero TODO/FIXME/HACK, zero clippy warnings, zero test failures.

| Spring | Post-Evolution State | Next Target |
|--------|---------------------|-------------|
| healthSpring | V61, gS L5, UniBin, 999+ tests, 0 clippy | Tier 4 (JH-11 blocked) |
| ludoSpring | V58, gS L4, UniBin, 665+ tests, 100 exps fossilized | Tier 4, `CompositionContext` |
| hotSpring | latest, gS L5, UniBin, 1,002 tests | `serve` on UniBin, Tier 3 |
| wetSpring | V155, gS L4, UniBin, 1,209 tests | Tier 3 → 4 |
| airSpring | v0.10.0, gS L2, UniBin, 1,364 tests | gS L4+, Tier 3 |
| neuralSpring | S195, gS L3, UniBin, 1,432 tests | gS L4+, latency-aware |
| groundSpring | V127, gS L4, UniBin, 965+ tests | `serve` on UniBin, Tier 2 |

**Remaining for Wave 3**: `CompositionContext` full migration (PrimalClient still
encapsulated), Tier 4 rewiring (blocked by JH-11 token federation), barraCuda
version alignment across springs.

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
