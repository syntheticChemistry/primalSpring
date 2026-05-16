# Garden Products — Evolution Blurb (May 16, 2026)

**Audience**: projectFOUNDATION, projectNUCLEUS, esotericWebb IDE focus teams  
**Source**: primalSpring Wave 18 (local debt resolved, downstream interim prepared)  
**Context**: All gardens are local to ironGate. Delta springs are absorbing
Waves 16-17 evolution (playbook debt, Neural API signal elevation).
primalSpring has resolved its local deprecated-API debt and prepared
integration surfaces. This blurb captures what each garden should
absorb and evolve toward in the temporal interim.

---

## Cross-Cutting: What's Available from Springs (Wave 18)

| Surface | State | Entry Point |
|---------|-------|-------------|
| 451 registered capability methods | Stable | `config/capability_registry.toml` |
| 14 atomic signal graphs (4 tiers) | Stable | `graphs/signals/*.toml` |
| 41 validation scenarios (10 tracks) | Stable | `ecoPrimal/src/validation/scenarios/` |
| `CompositionContext` API | Modern — zero deprecated callers | `ctx.dispatch()`, `ctx.announce()`, `ctx.call()` |
| Foundation validation graph | 12-node NUCLEUS | `graphs/compositions/foundation_validation.toml` |
| 44-cell deployment matrix | lithoSpore cell PASS | `config/deployment_matrix.toml` |
| Signal Adoption Standard | Published | `wateringHole/SIGNAL_ADOPTION_STANDARD.md` |
| CATHEDRAL Split Guidance | Published | `wateringHole/handoffs/CATHEDRAL_SPLIT_SPRING_GUIDANCE_MAY16_2026.md` |

**NUCLEUS atomics available to all gardens**:

- **Tower** (3 primals: BearDog + Songbird + SkunkBat) — identity, trust, mesh
- **Node** (Tower + compute trio: toadStool + barraCuda + coralReef) — dispatch, tensor, shader
- **Nest** (Tower + NestGate + provenance trio: rhizoCrypt + loamSpine + sweetGrass) — storage, DAG, ledger, attribution
- **Full NUCLEUS** (13 primals) — all of the above + meta tier (petalTongue + Squirrel + biomeOS)

**Provenance stack**: Every garden can now stamp validation results
with full NUCLEUS provenance: `dag.session.create` → `dag.event.append`
→ `spine.seal` → `braid.create`. Or use the `nest.store` atomic signal
via `ctx.dispatch("nest.store", ...)` which collapses the 4-call sequence.

**ironGate locality advantage**: All three gardens share the ironGate
environment. Live NUCLEUS is already deployed and validated on ironGate.
Gardens can exercise full Tier 2 live IPC against the local NUCLEUS
without network dependencies.

---

## projectFOUNDATION — Knowledge Layer Evolution

### Posture: Layer with established guidestones, science, and data

projectFOUNDATION has a strong structural backbone (10 threads, source
TOMLs, expressions for 8/10 threads). The next evolution is **depth** —
filling the knowledge layer with validated science, provenance-stamped
evidence, and guidestone-certified results.

### What to Absorb

1. **Full provenance via NUCLEUS atomics**: Every validation result that
   flows into `projectFOUNDATION/validation/` should carry a NUCLEUS
   provenance chain. The foundation validation graph (`foundation_validation.toml`)
   defines the minimum composition: 12 nodes covering the full sediment
   pipeline. Run it locally on ironGate:

   ```bash
   cargo run -p primalspring --bin primalspring_unibin -- certify \
     --graph graphs/compositions/foundation_validation.toml
   ```

2. **Guidestone layering for thread evidence**: Each thread's validation
   results should carry guidestone-level certification:
   - **Tier 1** (structural): Graph parses, expected values present, BLAKE3 anchors
   - **Tier 2** (live): Primal IPC round-trips, method responses validated
   - **Tier 3** (provenance): DAG session wrapping validation runs, loamSpine lineage entries, sweetGrass attribution braids

3. **Signal dispatch for provenance storage**: Use `ctx.dispatch("nest.store", ...)`
   for the provenance leg. This replaces the 4-call manual sequence with a
   single atomic signal through biomeOS graph execution.

### Priority Evolution Items

| Priority | Item | Action |
|----------|------|--------|
| **P0** | BLAKE3 backfill | Fill empty `blake3 = ""` in source TOMLs by fetching and hashing actual data |
| **P0** | Thread 1 WCM RPC | Unblock 0/24 pending checks — needs RPC stack for WCM data retrieval |
| **P0** | Dated provenance folders | Each validation run → `projectFOUNDATION/validation/<spring>/YYYY-MM-DD/` with structured JSON |
| **P1** | Thread 4 expression | Environmental Genomics needs expression + targets (wetSpring + airSpring coordinate) |
| **P1** | ML sources for Thread 5 | neuralSpring ML surrogate validation data for B3/B4/B6 |
| **P2** | Thread 9-10 depth | ludoSpring/primalSpring have seeded Threads 9-10 — grow targets and expressions |

### Thread Status Quick Reference

