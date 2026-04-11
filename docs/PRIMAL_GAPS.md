# Primal Gap Registry

Structured inventory of known gaps per primal that block or degrade composable deployments.
Each entry links to the composition that exposes it and proposes a fix path.

> **Scope**: NUCLEUS primals only (the 10 core primals + 3 compute/ecosystem primals).
> Downstream springs and gardens (ludoSpring, esotericWebb, etc.) own their own debt
> and are NOT tracked here. See `graphs/downstream/` for proto-nucleate patterns.
> Springs/gardens do NOT have binaries in plasmidBin — only primals do.
>
> **Last updated**: 2026-04-11 — **PORTABILITY DEBT AUDIT INITIATED**.
> All 10 primals running UDS-only. `ss -tlnp | grep plasmidBin` returns **empty**.
> 7 primals modified (BearDog, Songbird, Squirrel, ToadStool, rhizoCrypt, sweetGrass, loamSpine)
> to make TCP opt-in via explicit `--port` flag. Same biomeOS graph deploys on any hardware/arch.
> TCP is opt-in only for Songbird federation (`--port 8080` enables covalent mesh).
>
> **Live validation (April 10 — NUCLEUS polish, session 2)**:
> - TCP ports: **0** (was 12 across 8 primals)
> - UDS sockets: **25** active in `/run/user/$UID/biomeos/`
> - C1-C7 compositions: **37/38 (97%)** — single partial: C2 `ai.query` (Ollama provider config)
> - **13/13 critical experiments ALL PASS** (exp001/002/003/004/051/069/075/077/079/089/091/092/093)
> - All 10 primals healthy over UDS (`health.liveness` OK), all `ALIVE` in launcher status
>
> **Squirrel AI provider chain (April 10)**:
> - Squirrel rebuilt with `deprecated-adapters` feature + 3 fixes:
>   1. `discovery.rs`: Accept biomeOS `primary_endpoint` field (not just `primary_socket`),
>      strip `unix://` prefix — Neural API → Songbird discovery now works.
>   2. `router.rs`: Don't register broken `local-ai` provider with HTTP URL as socket path;
>      `UniversalAiAdapter` only works with Unix sockets.
>   3. `openai.rs`: Read `OPENAI_DEFAULT_MODEL` env var (was hardcoded `gpt-4`);
>      handle OpenAI-compatible error responses before parsing as success.
> - Provider chain: Squirrel → OpenAI adapter → Songbird `http.request` → Ollama `/v1/` → tinyllama-cpu.
> - Created `tinyllama-cpu` Ollama model alias with `num_gpu=0` for CUDA-free inference.
> - C2 `ai.query` now passes (was the only C1–C7 failure).
>
> **Socket resolution evolution (April 10)**:
> - `resolve_primal_socket_with()` now has 4-tier fallback: env var → domain alias
>   (`.jsonrpc.sock` preferred) → `{primal}-{family}.sock` → `{primal}.sock` (plain).
> - Primals without `--socket` (loamSpine, sweetGrass, petalTongue) now reachable
>   via plain socket fallback — biomeOS finds `loamspine.sock` when
>   `loamspine-default.sock` doesn't exist.
> - ToadStool JSON-RPC forwarding fixed: prefers `compute-default.jsonrpc.sock`
>   over tarpc `compute-default.sock` for `capability.call`.
> - `NeuralBridge::discover()` now checks both `neural-api-{family}.sock` and
>   `biomeos-{family}.sock` — experiments find biomeOS regardless of socket naming.
>
> **biomeOS registry routing fix (April 10)**:
> - Root cause: `defaults.rs`, `mod.rs` (`load_from_config`), and `translation_startup.rs`
>   called `biomeos_core::family_discovery::get_family_id()` which derives a hex hash from
>   `.family.seed`, producing socket paths like `beardog-8ff3b864a4bc589a.sock`. But primals
>   run with `BIOMEOS_FAMILY_ID=default`, so sockets are `beardog-default.sock`.
> - Fix: Threaded `family_id: &str` through `load_defaults_into()`, `load_from_config()`,
>   and `load_defaults()` APIs. `NeuralApiServer` passes `self.family_id` ("default") instead.
> - Socket alias: `resolve_primal_socket_with()` now maps `toadstool→compute` and
>   `nestgate→storage` domain sockets when the domain-based path exists on disk.
> - Verified: `capability.call("crypto.encrypt")` → BearDog → success.
>   `capability.call("storage.put")` → NestGate → success.
>
> **NUCLEUS deployment patterns (April 10)**:
> - ToadStool: JSON-RPC socket separated from tarpc (`compute.jsonrpc.sock` vs `compute.sock`),
>   `--socket` CLI flag wired through to `run_server_main`, legacy symlinks for both protocols.
> - NestGate: `--socket` CLI flag added to `Commands::Server`, wired through dispatch to set
>   `NESTGATE_SOCKET` env var, feeding into `SocketConfig::from_environment()` tier-1 resolution.
> - primalSpring: BTSP client handshake module (`btsp_handshake.rs`) with HKDF-SHA256 key
>   derivation + HMAC-SHA256 challenge response matching BearDog's `crypto.rs`. Auto-detection
>   in `Transport::connect()` via `security_mode_from_env()`. Both rebuilt to plasmidBin.
>
> **BTSP Phase 2 ECOSYSTEM CASCADE (April 9–10)**: **11/13** primals enforce handshake on UDS
> accept. Songbird Wave 133, ToadStool S198, barraCuda Sprint 39 ↑, coralReef Iter 78 ↑,
> rhizoCrypt S31, loamSpine, sweetGrass all wired. petalTongue Phase 1 COMPLETE (Phase 2 stub).
> skunkBat Phase 1 only.
> coralReef Phase 2 COMPLETE ↑ (Iter 78 — real BearDog RPC, degraded when absent). skunkBat
> new JSON-RPC IPC server + Phase 1 COMPLETE. **BearDog is the sole handshake provider,
> not a consumer — its status as "already complete" is by design.**
>
> **Capability Wire Standard v1.0 (April 8)**: Convergence target defined. Flat `methods`
> array + `primal` + `version` MUST fields. 8/13 primals at L2+ (BearDog L2, Songbird L3,
> NestGate L3, ToadStool L3, Squirrel L2, rhizoCrypt L3, loamSpine L2/L3, sweetGrass L3).
> barraCuda L2. petalTongue L2/L3. coralReef L2 ↑. skunkBat L2 ↑. sourDough/bingoCube: NONE (CLI tools).
>
> **plasmidBin (April 10)**: ~~`doctor.sh --quick` reports 9/11 DYNAMIC~~ **RESOLVED** —
> full `--target x86_64-unknown-linux-musl` rebuild. 12/12 static, stripped, ecoBin compliant.
>
> **Trio witness evolution (April 7)**: `WireAttestationRef` → `WireWitnessRef`.
> Self-describing `kind`/`encoding`/`algorithm`/`tier`/`context` fields. Trio harvested
> to plasmidBin (glibc → musl). See `wateringHole/handoffs/PRIMALSPRING_TRIO_WITNESS_HARVEST_HANDOFF_APR07_2026.md`.

---

## Portability Debt Registry (April 11, 2026)

Cross-cutting non-portable dependencies that violate the ecoBin "pure Rust, universal
binary" principle. Organized by severity class. Each class follows the same resolution
pattern: **identify → centralize in one primal → delegate via IPC → ban in consumers**.

### Class 1: C Crypto — SOLVED (Tower Atomic Delegation)

`ring` (C/ASM crypto) blocked musl cross-compile and single-target builds. Solution:
BearDog provides RustCrypto in-process (pure Rust). Other primals delegate crypto
to BearDog via JSON-RPC IPC. `deny.toml` bans `ring`, `openssl`, `aws-lc-sys`
ecosystem-wide. This established the delegation pattern.

| Primal | Had | Replaced With | Pattern |
|--------|-----|---------------|---------|
| Songbird | `ring` (C/ASM TLS) | `rustls_rustcrypto` + BearDog IPC | Tower Atomic delegation |
| NestGate | `aws-lc-rs` / `ring` | `ureq` + `rustls-rustcrypto` (pure Rust); system `curl` (installer) | `reqwest` → `ureq` migration |
| barraCuda | Banned in deny.toml | Never had — preemptive ban | Policy |
| Squirrel | `libloading` (FFI) | Removed (alpha.46) | Direct elimination |

**Class 1 leak — NestGate NG-08: RESOLVED** (April 11, 2026). `ring` v0.17.14 was present
via `rustls` → `reqwest` → `nestgate-rpc`. Fix applied: `reqwest` replaced with `ureq` 3.3
+ `rustls-rustcrypto` (pure Rust crypto provider). Verified: `cargo tree -i ring` returns
"did not match any packages"; `cargo deny check bans` PASS.

### Class 2: GPU/Vulkan Dynamic Linking — OPEN (Node Atomic Delegation)

The same class of problem as ring but for compute hardware. The dependency chain:

```
wgpu 28.0.0  →  wgpu-hal 28.0.1  →  ash 0.38.0 (Vulkan bindings)
                                   →  metal 0.33.0 (Apple)
                                   →  windows-rs (DX12)
                                   →  renderdoc-sys

ash 0.38.0   →  libloading 0.8.9  →  dlopen(libvulkan.so.1)  ← FAILS on musl-static
```

**Why musl-static breaks**: musl's `dlopen` implementation cannot load glibc-linked
shared objects. `libvulkan.so.1` (and all GPU ICDs) require glibc. Therefore ecoBin
musl-static binaries can **never** access GPU hardware through the wgpu path.
This is not a bug — it's a fundamental incompatibility between static linking and
dynamic GPU driver loading.

**Affected primals** (compile-time wgpu dependency):

| Primal | wgpu Version | Feature-Gated? | Impact |
|--------|-------------|----------------|--------|
| barraCuda | 28.0.0 | `gpu` feature (default ON) | ecoBin binary always CPU-only |
| toadStool | 22.0.0 | `wgpu` feature (optional) | GPU features unavailable in ecoBin |
| petalTongue | via eframe/egui | Inherent to GUI | Headless mode avoids; acceptable |

**Existing abstractions (partial solutions)**:

| Abstraction | Location | Status | What It Does |
|------------|----------|--------|--------------|
| `GpuBackend` trait | `barraCuda/device/backend.rs` | Done | Backend-agnostic compute interface (9 required methods) |
| `WgpuDevice` | `barraCuda/device/wgpu_backend.rs` | Done | Implements `GpuBackend` via wgpu (needs dlopen — non-portable) |
| `SovereignDevice` | `barraCuda/device/sovereign_device.rs` | Wired | Implements `GpuBackend` via IPC to coralReef+toadStool (portable) |
| `CpuExecutor` | `barraCuda/unified_hardware/cpu_executor.rs` | Done | Native Rust CPU math execution |
| `cpu-shader` + `naga-exec` | `barracuda-naga-exec` crate | Partial | Interprets WGSL shaders on CPU via naga IR (same math, scalar speed) |
| `Auto::new()` | `barraCuda/device/mod.rs` | Done | Tries GPU → CPU software rasterizer → `Err`. **Missing**: no fallback to SovereignDevice or cpu-shader |
| `coral-gpu` | `coralReef/crates/coral-gpu/` | In progress | Sovereign GPU compute — replaces wgpu for compute. No wgpu dependency in production |

**The resolution pattern (Node Atomic Delegation)** mirrors Tower Atomic:

| Tower (SOLVED) | Node (TO SOLVE) |
|----------------|-----------------|
| BearDog: pure Rust crypto | barraCuda: pure Rust math (WGSL) |
| Songbird: TLS via BearDog IPC | barraCuda: GPU via toadStool+coralReef IPC |
| Consumer delegates crypto | Consumer delegates compute dispatch |
| `deny.toml` bans `ring` | Future: `deny.toml` bans direct `wgpu` in consumers |

**Gaps to close** (mapped to BC-06/07/08):

- **BC-06**: Architectural constraint — document, don't fix musl. ecoBin = CPU-only for wgpu path.
- **BC-07**: Wire `SovereignDevice` into `Auto::new()` fallback chain. The trait exists, the impl exists, the IPC wiring exists — they just aren't connected in the failure path.
- **BC-08**: Make `cpu-shader` feature default-on. `barracuda-naga-exec` already interprets WGSL on CPU. Batch ops already have `#[cfg(feature = "cpu-shader")]` paths with scalar Rust fallbacks.

**Target state**: barraCuda computes on **any** hardware:
1. wgpu GPU (development, glibc hosts with GPU) — fastest
2. SovereignDevice IPC (NUCLEUS deployment, coralReef+toadStool available) — GPU via IPC
3. cpu-shader/naga-exec (ecoBin, Docker, no peers) — CPU WGSL interpretation
4. Scalar Rust (absolute minimum, no naga) — native f64 fallback

### Class 3: Remaining C Surfaces — PARTIAL

| ID | Primal | Dependency | Severity | Production? | Status |
|----|--------|-----------|----------|-------------|--------|
| NG-08 | NestGate | `ring` v0.17.14 via `rustls` → `reqwest` | **High** | **YES** — `nestgate-rpc` production path | **OPEN** — deny.toml ban exists but ring resolves anyway |
| CR-01 | coralReef | Missing `deny.toml` C/FFI ban list | Medium | Policy gap | **OPEN** — only license/advisory bans; no `ring`/`openssl` bans |
| CR-02 | coralReef | `cudarc` (CUDA FFI) | Low | Feature-gated (`cuda`) | Acceptable — sovereign path (`coral-gpu`) is pure Rust |
| SG-01 | sweetGrass | `ring` via testcontainers → bollard → rustls | Low | **No** — dev-deps only | Acceptable — does not affect ecoBin binary |
| SB-02 | Songbird | `ring-crypto` opt-in feature | Low | **No** — opt-in, not default | Acceptable — default path uses `rustls_rustcrypto` |
| PT-12 | petalTongue | eframe/egui/glow (OpenGL/Vulkan GUI) | Low | Only in GUI mode | Acceptable — headless (`PETALTONGUE_HEADLESS=true`) avoids |
| TS-03 | toadStool | `wgpu`/`ash`/`vulkano`/`wasmtime`/`esp-idf-sys` | Low | All feature-gated | Acceptable — core crate does not require wgpu by default |
| BD-01 | bearDog | `ndk-sys`/`security-framework-sys` | Low | Target-gated (Android/macOS) | Acceptable — Linux ecoBin unaffected |

