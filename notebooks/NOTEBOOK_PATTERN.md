# Public Notebook Pattern — primalSpring

How to create public-facing notebooks for your spring. This is
primalSpring's adaptation of the wetSpring exemplar pattern.

## Directory Convention

```
your-spring/
  notebooks/
    NOTEBOOK_PATTERN.md          ← this file (copy to your spring)
    01-domain-validation.ipynb   ← flagship validation story
    02-benchmark-comparison.ipynb← Python vs Rust vs GPU
    03-paper-reproductions.ipynb ← per-researcher evidence
    04-cross-spring.ipynb        ← ecosystem connections
    05-domain-deep-dive.ipynb    ← your most compelling discovery
```

## Cell Structure

Every notebook follows the same structure:

1. **Title cell** (markdown): Title, one-paragraph context, data sources, "for other springs" adaptation note
2. **Imports + data loading** (code): Load from `../experiments/results/*.json`
3. **Domain-specific cells** (code + markdown): Visualization and analysis
4. **Summary cell** (markdown): Validation table, provenance note, links to primals.eco

## Data Loading Pattern

```python
import json
from pathlib import Path

RESULTS = Path('..') / 'experiments' / 'results'

def load(path):
    with open(RESULTS / path) as f:
        return json.load(f)

data = load('composition_validation.json')
```

Notebooks load **frozen data** (committed JSON artifacts), not live API responses.
This means they work without primals running. When Tier 2 JSON-RPC APIs are
available, notebooks can also call primals directly (see Tier 2 stubs).

## Frozen Data for primalSpring

| File | Contents |
|------|----------|
| `composition_validation.json` | Deploy graph stats, bond types, structural checks |
| `test_suite_report.json` | Module-level test counts, timings, categories |
| `experiment_catalog.json` | All 85 experiments, categorized by focus area |
| `security_convergence.json` | BTSP Phase 3 state, PG-55–59 resolution, timeline |
| `cross_spring_matrix.json` | Spring consumption, primal roles, ecosystem flows |
| `benchmark_timing.json` | Compilation, test suite, binary benchmarks, Rust vs Python |

## Visualization Standards

- Use `matplotlib` (available everywhere, renders to static PNG)
- Save figures to `/tmp/primalspring_<notebook>_<chart>.png`
- Color palette: `#2ecc71` (pass/ok), `#e74c3c` (fail), `#3498db` (info)
- Always include chart titles with key numbers

## Adapting for Your Spring

1. Copy this directory structure
2. Replace data paths with your `experiments/results/` JSONs
3. Update the narrative for your domain
4. Keep the cell structure (title → load → analyze → summary)
5. Add your spring to `shared/abg/commons/<spring>-public/notebooks/` symlink
