<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 21: Garden Product Evolution — Primal Composition Patterns

**Date:** May 17, 2026 PM
**From:** primalSpring (coordination)
**To:** projectNUCLEUS team, projectFOUNDATION team, lithoSpore team
**Context:** All 8 springs have absorbed lithoSpore's downstream audit and are
at zero debt. wetSpring's breseq pipeline is producing the ecosystem's first
real-data ferment transcript braids. The spring delta is stable. Gardens should
now absorb Wave 20 patterns and evolve toward primal composition.

---

## Why This Matters Across All Three Teams

Your three products form a stack:

```
lithoSpore (verification chassis)
  │  carries science + proves it's correct
  │  USB-deployable, airgapped, grows in any environment
  │
  ├── projectFOUNDATION (knowledge layer)
  │     DAGs and braids from upstream springs make science
  │     replicable, auditable, and validatable
  │     10 threads × provenance folders × BLAKE3 anchoring
  │
  └── projectNUCLEUS (sovereignty layer)
        cellMembrane is the external substrate
        infrastructure + code sovereignty for deployment
        gate TOMLs route science to hardware
```

lithoSpore is conceptually a slice of both: it validates projectFOUNDATION's
knowledge (7 LTEE modules from 4 springs) and deploys through projectNUCLEUS's
infrastructure (cellMembrane for geo-delocalized Tier 2 via Songbird TURN).
Cross-knowledge between your teams accelerates all three.

---

## What Changed Upstream (Waves 19-20)

The spring delta evolved significantly since your last absorption (~Wave 18).
Here's what you missed and what's now available:

### 1. Canonical Schemas — SHIPPED

primalSpring Wave 20 defined and validated `primal.list` and `capability.list`
canonical responses:

```json
{ "primals": [...], "count": N }
{ "capabilities": [...], "count": N, "primal": "springName" }
```

**projectNUCLEUS**: Your P0 ask for these schemas is resolved. Your discovery
cascade documentation should reference the canonical shapes. Update
`EVOLUTION_GAPS.md` to mark these as shipped.

**lithoSpore**: Your `query_capabilities()` in `litho-core::discovery` should
expect the canonical envelope. If you're parsing raw arrays, the `count` and
`primal` fields are now guaranteed.

**projectFOUNDATION**: Your `COMPOSITION_GAPS.md` references these as gaps —
mark as resolved.

### 2. Method Stability Tiers

`capability_registry.toml` now annotates every method group:

| Tier | Meaning | Consumer Guidance |
|------|---------|-------------------|
| **stable** | Wire name frozen | Safe to hardcode in dispatch maps, gate TOMLs, scope manifests |
| **evolving** | May change with deprecation cycle | Document dependency; watch wateringHole for migration guides |
| **internal** | Implementation detail | Do not depend externally |

All 8 springs have annotated their niche capabilities. The pattern:

```toml
[health]
stability = "stable"
methods = ["health.version", "health.liveness", "health.metrics"]
```

**All three teams**: Annotate your consumed methods with stability awareness.
If your dispatch maps, gate TOMLs, scope manifests, or workload definitions
reference a method, know its tier.

### 3. Degradation Behavior Standard

All 8 springs now document per-primal degradation in `docs/DEGRADATION_BEHAVIOR.md`.
The ecosystem invariant:

> **Science is never gated behind primal availability.**
> All RPC calls return `Result`. No method panics on unreachable primals.

**lithoSpore**: You pioneered this pattern — your `try_record_tier3()` is the
reference implementation. Springs now document the same behavior.

**projectNUCLEUS / cellMembrane**: Document what happens when membrane primals
are unreachable. If Songbird TURN relay is down, does lithoSpore degrade to
standalone? If BearDog auth is unreachable, does the VPS gate still serve?
Write it down per service.

**projectFOUNDATION**: Your `foundation_validate.sh` workload runners should
document degradation. Thread 1 WCM is upstream-blocked on RPC — that's degraded,
not broken. Partial provenance (DAG without braid) is valid.

### 4. Cross-Tier Parity Pattern

lithoSpore's `litho parity` proved it: run Python Tier 1 and Rust Tier 2
side-by-side, compare numerically, prove mathematical stability. airSpring then
added 3 new cross-tier validators. The ecosystem now has a formal three-layer
proof structure:

```
Tier 1: Python notebook/script → expected_values.json
Tier 2: Rust validator binary → compare against expected_values.json
Tier 3: Primal composition (trio provenance) → verify computation chain
```

**lithoSpore**: You own this pattern. 7/7 modules MATCH. The `ParityReport`
JSON output format should be documented as the ecosystem standard for other
products to adopt.

