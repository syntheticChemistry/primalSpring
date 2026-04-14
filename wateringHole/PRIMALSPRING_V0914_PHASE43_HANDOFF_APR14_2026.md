# primalSpring v0.9.14 Phase 43 Handoff — April 14, 2026

**From**: primalSpring (coordination + composition spring)
**To**: Primal teams, spring teams, biomeOS
**License**: AGPL-3.0-or-later

---

## What This Covers

Phase 42–43 introduced multi-tier genetics identity, BTSP Phase 3 encrypted channels,
cross-architecture deployment via biomeOS Neural API, ionic bond protocol, and content
distribution federation. This handoff summarizes composition gaps, evolution paths, and
patterns relevant to each team.

---

## Primal Handoffs

### BearDog (Security + Crypto)

**What primalSpring validated**:
- BTSP Phase 1 (FAMILY_SEED auth via HMAC-SHA256) and Phase 2 (secure-by-default cascade across 12/12 primals) both stable
- Phase 3 types defined: `Phase3Cipher` (ChaCha20-Poly1305), `NegotiateRequest`/`NegotiateResponse`, `SessionKeys` with HKDF-SHA256 key derivation
- `BtspEnforcer` now uses explicit deny semantics per `TrustModel` (NuclearLineage, MitoBeaconFamily, TagOpen)
- Cross-arch: BearDog alive on Pixel (aarch64 + GrapheneOS), HSM (Titan M2) detected but not yet probed via `crypto.hsm_*`
- `genetic.*` RPCs: Mito-beacon, nuclear lineage, proof, and entropy mixing test scaffolding in place (integration tests `#[ignore]` pending live BearDog)

**Evolution path**:
- Implement BTSP Phase 3 negotiation server-side (client types exist in primalSpring)
- Wire `genetic.mito_beacon_derive`, `genetic.nuclear_spawn_generation`, `genetic.lineage_proof` RPCs
- HSM integration: `crypto.hsm_probe`, `crypto.hsm_sign` for Titan M2 / TPM2
- Nuclear genetics copy resistance: ensure `genetic.nuclear_spawn_generation` always creates a new mixed generation, never clones

### Songbird (Discovery + Networking)

**What primalSpring validated**:
- BirdSong beacon exchange works cross-architecture (encrypt/decrypt round-trip via exp063)
- HTTP transport confirmed: Songbird uses HTTP POST JSON-RPC (not raw TCP)
- `tcp_rpc_multi_protocol` auto-fallback handles this transparently from primalSpring
- Federation port `--port 8080` for covalent mesh remains TCP opt-in only

**Evolution path**:
- Mito-beacon discovery integration: BirdSong beacons should carry mito-beacon tier metadata for dark forest discovery
- STUN/NAT traversal with mito-beacon credentials (not nuclear) — discovery must never expose authorization material
- Content distribution federation: Songbird is the natural seeder/leecher network layer for the `content_distribution_federation.toml` graph

### NestGate (Storage + Federation)

**What primalSpring validated**:
- Nest Atomic fully validated (8/8 gates), NG-08 ring dependency RESOLVED (reqwest → ureq + rustls-rustcrypto)
- Storage operations work through Neural API `capability.call` routing
- `--socket` CLI flag wired through dispatch for UDS-first deployment

**Evolution path**:
- Encrypted-at-rest secrets: NestGate should hold encrypted secrets alongside BearDog for zero-knowledge auth
- Content distribution: `ContentManifest` + BLAKE3 content addressing types defined in `bonding/content_distribution.rs`
- Federation replication with nuclear lineage verification (not just mito-beacon) for permission-bearing data

### Squirrel (AI + MCP)

**What primalSpring validated**:
- AI provider chain working: Squirrel → OpenAI adapter → Songbird → Ollama → tinyllama-cpu
- Neural API bridge: `discovery.rs` accepts `primary_endpoint` field, strips `unix://` prefix
- 8 MCP tools via `mcp.tools.list` for AI-driven ecosystem introspection

**Evolution path**:
- AI chain with genetics-aware routing: Squirrel should respect mito-beacon trust tiers when routing `ai.query` to external providers
- Content curation: Squirrel as content distribution arbiter using BLAKE3 manifests

### ToadStool (Compute Substrate)

**What primalSpring validated**:
- Node Atomic fully validated (5/5 gates), JSON-RPC socket separated from tarpc
- `compute-default.jsonrpc.sock` preferred for `capability.call` forwarding

**Evolution path**:
- Compute with lineage: attach nuclear lineage proof to compute results for provenance chains
- Cross-arch compute dispatch: biomeOS should route compute to appropriate arch (x86_64 for GPU, aarch64 for edge)

### coralReef + barraCuda (GPU + Math)

