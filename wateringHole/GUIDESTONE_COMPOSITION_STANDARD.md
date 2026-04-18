# guideStone Composition Standard

**Date**: April 18, 2026
**Version**: v1.0.0
**From**: primalSpring v0.9.15
**License**: AGPL-3.0-or-later
**Reference implementation**: hotSpring-guideStone-v0.7.0

---

## What a guideStone Is

A guideStone is a **self-validating deployable** that carries its own
benchmark. It deploys as a node in a NUCLEUS graph, validates the primal
composition on startup, then serves domain capabilities.

The spring binary proved Python-to-Rust parity (Level 2 — the "Rust proof").
The guideStone proves NUCLEUS parity (Level 5+ — the "primal proof"). After
that, the NUCLEUS runs independently. The spring steps aside.

```
Validation ladder:
  Level 1: Python baseline — peer-reviewed science, documented provenance
  Level 2: Rust validation — spring binary proves faithful port (DONE)
  Level 3: barraCuda CPU — primal WGSL shaders, CPU fallback
  Level 4: barraCuda GPU — sovereign shader execution
  Level 5: guideStone — self-validating NUCLEUS node (THIS)
  Level 6: NUCLEUS deployment — biomeOS deploys, guideStone validates
```

## The 5 guideStone Properties

Every guideStone certifies these properties. They are defined against a
shared standard and instantiated per domain. A guideStone that satisfies
all 5 is **guideStone-ready** for deployment.

### Property 1: Deterministic Output

Same binary produces same results on the same architecture. Cross-substrate
parity (Python vs Rust CPU vs Rust GPU) is validated and documented. No
environment-dependent behavior in the science output.

**Checklist**:
- Same binary, same results on target architecture
- CPU-only path produces full validation output
- GPU parity verified within stated tolerances (when applicable)
- No locale, timezone, or hostname in science output
- Cross-substrate tolerance derivations documented per observable

### Property 2: Reference-Traceable

Every numeric output traces to a paper, standard, or mathematical proof.
References are machine-readable in structured output. No orphan numbers.

**Checklist**:
- Every value has provenance (`paper`, `standard`, or `proof` field)
- References are machine-readable (JSON, not just comments)
- RunManifest embedded in output (timestamp, git commit, engine version)
- Optional telemetry sidecar for streaming observables

### Property 3: Self-Verifying

The artifact verifies its own integrity on execution. Tampered inputs are
detected and reported as errors.

**Checklist**:
- CHECKSUMS file present (SHA-256)
- Checksums validated before execution
- Tampered input detected → non-zero exit
- Domain-specific integrity checks (e.g. ILDG CRC for gauge configs)

### Property 4: Environment-Agnostic

Pure Rust, ecoBin compliant. No runtime dependencies, no network, no sudo,
no GPU required for core validation. Deploys from plasmidBin ecobins on
any clean machine.

**Checklist**:
- ecoBin compliant (pure Rust, zero C deps in application code)
- No downloads, no package managers at runtime
- No sudo required
- CPU-only mode covers full validation output
- No hardcoded paths or platform assumptions
- Static musl targets for release binaries

### Property 5: Tolerance-Documented

Every tolerance has a derivation — physical, mathematical, or precision-
theoretic. No magic numbers. Domain experts can audit the justification
from the structured output alone.

**Checklist**:
- Every tolerance has a derivation in metadata
- Derivations are physical/mathematical (not "it works with this value")
- Justifications are machine-readable (`tolerance_justification` field)
- No unexplained thresholds

---

## The Bare guideStone Principle

Properties 1–5 hold **without any primals running**. The guideStone is
a standalone artifact that validates its domain science using only its
own binary and reference data. This is the strongest reproducibility
guarantee: pull to a clean machine, run the guideStone, get certified
results.

When primals are present (NUCLEUS deployed), the guideStone activates
**additive** capabilities:

