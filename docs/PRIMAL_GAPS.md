# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.

> **Scope**: NUCLEUS primals only (13 core + compute/ecosystem primals).
> Downstream springs and gardens own their own debt and are NOT tracked here.
>
> **Current phase: INTERSTADIAL** — stadial gate cleared April 16, 2026.
> All 13 primals at modern async Rust parity: `async-trait` eliminated (13/13),
> enum dispatch (13/13), `cargo deny check bans` passes (13/13), Edition 2024 (13/13).
>
> **Last updated**: 2026-05-11
>
> **Full history**: archived in `fossilRecord/primal_gaps_phase60_may2026/PRIMAL_GAPS_FULL_HISTORY.md`

---

## Ecosystem Status (May 11, 2026)

**267+ PASS, 0 FAIL, 0 KNOWN_GAP** — projectNUCLEUS Phase 60+ validation, darkforest v0.2.1.

| Primal | Tests | JH-0 | BTSP P3 | Wire Std | Debt Status |
|--------|------:|:----:|:-------:|:--------:|-------------|
| bearDog | 14,784+ | **ADOPTED** | FULL | L2 | **CLEAN** — HSM mock `#[cfg(test)]` (Wave 98) |
| songbird | 7,178+ | **ADOPTED** | FULL | L3 | **CLEAN** — DF-3 CallerContext wired (TCP transport-aware) |
| toadStool | 22,833+ | **ADOPTED** | FULL | L3 | **CLEAN** — DF-2 auth.mode env + eprintln→tracing (S233) |
| biomeOS | 7,919 | **ADOPTED** | FULL | consumer | **CLEAN** — test helpers gated `#[cfg(test)]` (v3.49) |
| nestgate | 8,915+ | **ADOPTED** | FULL | L3 | **CLEAN** — dep hygiene + hardcode cleanup (S61), content.* transport parity (S60) |
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
| GAP-12 | 15 ludoSpring IPC methods need canonical registration | **RESOLVED** — 28 `game.*` methods in `config/capability_registry.toml` (413 total, zero drift) |
| U1 | CHECKSUMS stale after Phase 59 refactoring | **RESOLVED** — regenerated with 25 tracked files (UniBin, certification, scenarios, registry) |
| U2 | 5 deploy graphs missing `by_capability` | **FALSE POSITIVE** — only manifests (parameter tables, not node-bearing graphs) lack field; all actual `[[graph.nodes]]` graphs have `by_capability` |
| U3 | 8 profile graphs missing `bonding_policy` | **RESOLVED** — 9/9 profile graphs already have `bonding_policy` |

### Resolved (upstream evolution wave May 10, 2026)

| ID | Owner | What | Resolution |
|----|-------|------|------------|
| JH-11 | bearDog/biomeOS | Cross-primal token federation | **RESOLVED** — bearDog Wave 99 `auth.public_key` (Ed25519 key distribution) + biomeOS v3.51 `BearDogVerifier` (IPC-based cross-primal verification) |
| GAP-06 | rhizoCrypt | No UDS transport | **RESOLVED** — S66 confirms UDS operational since S23, provenance trio integration test added |
| GAP-03 | biomeOS | Cell graph live deploy not tested | **RESOLVED** — biomeOS v3.51 `composition.deploy` route alias for `graph.execute` |
| GAP-09 | biomeOS | Neural API registration endpoint | **RESOLVED** — biomeOS v3.51 `method.register` endpoint for spring method registration |

Also resolved by upstream teams (not previously tracked as gaps):

| What | Resolution |
|------|------------|
| `composition.status` method | biomeOS v3.51 — `{ active_users, primal_health, resource_pressure }` |
| bearDog TLS + rate limiting (H2-10/H2-11) | bearDog Wave 100 — rustls X.509 termination + per-IP sliding-window rate limiter |
| petalTongue PT-1 through PT-5 (sovereignty) | All resolved — `--docroot`, `WebServeConfig`, `--ipc`, `--workers`, NestGate content backend (PT-13) |
| petalTongue notebook rendering | `.ipynb` → HTML with `metadata.title` + `strip_sources` |
| songbird NAT traversal (H2-13 through H2-16) | Wave 196-197 — STUN wire-compliant, RFC 5766 TURN client, Cloudflare DDNS, 5-tier `ConnectionFallbackChain` all live |
| biomeOS token forwarding | v3.50 — `_bearer_token` propagated through all capability routing paths |

### Zero open upstream gaps