| Thread | Status | Passing | Action |
|--------|--------|---------|--------|
| 1 WCM | **BLOCKED** | 0/24 | Needs RPC stack |
| 2 Plasma | **PASS** | 12/12 | Maintain, add provenance stamps |
| 3 Immunology | **ACTIVE** | Partial | healthSpring targets pending |
| 4 Env Genomics | **PARTIAL** | — | Needs expression + targets |
| 5 LTEE/Evolution | **ACTIVE** | 14/18 | 4 pending (wetSpring B7) |
| 6 Agricultural | **PASS** | 36/36 | Maintain, add provenance stamps |
| 7 Anderson | **PASS** | 18/18 | Maintain, add provenance stamps |
| 8 Human Health | **ACTIVE** | Partial | healthSpring expanding |
| 9 Gaming | **ACTIVE** | Seeded | ludoSpring T9 growing |
| 10 Provenance | **ACTIVE** | Seeded | primalSpring co-owns |

### Key Integration Pattern

```
spring runs validation
  → ValidationResult JSON exported
  → dag.session.create (rhizoCrypt)
  → dag.event.append (validation evidence)
  → spine.seal (loamSpine certifies)
  → braid.create (sweetGrass attributes)
  → results copied to projectFOUNDATION/validation/<spring>/YYYY-MM-DD/
```

On ironGate this entire pipeline runs locally against the deployed NUCLEUS.

---

## projectNUCLEUS — Sovereignty Deployment Evolution

### Posture: Continue structural and deployment sovereignty

projectNUCLEUS has cleared the interstadial exit gate. Forgejo is PRIMARY
(32 repos, 3 orgs, dual-push mirror). The sovereignty shadow layer is
operational. The next evolution is **hardening the sovereignty infrastructure**
and completing the remaining cutover items.

### What to Absorb

1. **Signal simplification for workloads**: Workload TOMLs can now reference
   atomic signals instead of method sequences. A workload specifying
   `nest.store` replaces the 4-call `content.put → dag.event.append →
   spine.seal → braid.create` pattern. See `wateringHole/SIGNAL_ADOPTION_STANDARD.md`.

2. **Membrane composition validation**: primalSpring validates the VPS
   membrane graph structurally via `s_membrane_composition` and live via
   `s_sovereignty_parity`. The calibrate-shadow-cutover protocol
   (`SOVEREIGNTY_STANDARDS.md`) provides the gate methodology.

3. **Modern composition API**: All primalSpring code now uses
   `validate_composition_ctx` and `CompositionContext` — no legacy fallbacks.
   NUCLEUS operational tooling should follow the same pattern.

### Priority Evolution Items

| Priority | Item | Action |
|----------|------|--------|
| **P0** | Forgejo Actions CI | Port CI pipelines from GitHub Actions to Forgejo Actions — sovereignty of CI itself |
| **P0** | BTSP JupyterHub cutover | Dual-auth shadow is active; complete the cutover for sovereign compute sessions |
| **P1** | Sovereign DNS | knot-dns deployment (H2-17 through H2-20) — removes Cloudflare dependency |
| **P1** | petalTongue extracellular wiring | Connect petalTongue to the VPS membrane for visualization through the sovereign channel |
| **P2** | Signal-based workloads | Migrate workload TOMLs from method sequences to atomic signal references |
| **P2** | Membrane auto-healing | Rolling `membrane_7day.toml` baselines should auto-detect and alert on sovereignty regression |

### Sovereignty Layer Status

| Layer | Status | Evidence |
|-------|--------|----------|
| 1. Primal Capabilities | **PASS** | 451 methods, 13/13 primals LIVE |
| 2. Security | **PASS** | BTSP 13/13 AEAD, MethodGate 13/13, Dark Forest PASS |
| 3. Deployment | **PASS** | VPS membrane live, BearDog TLS shadow 11ms, Channel 3 TLS |
| 4. Composition | **ACTIVE** | Forgejo PRIMARY, shadow layer operational, DNS pending |

### ironGate Advantage

projectNUCLEUS deployments validated on ironGate serve as the clean-room
reference. ironGate's `fetch_primals.sh` bootstraps a fresh NUCLEUS without
prior state — the sovereign deployment pattern that VPS mirrors.

---

## esotericWebb — UI, Agentic, and Composition Evolution

### Posture: Ingest massive evolution, refine UI interaction and agentic patterns

esotericWebb at V7 is the most composition-sophisticated garden product:
7 primal domains consumed via PrimalBridge, graceful 4-pattern degradation,
CRPG substrate with narrative DAG, 342 tests. The amount of upstream
evolution available for absorption is substantial — signals, provenance
trio E2E, petalTongue scene types, Squirrel mechanical constraints.

### What to Absorb

1. **Signal dispatch adoption**: Replace multi-call sequences in bridge
   code with `ctx.dispatch()`. The provenance bridge (`provenance.rs`) can
   collapse its DAG+ledger+braid sequence to `nest.store` and `nest.commit`
   signals. Signal adoption standard: `wateringHole/SIGNAL_ADOPTION_STANDARD.md`.