**projectFOUNDATION**: Your 6 barraCuda CPU parity benchmarks are natural
Tier 1→2 proofs. Formalize them. Reference `primalSpring/docs/VALIDATION_TIERS.md`
§ Cross-Tier Parity Pattern.

**projectNUCLEUS**: Your `TIER2_CEREMONY_DESIGN.md` should reference the parity
pattern for BearDog RPC sequence validation.

### 5. Trio Transaction Semantics

`infra/wateringHole/PROVENANCE_TRIO_INTEGRATION_GUIDE.md` now documents partial
completion states:

| State | DAG | Spine | Braid | Valid? |
|-------|:---:|:-----:|:-----:|:------:|
| Full | YES | YES | YES | YES — complete provenance chain |
| Partial (DAG+spine) | YES | YES | no | YES — ledger entry, no attribution |
| Partial (DAG only) | YES | no | no | YES — session recorded, unbacked |
| None | no | no | no | YES — standalone mode |

**Rule**: No rollback on partial. Consumer decides whether partial is acceptable.

**lithoSpore**: Your Tier 3 wiring already handles this — `primals_reached`
tracking (which you learned from airSpring/wetSpring) fits naturally.

**projectFOUNDATION**: When recording provenance for thread validation runs,
accept partial. A DAG session ID without a braid is better than no provenance.
Your `validation/<spring>/<date>/` folders should include whatever trio data
is available.

**projectNUCLEUS**: cellMembrane deployments may run in environments where only
some trio primals are reachable. Gate configurations should handle partial
provenance gracefully.

### 6. Ferment Transcript Pattern — The Big One

wetSpring V177 is executing the ecosystem's first real-data composition:

```
NCBI (SRP001569, Barrick 2009)
  → wetSpring breseq pipeline (breseq 0.40.1 on 4TB NVMe at southGate)
  → provenance trio (rhizoCrypt DAG → loamSpine spine → sweetGrass braid)
  → ferment transcript braid (JSON wire format)
  → lithoSpore data.toml (upstream_braid, upstream_dag_session, upstream_spring)
  → USB artifact (validates science + verifiable computation chain)
```

**Status**: 3/7 Barrick 2009 clones done (REL1164M: 579, REL2179M: 608,
REL8593M: 1108 mutations). Accumulation trend confirmed. First ferment braid
exported to `provenance/braids/barrick_2009_mutations.json`.

Wire format:

```json
{
  "dataset_id": "barrick_2009_mutations",
  "spring": "wetSpring",
  "spring_version": "0.1.0",
  "braid_id": "<from sweetGrass — empty in standalone>",
  "dag_session_id": "<from rhizoCrypt>",
  "dag_merkle_root": "<BLAKE3>",
  "spine_id": "<from loamSpine>",
  "computation": {
    "tool": "breseq",
    "tool_version": "0.40.1",
    "input_accession": "SRP001569",
    "node_count": 7,
    "wall_time_seconds": 3793
  },
  "summary_blake3": "529e34ee..."
}
```

**lithoSpore**: You defined this contract. wetSpring is fulfilling it. When
Barrick 2009 completes (4 clones remaining), the braid flows into your
`data.toml` as `upstream_braid`. Airgapped it's documentation, online it's a
verifiable chain. Tenaillon 2016 (~200 GB, 264 genomes) is next.

**projectFOUNDATION**: Thread 5 (LTEE) should prepare to receive braid evidence
from wetSpring. Your `validation/wetSpring/` provenance folders should reference
braid IDs. This is the first thread with verifiable upstream computation
provenance — the science goes from "trust the published numbers" to "verify
the computation chain."

**projectNUCLEUS**: The ferment transcript is a real production dispatch pattern.
Your `SCIENCE_DISPATCH_MAP.md` should document the route: spring computation →
trio provenance → braid export → lithoSpore ingestion → USB deployment. When
cellMembrane hosts a Tier 2 relay, the braid verification chain extends through
your infrastructure.

---

## Per-Team Specific Guidance

### lithoSpore

You're the furthest along — your patterns drove the spring evolution. Focus on:

1. **Absorb wetSpring Exp381 braids** when Barrick 2009 completes (4 clones
   remaining). This upgrades Module 2 (ltee-mutations) from summary-only to
   computation-verified.
2. **Document `ParityReport` as ecosystem standard** — springs are adopting your
   cross-tier parity pattern. Publish the JSON schema so projectFOUNDATION and
   projectNUCLEUS can consume parity results in their validation pipelines.
3. **Coordinate with cellMembrane** on geo-delocalized Tier 2 — your three
   operating modes (standalone/LAN/geo-delocalized) map to cellMembrane's
   escalation phases. Phase 1 Tower composition is next for cellMembrane.
4. **Update lithoSpore refs** in README: primalSpring registry is at 456 methods
   (Wave 20), biomeOS is at v3.57, sweetGrass at v0.7.35.