All upstream gaps from the projectNUCLEUS audit are now resolved. Remaining items are
hardening and future-horizon work (Tor relay, QUIC multi-path, full `cloudflared`
orchestration, TURN refresh lifecycle) — none blocking current interstadial goals.

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

**RESOLVED** (May 10, 2026)

- bearDog Wave 99: `auth.public_key` endpoint — Ed25519 verifying key in base64/hex/DID
  formats. Any primal can call once, cache key, verify ionic tokens locally.
- biomeOS v3.50: `_bearer_token` propagated through all capability routing paths.
- biomeOS v3.51: `BearDogVerifier` for IPC-based cross-primal token verification.
  Degrades gracefully to local parsing when bearDog unreachable.
- primalSpring: `TokenVerifier` trait, `scope_permits_method()`, `call_authenticated()`,
  scenarios `s_bearer_token`/`s_gate_failure`/`s_gate_routing`, experiments exp108-111.

**JH-5 (audit forwarding) and Tier 4 rewiring are now unblocked.**

---

### Wave 2: JH-5 — Cross-Primal Audit Log Forwarding

**Owner**: skunkBat + rhizoCrypt + sweetGrass
**Priority**: MEDIUM — **UNBLOCKED** (JH-11 resolved May 10, 2026)
**Target**: Next coordination pass

skunkBat Phase 2 (local event instrumentation) is complete — all 7 event kinds emit
from live code paths. Phase 3 (forwarding security events to rhizoCrypt DAG +
sweetGrass braid) via authenticated cross-primal IPC is now unblocked.

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
| healthSpring | V62, gS L5, UniBin, 999 tests, skunkBat in graphs | Tier 4 rewiring |
| ludoSpring | V61, gS L4, UniBin, 854 tests, skunkBat IPC, optional barraCuda | foundation seeding, plasmidBin |
| hotSpring | v0.6.32+, gS L6, UniBin, 1,025 tests, 188 experiments | sovereign GPU barriers |
| wetSpring | V159, gS L4, UniBin, 1,962 tests, barraCuda IPC routing | 8 open gaps (was 15) |
| airSpring | v0.10.0, gS L2+, UniBin, 1,327 tests, 9 scenarios | gS L4+, Tier 4 |
| neuralSpring | S199/V149, gS L3, UniBin, 1,450 tests, compute.dispatch | gS L4+, foundation Threads 5+7 |
| groundSpring | V131, gS L4, UniBin, 1,101 tests, guidestone modularized | plasmidBin, Tier 4 |

**Wave 3 COMPLETED** (May 9). Post-interstadial push (May 10-11) achieved:
8/8 skunkBat Rust IPC, 8/8 `method.register`, 8/8 CI cross-sync 413,
8/8 `composition.status`, 8/8 NUCLEUS workload TOMLs. Tier 4 rewiring
and `CompositionContext` migration now **UNBLOCKED** by JH-11.

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
| nestgate | `NESTGATE_V470_SESSION61_DEEP_DEBT_HYGIENE_MAY11_2026.md` |
| petalTongue | `PETALTONGUE_V166_JH0_METHOD_GATE_HANDOFF_MAY08_2026.md` |
| skunkBat | `SKUNKBAT_V020DEV_JH5_AUDIT_LOG_HANDOFF_MAY08_2026.md` |

---

## Evolution Cycle Ownership Model

Every gap in the ecosystem belongs to exactly one layer of the evolution cycle.
When a gap is identified, it should be tagged with its owner layer. This prevents
ambiguity about who acts on what, and which gaps block downstream work.

### Sentinel-Stadial Model (May 11, 2026)

Primals are **sentinels** — the least composed, most climate-responsive entities
in the ecosystem. They feel shifts first and respond first. They are already in
their own **stadial cycle**, with primalSpring as their **external validation
gate**. This is analogous to how Cloudflare/Barrick are stadial gates for
downstream products.

```
L1 (Primals — sentinel-stadial)
  │ validated against
  ▼
L2 (primalSpring — stadial gate for primals)
  │ 413 registry, MethodGate enforcement, deploy graph coherence,
  │ guidestone certification, CompositionContext contracts
  │
  │ patterns flow downstream
  ▼
L3 (Springs — interstadial) → L4 (Products — interstadial) → L5 (Foundation)
```

The key distinction: **primals are ahead of the ecosystem**. They have shipped
their capabilities. primalSpring is the **pressure** that validates quality — any
primal not passing the gate creates upstream debt that blocks everything
downstream. The river delta and products are still interstadial, absorbing
primal capabilities into compositions and deployments.

### Layer 1: Upstream Primals — Sentinel-Stadial (13 core primals)

