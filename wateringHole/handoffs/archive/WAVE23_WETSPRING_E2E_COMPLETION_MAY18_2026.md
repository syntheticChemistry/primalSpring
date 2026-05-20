# Wave 23: wetSpring E2E Study Completion

**Date:** May 18, 2026
**From:** primalSpring (coordination spring)
**To:** wetSpring team (southGate)
**Priority:** HIGH — first full ecosystem E2E proof
**Blockers:** None — sovereign pipeline live, trio wired, socket hygiene shipped
**License:** AGPL-3.0-or-later

---

## Goal

Complete the first end-to-end ecosystem proof: live science → primal
composition → provenance braids → lithoSpore validation → replication.
This validates the entire Python → Rust → Primal pipeline from mountain
to ocean.

---

## Current State (wetSpring V177)

| Track | Status |
|-------|--------|
| Barrick 2009 (7 clones) | **LIVE** — sovereign Rust pipeline, trio wired, braids shipping |
| Tenaillon 2016 (264 clones) | **QUEUED** — SRA download in progress, `compute.fan_out` ready |
| Ferment transcript braids | **SHIPPING** — lithoSpore wire format, `provenance.export_braid` live |
| Server-side socket hygiene | **DONE** — `unlink` before `bind` in `ipc/server.rs` |
| Consumer-side socket pattern | **NOT YET ABSORBED** — see §2 below |
| Cross-tier parity (Gap #9) | **OPEN** — formal 3-layer proof structure not yet adopted |

---

## 1. Barrick 2009 Completion

### What's Done

- `Exp381`: breseq pipeline via Nest Atomic — 7 SRA runs (SRP001569)
- REL606 reference genome, GPU+CPU hybrid (RTX 4060)
- FM-index → SmithWatermanGpu → Tensor::scan → SnpCallingF64
- Trio wired: rhizoCrypt (DAG), loamSpine (ledger), sweetGrass (attribution)
- `provenance.export_braid` shipping braids to lithoSpore

### What's Remaining

- [ ] **Complete all 7 clones at full depth** (3/7 at full depth per latest commit)
- [ ] **Cross-validate sovereign vs breseq** — mutation counts per clone should
  match breseq 0.40.1 output within documented tolerance
- [ ] **Braid seal** — once all 7 clones complete, seal the session with
  `dag.seal` and export the final aggregate braid
- [ ] **lithoSpore ingestion** — hand off sealed braid for USB artifact inclusion
  (see `WETSPRING_LITHO_USB_HANDOFF_MAY18_2026.md`)

---

## 2. Consumer-Side Socket Pattern (Absorb from primalSpring)

wetSpring's server-side socket hygiene is done (`unlink` before `bind`).
The consumer-side pattern — what happens when wetSpring *discovers* other
primals — needs the connect-probe upgrade.

### Where to Wire It

wetSpring discovers primals for trio calls (rhizoCrypt, loamSpine, sweetGrass)
and for GPU dispatch (barraCuda, toadStool, coralReef). The discovery paths
that need the pattern:

| Discovery Site | File | Current | Target |
|----------------|------|---------|--------|
| Trio discovery | `barracuda/src/ipc/provenance/*.rs` | Socket file-exists check | `socket_is_alive()` connect-probe |
| GPU dispatch | `barracuda/src/ipc/` | Socket file-exists check | `socket_is_alive()` connect-probe |
| Science health | `barracuda/src/bin/validate_science_pipeline.rs` | Direct connect | Add negative cache |

### The Pattern (Python for wetSpring's validation scripts)

```python
import socket, os

_dead_sockets: set[str] = set()

def socket_is_alive(path: str, timeout: float = 0.05) -> bool:
    if not os.path.exists(path) or path in _dead_sockets:
        return False
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(timeout)
        s.connect(path)
        s.close()
        return True
    except (ConnectionRefusedError, FileNotFoundError, OSError):
        _dead_sockets.add(path)
        return False
```

### The Pattern (Rust for wetSpring's IPC code)

```rust
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

pub fn socket_is_alive(path: &Path) -> bool {
    if !path.exists() { return false; }
    UnixStream::connect(path)
        .map(|s| { let _ = s.set_read_timeout(Some(Duration::from_millis(50))); true })
        .unwrap_or(false)
}
```

See `primalSpring/ecoPrimal/src/ipc/discover.rs` for the full implementation
with `DEAD_SOCKET_CACHE`.

### Impact for Tenaillon 264

The 264-clone pipeline calls trio 3× per clone. Without the pattern:
- 264 × 3 × ~100ms per stale socket = **~79 seconds wasted** if trio is down
- With connect-probe: 264 × 3 × ~1ms = **~0.8 seconds** (ECONNREFUSED instant)
- With negative cache: 1 probe + 0 = **~0.05 seconds** total

---

## 3. Tenaillon 2016 (264 Clones) — Next Phase

### Prerequisites

- [ ] Barrick 2009 fully complete (validates pipeline correctness)
- [ ] `compute.fan_out` tested with ≥7 parallel work units (Barrick serves as proof)
- [ ] Consumer socket pattern absorbed (performance-critical for 264 clones)

### Execution Plan

| Phase | What | Validates |
|-------|------|-----------|
| 1. SRA download | 264 FASTQ files from SRA | NestGate content pipeline |
| 2. Reference prep | FM-index build for REL606 | barraCuda GPU primitives |
| 3. Fan-out dispatch | `compute.fan_out` via toadStool | Parallel compute composition |
| 4. Per-clone pipeline | map → pileup → call → trio | Full sovereign pipeline |
| 5. Cross-validate | Sovereign vs Tenaillon 2016 published data | Science parity |
| 6. Braid aggregate | Seal 264-clone session | Provenance completeness |
| 7. lithoSpore handoff | USB artifact with braids + data | E2E ecosystem proof |

### toadStool `compute.fan_out` Contract

toadStool S263 shipped `compute.fan_out` — DAG-aware dispatch with substrate
filtering and per-unit assignment/queuing. wetSpring calls it as:

```json
{
  "jsonrpc": "2.0",
  "method": "compute.fan_out",
  "params": {
    "work_units": [
      { "id": "REL1164M", "substrate": "gpu", "pipeline": "resequencing" },
      { "id": "REL2179M", "substrate": "gpu", "pipeline": "resequencing" }
    ]
  }
}
```

---

## 4. Cross-Tier Parity (Gap #9)

wetSpring Gap #9 asks for a formal 3-layer parity proof:

```
Layer 1: Python baseline (breseq 0.40.1 output)
Layer 2: Rust port (sovereign pipeline output)
Layer 3: Primal composition (same pipeline via IPC)
```

For Barrick 2009, this means:
- **L1 vs L2**: breseq mutation calls ≈ sovereign Rust mutation calls
  (per-clone tolerance documented)
- **L2 vs L3**: local Rust execution ≈ IPC-composed execution
  (bitwise identical — same code, different invocation path)

lithoSpore's parity pattern (`litho parity`) is the reference implementation.
Adopt it for the Barrick 2009 dataset first, then carry forward to Tenaillon.

---

## 5. Composition Dependencies

| Primal | Role in Pipeline | Current Wire |
|--------|-----------------|--------------|
| barraCuda | GPU math (SmithWaterman, Tensor, SNP) | v0.4.0, 72 methods |
| toadStool | `compute.fan_out` dispatch | v0.2.0, S263 |
| coralReef | Shader compilation (if GPU codegen needed) | v0.2.0 |
| rhizoCrypt | DAG session (provenance) | v0.14.0, `dag.dehydrate` |
| loamSpine | Ledger commit (permanent record) | v0.9.16, `ledger.commit` |
| sweetGrass | Attribution (credit allocation) | v0.7.37, braid weaving |
| nestGate | Content storage (SRA data, reference genomes) | `content.put`/`get` |
| biomeOS | NeuralAPI orchestration | v3.61, `composition.deploy` |

---

## 6. Deliverables

| # | Deliverable | Blocks |
|---|-------------|--------|
| D1 | Barrick 2009: 7/7 clones at full depth + sealed braid | D3, D5 |
| D2 | Consumer socket pattern absorbed (Rust IPC + Python scripts) | D4 |
| D3 | Barrick 2009: cross-validated against breseq (L1 vs L2 parity) | D5 |
| D4 | Tenaillon 2016: `compute.fan_out` dispatch running 264 clones | D6 |
| D5 | Barrick 2009: lithoSpore USB artifact with sealed braids | Wave 24 |
| D6 | Tenaillon 2016: complete + sealed + lithoSpore handoff | Wave 24 |

---

## References

- `WETSPRING_SOVEREIGN_PIPELINE_HANDOFF_MAY17_2026.md` — pipeline architecture
- `WETSPRING_LITHO_USB_HANDOFF_MAY18_2026.md` — USB artifact spec
- `WETSPRING_STALE_SOCKET_VALIDATION_MAY18_2026.md` — socket pattern details
- `LITHOSPORE_FERMENT_TRANSCRIPT_BRAID_HANDOFF_MAY17_2026.md` — braid contract
- `WAVE22_COMPOSITION_PATTERNS_HANDOFF_MAY18_2026.md` — ecosystem-wide patterns
- `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.3.0 — socket liveness (§5-6)