| NUCLEUS Feature | What It Adds | Graceful Degradation |
|-----------------|--------------|---------------------|
| BearDog signing | Ed25519 detached signature on validation receipt | Skip — no signature fields |
| rhizoCrypt DAG | Computation trace with merkle root | Skip — no DAG fields |
| toadStool reporting | Silicon performance surface, capability query | Fall back to local GPU enumeration |
| Songbird discovery | Capability-based primal resolution | Fall back to filesystem scan |

Physics/science output is **identical** with or without NUCLEUS. The
NUCLEUS layer adds metadata, signing, and provenance — not math.

---

## guideStone as a NUCLEUS Node

The guideStone deploys as a node in the proto-nucleate graph. biomeOS
launches it alongside primals. It operates in two modes:

### Mode 1: In-Graph (deployment-time validation)

biomeOS starts the NUCLEUS graph. The guideStone node:
1. Discovers available primals via capability-based IPC
2. Runs self-validation benchmarks against its `validation_capabilities`
3. Compares IPC results against Python/Rust baselines
4. If all pass: reports healthy, serves domain capabilities
5. If any fail: exit 1, NUCLEUS deployment is invalid

This is the normal deployment mode. The NUCLEUS is self-validating.

### Mode 2: External (standalone check)

Run the guideStone binary outside a NUCLEUS, on any machine:
- With primals running: validates IPC parity (Level 5 proof)
- Without primals: validates bare properties 1–5 (standalone)
- Exit 0/1/2 semantics: pass/fail/skip (no primals discovered)

This mode is how springs validate during development. It's also how
CI pipelines verify that a NUCLEUS deployment is correct.

---

## The Validation Window

The IPC client that each spring builds (hotSpring's `NucleusContext`,
neuralSpring's `IpcMathClient`, healthSpring's `math_dispatch`, wetSpring's
`rpc_call`) is a **validation window** — temporary tooling to prove the
math works through NUCLEUS before the guideStone takes over.

```
Spring evolution:
  1. Spring binary (lib dep) proves Python → Rust parity    ← Level 2 DONE
  2. Spring IPC client validates Rust → NUCLEUS parity       ← validation window
  3. guideStone binary (IPC only) deploys in NUCLEUS         ← Level 5 target
  4. Spring steps aside; NUCLEUS self-validates and serves   ← Level 6 target
```

Springs keep their `barracuda` library dependency for the Level 2
comparison path. The guideStone binary uses pure IPC (via
`primalspring::composition::CompositionContext` or equivalent discovery).
The library dep stays for development-time parity comparison; the
guideStone deploys without it.

---

## The Universal Pattern

The difference between a hotSpring NUCLEUS and a healthSpring NUCLEUS is
the guideStone's domain science. The deployment pattern is identical:

| Aspect | hotSpring | healthSpring | wetSpring | neuralSpring |
|--------|-----------|-------------|-----------|-------------|
| Domain | QCD physics, GPU compute | Patient data, compliance | Life science, fluids | ML inference, spectral |
| guideStone validates | Plaquette, energy density, flow | Clinical math, dual-tower egress | Hydro, provenance chains | Spectral, eigendecomp, correlation |
| Unique constraint | f64 GPU parity | Dual-tower ionic bridge | Provenance registry | 18 barraCuda surface gaps |
| NUCLEUS primals | Tower + Node + Nest | 2× Tower + Node + Nest (enclave) | Tower + Node + Nest + Meta | Tower + Node + Meta |
| Pattern version | guideStone-v0.7.0 (CERTIFIED) | In progress | V145 validation exists | V133 IpcMathClient exists |

But the deployment, self-validation, graceful degradation, primal
interaction, exit code semantics, and serving pattern are the **same**.

---

## guideStone Readiness Levels

