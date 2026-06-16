# Silicon-Atheist Evolution Playground

**Owner**: primalSpring  
**Status**: Spec — Wave 115+ implementation target  
**Date**: June 16, 2026 (Wave 114)  
**Principle**: Universal math over platform theology

---

## Thesis

primalSpring evolves from a validation harness into a **full evolution playground**
where primals are tested against every architecture, OS constraint, and deployment
topology. Any primal behavior that depends on platform assumptions (writable `/var/run`,
UDS availability, x86 alignment, Linux syscall ABI) is **silicon theology** — coupling
to a specific substrate that the universal math doesn't require.

The playground applies selection pressure from diverse deployment targets. Primals that
survive all targets have converged on platform-agnostic patterns. Those that fail reveal
evolution debt for their teams.

This aligns with barracuda's whitePaper thesis: computation is math, not silicon.
The ecoPrimals system must eventually run on bare hardware without Linux, without
a specific ISA, without assumptions about the substrate.

---

## Architecture Constraint Surface

### Tier 1: Current Targets (Wave 114)

| Gate | Arch | OS | Constraints |
|------|------|----|-------------|
| **eastGate** | x86_64-musl | Ubuntu/Pop!_OS | Permissive — baseline |
| **fieldGate** | x86_64-musl | Debian (NUC) | LAN-only, headless, limited RAM |
| **grapheneGate** | aarch64-musl | GrapheneOS | SELinux sock_file denial, EROFS system dirs, mobile memory |
| **ironGate** (VPS) | x86_64-musl | Debian | WAN, firewalled, relay-dependent |

### Tier 2: Near-Term Expansion (Wave 115-120)

| Gate | Arch | OS | Constraints | Owner |
|------|------|----|-------------|-------|
| **flockGate** | x86_64-musl | TBD | WAN + NAT traversal, relay-only | cellMembrane |
| **fieldMouse** | aarch64/riscv | Embedded Linux | ≤256MB RAM, no swap, GPIO | toadStool |
| **chimeraDeploy** | mixed | biomeOS | Multi-arch composition, heterogeneous nodes | biomeOS |

### Tier 3: Endgame (Wave 130+)

| Gate | Arch | OS | Constraints | Notes |
|------|------|----|-------------|-------|
| **bareMetal** | riscv64 | ecoPrimals (native) | No Linux, no libc, hardware-direct | The end state |
| **fpgaGate** | custom | HDL-synthesized | Primals as hardware circuits | barracuda universal math |
| **wasmGate** | wasm32 | Browser/WASI | Sandboxed, no filesystem, message-passing only | Portable distribution |

---

## Selection Pressures by Target

Each deployment target applies specific selection pressures. A primal must survive
ALL pressures from its declared support tier, or it carries evolution debt.

### Pressure Categories

| Category | What it tests | Example failure |
|----------|---------------|-----------------|
| **Filesystem** | Path assumptions, dir creation | Songbird EROFS on grapheneGate |
| **IPC Transport** | UDS/TCP/relay assumptions | SkunkBat UDS-only on SELinux |
| **Memory** | Heap allocation, caching strategies | OOM on fieldMouse |
| **Endianness** | Data serialization assumptions | Binary protocol on mixed-endian mesh |
| **Alignment** | Struct packing, pointer alignment | Misalignment on riscv |
| **Syscall ABI** | Linux-specific calls, ioctl | N/A on bareMetal/wasmGate |
| **Network** | LAN assumption, latency tolerance | Timeout on WAN relay |
| **Concurrency** | Thread availability, async runtime | Single-core on fieldMouse |
| **Time** | Clock monotonicity, NTP assumptions | Drift on isolated embedded |

---

## toadStool: RISC-V Gateway

toadStool already manages GPU compute dispatch. Its evolution path includes
**RISC-V architecture ownership**:

- Cross-compile target: `riscv64gc-unknown-linux-musl`
- Validation: primals must produce identical outputs on riscv64 and x86_64
- Hardware targets: SiFive boards, Milk-V, FPGA softcores
- Role: prove that compute primals (barracuda tensor, rhizocrypt DAG) are
  architecture-independent at the math level

