# primalSpring Phase 43 Final Handoff — April 15, 2026

**From**: primalSpring (coordination + composition spring)
**To**: Primal teams, spring teams, biomeOS, garden teams
**License**: AGPL-3.0-or-later

---

## Executive Summary

NUCLEUS composition is **fully validated**. All critical upstream gaps are resolved.
Downstream springs and gardens can begin absorbing composition patterns immediately.

| Metric | Value |
|--------|-------|
| exp091 (routing matrix) | **12/12 ALL PASS** |
| exp094 (composition parity) | **19/19 ALL PASS** |
| exp096 (cross-arch Pixel) | **14/15 PASS** (HSM pending) |
| biomeOS composition gaps | **7/7 RESOLVED** |
| plasmidBin primals | **13/13 ecoBin compliant** |
| Deploy graphs | **119 total** (42 biomeOS + 77 primalSpring) |
| Fragments | **6 canonical** (tower, node, nest, nucleus, meta_tier, provenance_trio) |

---

## Part 1: Primal Team Handoffs

### BearDog — Security + Crypto

**What works**:
- BTSP Phase 1-2 stable across 12/12 primals
- Protocol auto-detection on TCP: first-byte peek distinguishes JSON-RPC vs BTSP
- `genetic.*` RPCs: mito-beacon derivation, nuclear genesis, lineage proofs, entropy mixing
- Ed25519 keypair generation on both x86_64 and aarch64 (software backend)
- BTSP Phase 3 types defined (ChaCha20-Poly1305, HKDF-SHA256)

**What needs evolution**:
- **BTSP Phase 3 server-side negotiation** — client types exist, server needs implementation
- **HSM/Titan M2 integration** — `crypto.generate_keypair` software backend works, hardware not wired
- **Nuclear genetics copy resistance enforcement** — ensure `genetic.nuclear_spawn_generation` always creates fresh mixed generation
- **Ionic bond `crypto.sign_contract`** — for cross-family metered capability sharing

**Key pattern to maintain**: First-byte peek (`0x7B` = JSON-RPC, else BTSP) on all TCP and UDS listeners. biomeOS forwards plain JSON-RPC for local composition.

### Songbird — Discovery + Networking

**What works**:
- BirdSong beacon encrypt/decrypt round-trip validated cross-architecture
- HTTP POST JSON-RPC transport; `tcp_rpc_multi_protocol` auto-detects
- Federation port `--port 8080` for covalent mesh

**What needs evolution**:
- **Mito-beacon metadata in BirdSong beacons** — dark forest discovery
- **STUN/NAT with mito-beacon credentials** (not nuclear) — discovery must never expose authorization
- **Content distribution federation** — natural seeder/leecher for `content_distribution_federation.toml`

### NestGate — Storage + Federation

**What works**:
- Nest Atomic fully validated (8/8 gates); ring-free (ureq + rustls-rustcrypto)
- **First-byte peek on UDS** (`f1e1da78d`) — same pattern as BearDog TCP. JSON-RPC auto-detected, BTSP preserved for remote connections
- Storage round-trips work through biomeOS Neural API routing
- Cross-nest round-trips validated (exp094)

**What needs evolution**:
- **Encrypted-at-rest secrets** alongside BearDog for zero-knowledge auth
- **Content distribution**: `ContentManifest` + BLAKE3 content addressing
- **Federation replication with nuclear lineage verification** — not just mito-beacon

**Key lesson**: NestGate's BTSP enforcement on UDS was blocking biomeOS composition. The first-byte peek fix is now the required pattern for any primal accepting socket connections.

### biomeOS — Orchestration

**What works**:
- Neural API routes capabilities to correct primals via `CapabilityTranslationRegistry`
- `--family-id` correctly propagated through translation defaults/config loading (`ad4d4490`)
- Graph executor reports per-node success/failure with `completed_nodes`/`failed_nodes`
- TCP cascade for cross-arch deployment (Pixel aarch64 via `--tcp-only`)
- Graph env var substitution (two-pass resolution)

**What needs evolution**:
- **Graph-level genetics** — graphs should declare their required genetics tier
- **Tick-loop scheduling** — 60Hz composition budget for game/real-time graphs
- **Deploy class resolution** — auto-select tower/node/nest/nucleus based on graph fragments

**Key lesson**: `family_id` must be threaded explicitly through all internal components. Using `std::env::set_var` violates `#![forbid(unsafe_code)]`.

### Provenance Trio — rhizoCrypt + loamSpine + sweetGrass

**What works**: All three primals IPC-stable (TCP_NODELAY + flush-on-write). Storage round-trips, DAG operations, and attribution validated through exp094.

**What needs evolution**: Federated provenance with nuclear lineage verification. Cross-site Merkle comparison.

### Squirrel — AI + MCP

**What works**: AI provider chain (Squirrel → OpenAI → Songbird → Ollama). 8 MCP tools for AI-driven ecosystem introspection.

**What needs evolution**: Genetics-aware routing for `ai.query` to external providers. Content curation via BLAKE3 manifests.

### ToadStool + barraCuda + coralReef — Compute