| Level | Meaning |
|-------|---------|
| **0 — Not started** | No guideStone artifact or validation binary |
| **1 — Validation exists** | `validate_primal_proof` or equivalent binary, IPC parity checks |
| **2 — Properties 1–5 documented** | All tolerances justified, provenance traced, determinism verified |
| **3 — Bare guideStone works** | Standalone binary passes all checks without any primals |
| **4 — NUCLEUS guideStone works** | In-graph deployment, self-validates against live primals |
| **5 — Certified** | Cross-substrate parity, full NUCLEUS additive layer, ready for Level 6 |

### Current Spring Readiness

| Spring | Level | Evidence | Blockers |
|--------|-------|----------|----------|
| hotSpring v0.6.32 | **5 — Certified** | guideStone-v0.7.0: all 5 properties, cross-substrate parity (Python/CPU/GPU), NUCLEUS additive layer (BearDog signing, rhizoCrypt DAG, toadStool reporting) | aarch64 CI |
| healthSpring V53 | **1 — Validation exists** | exp122 IPC parity, `math_dispatch.rs` feature-gated routing | Property 4 blocked by patient data compliance (dual-tower enclave needed for bare guideStone); 9/11 methods still library-only |
| neuralSpring V133 | **1 — Validation exists** | `IpcMathClient` (9 methods), `validate_proto_nucleate_capabilities` (7 caps) | 18 barraCuda surface gaps block full self-validation (eigh, Pearson, chi-squared, etc.) |
| wetSpring V145 | **1 — Validation exists** | Exp403 `validate_primal_parity_v1` (5 primals over IPC), 22 CONSUMED_CAPABILITIES | Needs guideStone packaging (properties, checksums, cross-substrate) |
| ludoSpring V44 | **1 — Validation exists** | Four-layer validation (Python→Rust→IPC→primal proof), `validate_primal_proof` binary | Needs guideStone packaging |
| airSpring v0.10.0 | **0 — Not started** | Pre-delta; 90.56% coverage; no NUCLEUS wiring | No IPC client yet |
| groundSpring V124 | **0 — Not started** | Pre-delta; 92% coverage; no NUCLEUS wiring | No IPC client yet |

---

## primalSpring guideStone: Base Composition Certification

primalSpring's guideStone validates **composition correctness** — not domain
science. It certifies that a NUCLEUS deployment is structurally sound,
IPC-healthy, and cryptographically functional. Domain guideStones (hotSpring,
healthSpring, etc.) inherit this base certification and only validate their
own science on top.

### Layered Certification Model

```
Domain guideStone (hotSpring, healthSpring, ...)
  └── inherits → primalSpring guideStone (composition correctness)
                    └── Layer 0: Bare Properties (graph parsing, fragments, manifests)
                    └── Layer 1: Discovery (all primals discoverable)
                    └── Layer 2: Atomic Health (Tower, Node, Nest health)
                    └── Layer 3: Capability Parity (math, storage, shader IPC)
                    └── Layer 4: Cross-Atomic Pipeline (hash → store → retrieve → verify)
                    └── Layer 5: Bonding Model (policy enforcement, trust ordering)
                    └── Layer 6: BTSP + Crypto (hash parity, cipher policy, Ed25519)
```

If the primalSpring guideStone passes, a downstream domain guideStone can
skip re-verifying discovery, health, and crypto — it inherits that
certification and only validates domain science.

### The 6 Layers

| Layer | What It Validates | Bare? |
|-------|------------------|-------|
| **0 — Bare Properties** | All deploy graphs parse, fragments resolve, manifest consistent, bonding types well-formed | Yes |
| **1 — Discovery** | All primals in the graph discoverable via capability scan | No |
| **2 — Atomic Health** | Tower, Node, Nest primals respond to `health.liveness` | No |
| **3 — Capability Parity** | `stats.mean`, `tensor.matmul`, `storage.store/retrieve`, `compile.capabilities` | No |
| **4 — Cross-Atomic Pipeline** | Tower hash → Nest store → retrieve → verify (end-to-end proof) | No |
| **5 — Bonding Model** | All `BondType` variants well-formed, cipher policy ordering, metering | Yes |
| **6 — BTSP + Crypto** | `crypto.hash` determinism, BTSP cipher policy, Ed25519 sign/verify roundtrip | Partial |