**What primalSpring validated**:
- BTSP Phase 2 cascade complete (coralReef Iter 78, barraCuda Sprint 39)
- Class 2 GPU/Vulkan delegation to Node Atomic resolved

**Evolution path**:
- Pure Rust WGSL shader composition model confirmed as the path forward
- No genetics changes needed — these are pure compute, delegating all auth to BearDog

### Provenance Trio (rhizoCrypt + loamSpine + sweetGrass)

**What primalSpring validated**:
- All three alive on UDS, BTSP Phase 2 wired
- RootPulse workflows validated structurally (exp020–022)

**Evolution path**:
- Attach nuclear lineage proofs to provenance commits (currently plain FAMILY_ID)
- Content distribution provenance: link BLAKE3 content manifests to provenance DAG

---

## Spring Team Handoffs

### NUCLEUS Deployment Patterns (for all springs)

**The canonical deployment path** — valid for every spring:

1. Write a `*_proto_nucleate.toml` graph (use `graphs/downstream/` examples or copy `exp094`)
2. biomeOS deploys it: `biomeos neural-api --graphs-dir ./graphs`
3. Your spring code calls `capability.call("domain.method")` via JSON-RPC to the Neural API
4. Neural API routes to the correct primal — your code never imports primal crates

**Cross-architecture**:
- Use `--tcp-only` for Android/Windows deployments
- `tcp_rpc_multi_protocol` tries raw TCP then HTTP POST — transport agnostic
- `build_ecosystem_musl.sh` cross-compiles all primals for x86_64 and aarch64

**Genetics in graphs**:
- `[nodes.operation.environment]` should carry `FAMILY_ID` and `FAMILY_SEED` for BTSP Phase 1
- Nuclear lineage (Phase 2+) comes from BearDog at runtime, not from graph env
- Mito-beacon credentials for discovery are separate from nuclear auth — safe to share in federation configs

### Specific Spring Guidance

| Spring | What primalSpring learned | Action |
|--------|--------------------------|--------|
| hotSpring | QCD pipeline graph validated structurally | Wire live BearDog crypto signing into df64 precision provenance |
| wetSpring | 11-node science NUCLEUS graph pattern works | Ensure `genetic.lineage_proof` attached to biology data exports |
| neuralSpring | Inference pipeline graph works, WGSL shader composition confirmed | Route `inference.*` through Squirrel with mito-beacon trust gating |
| healthSpring | Dual-tower enclave graph with ionic bond | Strict nuclear lineage for clinical data — never share nuclear across trust boundaries |
| ludoSpring | Storytelling composition (exp088) works E2E | Game session genetics: mito-beacon for guild membership, nuclear for save data ownership |
| airSpring | 41 NUCLEUS niche capabilities | Cross-spring ecology pipeline validated (exp080) with biomeOS routing |
| groundSpring | Typed errors, ValidationSink patterns | Foundation for all experiment validation; no genetics-specific changes |

---

## biomeOS Handoff

See dedicated blurb: the biomeOS-specific composition gaps and evolution path are documented
separately for the biomeOS team below.

### Upstream Gaps (blocking primalSpring 9/9 cross-arch)

1. **TCP endpoint propagation in NeuralRouter**: When primals start with `--tcp-only` (or
   on platforms without UDS), NeuralRouter still registers Unix socket endpoints. It needs
   to detect TCP-bound primals and register their TCP endpoints for `capability.call` routing.

2. **Graph environment variable substitution**: `[nodes.operation.environment]` sections in
   TOML graphs contain `${FAMILY_ID}`, `${XDG_RUNTIME_DIR}` etc. These are passed literally
   to child processes instead of being substituted from the execution context.

3. **bootstrap.rs environment inheritance**: Patched locally — `BIOMEOS_PLASMID_BIN_DIR`,
   `ECOPRIMALS_PLASMID_BIN`, `XDG_RUNTIME_DIR`, `FAMILY_SEED` need to flow from process
   env into `ExecutionContext` for the graph executor. The patch was pushed upstream.

### Composition Patterns Validated

- `tower_atomic_bootstrap.toml` works on both x86_64 (UDS) and aarch64 (TCP)
- `biomeos neural-api --tcp-only --port 9000` is the cross-platform entry point
- `primal_launch_profiles.toml` correctly wires per-primal CLI args and env
- SocketNucleation provides deterministic socket paths for inter-primal communication
- biomeOS as composition substrate works — manual primal wiring is deprecated

### Evolution for biomeOS

- NeuralRouter should support hybrid endpoint registration (UDS + TCP per primal)
- Graph env substitution engine (similar to shell variable expansion)
- `--tcp-only` should cascade to child primal spawn args automatically
- Capability routing through TCP endpoints needs the same resilience as UDS (circuit breaker, retry)

---

**License**: AGPL-3.0-or-later
