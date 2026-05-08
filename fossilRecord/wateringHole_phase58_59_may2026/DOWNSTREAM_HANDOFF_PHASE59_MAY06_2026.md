# Downstream Handoff — Phase 59 Security Convergence

**Date**: May 6, 2026
**From**: primalSpring v0.9.24 (Phase 59)
**For**: projectNUCLEUS, sporeGarden/foundation, all spring teams
**License**: AGPL-3.0-or-later

---

## What Happened

Phase 59 closed all open security gaps from the projectNUCLEUS Phase 2a
penetration test. The ecosystem is now fully converged:

- **13/13 primals** default to `127.0.0.1` bind (PG-55 RESOLVED)
- **NestGate** BTSP method-level auth gating live (PG-56 RESOLVED)
- **skunkBat** multi-dimensional anomaly detection seeded (PG-57 RESOLVED)
- **Songbird** HTTP/IPC bind separation (PG-58 RESOLVED)
- **sweetGrass** `--http-address` and `--port` formats documented (PG-59 RESOLVED)
- **Foundation layer** absorbed — `exp107` validates full sediment pipeline via IPC
- **Discovery Escalation Hierarchy** live (Songbird → biomeOS Neural API → UDS → socket → TCP)
- **Zero open upstream gaps**

Pull `primalSpring`, `infra/wateringHole`, and all `primals/` repos before starting.

---

## For projectNUCLEUS (ironGate)

### What to absorb

1. **Bind policy enforcement**: Your deploy scripts should pass `--bind 127.0.0.1`
   (or rely on new defaults) for all primals. The `bind_policy = "localhost"` field
   in graph metadata tells guidestone to validate this. Update your
   `deploy.sh` to remove any `--bind 0.0.0.0` overrides unless intentional.

2. **PG-56 NestGate auth**: If your composition accesses `storage.list` over TCP
   fallback (Tier 5), note that TCP fallback is **not** gated by BTSP. Use UDS
   or complete BTSP handshake for authenticated access.

3. **Foundation validation graph**: `graphs/compositions/foundation_validation.toml`
   defines a 12-node NUCLEUS for scientific validation. If you want to validate
   foundation threads through your deployment, include this graph.

4. **Checksums**: primalSpring CHECKSUMS are now generated via
   `tools/regenerate_checksums.sh`. Your absorption pipeline should verify
   `validation/CHECKSUMS` matches after pull.

### What changed in primalSpring you consume

| Component | Change | Impact |
|-----------|--------|--------|
| `PrimalDeployProfile.bind_flag` | All 13 primals now return `Some(flag)` | Deploy scripts can use `profile.bind_flag` to set bind addresses |
| `GraphMetadata.bind_policy` | `"localhost"` / `"lan"` / `"any"` | Guidestone validates bind intent matches primal defaults |
| `structural_checks` | `fallback="skip"` + `required=false` validation | Foundation/validation graphs pass; inconsistent graphs caught |
| `GraphMetadata.purpose` | `"validation"` / `"foundation"` | Provenance trio (dag, ledger, attribution) required for these |

---

## For sporeGarden/foundation

### What primalSpring provides

1. **Rust IPC validation** via `exp107_foundation_validation`: 8-phase pipeline
   (structural → discovery → health → provenance → storage → compute → ledger →
   attribution) that replaces the shell script `deploy/foundation_validate.sh`
   with type-safe Rust using `CompositionContext`.

2. **Foundation validation graph** at `graphs/compositions/foundation_validation.toml`:
   12-node NUCLEUS with `purpose = "foundation"`, optional nodes (coralReef,
   petalTongue, squirrel) marked `fallback = "skip"`.

3. **Structural validation**: guidestone now enforces that `foundation`-purpose
   graphs include the provenance trio (`dag`, `ledger`, `attribution`).

### Next steps for foundation

- Feed `exp107` output into your `lineage/THREAD_INDEX.toml` as validation evidence
- Map your workload manifests to barraCuda JSON-RPC methods
- Use sweetGrass `braid.create` for attribution after each validated thread

