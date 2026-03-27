# gen4 Composition Audit — primalSpring vs esotericWebb

**Date**: March 24, 2026
**Status**: Active — Phase 17 planning
**Context**: esotericWebb (sporeGarden) is the first gen4 product consuming
primals via IPC. This audit identifies gaps between what primalSpring validates
(gen3) and what gen4 products actually need.

---

## Methodology

Compared three surfaces:
1. **primalSpring capability registry** (`config/capability_registry.toml`) — 37 capabilities
2. **esotericWebb deploy graphs** (`graphs/webb_*.toml`) — 6 composition health nodes
3. **esotericWebb source** (`webb/src/ipc/`) — bridge, resilience, session pipeline

---

## Shortcoming 1: Composition Health Namespace Mismatch

primalSpring exposes gen3 composition health:

| primalSpring Capability | Stack |
|------------------------|-------|
| `composition.tower_health` | BearDog + Songbird |
| `composition.node_health` | Tower + ToadStool |
| `composition.nest_health` | Tower + NestGate |
| `composition.nucleus_health` | Full NUCLEUS |
| `composition.tower_squirrel_health` | Tower + Squirrel |

Webb expects gen4 composition health:

| Webb Expected Capability | Stack |
|--------------------------|-------|
| `composition.webb_tower_health` | BearDog + Songbird |
| `composition.webb_node_health` | Tower + ToadStool |
| `composition.webb_nest_health` | Tower + NestGate |
| `composition.webb_ai_viz_health` | Tower + Squirrel + PetalTongue |
| `composition.webb_provenance_health` | Nest + Provenance Trio |
| `composition.webb_full_health` | All 8 domains |

**Gaps:**

| Gap | Severity | Notes |
|-----|----------|-------|
| No `webb_*` namespace at all | High | Webb graphs reference `composition.webb_*_health` — primalSpring returns nothing for these |
| No `ai_viz` composition | Medium | primalSpring has `tower_squirrel_health` but not the 3-way AI+Viz overlay Webb uses |
| No `provenance` composition | Medium | primalSpring validates provenance trio structurally but doesn't expose a composition health endpoint for it |
| No `full` composition | Medium | `nucleus_health` covers NUCLEUS primals but Webb's "full" includes 8 domain-specific primals |

**Resolution**: Add 6 `composition.webb_*_health` capabilities to the registry
and implement handlers that map to existing atomic validation + domain probes.
The gen3 capabilities remain — gen4 capabilities layer on top.

---

## Shortcoming 2: Transport Priority Inversion

primalSpring discovers primals UDS-first (local socket):
```
socket_path() → XDG_RUNTIME_DIR/biomeos/{primal}.sock
  → /tmp/{primal}.sock fallback
  → env override
```

Webb discovers primals TCP-first:
```
PrimalBridge::discover() → tcp_addr from registry
  → UDS fallback from socket_path
  → absent → degrade
```

**Impact**: A stack that passes primalSpring's gen3 validation (UDS) could
fail Webb's gen4 composition (TCP) if the primal doesn't bind a TCP listener.

**Known example**: rhizoCrypt is TCP-only (`:9401/rpc`), ignores
`RHIZOCRYPT_SOCKET` env var — passes Webb's TCP discovery but fails
primalSpring's UDS-first probing unless the env override is set manually.

**Resolution**: Phase 17 experiments should validate both transport orderings.
Add a `transport_priority` parameter to composition health probes: `uds_first`
(gen3 default) and `tcp_first` (gen4 mode).

---

## Shortcoming 3: No Capability Drift Detection

Webb uses capability strings in four independent locations with no shared schema:

| Surface | Source | Example Mismatch |
|---------|--------|-----------------|
| Bridge method constants (`ipc/mod.rs`) | `dag.session.create` | Wire-level method name |
| Deploy graph capabilities (`graphs/*.toml`) | `composition.webb_full_health` | Expected of validators |
| Capability registry (`capability_registry.toml`) | `webb.session.start` | Self-knowledge |
| Niche YAML (`niches/*.yaml`) | `game.*` (glob pattern) | BYOB declaration |

Webb cannot self-validate this consistency because it's a product, not a
validator. primalSpring already validates deploy graph capabilities and registry
consistency for gen3 primals — extending to gen4 product surfaces is natural.

**Resolution**: New experiment (exp077) that:
1. Parses Webb's `capability_registry.toml` for declared methods
2. Parses Webb's deploy graph nodes for expected capabilities
3. Parses Webb's `PRIMAL_DOMAINS` from `ipc/mod.rs` for domain→primal mapping
4. Cross-references all three against primalSpring's own registry
5. Reports drift as structured validation failures

---

## Shortcoming 4: No Session Pipeline Ordering Validation

Webb's `GameSession::act()` composes primals in a strict 6-phase pipeline:

