# SPDX-License-Identifier: CC-BY-SA-4.0

# Graph Consolidation & NUCLEUS Deployment Handoff

**Date**: April 16, 2026
**From**: primalSpring (coordination spring)
**To**: All spring teams — airSpring, groundSpring, wetSpring, hotSpring, neuralSpring, healthSpring, ludoSpring, esotericWebb
**Version**: v0.9.15

---

## 1. Graph Consolidation Summary

primalSpring's deploy graph inventory has been consolidated from **78 to 56 TOMLs**
through three structural changes:

| Pattern | Before | After | Mechanism |
|---------|--------|-------|-----------|
| Spring validation | 13 files (7 per-spring + 6 composition + 2 infra) | 4 files | Template + manifest + 2 unique |
| Spring deploy | 5 per-spring files | 2 files | Template + manifest |
| Downstream proto-nucleate | 7 per-spring files | 3 files | Template + manifest + 1 unique |
| Root duplicates | `primalspring_deploy.toml`, `full_overlay.toml` | 0 | Absorbed into `nucleus_complete.toml` and `profiles/full.toml` |
| fossilRecord/graphs | 2 stale snapshots | 0 | Deleted |

### Template + Manifest Pattern

Instead of N nearly-identical TOML files differing only in spring name and capabilities,
we now have:

- **Template** (`*_template.toml`): The structural graph skeleton with placeholder variables.
- **Manifest** (`*_manifest.toml`): A table of per-spring parameters. biomeOS reads
  the manifest to instantiate concrete graphs from the template.

Files:

| Directory | Template | Manifest | Parameters |
|-----------|----------|----------|------------|
| `graphs/spring_validation/` | `spring_validate_template.toml` | `spring_validate_manifest.toml` | 6 springs + 9 composition subsystem validators |
| `graphs/spring_deploy/` | `spring_deploy_template.toml` | `spring_deploy_manifest.toml` | 5 science springs |
| `graphs/downstream/` | `proto_nucleate_template.toml` | `downstream_manifest.toml` | 7 springs |

### Fragment-First Composition (`resolve = true`)

Profiles (`graphs/profiles/*.toml`) no longer duplicate the full node list. Instead:

1. Declare `resolve = true` and `fragments = [...]` in `[graph.metadata]`.
2. At load time, `load_graph()` calls `resolve_fragments()` which reads each
   fragment from `graphs/fragments/*.toml` and merges them as a base node layer.
3. The profile's own `[[graph.nodes]]` entries override or extend the fragment base.

This makes profiles ~15 lines instead of ~40, and ensures fragment changes propagate
automatically to all profiles that reference them.

**6 canonical fragments** (`graphs/fragments/`):

| Fragment | Primals | Particle Role |
|----------|---------|---------------|
| `tower_atomic` | BearDog + Songbird | Electron (trust boundary) |
| `node_atomic` | Tower + ToadStool + barraCuda + coralReef | Proton (compute) |
| `nest_atomic` | Tower + NestGate + provenance trio | Neutron (storage) |
| `nucleus` | Tower + Node + Nest | Atom (full composition) |
| `meta_tier` | biomeOS + Squirrel + petalTongue | Cross-atomic orchestration |
| `provenance_trio` | rhizoCrypt + loamSpine + sweetGrass | Nest sub-pattern |

---

## 2. What Changed for Springs

### Your spring's deploy and validation graphs are now parameterized

If you previously referenced individual files like `airspring_validate.toml` or
`neuralspring_deploy.toml`, those files no longer exist. Your spring's parameters
now live in the corresponding manifest:

| Old reference | New location |
|---------------|-------------|
| `*_validate.toml` (per-spring) | `spring_validate_manifest.toml` → `[[spring]]` entry |
| `*_deploy.toml` (per-spring) | `spring_deploy_manifest.toml` → `[[spring]]` entry |
| `*_proto_nucleate.toml` (per-spring) | `downstream_manifest.toml` → `[[downstream]]` entry |
| `healthspring_enclave_proto_nucleate.toml` | **Unchanged** (unique dual-tower pattern) |

