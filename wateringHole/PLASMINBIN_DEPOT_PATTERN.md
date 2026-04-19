# plasmidBin Depot Pattern — Remote NUCLEUS Deployment

**Date**: April 20, 2026
**From**: primalSpring v0.9.15
**License**: AGPL-3.0-or-later

---

## What plasmidBin Is

`plasmidBin` is the binary distribution channel for ecoPrimals. It
contains static musl ELF binaries for all NUCLEUS primals — stripped,
cross-compiled, and verified via BLAKE3 checksums. Any machine that
clones plasmidBin can launch a NUCLEUS without compiling anything.

```
plasmidBin/
├── primals/           # 14 static ELF binaries (13 primals + primalspring_primal)
│   ├── beardog        # 7.5M — Tower: crypto spine
│   ├── songbird       # 17M  — Tower: discovery + HTTP
│   ├── toadstool      # 11M  — Node: compute dispatch
│   ├── barracuda      # 5.0M — Node: GPU math (WGSL shaders)
│   ├── coralreef      # 6.8M — Node: shader compiler
│   ├── nestgate       # 7.9M — Nest: content-addressed storage
│   ├── rhizocrypt     # 5.8M — Nest: DAG lineage
│   ├── loamspine      # 4.8M — Nest: permanent ledger
│   ├── sweetgrass     # 6.1M — Nest: semantic attribution
│   ├── biomeos        # 14M  — Meta: orchestrator
│   ├── squirrel       # 4.6M — Meta: AI coordination
│   ├── petaltongue    # 27M  — Meta: UI/visualization
│   ├── skunkbat       # 2.2M — Defense: threat detection
│   └── primalspring_primal  # 2.0M — Coordination
├── manifest.toml      # Ecosystem genome: primals, springs, atomics
├── checksums.toml     # BLAKE3 hashes per primal per target triple
├── sources.toml       # Repo map for building from source
├── ports.env          # Canonical TCP port assignments + compositions
├── nucleus_launcher.sh  # Dependency-ordered startup + Phase 5 seeding
├── start_primal.sh    # Per-primal startup wrapper (CLI audit map)
├── stop_gate.sh       # Kill all running primals
├── validate_composition.sh  # Post-deployment composition check
├── validate_gate.sh   # Remote gate validation
├── deploy_gate.sh     # SSH/SCP remote deployment
├── deploy_pixel.sh    # ADB deployment (Android/Pixel)
├── fetch.sh           # Download binaries from GitHub Releases
├── harvest.sh         # Build from local source
└── doctor.sh          # Diagnostic tool
```

## The Depot Pattern

### For Downstream Springs

Springs clone plasmidBin as a **binary depot** — a local cache of
pre-built primals. No compilation needed. The workflow:

```bash
# 1. Clone the depot (one-time)
git clone https://github.com/ecoPrimals/plasmidBin.git
cd plasmidBin

# 2. Launch a NUCLEUS for your spring's niche
./nucleus_launcher.sh \
    --family-id my-spring-validation \
    --composition full

# 3. Run your guideStone against the live NUCLEUS
FAMILY_ID=my-spring-validation \
    cargo run --bin myspring_guidestone

# 4. Stop when done
./stop_gate.sh
```

### Family Isolation

Every NUCLEUS launch uses a `--family-id` which namespaces sockets:

```
/run/user/1000/biomeos/{primal}-{family-id}.sock
```

Multiple NUCLEUS instances can run simultaneously with different family
IDs. Springs can validate against different compositions in parallel.

### Composition Selection

The launcher supports named compositions:

| Composition | Primals | Use Case |
|-------------|---------|----------|
| `tower` | beardog, songbird | Trust boundary only |
| `node` | Tower + toadstool, barracuda, coralreef | Compute tier |
| `nest` | Tower + nestgate, rhizocrypt, loamspine, sweetgrass | Storage tier |
| `nucleus` | Tower + Node + Nest (9 core primals) | Full atom |
| `full` | NUCLEUS + biomeos, squirrel, petaltongue | Everything |
| `niche-hotspring` | NUCLEUS (no meta-tier) | hotSpring's niche |
| `niche-neuralspring` | Node + biomeos, squirrel | neuralSpring's niche |
| `niche-healthspring` | Nest + biomeos, squirrel | healthSpring's niche |

### Pre-Flight with primalSpring guideStone

Before running domain guideStones, validate the composition is sound:

```bash
FAMILY_ID=my-spring-validation \
    cargo run --manifest-path path/to/primalSpring/ecoPrimal/Cargo.toml \
    --bin primalspring_guidestone

# Exit 0 → composition valid, proceed with domain guideStone
# Exit 1 → composition broken, fix before domain validation
# Exit 2 → bare checks only, no primals discovered
```

## Current State (April 20, 2026)

### Validated

- **14/14 binaries** present and executable (x86_64 musl-static, stripped)
- **nucleus_launcher.sh**: full 5-phase deployment tested
  - Phase 1: Runtime preparation
  - Phase 2: Stop existing primals
  - Phase 3: Start in dependency order (beardog → petaltongue)
  - Phase 4: Health sweep
  - Phase 5: Songbird registry seeding
- **primalspring_guidestone**: 67/67 ALL PASS against live NUCLEUS

### Known Issues

- **BearDog** requires `BEARDOG_FAMILY_SEED` env var in production mode.
  Launcher should document or auto-generate this. (PG-19)
- **Songbird/petalTongue** speak HTTP on UDS; raw JSON-RPC IPC clients get
  protocol errors. They're reachable but need HTTP framing.
- **BarraCuda** runs degraded without GPU (`cpu-shader only`). This is
  correct behavior on headless/CI machines.
- **aarch64** gaps: nestgate, rhizocrypt, petaltongue, barracuda, coralreef
  have no aarch64 binaries in checksums.

### For CI Pipelines

```bash
# Minimal CI validation loop:
git clone --depth 1 https://github.com/ecoPrimals/plasmidBin.git
cd plasmidBin
export BEARDOG_FAMILY_SEED="ci-$(date +%s)"
./nucleus_launcher.sh --family-id ci-run --composition nucleus
FAMILY_ID=ci-run primalspring_guidestone  # pre-flight
FAMILY_ID=ci-run myspring_guidestone      # domain validation
./stop_gate.sh
```

---

*plasmidBin is the clean-machine deployment path. Pull the repo, launch
the NUCLEUS, validate with your guideStone. No compilation, no package
managers, no network dependencies at runtime. The binaries are the
deployment.*
