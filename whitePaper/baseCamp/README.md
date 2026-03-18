# primalSpring baseCamp — Coordination and Composition Validation

**Date**: March 17, 2026  
**Status**: Phase 0→1 — 38 experiments, 69 unit tests, real discovery, honest skip/fail

---

## What This Is

Where baseCamp papers for other springs explore scientific questions using the
ecoPrimals infrastructure, primalSpring's baseCamp explores **the infrastructure
itself**. The "papers" are the atomics. The "experiments" are composition patterns.
The validation target is biomeOS and the Neural API.

## The Paper

See `whitePaper/gen3/baseCamp/23_primal_coordination.md` for the full baseCamp
paper documenting primalSpring's validation of ecosystem coordination.

## Experiments by Track

| Track | Domain | Experiments | Key Question |
|-------|--------|-------------|--------------|
| 1 | Atomic Composition | exp001–006 | Do atomics deploy correctly? |
| 2 | Graph Execution | exp010–015 | Do all 5 coordination patterns work? |
| 3 | Emergent Systems | exp020–025 | Do Layer 3 systems emerge correctly? |
| 4 | Bonding & Plasmodium | exp030–034 | Does multi-gate coordination work? |
| 5 | coralForge | (exp025) | Does the neural object pipeline work? |
| 6 | Cross-Spring | exp040–044 | Do cross-spring data flows work? |
| 7 | Showcase-Mined | exp050–059 | Do early coordination patterns from phase1/phase2 work? |

## Current State (v0.1.0)

| Metric | Value |
|--------|-------|
| Experiments | 38 (7 tracks) |
| Unit tests | 69 |
| clippy (pedantic+nursery) | 0 warnings |
| cargo doc (missing_docs) | 0 warnings |
| `#[allow()]` in production | 0 |
| unsafe_code | Workspace-level `forbid` via `[workspace.lints.rust]` |
| C dependencies | 0 (pure Rust, ecoBin compliant) |
| IPC client | Real Unix socket client with JSON-RPC 2.0 |
| Discovery | Runtime `discover_primal()` via env/XDG/temp_dir + Neural API |
| Validation | `check_bool` (real) + `check_skip` (honest scaffolding) |
| Dishonest scaffolding | 0 (all experiments use honest skip or real validation) |
