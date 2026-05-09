# Cross-Spring Composition Parity Handoff

**From**: primalSpring Phase 60 (v0.9.25)
**Date**: May 8, 2026
**To**: All delta spring teams + downstream products

## What This Is

primalSpring audited all 8 springs (7 delta + itself) against the full
`papers → Python/R → Rust → primals → NUCLEUS composition` pipeline.
Below are per-spring evolution targets. Full scorecard is at
`docs/CROSS_SPRING_PARITY_SCORECARD.md`.

## Ecosystem Context

- **389** registered capability methods (canonical: `config/capability_registry.toml`)
- **13/13** primals in plasmidBin v2026.05.08 (all 3 architectures)
- **JH-0 MethodGate** adopted by all 13 primals
- **exp094** now validates Tower + Node + Nest + Cross-Atomic parity
- PRIMAL_GAPS items 1-4 updated to **IMPLEMENTED**

---

## Per-Spring Evolution Targets

### hotSpring — Score: STRONG

**Current**: L5 guidestone (reference impl), unconditional primalSpring dep, 993 tests, registry sync test, 17 paper notebooks
**What's working**: The model spring. Only one with unconditional primalSpring dep. Registry sync test is the pattern others should follow.

**Evolution targets**:
1. **Add more deploy graphs** — only 1 graph vs 7+ for healthSpring/wetSpring. The QCD domain has clear multi-graph opportunities (lattice, plasma, nuclear EOS).
2. **Migrate exp binaries to exp crates** — current `exp*.rs` under `src/bin/` are hard to test independently. Consider workspace `experiments/` pattern (like healthSpring/ludoSpring).
3. **barraCuda optional** — Make `barracuda` path dep `optional = true` with IPC-first default for sovereign deployment.

---

### healthSpring — Score: STRONG

**Current**: L5 guidestone, 94 exp crates, 948+ tests, typed IPC clients, 7 graphs
**What's working**: Deepest experiment crate set after ludoSpring. Live IPC in exp117-122 shows the sovereign path.

**Evolution targets**:
1. **Create `capability_registry.toml`** — currently uses Rust constants only. Add TOML + sync test to match neuralSpring/hotSpring pattern.
2. **Convert Python scripts to notebooks** — 54 `.py` baselines exist but not as `.ipynb`. Paper-linkage would be clearer in notebook form.
3. **Feature-gate primalSpring to unconditional** — currently `guidestone` only. Consider following hotSpring's unconditional pattern to enable composition tooling in all builds.

---

### wetSpring — Score: STRONG

**Current**: L4 guidestone (38/38 NUCLEUS), 1,594 tests, 7 graphs, 19 notebooks
**What's working**: Highest test count. Good notebook coverage with tiered reproducibility.

**Evolution targets**:
1. **Explicitly ban `ring`** in `deny.toml` — only spring not banning it. Align with ecosystem posture.
2. **Add registry cross-sync test** — TOML exists but no automated check against primalSpring's 389-method canonical.
3. **Create composition experiment crates** — 0 exp crates currently. The IPC infrastructure exists in guidestone; extract into standalone experiments for downstream replication.

---

### neuralSpring — Score: GOOD

**Current**: L3 guidestone, 1,387 tests, registry with sync test, 10 notebooks, IpcMathClient
**What's working**: One of only 3 springs with registry sync test. Clean deny.toml (bans ring/openssl/rustls).

**Evolution targets**:
1. **Advance guidestone to L4-L5** — L4 (live NUCLEUS) and L5 (primal proof) are explicitly pending.
2. **Add more deploy graphs** — only 1 graph. Inference domain has clear graph opportunities (model routing, tokenization pipeline).
3. **Create composition experiment crates** — IPC lives in `playGround/` but not as standalone `exp*` crates.
4. **Resolve 18 barraCuda IPC surface gaps** — documented in gap-status.json.

---

### ludoSpring — Score: GOOD

**Current**: L5 guidestone, 100 exp crates, 820 tests, 12 graphs, pure composition model
**What's working**: Most experiment crates (100). First "pure composition" spring (no spring binary in end state).

**Evolution targets**:
1. **Add registry cross-sync test** — TOML exists with internal sync but no test against primalSpring's canonical.
2. **Create paper notebooks** — 0 `.ipynb` files. Python baselines exist in `baselines/python/` but aren't in notebook form. Game science papers should be documented.
3. **Fix git tracking** — `ludoSpring` branch tracking is disconnected from remote (observed during pull).

---

### groundSpring — Score: MODERATE

**Current**: L3 guidestone, 965+ tests, 34 notebooks, 6 graphs, registry TOML at root
**What's working**: Richest notebook set (34). Good graph count. Uses CompositionContext in guidestone.

**Evolution targets**:
1. **Advance guidestone to L4+** — currently L3 ("bare scaffold + IPC wiring"). Wire full NUCLEUS checks.
2. **Add registry sync test** — TOML exists but not tested against primalSpring or internally.
3. **Create composition experiment crates** — 0 exp crates. The 34 notebooks suggest deep science, but no Rust experiment isolation.
4. **Feature-gate → default primalSpring dep** — currently `guidestone` only. Consider wider integration.

---

### airSpring — Score: NEEDS WORK

**Current**: L1 guidestone, 1,364 tests, no registry file, no primalSpring dep, 25 notebooks, 4 graphs
**What's working**: High test count (1,364). Good notebook coverage (25) with FAO-56 paper baselines.

**Evolution targets** (priority order):
1. **Add `capability_registry.toml`** — only spring with no registry file at all. Create one following neuralSpring/wetSpring pattern.
2. **Add primalSpring as feature-gated dep** — currently absent. Needed for guidestone upgrade.
3. **Advance guidestone to L3+** — currently L1 (P1-P5 bare checks only). Wire CompositionContext for discovery + liveness.
4. **Create composition experiment crates** — 0 exp crates. The 25 notebooks + 4 graphs show the domain is ready for IPC-based validation.
5. **Promote deny.toml** — exists in sub-crates only. Add root-level workspace deny.toml.

---

## Universal Evolution Targets (All Springs)

1. **barraCuda `optional = true`** — Every spring links barraCuda as a mandatory path dep. Make it optional with IPC-first for sovereign deployment.
2. **Registry cross-sync test** — No spring currently tests its methods against primalSpring's canonical 389-method registry. Add a CI step: `diff <(grep -o 'method = ".*"' your_registry.toml) <(grep -o 'method = ".*"' primalSpring/config/capability_registry.toml)`.
3. **exp094 replication** — primalSpring's exp094 validates full NUCLEUS parity. Each spring should replicate this pattern for their niche (exp094 is the template; exp095 is the proto-nucleate scaffold).

## For Downstream Products (sporeGarden)

- **esotericWebb**: plasmidBin v2026.05.08 has all 13 primals fresh. `fetch.sh` will pull the latest. Your 23 bridge methods are all in the 389-method registry.
- **Future products**: See `plasmidBin/manifest.toml` `[niches]` section for per-spring primal requirements. Your niche defines which primals you need.

## For projectNUCLEUS

- All JH-0 through JH-4 gaps **RESOLVED**, JH-5 Phase 2 **locally complete**
- plasmidBin CI now runs daily catch-up cron (dispatch drops fixed)
- Cross-spring audit complete — scorecard available for deployment readiness assessment
