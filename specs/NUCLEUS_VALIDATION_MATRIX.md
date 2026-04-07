# NUCLEUS Validation Matrix

**Date**: April 7, 2026  
**Phase**: 25+ (planning)  
**Purpose**: Define the validation matrix for NUCLEUS composition patterns across downstream springs and sporeGarden products, based on gen4 architecture (`infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md`) and primalSpring's Phase 25 modernization results.

---

## Context

primalSpring validates the coordination layer. Phase 25 cleaned all legacy patterns and confirmed Tower Atomic HTTPS works end-to-end. The next evolution step is validating that these patterns compose correctly in downstream springs and gen4 products (sporeGarden).

This matrix defines what each spring and product must demonstrate to confirm NUCLEUS readiness.

---

## Meta-Patterns to Nucleate

These are the composition patterns proven in primalSpring that downstream systems must absorb:

| Pattern | primalSpring Reference | What It Proves |
|---------|----------------------|----------------|
| **Tower Atomic** | `nest-deploy.toml` Phase 1-2 | BearDog + Songbird compose; TLS 1.3 works |
| **Nest Atomic** | `nest-deploy.toml` Phase 1-4 | Tower + NestGate + Squirrel; storage + AI |
| **Node Atomic** | `node_atomic_compute.toml` | Tower + ToadStool; GPU compute |
| **Full NUCLEUS** | `nucleus_complete.toml` | Tower + Nest + Node; all capability domains |
| **Graph-Deployed Composition** | `[[graph.nodes]]` format | biomeOS deploys primals via TOML graphs |
| **Capability Routing** | `capability.call` via Neural API | biomeOS routes method calls to correct primal |
| **HTTPS Through Tower** | `validate_https` node | End-to-end TLS via BearDog→Songbird, no external TLS |
| **Covalent Bonding** | `basement_hpc_covalent.toml` | Multi-node with shared `FAMILY_ID`, mesh discovery |
| **Graceful Degradation** | gen4 COMPOSITION_PATTERNS §III | Product runs even if primals absent |
| **health.liveness** | All primals | Universal JSON-RPC health check (no HTTP) |

---

## Validation Matrix

### Columns

| Column | What to Validate | Method |
|--------|-----------------|--------|
| **A: Graph Format** | Uses `[[graph.nodes]]` with `id` field | Structural parse |
| **B: Capability Names** | All methods use canonical dotted names | Registry cross-check |
| **C: health.liveness** | All primals respond to `health.liveness` | JSON-RPC probe |
| **D: HTTPS Validation** | HTTPS through Tower Atomic works | `http.get` via Neural API |
| **E: Nest Atomic** | NestGate storage round-trip | `storage.store` + `storage.retrieve` |
| **F: Node Atomic** | ToadStool compute available | `compute.submit` |
| **G: AI Routing** | Squirrel `ai.query` via Neural API | `capability.call` |
| **H: Covalent Ready** | Multi-node graph, `FAMILY_ID`, mesh | Graph structure + exp073 pattern |
| **I: Graceful Degradation** | Product runs standalone (no primals) | Launch without stack |
| **J: sporeGarden Deploy** | BYOB deploy graph, plasmidBin binaries | `prepare_spore_payload.sh` |

### Rows (Springs)

| Spring | Domain | A | B | C | D | E | F | G | H | I | J |
|--------|--------|---|---|---|---|---|---|---|---|---|---|
| **primalSpring** | Coordination | **PASS** | **PASS** | **PASS** | **PASS** | **PASS** | **PASS** | structural | structural | n/a | structural |
| **wetSpring** | Biology | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **hotSpring** | Physics | pending | pending | pending | pending | pending | **likely** | pending | pending | n/a | pending |
| **airSpring** | Agriculture | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **groundSpring** | Uncertainty | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **neuralSpring** | ML/Neural | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **healthSpring** | Health | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |
| **ludoSpring** | Game Science | pending | pending | pending | pending | pending | pending | pending | pending | n/a | pending |

### Rows (sporeGarden Products)