### Running as a Pre-Flight

Domain guideStones can optionally call the primalSpring guideStone as a
pre-flight check before their own validation:

```bash
# Pre-flight: ensure composition is sound
primalspring_guidestone
# Exit 0 → proceed with domain validation
# Exit 1 → composition broken, domain validation meaningless
# Exit 2 → no primals, bare checks only
```

### Binary

```bash
# From plasmidBin or cargo
cargo run --bin primalspring_guidestone
PRIMALSPRING_JSON=1 cargo run --bin primalspring_guidestone  # JSON output
```

---

## For Downstream Springs

### Evolving Toward guideStone

1. **You already have the validation binary** (`validate_primal_proof` or
   equivalent). That's Level 1.

2. **Document the 5 properties** for your domain science. Which tolerances
   need justification? Which references need machine-readable provenance?
   What is your cross-substrate parity story? That's Levels 2–3.

3. **Package as a deployable binary** that discovers primals on startup,
   self-validates, then serves domain capabilities. This becomes the
   guideStone node in your proto-nucleate graph. That's Level 4.

4. **Certify cross-substrate parity** (Python vs Rust CPU vs Rust GPU
   where applicable). Run the guideStone in Docker across substrates.
   That's Level 5.

### What the guideStone Binary Looks Like

```rust
use primalspring::composition::{
    CompositionContext, validate_parity, validate_liveness,
    capability_to_primal, method_to_capability_domain,
};
use primalspring::validation::ValidationResult;
use primalspring::tolerances;

fn main() {
    let mut ctx = CompositionContext::from_live_discovery_with_fallback();
    let mut v = ValidationResult::new("myspring guideStone");

    // Property 3: Self-verifying — check binary integrity
    verify_checksums();

    // Bare guideStone: properties 1–5 pass without primals
    validate_bare_properties(&mut v);

    // NUCLEUS additive layer: IPC parity against live primals
    let alive = validate_liveness(
        &mut ctx, &mut v,
        &["tensor", "security", "compute"],
    );
    if alive == 0 {
        // Bare guideStone still passed; NUCLEUS not deployed
        v.finish(); // exit 0 if bare passed, exit 2 if not
        return;
    }

    // Validate domain science through IPC
    for cap in VALIDATION_CAPABILITIES {
        validate_parity(&mut ctx, &mut v, /* ... */);
    }

    // Sign receipt if BearDog is available (additive)
    if ctx.has_capability("security") {
        sign_receipt(&mut ctx, &v);
    }

    v.finish(); // exit 0 if all pass, exit 1 if any fail
}
```

### Manifest Entry

Your entry in `downstream_manifest.toml` now includes guideStone metadata:

```toml
[[downstream]]
spring_name = "myspring"
owner = "mySpring"
domain = "my_domain"
particle_profile = "balanced"
fragments = ["tower_atomic", "node_atomic", "nest_atomic"]
depends_on = ["beardog", "songbird", "toadstool", "barracuda", "coralreef", "nestgate"]
guidestone_binary = "myspring_guidestone"
guidestone_readiness = 1
guidestone_properties = { deterministic = true, traceable = false, self_verifying = false, env_agnostic = true, tolerance_documented = false }
validation_capabilities = [
    "tensor.matmul",
    "stats.mean",
    "compute.dispatch",
    "crypto.hash",
]
```

---

*The guideStone pattern was proven by hotSpring (v0.7.0, 59/59 checks,
10 papers, 3 substrates, all 5 properties certified). This standard
extracts that pattern for ecosystem-wide adoption. The goal: every
NUCLEUS deployment is self-validating. Pull to a clean machine, deploy
from plasmidBin, the guideStone proves the science works.*