**What works**: Node Atomic validated (5/5 gates). Pipeline scheduling (`compute.dispatch.pipeline.submit`). CPU-shader default-on. SovereignDevice IPC fallback.

**What needs evolution**: Cross-arch compute dispatch routing. Lineage proofs on compute results.

---

## Part 2: Spring Team Handoffs

### What Springs Can Absorb Now

1. **Full NUCLEUS from plasmidBin** — 13/13 ecoBin primals, `nucleus_launcher.sh` supports `--composition tower|node|nest|nucleus|full`
2. **Proto-nucleate graphs** — `graphs/downstream/{yourspring}_*_proto_nucleate.toml` define your target composition
3. **CompositionContext** — capability-keyed IPC client with `from_live_discovery_with_fallback()`
4. **validate_parity / validate_parity_vec** — compare local baselines against primal IPC results
5. **exp095 template** — copy-and-rename starter experiment for any spring
6. **Fragment composition** — `[graph.metadata] fragments = ["tower_atomic", "node_atomic", ...]`
7. **Bonding patterns** — covalent (same family), ionic (cross-family), metallic (shared fleet)

### The Maturity Ladder

```
Level 1: Python baseline        → ground truth from peer-reviewed science
Level 2: Rust validation        → faithful port, tolerance-gated
Level 3: barraCuda CPU          → same math via primal IPC (WGSL CPU fallback)
Level 4: barraCuda GPU          → sovereign shader execution
Level 5: Primal composition     → ALL math via NUCLEUS primals (IPC only)
Level 6: Deploy graph validated → proto-nucleate satisfied via biomeOS
```

At Level 5+, the spring's binary becomes fossil record. The graph IS the product.

### Per-Spring Status

| Spring | Stage | Deploy Graphs | What Unblocks Next |
|--------|-------|---------------|--------------------|
| hotSpring | composing | 1 (QCD deploy) | BTSP Phase 3 server + ionic runtime |
| neuralSpring | composing | 1 (inference) | Squirrel `register_provider` + coralReef multi-stage |
| wetSpring | composing | 7 (deploy + workflows) | Trio IPC stability (RESOLVED) + `capability.resolve` |
| healthSpring | composing | 7 (deploy + workflows) | BearDog BTSP server + ionic bond runtime |
| airSpring | composing | 5 (deploy + pipelines) | TensorSession + batched ODE upstream |
| groundSpring | composing | 6 (deploy + validation) | RAWR GPU kernel + eigenvector GPU path |
| ludoSpring | composing | (via primalSpring) | rhizoCrypt/loamSpine UDS stability (RESOLVED) |

### What Springs Should Hand Back

When you discover a gap during absorption:
1. Document it in primalSpring `docs/PRIMAL_GAPS.md`
2. Propose the JSON-RPC method signature
3. Build a validation experiment or graph
4. Submit via PR — primalSpring triages to the responsible primal

---

## Part 3: Composition Patterns Reference

### Protocol Auto-Detection (REQUIRED for all primals)

```
First byte 0x7B ('{') → JSON-RPC (bypass BTSP)
Any other byte        → BTSP binary handshake
```

Implemented in BearDog (TCP), NestGate (UDS). All primals accepting socket connections must implement this pattern.

### biomeOS Neural API Deployment

```bash
biomeos neural-api --tcp-only --port 9000 --graphs-dir ./graphs --family-id nucleus01
```

### Graph Fragment Composition

```toml
[graph.metadata]
fragments = ["tower_atomic", "node_atomic", "nest_atomic"]
bonding_primary = "covalent"
genetics_tier = "nuclear"
```

### Three-Tier Genetics

| Tier | Name | For | Properties |
|------|------|-----|------------|
| 1 | Mito-Beacon | Discovery, NAT | Inherited, cloneable, multiple per system |
| 2 | Nuclear | Permissions, auth | Fresh per generation, !Clone, non-fungible |
| 3 | Tags | Open channels | From legacy FAMILY_SEED, no key material |

### Socket Convention

```
$XDG_RUNTIME_DIR/biomeos/{primal}-{family_id}.sock
```

---

## Part 4: Key Documents

| Document | Location | Purpose |
|----------|----------|---------|
| Gap registry | `docs/PRIMAL_GAPS.md` | Per-primal gap inventory |
| Composition guidance | `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` | How springs compose primals |
| Upstream cross-talk | `wateringHole/UPSTREAM_CROSSTALK_AND_DOWNSTREAM_ABSORPTION.md` | Protocol auto-detection, socket naming, absorption checklist |
| Leverage guide | `wateringHole/PRIMALSPRING_ECOSYSTEM_LEVERAGE_GUIDE.md` | Library patterns, evolution cycles |
| Spring alignment | `wateringHole/NUCLEUS_SPRING_ALIGNMENT.md` | Per-spring atomic alignment matrix |
| biomeOS gaps | `wateringHole/BIOMEOS_COMPOSITION_GAPS_APR14_2026.md` | All 7 gaps documented and RESOLVED |
| Proto-nucleates | `graphs/downstream/` | Per-spring target composition graphs |
| Fragments | `graphs/fragments/` | Canonical NUCLEUS building blocks |

---

**License**: AGPL-3.0-or-later
