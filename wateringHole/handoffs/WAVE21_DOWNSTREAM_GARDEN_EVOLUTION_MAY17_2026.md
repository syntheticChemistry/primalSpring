<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Wave 21: Downstream Garden Evolution — projectFOUNDATION + projectNUCLEUS

**Date:** May 17, 2026 PM
**From:** primalSpring (coordination)
**To:** projectFOUNDATION team, projectNUCLEUS team (incl. cellMembrane)
**Context:** All 8 springs have absorbed lithoSpore audit patterns. wetSpring is executing the first real-data ferment transcript braid. The spring delta is stable. It's time for gardens to absorb Wave 20 patterns.

---

## What Changed Since Your Last Absorption (Wave 18)

primalSpring evolved through Waves 19-20 and absorbed a full downstream audit
from lithoSpore. The delta springs then self-evolved to match. Here's what you
missed:

### 1. Canonical Schemas — SHIPPED (Wave 20)

**projectNUCLEUS**: Your P0 ask for `primal.list` and `capability.list` canonical
schemas has been **resolved**. primalSpring Wave 20 defined and validated both:

```json
// primal.list canonical response
{ "primals": [...], "count": N }

// capability.list canonical response
{ "capabilities": [...], "count": N, "primal": "springName" }
```

Validated in `ecoPrimal/src/validation/scenarios/s_schema_standard.rs`. All 8
springs emit the canonical envelope. Your discovery cascade
(`primal.list` → env → defaults → `health.liveness` + `capability.list`) should
now expect these exact shapes.

### 2. Method Stability Tiers (Wave 20 PM)

`capability_registry.toml` now annotates every method group with
`stability = "stable" | "evolving" | "internal"`:

- **stable**: Wire names frozen. Downstream consumers (lithoSpore, springs, gardens)
  may depend on these names indefinitely.
- **evolving**: Wire name may change with a deprecation cycle.
- **internal**: Implementation detail — do not depend externally.

**projectFOUNDATION**: Annotate your workload TOMLs with the stability tier of
the methods they exercise. If a workload depends on an `evolving` method,
document the dependency so you're aware of wire-name changes.

**projectNUCLEUS**: Annotate your gate TOMLs' `[science]` dispatch metadata with
stability tier awareness. Your `SCIENCE_DISPATCH_MAP.md` should reference tiers.
cellMembrane gate configurations should document stability tier of membrane
methods they exercise.

### 3. Degradation Behavior Standard (Wave 20 PM)

All 8 springs now have `docs/DEGRADATION_BEHAVIOR.md` documenting per-primal
degradation when primals are unreachable. The invariant:

> **Science is never gated behind primal availability.**

All RPC calls return `Result`. Callers decide whether to skip, retry, or abort.
`has_capability()` provides pre-call reachability checks.

**projectFOUNDATION**: Your `foundation_validate.sh` and workload runners should
document what happens when primals are unreachable. If Thread 1 WCM is
upstream-blocked on RPC, that's fine — document it as degraded, not broken.

**projectNUCLEUS / cellMembrane**: Your gate TOMLs should document fallback
behavior. If `toadStool` is unreachable, does `signal_executor.sh` degrade or
fail? cellMembrane's membrane deployment should document degradation when
upstream primals (Songbird TURN, BearDog auth) are unreachable.

### 4. Cross-Tier Parity Pattern (Wave 20 PM)

lithoSpore proved the pattern: run the same validation in Python (Tier 1) and
Rust (Tier 2), compare numerically, and prove mathematical stability. airSpring
added 3 new cross-tier validators. The pattern is:

```
Tier 1: Python notebook/script → expected_values.json
Tier 2: Rust validator binary → compare against expected_values.json
Tier 3: Primal composition (trio provenance) → verify computation chain
```

**projectFOUNDATION**: Your 6 barraCuda CPU parity benchmarks already follow this
pattern. Formalize them as Tier 1→2 parity proofs. Reference
`primalSpring/docs/VALIDATION_TIERS.md` § Cross-Tier Parity Pattern.

**projectNUCLEUS**: Your `TIER2_CEREMONY_DESIGN.md` should reference this pattern
for BearDog RPC sequence validation.

### 5. Ferment Transcript / Upstream Braid (Wave 20 PM)

This is the big one for projectFOUNDATION Thread 5:

wetSpring V177 is executing Exp381 — breseq pipeline on Barrick 2009
(SRP001569, 7 Ara-1 clones). 3/7 clones done. First ferment transcript braid
exported. The wire format:

