# plasmidBin Depot Pattern — Remote NUCLEUS Deployment

**Date**: April 29, 2026
**From**: primalSpring v0.9.24
**License**: AGPL-3.0-or-later

---

## What plasmidBin Is

`plasmidBin` is the binary distribution channel for ecoPrimals. It
distributes static musl ELF binaries for all 12 NUCLEUS primals —
stripped, cross-compiled, and verified via BLAKE3 checksums. Binaries
are published as GitHub Releases and fetched on demand. No compilation
and no git clone required.

```
~/.local/share/ecoPrimals/plasmidBin/    (XDG default)
├── primals/
│   └── x86_64-unknown-linux-musl/
│       ├── beardog        # Tower: crypto spine
│       ├── songbird       # Tower: discovery + HTTP
│       ├── toadstool      # Node: compute dispatch
│       ├── barracuda      # Node: GPU math (WGSL shaders)
│       ├── coralreef      # Node: shader compiler
│       ├── nestgate       # Nest: content-addressed storage
│       ├── rhizocrypt     # Nest: DAG lineage
│       ├── loamspine      # Nest: permanent ledger
│       ├── sweetgrass     # Nest: semantic attribution
│       ├── biomeos        # Meta: orchestrator
│       ├── squirrel       # Meta: AI coordination
│       ├── petaltongue    # Meta: UI/visualization
│       └── skunkbat       # Defense: threat detection
└── checksums.toml         # BLAKE3 hashes per primal per target triple
```

The plasmidBin GitHub repository also contains:
- `sources.toml` — central registry mapping primals to source repos
- `checksums.toml` — BLAKE3 hashes per primal per target triple
- `manifest.toml` — ecosystem genome: primals, springs, atomics
- `ports.env` — canonical TCP port assignments + compositions
- `fetch.sh` — download binaries from GitHub Releases (used by consumers)
- `harvest.sh` — build from local source and publish to Releases
- `build-primal.sh` — clone + build a single primal from source
- `nucleus_launcher.sh` — dependency-ordered startup + Phase 5 seeding
- `start_primal.sh`, `stop_gate.sh`, `doctor.sh` — operational scripts

**Note**: `primalspring_primal` and `esotericWebb` are NOT primals and
are NOT distributed via plasmidBin. Spring binaries are Rust validation
artifacts. A spring IS a composition of the 12 primals.

## Acquiring Binaries

### For Downstream Springs (Recommended)

Springs use `fetch_primals.sh` or `fetch.sh` to download pre-built
binaries from GitHub Releases into a standard XDG cache location:

```bash
# Option A: primalSpring's fetch_primals.sh (any spring can copy this)
./tools/fetch_primals.sh

# Option B: plasmidBin's fetch.sh (from the repo directly)
curl -sSL https://raw.githubusercontent.com/ecoPrimals/plasmidBin/main/fetch.sh | bash

# Binaries land in $XDG_DATA_HOME/ecoPrimals/plasmidBin/primals/{triple}/
# Override location: export ECOPRIMALS_PLASMID_BIN=/custom/path
```

### Binary Discovery Order

All primalSpring tools and Rust code use the same 3-tier search:

1. `$ECOPRIMALS_PLASMID_BIN` — explicit override (set by user or CI)
2. `$BIOMEOS_PLASMID_BIN_DIR` — biomeOS-provided path
3. `$XDG_DATA_HOME/ecoPrimals/plasmidBin` — default (`~/.local/share/...`)

Within each base directory, the `primals/{target-triple}/{name}` layout
is tried first (matching `fetch.sh` output), then flat `primals/{name}`
as fallback.

### For CI Pipelines

```bash
# Minimal CI validation loop:
./tools/fetch_primals.sh                          # download binaries
export BEARDOG_FAMILY_SEED="ci-$(date +%s)"
./tools/composition_nucleus.sh start              # launch NUCLEUS
FAMILY_ID=ci-run primalspring_guidestone           # pre-flight
FAMILY_ID=ci-run myspring_guidestone               # domain validation
./tools/composition_nucleus.sh stop
```

## CI/CD Auto-Harvest Pipeline

plasmidBin binaries are automatically rebuilt when primals push to `main`:

```
Primal repo push → notify-plasmidbin.yml (repository_dispatch)
                  → auto-harvest.yml (clone, build musl-static, publish Release)
```

**Components** (in `ecoPrimals/plasmidBin`):
- `.github/workflows/auto-harvest.yml` — triggered by `repository_dispatch`,
  `workflow_dispatch` (manual), and weekly `schedule`
- `build-primal.sh` — clones a primal, builds musl-static, stages to
  `/tmp/primalspring-deploy/`. Supports `build_args` (e.g. biomeOS workspace
  `-p biomeos-unibin`) and `needs_sibling` (e.g. skunkBat needs sourDough)
- `templates/notify-plasmidbin.yml` — drop-in workflow for primal repos
- `sources.toml` — maps primal names to GitHub repos with build metadata

**For primal teams**: Copy `templates/notify-plasmidbin.yml` to
`.github/workflows/` in your repo. On push to `main`, it fires a
`repository_dispatch` event to plasmidBin which triggers a rebuild.

## The Depot Pattern

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

### For Interactive Compositions (Shell)

primalSpring provides `composition_nucleus.sh` — a parameterized launcher
that replaces ad-hoc primal startup for interactive development:

```bash
# Launch a full NUCLEUS for your domain
COMPOSITION_NAME=myspring \
    primalSpring/tools/composition_nucleus.sh start

# Check status
COMPOSITION_NAME=myspring primalSpring/tools/composition_nucleus.sh status

# Run your domain composition
COMPOSITION_NAME=myspring bash my_composition.sh

# Shut down
COMPOSITION_NAME=myspring primalSpring/tools/composition_nucleus.sh stop
```

`composition_nucleus.sh` handles:
- Dependency-ordered primal startup (same as nucleus_launcher.sh)
- Family-aware socket naming and capability alias symlinks
- petalTongue live mode (GUI) or server mode (headless)
- Configurable primal list via `PRIMAL_LIST` env var
- Health check summary on startup

See `wateringHole/DOWNSTREAM_COMPOSITION_EXPLORER_GUIDE.md` for the full
composition library API and per-spring exploration guidance.

## Known Issues

- **BearDog** requires `BEARDOG_FAMILY_SEED` env var in production mode.
  Launcher should document or auto-generate this. (PG-19)
- **Songbird/petalTongue** speak HTTP on UDS; raw JSON-RPC IPC clients get
  protocol errors. They're reachable but need HTTP framing.
- **BarraCuda** runs degraded without GPU (`cpu-shader only`). This is
  correct behavior on headless/CI machines.
- **BTSP enforcement**: biomeOS `nucleus` mode enforces BTSP authentication.
  primalSpring's `NeuralBridge` does not yet authenticate — use `api` mode
  for composition validation until BTSP client support is added.

---

*plasmidBin is the clean-machine deployment path. Fetch the binaries,
launch the NUCLEUS, validate with your guideStone. No compilation, no
git clone required, no network dependencies at runtime. The binaries
are the deployment.*
