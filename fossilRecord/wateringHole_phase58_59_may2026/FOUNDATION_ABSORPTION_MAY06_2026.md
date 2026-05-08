# Foundation Absorption — primalSpring as Validation Pressure

**Date**: 2026-05-06
**From**: primalSpring v0.9.24 (Phase 59)
**For**: projectNUCLEUS, spring teams, sporeGarden/foundation

---

## What Happened

primalSpring absorbed the `sporeGarden/foundation` layer — not by modeling
foundation's domain (sources, targets, workloads, sediment), but by proving
the underlying NUCLEUS composition can execute every RPC call the
foundation pipeline requires.

This is **validation pressure**: primalSpring drives the composition through
live IPC and reports what works, what skips, and what fails. Foundation and
projectNUCLEUS absorb these patterns downstream.

## What Changed in primalSpring

### 1. Graph Schema Extensions

`ecoPrimal/src/deploy/mod.rs`:

- **`GraphNode.fallback: Option<String>`** — Graceful degradation for
  optional nodes. Foundation-style graphs mark non-critical primals
  (petalTongue, squirrel, coralReef) with `fallback = "skip"`.
- **`GraphMetadata.purpose: Option<String>`** — Composition intent:
  `"deployment"`, `"validation"`, or `"foundation"`. Structural checks
  use this to enforce stricter invariants on validation compositions.

### 2. Structural Check Evolution

`ecoPrimal/src/deploy/validation.rs`:

- **Fallback consistency**: `fallback = "skip"` requires `required = false`.
  A node cannot be both skippable and mandatory.
- **Provenance trio requirement**: Graphs with `purpose = "validation"` or
  `purpose = "foundation"` must include `dag`, `ledger`, and `attribution`
  capabilities. Validation compositions without the provenance trio are
  structurally incomplete.

### 3. Foundation Validation Graph

`graphs/compositions/foundation_validation.toml`:

Full NUCLEUS composition (12 nodes) adapted from
`sporeGarden/foundation/graphs/foundation_validation.toml`:

| Phase | Nodes | Required |
|-------|-------|----------|
| 0 | biomeOS Neural API | yes (spawn=false) |
| 1–2 | Tower (BearDog + Songbird) | yes |
| 3–5 | Node (toadStool + barraCuda + coralReef) | coralReef optional |
| 6–9 | Nest (NestGate + rhizoCrypt + loamSpine + sweetGrass) | all required |
| 10–11 | Meta-tier (petalTongue + squirrel) | both optional |

Optional nodes use `fallback = "skip"` and `required = false`.

### 4. Experiment 107 — Foundation Validation

`experiments/exp107_foundation_validation/`:

8-phase Rust binary validating the sediment pipeline through live IPC:

1. **Structural** — parse graph, verify purpose, node count, trio caps
2. **Discovery** — `CompositionContext` resolves all capabilities
3. **Health** — probe 8 required primals
4. **Provenance** — `dag.session.create` → `dag.event.append` → `dag.session.complete`
5. **Storage** — `storage.store` + `storage.get` roundtrip
6. **Compute** — `health.liveness` on toadStool
7. **Ledger** — `spine.create` + `entry.append`
8. **Attribution** — `braid.create`

When primals are unavailable, phases skip gracefully with `check_skip`.
When live, this proves the full foundation composition works end-to-end
through Rust IPC — no shell scripts, no mocks.

### 5. Quality

- All workspace tests pass (568 lib + 10 integration + 10 doc + new tests)
- `cargo clippy --workspace` — 0 warnings
- CHECKSUMS regenerated (1/18 changed: `deploy/mod.rs`)

## What projectNUCLEUS Should Do

1. **Pull primalSpring** — get the graph schema extensions and structural checks
2. **Absorb `graphs/compositions/foundation_validation.toml`** — adapt it to
   your operational graph format with RPC phases
3. **Absorb structural check patterns** — `fallback` consistency and provenance
   trio requirements strengthen your own graph validation
4. **Run exp107 against your deployment** — it gracefully skips unavailable
   primals, so it works as a progressive validation tool

## What Spring Teams Should Do

1. **Pull primalSpring** — the `CompositionContext` patterns in exp107 are the
   reference for how to drive multi-primal validation through IPC
2. **Use `purpose = "validation"` in your own graphs** — structural checks
   will enforce provenance trio presence
3. **Use `fallback = "skip"` for optional primals** — graceful degradation
   makes your compositions more resilient

## What We Did NOT Do

- Did not create a `foundation` module in primalSpring — foundation owns
  its domain schemas (sources, targets, workloads, sediment lifecycle)
- Did not model the sediment layer as Rust types — the pipeline is
  foundation's concern
- Did not add `CompositionPurpose` or `AtomicType::Foundation` — foundation
  validation uses existing atomics (Tower + Node + Nest)

The boundary is clear: **primalSpring validates the composition plumbing;
foundation defines what flows through it.**