### Your spring's composition subsystem validators are consolidated

The 9 composition validators (C1–C7 + infra) that were individual TOMLs are now
`[[composition]]` entries in `spring_validate_manifest.toml`. If you reference
specific composition validators, update to the manifest.

### `full_overlay.toml` and `primalspring_deploy.toml` no longer exist

- `full_overlay.toml` → Use `profiles/full.toml` (thin profile, `resolve = true`)
- `primalspring_deploy.toml` → Use `nucleus_complete.toml` (the canonical NUCLEUS graph)

---

## 3. NUCLEUS Deployment via NeuralAPI

### Canonical deployment command

```bash
biomeos deploy --graph graphs/nucleus_complete.toml
```

For profiles (subsets of NUCLEUS):

```bash
biomeos deploy --graph graphs/profiles/tower.toml
biomeos deploy --graph graphs/profiles/node.toml
biomeos deploy --graph graphs/profiles/nest.toml
biomeos deploy --graph graphs/profiles/full.toml
```

### How `load_graph()` works now

```
1. Parse TOML → GraphMeta
2. If metadata.resolve == true && metadata.fragments is non-empty:
   a. For each fragment name in metadata.fragments:
      - Load graphs/fragments/{name}.toml
      - Extract its [[graph.nodes]]
   b. Merge all fragment nodes as base layer (ordered by fragment order)
   c. Apply the graph's own [[graph.nodes]] as overrides on top
3. Return the fully resolved GraphMeta
```

### Creating a custom profile for your spring

If your spring needs a custom NUCLEUS subset, create a thin profile:

```toml
[graph]
name = "yourspring_custom_profile"
description = "Custom NUCLEUS profile for yourSpring"
pattern = "Sequential"
version = "1.0.0"

[graph.metadata]
fragments = ["tower_atomic", "node_atomic"]
resolve = true
composition_model = "nucleated"

[[graph.nodes]]
name = "yourspring_domain_overlay"
binary = "yourspring_primal"
by_capability = "your_domain"
order = 20
capabilities = ["your.capability"]
```

The fragment nodes are inherited; you only specify your delta.

---

## 4. Primal Evolution Patterns Learned

These patterns emerged from the stadial parity sweep and are now ecosystem standard:

### Tower Atomic Delegation
- **Songbird provides TLS** — no spring should bundle `ring`, `rustls`, or any TLS crate.
- **BearDog provides crypto** — no spring should bundle `ed25519-dalek`, `x25519-dalek`, or
  direct crypto primitives. Use `crypto.hash`, `crypto.sign`, `crypto.verify` via IPC.
- If your `Cargo.lock` contains `ring` or `aws-lc-rs` transitively, audit whether it's
  actually compiled. If it comes from a feature-gated path you don't use, it's ghost debt.

### `#[expect]` over `#[allow]`
All lint suppressions should use `#[expect(lint_name, reason = "...")]` instead of
`#[allow(lint_name)]`. The `expect` attribute will warn if the lint is ever fixed,
preventing stale suppressions from accumulating.

### Edition 2024
All workspace members should declare `edition = "2024"` and `rust-version = "1.87"`.

### `deny.toml` discipline
Every workspace should have a `deny.toml` enforcing:
- License allowlist (AGPL-compatible)
- C dependency ban (ecoBin compliance)
- Advisory database checks

### Zero `dyn` / zero `async-trait`
Modern Rust (1.75+) supports `async fn` in traits natively (RPITIT). All `dyn Trait`
dispatch and `#[async_trait]` usage should be replaced with concrete generics or
`impl Trait` returns.

---

## 5. What Springs Should Absorb

### Update local deploy graph copies
If your spring maintains local copies of primalSpring deploy graphs, delete them and
reference the canonical versions in `primalSpring/graphs/`. The manifests are the
single source of truth for your spring's parameters.