### Ring Transitive Audit (April 11, 2026 — `cargo tree -i ring --edges normal`)

| Primal | ring in production? | Path | Action |
|--------|--------------------|----|--------|
| Squirrel | **No** | Not in tree | Clean |
| Songbird | **No** | Not in tree (opt-in `ring-crypto` feature not compiled) | Clean |
| NestGate | **YES** | `ring` → `rustls` → `reqwest` → `nestgate-rpc` → production binary | **NG-08: Fix required** |
| sweetGrass | **No** (dev only) | `ring` → `rustls` → `bollard` → `testcontainers` (dev-deps) | Clean for ecoBin |
| barraCuda | **No** | Banned in deny.toml, not in tree | Clean |
| coralReef | **Unaudited** | No deny.toml ban list (CR-01) | **Audit needed** |

---

## Cross-Spring Upstream Gap Synthesis (April 11, 2026)

Consolidated from April 11 handoffs across all 7 science springs. These are gaps
that multiple springs independently report as blocking their composition evolution.
Each maps to a specific primal team for resolution.

### Recurring Blockers (reported by 3+ springs)

| Gap | Affected Springs | Owner | Status |
|-----|-----------------|-------|--------|
| **BearDog BTSP server endpoint** — no ecosystem BTSP server exists; springs have client stubs but nothing to connect to | hotSpring, healthSpring, neuralSpring, ludoSpring | **BearDog team** | **OPEN** — BearDog is the handshake *provider*, not consumer; needs `btsp.server.*` RPC surface |
| **Ionic bond runtime** — `crypto.ionic_bond` / cross-family GPU lease / data egress fence negotiation | hotSpring (GAP-HS-005), healthSpring (§2), ludoSpring | **BearDog team** | **OPEN** — bonding model is documented in graphs but no runtime protocol |
| **Canonical inference namespace** — springs accept `inference.*` / `model.*` / `ai.*` inconsistently | healthSpring (§4), neuralSpring (Gap 1), ludoSpring (GAP-10) | **primalSpring + Squirrel + neuralSpring** | **OPEN** — need single canonical namespace |
| **TensorSession adoption** — fused multi-op GPU pipelines; springs defer because API unstable | hotSpring (GAP-HS-027), healthSpring, wetSpring | **barraCuda team** | **OPEN** — API exists but not stable enough for spring adoption |
| **Provenance trio IPC stability** — trio endpoints panic, TCP-only, or unreachable | wetSpring (PG-02), ludoSpring, healthSpring | **rhizoCrypt + loamSpine + sweetGrass teams** | **PARTIAL** — UDS wired but endpoints not fully stable |
| **NestGate storage IPC** — `storage.retrieve` / persistent cross-spring data | wetSpring (PG-04), neuralSpring (Gap 5), healthSpring | **NestGate team** | **OPEN** — NestGate has IPC surface but not wired for cross-spring storage |
| **`capability.resolve` / capability-first discovery** — springs want to route by capability, not primal name | wetSpring (PG-03), healthSpring (§3), all springs | **biomeOS + Songbird** | **OPEN** — `capability.discover` exists but `capability.resolve` (single-step) does not |

### Per-Primal Upstream Tasks (from spring handoffs)

