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

**13/13 at zero debt. JH-11 token federation deferred ecosystem-wide.**

---

## Open Items

### 1. JH-11: Cross-Primal Token Federation (Deferred)

**Owner**: Ecosystem architecture (bearDog + biomeOS)
**Severity**: Medium (architecture gap, not blocking)
**Deferred until**: Tier 1-2 debt fully cleared

Each primal's MethodGate validates ionic tokens independently. BearDog-issued tokens
cannot currently be verified by other primals without shared key distribution. biomeOS
composition forwarding (`_resource_envelope` in v3.48) is the production workaround.

**Prerequisite**: All primals must implement JH-0 token scope validation (currently
presence-only, not scope-checked). bearDog's `auth.verify_ionic` exists but no primal
calls it on inbound requests yet.

---

### 2. JH-5: Cross-Primal Audit Log Forwarding (Deferred)

**Owner**: skunkBat + rhizoCrypt + sweetGrass
**Severity**: Low
**Deferred until**: JH-11 token federation path is clearer

skunkBat Phase 2 (local event instrumentation) is complete — all 7 event kinds emit
from live code paths. Phase 3 (forwarding security events to rhizoCrypt DAG +
sweetGrass braid) requires cross-primal IPC with authenticated tokens.

---

### 3. PG-63: Matplotlib Agg Guidance Conflict (Minor)

**Owner**: sporePrint / wateringHole docs
**Severity**: Low (documentation inconsistency)

`CONTENT_GUIDE.md` says "don't set Agg" while `SPRING_EVOLUTION_TARGETS.md` says
"use Agg" for notebook rendering. Needs reconciliation.

---

### 4. PG-54: Adaptive Sensor Polling (Deferred by Design)

**Owner**: primalSpring composition library
**Severity**: Low

Fixed `POLL_INTERVAL` (0.5s) in `nucleus_composition_lib.sh` doesn't suit all domains.
Future: allow domain hooks to specify tick mode (fixed, adaptive, event-driven).

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