2. **`primal.announce` for self-registration**: esotericWebb's PrimalBridge
   currently discovers primals but doesn't register itself. If Webb exposes
   methods (e.g., `webb.session.status`, `webb.content.list`), it can
   announce via `ctx.announce("esotericWebb", &methods, &socket)`.

3. **Provenance trio E2E for storytelling sessions**: The individual primals
   (rhizoCrypt, loamSpine, sweetGrass) are all validated. The full session
   loop — `session.create → event.append per scene → lineage.branch on
   player choice → braid.create for AI narration → lineage.certify on
   session end` — can now be exercised against live NUCLEUS on ironGate.

4. **ludoSpring IPC alignment**: ludoSpring still needs to expose 6 methods
   Webb calls (`game.narrate_action`, `game.npc_dialogue`, `game.voice_check`,
   `game.push_scene`, `game.begin_session`, `game.complete_session`). This is
   ludoSpring's P0. Webb's bridge code is already wired — once methods appear,
   the connection activates.

5. **petalTongue scene evolution**: `SceneType::DialogueTree` for narrative
   rendering, NPC portraits, dialogue options, ability check indicators.
   This enables the full storytelling UI experience.

### Priority Evolution Items

| Priority | Item | Action |
|----------|------|--------|
| **P0** | Signal dispatch in bridges | Replace multi-call in `provenance.rs` with `ctx.dispatch("nest.store", ...)` |
| **P0** | Provenance E2E session | Wire full `session.create → events → certify` loop in a real CRPG session on ironGate |
| **P0** | Transport convergence | Confirm UDS fallback stable; test `storytelling-x86-homelan-uds` matrix cell |
| **P1** | `primal.announce` self-registration | Register Webb's own methods for discoverability |
| **P1** | Squirrel mechanical constraints | Pass game mechanical context (dice, stats, cooldowns) as structured data to AI prompt |
| **P1** | petalTongue dialogue scenes | Coordinate `SceneType::DialogueTree` — branch viz, NPC portraits, stat blocks |
| **P2** | Content pack format | `.webbpack` signed archive (BearDog), NestGate storage, loamSpine certification |
| **P2** | Songbird filtered discovery | Coordinate with upstream — `discovery.query({ capabilities: ["game.*"] })` |

### Agentic Refinement

esotericWebb is the strongest test case for the agentic composition pattern:

- **Squirrel as AI DM**: `ai.chat` with context windows — already works,
  but narration ignores game mechanics. Fix: pass `resolved_predicates`,
  `ability_costs`, `dice_results` as structured context, post-validate
  output against game state via ludoSpring `game.voice_check`.

- **Signal planning for complex intent**: `ctx.signal_plan("player wants
  to negotiate with the merchant")` → Squirrel decomposes into signal
  sequence → `ctx.execute_plan()`. This is the highest-leverage agentic
  pattern for storytelling.

- **60Hz composition loop**: The storytelling loop
  (`input → scene → game science → AI narration → provenance → viz → repeat`)
  runs at 60Hz through biomeOS `ContinuousSession`. All pieces exist;
  the integration is the evolution.

### ironGate Advantage

esotericWebb on ironGate has direct UDS access to all 13 NUCLEUS primals.
The full storytelling stack can be exercised locally:
- Desktop NUCLEUS (`tools/desktop_nucleus.sh`) provides the substrate
- esotericWebb composes on top via deploy graph fragments
- petalTongue `live` mode provides the display surface
- Squirrel provides AI narration through local or HTTP provider
- Provenance trio stamps every session for replay and verification

---

## Cross-Cutting Timeline

| When | What | Who |
|------|------|-----|
| **Now** | Signal adoption, BLAKE3 backfill, provenance folder creation | projectFOUNDATION |
| **Now** | Forgejo Actions CI, JupyterHub cutover continuation | projectNUCLEUS |
| **Now** | Signal dispatch in bridges, provenance E2E session on ironGate | esotericWebb |
| **After springs absorb** | ludoSpring 6-method IPC expansion unblocks Webb | esotericWebb + ludoSpring |
| **After springs absorb** | Thread evidence with full NUCLEUS provenance chains | projectFOUNDATION + all springs |
| **Wave 19** | Delta spring convergence, upstream gap surfacing via signal dispatch parity | primalSpring |
| **Glacial horizon** | Sovereign DNS, petalTongue dialogue scenes, content pack format | All gardens |

---

## Summary — One Sentence Each

**projectFOUNDATION**: Layer your 10 threads with guidestone-certified
science, BLAKE3-anchored data, and full NUCLEUS provenance chains — the
foundation validation graph and provenance trio are ready on ironGate.

**projectNUCLEUS**: Your sovereignty infrastructure is operational and
validated — harden it by completing Forgejo Actions CI, JupyterHub
cutover, and sovereign DNS while absorbing signal-based workload
simplification.

**esotericWebb**: You have massive upstream evolution to ingest — signal
dispatch collapses your bridge code, provenance trio E2E is ready for
real sessions, and the agentic composition loop is one integration pass
from end-to-end on ironGate.