**Owner**: Individual primal teams (bearDog, songbird, toadStool, etc.)
**Scope**: Primal-internal code quality, capability correctness, IPC contracts
**Phase**: **Stadial** — capabilities shipped, responding to gate pressure
**Current**: **13/13 passing the primalSpring gate.** Zero upstream debt.
All primals: MethodGate (JH-0 + JH-2), BTSP Phase 3 AEAD, Edition 2024,
deny.toml (ring + openssl banned), plasmidBin musl-static ecoBin.

**Stadial pressure on primals** (primalSpring as gate):
- 413-method canonical registry — drift is rejected
- MethodGate enforcement — 12/13, squirrel remaining
- Deploy graph coherence — all primals must compose cleanly
- Guidestone certification — primals participate in spring gS levels
- Upstream crate extraction (stadial external) — wgsl-precision, proc-sysinfo
- Framework parity (stadial external) — Kokkos, LAMMPS, SciPy benchmarks

### Layer 2: primalSpring — The Stadial Gate

**Owner**: primalSpring team
**Scope**: Canonical capability registry (413 methods), deploy graph library,
composition validation, gap registry, `CompositionContext` API, two-tier
validation harness (Tier 1 Rust / Tier 2 Live IPC), guidestone certification
**Role**: **Stadial gate for L1 primals.** The registry, MethodGate check,
graph coherence, and guidestone layers are the validation pressure that
primals must pass. Patterns validated here flow downstream to springs/products.
**Current**: 413 methods, 680 tests, zero debt. Active coordination targets:
- PG-54: Adaptive composition tick model (LOW, deferred post-Tier 4)
- PG-63: Matplotlib Agg guidance reconciliation (LOW, docs)

### Layer 3: River Delta — Interstadial (8 springs)

**Owner**: Individual spring teams
**Scope**: Domain science, spring-internal debt, barraCuda coupling, gS levels,
foundation seeding, plasmidBin release readiness
**Phase**: **Interstadial** — absorbing primal capabilities, pre-wiring compositions
**Current**: Post-interstadial targets all green (8/8 on 5 axes). Per-spring:

| Spring | Open Gaps | Owner-Layer Items |
|--------|-----------|-------------------|
| wetSpring | 8 (PG-01–PG-22, 14 closed) | barraCuda IPC expansion, remaining PG gaps |
| hotSpring | 0 local | sovereign GPU barriers (coralReef), GAP-HS-030 renumber |
| neuralSpring | 0 local | gS L3→L4, foundation Threads 5+7 |
| airSpring | 0 local | gS L2→L4 |
| ludoSpring | 0 local | foundation seeding, plasmidBin release |
| groundSpring | 0 local | plasmidBin release, PRNG Phase 2b (barraCuda) |
| healthSpring | 0 local | none identified |

### Layer 4: Downstream Products — Interstadial (projectNUCLEUS, gardens)

**Owner**: Product teams
**Scope**: Gate deployment, sovereignty horizons, composition absorption,
workload validation, foundation integration
**Phase**: **Interstadial** — pre-wiring sovereignty, shadow runs not yet started
**Current** (projectNUCLEUS):
- Horizon 1: **COMPLETE** — external security, darkforest v0.2.1
- Horizon 2: **70%** — 2a done, 2b ready, 3a intermediate (cell membrane live), 3b/3c upstream shipped, 4 intermediate (DoT)
- Horizon 3: **20%** — H3-07/H3-08 unblocked, rest future
- Absorption targets: `composition.deploy(graph)`, Tier 4 rewiring, skunkBat in smaller compositions

### Layer 5: Foundation (sporeGarden/foundation)

**Owner**: Foundation team + contributing springs
**Scope**: Public data anchoring, provenance validation, thread coverage
**Current**: 10 domain threads, 100+ data sources. Springs seeding:
- airSpring: Thread 6 (ag) — 36/36 targets validated
- hotSpring: Thread 2 seeded
- neuralSpring: Threads 5+7 documented, ready for contribution
- groundSpring: Thread 7 (Anderson) index fixed

### Gap Flow — Sentinel Model

```
L1 (Primals — sentinels, stadial-first)
  │
  │ validated against ↓
  │
L2 (primalSpring — stadial gate)
  │ 413 registry, MethodGate, deploy graphs, guidestone cert
  │
  │ patterns flow downstream ↓
  │
L3 (Springs — interstadial, absorbing primal capabilities)
  │ domain science, IPC rewiring, foundation seeding
  │
  │ compositions flow downstream ↓
  │
L4 (Products — interstadial, pre-wiring sovereignty)
  │ shadow runs, deployment, external-facing artifacts
  │
  │ data anchoring ↓
  │
L5 (Foundation — knowledge layer, thread coverage)
```

