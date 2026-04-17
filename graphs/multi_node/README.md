# Multi-Node Bonding Graphs

**Date**: April 13, 2026

These graphs define multi-node deployment topologies that bond multiple NUCLEUS
instances together using the chemistry-inspired bonding model (Covalent, Ionic,
Metallic, Weak, OrganoMetalSalt).

## Schema Differences from Standard Deploy Graphs

Multi-node graphs use a **different TOML dialect** from the single-node
`DeployGraph` format used by `nucleus_complete.toml` and friends.

| Feature | Standard (`DeployGraph`) | Multi-Node |
|---------|--------------------------|------------|
| Node identifier | `name = "beardog"` | `id = "gate_beardog"` |
| Binary field | `binary = "beardog_primal"` (required) | Not present |
| Startup order | `order = 1` (required, unique) | Implicit via `depends_on` |
| Health method | `health_method = "health.liveness"` | Not present |
| Operation | Flat on node | Nested `[graph.nodes.operation]` sub-table |
| Primal routing | `by_capability = "security"` | Nested `[graph.nodes.primal]` sub-table |
| Constraints | Not present | Nested `[graph.nodes.constraints]` sub-table |
| Output label | Not present | `output = "beardog_genesis"` |

As of v0.9.13, the `DeployGraph` parser accepts both dialects:
- `id` is accepted as an alias for `name`
- `binary`, `order`, and `health_method` default to empty/zero when absent
- `primal`, `operation`, `constraints`, `output` are captured as opaque values
- `structural_checks()` skips binary/health/order validation for multi-node graphs

## Bonding Metadata

Each multi-node graph includes `[graph.metadata]` and `[graph.bonding_policy]`
sections that define the trust model, cipher requirements, and bond types between
nodes. These are validated by `bonding::graph_metadata::validate_graph_bonding()`.

## Current Graphs

| Graph | Bond Type | Description |
|-------|-----------|-------------|
| `basement_hpc_covalent.toml` | Covalent | Same-family basement HPC mesh |
| `friend_remote_covalent.toml` | Covalent | Remote friend's machine, same family seed |
| `data_federation_cross_site.toml` | Ionic | Cross-family data federation |
| `idle_compute_federation.toml` | Ionic | Cross-family idle compute sharing |
| `three_node_covalent_cross_network.toml` | Covalent | 3-node cross-network mesh |

## Deployment

These graphs are consumed by `biomeos atomic deploy`, not by `primalSpring`'s
`graph.validate` JSON-RPC method. The biomeOS atomic executor interprets the
nested `operation` sub-tables as execution steps.

```bash
biomeos atomic deploy graphs/multi_node/basement_hpc_covalent.toml \
  --env FAMILY_ID=cf7e8729 \
  --env NODE_ID=northgate \
  --env XDG_RUNTIME_DIR=/run/user/1000
```
