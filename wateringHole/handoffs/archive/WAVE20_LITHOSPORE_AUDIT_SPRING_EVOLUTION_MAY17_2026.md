<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

# Wave 20 PM — lithoSpore Audit Absorption & Ecosystem Evolution

**Date**: May 17, 2026 (PM)
**Source**: primalSpring — lithoSpore downstream audit + infra/wateringHole cleanup
**Audience**: All 7 delta spring teams
**Registry**: 456 methods, stability tiers now annotated
**Status**: All springs at zero Wave 20 debt — this blurb is forward-looking evolution

---

## What Changed in primalSpring

lithoSpore pushed a full evolution pass — Tier 3 provenance trio wired via
JSON-RPC, cross-tier parity (`litho parity` proves Python and Rust agree on
all 7 modules), and a two-tier data model formalized with upstream braid
handoff. They filed 8 requests (R1–R8). primalSpring resolved R1–R4 locally:

1. **Degradation behavior documented** — `CompositionContext` now has a
   per-capability degradation table (what happens when dag/spine/braid/
   visualization/discovery/crypto/compute are unreachable)
2. **Method stability tiers** — `capability_registry.toml` now has
   `stability = "stable"` on 12 deployed-consumer domains. Renaming a
   stable method requires versioning and downstream notification.
3. **Trio transaction semantics** — `PROVENANCE_TRIO_INTEGRATION_GUIDE.md`
   in infra/wateringHole now documents partial completion states, consumer
   rules, and lithoSpore's reference implementation
4. **UDS socket ownership** — `CAPABILITY_BASED_DISCOVERY_STANDARD.md`
   now documents who binds each socket (primal sockets, songBird
   discovery.sock, biomeOS biomeos.sock), stale detection, crash recovery

---

## What Every Spring Should Do

### 1. Absorb Stability Tier Awareness

`config/capability_registry.toml` now annotates domains with `stability`:
- `stable` — deployed consumers depend on these names; do not rename
- `evolving` — name may change between major waves; notify downstream
- `internal` — test fixture or internal-only; no stability guarantee

**Action**: If your spring's niche methods use capability strings from the
registry, check that you're using the canonical stable names. If you've
invented local aliases, align them or document them as wire-name aliases
(the sweetGrass pattern from GAP-36).

### 2. Document Your Degradation Behavior

lithoSpore's R1 was: "when your primal is unreachable, what does the
consumer see?" primalSpring documented the consumer side in
`CompositionContext`. Each spring should document its own:

- What happens when you call a primal that's down?
- Do you return `Err`, `None`, `Skip`, or panic?
- Is your domain logic gated behind primal availability?

**Pattern**: `has_capability()` before `call()`. Never gate science behind
provenance — provenance is enrichment.

### 3. Absorb Cross-Tier Parity Pattern

If your spring has both Python notebooks AND Rust validation for the same
science, you should implement parity checking:

1. Run the Python baseline → capture expected values as JSON
2. Run the Rust implementation → capture computed values as JSON
3. Compare within documented tolerances
4. Report structured parity results

lithoSpore's `litho parity` proves all 7 modules match across tiers.
This is the three-layer proof: Tier 1 confirms science (Python), Tier 2
confirms implementation (Rust), Tier 3 confirms provenance (trio chain).

**See**: `primalSpring/docs/VALIDATION_TIERS.md` — new Tier 3 and parity sections.

### 4. Review Trio Transaction Semantics

The provenance trio commit flow is **not atomic**. The new documentation
in `PROVENANCE_TRIO_INTEGRATION_GUIDE.md` defines partial completion:

- DAG without braid = valid partial provenance
- Braid without spine = valid attribution without permanence
- There is no rollback — DAG sessions are append-only
- Partial state must be reported (e.g., `primals_reached` list)
- Never error on partial provenance — domain logic must not fail

If your spring records provenance via the trio, review these rules.

---

## Per-Spring Guidance

### wetSpring — HIGHEST PRIORITY

wetSpring is the ecosystem's critical path for the first end-to-end study.
You're already on southGate with ~3TB NVMe processing the Tenaillon 2016
dataset (264 genomes, ~200 GB).

**Ferment transcript pattern**: lithoSpore needs braids from you. The full
contract is in infra/wateringHole:

`handoffs/LITHOSPORE_FERMENT_TRANSCRIPT_BRAID_HANDOFF_MAY17_2026.md`

What you MUST produce:
1. Summary statistics matching published claims (mutation rates, spectrum)
2. A sweetGrass braid covering the full computation (breseq pipeline)
3. A rhizoCrypt DAG session with events per pipeline step
4. A loamSpine entry recording final result hash + timestamp