**barraCuda** (reported by: hotSpring, neuralSpring, groundSpring, airSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| BC-07: Wire `SovereignDevice` into `Auto::new()` fallback | primalSpring benchScale audit | Medium |
| BC-08: Make `cpu-shader` default-on | primalSpring benchScale audit | Medium |
| `TensorSession` stabilization for spring adoption | hotSpring GAP-HS-027, healthSpring | Medium |
| `plasma_dispersion` feature-gate bug (`domain-lattice` required) | neuralSpring Gap 9 | Low |
| 29 shader absorption candidates from neuralSpring | neuralSpring Gap 10 | Low |
| RAWR GPU kernel (currently CPU-only `stats::rawr_mean`) | groundSpring | Low |
| Batched `OdeRK45F64` for Richards PDE | airSpring evolution_gaps | Low |

**coralReef** (reported by: neuralSpring, hotSpring, healthSpring)

| Task | Source | Priority |
|------|--------|----------|
| CR-01: Add `deny.toml` C/FFI ban list | primalSpring portability audit | Medium |
| Multi-stage ML pipeline support via `shader.compile.wgsl` | neuralSpring handoff | Medium |
| IPC timing for `shader.compile` in deployment | neuralSpring, healthSpring | Low |

**toadStool** (reported by: wetSpring, neuralSpring, airSpring)

| Task | Source | Priority |
|------|--------|----------|
| Stable `compute.dispatch.submit` / `compute.execute` IPC | wetSpring PG-05, neuralSpring | Medium |
| Pipeline scheduling for ordered dispatch | neuralSpring handoff | Low |

**NestGate** (reported by: wetSpring, neuralSpring, healthSpring)

| Task | Source | Priority |
|------|--------|----------|
| ~~NG-08: Eliminate `ring` from production build~~ | primalSpring portability audit | ~~**High**~~ **RESOLVED** (April 11) |
| `storage.retrieve` for large/streaming tensors | neuralSpring, wetSpring PG-04 | Medium |
| Cross-spring persistent storage IPC | healthSpring, wetSpring | Medium |

**BearDog** (reported by: hotSpring, healthSpring, neuralSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| BTSP server endpoint (`btsp.server.*`) | healthSpring §10, hotSpring GAP-HS-006 | **High** |
| Ionic bond runtime (`crypto.ionic_bond`) | hotSpring GAP-HS-005, healthSpring §2 | Medium |
| Signed capability announcements | neuralSpring handoff | Low |

**Squirrel** (reported by: neuralSpring, healthSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| `inference.register_provider` wire method | neuralSpring Gap 1 | Medium |
| Stable ecoBin binary for composition deployments | healthSpring §9 | Medium |

**biomeOS / Songbird** (reported by: wetSpring, healthSpring, ludoSpring)

| Task | Source | Priority |
|------|--------|----------|
| `capability.resolve` single-step routing | wetSpring PG-03, healthSpring §3 | Medium |
| Deploy-time `consumed_capabilities` completeness check | wetSpring V143 handoff | Low |
| `lifecycle.composition` for live dashboards | ludoSpring handoff | Low |

### Spring Evolution Status (April 11, 2026)

| Spring | Version | Stage | Deploy Graphs | Tests | barraCuda | deny.toml | plasmidBin Ready? |
|--------|---------|-------|---------------|-------|-----------|-----------|-------------------|
| **hotSpring** | v0.6.32 | composing | 1 (QCD deploy) | 4,422+ | 0.3.11 (git rev) | **Missing** | Yes — niche-hotspring in manifest |
| **neuralSpring** | v0.1.0 / S181 | composing | 1 (inference deploy) | many | 0.3.11 (path) | Weak (no bans) | Yes — niche-neuralspring in manifest |
| **wetSpring** | V143 | composing | 7 (deploy + workflows) | 1,950 | 0.3.11 (pinned) | Good (openssl banned) | Yes — niche-wetspring in manifest |
| **healthSpring** | V52 / 0.8.0 | composing | 7 (deploy + workflows) | 985+ | 0.3.11 (rev pin) | Good (ring exception for rustls) | Yes — niche-healthspring in manifest |
| **airSpring** | v0.10.0 | composing | 5 (deploy + pipelines) | 1,364 | 0.3.11 (path) | Present | Yes — niche-airspring in manifest |
| **groundSpring** | V124 | composing | 6 (deploy + validation) | many | 0.3.11 (path) | Present | Yes — niche-groundspring in manifest |
| **ludoSpring** | V41 | composing | (via primalSpring) | — | (via barraCuda) | — | Yes — pure composition |

### Spring deny.toml Compliance

| Spring | deny.toml? | `ring` banned? | `openssl` banned? | Notes |
|--------|-----------|---------------|-------------------|-------|
| hotSpring | **No** | N/A | N/A | **Gap: needs deny.toml** |
| neuralSpring | Yes (weak) | **No** | **No** | Only license/advisory checks; **no C/FFI bans** |
| wetSpring | Yes | **No** | **Yes** | Bans openssl + sys crates; ring not explicitly banned |
| healthSpring | Yes | **Exception** | **Yes** | ring allowed as rustls wrapper; explicit evolution note |
| airSpring | Yes | Unknown | Unknown | Present but not fully audited |
| groundSpring | Yes | Unknown | Unknown | Present but not fully audited |

---

## biomeOS

| ID | Gap | Status |
|----|-----|--------|
| BM-01 | `graph.deploy` routing | **RESOLVED** (v2.79 — `graph.execute`) |
| BM-02 | `health.liveness` on Neural API | **RESOLVED** (v2.81) |
| BM-03 | `unix://` prefix on `capability.discover` | **RESOLVED** (v2.79 — `strip_unix_uri`) |
| BM-04 | Late primal registration invisible | **RESOLVED** (v2.81 — `topology.rescan` + lazy discovery) |
| BM-05 | Multi-shape probe response | **RESOLVED** (v2.81) |
| BM-06 | `discover_capability` lacks domain prefix matching | **RESOLVED** (v2.92 — `try_prefix_lookup` + `capability_to_provider_fallback` last resort). Deploy graphs also include bare domain aliases as belt-and-suspenders. |
| BM-07 | Registry stores `{primal}-{hash}.sock` instead of live sockets | **RESOLVED** (April 10 — `get_family_id()` → `self.family_id` in defaults, config, domain bridge; socket alias for toadstool→compute, nestgate→storage) |
| BM-08 | Socket resolution misses primals without `--socket` flag | **RESOLVED** (April 10 — plain `{primal}.sock` fallback in `resolve_primal_socket_with()` for loamSpine, sweetGrass, petalTongue) |
| BM-09 | `capability.call` forwards to tarpc socket instead of JSON-RPC | **RESOLVED** (April 10 — `.jsonrpc.sock` preferred over `.sock` for domain aliases in socket resolution) |
| BM-11 | ToadStool dual-socket: `build_socket_path` + `resolve_primal_socket` lack explicit JSON-RPC preference | **RESOLVED** (April 10 — `prefers_jsonrpc` flag in `socket.rs`, `.jsonrpc.sock` sibling check in `path_builder.rs`, stale socket cleanup in launcher) |

**Compliance** (v3.00 — April 9 second pull): clippy **CLEAN**, fmt **PASS**, **7,724 tests PASS** ↑, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, SPDX present. Zero `#[allow(`. **BTSP Phase 1 COMPLETE** (v2.98). **BTSP Phase 2 COMPLETE** ↑↑ — `btsp_client.rs` expanded to 524+ lines: full server-side handshake (`server_handshake()`) wired into Neural API UDS listener (`handle_connection_with_btsp`), enforce vs warn-only modes, graceful fallback for raw JSON-RPC clients. Wire types: `ClientHello/ServerHello/ChallengeResponse/HandshakeComplete`. BearDog delegation. **v3.00 evolution**: async-trait → native async fn (Edition 2024), `itertools` + `async-trait` removed from 4 crates, license `-only` → `-or-later` across all docs/scripts/LICENSE-ORC, orphan `nucleus_executor.rs` deleted (288 LOC), `/tmp/biomeos` → centralized `runtime_paths`. **Discovery compliance: COMPLETE**.

---

## petalTongue

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| PT-01 | Socket at non-standard path | **RESOLVED** — `biomeos/petaltongue.sock` |
| PT-02 | No live push to browser | **RESOLVED** — SSE `/api/events` |
| PT-03 | `motor_tx` not wired in server mode | **RESOLVED** — drain channel wired |
| PT-04 | No `ExportFormat::Html` in headless CLI | Low | Partially — `ExportFormat::Html` exists in headless path + IPC; needs product validation |
| PT-05 | `visualization.showing` returns false | **RESOLVED** — `RenderingAwareness` auto-init in `UnixSocketServer` |
| PT-06 | `callback_method` poll-only dispatch | Low | Code-complete — `push_delivery.rs` module, `broadcast()`, `CallbackDispatch` wired; **not enabled on live server** (`callback_tx` = `None` at startup; server wiring needed to activate push) |
| PT-07 | No external event source in server mode | **RESOLVED** — periodic discovery refresh wired |
| PT-08 | No BTSP Phase 1 (`BIOMEOS_INSECURE` guard) | **RESOLVED** ↑ — `btsp.rs` module: `validate_insecure_guard()`, family-scoped sockets, domain symlinks |
| PT-09 | BTSP Phase 2 (handshake integration) | Low | Phase 2 stub — `handshake_policy` logs warning, connections accepted without handshake |
| PT-10 | `--socket` CLI flag missing | **RESOLVED** | April 10 — `--socket` flag added to `Commands::Server`, plumbed via `UnixSocketServer::with_socket_path()` |
| PT-11 | Only `visualization` domain symlink | **RESOLVED** | April 10 — now creates `visualization.sock`, `ui.sock`, `interaction.sock` symlinks (create+drop) |

**Compliance** (v1.6.6+ — April 10): clippy **CLEAN**, fmt **PASS**, `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX present. Zero `todo!`/`unimplemented!`/`FIXME`. Tests **ALL PASS**. **BTSP Phase 1 COMPLETE** ↑↑. **BTSP Phase 2 STUB** — `handshake_policy` logs but does not enforce. **`--socket` CLI flag** wired via `with_socket_path()`. **Domain symlinks**: `visualization`, `ui`, `interaction`. **Capability Wire Standard L2/L3**.

---

## barraCuda

BC-01–BC-05 **RESOLVED**. New architectural gaps BC-06–BC-08 identified during benchScale
NUCLEUS deployment validation (April 11). Math is universal — these gaps block
barraCuda from fulfilling its role as a hardware-agnostic math primal.

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| BC-01 | Fitts formula variant | **RESOLVED** (Sprint 25 — `variant` param, Shannon default) |
| BC-02 | Hick formula off-by-one | **RESOLVED** (Sprint 25 — `include_no_choice` param) |
| BC-03 | Perlin3D lattice | **RESOLVED** (Sprint 25 — proper gradients + quintic fade) |
| BC-04 | No plasmidBin binary | **RESOLVED** (April 1 harvest, 4.5M static-pie stripped) |
| BC-05 | `barracuda server` panics without GPU | **RESOLVED** (Sprint 39 — `Auto::new()` returns `Err`, server starts with `device = None`, health reports `Degraded`. Stale binary in plasmidBin was pre-Sprint 39; refreshed April 11) |
| BC-06 | musl-static binary can't access GPU | Medium | **OPEN** — wgpu requires `dlopen(libvulkan.so.1)` at runtime, which musl doesn't support. ecoBin musl-static binaries will **always** run in CPU-only mode. Fix: IPC delegation to coralReef+toadStool (glibc or host GPU access), or pure-CPU scalar fallback for all WGSL ops |
| BC-07 | No toadStool→coralReef IPC delegation | Medium | **OPEN** — when barraCuda has no local device, it should discover toadStool (hardware) + coralReef (compiler) via IPC and delegate compute. Currently it just runs with `device = None` and reports `Degraded`. The dispatch chain should be: barraCuda → toadStool (discover hardware) → coralReef (compile WGSL) → execute |
| BC-08 | No pure-CPU scalar fallback | Low | **OPEN** — math ops expressed as WGSL shaders have no scalar CPU fallback when wgpu is unavailable. A CPU-only codegen path (or Rust scalar implementations of each WGSL op) would allow barraCuda to compute anywhere without wgpu. This is the "math is universal" principle |

**Compliance** (Sprint 39 — April 10): clippy **CLEAN** (`-D warnings`, pedantic + nursery), fmt **PASS**, `deny.toml` present (bans openssl/native-tls/ring/aws-lc-sys), zero `todo!`/`unimplemented!`/`FIXME`. **4,422 tests PASS** (nextest CI). `#![forbid(unsafe_code)]` on `barracuda` + `barracuda-core`. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `guard_connection()` implements full 6-step handshake relay: `ClientHello` → `btsp.session.create` → `ServerHello` → `ChallengeResponse` → `btsp.session.verify` → `HandshakeComplete`. Capability-based crypto provider discovery (`crypto-{fid}.sock` → `crypto.sock` → `*.json` scan). All 3 accept loops guarded (Unix, TCP, tarpc). Legacy/non-BTSP clients degrade gracefully (2s timeout). **Capability Wire Standard L2**. Nextest `gpu-serial` extended to stress/gpu profiles. **Note**: `BufReader` lifetime gap between handshake phases (edge-case for fast/coalescing clients); post-handshake stream encryption not yet applied.

---

## Squirrel

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SQ-01 | Abstract-only socket | **RESOLVED** (alpha.25b — `UniversalListener`) |
| SQ-02 | `LOCAL_AI_ENDPOINT` not wired into `AiRouter` | **RESOLVED** (alpha.27 — step 1.5 discovery, `resolve_local_ai_endpoint()`) |
| SQ-03 | `deprecated-adapters` feature flag docs | **RESOLVED** — documented in `CURRENT_STATUS.md` feature-gates table; intentional retention until v0.3.0 with migration path to `UniversalAiAdapter` + `LOCAL_AI_ENDPOINT` |

**Compliance** (alpha.46+ — April 9 second pull): Zero `todo!`/`unimplemented!`/`FIXME` in non-test code. fmt **PASS**. clippy **PASS**. **7,203 tests PASS**. `deny.toml` present. Workspace `forbid(unsafe_code)`. **BTSP Phase 1 COMPLETE** (alpha.44). **BTSP Phase 2 COMPLETE** ↑↑ — `btsp_handshake.rs` (627 LOC) implements full server-side handshake on UDS accept with BearDog delegation (`btsp.session.create`, `btsp.session.verify`). `maybe_handshake()` called in both abstract+filesystem UDS accept paths in `jsonrpc_server.rs`. Length-prefixed wire framing per standard. `is_btsp_required()` checks `FAMILY_ID` + `BIOMEOS_INSECURE`. Provider discovery: env → manifest scan → well-known `beardog-{fid}.sock`. **Capability Wire Standard L2**. Smart refactoring: session/mod.rs, transport/client.rs, context_state.rs, api.rs all under 600 LOC. Dependency purge: pprof/openai/libloading removed, flate2 → pure Rust backend. **Inference provider bridge** ↑ — `inference.complete`/`embed`/`models` wire methods dispatched via `handlers_inference.rs`, bridging ecoPrimal wire standard to `AiRouter`.

---

## songBird

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SB-01 | `health.liveness` canonical name | **RESOLVED** (wave89-90) |
| SB-02 | CLI `ring-crypto` opt-in feature | Low | Near-resolved — `rcgen` removed from lockfile (wave93); `ring` still in `Cargo.lock` but **not compiled** in default build; `ring-crypto` is opt-in CLI feature with single `cfg`-gated call. Default uses `rustls_rustcrypto`. Lockfile refresh would remove stale `ring` stanza |
| SB-03 | `sled` in orchestrator/sovereign-onion/tor | Low | Improved — wave93 feature-gated sled (`optional = true` + `dep:sled`) in all 3 crates. `sled-storage` default-on in orchestrator + sovereign-onion; opt-in `persistent-cache` for tor. Pending NestGate storage API |

**Compliance** (Wave 133 — April 9 wave 3): clippy **CLEAN**, fmt **PASS**. `forbid(unsafe_code)` per-crate, `deny.toml` present, SPDX present. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake()` in `ipc/btsp.rs`, wired into UDS accept loop (`connection.rs` branches on `btsp_active`), BearDog delegation via `SecurityRpcClient`. `BtspClient` + `btsp_client.rs`. Length-prefixed frames after handshake. **Capability Wire Standard L3**.

---

## NestGate

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| NG-01 | IPC storage backend inconsistency | **RESOLVED** ↑ — `SemanticRouter::new()` enforces `FileMetadataBackend` in production; `InMemoryMetadataBackend` only used in tests/ephemeral. NG-01 compliance: file backend mandatory when `FAMILY_ID` set |
| NG-02 | Session API inconsistency | **RESOLVED** — `semantic_router/session.rs` added; `SemanticRouter::call_method` dispatches `session.save`/`load`/`list`/`delete` |
| NG-03 | `data.*` handlers delegation | **RESOLVED** ↑ — `data.*` wildcard delegation replaces hardcoded NCBI/NOAA/IRIS stubs. Returns structured `NotImplemented` with `discovery.query` redirect. Explicitly excluded from `capabilities.list`. Tested in `data_wildcard_returns_delegation_not_implemented` |
| NG-04 | C dependency (`aws-lc-rs`/`ring`) | **RESOLVED** — `ring` eliminated; `reqwest` → `ureq` + `rustls-rustcrypto` (April 11, 2026) |
| NG-05 | Crypto crates not fully delegated | **RESOLVED** — `nestgate-security` zero crypto deps, all via BearDog IPC `CryptoDelegate` |

| NG-06 | `--socket` CLI flag not wired in `Commands::Server` | **RESOLVED** | April 10 — `--socket` flag added to `Commands::Server`, sets `NESTGATE_SOCKET` env var before `run_daemon`, feeds into `SocketConfig::from_environment()` tier-1 resolution |
| NG-07 | aarch64-musl segfault | **RESOLVED** | Static-PIE + musl ≤1.2.2 crash in `_start_c/dlstart.c`. Fixed: `-C relocation-model=static` in `.cargo/config.toml` for both x86_64 and aarch64 targets |
| NG-08 | `ring` v0.17.14 in production via `rustls` default crypto | **High** | **RESOLVED** (April 11) — `reqwest` replaced with `ureq` 3.3 + `rustls-rustcrypto`. `cargo tree -i ring` clean. `cargo deny check bans` PASS |

**Compliance** (April 11, 2026): Clippy **ZERO WARNINGS** (`--workspace --lib`), fmt **PASS**, **11,856+ tests PASS**, `cargo doc -D warnings` **PASS**, `cargo deny check bans` **PASS**. `forbid(unsafe_code)` per-crate + workspace `deny`. SPDX present. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE**. **NG-08 RESOLVED** — `reqwest` → `ureq` + `rustls-rustcrypto`; `ring`/`openssl`/`aws-lc-rs`/`native-tls` fully eliminated; `cargo tree -i ring` clean. **NG-01 RESOLVED** — `FileMetadataBackend` enforced in production. **NG-03 RESOLVED** — `data.*` wildcard delegation. **NG-06 RESOLVED** — `--socket` CLI flag. Dead code cleaned (unwired modules, `if false` stubs, `#[allow(dead_code)]` → `#[expect]`). Zero TODO/FIXME/HACK in production code. **Capability Wire Standard L3**.

---

## rhizoCrypt

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| RC-01 | TCP-only transport | **RESOLVED** (v0.14.0-dev s23 — `--unix`, `UdsJsonRpcServer`, `biomeos/` path) |
| RC-02 | Witness wire evolution | **RESOLVED** (v0.14.0-dev — `WireWitnessRef`: kind/evidence/encoding/algorithm/tier/context) |

**Compliance** (S31 — April 9 wave 3): clippy clean, fmt clean, `deny(unsafe_code)` + `forbid`, `deny.toml` present, tests pass. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — new `rhizo-crypt-rpc/src/btsp/` module (framing.rs, server.rs, types.rs): `BtspServer::accept_handshake` implements full 4-step handshake, wired into UDS accept loop (`serve_inner` → `handle_uds_connection`). **Local crypto** (HKDF, X25519, HMAC-SHA256) — does NOT delegate to BearDog (self-sovereign approach). Client handshake in `btsp/handshake.rs`. **Capability Wire Standard L3**.

---

## loamSpine

All gaps **RESOLVED**.

| ID | Gap | Status |
|----|-----|--------|
| LS-03 | Panic on startup | **RESOLVED** (v0.9.15 — infant discovery fails gracefully) |
| LS-04 | Witness wire evolution | **RESOLVED** (v0.9.16 — `WireWitnessRef` in `trio_types.rs`, witnesses on wire summaries) |
| LS-05 | `--socket` CLI flag missing | **RESOLVED** | April 10 — `--socket` flag added to `Command::Server`, passed directly to `run_server` (no env mutation, respects `forbid(unsafe_code)`) |

**Compliance** (v0.9.16+ — April 10): clippy clean, fmt **PASS**, `forbid(unsafe_code)` workspace, `deny.toml` present, tests pass. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake()` in `crates/loam-spine-core/src/btsp.rs`, wired into UDS accept loop (`run_jsonrpc_uds_server` → `handle_uds_connection`). **Delegates to BearDog** (`btsp.session.create`, `btsp.session.verify`, `btsp.negotiate`). Tests with mock BearDog + mock client in `btsp_tests.rs`. **`--socket` CLI flag** wired. **Capability Wire Standard L2/L3**.

---

## toadStool

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| TS-01 | coralReef discovery not pure capability-based | Low | **RESOLVED** — `VisualizationClient` (shader client) uses `capability.discover("shader")` Tier 1, then filesystem fallback Tiers 0/2/3. No 6-step pattern remains. |
| TS-02 | `compute.sock` tarpc-only; JSON-RPC probes fail | **RESOLVED** | April 10 — `jsonrpc_socket` now `compute.jsonrpc.sock` (separate from tarpc `compute.sock`). Legacy symlinks: `toadstool.jsonrpc.sock` → `compute.jsonrpc.sock` |
| TS-03 | `--socket` CLI flag parsed but not wired | **RESOLVED** | April 10 — `socket_override` param added to `run_server_main`, wired through dispatch. Overrides `get_socket_path()` resolution |
| TS-04 | `ollama.*`/`inference.*` semantic mappings advertised but not dispatched | **RESOLVED** | April 10 — Removed from `mappings_extended.rs`. Inference is Squirrel's domain via ecoPrimal wire standard. ToadStool is compute substrate, not model serving. |

**Compliance** (S198+ — April 10 NUCLEUS patterns): Clippy **CLEAN**, fmt **PASS**. 21,600+ tests **PASS**. `deny.toml` present. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `BtspServer::accept_handshake` wired into JSON-RPC Unix accept (`pure_jsonrpc/connection/unix.rs`) and tarpc accept (`unix_maybe_btsp_before_tarpc`), feature-gated behind `btsp` feature + env check. `BtspClient` in `toadstool_common::btsp`. Fuzz targets added (`fuzz_btsp_framing.rs`). **Capability Wire Standard L3**. **Socket separation COMPLETE** — JSON-RPC and tarpc bind distinct sockets. `--socket` CLI override wired to `run_server_main`.

---

## sweetGrass

All gaps **RESOLVED**. TCP JSON-RPC added, `cargo-deny`, `forbid(unsafe)`.

| ID | Gap | Status |
|----|-----|--------|
| SG-01 | Witness wire evolution | **RESOLVED** (v0.7.27 — `Witness` type, `EcoPrimalsAttributes.witnesses`, kind/evidence/encoding) |
| SG-02 | `--socket` CLI flag missing | **RESOLVED** | April 10 — `--socket` flag added to `Commands::Server`, plumbed via `start_uds_listener_at()` / `cleanup_socket_at()` |

**Compliance** (v0.7.27+ — April 10): clippy clean, fmt clean, `deny(unsafe_code)` workspace + `forbid` per-crate, `deny.toml` present, tests pass. **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — new `sweet-grass-service/src/btsp/` module (mod.rs, protocol.rs, server.rs): `perform_server_handshake()` wired into UDS accept (`handle_uds_connection_btsp` in `uds.rs`) + TCP (`tcp_jsonrpc.rs`). **Delegates to BearDog** (`btsp.session.create/verify/negotiate`). Client: `perform_handshake()` in `sweet-grass-integration/src/btsp/protocol.rs`. **`--socket` CLI flag** wired. **Capability Wire Standard L3**.

---

## sourDough

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| SD-01 | Missing `deny.toml` | Low | Open — cargo-deny not configured |
| SD-02 | musl cross-compilation | Low | Open — binary builds not yet wired for ecoBin |
| SD-03 | genomeBin signing | Low | Open — sequoia-openpgp not implemented |

**Compliance** (v0.1.0 — f1cc802): clippy **CLEAN** (`all` + `pedantic` + `nursery`), fmt **PASS**, `forbid(unsafe_code)` at workspace level, `deny.toml` **MISSING**, SPDX AGPL-3.0-or-later in Cargo.toml. **239 tests, 0 failures** (unit + integration + e2e + doctests), coverage 96%+. Edition 2024, workspace lints centralized. Zero `TODO`/`FIXME`/`HACK`/`unimplemented!` in source. **Discovery compliance: NEAR-CLEAN** — 1 BearDog string in CLI genomebin.rs (cosmetic). Scaffold independence confirmed: generated primals have no runtime dependency on sourDough.

---

## coralReef

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| CR-01 | No BTSP Phase 1 (`BIOMEOS_INSECURE` guard) | **RESOLVED** ↑ — `validate_insecure_guard()` in glowplug, core, ember; called from `main.rs` |
| CR-02 | No `capabilities.list` with flat `methods` array | **RESOLVED** ↑ — `capability.list` + `identity.get` with flat `methods` (uses singular `capability.list` not `capabilities.list`) |
| CR-03 | BTSP Phase 2 (handshake) | **RESOLVED** ↑↑ — `guard_connection()` (renamed from `gate_connection`) in all 3 crates: BearDog delegation via `btsp.session.create`, capability-based crypto socket discovery, `BtspOutcome` enum. Async in core/glowplug, blocking in ember. Degraded mode when provider missing. |
| CR-04 | Typed errors (`Result<_, String>` in driver) | Low | Partial — Waves 1–3: `PciDiscoveryError`, `ChannelError`, `DevinitError` with `#[from]` into `DriverError`. ~20 deep HW functions still use `Result<_, String>` (Wave 4+ pending). |
| CR-05 | `cpu_exec.rs` dead code | Low | File exists but `mod cpu_exec` not in `service/mod.rs` — not compiled. `interpret_simple` is a no-op stub. Phase 3 CPU validation/execute prep, not yet wired to IPC dispatch. |

**Compliance** (Iter 78 — April 10): clippy **CLEAN** (pedantic + nursery, 0 warnings), fmt **PASS**, `forbid(unsafe_code)` on coralreef-core + nak-ir-proc + stubs, `deny.toml` present (bans wildcards, yanked-deny). **4,459 tests, 0 failures**, ~153 ignored (HW-gated). SPDX present. **0 files over 1000 LOC** (7 large files split into modules: `nv_metal`, `memory`, `vfio_compute`, `falcon_capability`, `knowledge`, `device`, `codegen/ops`). `coral-driver` opts out of workspace `unsafe_code = "deny"` (ioctl/mmap/MMIO required). **BTSP Phase 1 COMPLETE**. **BTSP Phase 2 COMPLETE** ↑↑ — `guard_connection()` calls `btsp.session.create` on real UDS, parses `session_id`, degrades when provider absent. Wired into Unix JSON-RPC, TCP newline, tarpc accept paths. Full challenge-response + encrypted framing still Phase 3. **Capability Wire Standard L2** ↑ — `capability.list` + `identity.get` with flat `methods`. Uses singular `capability.list` (not `capabilities.list`). **Note**: crypto socket discovery paths differ between core (`config::discovery_dir()`) and ember/glowplug (`XDG_RUNTIME_DIR`) — potential cross-process alignment issue for NUCLEUS. tarpc `Result<_, String>` → `TarpcCompileError` (typed serde-friendly wrapper). `#[allow]` → `#[expect]` lint migration started.

---

## bearDog

| ID | Gap | Severity | Status |
|----|-----|----------|--------|
| BD-01 | `crypto.verify_ed25519` does not accept `encoding` hint | **RESOLVED** ↑ — Wave 33: per-field `message_encoding`, `signature_encoding`, `public_key_encoding` + global `encoding` default. Semantic aliases `crypto.ed25519.sign`/`crypto.ed25519.verify` added. Tests cover hex/mixed encodings. |

**Compliance** (Wave 33 — April 9 wave 4): clippy clean, fmt clean, `forbid(unsafe_code)` at workspace level, `deny.toml` present. SPDX present. **Coverage 90.51%** (llvm-cov). **14,593+ tests, 0 failures.** **0 files over 1000 LOC** (runtime.rs 1244→360, socket_config.rs 1111→668). `#[allow(` 193→75 (62% reduction), `#[expect(reason` 361→476. **BTSP Phase 2+3 COMPLETE**. **Capability Wire Standard L2**. **Dynamic `ipc.register`** with orchestration registry (non-blocking + heartbeat). **Standalone startup** (`standalone-{uuid}` on missing `NODE_ID`). **BD-01 RESOLVED**. Minor: `capabilities.rs` `operation_dependencies` says `btsp.negotiate` but handler is `btsp.session.negotiate`.

---

## BTSP Secure-by-Default Compliance (April 9, 2026)

Per `BTSP_PROTOCOL_STANDARD.md` v1.0: All primals MUST implement socket naming
alignment (Phase 1) and BTSP handshake (Phase 2) when `FAMILY_ID` is set.

### Phase 1: Socket Naming + BIOMEOS_INSECURE Guard

| Primal | Socket Naming | INSECURE Guard | Family-Scoped | Domain-Based | Status |
|--------|:---:|:---:|:---:|:---:|--------|
| BearDog | PASS | PASS | PASS | PASS (`security`) | **COMPLETE** |
| Songbird | PASS | PASS | PASS | partial (`network`) | **COMPLETE** |
| biomeOS | PASS | PASS | PASS | — | **COMPLETE** (v2.98) |
| NestGate | PASS | PASS | PASS | PASS (`storage`) | **COMPLETE** |
| ToadStool | PASS | PASS | PASS | pending | **COMPLETE** (S192) |
| Squirrel | PASS | PASS | PASS | PASS (`ai`) | **COMPLETE** (alpha.44) |
| rhizoCrypt | PASS | PASS | PASS | PASS (`dag`) | **COMPLETE** (S29) |
| loamSpine | PASS | PASS | PASS | PASS (`permanence`) | **COMPLETE** (v0.9.16) |
| sweetGrass | PASS | PASS | PASS | partial | **COMPLETE** (v0.7.27) |
| barraCuda | PASS | PASS | PASS | PASS (`math`) | **COMPLETE** (Sprint 31) |
| petalTongue | PASS ↑ | PASS ↑ | PASS ↑ | PASS (`visualization`) | **COMPLETE** ↑↑ |
| coralReef | PASS ↑ | PASS ↑ | PASS ↑ | PASS (`shader`/`device`) | **COMPLETE** ↑↑ |
| skunkBat | PASS ↑ | PASS ↑ | PASS ↑ | — | **COMPLETE** ↑↑ |

### Phase 2: BTSP Handshake Integration

| Primal | Handshake on Accept | Handshake Client | Status |
|--------|:---:|:---:|--------|
| BearDog | **YES** (`perform_server_handshake`) | **YES** (reference impl) | **COMPLETE** — Wave 31 |
| Songbird | **YES** ↑↑ (`perform_server_handshake`) | **YES** (`BtspClient`) | **COMPLETE** ↑↑ — Wave 133 |
| biomeOS | **YES** (`handle_connection_with_btsp`) | **YES** (`btsp_client.rs`) | **COMPLETE** — v3.00 |
| NestGate | **YES** (`btsp_server_handshake.rs`) | **YES** (`btsp_client.rs`) | **COMPLETE** — both UDS paths |
| ToadStool | **YES** ↑↑ (`BtspServer::accept_handshake`) | **YES** ↑ (`BtspClient`) | **COMPLETE** ↑↑ — S198 |
| Squirrel | **YES** (`btsp_handshake.rs`) | **YES** | **COMPLETE** — alpha.46+ |
| rhizoCrypt | **YES** ↑↑ (`BtspServer::accept_handshake`) | **YES** (`btsp/handshake.rs`) | **COMPLETE** ↑↑ — S31 (local crypto) |
| loamSpine | **YES** ↑↑ (`perform_server_handshake`) | mock only | **COMPLETE** ↑↑ — BearDog delegation |
| sweetGrass | **YES** ↑↑ (`perform_server_handshake`) | **YES** (`btsp/protocol.rs`) | **COMPLETE** ↑↑ — BearDog delegation |
| barraCuda | **YES** ↑↑ (`guard_connection`) | **YES** (BearDog relay) | **COMPLETE** ↑↑ — Sprint 39 (full 6-step relay: ClientHello→create→ServerHello→ChallengeResponse→verify→Complete) |
| petalTongue | stub (warn-only) | no | **STUB** ↑ — Phase 1 done, Phase 2 log-only |
| coralReef | **YES** ↑↑ (`guard_connection`) | **YES** (BearDog session.create) | **COMPLETE** ↑↑ — Iter 78 (real UDS RPC to BearDog, session_id parsed, degraded when provider absent) |
| skunkBat | no | no | **NOT STARTED** — Phase 1 only |

**Phase 2 ecosystem cascade (April 9–10)**: **11/13** primals now enforce BTSP handshake on
incoming UDS connections: BearDog, Songbird, biomeOS, NestGate, ToadStool, Squirrel,
rhizoCrypt, loamSpine, sweetGrass, **barraCuda** ↑ (Sprint 39), **coralReef** ↑ (Iter 78).
**Tower Atomic: 100%.** **Node Atomic: 100%.** **NUCLEUS: 100%.**
petalTongue has Phase 1 + Phase 2 stub (warn-only). skunkBat Phase 1 only.
**Note**: Full challenge-response + encrypted framing (Phase 3) not yet applied to
post-handshake streams in barraCuda or coralReef.

**plasmidBin validation (April 10)**: Full musl-static rebuild confirms all BTSP Phase 1+2
code is now in the deployed binaries. Previous plasmidBin binaries (Apr 8) predated the
cascade and had no handshake enforcement. 12/12 ecoBin compliant.

---

## Capability Wire Standard Compliance (April 9, 2026)

Per `PRIMAL_CAPABILITY_WIRE_STANDARD_APR08_2026.md` v1.0: every primal's
`capabilities.list` MUST include `primal`, `version`, `methods` (flat array).

| Primal | Wire Level | `methods` flat | `identity.get` | `provided_capabilities` | Notes |
|--------|:---:|:---:|:---:|:---:|---|
| BearDog | **L2** | YES | YES | YES (Format E groups) | Reference impl |
| Songbird | **L3** | YES | YES | YES | `capability_tokens.rs` |
| biomeOS | consumer | parses all | probes peers | — | 5-format adaptive parser |
| NestGate | **L3** | YES | YES | YES | `model_cache_handlers.rs` |
| ToadStool | **L3** | YES | YES | YES | `handler/core.rs` |
| Squirrel | **L2** | YES | YES | partial | `handlers_capability.rs` |
| rhizoCrypt | **L3** | YES | YES | YES | `niche.rs` — full composable |
| loamSpine | **L2/L3** | YES | YES | partial | Wire Standard sprint complete |
| sweetGrass | **L3** | YES | YES | YES | Full composable |
| barraCuda | **L2** | YES | YES | partial | Sprint 31 |
| petalTongue | **L2/L3** | YES | YES | partial | `system.rs` |
| coralReef | **L2** ↑ | YES ↑ | YES | partial | `capability.list` (singular) + `identity.get` |
| skunkBat | **L2** ↑ | YES ↑ | YES ↑ | partial | New JSON-RPC server, both `capability.list` + `capabilities.list` |
| sourDough | **NONE** | NO | NO | NO | Scaffolding tool, not IPC primal |
| bingoCube | **NONE** | NO | NO | NO | CLI tool, not IPC primal |

---

## plasmidBin Binary Inventory (April 10, 2026 — full musl rebuild)

All 12 x86_64 primals rebuilt with `--target x86_64-unknown-linux-musl` and stripped.
**12/12 ecoBin compliant** — zero dynamic library dependencies, no interpreter.

| Binary | Size | Linkage | ecoBin | Build Date | BTSP P1+P2 in Binary? |
|--------|------|---------|:---:|------------|:---:|
| beardog | 7.4M | **STATIC** | YES | Apr 10 | **YES** |
| songbird | 17M | **STATIC** | YES | Apr 10 | **YES** |
| nestgate | 7.9M | **STATIC** | YES | Apr 10 | **YES** |
| squirrel | 3.4M | **STATIC** | YES | Apr 10 | **YES** |
| toadstool | 11M | **STATIC** | YES | Apr 10 | **YES** |
| petaltongue | 29M | **STATIC** | YES | Apr 10 | Phase 1 (Phase 2 stub) |
| biomeos | 17M | **STATIC** | YES | Apr 10 | **YES** (BM-07 fix) |
| rhizocrypt | 5.7M | **STATIC** | YES | Apr 10 | **YES** |
| loamspine | 4.4M | **STATIC** | YES | Apr 10 | **YES** |
| sweetgrass | 8.9M | **STATIC** | YES | Apr 10 | **YES** |
| barracuda | 4.7M | **STATIC** | YES | Apr 10 | **YES** ↑ (full 6-step relay) |
| skunkbat | 2.2M | **STATIC** | YES | Apr 10 | Phase 1 only |

**aarch64** (5 binaries): beardog, songbird, squirrel, toadstool static+stripped; biomeos static NOT stripped.

**PLASMIBIN-STALE RESOLVED.** All x86_64 binaries now include BTSP Phase 1+2 code
from the April 9 ecosystem cascade. musl-static compliance: 12/12 (was 2/11).

---

## Per-Primal Team Handoff (April 10, 2026)

Copy-paste blurbs for team assignment. Grouped by priority.

### Primals Needing Dedicated Team Evolution

**barraCuda** — BTSP Phase 2 **COMPLETE** ↑↑ (Sprint 39 — full 6-step handshake relay).
GPU panic **RESOLVED** (BC-05 — `Auto::new()` returns `Err`, health `Degraded`).
`fault_injection` test SIGSEGV persists (nextest `gpu-serial` workaround). 4,422 tests.
**Remaining**: `BufReader` lifetime edge-case in handshake relay, post-handshake stream
encryption (Phase 3), musl rebuild with Sprint 39 changes. **Effort: low.**

**coralReef** — BTSP Phase 2 **COMPLETE** ↑↑ (Iter 78 — `guard_connection()` with real
BearDog RPC in all 3 crates, degraded mode when provider absent). 7 large files split
into modules, typed driver errors Waves 1–3. 4,459 tests. `cpu_exec.rs` exists but is
dead code (not wired into `service/mod.rs`). Wire Standard L2 (singular `capability.list`).
**Remaining**: CR-04 typed errors Wave 4+ (~20 HW functions still `Result<_, String>`),
CR-05 `cpu_exec.rs` wiring, crypto socket discovery path alignment across crates, musl
rebuild with Iter 78 changes. **Effort: low-medium.**

### Deferred (later development cycle)

**skunkBat** — BTSP Phase 2 not started (Phase 1 + Wire L2 done). Deferred to later cycle.

**sourDough** — `deny.toml` missing, musl build, genomeBin signing. Scaffolding CLI tool, not IPC primal. Deferred to later cycle.

### Primals With Tractable Local Fixes

**biomeOS** — BM-10: method translation **RESOLVED**. BM-11: ToadStool dual-socket
**RESOLVED** (`prefers_jsonrpc` + `.jsonrpc.sock` sibling check). **All tractable biomeOS gaps resolved.**

**ToadStool** — TS-01: coralReef discovery **RESOLVED** (`capability.discover("shader")` Tier 1).
Compute socket resolution fully functional via BM-11 (`prefers_jsonrpc` flag + `.jsonrpc.sock`
sibling preference). **All tractable ToadStool gaps resolved.**

**Songbird** — SB-02: `ring` lockfile ghost (not compiled, just stale `Cargo.lock` stanza).
SB-03: `sled` feature-gated but default-on in orchestrator/sovereign-onion — pending
NestGate storage API migration. **Effort: low. Polish items, no runtime blockers.**

**petalTongue** — PT-10 `--socket` **RESOLVED**, PT-11 domain symlinks **RESOLVED** (`ui`, `interaction`, `visualization`).
Remaining: PT-04 HTML export (partial), PT-06 push delivery (`callback_tx` not activated), PT-09 BTSP Phase 2 stub.
**Effort: low-medium. Functional for NUCLEUS.**

**NestGate** — aarch64-musl segfault **RESOLVED**, NG-08 `ring` leak **RESOLVED** (April 11 —
`reqwest` → `ureq` + `rustls-rustcrypto`; zero C/ASM crypto in production binary).
All code gaps resolved. Open: `storage.retrieve` streaming variant + cross-spring storage IPC.
**Reference standard alongside BearDog.**

**loamSpine** — LS-03 startup crash **RESOLVED** (v0.9.15 — infant discovery graceful
degradation). No `--socket` CLI flag (uses plain socket fallback). Connection closes after
first response — primalSpring now calls `capabilities()` before `health_check()` as
workaround. **Effort: trivial (connection reuse would be nice but not blocking).**

### Reference Standard Primals (Working Well)

**BearDog** — Gold standard. Zero-port, BTSP Phase 2+3 complete, 14,593 tests, 90.51%
coverage, all files under 1000 LOC, dynamic `ipc.register`, standalone startup. Only minor:
`btsp.negotiate` vs `btsp.session.negotiate` metadata inconsistency.

**Songbird** — Zero-port default, federation opt-in via `--port`. BTSP Phase 2 complete,
Wire Standard L3. The gateway model for all external communication.

**Squirrel** — Zero-port, BTSP Phase 2 complete, AI provider chain fully operational
(Squirrel → OpenAI adapter → Songbird → Ollama). 7,203 tests. Wire Standard L2.
**Inference provider bridge**: `inference.complete`/`embed`/`models` via ecoPrimal wire standard.

**biomeOS** — Orchestration substrate. BTSP Phase 2 complete, 7,724 tests, registry routing
fixed (BM-07/08/09), BM-10 method translation + BM-11 ToadStool dual-socket **RESOLVED**.
**All gaps resolved.**

**NestGate** — 11,856 tests, BTSP Phase 2 complete, Wire Standard L3. `--socket` wired.
Fully functional on x86_64.

**ToadStool** — 21,600 tests, BTSP Phase 2 complete, Wire Standard L3. Socket separation
complete (JSON-RPC vs tarpc). `--socket` wired.

**Provenance Trio (rhizoCrypt + loamSpine + sweetGrass)** — All three BTSP Phase 2 complete,
Wire Standard L2/L3. Witness wire (`WireWitnessRef`) fully standardized. rhizoCrypt uses
local crypto (self-sovereign), loamSpine/sweetGrass delegate to BearDog.

### Downstream (NOT in this registry — reference only)

**ludoSpring** — Spring (not a primal). Binary NOT in plasmidBin. IPC surface: 8 `game.*`
methods; esotericWebb needs 6 more. See `graphs/downstream/ludospring_proto_nucleate.toml`.

**esotericWebb** — Garden/composition (not a primal). Binary NOT in plasmidBin. Transport
needs UDS negotiation. See `graphs/downstream/esotericwebb_proto_nucleate.toml`.

---

## Priority Order

**0 HIGH blockers. 2 MEDIUM. 7 LOW. Zero runtime blockers.** (sourDough + skunkBat deferred)

**High**: ~~PLASMIBIN-STALE~~ **RESOLVED** (April 10 — full musl-static rebuild, 12/12 ecoBin).

**Medium** (degrades composition/experiment quality):
1. **BTSP-BARRACUDA-WIRE** — barraCuda `guard_connection()` does session creation but not full X25519 challenge-response on client stream
2. **IONIC-RUNTIME** — Ionic bond propose→accept→seal needs BearDog `crypto.sign_contract`

**Resolved this session (April 10 NUCLEUS patterns)**:
- ~~**NESTGATE-UDS**~~ **RESOLVED** — `--socket` CLI flag added and wired through dispatch → `NESTGATE_SOCKET` env var → `SocketConfig` tier-1 resolution. C5 now PASS (5/5).
- ~~**TS-UDS-JSONRPC**~~ **RESOLVED** — JSON-RPC gets dedicated `compute.jsonrpc.sock` socket, separate from tarpc `compute.sock`. Legacy symlinks for both protocols. `--socket` CLI flag wired to `run_server_main`.
- ~~**NEURAL-API-DOUBLE-PREFIX**~~ **RESOLVED** (prior session) — `capability.call` strips leading domain prefix from operation parameter.
- **BTSP-CLIENT** — primalSpring BTSP client handshake implemented (`btsp_handshake.rs`), integrated into `Transport::connect()` with auto-detection via `security_mode_from_env()`.

**Low** (polish, owned by primal teams):
3. **SB-02** — `ring` lockfile ghost
4. **SB-03** — `sled` feature-gated but default-on
5. **PT-09** — petalTongue Phase 2 stub (warn-only, no enforcement)
6. ~~**PT-DOMAINS**~~ **RESOLVED** (April 10 — `ui.sock` + `interaction.sock` symlinks added)
7. ~~**CR-03**~~ **RESOLVED** (Iter 78 — `guard_connection()` with real BearDog RPC, degraded when absent)
8. ~~**BC-GPU-PANIC (BC-05)**~~ **RESOLVED** (Sprint 39 — `Auto::new()` → `Err`, health `Degraded`)
9. ~~**EXP091-REGISTRY**~~ **RESOLVED** (April 10 — `get_family_id()` → `self.family_id`; socket alias mapping)
10. **EXP-TCP-UDS** — exp085/exp090 hardcode TCP ports; need UDS discovery migration
11. **BTSP-E2E** — Full end-to-end BTSP test (non-default FAMILY_ID + FAMILY_SEED) not yet validated against live stack

**Deferred** (later development cycle):
- **SD-01/02/03** — sourDough `deny.toml`, musl, genomeBin signing
- **SKUNKBAT-BTSP-P2** — skunkBat BTSP Phase 2 (Phase 1 + Wire L2 done)

---

## Guideline Compliance Matrix (April 9, 2026)

| Primal | Clippy | Fmt | `deny.toml` | License | Edition | Tests | BTSP P1 | BTSP P2 | Wire |
|--------|--------|-----|-------------|---------|---------|-------|---------|---------|------|
| biomeOS | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (7,724)** ↑ | **PASS** | **PASS** ↑↑ | consumer |
| BearDog | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (14,593)** ↑ | **PASS** | **PASS** | **L2** |
| Songbird | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | **PASS** | **PASS** ↑↑ | **L3** |
| NestGate | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (11,856)** | **PASS** | **PASS** | **L3** |
| petalTongue | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (176)** | **PASS** ↑↑ | stub | **L2** |
| Squirrel | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (7,203)** ↑ | **PASS** | **PASS** ↑↑ | **L2** |
| toadStool | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (21,600)** | **PASS** | **PASS** ↑↑ | **L3** |
| sweetGrass | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | **PASS** | **PASS** ↑↑ | **L3** |
| rhizoCrypt | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | **PASS** | **PASS** ↑↑ | **L3** |
| loamSpine | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | **PASS** | **PASS** ↑↑ | **L2** |
| barraCuda | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (3,899)** | **PASS** | partial ↑ | **L2** |
| sourDough | **CLEAN** | **PASS** | **MISSING** | `-or-later` | 2024 | **PASS (239)** | FAIL | — | NONE |
| coralReef | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | **PASS (4,257)** | **PASS** ↑↑ | scaffold | **L2** ↑ |
| bingoCube | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | N/A | N/A | NONE |
| skunkBat | **CLEAN** | **PASS** | YES | `-or-later` | 2024 | PASS | **PASS** ↑↑ | — | **L2** ↑ |

**Legend**: ↑ = improved since last audit. BTSP P1 = socket naming + insecure guard. BTSP P2 = handshake on accept/client. Wire = Capability Wire Standard level.

### Compliance Evolution (April 9 — BTSP Phase 2 ecosystem cascade)

**BTSP Phase 2 rollout effectively complete.** **11/13** primals enforce full handshake on accept
(+barraCuda Sprint 39, +coralReef Iter 78). 1 has Phase 1 + stub (petalTongue). 1 Phase 1 only (skunkBat).
All 13 primals have Phase 1 (guard + socket naming). **Tower Atomic: 100%. Node Atomic: 100%.
NUCLEUS: 100%.** primalSpring itself: clippy ZERO warnings, fmt PASS, all tests PASS.

1. **Songbird**: **BTSP Phase 2 COMPLETE** ↑↑ (Wave 133) — `perform_server_handshake()` in `ipc/btsp.rs`, wired into UDS accept loop, BearDog delegation via `SecurityRpcClient`. `BtspClient` + connection managers.
2. **ToadStool**: **BTSP Phase 2 COMPLETE** ↑↑ (S198) — `BtspServer::accept_handshake` on JSON-RPC Unix + tarpc paths, feature-gated. `BtspClient`. Fuzz targets (`fuzz_btsp_framing.rs`).
3. **barraCuda**: **BTSP Phase 2 COMPLETE** ↑↑ (Sprint 39) — `guard_connection()` full 6-step handshake relay in all 3 accept loops. BearDog delegation via capability-based `crypto` socket discovery. Legacy clients degrade (2s timeout).
4. **rhizoCrypt**: **BTSP Phase 2 COMPLETE** ↑↑ (S31) — `BtspServer::accept_handshake` in UDS accept. Local crypto (self-sovereign — HKDF/X25519/HMAC-SHA256, no BearDog delegation).
5. **loamSpine**: **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake` in UDS accept, BearDog delegation (`btsp.session.create/verify/negotiate`). Mock tests.
6. **sweetGrass**: **BTSP Phase 2 COMPLETE** ↑↑ — `perform_server_handshake` in UDS + TCP accept, BearDog delegation. Client `perform_handshake` in integration crate.
7. **petalTongue**: **BTSP Phase 1 COMPLETE** ↑↑ — new `btsp.rs` module: guard, family-scoped sockets, domain symlinks. Phase 2 stub (warn-only).
8. **coralReef**: **BTSP Phase 2 COMPLETE** ↑↑ (Iter 78) — `guard_connection()` in all 3 crates (async core/glowplug, blocking ember). Real UDS RPC to BearDog `btsp.session.create`. Degraded mode when provider absent. **Wire Standard L2** ↑ (`capability.list` + flat `methods`). 7 large files split into modules, typed driver errors (Waves 1–3).
9. **skunkBat**: **JSON-RPC IPC server from scratch** ↑↑ + **BTSP Phase 1 COMPLETE** + **Wire Standard L2**. Phase 2 not started.
10. **BearDog**: Wave 33 — **BD-01 RESOLVED** (per-field encoding hints + semantic aliases). 90.51% coverage. 14,593+ tests. `#[allow(` 193→75. `runtime.rs` 1244→360 LOC. Dynamic `ipc.register`. Standalone startup (`standalone-{uuid}`). 0 files over 1000 LOC. Minor: `btsp.negotiate` vs `btsp.session.negotiate` metadata inconsistency.
11. **Squirrel/biomeOS/NestGate**: Phase 2 complete (wave 2, unchanged).

---

## Resolved Gaps Summary

| ID | Primal | What Was Fixed | Resolved In |
|----|--------|---------------|-------------|
| BM-01–05 | biomeOS | Graph routing, health, discovery, multi-shape | v2.79–v2.81 |
| BC-01–04 | barraCuda | Fitts/Hick/Perlin fixes, plasmidBin harvest | Sprint 25 |
| PT-01–03, PT-05, PT-07 | petalTongue | Socket, SSE, motor_tx, awareness init, server discovery | IPC compliance evolution |
| SQ-01–03 | Squirrel | Filesystem socket, `LOCAL_AI_ENDPOINT`, feature flag docs | alpha.25b–27 |
| SB-01 | songBird | `health.liveness` canonical | wave89-90 |
| NG-04–05 | NestGate | ring/aws-lc-rs eliminated, crypto delegated to BearDog | deep debt evolution |
| RC-01 | rhizoCrypt | UDS transport + biomeos/ path | v0.14.0-dev s23 |
| LS-03 | loamSpine | Startup panic → graceful degradation | v0.9.15 |
| LS-04 | loamSpine | Witness wire evolution (`WireWitnessRef` in `trio_types.rs`) | v0.9.16 |
| RC-02 | rhizoCrypt | Witness wire evolution (`WireWitnessRef`, evidence, kind) | v0.14.0-dev |
| SG-01 | sweetGrass | Witness wire evolution (`Witness`, `EcoPrimalsAttributes.witnesses`) | v0.7.27 |

| TS-01 | toadStool | coralReef `capability.discover` | S173-2 |
| PT-04 | petalTongue | HTML graph export | deep debt evolution |
| PT-06 | petalTongue | callback_tx push notifications | deep debt evolution |
| NG-01 | NestGate | FileMetadataBackend enforced in production | April 9 (d65ee214) |
| NG-03 | NestGate | `data.*` wildcard delegation (NCBI/NOAA stubs replaced) | April 9 (d65ee214) |
| PT-08 | petalTongue | BTSP Phase 1 (guard + family-scoped sockets) | April 9 (4544f96) |
| CR-01 | coralReef | BTSP Phase 1 (`validate_insecure_guard` in glowplug/core/ember) | April 9 (4f03cbf) |
| CR-02 | coralReef | Wire Standard L2 (`capability.list` + flat `methods`) | April 9 (4f03cbf) |
| BD-01 | BearDog | Per-field encoding hints for `crypto.verify_ed25519` + semantic aliases | April 9 (834bcbc — Wave 33) |
| NG-06 | NestGate | `--socket` CLI flag wired through dispatch → `NESTGATE_SOCKET` env var | April 10 (NUCLEUS patterns) |
| TS-02 | toadStool | JSON-RPC socket separated from tarpc (`compute.jsonrpc.sock`) | April 10 (NUCLEUS patterns) |
| TS-03 | toadStool | `--socket` CLI flag wired to `run_server_main` | April 10 (NUCLEUS patterns) |
| — | primalSpring | BTSP client handshake (`btsp_handshake.rs`) + Transport auto-detection | April 10 (NUCLEUS patterns) |
| BM-07 | biomeOS | Registry routing — `get_family_id()` → `self.family_id` + socket alias mapping | April 10 (registry fix) |
| BM-08 | biomeOS | Plain socket fallback for primals without `--socket` | April 10 (socket resolution) |
| BM-09 | biomeOS | JSON-RPC socket preference over tarpc for `capability.call` | April 10 (socket resolution) |
| — | primalSpring | `NeuralBridge::discover()` checks both `neural-api-` and `biomeos-` sockets | April 10 (NeuralBridge fix) |
| LS-03 | loamSpine | Startup crash reconciled — `deployment_matrix.toml` `loamspine-startup-crash` marked resolved (was v0.9.15) | April 10 (gap audit) |
| BM-11 | biomeOS | ToadStool dual-socket: `prefers_jsonrpc` in `socket.rs` + `.jsonrpc.sock` sibling in `path_builder.rs` | April 10 (NUCLEUS polish) |
| SG-02 | sweetGrass | `--socket` CLI flag → `start_uds_listener_at()` / `cleanup_socket_at()` | April 10 (NUCLEUS polish) |
| — | primalSpring | `extract_capability_names` handles `capabilities`/`methods` wrappers; caps-first health ordering | April 10 (NUCLEUS polish) |
| — | primalSpring | `strip_unix_uri` made public; exp077 `ai_health_routed` uses direct socket probe | April 10 (NUCLEUS polish) |
| — | nucleus_launcher | Capability domain symlinks + primal family-alias symlinks + stale socket cleanup | April 10 (NUCLEUS polish) |
| BC-05 | barraCuda | GPU panic → graceful `Degraded` (`Auto::new()` returns `Err`, no panic) | Sprint 39 (April 10 pull) |
| CR-03 | coralReef | BTSP Phase 2 — `guard_connection()` with real BearDog RPC in all 3 crates | Iter 78 (April 10 pull) |
| CR-04 | coralReef | Typed driver errors Waves 1–3 (`PciDiscoveryError`/`ChannelError`/`DevinitError`) | Iter 78 (April 10 pull) |
| TS-04 | toadStool | `ollama.*`/`inference.*` semantic mappings removed — inference is Squirrel's domain | April 10 (inference abstraction) |

**50 gaps resolved** across the full cycle (includes LS-03 reconciliation, BC-05 GPU panic, CR-03 BTSP Phase 2, TS-04 inference cleanup). **10 open** (0 high, 2 medium, 8 low).
3 downstream items (ludospring-ipc-surface, ludospring-plasmidbin, esotericwebb-transport) reclassified — not NUCLEUS scope.
10 build/test debt items resolved (April 6). 3 trio witness wire gaps (April 7).
**April 9 wave 1**: PT-08/PT-09, CR-01/CR-02/CR-03 added.
**April 9 wave 2**: NG-01, NG-03 RESOLVED. Squirrel/NestGate/biomeOS BTSP Phase 2 COMPLETE.
**April 9 wave 3**: PT-08, CR-01, CR-02 RESOLVED. Ecosystem-wide BTSP Phase 2 cascade.
**April 9 wave 4**: BD-01 RESOLVED (Wave 33 — encoding hints + semantic aliases).
**April 10 rebuild**: PLASMIBIN-STALE **RESOLVED** — full musl-static rebuild (12/12 ecoBin).
**April 10 NUCLEUS patterns**: NG-06 (NestGate `--socket`), TS-02 (socket separation),
TS-03 (`--socket` wiring), NEURAL-API-DOUBLE-PREFIX all **RESOLVED**. primalSpring BTSP
client handshake implemented.
**April 10 registry fix**: BM-07 **RESOLVED** — `get_family_id()` → `self.family_id` in
`defaults.rs`, `mod.rs`, `translation_startup.rs`; socket alias for toadstool→compute,
nestgate→storage in `socket.rs`.
**April 10 socket resolution**: BM-08/BM-09 **RESOLVED** — plain `{primal}.sock` fallback
for primals without `--socket` (loamSpine, sweetGrass, petalTongue); `.jsonrpc.sock`
preferred over tarpc for domain alias forwarding. `NeuralBridge::discover()` updated
to find `biomeos-{family}.sock`. exp091 Routing Matrix: **12/12 ALL PASS**.
C1-C7: **37/38 (97%)**. 72 experiments: **451/498 (90.6%)** ↑↑.
**primalSpring local**: clippy ZERO, fmt PASS, tests PASS.
**April 10 NUCLEUS polish** (session 2): BM-11 **RESOLVED** — explicit JSON-RPC socket
preference in `resolve_primal_socket` (`prefers_jsonrpc` for ToadStool) + `.jsonrpc.sock`
sibling check in `build_socket_path`. SG-02 **RESOLVED** — `sweetGrass --socket` CLI flag.
Launcher: capability domain symlinks + primal family-alias symlinks + stale socket cleanup.
primalSpring: `extract_capability_names` handles `capabilities`/`methods` wrapper keys;
`check_capability_health` calls `capabilities()` before `health_check()` for primals that
close connection after first response (loamSpine). exp077 `ai_health_routed` fixed via
direct Squirrel health probe on discovered AI socket.
**Critical experiments: 13/13 ALL PASS** (exp001/002/003/004/051/069/075/077/079/089/091/092/093).
C1-C7: **37/38 (97%)** — single partial: C2 `ai.query` (Ollama provider config).

---

## Capability-Based Discovery Compliance (April 6, 2026)

Per `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.2: primals MUST discover each other
by capability domain via Neural API, not by hardcoded primal names in routing code.

Scan methodology: `rg -l` for other-primal names in non-test, non-archive, non-target source
files. Self-references excluded. biomeOS excluded from grading (orchestrator — primal-name
awareness is in-domain for routing).

| Primal | Compliance | Other-Primal Files | Env-Var Refs | Trend |
|--------|-----------|-------------------|--------------|-------|
| sourDough | **P→C** | 5 | 2 | Stable |
| bingoCube | **P→C** | 9 | 0 | Stable |
| coralReef | **P→C** | 31 | 17 | Stable |
| NestGate | **P** | 63 | 97 | Improving ↑ |
| loamSpine | **P** | 72 | 155 | Stable |
| sweetGrass | **P** | 93 | 196 | Stable |
| skunkBat | **P** | 66 | 9 | New entry |
| BearDog | **P** | 131 | 147 | New methodology (broader scan) |
| barraCuda | **P** | 136 | 17 | New entry |
| petalTongue | **P** | 168 | 106 | Stable |
| rhizoCrypt | **P** | 127 | 204 | Broader scan vs previous |
| Songbird | **P** | 193 | 329 | Improving ↑ |
| Squirrel | **D** | 205 | 232 | Stable |
| toadStool | **D** | 285 | 203 | Improving ↑ |
| biomeOS | *(orchestrator)* | 458 | 733 | In-domain routing refs |

**Note on methodology change**: This scan uses a broader regex than previous audits
(includes all 15 primal tokens, case-insensitive, across all non-test source files).
Previous audits used a narrower scope, producing lower absolute numbers. Relative
rankings and trends are consistent.

### Discovery Compliance Priority

1. **toadStool** — 285 files (was 384 previous scan). Improving but still highest non-orchestrator.
2. **Squirrel** — 205 files. Many refs are acceptable (logging, aliases, serde compat).
3. **Songbird** — 193 files. Strongest improvement trajectory across audit cycles.
4. **petalTongue** — 168 files. UI backends reference primal names for display/discovery.
5. **NestGate** — 63 files. Near-compliant. Overstep shedding continues.

Full per-primal details: `wateringHole/ECOSYSTEM_COMPLIANCE_MATRIX.md` §Tier 4: Discovery / Self-Knowledge.

---

## Overstep Audit (April 2, 2026)

Cross-referenced against `wateringHole/PRIMAL_RESPONSIBILITY_MATRIX.md`. No new boundary violations found.

| Primal | Overstep Status | Detail |
|--------|----------------|--------|
| biomeOS | **Known** | `redb` in `biomeos-graph` (metrics storage) — borderline operational state vs NestGate domain |
| BearDog | **Known** | `axum` in `beardog-integration` (HTTP); AI/neural tree in `beardog-core` (~36 files) |
| Songbird | **Known** | `sled` persistence in orchestrator/sovereign-onion (SB-03, now feature-gated) |
| NestGate | **Known** | Crypto, discovery, network, MCP, orchestration — all documented in matrix; `nestgate-security` crypto delegated to BearDog (NG-05 RESOLVED) |
| toadStool | **Cleaned (S169)** | 30+ methods + 10,659 lines removed; only TS-01 (coralReef discovery) + security sandbox remain |
| sweetGrass | **CLEAN** | Own storage crates (sled/redb) are in domain |
| rhizoCrypt | **CLEAN** | TCP listener is standard dual-mode IPC per protocol, not networking overstep |
| loamSpine | **CLEAN** | TCP/HTTP listeners are standard IPC dual-mode per protocol |
| petalTongue | **CLEAN** | axum serves visualization UI (in domain); no embedded compute/storage/AI |
| Squirrel | **Known** | `sled`/`sqlx` persistence, `ed25519-dalek`/TLS — documented; broader than "cache only" |

---

## plasmidBin Inventory (SUPERSEDED — see BTSP tables above for live status)

Moved to "plasmidBin Binary Inventory (April 9, 2026)" section above.
See `infra/plasmidBin/doctor.sh --quick` for live status.

---

## primalSpring Rewiring Status (April 1, 2026)

| Area | Status |
|------|--------|
| `methods.rs` | Aligned — `graph.execute`, `topology.rescan`, `ember.*`, `shader.compile` removed, `ai.*`, `visualization.*`, `interaction.*` added |
| `NeuralBridge` | Aligned — `topology_rescan()` added, `graph.execute` call correct |
| `discover.rs` | Aligned — plain socket name discovery (`{name}.sock`, `{name}-ipc.sock`) added |
| `capability.rs` | Aligned — 4-format parsing, `strip_unix_uri`, multi-shape |
| `validate_compositions.py` | Aligned — SQ-02 messaging updated, NestGate `family_id`, C7 Squirrel check live |
| Composition graphs (C1–C7) | Clean — no stale references |
| Cargo.toml | `edition = "2024"`, `rust-version = "1.87"` |
| Tests | 403 pass (10/10 unit, 4/4 doc-tests) |

---

## Live Validation Results (April 10, 2026 — NUCLEUS polish session 2)

### Deployment Method

biomeOS Neural API (`--family-id default`) with all 10 NUCLEUS primals from freshly rebuilt
musl-static plasmidBin (April 10). Startup via `nucleus_launcher.sh` with stale socket
cleanup, capability domain symlinks, and primal family-alias symlinks. Neural API socket
at `biomeos-default.sock`, primals at standard `{primal}-default.sock` or capability-domain
sockets. ToadStool with separated JSON-RPC/tarpc sockets (BM-11). biomeOS rebuilt with
registry routing (BM-07), plain fallback (BM-08), JSON-RPC preference (BM-09/BM-11).

### Composition Validation (C1–C7)

```
  C1: Render                           6/6   PASS
  C2: Narration                        3/4   PARTIAL (ai.query — no API keys configured)
  C3: Session Readiness                5/5   PASS
  C4: Game Science Readiness           5/5   PASS
  C5: Persistence                      5/5   PASS ↑↑ (was FAIL — NestGate --socket wired)
  C6: Proprioception                   5/5   PASS
  C7: Product Readiness                8/8   PASS ↑ (NestGate now discoverable)

  TOTAL                                37/38  (97%) ↑↑
```

### Experiment Results (72 experiments, full suite)

Key results (13 critical experiments):

| Experiment | Checks | Result | Notes |
|---|---|---|---|
| exp001 Tower Atomic | 13/13 | **ALL PASS** | Full tower composition |
| exp002 Node Atomic | 13/13 | **ALL PASS** | Full node composition |
| exp003 Nest Atomic | 17/17 | **ALL PASS** | NestGate `--socket` wiring RESOLVED |
| exp004 Full NUCLEUS | **29/29** | **ALL PASS** ↑↑↑ | Was 26/29 → **29/29**. Domain aliases + JSON-RPC socket preference + caps-first ordering |
| exp051 Socket Discovery | 4/4 | **ALL PASS** | All expected sockets found |
| exp069 Graph Overlay | 25/25 | **ALL PASS** | Full graph composition |
| exp075 Neural API Live | **12/12** | **ALL PASS** ↑ | Was 11/12 → **12/12**. Birdsong beacon now routing |
| exp077 Squirrel Bridge | **5/5** | **ALL PASS** ↑ | Was 4/5 → **5/5**. Health routing via discovered socket |
| exp079 Spring Deploy | 24/24 | **ALL PASS** | Full deployment sweep |
| exp089 BearDog Witness | 15/15 | **ALL PASS** | WireWitnessRef full round-trip |
| exp091 Routing Matrix | **12/12** | **ALL PASS** ↑↑↑ | All 10 NUCLEUS capability domains route correctly via biomeOS |
| exp092 Dual Tower Ionic | 18/18 | **ALL PASS** | Full ionic bond |
| exp093 Covalent Mesh | 22/22 | **ALL PASS** | Full covalent mesh backup |

**13/13 critical experiments: ALL PASS** (April 10 session 2)

### Root Causes of Remaining Failures

| Category | Impact | Root Cause |
|---|---|---|
| ~~biomeOS registry socket paths~~ | ~~exp091~~ | **RESOLVED** — all 10 domains route correctly (12/12) |
| ~~Socket resolution for plain sockets~~ | ~~loamSpine, sweetGrass, petalTongue~~ | **RESOLVED** — plain `{primal}.sock` fallback in BM-08 |
| ~~JSON-RPC vs tarpc forwarding~~ | ~~compute domain~~ | **RESOLVED** — `.jsonrpc.sock` preferred in BM-09 + BM-11 (ToadStool dual-socket) |
| ~~No AI API keys~~ | ~~C2 partial~~ | **RESOLVED** — Squirrel OpenAI adapter + Ollama `tinyllama-cpu` (SQ-03/04/05) |
| ~~Harness binary discovery~~ | ~~exp004~~ | **RESOLVED** — domain aliases in launcher + `prefer_jsonrpc_socket()` in primalSpring (BM-11) |
| ~~Birdsong beacon forwarding~~ | ~~exp075~~ | **RESOLVED** — birdsong beacon now routing through biomeOS substrate |
| ~~AI health routing~~ | ~~exp077~~ | **RESOLVED** — discover AI socket via biomeOS, probe health directly |
| LAN probe | exp090 (1 failure) | Multi-node LAN test expects additional peers; single-gate dev stack |

### Comparison

| Date | Composition | Experiments | Deployment | Binaries |
|---|---|---|---|---|
| April 1 | 43/44 (98%) | — | Manual UDS startup | glibc dynamic |
| April 10 (pre-rebuild) | 31/34 (91%) | 92/101 (91%) | Graph-based (biomeOS Neural API) | glibc dynamic (Apr 8) |
| April 10 (post-rebuild) | 31/34 (91%) | 95/117 (81%) | Fresh musl-static, Neural API | musl-static (Apr 10) |
| April 10 (NUCLEUS) | 37/38 (97%) | 79/101 (78%) | NUCLEUS patterns deployed | musl-static + socket fixes |
| April 10 (registry fix) | 37/38 (97%) | 400/458 (87%) | Registry routing fixed | musl-static + BM-07 |
| April 10 (full routing) | 37/38 (97%) | 451/498 (90.6%) ↑↑ | All primals routable + NeuralBridge fix | musl-static + BM-07/08/09 |
| April 10 (AI online) | 38/38 (100%) | 71/72 (98.6%) ↑↑ | Squirrel AI via Ollama + all fixes | musl-static + SQ-03/04/05 |
| **April 10 (polished)** | **37/38 (97%)** | **13/13 critical ALL PASS** ↑↑↑ | **Full domain routing + stale cleanup + biomeOS JSON-RPC preference** | **musl-static + BM-11 + SG-02** |

exp091 Routing Matrix: 0/1 → 4/12 → **12/12 ALL PASS** (all 10 NUCLEUS capability domains).
`capability.call` verified end-to-end for all domains: crypto→BearDog, discovery→Songbird,
compute→ToadStool, storage→NestGate, ai→Squirrel, dag→rhizoCrypt, spine→loamSpine,
braid→sweetGrass, http→Songbird, mesh→Songbird.

Squirrel AI chain: Squirrel → OpenAI adapter → Songbird `http.request` → Ollama → `tinyllama-cpu`.
Only remaining failure: exp090 (LAN probe — multi-node test on single-gate dev stack).

### Next Steps

1. ~~**PLASMIBIN-REBUILD**~~: **DONE** — all 12 primals rebuilt as musl-static (Apr 10)
2. ~~**NestGate UDS**~~: **DONE** — `--socket` CLI flag wired (Apr 10)
3. ~~**ToadStool JSON-RPC UDS**~~: **DONE** — socket separation + `--socket` wiring (Apr 10)
4. ~~**Neural API routing fix**~~: **DONE** (prior session) — double-prefix stripped in `capability.call`
5. ~~**BTSP client handshake**~~: **DONE** — `btsp_handshake.rs` in primalSpring (Apr 10)
6. ~~**biomeOS registry socket paths**~~: **DONE** — `get_family_id()` → `self.family_id` + socket alias mapping (Apr 10)
7. ~~**Socket resolution fallback**~~: **DONE** — plain `{primal}.sock` fallback for loamSpine/sweetGrass/petalTongue (Apr 10, BM-08)
8. ~~**JSON-RPC socket preference**~~: **DONE** — `.jsonrpc.sock` preferred over tarpc for domain aliases (Apr 10, BM-09)
9. ~~**NeuralBridge discovery**~~: **DONE** — checks both `neural-api-{family}.sock` and `biomeos-{family}.sock` (Apr 10)
10. ~~**exp091 routing matrix**~~: **DONE** — 12/12 ALL PASS with correct method names + routing-success scoring (Apr 10)
11. ~~**Squirrel AI provider chain**~~: **DONE** — Squirrel rebuilt with `deprecated-adapters`, Neural API discovery fix (`primary_endpoint`), local-ai fallback fix, `OPENAI_DEFAULT_MODEL` env var (SQ-03/04/05, Apr 10)
12. ~~**biomeOS method name translation (BM-10)**~~: **DONE** — `query_ai` → `ai.query` in `defaults.rs`, `capability_registry.toml`, `inference.rs`, 18 deploy graphs (Apr 10)
13. ~~**petalTongue domain registration**~~: **DONE** — `ui.sock` + `interaction.sock` symlinks alongside `visualization.sock` (Apr 10)
14. ~~**loamSpine/petalTongue `--socket` wiring**~~: **DONE** — `--socket` CLI flag added to both (loamSpine via `run_server` param, petalTongue via `with_socket_path()`) (Apr 10)
15. ~~**sweetGrass `--socket` wiring (SG-02)**~~: **DONE** — `--socket` CLI flag plumbed via `start_uds_listener_at()` / `cleanup_socket_at()` (Apr 10)
16. ~~**ToadStool dual-socket resolution (BM-11)**~~: **DONE** — `prefers_jsonrpc` in `socket.rs`, `.jsonrpc.sock` sibling in `path_builder.rs` (Apr 10)
17. ~~**exp004 Full NUCLEUS 29/29**~~: **DONE** — domain aliases in launcher + caps-first health ordering + `extract_capability_names` format E/F (Apr 10)
18. ~~**exp075 birdsong beacon**~~: **DONE** — 12/12 ALL PASS (Apr 10)
19. ~~**exp077 Squirrel Bridge**~~: **DONE** — 5/5 ALL PASS, `ai_health_routed` via direct socket probe (Apr 10)
20. ~~**Launcher polish**~~: **DONE** — capability domain symlinks, family-alias symlinks, stale socket cleanup (Apr 10)
21. ~~**barraCuda GPU fallback (BC-05)**~~: **DONE** — Sprint 39: `Auto::new()` → `Err`, server starts with `device = None`, health `Degraded` (Apr 10 pull)
22. ~~**ToadStool AI dispatch wiring (TS-04)**~~: **DONE** — `ollama.*` / `inference.*` semantic mappings removed from `mappings_extended.rs`. Inference is Squirrel's domain via ecoPrimal wire standard (Apr 10).
23. **BTSP end-to-end**: Full test with non-default FAMILY_ID + FAMILY_SEED against live stack
24. **Ollama CUDA**: Ollama service CUDA OOM on `llama3.2:1b/3b` and `phi3`; only `tinyllama-cpu` works. GPU models need Ollama service restart with `OLLAMA_NUM_GPU=0` or GPU driver fix.
25. ~~**Inference provider abstraction**~~: **DONE** — Vendor-agnostic `inference.*` wire standard defined in `ecoPrimal/src/inference/` (types + discovery client). Squirrel bridges `inference.complete`/`embed`/`models` via `AiRouter`. neuralSpring exposes stub handlers on JSON-RPC socket. ToadStool cleaned up (TS-04). See `ai_inference_provider_abstraction` plan (Apr 10).
26. ~~**Inference composition nucleation**~~: **DONE** — Proto-nucleate graphs created in primalSpring for neuralSpring to absorb. ML inference is a WGSL shader composition of coralReef + toadStool + barraCuda; neuralSpring is the application layer. `neuralspring_inference_proto_nucleate.toml` (BYOB composition), `neuralspring_inference_pipeline.toml` (shader pipeline), `neuralspring_deploy.toml` updated with `inference.*` capabilities + `coralreef` dependency. See `nucleate_ml_inference_graphs` plan (Apr 10).
27. ~~**hotSpring QCD nucleation**~~: **DONE** — Proto-nucleate + pipeline graphs for lattice QCD as WGSL shader composition. Proton-heavy: metallic GPU fleet + df64 + provenance. Deploy updated with QCD capabilities + required shader primals. Missing launch profile added. See `spring_proto-nucleate_evolution` plan (Apr 10).
28. ~~**healthSpring enclave nucleation**~~: **DONE** — Dual-tower enclave proto-nucleate + clinical pipeline. Neutron-heavy: ionic bond between patient enclave (Tower A) and analytics tower (Tower B). BondingPolicy egress fence. Deploy updated with clinical capabilities + required NestGate/Squirrel/provenance. See `spring_proto-nucleate_evolution` plan (Apr 10).

---

## Inference Provider Abstraction (April 10, 2026)

Vendor-agnostic inference wire standard decoupling the ecosystem from Ollama/CUDA.

**Architecture**: `ecoPrimal/src/inference/` defines the ecosystem-level contract:
- `CompleteRequest`/`CompleteResponse` — text/chat completion
- `EmbedRequest`/`EmbedResponse` — embedding generation
- `ModelInfo`/`ModelsResponse` — model discovery
- `ProviderInfo` — routing metadata (locality, latency, cost)
- `InferenceClient` — discovery + typed JSON-RPC client over UDS

**Wire methods**: `inference.complete`, `inference.embed`, `inference.models` (JSON-RPC 2.0).

**Provider discovery**: `INFERENCE_PROVIDER` env var → `inference.sock` → `squirrel.sock` → family-suffixed sockets.

**Changes delivered**:

| Component | What Changed |
|-----------|-------------|
| **ecoPrimal** | New `inference` module: wire types (`types.rs`), discovery client, `INFERENCE_PROVIDER` env resolution |
| **Squirrel** | `inference.complete`/`embed`/`models` dispatch routes in `jsonrpc_server.rs`, handler bridge in `handlers_inference.rs`, capability advertisement in `niche.rs` |
| **ToadStool** | `ollama.*`/`inference.*` semantic mappings removed from `mappings_extended.rs` (TS-04 RESOLVED) |
| **neuralSpring** | `inference.complete`/`embed`/`models` handlers in `inference.rs`, dispatch routes in `main.rs`, capabilities registered in `niche.rs`/`config.rs`/`capability_registry.toml` |

**Composition nucleation** (April 10 — proto-nucleate graphs for neuralSpring):

primalSpring defines the composition targets that neuralSpring absorbs and evolves against.
No neuralSpring files are modified — primalSpring is the composition evolution environment.

Key insight: ML inference is a **WGSL shader composition** of existing NUCLEUS primals.
barraCuda's 826 WGSL shaders already provide the math (matmul, attention, softmax, gelu).
coralReef compiles the shader programs. toadStool dispatches them to GPU/CPU. neuralSpring
is the **application layer** that composes these primals into inference pipelines — and
helps evolve tokenization as shader operations within the primals themselves.

| Graph | Purpose |
|-------|---------|
| `graphs/downstream/neuralspring_inference_proto_nucleate.toml` | Core BYOB composition: Tower + coralReef (shader compile) + toadStool (compute dispatch) + barraCuda (tensor shaders) + Squirrel (inference routing) + NestGate (weight cache) + neuralSpring (shader composition application). Phase 0–5 BYOB pattern. |
| `graphs/neuralspring_inference_pipeline.toml` | Pipeline data flow: Squirrel → neuralSpring (compose) → coralReef (compile WGSL) → toadStool (dispatch) → barraCuda (tensor shaders) → NestGate (cache). ML equivalent of `coralforge_pipeline.toml`. |
| `graphs/spring_deploy/neuralspring_deploy.toml` | Updated: `inference.complete`/`embed`/`models` added to neuralSpring's capability surface. `coralreef` + `barracuda` + `squirrel` added to `depends_on`. |

**What neuralSpring absorbs and evolves** (all via WGSL shader composition):
- Compose barraCuda's matmul + attention + softmax shaders into transformer forward passes
- Evolve tokenization as WGSL shader operations within the primals (tokenizer kernels, BPE shaders)
- Use coralReef to compile new WGSL for model-specific ops (rotary embeddings, flash attention, KV-cache kernels)
- Use toadStool to schedule multi-stage shader pipelines across GPU/CPU substrates
- Wire `inference.complete`/`embed`/`models` handlers as shader composition orchestration
- Register as inference provider via `capability.announce` (Squirrel discovers dynamically)
- Load safetensors model weights via NestGate for shader parameter initialization

**Remaining** (future work — neuralSpring absorbs the above; ecosystem-level):
- neuralSpring shader composition wiring: compose existing WGSL ops into tokenization + forward pass pipelines
- `inference.embed`: Squirrel handler returns method-not-found until embedding provider registered
- Direct Ollama adapter: bypass Songbird proxy hop for local inference latency
- Provider health monitoring: track latency/reliability per provider for routing decisions

---

## Spring Evolution Nucleation (April 10, 2026)

Proto-nucleate composition graphs for downstream springs. primalSpring defines the
composition targets; springs absorb them and evolve; patterns flow back up to
primalSpring, which refines and passes requirements upstream to primals as needed.

### hotSpring — Lattice QCD / HPC Physics (proton-heavy)

Lattice QCD is a WGSL shader composition of the same three compute primals
(barraCuda + coralReef + toadStool). barraCuda's shaders provide matmul (SU(3)
gauge links), FFT (momentum-space propagators), and df64 double-precision
emulation critical for QCD. coralReef compiles domain-specific WGSL (Wilson/Dirac
operators, conjugate gradient solvers, HMC integrators). toadStool dispatches to
a metallic GPU fleet pool for multi-GPU lattice partitioning.

| Graph | Purpose |
|-------|---------|
| `graphs/downstream/hotspring_qcd_proto_nucleate.toml` | Core BYOB: Tower + coralReef (compile QCD WGSL) + toadStool (metallic GPU fleet dispatch) + barraCuda (df64 tensor shaders) + NestGate (gauge config cache) + Provenance trio (reproducibility) + hotSpring (physics application) |
| `graphs/hotspring_qcd_pipeline.toml` | Pipeline: hotSpring (lattice + HMC) → coralReef (compile) → toadStool (dispatch) → barraCuda (execute) → NestGate (store configs) → sweetGrass (provenance) |
| `graphs/spring_deploy/hotspring_deploy.toml` | Updated: `coralreef` + `barracuda` now `required = true`, `depends_on` includes all three shader primals, QCD capabilities added (`physics.lattice_gauge_update`, `physics.hmc_trajectory`, `physics.wilson_dirac`, `compute.df64`) |

**CERN-level deployment** requires metallic bonding (GPU fleet pool, already sketched
in `bonding/metallic_gpu_pool.toml`) + ionic lease (CERN infrastructure provides
GPU capacity via ionic bond with different FAMILY_ID, metered, time-windowed).

**What hotSpring absorbs and evolves**: compose SU(3) gauge field update kernels from
barraCuda shaders, compose df64 precision for Wilson/Dirac evaluation, use toadStool
metallic pool for multi-GPU lattice partitioning, wire ionic bond for CERN cloud lease,
provenance-witness every gauge configuration. **Passes back**: df64 precision requirements,
multi-GPU dispatch patterns, HPC bonding gaps.

### healthSpring — Dual-Tower Enclave (neutron-heavy)

healthSpring has the strongest security requirements in the ecosystem. Patient records
NEVER leave the enclave. The composition uses a dual-tower ionic bond pattern
(evolved from the sketch at `sketches/mixed_atomics/dual_tower_ionic.toml`):

- **Tower A** (FAMILY_A): Patient data enclave — BearDog A + NestGate A (BondingPolicy
  egress fence) + healthSpring (ingest, de-identify, aggregate) + Provenance trio A
  (regulatory audit trail)
- **Tower B** (FAMILY_B): Analytics tower — BearDog B + Squirrel (clinical AI inference
  on de-identified data only) + NestGate B (model weights) + Provenance trio B
- **Ionic bridge**: `capabilities_denied = ["storage.*", "dag.*"]` — only de-identified
  aggregates cross. Metered, time-windowed, fully auditable.

| Graph | Purpose |
|-------|---------|
| `graphs/downstream/healthspring_enclave_proto_nucleate.toml` | Dual-tower BYOB: Tower A (patient enclave) + ionic bridge + Tower B (clinical AI). BondingPolicy egress fence on NestGate A. |
| `graphs/healthspring_clinical_pipeline.toml` | Pipeline: healthSpring (ingest + de-identify) → NestGate-A (enclave) → ionic bridge (aggregates) → Squirrel (clinical AI) → NestGate-B (cache) → sweetGrass (audit) |
| `graphs/spring_deploy/healthspring_deploy.toml` | Updated: clinical capabilities (`health.pharmacology`/`genomics`/`clinical`/`de_identify`/`aggregate`), NestGate + Squirrel + Provenance trio now `required`, `bonding_policy` metadata for enclave pattern |

**What healthSpring absorbs and evolves**: implement dual-tower deployment with separate
FAMILY_IDs, wire BondingPolicy egress fence on NestGate, compose ionic bond for
de-identified aggregate sharing, wire Squirrel inference for clinical AI, covalent mesh
sharding for fault tolerance. **Passes back**: BondingPolicy enforcement gaps, ionic
bridge metering requirements, NestGate enclave mode needs.

### Launch profile gap (RESOLVED)

`config/primal_launch_profiles.toml` was missing `[profiles.hotspring]` — all other
springs had profiles. Added with socket wiring for barraCuda, coralReef, toadStool,
NestGate, BearDog, and biomeOS.

---

## Ecosystem Audit Debt (April 6, 2026)

### License Alignment — **COMPLETE**

All 15 primals now on `AGPL-3.0-or-later`. Zero license debt remaining.

### Build/Test Debt

| Primal | Category | Issue | Status |
|--------|----------|-------|--------|
| barraCuda | ~~compile~~ | ~~E0061~~ `eval_math` decomposition | **FIXED** (Sprint 29) |
| barraCuda | ~~file size~~ | ~~`executor.rs` 1,097 lines~~ → split (max 845 LOC) | **FIXED** ↑ |
| barraCuda | **test** | `fault_injection` SIGSEGV (thread cap added Sprint 29) | Open |
| BearDog | ~~fmt~~ | ~~1 file diff~~ | **FIXED** ↑ |
| Songbird | ~~fmt~~ | ~~2 file diffs~~ | **FIXED** ↑ |
| toadStool | ~~fmt~~ | ~~1,899 diffs~~ → ~~1 diff~~ → **0 diffs** | **FIXED** ↑↑ |
| toadStool | ~~clippy~~ | ~~`manual_let_else`, deprecated `GenericArray`~~ | **FIXED** ↑↑ |
| NestGate | ~~fmt~~ | ~~`migration.rs:189`~~ | **FIXED** |
| coralReef | ~~clippy~~ | ~~7 errors in `coral-gpu` tests~~ | **FIXED** ↑ |
| bingoCube | ~~clippy~~ | ~~15 cast errors~~ | **FIXED** ↑ |
| bingoCube | ~~edition~~ | ~~2021~~ → **2024** | **FIXED** ↑ |
| rhizoCrypt | ~~clippy~~ | ~~5 `doc_markdown` warnings~~ | **FIXED** (39 warnings resolved) ↑ |
| sweetGrass | ~~clippy~~ | ~~1 unused import~~ | **FIXED** ↑ |
| sweetGrass | ~~config~~ | ~~`.cargo/config.toml` target-dir points to `/home/southgate/`~~ | **FIXED** (already cleaned) |
| petalTongue | **test** | 1 flaky test (`test_resolve_instance_id_error_message_invalid`) | Open (passes on retry) |

| ~~NUCLEUS~~ | ~~**plasmidBin**~~ | ~~All x86_64 binaries predate BTSP Phase 2 (Apr 8)~~ | **RESOLVED** (Apr 10 — full musl rebuild) |
| ~~NUCLEUS~~ | ~~**musl**~~ | ~~9/11 x86_64 binaries are DYNAMIC~~ | **RESOLVED** (Apr 10 — 12/12 musl-static) |
| ~~NestGate~~ | ~~**UDS**~~ | ~~`service start` is HTTP-only; no `--socket` flag for UDS listener~~ | **RESOLVED** (Apr 10 — `--socket` flag wired) |
| ~~Neural API~~ | ~~**routing**~~ | ~~`capability.call` double-prefixes method names~~ | **RESOLVED** (Apr 10 — domain prefix strip) |
| petalTongue | ~~**domains**~~ | ~~Only `visualization` symlink created~~ | **RESOLVED** (Apr 10 — `ui.sock` + `interaction.sock` symlinks) |
| ~~ToadStool~~ | ~~**UDS-JSONRPC**~~ | ~~`compute.sock` serves tarpc-only; exp002 expects JSON-RPC~~ | **RESOLVED** (Apr 10 — socket separation) |
| ~~barraCuda~~ | ~~**GPU panic**~~ | ~~`barracuda server` panics without GPU~~ | **RESOLVED** (Sprint 39 — `Auto::new()` → `Err`, server starts with `device = None`, health `Degraded`) |
| ~~biomeOS~~ | ~~**registry routing**~~ | ~~Registry stores `{primal}-{hash}.sock` instead of live socket paths~~ | **RESOLVED** (Apr 10 — BM-07) |

**Resolved this cycle:** 21 build/test debt items (+11 this push: fmt×3, executor split, plasmidBin rebuild, musl compliance, NestGate UDS, ToadStool socket separation, Neural API routing, biomeOS registry routing, barraCuda GPU panic). **Remaining:** 2 (barraCuda SIGSEGV test, petalTongue flaky test).

---

## Downstream Spring/Garden Architecture (April 10, 2026)

Springs and gardens are **consumers** of NUCLEUS, not primals. Key changes:

1. **`nucleus_launcher.sh`**: Removed Phase 5 (ludoSpring/esotericWebb). Launcher now
   Phases 0-4 only (10 core NUCLEUS primals). `find_binary` only searches `plasmidBin/`
   and `primals/` release targets.

2. **`primal_launch_profiles.toml`**: Spring profiles (ludospring, groundspring, etc.)
   reclassified under "DOWNSTREAM SPRING/GARDEN PROFILES — reference only" section.
   Not launched by NUCLEUS, just documented for biomeOS graph wiring reference.

3. **`deployment_matrix.toml`**: `ludospring-ipc-surface` and `esotericwebb-transport`
   marked `scope = "downstream"`. `ludospring-plasmidbin` marked `resolved = true`
   (spring binaries don't belong in plasmidBin). `loamspine-startup-crash` marked
   `resolved = true` (LS-03 fixed in v0.9.15). Topology primal lists cleaned:
   springs/gardens moved to separate `springs`/`downstream` keys.

4. **Proto-nucleate graphs**: `graphs/downstream/` now contains 5 proto-nucleate patterns:
   - `ludospring_proto_nucleate.toml` — game science composition
   - `esotericwebb_proto_nucleate.toml` — narrative composition
   - `neuralspring_inference_proto_nucleate.toml` — ML inference as WGSL shader composition
   - `hotspring_qcd_proto_nucleate.toml` ↑ — lattice QCD / HPC physics as WGSL shader
     composition (proton-heavy: metallic GPU pool + df64 + provenance for reproducibility)
   - `healthspring_enclave_proto_nucleate.toml` ↑ — dual-tower enclave for clinical data
     (neutron-heavy: ionic bond between patient data Tower A and analytics Tower B,
     BondingPolicy egress fence, regulatory provenance audit)

   All NUCLEUS nodes use `spawn = false`. ludoSpring and esotericWebb proto-nucleates now use `composition_model = "pure"` (no downstream binary).
   Pipeline graphs: `neuralspring_inference_pipeline.toml`, `hotspring_qcd_pipeline.toml` ↑,
   `healthspring_clinical_pipeline.toml` ↑.

5. **BYOB template**: `graphs/spring_byob_template.toml` updated with Tower Atomic
   security metadata, bonding policy, `spawn = false` for all NUCLEUS nodes, and notes
   that spring binaries are NOT in plasmidBin.

6. **Stale binary names**: `ludospring_primal` → `ludospring` in remaining science/sketch
   graphs (4 files fixed).

7. **Secure-by-default graph rewiring (April 10)**: All 93 deploy graphs now include
   `[graph.metadata]` with `security_model = "btsp_enforced"`, `transport = "uds_only"`,
   `tcp_ports = 0`. Previously 91/101 graphs were missing security metadata (plaintext-first
   patterns from early evolution). Bonding-specific graphs get appropriate `[graph.bonding_policy]`:
   ionic gets `encryption_tiers.cross_family = "full"`, metallic gets `encryption_tiers.fleet = "hmac_plain"`,
   OrganoMetalSalt gets composite tiers. Multi-node/federation graphs add `federation_transport = "songbird_tcp"`.
   ludoSpring/esotericWebb nodes in 14 science/sketch/gen4 graphs annotated with
   `spawn = true` + `security_model = "tower_delegated"` to clarify downstream boundary.
   Pre-existing metadata fields (trust_model, internal_bond_type, etc.) preserved via merge.

## Composition Evolution (April 9, 2026)

ludoSpring and esotericWebb are **NOT primals and NOT spawnable binaries**. They are
pure compositions of NUCLEUS primals — the graph defines the product, biomeOS executes it.
Their experiments and validation work prove the composition patterns work (Fitts, Flow,
DDA, session lifecycle, provenance) — they are the "papers" that validate graph-driven products.

### What Changed

- **Proto-nucleate graphs rewritten** (`graphs/downstream/`): `ludospring_proto_nucleate.toml`
  and `esotericwebb_proto_nucleate.toml` now have `composition_model = "pure"` and no `spawn = true`
  binary nodes. All capabilities map to existing NUCLEUS primals.
- **10 graphs consolidated**: 8 graphs rewritten to replace ludo/webb binary nodes with
  constituent primals (barraCuda for game math, Squirrel for AI/narration, petalTongue
  for rendering, NestGate for storage, provenance trio for session integrity).
- **7 graphs deleted**: 5 redundant sketches + 2 duplicate compositions.
- **6 canonical fragments** (`graphs/fragments/`): `tower_atomic`, `provenance_trio`,
  `node_atomic`, `nucleus` — documenting the "periodic table" of composition.
- **100% metadata annotation**: Every deploy graph carries `composition_model` and `fragments` (atomic-aligned: `tower_atomic`, `node_atomic`, `nest_atomic`, `meta_tier`, `nucleus`, `provenance_trio`).

### How Capabilities Map

| Old Binary Node | Replacement Primals | Capabilities |
|-----------------|---------------------|-------------|
| `ludospring` | barraCuda + toadStool | GPU math: Fitts, Flow, Perlin, WFC, engagement |
| `ludospring` | Squirrel | AI: DDA, analysis, accessibility |
| `esotericwebb` | Squirrel + petalTongue | AI narration + scene rendering |
| `esotericwebb` | NestGate + provenance trio | Session storage + DAG integrity |

### What This Means for Other Springs

This is the same pattern as neuralSpring (ML inference = composition of WGSL shaders)
extended to game science and narrative products. **Every "product" is a graph. Every graph
is a composition of primals. biomeOS is the CPU.** Springs that want to evolve game science,
narrative, or interactive capabilities should compose the same primals — not build new binaries.
