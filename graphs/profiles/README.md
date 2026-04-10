# NUCLEUS Profiles — Atomic Deployment Tiers

Profiles are **slices of NUCLEUS** — each activates a subset of the three
atomics, optionally with meta-tier primals. They replace the old root-level
subset graphs with an architecturally aligned naming scheme.

## The Three Atomics

| Profile | Atomic | Particle | Primals |
|---------|--------|----------|---------|
| `tower` | Tower | Electron | BearDog + Songbird |
| `node` | Node | Proton | Tower + ToadStool + barraCuda + coralReef |
| `nest` | Nest | Neutron | Tower + NestGate + provenance trio |

## Compound Profiles

| Profile | Composition | Description |
|---------|------------|-------------|
| `nucleus` | Tower + Node + Nest | Full NUCLEUS — all 9 domain primals |
| `full` | NUCLEUS + meta-tier | Maximum deployment — all primals including biomeOS, Squirrel, petalTongue |

## Meta-Tier Overlays

| Profile | Base + Meta | Description |
|---------|------------|-------------|
| `tower_ai` | Tower + Squirrel | Trust boundary with AI coordination |
| `tower_viz` | Tower + Squirrel + petalTongue | Trust boundary with AI + visualization |
| `node_ai` | Node + Squirrel | Compute tier with AI-directed workloads |
| `nest_viz` | Nest + petalTongue | Storage tier with visual dashboards |

## Usage

```bash
biomeos deploy --graph graphs/profiles/tower.toml     # minimal
biomeos deploy --graph graphs/profiles/nucleus.toml    # full NUCLEUS
biomeos deploy --graph graphs/profiles/full.toml       # everything
```

## Relationship to nucleus_complete.toml

`nucleus_complete.toml` remains the canonical **reference graph** at root
level with every primal declared. Profiles are subsets — each carries
`base = "nucleus_complete"` in its metadata to express this relationship.

## Deployment Class Axis (gen4)

From GUIDESTONE: `NUCLEUS → Niche → fieldMouse`

- **NUCLEUS**: Full primal composition (profiles in this directory)
- **Niche**: Domain-specific deployment (spring deploy graphs)
- **fieldMouse**: Lightweight edge deployment (future)