### toadStool Arch Compatibility Matrix

| Primal | x86_64 | aarch64 | riscv64 | wasm32 | bare |
|--------|--------|---------|---------|--------|------|
| beardog | ✅ | ✅ | TBD | TBD | TBD |
| songbird | ✅ | ⚠️ (PID) | TBD | TBD | TBD |
| skunkbat | ✅ | ⚠️ (UDS) | TBD | TBD | TBD |
| barracuda | ✅ | ✅ | TARGET | TARGET | ENDGAME |
| toadstool | ✅ | ✅ | OWNER | TBD | TBD |
| rhizocrypt | ✅ | ✅ | TARGET | TBD | TBD |
| sweetgrass | ✅ | ✅ | TBD | TARGET | TBD |
| squirrel | ✅ | ✅ | TBD | TARGET | TBD |
| petaltongue | ✅ | ✅ | TBD | TBD | TBD |
| loamspine | ✅ | ✅ | TBD | TBD | TBD |
| coralreef | ✅ | ✅ | TBD | TBD | TBD |
| nestgate | ✅ | ✅ | TBD | TBD | TBD |
| cellmembrane | ✅ | ✅ | TBD | N/A | TBD |

---

## fieldMouse: Embedded Chimeric Deployment

fieldMouse represents the smallest viable deployment — a constrained embedded
system where a full NUCLEUS cannot run, but individual primals or micro-compositions
can operate as autonomous agents.

### fieldMouse Constraints

- RAM: 128-512 MB (no room for full 13-primal NUCLEUS)
- Storage: SD card or eMMC (wear leveling matters)
- Network: WiFi or BLE (intermittent, high-latency)
- Compute: Single-core or dual-core ARM/RISC-V
- Power: Battery or solar (sleep states matter)

### fieldMouse Composition: Micro-Atomic

```toml
[composition]
atomic_type = "micro"
primals = ["beardog", "skunkbat"]  # Identity + Defense only
transport = "tcp_minimal"          # No UDS, no federation
mesh = false                       # Standalone agent

[constraints]
max_heap_mb = 64
max_binary_size_mb = 4
no_async_runtime = false           # Allow tokio but single-threaded
no_filesystem_writes = true        # Read-only after deploy
```

### Chimeric Deployment (biomeOS)

A chimeric deployment mixes architectures in a single NUCLEUS mesh:

```
eastGate (x86_64) ──mesh──> grapheneGate (aarch64) ──relay──> fieldMouse (riscv64)
     │                            │                                │
  [full NUCLEUS]          [Tower Atomic]                   [Micro Atomic]
  13 primals               3 primals                        2 primals
```

The mesh protocol must be architecture-agnostic: JSON-RPC over TCP with riboCipher
framing. No binary struct passing, no pointer-width assumptions, no endianness in
the wire format.

---

## Evolution Toward ecoPrimals-as-OS

### Phase 1: Linux Abstraction (Current → Wave 120)

Primals run on Linux but minimize syscall surface:
- No direct filesystem — abstract through capability interfaces
- No `fork()`/`exec()` — static binary, single-process per primal
- No libc dependency — musl static linking already achieved
- Transport: TCP sockets only (most portable)

### Phase 2: WASI/Capability Abstraction (Wave 120-130)

Primals compile to WASM and run under a capability-based runtime:
- No filesystem access unless granted
- No network unless granted
- Memory sandboxed per primal
- IPC through host-provided channels

This proves primals don't need Linux — they need *capabilities*.

### Phase 3: Bare-Metal Runtime (Wave 130+)

A minimal runtime replaces Linux entirely:
- Hardware initialization (MMU, interrupts, timers)
- Primal scheduler (cooperative or preemptive)
- Transport layer (hardware-direct ethernet/serial)
- No processes, no filesystems — primals ARE the processes

This is where ecoPrimals becomes its own OS. The runtime provides:
- Memory isolation between primals (MMU page tables)
- IPC channels (shared memory rings or message queues)
- Hardware abstraction (drivers as specialized primals)
- Boot: firmware → runtime → NUCLEUS composition → mesh

