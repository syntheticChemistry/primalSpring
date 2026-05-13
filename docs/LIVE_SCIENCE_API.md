# Live Science API — Tier 2 Wire Contract

**Version**: 1.1.0
**Date**: May 13, 2026
**Owner**: primalSpring (L2) + toadStool (L1) + barraCuda (L1)
**Status**: ACTIVE — All Tier 2 methods IMPLEMENTED. 7/7 delta springs wired.

---

## Purpose

The Live Science API defines the JSON-RPC methods that downstream products
(projectNUCLEUS, lithoSpore, foundation notebooks) need to interact with
primal workloads at runtime. This is the Tier 2 interface — beyond Tier 0
(CLI) and Tier 1 (notebook + sporePrint).

Tier 2 enables:
- Notebooks querying live compute workloads
- lithoSpore modules validating against running primals
- projectNUCLEUS deploy graphs wiring workload dispatch
- Foundation threads consuming live provenance chains

---

## Methods

### toadStool (compute dispatch)

#### `toadstool.validate`

Validate a workload TOML against the current compute environment without
executing it. Returns compatibility report.

```json
{
  "method": "toadstool.validate",
  "params": {
    "workload_path": "/path/to/workload.toml",
    "dry_run": true
  }
}
```

**Response**:

```json
{
  "result": {
    "valid": true,
    "gpu_available": true,
    "precision_tier": "DF64",
    "estimated_dispatch_time_ms": 1200,
    "warnings": [],
    "required_capabilities": ["compute", "shader"]
  }
}
```

**Blocks**: projectNUCLEUS Tier 2 for all springs. Without this, notebooks
cannot pre-flight workloads before dispatch.

#### `toadstool.list_workloads`

List registered workload TOMLs and their status.

```json
{
  "method": "toadstool.list_workloads",
  "params": {
    "filter": "active"
  }
}
```

**Response**:

```json
{
  "result": {
    "workloads": [
      {
        "id": "yukawa_md_force",
        "path": "workloads/yukawa_md.toml",
        "status": "ready",
        "last_run": "2026-05-12T14:30:00Z",
        "precision_tier": "F32"
      }
    ],
    "total": 1
  }
}
```

#### `compute.dispatch.submit`

Submit a workload for execution. Already defined in the compute trio IPC
contract (Wave 8). Included here for completeness.

```json
{
  "method": "compute.dispatch.submit",
  "params": {
    "binary_b64": "<base64 shader binary>",
    "dispatch_dims": [256, 1, 1],
    "buffers": [
      { "data_b64": "<base64>", "size": 1024, "binding": 0 }
    ],
    "timeout_ms": 30000
  }
}
```

### barraCuda (math workloads)

#### `barracuda.precision.route`

Query the optimal precision strategy for a given operation class and
tolerance requirements. Returns the recommended precision tier, shader
name, and whether the compiler (coralReef) is required.

```json
{
  "method": "barracuda.precision.route",
  "params": {
    "operation": "stats.mean",
    "input_range": "normal",
    "tolerance": "1e-9",
    "domain": "lattice_qcd",
    "hardware_hint": "compute"
  }
}
```

**Response**:

```json
{
  "result": {
    "strategy": "f64",
    "shader": "stats_mean_f64",
    "precision_tier": 7,
    "recommended_tier": "DF64",
    "fma_safe": false,
    "requires_compiler": true,
    "hardware_hint": "compute"
  }
}
```

### coralReef (shader compilation)

#### `shader.compile.wgsl`

Compile a WGSL shader to the target ISA. Already defined in the compute trio
IPC contract (Wave 8).

```json
{
  "method": "shader.compile.wgsl",
  "params": {
    "source": "<WGSL source>",
    "target": "ptx",
    "sm_version": 70
  }
}
```

### nestGate (content + storage)

#### `content.put` / `content.get`

Store and retrieve content-addressed data (CAS). Shipped in NestGate Session 60
with full transport parity across all 4 surfaces (UDS, SemanticRouter,
isomorphic IPC, HTTP).

> **Note**: `content.*` (CAS — BLAKE3-addressed, immutable) and `storage.*`
> (generic blob API — mutable, keyed) are **separate** domains in the
> capability registry. Both are owned by nestGate but serve different roles.
> See `config/capability_registry.toml` for the full method listings.

### Provenance trio (DAG + ledger + attribution)

#### `provenance.session.create` → `provenance.event.append` → `spine.seal`

The provenance pipeline for workload attestation. Each compute dispatch can
emit a blake3 hash witness through the DAG, commit it to the ledger, and
anchor attribution.

> **Wire name aliases**: `provenance.session.create` and `dag.session.create`
> are aliases — rhizoCrypt accepts both. loamSpine's ledger surface uses
> `session.create`/`session.state` on the `ledger` capability, while the
> registry also defines `entry.append`/`entry.get` under the `entry` domain.

---

## Implementation Status

| Method | Owner | Status | Blocks |
|--------|-------|--------|--------|
| `toadstool.validate` | toadStool | **IMPLEMENTED** (S250) | — |
| `toadstool.list_workloads` | toadStool | **WIRED** (S245+) | — |
| `compute.dispatch.submit` | toadStool | **WIRED** (Wave 8) | — |
| `barracuda.precision.route` | barraCuda | **IMPLEMENTED** (v0.4.0, 649 tests) | — |
| `shader.compile.wgsl` | coralReef | **WIRED** (Wave 8) | — |
| `content.put/get` | nestGate | **SHIPPED** (Session 60, 4-surface parity) | — |
| `provenance.session.create` | rhizoCrypt | **SHIPPED** (alias: `dag.session.create`) | — |
| `session.create` / `entry.append` | loamSpine | **SHIPPED** (ledger + entry domains) | — |

---

## Adoption Path

1. ~~**toadStool implements `toadstool.validate`**~~ — **DONE** (S250) + list_workloads (S245+)
2. ~~**barraCuda implements `precision.route`**~~ — **DONE** (v0.4.0, 649 tests)
3. ~~**Delta springs wire Tier 2**~~ — **DONE** (7/7 springs wired, May 13)
4. **projectNUCLEUS wires Tier 2** notebooks to use JSON-RPC instead of CLI
5. **lithoSpore modules** gain live validation via `toadstool.validate`
6. **Foundation threads** can consume live provenance chains for reproducibility
