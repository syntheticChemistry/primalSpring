# Graph Fragments — Canonical Composition Patterns

Fragments define the **reusable building blocks** that deploy graphs compose.
They are the "periodic table" of NUCLEUS composition — every graph is built
from combinations of these canonical patterns.

Fragments are **documentation**, not a runtime merge system. biomeOS deploys
complete graphs; fragments show which canonical patterns each graph includes.

## Fragments

| Fragment | Nodes | Description |
|----------|-------|-------------|
| `tower_base` | biomeOS + BearDog + Songbird | Minimum viable composition: orchestration + security + discovery |
| `provenance_trio` | rhizoCrypt + loamSpine + sweetGrass | DAG + ledger + attribution chain for any lineage workflow |
| `wgsl_shader_pipeline` | coralReef + toadStool + barraCuda | Compile + dispatch + execute for GPU/CPU compute |
| `nucleus_core` | Tower Base + toadStool + NestGate + Squirrel | 6 core primals forming a complete NUCLEUS |

## How Graphs Compose Fragments

Every deploy graph in `graphs/` carries `[graph.metadata]` with a `fragments`
array listing which canonical patterns it includes:

```toml
[graph.metadata]
fragments = ["tower_base", "provenance_trio"]
composition_model = "pure"  # no downstream binary spawned
```

## Composition Models

| Model | Meaning |
|-------|---------|
| `pure` | All nodes are NUCLEUS primals (`spawn = false`). The graph IS the product. |
| `nucleated` | NUCLEUS base + a downstream spring binary (`spawn = true`). The spring discovers primals via IPC. |
| `validation` | Structural validation graph — primalSpring probes capabilities. |

## The Composition Principle

Complex functions emerge from composing base primals via Neural API graphs.
You never build a new primal to achieve a higher-order capability — you compose
existing ones.

- **ML inference** = barraCuda matmul/attention + coralReef compile + toadStool dispatch
- **QCD physics** = barraCuda df64 + coralReef QCD operators + toadStool GPU fleet
- **Game science** = barraCuda Fitts/Perlin/WFC + toadStool dispatch + Squirrel DDA
- **CRPG product** = Squirrel narration + petalTongue rendering + NestGate sessions + provenance trio

Primals are the instruction set. Graphs are the program. biomeOS is the CPU.