### projectFOUNDATION

You're the knowledge layer. Springs are producing real provenance. Focus on:

1. **Absorb Wave 20 canonical schemas** — mark `primal.list` / `capability.list`
   gaps as RESOLVED in `COMPOSITION_GAPS.md`.
2. **Prepare Thread 5 for braid evidence** — wetSpring's ferment transcript
   braids will be the first machine-verifiable provenance flowing into your
   thread validation. Your `validation/wetSpring/` folder convention is ready;
   add a `braids/` subfolder for ferment transcripts.
3. **BLAKE3 backfill** — many `data/sources/*.toml` files have empty `blake3 = ""`
   fields. lithoSpore demonstrates BLAKE3 anchoring for all data artifacts.
   Backfill these hashes using `litho fetch` as reference.
4. **Add stability tier awareness** to workload TOMLs and thread documentation.
   If a workload depends on an `evolving` method, document the dependency.
5. **Formalize barraCuda benchmarks as parity proofs** — your 6 CPU parity
   benchmarks are natural Tier 1→2 proofs. Use lithoSpore's `ParityReport`
   format.

### projectNUCLEUS + cellMembrane

You're the sovereignty and infrastructure layer. Focus on:

1. **Mark resolved P0 asks** — `primal.list` and `capability.list` canonical
   schemas are shipped (Wave 20). Update `EVOLUTION_GAPS.md`.
2. **Update lithoSpore integration refs** — your README references 6/7 modules
   PASS. It's now 7/7 (75/75 checks), with Tier 3 wired and cross-tier parity.
3. **Add stability tier awareness** to gate TOMLs' `[science]` dispatch metadata
   and `SCIENCE_DISPATCH_MAP.md`.
4. **cellMembrane degradation behavior** — document what happens when relay
   primals are unreachable. Phase 0.5 (relay + RustDesk + multi-gate SSH) to
   Phase 1 (Tower composition) transition should define degradation at each step.
5. **Ferment transcript dispatch routing** — document the wetSpring → trio →
   braid → lithoSpore route in `SCIENCE_DISPATCH_MAP.md`. This is a production
   dispatch pattern that flows through your infrastructure.
6. **cellMembrane sovereign DNS** — knot-dns items (H2-17 through H2-20) are
   still open. These enable fully sovereign routing for geo-delocalized lithoSpore
   instances.

---

## Cross-Team Coordination Points

| Topic | Teams | What |
|-------|-------|------|
| Ferment braid ingestion | lithoSpore ↔ wetSpring | Barrick 2009 completion → Module 2 upgrade |
| Geo-delocalized Tier 2 | lithoSpore ↔ cellMembrane | Songbird TURN relay → remote validation |
| Thread 5 braid evidence | projectFOUNDATION ↔ wetSpring | First computation-verified thread provenance |
| ParityReport standard | lithoSpore → all | JSON schema for cross-tier parity results |
| Membrane degradation | cellMembrane → lithoSpore | Phase 0.5→1 fallback behavior |
| Science dispatch routing | projectNUCLEUS → lithoSpore | Gate TOML routing for science validation |
| BLAKE3 anchoring | lithoSpore → projectFOUNDATION | Data integrity pattern for source TOMLs |

---

## Ecosystem Posture

| Metric | Value |
|--------|-------|
| Registry methods | 456 (stable, zero drift) |
| Delta spring tests | 9,539+ across 8 springs |
| lithoSpore modules | 7/7 Tier 2 PASS (75/75 checks), 117 tests |
| lithoSpore parity | 7/7 modules MATCH (Python ↔ Rust) |
| lithoSpore Tier 3 | Wired (JSON-RPC trio), graceful degradation |
| projectFOUNDATION threads | 9/10 active (Thread 4 sole remaining) |
| projectFOUNDATION targets | 184 total, 146 validated (79.3%) |
| projectNUCLEUS tests | 55 Rust (darkforest 34, tunnelKeeper 21) |
| cellMembrane phase | 0.5 (relay + RustDesk + multi-gate SSH) |
| Ferment braids exported | 1 (Barrick 2009, standalone — trio pending full composition) |
| Upstream blockers | All SHIPPED (UB-1 through UB-4) |
| Next critical path | wetSpring Barrick completion → Tenaillon 2016 → end-to-end study |

**Evolution pace**: Stadial — methodical and deliberate. The spring delta is
stable. Your products should absorb patterns at your own pace. The ferment
transcript from wetSpring is the highest-leverage item: it proves the entire
ecosystem pipeline works on real data — from NCBI accession to USB deployment.

When lithoSpore hands a USB to the Barrick Lab with wetSpring's computation
braids inside, the hypogeal cotyledon carries not just the science but the
proof that the computation was done correctly. That's the ecosystem working.