Gaps propagate **upward** (springs expose primal gaps → primalSpring gates them
→ primals resolve). Patterns propagate **downward** (primals ship capabilities
→ primalSpring validates → springs absorb → products deploy).

---

## Wave 6: Targeted GuideStone (LTEE) — May 11, 2026

The ecosystem's first **deployable subsystem**: a self-contained, USB-portable
artifact that leaves ecosystem possession. The LTEE guideStone reproduces
Barrick/Lenski LTEE papers and generates new predictions via the Anderson
disorder framework. This is a **projectNUCLEUS subsystem**.

Standard: `infra/wateringHole/TARGETED_GUIDESTONE_STANDARD.md`
Handoff: `infra/wateringHole/handoffs/LTEE_GUIDESTONE_SUBSYSTEM_HANDOFF_MAY11_2026.md`

### Wave 6 Ownership

| Layer | Responsibility | Status |
|-------|---------------|--------|
| L2 (primalSpring) | Targeted GuideStone standard, scope graph schema, validation harness pattern | **DONE** — standard defined |
| L3 (springs) | LTEE paper queue items (36 assignments across 6 springs), binary builds, scenario implementations | **SEEDED** — queues populated, reproduction work begins |
| L4 (projectNUCLEUS) | Integration as subsystem, workload TOMLs, deployment testing, USB packaging | **ARCHITECTURE** — handoff created, phases 2-5 pending |
| L5 (foundation) | Thread 04 (enviro genomics) + Thread 02 (plasma physics) data anchoring for LTEE datasets | **PENDING** — awaiting spring reproductions |

### Wave 6 Paper-Spring Assignments

| Spring | Papers | Count |
|--------|--------|------:|
| wetSpring | B1–B8, E1, E5 | 10 |
| neuralSpring | B1–B4, B6–B9, E2–E5 | 12 |
| groundSpring | B1–B4, B6–B9 | 8 |
| hotSpring | B2, B9 | 2 |
| healthSpring | B5, E2, E4 | 3 |
| airSpring | E3 | 1 |
| **Total** | | **36** |

### Wave 6 Milestones

- [x] Phase 1: Architecture + queue seeding (THIS UPDATE)
- [ ] Phase 2: Spring reproductions (L3) — **INTERSTADIAL**
- [ ] Phase 3: Binary bundle + data assembly (L2 + L4) — **INTERSTADIAL**
- [ ] Phase 4: Integration + deployment testing (L4) — **STADIAL**
- [ ] Phase 5: External deployment to Barrick Lab (L4) — **STADIAL**

---

## Interstadial Exit Criteria (May 11, 2026)

The interstadial ends when sovereignty capabilities are structurally wired and
shadow runs can begin. Five pillars define the exit gate. Full details:
`infra/wateringHole/INTERSTADIAL_EXIT_CRITERIA.md`

### Interstadial Targets by Layer

| Layer | Interstadial Target | Gate Condition |
|-------|-------------------|----------------|
| **L1 (Primals)** | MethodGate parity 13/13 | MethodGate shipped for all primals |
| **L2 (primalSpring)** | CompositionContext coordination pass, lithoSpore standard | 2+ lithoSpore modules PASS Tier 1 |
| **L3 (Springs)** | 4+ springs `optional=true`, gS convergence (air/neural → L4), LTEE reproductions begin | wetSpring < 5 PG gaps, 2+ foundation threads seeded |
| **L4 (Products)** | H2 shadow runs (TLS/NAT/NestGate/BTSP auth), ABG WCM compositions | H2-2b/3a/3b/3c in shadow-run state |
| **L5 (Foundation)** | Threads 3, 5, 8, 10 sources/targets, LTEE data anchoring | 7+/10 threads with sources |

### Stadial Targets by Layer

| Layer | Stadial Target | External Driver |
|-------|---------------|-----------------|
| **L1 (Primals)** | Upstream crate extraction (wgsl-precision, proc-sysinfo) | crates.io community |
| **L2 (primalSpring)** | Framework parity benchmarks | Kokkos, LAMMPS, SciPy |
| **L3 (Springs)** | lithoSpore Phases 4-5, all springs Tier 4 | Barrick Lab USB, peer validation |
| **L4 (Products)** | H2 cutover (Cloudflare → sovereign), H3 begin | Cloudflare baselines, GitHub → Forgejo |
| **L5 (Foundation)** | All threads with validated targets, ABG in production | ABG users, faculty network |