| Product | Domain | A | B | C | D | E | F | G | H | I | J |
|---------|--------|---|---|---|---|---|---|---|---|---|---|
| **esotericWebb** | CRPG Engine | pending | pending | pending | pending | pending | pending | pending | pending | **required** | pending |
| **helixVision** | Genomics | planned | planned | planned | planned | planned | planned | planned | planned | **required** | planned |

---

## Validation Approach per Spring

Each spring has a `validate_nucleus_*` binary or equivalent. The matrix cells are validated by:

1. **Structural**: Spring has a biomeOS deploy graph in `graphs/spring_deploy/` (primalSpring has these for all 7). Validate it parses with `[[graph.nodes]]` format.
2. **Live**: Spring's deploy graph is executed on Eastgate with live primals. The spring primal starts, discovers NUCLEUS primals, and performs its domain validation.
3. **Product**: For sporeGarden products, the full composition pipeline runs — PrimalBridge connects to all required primals, graceful degradation works, standalone mode functional.

---

## Priority Order

### Phase A: Graph Format Compliance (columns A + B)

All springs already have nucleated deploy graphs in `primalSpring/graphs/spring_deploy/`. Validate these use canonical `[[graph.nodes]]` format and capability names. This is already done — they were migrated in Phase 25.

Action: Each spring team should verify their local graph files (if any) match `[[graph.nodes]]` format.

### Phase B: Health + HTTPS Validation (columns C + D)

Deploy each spring's NUCLEUS graph and validate:
- All primals respond to `health.liveness`
- HTTPS through Tower Atomic returns a valid response

This requires live primals. primalSpring's `AtomicHarness` can drive this.

### Phase C: Storage + Compute + AI (columns E + F + G)

Validate domain-specific primal interactions:
- NestGate `storage.store`/`storage.retrieve` for experiment data
- ToadStool `compute.submit` for GPU workloads (hotSpring, airSpring, groundSpring)
- Squirrel `ai.query` for AI-assisted analysis

### Phase D: Multi-Node + Covalent (column H)

Validate covalent bonding readiness:
- Each spring's graph can extend to multi-node deployment
- `FAMILY_ID` propagation works across gates
- BirdSong mesh discovery finds peer spring instances

### Phase E: sporeGarden Deployment (columns I + J)

For products:
- Graceful degradation validated (standalone mode works)
- BYOB deploy graph defines full primal topology
- `prepare_spore_payload.sh` produces deployable payload

---

## Integration with Existing Infrastructure

| Component | Role in Matrix |
|-----------|---------------|
| `primalSpring/graphs/spring_deploy/*.toml` | Nucleated deploy graphs for all 7 springs |
| `primalSpring/config/deployment_matrix.toml` | 43-cell deployment matrix (arch × topology × preset × transport) |
| `primalSpring/scripts/validate_deployment_matrix.sh` | Matrix runner |
| `primalSpring/scripts/validate_remote_gate.sh` | Remote gate NUCLEUS health probe |
| `primalSpring/scripts/prepare_spore_payload.sh` | USB spore payload assembly |
| `primalSpring/ecoPrimal/src/harness/` | `AtomicHarness` for live composition |
| `primalSpring/experiments/exp090_tower_atomic_lan_probe/` | LAN discovery validation |
| `infra/whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md` | gen4 patterns (PrimalBridge, graceful degradation, deploy graphs) |

---

## Relationship to gen4

The gen4 vision (`COMPOSITION_PATTERNS.md`) introduces two patterns that extend the NUCLEUS matrix:

1. **Dual Surface** (Creator + Developer): The Creator surface (YAML/CLI) requires graceful degradation (column I). The Developer surface (Rust/PrimalBridge) requires all columns A-H.

2. **PrimalBridge**: Each gen4 product has a bridge that connects to 8+ primal domains. The matrix validates that these domains are reachable through NUCLEUS composition.

The NUCLEUS validation matrix is the gen3→gen4 bridge checkpoint: when all springs pass columns A-H, products can trust the composition layer.

---

## Next Steps

1. **Immediate**: Verify Phase 25 graph migration covers all spring deploy graphs (already done).
2. **Short-term**: Run live Phase B validation on Eastgate for each spring's deploy graph.
3. **Medium-term**: Extend exp090 pattern to validate each spring's NUCLEUS health remotely.
4. **Long-term**: Automate the full matrix as a CI pipeline per spring.
