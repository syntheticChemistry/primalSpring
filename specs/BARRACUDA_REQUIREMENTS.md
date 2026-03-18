# primalSpring — BarraCuda Requirements

**Date**: March 17, 2026  
**Status**: Minimal — primalSpring validates coordination, not math (confirmed by audit)

---

## Overview

primalSpring has minimal barraCuda requirements. Its domain is primal
coordination, not numerical computation. It consumes barraCuda only
indirectly — through the primals it orchestrates.

## Direct Requirements

None. primalSpring does not import barraCuda as a dependency.

## Indirect Requirements (via orchestrated primals)

When primalSpring deploys a Full NUCLEUS and runs experiments, the
underlying primals consume barraCuda:

| Primal | barraCuda Usage |
|--------|-----------------|
| ToadStool | GPU dispatch of WGSL shaders |
| BarraCuda | Math kernel library |
| coralReef | Shader compilation |

primalSpring validates that these primals respond correctly to capability
calls. The math correctness is validated by domain springs (hotSpring,
wetSpring, etc.).

## Absorption Candidates

None. primalSpring will not contribute shaders or math to barraCuda.
Its contribution is validating that the coordination layer correctly
composes the primals that consume barraCuda.
