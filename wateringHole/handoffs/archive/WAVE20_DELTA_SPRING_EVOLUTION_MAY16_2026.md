# Wave 20 Delta Spring Evolution — Schema Standardization + E2E Validation

**Date**: May 16, 2026 (Wave 20)
**Source**: primalSpring Ecosystem Status Assessment
**Audience**: All 7 delta springs (hotSpring, healthSpring, wetSpring, neuralSpring, ludoSpring, groundSpring, airSpring)
**Registry**: 452 methods (up from 451 — `primal.list` added)

---

## What Changed in primalSpring Wave 20

1. **`primal.list` canonical schema** — new method in `capability_registry.toml`. biomeOS will serve `{ "primals": [...], "count": N }` where each entry has `name` + `socket` (required) and `pid`, `capabilities`, `status`, `version` (optional).

2. **`capability.list` response standardization** — canonical shape is now:
   ```json
   { "capabilities": ["security", "content", ...], "count": 42, "primal": "biomeos" }
   ```
   `capabilities` MUST be an array of strings. `count` MUST match array length. Extra fields are allowed but the canonical subset must be present.

3. **`nest.commit` E2E validation** — new scenario (`s_nest_commit_live`) validates the full signal pipeline: `event.append` → `crypto.sign` → `content.put` → `session.commit` → `braid.create`. Skip-tolerant for pre-v3.57 biomeOS.

4. **Thread 10 provenance wiring** — `primalspring validate --provenance-dir <path>` writes `results.json` + `provenance.toml` to projectFOUNDATION's validation folder convention.

5. **Schema validation scenario** — `s_schema_standard` probes registry presence, local response shape, live biomeOS `capability.list` and `primal.list` schemas.

6. **Primal-blocked asks documented** — toadStool sandbox, barraCuda/coralReef, ionic bridge, sweetGrass TCP, biomeOS schema gaps are formally asked in `PRIMAL_BLOCKED_ASKS_MAY16_2026.md`.

---

## What Each Spring Should Absorb

### All Springs (common checklist)

- [ ] **Registry sync**: Verify your local capability cross-sync tests target **452** (was 451). The new method is `primal.list` under `[primal]`.
- [ ] **`capability.list` canonical envelope**: Ensure your `capability.list` / `capabilities.list` handler returns at minimum `{ "capabilities": [...], "count": N }`. Extra fields (science splits, domains, methods, etc.) are fine — the canonical subset must be present.
- [ ] **`count` field**: Add `"count": caps.len()` to your `capability.list` response if not already present. Every downstream consumer (projectNUCLEUS, projectFOUNDATION) expects this.
- [ ] **Thread 10 provenance**: If your `validate` binary supports `--format json`, consider adding `--provenance-dir` to write provenance artifacts. Foundation workloads will call your binary with this flag.

### Per-Spring Guidance

---

#### hotSpring

**Current**: `capability.list` returns `{ "capabilities": state.capabilities }` — array shape correct, missing `count`. `primal.announce` adopted as `try_primal_announce` + legacy. `nest.commit`/`nest.store` documented as candidates but not adopted.

**Wave 20 absorb**:
- [ ] Add `"count": state.capabilities.len()` to `capability.list` response
- [ ] Add `primal.list` to registry sync target (452)
- [ ] Consider `nest.commit` signal dispatch if Titan V pipeline benefits from ledger commit provenance
- [ ] Add schema-standard-style drift check against primalSpring canonical response shapes

**Priority**: Low — mostly aligned. `count` is the only gap.

---

#### healthSpring

**Current**: `capability.list` returns rich object with `methods`, `total`, `science`/`infrastructure` splits — diverges from canonical but is the richest response. `primal.announce` adopted with fallback. NestComposition wired for signal dispatch but `nest.commit` E2E is manual multi-call, not signal-path.

**Wave 20 absorb**:
- [ ] Add canonical subset fields: ensure `"capabilities"` (string array) and `"count"` are present alongside enriched fields
- [ ] Evaluate migrating manual nest-atomic chain to signal-path `ctx.dispatch("nest.commit", ...)` — primalSpring's `s_nest_commit_live` can serve as template
- [ ] Registry sync: 452
- [ ] Consider `--provenance-dir` for Thread 10 workload (PK/PD validation is a strong Thread 10 candidate)

**Priority**: Medium — `capability.list` needs canonical envelope addition; nest.commit signal-path would strengthen E2E coverage.

---

#### wetSpring

**Current**: `capability.list` returns `methods`, `provided_capabilities`, `consumed_capabilities`, `capabilities` — has the `capabilities` array but no `count`. Registry `[signals]` has `pending = ["primal.announce"]` despite `primal.announce` being consumed — stale. Both `nest.commit` and `nest.store` wired in provenance code.

**Wave 20 absorb**:
- [ ] Add `"count"` to `capability.list` response
- [ ] Fix registry: move `primal.announce` from `[signals].pending` to consumed/active
- [ ] Registry sync: 452
- [ ] B7 Tier 3 provenance would benefit from `--provenance-dir` pattern when sweetGrass TCP gap resolves

**Priority**: Low — mostly aligned. Registry stale flag is the main cleanup.