---

## For Spring Teams (hotSpring, healthSpring, wetSpring, neuralSpring, ludoSpring, airSpring, groundSpring)

### What's new since Phase 58

1. **All security gaps closed**: The pen test is done. Your springs inherit the
   hardened primal binaries from genomeBin/plasmidBin without changes.

2. **Bind defaults are localhost**: If you spawn primals in tests or development,
   they now bind `127.0.0.1` by default. Pass `--bind 0.0.0.0` explicitly only
   when you need cross-host access.

3. **Discovery Escalation Hierarchy**: `CompositionContext::discover()` is the
   canonical entry point. It tries 5 tiers automatically. If you were hardcoding
   UDS paths or TCP addresses, switch to `discover()`.

4. **Capability taxonomy**: `"provenance"` is now a routing alias for `"dag"`.
   If you used `by_capability = "provenance"` in your graphs, change to `"dag"`.
   Runtime routing still resolves both, but graphs should be canonical.

### Your rewiring priority (updated from Phase 58)

| Spring | Rewiring Tier | Next Step |
|--------|---------------|-----------|
| ludoSpring | 3→4 | Close remaining `barracuda::` calls, make IPC default |
| healthSpring | 3→4 | Expand `primal-proof` coverage, flip default to IPC |
| hotSpring | 2→3 | Build `src/ipc/` tree, validate barraCuda absorption |
| wetSpring | 2→3 | Route handler compute through ecobin IPC |
| airSpring | 2→3 | Unpause delta, build guideStone against live NUCLEUS |
| neuralSpring | 2→3 | Graduate `ipc_dispatch.rs` to `src/ipc/` tree |
| groundSpring | 1→2 | Expand `ipc.rs` into `src/ipc/` directory |

### Patterns to adopt

1. **`tools/regenerate_checksums.sh`**: Adapt for your spring. Your guideStone
   should verify its own CHECKSUMS manifest.

2. **`exp107` as template**: The 8-phase validation pattern (structural → discovery →
   health → provenance → storage → compute → ledger → attribution) is the canonical
   way to prove your composition works end-to-end.

3. **`GraphMetadata.purpose`**: Tag your validation graphs with `purpose = "validation"`
   so guidestone enforces the provenance trio requirement.

---

## Ecosystem State

| Metric | Value |
|--------|-------|
| Primals | 13/13 BTSP Phase 3 FULL AEAD |
| Bind defaults | 13/13 `127.0.0.1` |
| Security gaps | Zero open (PG-55–PG-59 all RESOLVED) |
| Discovery | 5-tier escalation hierarchy live |
| primalSpring tests | 661 (613 passed + 48 ignored) |
| Experiments | 85 (19 tracks) |
| Deploy graphs | 74 |
| Foundation | exp107 validating 8-phase sediment pipeline |

---

## Key References

| Topic | Location |
|-------|----------|
| Gap registry | `primalSpring/docs/PRIMAL_GAPS.md` |
| IPC method map | `primalSpring/docs/NUCLEUS_IPC_METHOD_MAP.md` |
| Bind address table | `primalSpring/docs/NUCLEUS_IPC_METHOD_MAP.md` (bottom) |
| Foundation guide | `primalSpring/wateringHole/FOUNDATION_ABSORPTION_MAY06_2026.md` |
| Upstream blurbs | `primalSpring/wateringHole/UPSTREAM_BLURBS_PHASE59_MAY06_2026.md` |
| Discovery hierarchy | `primalSpring/ecoPrimal/src/composition/context.rs` |
| Spring audit | `infra/wateringHole/SPRING_NUCLEUS_AUDIT_MAY2026.md` |
| Checksum tool | `primalSpring/tools/regenerate_checksums.sh` |

---

**Zero gaps. Zero blockers. The math is absorbed, the stack is encrypted, the
channels are secure. Rewire, validate, and evolve.**
