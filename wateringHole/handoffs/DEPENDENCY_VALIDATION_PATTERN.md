# Pattern: Data Dependency Validation (Pre-Dispatch Staging)

**Origin**: toadStool S266 — `validate_data_dependencies()` workload staging  
**Abstracted in**: `primalspring::validation::dependency`  
**Dissemination target**: All primals that consume input files before dispatch

---

## Problem

Before dispatching a workload (GPU kernel, pipeline stage, experiment run),
you need to verify that all required input artifacts exist and haven't been
corrupted since they were staged. Without this, dispatchers encounter
cryptic runtime errors from missing or tampered files.

## Pattern

```rust
use primalspring::validation::dependency::{DependencySpec, validate_dependencies_at};

let deps = vec![
    DependencySpec::required("data/genome.fasta", Some("abc123...")),
    DependencySpec::required("config/pipeline.toml", None),
    DependencySpec::optional("data/annotations.gff", None),
];

let report = validate_dependencies_at(&deps, workload_dir);

if !report.is_ok() {
    for msg in &report.messages {
        eprintln!("{msg}");
    }
    return Err(DispatchError::MissingDependencies);
}

// Record on ValidationResult for structured reporting
report.record_on(&mut validation, "pipeline_stage_1");
```

## Key Types

### `DependencySpec`

| Field | Type | Description |
|---|---|---|
| `path` | `String` | Relative or absolute file path |
| `required` | `bool` | If `true`, absence is a FAIL; if `false`, absence is SKIP |
| `expected_blake3` | `Option<String>` | If set, file hash is verified after existence check |

Constructors: `DependencySpec::required(path, blake3)` and
`DependencySpec::optional(path, blake3)`.

### `DependencyReport`

| Field | Type | Description |
|---|---|---|
| `passed` | `usize` | Dependencies present (and hash-verified if specified) |
| `failed` | `usize` | Required missing or hash mismatch |
| `skipped` | `usize` | Optional absent |
| `messages` | `Vec<String>` | Human-readable per-dep results (`PASS:`, `FAIL:`, `SKIP:`) |

### Functions

- `validate_dependencies(deps)` — validate relative to CWD
- `validate_dependencies_at(deps, base)` — validate relative to a base directory

## Integration with ValidationResult

Call `report.record_on(&mut v, "prefix")` to emit:

- `{prefix}_deps_ok` — aggregate pass/fail
- `{prefix}_dep_detail` — one check per failed dependency (for CI drill-down)

## Adoption Guidance

| Primal | Use Case |
|---|---|
| toadStool | GPU workload staging (already native) |
| barraCuda | Pipeline input validation (FASTA, reference files) |
| coralReef | Shader source validation before compilation |
| loamSpine | Chain anchor payloads before publish |
| biomeOS | Census data files before aggregation |

## BLAKE3 Verification

When `expected_blake3` is set, the file is read and hashed with BLAKE3.
This catches:

- Silent corruption (bit rot, incomplete transfers)
- Tampering (integrity verification for chain-anchored artifacts)
- Stale caches (file modified since manifest was generated)

Generate expected hashes with: `b3sum <file>` or via Rust:
```rust
let hash = blake3::hash(&std::fs::read(path)?).to_hex().to_string();
```

## When NOT to Use

- In-memory data (no filesystem dependency)
- Streaming inputs (data arrives incrementally)
- Dependencies retrieved via RPC (use health checks instead)
