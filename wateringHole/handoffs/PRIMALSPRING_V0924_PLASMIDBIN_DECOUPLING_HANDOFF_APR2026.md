# primalSpring v0.9.24 — plasmidBin Decoupling + CI/CD Pipeline Handoff

**Date**: April 29, 2026
**Version**: 0.9.24 (Phase 56c)
**From**: primalSpring (coordination and composition validation)
**To**: All primal teams + spring teams + garden teams
**License**: AGPL-3.0-or-later

---

## Summary

Phase 56c decouples primalSpring from direct filesystem coupling to the
`plasmidBin` repository and establishes the standard consumer pattern that
all downstream springs and deployments should follow. A CI/CD pipeline
now auto-harvests primal binaries from source on push, distributing them
via GitHub Releases.

**Key outcomes**:
- 20 files modified to remove all `../plasmidBin` and `../../infra/plasmidBin` paths
- Binary discovery standardized: `$ECOPRIMALS_PLASMID_BIN` → XDG cache
- `tools/fetch_primals.sh` bootstraps binaries from GitHub Releases
- plasmidBin CI/CD pipeline live (`auto-harvest.yml` + `notify-plasmidbin.yml`)
- GAP-27 (stale biomeOS binary) resolved — CI auto-rebuilds from source

---

## What Changed

### Standard Consumer Pattern

primalSpring no longer assumes a sibling `plasmidBin/` directory. All
binary discovery follows a 3-tier search:

1. `$ECOPRIMALS_PLASMID_BIN` — explicit override (CI, custom installs)
2. `$BIOMEOS_PLASMID_BIN_DIR` — biomeOS-provided path
3. `$XDG_DATA_HOME/ecoPrimals/plasmidBin` — default (`~/.local/share/...`)

Within each base directory, `primals/{target-triple}/{name}` is tried
first (matching `fetch.sh` output), then flat `primals/{name}` fallback.

**This is the pattern all springs and deployments should adopt.**

### Files Modified

**Rust source** (4 files):
- `ecoPrimal/src/launcher/discovery.rs` — rewrote `discover_binary()` to
  use 3-tier search with `primals/{triple}/{name}` layout. Removed
  `RELATIVE_PLASMID_TIERS` (`./plasmidBin`, `../plasmidBin`, `../../plasmidBin`).
  Added `xdg_plasmid_bin()` and `host_target_triple()`.
- `ecoPrimal/src/launcher/tests.rs` — new test asserting no relative
  `../plasmidBin` traversal occurs
- `ecoPrimal/src/launcher/mod.rs` — updated module docs
- `ecoPrimal/src/bin/primalspring_guidestone/entropy.rs` — replaced
  `../../infra/plasmidBin` with XDG default for seed fingerprinting

**Shell tools** (10 files):
- `tools/desktop_nucleus.sh`, `tools/composition_nucleus.sh`,
  `tools/cell_launcher.sh`, `tools/ttt_nucleus.sh`, `tools/live_nucleus.sh`,
  `tools/nucleus_launcher.sh`, `tools/nucleus_composition_lib.sh`,
  `tools/gen_seed_fingerprints.sh` — all replaced `$ECO_ROOT/infra/plasmidBin`
  fallbacks with `${ECOPRIMALS_PLASMID_BIN:-${XDG_DATA_HOME:-$HOME/.local/share}/ecoPrimals/plasmidBin}`

**Scripts** (4 files):
- `scripts/build_ecosystem_musl.sh`, `scripts/build_ecosystem_genomeBin.sh`,
  `scripts/validate_release.sh`, `scripts/validate_deployment_matrix.sh`,
  `scripts/validate_local_lab.sh`, `scripts/validate_remote_gate.sh` —
  same XDG default pattern

**Validation/Experiments** (2 files):
- `validation/seed_fingerprints.toml` — source path updated
- `experiments/exp065_petaltongue_tower_dashboard/src/main.rs` — uses
  `discover_binary()` instead of custom path search

**New files** (2):
- `tools/fetch_primals.sh` — self-contained bootstrap script
- `.cursorignore` — excludes `target/` from IDE indexing

### plasmidBin CI/CD Pipeline

Deployed to `ecoPrimals/plasmidBin`:

```
Primal repo push to main
  → .github/workflows/notify-plasmidbin.yml fires repository_dispatch
  → plasmidBin .github/workflows/auto-harvest.yml triggers
  → build-primal.sh clones, builds musl-static, stages
  → harvest.sh publishes to GitHub Releases with BLAKE3 checksums
```

**Components**:
- `auto-harvest.yml` — triggered by `repository_dispatch`, `workflow_dispatch`
  (manual), and weekly `schedule`. Concurrency-controlled (one build at a time).
- `build-primal.sh` — clones a primal, builds musl-static, stages to
  `/tmp/primalspring-deploy/`. Supports `build_args` for workspace crates
  and `needs_sibling` for cross-repo dependencies.
- `sources.toml` — central registry: 12 primals mapped to GitHub repos with
  `tag_pattern`, `assets`, `build_args`, `needs_sibling` metadata.
- `templates/notify-plasmidbin.yml` — drop-in workflow template for primal repos.

