# Archived: inference/ module — April 12, 2026

The vendor-agnostic inference provider abstraction (`InferenceClient`, wire types)
was removed from `ecoPrimal/src/` during the seasonal tightening.

## Why

- **Zero imports** anywhere in primalSpring (no experiment, server, or module used it)
- Inference is Squirrel/neuralSpring's domain, not composition validation
- The `inference.*` wire standard is documented in wateringHole — springs that need
  inference use Squirrel directly

## What's here

- `mod.rs` — `InferenceClient` with `complete`, `embed`, `models` methods
- `types.rs` — Wire types: `CompleteRequest`, `EmbedRequest`, `ModelsResponse`, etc.

## Revival path

If primalSpring needs to validate inference compositions (e.g., "does Squirrel
route inference.complete correctly through a NUCLEUS composition?"), this module
can be revived with actual experiment coverage.