### Reference manifests, not individual files
Update any scripts, CI, or documentation that references individual per-spring graph
files to point to the manifest + template pair instead.

### Adopt `resolve = true` for custom profiles
If your spring defines custom NUCLEUS profiles, adopt the fragment-first pattern.
This ensures your profiles automatically inherit fragment updates.

### Check your `Cargo.lock` for ghost debt
Run `cargo deny check` and `cargo tree -e features` to verify no banned C dependencies
are compiled. Transitive `ring`/`sled`/`reqwest` appearances in lockfiles are common
ghost debt.

---

## 6. Action Items per Spring

### airSpring
- [ ] Update any references to `airspring_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `airspring_ecology_proto_nucleate.toml` → `downstream_manifest.toml`
- [ ] Verify `deny.toml` bans ecoBin C deps

### groundSpring
- [ ] Update any references to `groundspring_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `groundspring_geoscience_proto_nucleate.toml` → `downstream_manifest.toml`
- [ ] Verify `deny.toml` bans ecoBin C deps

### wetSpring
- [ ] Update any references to `wetspring_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `wetspring_lifescience_proto_nucleate.toml` → `downstream_manifest.toml`
- [ ] Update any references to `wetspring_deploy.toml` → `spring_deploy_manifest.toml`

### hotSpring
- [ ] Update any references to `hotspring_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `hotspring_qcd_proto_nucleate.toml` → `downstream_manifest.toml`

### neuralSpring
- [ ] Update any references to `neuralspring_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `neuralspring_inference_proto_nucleate.toml` → `downstream_manifest.toml`
- [ ] Update any references to `neuralspring_deploy.toml` → `spring_deploy_manifest.toml`
- [ ] Unique role: as AI provider, every other spring gains inference capabilities through your evolution

### healthSpring
- [ ] Update any references to `healthspring_validate.toml` → `spring_validate_manifest.toml`
- [ ] `healthspring_enclave_proto_nucleate.toml` is **unchanged** — your unique dual-tower ionic bridge pattern is preserved as a standalone graph
- [ ] Update any references to `healthspring_deploy.toml` → `spring_deploy_manifest.toml`

### ludoSpring
- [ ] Update any references to `ludospring_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `ludospring_proto_nucleate.toml` → `downstream_manifest.toml`
- [ ] ludoSpring is a pure composition — the graph IS the product, biomeOS IS the execution engine

### esotericWebb
- [ ] Update any references to `esotericwebb_validate.toml` → `spring_validate_manifest.toml`
- [ ] Update any references to `esotericwebb_proto_nucleate.toml` → `downstream_manifest.toml`
- [ ] esotericWebb is a pure composition — the graph IS the product, biomeOS IS the execution engine

---

## 7. Key Files Reference

| File | Purpose |
|------|---------|
| `graphs/README.md` | Complete graph hierarchy and fragment resolution documentation |
| `graphs/fragments/*.toml` | 6 canonical atomic building blocks |
| `graphs/profiles/*.toml` | 9 thin compositions with `resolve = true` |
| `graphs/spring_validation/spring_validate_manifest.toml` | All spring + composition validation parameters |
| `graphs/spring_deploy/spring_deploy_manifest.toml` | All spring deploy parameters |
| `graphs/downstream/downstream_manifest.toml` | All downstream proto-nucleate parameters |
| `ecoPrimal/src/deploy/mod.rs` | `load_graph()` with fragment resolution |
| `wateringHole/PRIMALSPRING_COMPOSITION_GUIDANCE.md` | Full composition guidance for springs |
| `wateringHole/NUCLEUS_SPRING_ALIGNMENT.md` | Spring × atomic alignment matrix |

---

*This handoff documents the state of primalSpring v0.9.15 as of April 16, 2026.
570 tests, 75 experiments, 56 deploy graphs, 0 clippy warnings, 0 dyn, 0 async-trait.*