Wire format: JSON to `provenance/braids/{dataset_id}.json` in lithoSpore:
```json
{
  "dataset_id": "tenaillon_2016_genomes",
  "spring": "wetSpring",
  "braid_id": "braid-...",
  "dag_session_id": "dag-...",
  "dag_merkle_root": "...",
  "spine_id": "spine-...",
  "computation": {
    "tool": "breseq", "tool_version": "0.38.1",
    "input_accession": "PRJNA294072",
    "input_blake3": "...", "output_blake3": "...",
    "wall_time_seconds": 86400, "node_count": 264
  }
}
```

Priority datasets:
| Dataset | Computation | Data Size | Impact |
|---------|-------------|-----------|--------|
| Tenaillon 2016 | breseq on 264 genomes | ~200 GB | HIGHEST — Barrick/Lenski demo |
| Barrick 2009 | breseq on 19 genomes | ~15 GB | HIGH — mutation accumulation |

### hotSpring

No urgent debt. Your Wave 20 adoption was clean (3 new scenarios, metalForge
graphs, fossilized RPC fixed). Focus areas if evolving:

- Cross-tier parity for Anderson module (Python vs Rust)
- Absorb stability tier awareness in your niche methods
- Document degradation behavior for your IPC interactions

### healthSpring

Outstanding evolution with 39 new scenarios (18→57). Focus areas:

- Cross-tier parity for B5 Leonard PK/PD (Python vs Rust)
- B5 is the next lithoSpore module candidate (Module 8) — when ready,
  coordinate with lithoSpore on `expected_values.json` format
- Absorb stability tier awareness

### neuralSpring

Clean Wave 20 adoption (s_gpu_parity, typed toadStool IPC). Focus areas:

- B3/B4 ML surrogates for lithoSpore modules 3+4 (additive enrichment)
- GPU parity as a form of cross-tier parity (CPU vs GPU numerical agreement)
- Document degradation behavior for GPU-dependent paths

### ludoSpring

Strong Schell 20-lens implementation. Focus areas:

- Cross-tier parity for metalForge (Python baselines vs Rust)
- Absorb stability tier awareness in game.* methods
- `validate_tower_atomic` now feature-gated (confirmed working)

### groundSpring

All 5 LTEE papers complete (B1-B4, B6). Focus areas:

- Cross-tier parity for all 5 papers (comprehensive — you have both Python
  and Rust for everything)
- Wiser 2013 ferment braid (LOW priority — small data, already in lithoSpore)
- Absorb stability tier awareness

### airSpring

Strong 57-capability niche. Focus areas:

- Thread 4 expression seeding (coordinate with wetSpring)
- Cross-tier parity for foundation targets (Python notebooks vs Rust scenarios)
- Absorb stability tier awareness in ecology alias methods

---

## Ecosystem Posture

The ecosystem is stabilizing. The glacial checkpoint shows:

- **All 8 springs** at Wave 20, zero debt
- **13/13 primals** at zero debt, all BTSP AEAD
- **lithoSpore** deployed — first real consumer, 75/75 science checks
- **wetSpring on southGate** — first end-to-end study in progress
- **45 handoffs archived** — 48h rule enforced, fossil record clean

The pace is now methodical. The main evolution vectors are:

1. **wetSpring ferment transcripts** — the ecosystem's first provenance chain
   from raw data → computation → artifact → deployment
2. **Cross-tier parity adoption** — proving math stability across implementations
3. **Stability tier enforcement** — method names are now frozen for consumers
4. **Thread 4 seeding** — the last remaining foundation thread gap

After springs absorb this, upstream primal evolution (biomeOS `nest.store`
signal dispatch, `spore.instantiate`) is the next phase.

---

## Related Documents

- `infra/wateringHole/PROVENANCE_TRIO_INTEGRATION_GUIDE.md` — trio transaction semantics
- `infra/wateringHole/CAPABILITY_BASED_DISCOVERY_STANDARD.md` — UDS socket ownership
- `infra/wateringHole/handoffs/LITHOSPORE_FERMENT_TRANSCRIPT_BRAID_HANDOFF_MAY17_2026.md` — braid contract
- `infra/wateringHole/handoffs/LITHOSPORE_PRIMAL_SPRING_EVOLUTION_HANDOFF_MAY17_2026.md` — full lithoSpore audit
- `primalSpring/docs/VALIDATION_TIERS.md` — Tier 3 + parity pattern
- `primalSpring/config/capability_registry.toml` — stability tier annotations