---

#### neuralSpring

**Current**: `capability.list` returns `{ "primal", "capabilities": ALL_CAPABILITIES }` — array shape correct, has `primal`, missing `count`. `nest.store` in weight_loader and `s_signal_dispatch`; `nest.commit` documented as candidate. `primal.announce` adopted.

**Wave 20 absorb**:
- [ ] Add `"count": ALL_CAPABILITIES.len()` to `capability.list` response
- [ ] Registry sync: 452
- [ ] If weight persistence pipeline does session commits, wire `nest.commit` signal dispatch
- [ ] Consider schema-standard scenario for ML inference endpoint shape validation

**Priority**: Low — `count` addition only.

---

#### ludoSpring

**Current**: `capability.list` returns `domains` tree — **most divergent** from canonical shape. No top-level `capabilities` string array. `primal.announce` as dispatch alias. Signal dispatch wiring unclear — constants + methods listed but no `ctx.dispatch` evidence found.

**Wave 20 absorb**:
- [ ] **Critical**: Add canonical `"capabilities"` (flat string array) and `"count"` to `capability.list` response. Can keep `domains` tree alongside for game-engine consumers
- [ ] Registry sync: 452
- [ ] Verify signal dispatch wiring (`nest.store`, game signals) is through `ctx.dispatch()` not manual multi-call
- [ ] 6 outstanding `game.*` methods for esotericWebb — document as handoff if not already

**Priority**: Medium — `capability.list` shape is the biggest gap across all delta springs.

---

#### groundSpring

**Current**: `capability.list` returns `{ "domain", "capabilities": [...] }` — array shape correct, missing `count` and `primal`. `nest.store` wired in provenance; `nest.commit` candidate/next wave. Uses **`announce_or_register`** explicitly (unique among deltas — canonical naming). Root `capability_registry.toml` is MCP `[[tools]]` format, orthogonal to neural registry.

**Wave 20 absorb**:
- [ ] Add `"count"` and `"primal"` to `capability.list` response
- [ ] Consider `nest.commit` for LTEE provenance chain (B1-B4 could use ledger commits)
- [ ] Document MCP registry ↔ neural registry relationship (both valid, different surfaces)
- [ ] Registry sync: 452

**Priority**: Low — well aligned. groundSpring's explicit `announce_or_register` naming is already canonical.

---

#### airSpring

**Current**: `capability.list` returns `niche`/`version` + `science`/`infrastructure` arrays + `total` + `composition` — rich but no top-level `capabilities` string array. Both `nest.commit` and `nest.store` wired in provenance code with fallbacks. `primal.announce` adopted.

**Wave 20 absorb**:
- [ ] Add canonical `"capabilities"` (flat string array) and `"count"` to `capability.list` — can keep `science`/`infrastructure` splits alongside
- [ ] Registry sync: 452
- [ ] E3 LTEE queued — when started, wire `--provenance-dir` for Thread 5+6 capture
- [ ] AG-006 through AG-012 are primal-blocked (documented in asks handoff)

**Priority**: Medium — `capability.list` needs canonical envelope.

---

## Priority Matrix

| Spring | `capability.list` Gap | `count` Missing | `primal.list` Sync | `nest.commit` | Provenance Dir | Overall |
|--------|:--------------------:|:--------------:|:------------------:|:-------------:|:--------------:|:-------:|
| hotSpring | Low (array OK) | YES | YES | Candidate | Optional | **Low** |
| healthSpring | Medium (rich, no canonical subset) | YES | YES | Manual → signal | Useful | **Medium** |
| wetSpring | Low (has array) | YES | YES | Wired | Useful | **Low** |
| neuralSpring | Low (array OK) | YES | YES | Candidate | Optional | **Low** |
| ludoSpring | **High** (domains tree, no array) | YES | YES | Unclear | Optional | **Medium** |
| groundSpring | Low (array OK) | YES | YES | Candidate | Useful | **Low** |
| airSpring | Medium (splits, no flat array) | YES | YES | Wired | Useful | **Medium** |

## Cross-Cutting Observations

**Every spring is missing `count`** in their `capability.list` response. This is the single highest-leverage change: one line per spring, massive downstream impact.

**`primal.list`** is a biomeOS method — springs don't serve it, but they should sync their registry cross-tests against 452 to catch drift.

**`nest.commit` signal dispatch** varies widely: air/wet have it wired, health has manual multi-call, hot/neural/ground/ludo haven't adopted it. Springs with active provenance chains (health, ground LTEE, air E3) would benefit most from the signal-path pattern.

**Schema validation scenarios** don't exist in any delta spring. A `s_schema_standard`-style scenario is optional but strengthens the ecosystem's ability to catch biomeOS schema changes. Springs with CI sync tests (healthSpring, wetSpring, airSpring) are best positioned to add this.

---

## How To Use This

1. Start with the **common checklist** (all springs)
2. Apply your **per-spring guidance** items
3. Push to `wateringHole/` with a brief evolution note
4. primalSpring will pull and review on next wave

The ecosystem is stable and converging. These are refinements, not breaking changes. Take them at your pace — the stadial gate is methodical.