### The barracuda Principle

barracuda's universal math thesis states: any computation that requires a specific
architecture is insufficiently abstract. The endgame is primals expressed as pure
mathematical transformations that can be:
- Interpreted (wasm, JIT)
- Compiled (native binary per arch)
- Synthesized (FPGA bitstream)
- Proven (formal verification)

The same primal logic should produce identical results whether running as an x86
binary, an aarch64 binary, a RISC-V binary, a WASM module, or an FPGA circuit.

---

## primalSpring Playground Implementation

### Validation Scenario Evolution

Current scenarios test one gate at a time. Evolution:

| Level | What | Example |
|-------|------|---------|
| L0 (current) | Single-gate structural | `s_bootstrap_readiness` on eastGate |
| L1 (current) | Single-gate live | `s_ribocipher_acceptance` on eastGate |
| L2 (next) | Cross-gate same-arch | eastGate ↔ fieldGate (both x86_64) |
| L3 (next) | Cross-gate cross-arch | eastGate ↔ grapheneGate (x86→aarch64) |
| L4 (future) | Chimeric mesh | 3+ gates, mixed arch, relay-mediated |
| L5 (endgame) | Bare-metal equivalence | Same scenario, Linux vs ecoPrimals-OS |

### Fitness Function

```rust
pub struct ArchFitness {
    pub primal: &'static str,
    pub targets_declared: Vec<Target>,
    pub targets_passing: Vec<Target>,
    pub debt: Vec<EvolutionDebt>,
}

pub struct EvolutionDebt {
    pub target: Target,
    pub pressure: PressureCategory,
    pub failure: String,
    pub severity: Severity,  // Blocks deployment vs. degraded
}

pub enum Target {
    X86_64Musl,
    Aarch64Musl,
    Riscv64Musl,
    Wasm32Wasi,
    BareMetal(Arch),
}
```

### Cross-Arch Equivalence Testing

For any primal method `f(input) → output`:
```
assert_eq!(f_x86(input), f_aarch64(input));
assert_eq!(f_x86(input), f_riscv64(input));
assert_eq!(f_x86(input), f_wasm(input));
```

If outputs differ, the primal has silicon theology — it's computing something
architecture-dependent rather than universal math.

---

## Sourdough Convergence Implications

Patterns that survive all targets graduate to `sourdough_core`:
- Transport: riboCipher framing (architecture-neutral wire format)
- Serialization: JSON-RPC (text-based, endianness-free)
- State: capability-based access (no filesystem assumption)
- Concurrency: message-passing (no shared-memory assumption)

Primals don't import sourdough — they converge on it. The playground proves
which patterns are truly universal by subjecting them to every constraint.

---

## Milestones

| Wave | Milestone | Validates |
|------|-----------|-----------|
| 114 | grapheneGate pipeline proven | aarch64 + SELinux survival |
| 115 | fieldMouse micro-composition | Embedded constraints |
| 116 | Cross-gate mesh scenario (L3) | Architecture-crossing IPC |
| 118 | WASI compilation target | No-OS capability abstraction |
| 120 | riscv64 toadStool gate | Third arch equivalence |
| 125 | Chimeric deployment (L4) | Mixed-arch mesh |
| 130 | Bare-metal prototype | ecoPrimals without Linux |
| 135 | FPGA primal synthesis | Hardware-level math |

---

## References

- `whitePaper/gen4/architecture/COMPOSITION_PATTERNS.md` — barracuda universal math
- `specs/CROSS_GATE_GRAPH_EXECUTOR.md` — cross-gate graph execution
- `specs/NUCLEUS_VALIDATION_MATRIX.md` — current validation tiers
- `handoffs/primalSpring/AAR_WAVE114_GRAPHENEGATE_ADB_DEPLOYMENT_JUN16_2026.md` — grapheneGate field data
- `handoffs/primalSpring/GENETICS_ARCHITECTURE_EUKARYOTIC_MODEL_JUN16_2026.md` — signal transport model
