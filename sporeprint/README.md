# sporeprint/ — Content for primals.eco

**Owned by primalSpring** (Wave 46+). sporePrint repo at `infra/sporePrint`. Wave 47: bash→Rust complete (`render-notebooks`, `fetch-refresh` subcommands in `spore-validate`).

Files in this directory are published to [primals.eco](https://primals.eco) via
the sporePrint auto-refresh CI pipeline.

## How it works

1. When you push to `main`, your `notify-sporeprint.yml` workflow fires
2. If your dispatch payload includes `"content": "true"`, sporePrint CI
   clones this repo and copies `sporeprint/*.md` into `content/lab/`
3. A PR is created for human review before merging to the live site

## What goes here

- `validation-summary.md` — your spring's headline validation results
- Additional `.md` pages with Zola-compatible front matter
- Results, benchmarks, or experiment summaries you want visible on primals.eco

## Notebooks

Notebooks live in `notebooks/` (sibling to this directory) and are rendered
separately by sporePrint's `render_notebooks.sh`. See `notebooks/NOTEBOOK_PATTERN.md`
for the full pattern. Frozen data lives in `experiments/results/*.json`.

## Front matter requirements

Every `.md` file needs Zola TOML front matter with `[taxonomies]` for cross-referencing:

```toml
+++
title = "Your Page Title"
description = "One-line summary"
date = 2026-05-06

[taxonomies]
primals = ["barracuda", "toadstool"]
springs = ["yourspring"]
+++
```

See [CONTENT_GUIDE.md](https://github.com/ecoPrimals/wateringHole/blob/main/sporePrint/CONTENT_GUIDE.md)
for full documentation.

## Rust Evolution Roadmap

sporePrint is evolving toward Rust-native tooling:

- **`spore-validate`** (`infra/sporePrint/crates/spore-validate/`) — entity registry validation, link-lint, content taxonomy checks. Already Rust.
- **`render_notebooks.sh`** — replace with Rust nbconvert equivalent
- **`refresh-metrics.sh`** — replace with Rust metric sync (currently wraps `spore-validate refresh`)
- **Living content (S6)** — dynamic content via NestGate `content.put` instead of static GitHub Pages
- **Sovereign deploy** — cellMembrane VPS (`membrane.primals.eco`) instead of GitHub Pages fallback

Zola itself is already a Rust binary. The evolution is Rust-native tooling around Zola
plus dynamic content delivery off static Pages.
