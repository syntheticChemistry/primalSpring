# primalSpring Phase 60 — Downstream Handoff for Springs & Gardens

**Date**: May 7, 2026
**From**: primalSpring v0.9.25 (eastGate)
**For**: All springs in our river delta + gardens + projectNUCLEUS

---

## What Springs Should Absorb

### 1. Capability Registry (384 methods, 82 domains)

`config/capability_registry.toml` is the canonical source of truth for all JSON-RPC
method strings in the ecosystem. Springs should validate their method strings against it.

**New domains added this cycle:**
- `content.*` (8 methods) — NestGate content-addressed storage
- `viz.*` (4) — petalTongue visualization
- `math.*` (6), `rng.*` (1) — barraCuda math primitives
- `beacon.*` (2), `lineage.*` (1), `tls.*` (3) — BearDog crypto internals
- `tools.*` (2), `ionic.*` (1), `tool.*` (1) — primalSpring coordination

**Action**: Copy `config/capability_registry.toml` or validate your method strings
against it. Use `tools/check_method_strings.sh` as a pattern for your own CI.

### 2. Graph Method Validator Pattern

`tools/check_graph_methods.sh` validates graph TOML capability references against
the registry. Springs with their own graph TOMLs should adopt this pattern.

Key design: spring-domain capabilities (e.g., `physics.*`, `ecology.*`, `game.*`)
are separated from primal-domain methods. Only primal drift is an error.

### 3. Content-Addressed Storage (NestGate NG-1 through NG-4)

NestGate now has 8 `content.*` methods for content-addressed storage:
- `content.put` — store content, get BLAKE3 hash back
- `content.get` — retrieve by hash
- `content.publish` / `content.promote` — versioned manifests for atomic deploys
- `content.resolve` — path→hash→content resolution

Springs that store artifacts, notebooks, or validation results should consider
using `content.put` instead of raw `storage.store` for deduplication and integrity.

### 4. RootPulse Commit Workflow (Fully Executable)

biomeOS v3.45 aligned all 6 rootpulse graph files to match canonical method names.
The workflow is now 6/6 phases executable:

1. `spine.create` (LoamSpine) — create history spine
2. `dag.session.create` (rhizoCrypt) — open DAG session
3. `crypto.sign` with `message` param (BearDog) — sign merkle root
4. `dag.dehydration.trigger` (rhizoCrypt) — dehydrate session
5. `session.commit` (LoamSpine) — commit to permanent history
6. `braid.create` (sweetGrass) — create provenance braid

Springs that need provenance/history can compose these via biomeOS graph execution:
`biomeos graph execute rootpulse_commit --param SESSION_ID=abc --param SPINE_ID=s1`

### 5. Shader Rewire Guide (barraCuda)

barraCuda published a two-tier rewire guide for springs still linking local shaders:

**Tier A — JSON-RPC inline** (small data): `activation.softmax`, `activation.gelu`,
`math.sigmoid`, `stats.entropy`, `stats.chi_squared`, `ml.attention`

**Tier B — Tensor pipeline** (>10K elements): `tensor.create` → `tensor.batch.submit`

See `wateringHole/handoffs/BARRACUDA_V0312_STALE_AUDIT_TRIAGE_SHADER_REWIRE_GUIDE_MAY07_2026.md`

### 6. Deep Debt Patterns to Adopt

Patterns that worked well in primalSpring and are recommended for other springs:

- **Binary modularization**: Large `main.rs` files should extract into dispatch,
  metrics, lifecycle, niche modules. Aim for <200L main.rs.
- **Probe cache with TTL**: `CachedProbeResult<T>` pattern for IPC health checks.
- **Profile registry as data**: Deploy profiles as TOML-driven data, not code.
- **`#[expect(reason)]`**: Replace all `#[allow]` with `#[expect(reason = "...")]`.
- **Method string CI**: Validate all `"method.name"` literals against a registry.

### 7. sporePrint Notebook Pipeline

Per projectNUCLEUS Phase 59: all springs should create `notebooks/` directory with
5 notebooks following the pattern in `wetSpring/notebooks/NOTEBOOK_PATTERN.md`:

1. `01-domain-validation` — flagship validation story
2. `02-benchmark-comparison` — Python vs Rust vs GPU
3. `03-paper-reproductions` — per-researcher evidence map
4. `04-cross-spring-connections` — ecosystem flows
5. `05-domain-deep-dive` — most compelling insight

Load frozen data via `../experiments/results/*.json`. Use matplotlib for charts.
Push to main — `notify-sporeprint.yml` fires automatically.

## What primalSpring Needs from Springs

- **`sporeprint/validation-summary.md`** — stub exists, fill with headline numbers
- **Consumed capabilities list** — which primals your spring calls over IPC
- **Method string alignment** — validate your method literals against the registry
- **Experiment JSON results** — frozen data in `experiments/results/` for notebooks

## Ecosystem State Summary

| Metric | Value |
|--------|-------|
| BTSP Phase 3 | 13/13 FULL AEAD |
| Localhost default | 13/13 |
| Capability wire standard | 13/13 L2+ |
| Registered methods | 384 |
| Open upstream gaps | 0 (P1), 2 (P2), 6 (P3) |
| primalSpring tests | 666 (618 passed + 48 ignored, 0 failures) |
| Clippy warnings | 0 |

## Pull

- `primalSpring` main branch — registry, patterns, tools, handoffs
- `infra/wateringHole` — consolidated handoffs from all primal teams
- `projectNUCLEUS` — workload specs, benchScale framework, gap handbacks
