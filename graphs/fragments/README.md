# Graph Fragments — NUCLEUS Atomic Architecture

Fragments define the **three atomics** that compose into NUCLEUS, plus the
meta-tier primals that operate at any level. They are the "periodic table"
of primal composition — every graph is built from combinations of these
canonical structures.

Fragments are **documentation**, not a runtime merge system. biomeOS deploys
complete graphs; fragments show which canonical patterns each graph includes.

## The Three Atomics

| Fragment | Particle | Primals | Description |
|----------|----------|---------|-------------|
| `tower_atomic` | Electron | BearDog + Songbird | Trust boundary — crypto, discovery, mesh |
| `node_atomic` | Proton | Tower + ToadStool + barraCuda + coralReef | Compute substrate — dispatch, execute, compile |
| `nest_atomic` | Neutron | Tower + NestGate + provenance trio | Storage + provenance — content-addressed data + lineage |

## Supporting Patterns

| Fragment | Primals | Description |
|----------|---------|-------------|
| `nucleus` | Tower + Node + Nest (9 primals) | Full NUCLEUS — all three atomics bound together |
| `meta_tier` | biomeOS + Squirrel + petalTongue | Cross-atomic primals — orchestration, AI, UI |
| `provenance_trio` | rhizoCrypt + loamSpine + sweetGrass | Sub-pattern within Nest — DAG + ledger + attribution |

## Particle Model

From gen3 Paper 23 (mass-energy-information equivalence):

- **Tower = Electron**: Mediates bonding. All inter-gate communication flows through Tower.
- **Node = Proton**: Compute = energy. Fungible (a TFLOP is a TFLOP). Identity of a gate.
- **Nest = Neutron**: Data at rest. Non-fungible, content-addressed. Isotope = data profile.
- **NUCLEUS = Atom**: Tower + Node + Nest bound together. The deployment unit.
- **Meta-tier**: biomeOS, Squirrel, petalTongue — not part of any atomic, available at all levels.

## How Graphs Compose Fragments

Every deploy graph in `graphs/` carries `[graph.metadata]` with a `fragments`
array listing which canonical patterns it includes:

```toml
[graph.metadata]
fragments = ["tower_atomic", "nest_atomic"]
composition_model = "pure"
```

## Composition Models

| Model | Meaning |
|-------|---------|
| `pure` | All nodes are NUCLEUS primals (`spawn = false`). The graph IS the product. |
| `pure_nucleus` | Pure primal NUCLEUS composition for domain validation. Springs validate externally via IPC — no spring binaries as nodes. |
| `validation` | Structural validation graph — primalSpring probes capabilities. |

## The Composition Principle

Complex functions emerge from composing base primals via Neural API graphs.
You never build a new primal to achieve a higher-order capability — you compose
existing ones.

- **ML inference** = Node Atomic (barraCuda matmul/attention + coralReef compile + toadStool dispatch)
- **QCD physics** = Node Atomic (barraCuda df64 + coralReef QCD operators + toadStool GPU fleet)
- **Game science** = Node Atomic (barraCuda Fitts/Perlin/WFC + toadStool dispatch) + meta (Squirrel DDA)
- **CRPG product** = meta (Squirrel narration + petalTongue rendering) + Nest Atomic (NestGate sessions + provenance)

Primals are the instruction set. Graphs are the program. biomeOS is the CPU.