```json
{
  "dataset_id": "barrick_2009_mutations",
  "spring": "wetSpring",
  "braid_id": "<from sweetGrass>",
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

This braid flows to lithoSpore's `data.toml` as `upstream_braid` —
airgapped it's documentation, online it's a verifiable chain.

**projectFOUNDATION**: Thread 5 (LTEE) should prepare to receive braid evidence
from wetSpring. Your `validation/wetSpring/` provenance folders should include
braid references. This is the first real end-to-end study: NCBI data →
computation → provenance → verification → USB deployment.

**projectNUCLEUS**: Your `SCIENCE_DISPATCH_MAP.md` should document the ferment
transcript route: wetSpring computation → trio provenance → braid export →
lithoSpore ingestion. This is a real production dispatch pattern.

### 6. Trio Transaction Semantics (Wave 20 PM)

`PROVENANCE_TRIO_INTEGRATION_GUIDE.md` now documents partial completion states:

| State | Meaning |
|-------|---------|
| DAG only | Session valid but unbacked by ledger |
| DAG + spine | Ledger entry but no attribution braid |
| DAG + spine + braid | Full provenance chain |
| Nothing | No trio attempted (standalone mode) |

**Rule**: DAG without braid is **valid** — it just means partial provenance.
No rollback. No retry requirement. Consumer decides whether partial is acceptable.

**projectFOUNDATION**: Apply this to your workload runners. If `rhizoCrypt` is
available but `sweetGrass` isn't, record the DAG session ID in your provenance
folder. Partial provenance is better than none.

---

## Per-Garden Specific Guidance

### projectFOUNDATION

Repo folder: `gardens/projectFOUNDATION` (sporeGarden org).

**Priority 1 — Wave 20 Pattern Absorption**

1. Add stability tier awareness to workload TOMLs and thread documentation
2. Document degradation behavior for `foundation_validate.sh` workload runners
3. Update `COMPOSITION_GAPS.md` to mark primal.list/capability.list schemas
   as RESOLVED (they reference this as a gap — it's shipped)
4. Add ferment transcript awareness to Thread 5 documentation
5. Reference `primalSpring/docs/VALIDATION_TIERS.md` for the cross-tier parity
   pattern — your 6 barraCuda benchmarks are natural candidates

**Priority 2 — Thread 5 Braid Evidence**

wetSpring is producing real braid evidence for Barrick 2009. When the pipeline
completes (4 remaining clones), the braid JSON should be referenced in your
Thread 5 data sources. This is the first thread with verifiable upstream
computation provenance.

**Priority 3 — BLAKE3 Backfill**

Many `data/sources/*.toml` files have empty `blake3 = ""` fields. Now that
lithoSpore demonstrates BLAKE3 anchoring for all data artifacts, backfill these
hashes. `primalSpring/tools/regenerate_checksums.sh` shows the pattern.

### projectNUCLEUS

**Priority 1 — Absorb Resolved Asks**

Your P0 asks are shipped:
- `primal.list` canonical schema → **DONE** (Wave 20, validated in s_schema_standard.rs)
- `capability.list` shape standardization → **DONE** (Wave 20, all 8 springs emit canonical)

Update `specs/EVOLUTION_GAPS.md` and relevant handoff docs to mark these resolved.
Your discovery cascade documentation should reference the canonical shapes.

**Priority 2 — Wave 20 PM Pattern Absorption**

1. Add stability tier references to `SCIENCE_DISPATCH_MAP.md` and gate TOMLs
2. Document gate fallback behavior in the style of spring `DEGRADATION_BEHAVIOR.md`
3. Add ferment transcript dispatch routing to `SCIENCE_DISPATCH_MAP.md`
4. Reference cross-tier parity pattern in `TIER2_CEREMONY_DESIGN.md`

**Priority 3 — lithoSpore Integration Update**

Your README references lithoSpore at **6/7 modules PASS Tier 2**. lithoSpore is
now at **7/7 modules Tier 2 PASS (75/75 checks)**, with Tier 3 provenance trio
wired via JSON-RPC, cross-tier parity (`litho parity`), and the two-tier data
model formalized. Update your lithoSpore references.

**Priority 4 — Signal Executor and Ferment Awareness**

Your `signal_executor.sh` (Squirrel → biomeOS signal.dispatch) should be aware
of the ferment transcript pattern for science dispatch routing. When wetSpring
completes a breseq pipeline and exports a braid, that's a signal-dispatchable
event that could trigger downstream validation.

**Priority 5 — cellMembrane Integration**

cellMembrane (fieldMouse Tower deployment on external substrate) is owned by
projectNUCLEUS and handles infrastructure sovereignty. cellMembrane should:
- Document degradation behavior when upstream primals are unreachable
  (Songbird TURN relay, BearDog auth, biomeOS orchestration)
- Reference stability tiers for membrane-exercised methods
- Align gate configurations with primalSpring's
  `config/routing_config_reference.toml` membrane schema

---

## Ecosystem Posture

| Metric | Value |
|--------|-------|
| Registry methods | 452 (stable, zero drift) |
| Delta spring tests | 9,539+ across 8 springs |
| lithoSpore modules | 7/7 Tier 2 PASS (75/75 checks) |
| projectFOUNDATION threads | 9/10 active (Thread 4 sole remaining) |
| Ferment braids | 1 exported (Barrick 2009, standalone — trio fields pending full composition) |
| Upstream blockers | All SHIPPED (UB-1 through UB-4) |
| Next critical path | wetSpring Barrick 2009 completion → Tenaillon 2016 → end-to-end study |

**Evolution pace**: Stadial — methodical and deliberate. The spring delta is
stable. Downstream gardens should absorb patterns at their own pace. The ferment
transcript pattern from wetSpring is the highest-leverage item for projectFOUNDATION
Thread 5 and lithoSpore end-to-end validation.
