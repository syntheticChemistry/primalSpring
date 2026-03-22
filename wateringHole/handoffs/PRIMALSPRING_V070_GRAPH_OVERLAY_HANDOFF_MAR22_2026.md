# primalSpring v0.7.0 — Graph Overlay & Cross-Primal Discovery Handoff

**Date:** March 22, 2026
**From:** primalSpring v0.7.0
**To:** All primal and spring teams
**License:** AGPL-3.0-or-later

---

## Executive Summary

primalSpring v0.7.0 introduces two major capabilities that affect every team:

1. **Graph-Driven Overlay Composition** — any primal can be composed at
   any atomic tier via TOML deploy graphs. No code changes needed.
2. **Squirrel Cross-Primal Discovery** — Squirrel discovers sibling primals
   via env vars and socket scanning, then routes AI/tool/context requests.

**77/77 gates**, **253+ tests**, **49 experiments**, **19 deploy graphs**.

---

## 1. What Changed for Your Team

### Every Primal Team

Your primal can now be composed into **any** atomic tier by adding it to a
deploy graph. No harness code changes, no `AtomicType` enum changes.

**To add your primal to a composition:**

```toml
[[graph.node]]
name = "your_primal"
binary = "your_binary_name"
order = N
required = false
depends_on = ["beardog"]
health_method = "health.liveness"
by_capability = "your_domain"
capabilities = ["your.method1", "your.method2"]
```

Save as `graphs/your_overlay.toml`. The harness picks it up automatically.

### What Your Primal Needs (Minimum)

1. **Unix socket JSON-RPC 2.0** (line-delimited NDJSON)
2. **`health.liveness`** method returning `{"status": "ok"}`
3. **`capability.list`** method returning your capabilities
4. **Socket at** `$XDG_RUNTIME_DIR/biomeos/{name}-{family_id}.sock`
   OR accept `--socket` flag OR accept `{NAME}_SOCKET` env var

### Squirrel Integration

If you want Squirrel to route requests to your primal:

1. Declare your capabilities in a deploy graph with `by_capability`
2. Squirrel discovers you via:
   - Explicit env var: `{CAPABILITY}_PROVIDER_SOCKET` in launch profile
   - Socket scan: `$XDG_RUNTIME_DIR/biomeos/` directory
3. Users can then call `tool.list` on Squirrel to see your methods

---

## 2. New Patterns Available

### Graph Merge/Compose

Teams can create partial overlay graphs and merge them:

```rust
let base = load_graph("tower_atomic_bootstrap.toml")?;
let overlay = load_graph("your_overlay.toml")?;
let merged = merge_graphs(&base, &overlay);
```

### spawn=false Nodes

Graph nodes with `spawn = false` are NOT started by the harness. Use this for:
- Validation/coordination nodes (e.g. `validate_tower`)
- CPU fallback paths in conditional DAGs
- Aggregator nodes that don't need their own process

### Graph Execution Patterns (Track 2)

primalSpring now validates 3 of the 5 coordination patterns live:

| Pattern | Graph | Status |
|---------|-------|--------|
| Sequential | `tower_atomic_bootstrap.toml` | **LIVE** (exp010) |
| Parallel | `parallel_capability_burst.toml` | **LIVE** (exp011) |
| ConditionalDag | `conditional_fallback.toml` | **LIVE** (exp012) |
| Pipeline | `streaming_pipeline.toml` | Awaiting sweetGrass |
| Continuous | `continuous_tick.toml` | Awaiting provenance trio |

---

## 3. Available Deploy Graphs (19 total)

| Graph | Pattern | Primals |
|-------|---------|---------|
| `tower_atomic_bootstrap.toml` | Sequential | beardog, songbird |
| `tower_full_capability.toml` | Sequential | beardog, songbird (full caps) |
| `nest_deploy.toml` | Sequential | beardog, songbird, nestgate |
| `node_atomic_compute.toml` | Sequential | beardog, songbird, toadstool |
| `nucleus_complete.toml` | Sequential | beardog, songbird, nestgate, toadstool, sweetgrass |
| `tower_ai.toml` | Sequential | beardog, songbird, squirrel |
| `tower_ai_viz.toml` | Sequential | beardog, songbird, squirrel, petaltongue |
| `nest_viz.toml` | Sequential | beardog, songbird, nestgate, petaltongue |
| `node_ai.toml` | Sequential | beardog, songbird, toadstool, squirrel |
| `full_overlay.toml` | Sequential | beardog, songbird, nestgate, toadstool, squirrel |
| `provenance_overlay.toml` | Sequential | beardog, songbird, rhizocrypt, loamspine, sweetgrass |
| `parallel_capability_burst.toml` | Parallel | beardog, songbird, nestgate, toadstool |
| `conditional_fallback.toml` | ConditionalDag | beardog, songbird, toadstool |
| `streaming_pipeline.toml` | Pipeline | beardog, nestgate, sweetgrass |
| `continuous_tick.toml` | Continuous | all 7 primals |
| `coralforge_pipeline.toml` | Pipeline | beardog, songbird, nestgate, toadstool, sweetgrass |
| `spring_byob_template.toml` | Sequential | template |
| `primalspring_deploy.toml` | Sequential | primalspring coordination |

---

## 4. Integration Checklist for Your Primal

- [ ] Responds to `health.liveness` over Unix socket JSON-RPC
- [ ] Responds to `capability.list` with capability names
- [ ] Binary in `plasmidBin/primals/` (or buildable from source)
- [ ] Launch profile in `config/primal_launch_profiles.toml`
- [ ] At least one deploy graph references your primal
- [ ] Integration test in `ecoPrimal/tests/server_integration.rs`

---

## 5. Files to Reference

| File | Purpose |
|------|---------|
| `config/primal_launch_profiles.toml` | How the harness launches each primal |
| `graphs/*.toml` | All 19 deploy graph definitions |
| `ecoPrimal/src/deploy.rs` | Graph parsing, validation, merge |
| `ecoPrimal/src/harness/mod.rs` | Composition orchestration (AtomicHarness) |
| `ecoPrimal/src/launcher/mod.rs` | Binary discovery, spawn, socket nucleation |
| `ecoPrimal/src/ipc/client.rs` | PrimalClient for JSON-RPC calls |
| `specs/TOWER_STABILITY.md` | 77/77 gate progression |
| `specs/CROSS_SPRING_EVOLUTION.md` | Phase 8 evolution path |
| `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` | Full composition guide (10 sections) |

---

## 6. Evolution Cycle

```
Your team builds/evolves primal binary
    → Copy to plasmidBin/primals/
    → primalSpring validates via AtomicHarness
    → Gates documented in TOWER_STABILITY.md
    → Handoff back with findings + integration patterns
    → Your team absorbs patterns, evolves further
    → Repeat
```

This is the **virtuous cycle**: siloed focus, shared validation.

---

**License**: AGPL-3.0-or-later
