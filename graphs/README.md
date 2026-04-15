# primalSpring Graph Deployments

> 78 graph definitions for NUCLEUS composition validation and deployment.
> Date: April 15, 2026

## NUCLEUS-First Principle

**Every composition starts with NUCLEUS (or a subset). Springs add domain nodes as overlays.**

NUCLEUS is the canonical base deployment: Tower (BearDog + Songbird) + Node (toadStool +
barraCuda + coralReef) + Nest (NestGate + rhizoCrypt + loamSpine + sweetGrass). Springs
compose on top of this by adding their domain-specific nodes as graph overlays. The
`spring_deploy/spring_deploy_template.toml` codifies this pattern.

## Graph Hierarchy

```
fragments/           Atomic building blocks (tower, node, nest, nucleus, meta, provenance)
  └─ README.md       Fragment definitions and canonical names

profiles/            Validated composition profiles (reusable graph patterns)
  ├─ tower.toml      Tower Atomic only
  ├─ node.toml       Tower + Node compute
  ├─ nest.toml       Tower + Nest storage + provenance
  ├─ nucleus.toml    Full NUCLEUS (Tower + Node + Nest)
  └─ full.toml       NUCLEUS + Meta-tier

(root)               Core composition graphs
  ├─ tower_atomic_bootstrap.toml     Minimal Tower bootstrap
  ├─ nucleus_complete.toml           Full NUCLEUS with bonding
  ├─ node_atomic_compute.toml        Compute-focused composition
  └─ ...pipeline.toml                Domain pipeline graphs

patterns/            Minimal coordination pattern demos
  ├─ conditional_fallback.toml       ConditionalDag pattern
  ├─ parallel_capability_burst.toml  Parallel pattern
  ├─ streaming_pipeline.toml         Streaming pattern
  └─ continuous_tick.toml            Tick-loop pattern

spring_deploy/       Production deploy graphs per spring
  ├─ spring_deploy_template.toml     NUCLEUS-base template (copy this)
  ├─ neuralspring_deploy.toml        neuralSpring (gold standard)
  ├─ healthspring_deploy.toml        healthSpring
  ├─ airspring_deploy.toml           airSpring
  ├─ groundspring_deploy.toml        groundSpring
  └─ wetspring_deploy.toml           wetSpring

downstream/          Proto-nucleate graphs for springs/gardens
  └─ *_proto_nucleate.toml           Target composition per spring

spring_validation/   Validation suite graphs (exp-driven)
  └─ *_validate.toml                 Composition health checks

bonding/             Bonding model graphs (ionic, metallic, covalent)
multi_node/          Multi-node / multi-gate graphs (LAN, WAN, HPC)
cross_spring/        Cross-spring sweep graphs
chaos/               Chaos engineering / fault injection graphs
federation/          Content distribution / federation graphs
fossilRecord/        Archived root-level graphs (superseded by patterns/)
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

8 graphs exist in both formats. Each has a `# Source of truth:` comment header:

- **primalSpring owns** the validation/structural variant (with metadata, fragments, bonding)
- **biomeOS owns** the runtime execution variant (with spawn, health_check)

The duplicates: `tower_atomic_bootstrap`, `nucleus_complete`, `node_atomic_compute`,
and 5 spring deploy graphs (`neuralspring`, `healthspring`, `airspring`, `groundspring`,
`wetspring`).

## Adding a New Graph

1. Copy `spring_deploy/spring_deploy_template.toml`
2. Replace `YOURSPRING` with your spring name
3. Set `fragments` to include `"nucleus"` plus any additions
4. Add your domain nodes in the overlay section
5. Ensure all fragment names use canonical vocabulary above