```
1. narrate_action   (ludoSpring → Squirrel fallback)    DOMAIN_GAME → DOMAIN_AI
2. npc_dialogue     (ludoSpring → Squirrel)             DOMAIN_GAME → DOMAIN_AI
3. evaluate_flow    (ludoSpring)                         DOMAIN_GAME
4. push_scene_to_ui (petalTongue)                        DOMAIN_VISUALIZATION
5. dag_event_append (rhizoCrypt)                         DOMAIN_DAG
6. dag_session_close (rhizoCrypt, on ending)             DOMAIN_DAG
```

primalSpring validates individual IPC calls and graph execution patterns, but
has no test that exercises this specific cross-domain pipeline ordering.

**Impact**: A primal could pass all individual health checks but fail when
composed into this sequence (e.g., DAG append depends on session state set
by narrate_action, which depends on ludoSpring availability).

**Resolution**: New experiment (exp076) that simulates the `act()` pipeline
with real primals, verifying:
- Correct fallback chain (game → ai when game unavailable)
- State propagation between phases (session_id from DAG create flows to append)
- Degradation at each phase (skip vs fail vs default)
- Timing (phases must be sequential, not parallel)

---

## Shortcoming 5: Resilience Semantics Untested Against Real Consumer Patterns

primalSpring has the resilience primitives (absorbed from sibling springs):
- `IpcError::is_recoverable()` — from wetSpring V133
- `CircuitBreaker` with epoch-based state — from healthSpring V42
- `RetryPolicy` with exponential backoff

But these have never been tested against the specific patterns a gen4 consumer
uses. Webb's `resilient_call` has a precise contract:
- Circuit check before call (fail fast if open)
- Only `is_recoverable` errors trigger retry
- Non-recoverable errors immediately count as circuit failure
- All retries exhausted → circuit failure

**Gap**: primalSpring's `is_recoverable` classification may differ from Webb's.
Webb treats `MethodNotFound`, `InvalidParams`, `ParseError` as non-recoverable.
primalSpring's `is_recoverable` comes from wetSpring's IPC error taxonomy — the
overlap should be verified.

**Resolution**: Add property tests (proptest) that generate random IPC error
sequences and verify primalSpring's resilience primitives produce the same
pass/fail/retry decisions as Webb's `resilient_call` would.

---

## Shortcoming 6: No Degradation Correctness Testing

Webb has four explicit degradation patterns in `PrimalBridge`:

| Pattern | Behavior When Absent | primalSpring Equivalent |
|---------|---------------------|------------------------|
| `call_or_default` | Return typed default | `check_skip` (binary: skip or fail) |
| `call_fire` | No-op silently | None |
| `call_extract_id` | Return `None` | None |
| `call_passthrough` | Return empty JSON | None |

primalSpring's `check_skip` / `check_or_skip` is binary — either the primal
is present and we call it, or it's absent and we skip the test. Webb's
degradation is richer: absent primals return sensible defaults that keep the
game running.

**Impact**: primalSpring can verify "does this primal exist?" but not "does
the product get a reasonable result when this primal doesn't exist?"

**Resolution**: Phase 17 experiments should include degradation correctness
tests that:
1. Start a composition with deliberately absent primals
2. Call the composition health endpoint
3. Verify the response includes degraded-but-functional defaults
4. Verify no panics, no connection hangs, no leaked file descriptors

---

## Shortcoming 7: ludoSpring Not in plasmidBin

The entire `game` domain is unvalidatable in gen4 mode because ludoSpring has
no deployed binary in `plasmidBin`. This blocks:

- `composition.webb_full_health` (requires all 8 domains)
- Session pipeline ordering tests (phases 1-3 depend on `game.*` RPCs)
- ludoSpring + Webb `RulesetCert` validation (GAP-009)

**Resolution**: ludoSpring team deploys binary. primalSpring adds ludoSpring
to its launch profiles (already has provenance trio and Tower primals).

---

## Summary: Phase 17 Work Items

| Item | Experiments | Capability Registry | Code |
|------|------------|--------------------|----|
| 6 Webb composition health endpoints | exp075 | +6 capabilities | coordination handler dispatch |
| Transport priority (TCP-first mode) | exp075 | — | discover module dual-mode |
| Capability drift detection | exp077 | — | new validation module |
| Session pipeline ordering | exp076 | — | harness pipeline runner |
| Resilience semantics alignment | proptest | — | proptest_ipc extension |
| Degradation correctness | exp076 | — | harness degradation mode |
| ludoSpring launch profile | — | — | config/primal_launch_profiles |

---

## Priority Order

1. **Webb composition health endpoints** — unblocks gen4 deploy graph validation
2. **ludoSpring launch profile** — unblocks full pipeline testing (pending binary)
3. **Capability drift detection** — catches misalignment before products ship
4. **Session pipeline ordering** — validates the actual gen4 use case
5. **Transport priority** — ensures gen3 validation covers gen4 transport paths
6. **Resilience semantics** — verifies primalSpring's primitives match Webb's expectations
7. **Degradation correctness** — validates the gen4 graceful degradation contract
