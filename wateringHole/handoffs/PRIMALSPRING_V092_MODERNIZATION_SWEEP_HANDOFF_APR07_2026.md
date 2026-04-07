# primalSpring v0.9.2 — Modernization Sweep Handoff

**Date**: April 7, 2026  
**Phase**: 25  
**Version**: 0.9.2  
**Scope**: Pattern cleanup, graph format unification, capability naming, health probe deprecation, Tower Atomic LAN validation

---

## Summary

primalSpring Phase 25 is a comprehensive modernization sweep that cleans all legacy patterns, unifies graph formats with biomeOS, and validates the Tower Atomic composition over LAN. This handoff supersedes the v0.8.0 Mar 29 handoffs.

---

## What Changed

### 1. Capability Naming Cleanup (17 files)

All stale capability method names corrected to canonical dotted form:

| Old | Canonical | Files |
|-----|-----------|-------|
| `dag.dehydrate` | `dag.dehydration.trigger` | `capability_registry.toml`, `niche.rs`, 17 graph TOMLs |
| `dag.create_session` | `dag.session.create` | `primalspring_deploy.toml`, `nucleus_complete.toml` |
| `dag.append_event` | `dag.event.append` | `primalspring_deploy.toml`, `nucleus_complete.toml` |
| `dag.merkle_root` | `dag.merkle.root` | `primalspring_deploy.toml`, `nucleus_complete.toml` |
| `commit.session` | `session.commit` | `primalspring_deploy.toml`, `nucleus_complete.toml`, `continuous_tick.toml`, `data_federation_cross_site.toml` |
| `commit.entry` | `entry.append` | `primalspring_deploy.toml`, `nucleus_complete.toml` |
| `dag.health` / `commit.health` | `health.liveness` | `continuous_tick.toml` |

### 2. Graph Format Unification (NA-016 Resolved)

- **Parser upgraded**: `ecoPrimal/src/deploy/mod.rs` now accepts `[[graph.node]]` (legacy), `[[graph.nodes]]` (biomeOS-native), and top-level `[[nodes]]` via `serde(alias)` + runtime merge.
- **`GraphMeta`** gains optional `id: Option<String>` field for biomeOS `GraphId` compatibility.
- **87+ graphs migrated** from `[[graph.node]]` → `[[graph.nodes]]`.
- **5 multi-node graphs** converted from `[[nodes]]` → `[[graph.nodes]]` with `[graph.nodes.*]` subsections.
- `nest-deploy.toml` v4.0 established as the **gold standard** graph.

### 3. HTTP Health Probe Deprecated (NA-009 Resolved)

- `http_health_probe` in `ecoPrimal/src/ipc/tcp.rs` marked `#[deprecated]` — Songbird no longer exposes HTTP `/health`; Tower Atomic owns all HTTP.
- 4 experiments updated to use `tcp_rpc` with `health.liveness`:
  - exp073 (LAN covalent mesh)
  - exp074 (cross-gate health)
  - exp076 (cross-gate neural routing)
  - exp081 (deployment matrix sweep)

### 4. `nest-deploy.toml` v4.0 Gold Standard

- `[graph]` gains `id = "nest-deploy"`, version bumped to `4.0.0`.
- Songbird capabilities expanded: `mesh.init`, `mesh.auto_discover`, `mesh.peers`.
- New **Phase 5: HTTPS Validation** — `validate_https` node (order 5) calls `http.get` to `https://ifconfig.me/ip` through Tower Atomic end-to-end.
- Phase 6: `validate_nest` renumbered, depends on `validate_https`.

### 5. exp090 Tower Atomic LAN Probe (New)

Validates Tower Atomic's full LAN capability set:
- Local Tower health (BearDog + Songbird health.liveness)
- BirdSong mesh discovery (`mesh.init`, `mesh.auto_discover`, `mesh.peers`)
- Peer capability enumeration
- HTTPS through Tower Atomic (via NeuralBridge `capability.call` → Songbird `http.get`)
- STUN/NAT detection (`stun.get_public_address`)
- Topology summary report

### 6. exp073 Covalent Bonding Modernized

Enhanced with three new validation functions:
- **Neural API routing**: Validates `capability.call` routing through biomeOS
- **Genetic lineage**: Verifies `FAMILY_ID` propagation via BearDog `health.check` details
- **HTTPS through Tower**: End-to-end HTTPS validation via Tower Atomic

### 7. Basement HPC Graph Aligned

`basement_hpc_covalent.toml` updated:
- All capability lists use canonical dotted names
- HTTPS validation phase inserted between `gate_validate` and `announce_capabilities`
- `announce_capabilities` updated with specific capabilities (`compute.submit`, `storage.fetch_external`, `ai.query`)

### 8. Documentation Cleanup

- `SHOWCASE_MINING_REPORT.md`: "HTTP REST" → JSON-RPC 2.0 serialization
- `exp052`: doc comments updated to reflect JSON-RPC-only IPC
- `PRIMALSPRING_COMPOSITION_GUIDANCE.md`: `[[graph.node]]` → `[[graph.nodes]]`
- `CROSS_SPRING_EVOLUTION.md`: NA-009 and NA-016 marked RESOLVED with detailed notes; "What Changed (April 7)" section added

---

## Cross-Primal Alignment

| Primal | Update Pulled | Key Changes |
|--------|--------------|-------------|
| rhizoCrypt | v0.14.0 | `dag.dehydration.trigger` is the canonical method; alias still works |
| loamSpine | latest | `session.commit`, `entry.append` canonical |
| sweetGrass | latest | Attribution PROV-O stable |
| BearDog | latest | BD-01: `crypto.verify_ed25519` gains `encoding` hint |
| Songbird | rebased | TLS 1.3 ClientHello fix (CSPRNG `client_random`); SB-03 sled→NestGate migration in progress |
| NestGate | latest | Storage delegation pattern confirmed |
| Squirrel | latest | `ai.query`, `tool.execute`, `context.create` stable |
| ToadStool | latest | `compute.submit` canonical |
| biomeOS | latest | `capability.call` routing, `[[graph.nodes]]` native format |

---

## Resolved Gaps

| ID | Component | Resolution |
|----|-----------|------------|
| NA-009 | rhizoCrypt | `dag.dehydrate` → `dag.dehydration.trigger` everywhere |
| NA-016 | primalSpring / biomeOS | Graph format divergence eliminated; parser unified |

---

## Metrics

| Metric | Before | After |
|--------|--------|-------|
| Tests | 403 | 404 |
| Experiments | 67 (14 tracks) | 69 (15 tracks) |
| Deploy graphs | 89 | 92 |
| Graph format | mixed (`[[graph.node]]` / `[[graph.nodes]]` / `[[nodes]]`) | unified `[[graph.nodes]]` |
| `http_health_probe` callers | 4 experiments | 0 (deprecated) |
| Capability naming inconsistencies | 6 stale method names | 0 |

---

## Action Items for Downstream

1. **Springs**: Absorb `[[graph.nodes]]` format in any local graph files. Use `nest-deploy.toml` as template.
2. **Primals**: Review `CROSS_SPRING_EVOLUTION.md` for your primal's action items.
3. **biomeOS**: No changes needed — primalSpring aligned to biomeOS-native format.
4. **sporeGarden**: Products can reference `nest-deploy.toml` v4.0 as the deployment template.
