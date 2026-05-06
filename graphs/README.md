# primalSpring Graph Deployments

> 74 graph definitions for NUCLEUS composition validation and deployment.
> Fragment-first composition: profiles declare fragments + delta nodes;
> `load_graph()` resolves `resolve = true` graphs at parse time.
> Date: May 6, 2026

## NUCLEUS-First Principle

**Every composition starts with NUCLEUS (or a subset). Springs add domain nodes as overlays.**

NUCLEUS is the canonical base deployment: Tower (BearDog + Songbird) + Node (toadStool +
barraCuda + coralReef) + Nest (NestGate + rhizoCrypt + loamSpine + sweetGrass). Springs
compose on top of this by adding their domain-specific nodes as graph overlays. The
`spring_deploy/spring_deploy_template.toml` codifies this pattern.

## Graph Hierarchy

```
fragments/           6 canonical building blocks (unchanged)
  └─ README.md

profiles/            9 thin compositions (fragment refs + delta nodes, resolve = true)
  ├─ tower.toml      Tower Atomic only
  ├─ tower_ai.toml   Tower + Squirrel (AI)
  ├─ tower_viz.toml  Tower + Squirrel + petalTongue
  ├─ node.toml       Tower + Node compute
  ├─ node_ai.toml    Node + Squirrel (AI)
  ├─ nest.toml       Tower + Nest storage + provenance
  ├─ nest_viz.toml   Nest + petalTongue
  ├─ nucleus.toml    Full NUCLEUS (Tower + Node + Nest)
  └─ full.toml       NUCLEUS + Meta-tier

(root)               ~11 core composition graphs
  ├─ tower_atomic_bootstrap.toml     Minimal Tower bootstrap
  ├─ nucleus_complete.toml           Full NUCLEUS + coordination
  ├─ node_atomic_compute.toml        Compute-focused composition
  └─ ...pipeline.toml                Domain pipeline graphs

patterns/            4 coordination pattern demos
  ├─ conditional_fallback.toml       ConditionalDag pattern
  ├─ parallel_capability_burst.toml  Parallel pattern
  ├─ streaming_pipeline.toml         Streaming pattern
  └─ continuous_tick.toml            Tick-loop pattern

spring_deploy/       2 files (template + manifest)
  ├─ spring_deploy_template.toml     NUCLEUS-base template
  └─ spring_deploy_manifest.toml     5 spring parameters

downstream/          3 TOML + 2 markdown docs
  ├─ proto_nucleate_template.toml    NUCLEUS-base template
  ├─ downstream_manifest.toml        7 downstream parameters
  ├─ healthspring_enclave_proto_nucleate.toml  (unique dual-tower)
  ├─ README.md
  └─ NICHE_STARTER_PATTERNS.md

spring_validation/   5 files (template + manifest + 3 unique)
  ├─ spring_validate_template.toml   Per-spring validation template
  ├─ spring_validate_manifest.toml   6 springs + 9 compositions
  ├─ nucleus_atomics_validate.toml   Multi-phase NUCLEUS validation
  ├─ crypto_negative_validate.toml   Rejection path testing
  └─ gaming_niche_validate.toml      Gaming niche composition validation

compositions/        1 purpose-driven composition graph
  └─ foundation_validation.toml    Foundation sediment pipeline (12 nodes, fallback=skip on optional)

bonding/             5 genuinely unique bonding topologies
multi_node/          5 multi-gate federation graphs
cross_spring/        2 cross-spring validators
chaos/               2 fault injection graphs
federation/          1 content distribution
```

## Canonical Fragment Names

All graphs MUST use these 6 fragment names in their `fragments = [...]` declaration:

| Fragment | What It Means |
|----------|--------------|
| `tower_atomic` | BearDog + Songbird (trust boundary) |
| `node_atomic` | toadStool + barraCuda + coralReef (compute substrate) |
| `nest_atomic` | NestGate + provenance trio (storage + lineage) |
| `nucleus` | Tower + Node + Nest (the full atom) |
| `meta_tier` | biomeOS + Squirrel + petalTongue (orchestration + AI + UI) |
| `provenance_trio` | rhizoCrypt + loamSpine + sweetGrass (witness/ledger/attribution) |

Non-canonical names (`"node"`, `"nest"`, `"ai"`, `"visualization"`, `"provenance"`) have been
migrated to their canonical equivalents as of April 15, 2026.

## Dual-Format Story

There are two graph format conventions in the ecosystem:

### primalSpring format (`[[graph.nodes]]`)

Used for **validation and structural specification**. Includes `[graph.metadata]`
(security_model, transport, composition_model, fragments), `[graph.bonding_policy]`,
and per-node `security_model`, `capabilities` arrays.

```toml
[graph]
name = "example"
coordination = "sequential"

[graph.metadata]
fragments = ["tower_atomic", "nucleus"]

[[graph.nodes]]
name = "beardog"
binary = "beardog"
order = 1
```

### biomeOS format (`[[nodes]]`)

Used for **runtime execution** by the biomeOS graph executor. Simpler format
focused on spawn/health/dependency rather than structural metadata.

```toml
[[nodes]]
name = "beardog"
binary = "beardog"
order = 1
spawn = true
health_check = "health.liveness"
```

### Ownership

3 graphs exist in both formats. Each has a `# Source of truth:` comment header:

- **primalSpring owns** the validation/structural variant (with metadata, fragments, bonding)
- **biomeOS owns** the runtime execution variant (with spawn, health_check)

The duplicates: `tower_atomic_bootstrap`, `nucleus_complete`, `node_atomic_compute`.
Spring deploy graphs are now consolidated into template + manifest.

## Fragment Resolution (`resolve = true`)

Profiles declare `resolve = true` in `[graph.metadata]`. When `load_graph()` sees this
flag, it loads each fragment TOML, merges their nodes as the base layer, then overlays
the graph's own `[[graph.nodes]]` as delta overrides. This keeps profiles thin (~10 lines)
while fragments remain the canonical source of truth for node definitions.

## Adding a New Graph

1. Copy `spring_deploy/spring_deploy_template.toml`
2. Replace `YOURSPRING` with your spring name
3. Set `fragments` to include `"nucleus"` plus any additions
4. Add your domain nodes in the overlay section
5. Ensure all fragment names use canonical vocabulary above
