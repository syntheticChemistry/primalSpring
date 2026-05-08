# primalSpring Phase 60 — Deep Debt Evolution + Upstream Sovereignty Absorption

**Date**: May 7, 2026
**From**: primalSpring (eastGate)
**Version**: v0.9.25

---

## What Happened

Phase 60 executed a deep debt evolution plan and absorbed the full upstream
sovereignty response from all blurbed primals. primalSpring is now clean, modern,
and ready for downstream handoff.

## Deep Debt Evolution (9 tasks completed)

1. **Binary modularization** — `primalspring_primal` main.rs split from 762L monolith
   into 5 focused modules (dispatch, metrics, lifecycle, niche_setup, ipc_registration).
2. **Probe cache generalized** — `CachedProbeResult` with configurable TTL replaces
   inline ad-hoc caching across all probe paths.
3. **Profile registry centralized** — `deploy/profiles.rs` stores all 9 profiles as
   data, eliminating scattered hardcoded definitions.
4. **blake3 pure Rust** — `Cargo.toml` `default-features = false` + `pure` feature.
   Zero C compilation. ecoBin compliant.
5. **Unsafe annotated** — `env::set_var` in guidestone annotated with `#[expect]`,
   safety comment, and DEBT marker. Threading SeedConfig deferred (library API redesign).
6. **Hardcoded host centralized** — `DEFAULT_TCP_HOST` constant replaces 14 scattered
   `"127.0.0.1"` literals.
7. **Deploy warnings** — `DeployWarning` enum with 3 variants replaces inline string
   construction for graph validation warnings.
8. **Experiment refactoring** — exp096 (1352L→547L) and exp105 (1327L→510L) extracted
   into phase modules. Both under 800L threshold.
9. **Method string validation** — 211/211 source strings, 369 registered methods,
   353 graph refs checked with 0 primal drift.

## Upstream Sovereignty Absorption (14/14 gaps resolved)

All primals responded to our audit blurbs and shipped fixes:

| Primal | Gaps | Resolution |
|--------|------|------------|
| petalTongue | PT-1,3,4,5 | `--docroot` ServeDir, WebServeConfig, `--ipc` dual-port, `--workers` wired |
| NestGate | NG-1→NG-4 | Content-addressed storage (8 methods), manifests, blob visibility, streaming docs |
| biomeOS | RP-1→RP-5 | 6 rootpulse graphs realigned, standalone executor, type docs |
| LoamSpine | RP-2,3,5 | spine.create documented, hex strings, entry signing contract |
| BearDog | RP-1,5 | crypto.sign contract fixed, crypto.did_from_key added |
| rhizoCrypt | PG-60 | Readiness gate (-32002 not-ready on cold connect) |
| toadStool | PG-62 | health.liveness fast-path, timeout docs |
| barraCuda | stats.entropy, shaders | Both resolved (Sprint 50, Sprint 43). Rewire guide published |

## Registry Evolution

| Metric | Before | After |
|--------|--------|-------|
| Registered methods | 290 | 369 |
| Source strings validated | 208/208 | 208/208 |
| Graph refs checked | 217 drift | 0 primal drift (91 spring-domain advisory) |
| Domains | ~35 | 50+ |

New domains: `content.*`, `viz.*`, `beacon.*`, `lineage.*`, `tls.*`, `math.*`,
`rng.*`, `ionic.*`, `tools.*`. Key additions: `crypto.did_from_key`,
`network.beacon_exchange`, `storage.list_blobs`/`storage.blob_exists`.

## Graph Validator Evolution

`tools/check_graph_methods.sh` rewritten with two-tier output:
- **DRIFT** — primal methods missing from registry (errors, exit 1)
- **SPRING** — spring-domain capabilities in cell graphs (advisory only)
- `--strict` mode treats spring-domain as errors too
- `--fix` groups unregistered methods by domain for easy triage

## Validation

- **666 tests** (618 passed + 48 ignored), 0 failed
- **0 clippy warnings** (pedantic + nursery)
- **cargo check --workspace** clean
- **208/208** source method strings in registry
- **353** graph refs, 262 matched, 91 spring-domain, **0 primal drift**
- **CHECKSUMS** regenerated (18 files)

## Remaining Upstream Debt (P2/P3)

| Priority | Item | Owner |
|----------|------|-------|
| P2 | PT-13: NestGate backend for petalTongue web mode | petalTongue |
| P2 | PT-09: BTSP Phase 2 enforcement in petalTongue | petalTongue |
| P3 | SD-02/SD-03: sourDough musl + genomeBin signing | sourDough |
| P3 | Squirrel E2E inference parity | Squirrel + neuralSpring |
| P3 | coralReef transitive libc (mio→rustix) | coralReef |
| P3 | barraCuda 4 stateful API gaps (ESN, nautilus, ODE, SimpleMlp) | barraCuda |

## Files Changed (18 files, +450/-2942 lines)

- `config/capability_registry.toml` — +233 lines (76 new methods + 12 new domains)
- `docs/PRIMAL_GAPS.md` — 14 gaps marked RESOLVED, barraCuda updated
- `ecoPrimal/src/bin/primalspring_primal/main.rs` — 762L→41L (5 modules extracted)
- `experiments/exp096*/src/main.rs` — 1352L→547L (phases.rs extracted)
- `experiments/exp105*/src/main.rs` — 1327L→510L (phases.rs extracted)
- `tools/check_graph_methods.sh` — rewritten with spring-domain exclusion
- Various: deploy/profiles.rs, ipc/probes.rs, niche.rs, checksums.rs, tolerances/mod.rs
