# Experiment 115: Nest Ingest pseudoSpore

**Track**: 10 (Spore Gateway)
**Status**: Structural (pending live NUCLEUS)
**Date**: May 27, 2026

## Objective

Validate the spore ownership matrix end-to-end: a pseudoSpore emitted by any
spring can be ingested by `biomeos nucleus ingest` on a Nest Atomic, and the
round-trip preserves BLAKE3 integrity.

## Gate Criterion

**"Any spring can emit a pseudoSpore; any NUCLEUS can ingest it."**

## Checks

| # | Check | Status | Method |
|---|-------|--------|--------|
| 1 | `litho emit-pseudospore --domain-profile` produces valid structure | structural | scope.toml parse |
| 2 | Unified liveSpore.json has `envelope` + `validations` shape | structural | JSON parse |
| 3 | `biomeos nucleus ingest --verify` accepts the pseudoSpore | pending | live NUCLEUS |
| 4 | NestGate `storage.get` retrieves ingested content | pending | JSON-RPC probe |
| 5 | sweetGrass `braid.query` finds the ingestion braid | pending | JSON-RPC probe |
| 6 | `biomeos nucleus emit` re-packages the content (future) | planned | round-trip |
| 7 | Re-emitted pseudoSpore passes `litho audit` | planned | litho audit --json |

## Architecture

```
Spring (hotSpring/groundSpring/any)
  → litho emit-pseudospore --domain-profile ./domain_profile.toml
    → pseudoSpore directory (scope.toml, data.toml, liveSpore.json, ...)
      → biomeos nucleus ingest <spore-path> --verify --register
        → NestGate storage.put (content-addressed blobs)
        → sweetGrass braid.create (provenance registration)
        → loamSpine spine.append (permanence ledger)
        → Composition registry update
```

## Key Files

- `infra/wateringHole/SPORE_OWNERSHIP_MATRIX.md` — ownership boundaries
- `gardens/lithoSpore/crates/pseudospore-core/` — shared envelope crate
- `primals/biomeOS/crates/biomeos/src/modes/nucleus_ingest.rs` — gateway
- `gardens/lithoSpore/specs/PSEUDOSPORE_STANDARD.md` — unified schema

## Prerequisites

- biomeOS v3.76+ with `nucleus ingest` subcommand
- Live Nest Atomic (NestGate + provenance trio running)
- At least one pseudoSpore artifact (hotSpring v1.6.1 is first candidate)

## Scenario Integration

This experiment feeds into `s_nest_atomic.rs` as an additional scenario:
the spore ingest/verify round-trip through Nest Atomic storage.