**What plasmidBin distributes**: Exactly 12 NUCLEUS primals. `primalspring_primal`
and `esotericWebb` are NOT primals and are NOT distributed.

---

## For Primal Teams

### Add the Notification Workflow

Copy `ecoPrimals/plasmidBin/templates/notify-plasmidbin.yml` to
`.github/workflows/notify-plasmidbin.yml` in your repo. On push to `main`,
this fires a `repository_dispatch` event to plasmidBin triggering a rebuild
of your binary.

### Verify Your sources.toml Entry

Check `ecoPrimals/plasmidBin/sources.toml` for your primal. If your repo
requires special build flags or sibling dependencies, add:

```toml
[primals.yourprimal]
repo = "ecoPrimals/yourPrimal"
tag_pattern = "v*"
assets = ["yourprimal"]
build_args = "-p your-binary-crate"     # if workspace member
needs_sibling = "ecoPrimals/sibling"    # if cross-repo dependency
```

### Learnings from CI Builds

- **biomeOS**: required `build_args = "-p biomeos-unibin"` because the
  default `cargo build` builds only the library in a workspace
- **skunkBat**: required `needs_sibling = "ecoPrimals/sourDough"` for
  a cross-repo dependency that must be cloned adjacent

---

## For Spring Teams

### Consuming Binaries

```bash
# One-time bootstrap (or after primal updates):
./tools/fetch_primals.sh

# Binaries land in ~/.local/share/ecoPrimals/plasmidBin/primals/{triple}/
# Override: export ECOPRIMALS_PLASMID_BIN=/custom/path

# Launch NUCLEUS:
./tools/composition_nucleus.sh start

# Validate:
FAMILY_ID=my-validation cargo run --bin myspring_guidestone
```

### Rust Binary Discovery

Use `primalspring::launcher::discover_binary()`:

```rust
use primalspring::launcher::discover_binary;

let biomeos = discover_binary("biomeos")?;
let beardog = discover_binary("beardog")?;
```

This searches the 3-tier hierarchy and returns the first executable found.
No relative path assumptions. Works identically in dev and CI.

### DO NOT

- Clone `plasmidBin` as a sibling directory
- Hardcode paths like `../plasmidBin` or `../../infra/plasmidBin`
- Assume `plasmidBin` is a git submodule or workspace member
- Distribute `primalspring_primal` via plasmidBin (it's a dev artifact)

---

## NUCLEUS Composition Patterns

### biomeOS Neural API Deployment

The canonical deployment flow for any NUCLEUS composition:

1. `fetch_primals.sh` bootstraps binaries from GitHub Releases
2. `biomeos neural-api --tcp-only --port 9000 --graphs-dir ./graphs` starts the substrate
3. `tower_atomic_bootstrap.toml` is the genesis graph (BearDog + Songbird)
4. `primal_launch_profiles.toml` defines per-primal CLI args, env, and security
5. The Neural API routes `capability.call` RPCs to the correct primal
6. Cross-architecture: `--tcp-only` for Android/Windows (no Unix sockets)
7. `tcp_rpc_multi_protocol` auto-detects raw TCP vs HTTP POST per primal

### biomeOS Modes

- `biomeos api` — supports `capability.discover` and `capability.call`.
  Use for composition validation (what primalSpring's `NeuralBridge` uses).
- `biomeos neural-api` — graph execution mode. Use for production deployments.
- `biomeos nucleus` — full NUCLEUS orchestration with BTSP enforcement.
  Note: primalSpring's `NeuralBridge` does not yet authenticate via BTSP,
  so composition validation currently requires `api` mode.

---

## Learnings and Evolution Gaps

### BTSP Enforcement

biomeOS `nucleus` mode enforces BTSP authentication on all incoming RPC.
primalSpring's `NeuralBridge` sends unauthenticated JSON-RPC, which gets
rejected with empty responses. Until `NeuralBridge` implements the BTSP
4-step handshake client, composition validation must use `api` mode.

**Action for biomeOS team**: Consider a `--btsp-optional` flag for the
`nucleus` subcommand to allow mixed authenticated/unauthenticated clients
during development.

**Action for primalSpring**: Implement BTSP client handshake in
`NeuralBridge` to enable full `nucleus` mode validation.

### Socket Path Evolution

Newer primal versions create sockets at different paths than older versions.
`desktop_nucleus.sh` creates capability-aliased symlinks to bridge the gap.
The long-term solution is biomeOS socket registry with well-known paths.

### Partial Build Resilience

`build-primal.sh` in `--all` mode logs warnings for partial failures and
continues to harvest successful builds. This prevents a single broken
primal from blocking distribution of the other 11.

### Private Repos

BearDog is currently private. The CI pipeline supports private repos via
PAT authentication in `HARVEST_PAT`. Binaries are public (via GitHub
Releases) even when source is private.

---

## GAP-27 Resolution

The stale biomeOS binary in plasmidBin (pre-v3.31) that blocked exp106
graph management (3 FAILs) is now resolved by the CI/CD pipeline. biomeOS
is auto-built from source with `build_args = "-p biomeos-unibin"` and
distributed via GitHub Releases. Downstream consumers run `fetch_primals.sh`
to get the updated binary.

---

**License**: AGPL-3.0-or-later
