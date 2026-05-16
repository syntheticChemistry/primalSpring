# Primal-Blocked Gap Resolution — Upstream Asks

**Date**: May 16, 2026 (Wave 20)
**Source**: primalSpring ecosystem assessment, Phase B
**Audience**: Primal teams (biomeOS, toadStool, barraCuda, coralReef, BearDog, sweetGrass, loamSpine, rhizoCrypt, songBird)

## Context

The ecoPrimals ecosystem is in late interstadial convergence. All 8 springs are at zero local debt and Wave 17+ signal adoption. The remaining spring gaps are almost entirely **primal-blocked** — they cannot be resolved by spring teams alone. This handoff documents each gap, its upstream owner, the springs it blocks, and the expected resolution shape.

## Priority 1: toadStool Sandbox Gaps

**Owner**: toadStool team
**Blocks**: wetSpring (PG-02), airSpring (AG-006), healthSpring, 3+ foundation workloads

| Gap | Description | Expected Fix |
|-----|-------------|--------------|
| `working_dir` passthrough | Sandbox does not forward `working_dir` from workload TOML to the executed binary | Honor `[execution].working_dir` from workload TOML |
| Env var expansion | `${VAR}` syntax in workload TOML args/paths is not expanded before execution | Expand `${}` references using process environment before exec |
| Data dependency TOML | No way to declare input data dependencies between workloads | Add `[dependencies]` table with `requires = ["workload-id"]` semantics |

**Impact**: Foundation Thread 10 workloads cannot run under sandboxed toadStool. Springs using `toadStool` for compute dispatch cannot pass correct working directories to validation binaries.

## Priority 2: barraCuda / coralReef GPU Gaps

**Owner**: barraCuda team (GPU compute), coralReef team (shader management)
**Blocks**: ludoSpring (GAP-01, GAP-02), airSpring (AG-007), wetSpring (PG-03 resolved, but GPU dispatch pending), groundSpring (GAP-GS-009)

| Gap | Description | Expected Fix |
|-----|-------------|--------------|
| coralReef SM rebuild | Session Manager needs rebuild for IPC-first discovery (current socket path is stale) | Expose `coralreef.sm.status` and `coralreef.shader.compile` via stable UDS |
| barraCuda domain methods | `barraCuda.submit_and_map`, `barraCuda.memory_pool.status` not exposed via IPC | Register domain methods in biomeOS `capability_registry` and serve via UDS |
| GPU API `submit_and_map` | Alignment between barraCuda's internal API and the IPC contract | Standardize on `{ "shader_id": ..., "buffers": [...], "dispatch": {...} }` shape |

**Impact**: ludoSpring's game shader pipeline and airSpring's weather compute dispatch are gated on coralReef SM availability. groundSpring's BTSP/barraCuda interop (GAP-GS-009) needs barraCuda domain IPC.

## Priority 3: Ionic Bridge Standardization

**Owner**: BearDog team (`crypto.sign_contract`)
**Blocks**: hotSpring (GAP-HS-005), healthSpring (ionic bridge), groundSpring (GAP-GS-008)

| Gap | Description | Expected Fix |
|-----|-------------|--------------|
| `crypto.sign_contract` | Ionic bond formation requires BearDog to sign contract payloads; method not yet exposed | Add `crypto.sign_contract` to BearDog's served methods, returning `{ "signature": ..., "contract_id": ... }` |
| Ionic key exchange | No standardized key exchange protocol for ionic bond establishment | Expose `crypto.ionic_exchange` or integrate with existing `crypto.generate_keypair` |

**Impact**: Three springs have ionic bond scenarios that skip on every validation run. This is the last remaining bond type without full live validation coverage.

## Priority 4: Provenance Trio Usability

**Owner**: sweetGrass, loamSpine, rhizoCrypt teams
**Blocks**: All springs doing Tier 3 provenance validation

| Gap | Description | Expected Fix |
|-----|-------------|--------------|
| sweetGrass TCP without BTSP | sweetGrass requires BTSP session for TCP connections; non-BTSP consumers cannot connect | Expose a plain TCP fallback endpoint or UDS alongside BTSP |
| Hex string acceptance | loamSpine and rhizoCrypt reject hex-encoded hashes in some endpoints, expecting raw bytes | Accept both hex string and raw byte formats in `spine.seal`, `dag.event.append`, and `entry.append` |

**Impact**: Springs that lack BTSP wire (healthSpring, parts of airSpring) cannot reach sweetGrass for attribution braids. Hex/bytes mismatch causes spurious validation failures in trio pipeline scenarios.

## Priority 5: biomeOS Schema Standardization

**Owner**: biomeOS team
**Blocks**: projectNUCLEUS discovery cascade, projectFOUNDATION `primal_ipc.sh`

| Gap | Description | Expected Fix |
|-----|-------------|--------------|
| `primal.list` implementation | biomeOS does not yet serve `primal.list`; schema defined by primalSpring Wave 20 | Implement `primal.list` returning `{ "primals": [...], "count": N }` per canonical schema |
| `capability.list` normalization | Some primals return capabilities as objects, others as arrays | Standardize on `{ "capabilities": [...], "count": N }` — always array of strings |

**Impact**: primalSpring has defined and validated the canonical schemas (scenario `schema-standard`, Wave 20). biomeOS implementation would unblock NUCLEUS's `signal_executor.sh` discovery cascade and foundation's validation pipeline.

## Cross-Reference

These gaps correspond to the `Composition gaps` table in `docs/PRIMAL_GAPS.md` (priorities 1-8) and the Phase 32 per-spring gap summary in `docs/CROSS_SPRING_PARITY_SCORECARD.md`. Spring-specific GAP identifiers:

- hotSpring: GAP-HS-087, GAP-HS-005
- wetSpring: PG-02, PG-04, PG-05
- groundSpring: GAP-GS-008, GAP-GS-009
- airSpring: AG-006 through AG-012
- healthSpring: Ionic bridge, NestGate egress, BTSP interop
- ludoSpring: GAP-01 (coralReef SM), GAP-02 (barraCuda domain), GAP-04 (TensorSession)
- neuralSpring: BTSP session (minor)

## Resolution Expectations

These are **not blocking the stadial gate** — the ecosystem can continue converging without them. However, resolving Priority 1-3 would clear the majority of remaining spring gaps and enable full Tier 3 provenance validation across all springs.

Springs will track resolution via the `PRIMAL_GAPS.md` registry and the parity scorecard. No further spring-side evolution waves are needed for these items; they are pure upstream asks.
