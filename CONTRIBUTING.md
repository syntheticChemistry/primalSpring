# Contributing to primalSpring

primalSpring is the coordination and composition validation spring for the
ecoPrimals ecosystem. Contributions should maintain the zero-warning,
zero-unsafe, zero-C-dependency standard.

## Code Standards

- **Edition**: Rust 2024, MSRV 1.87
- **Unsafe**: `deny` at workspace level — allow-listed only for `std::env::set_var` in Rust 2024 entropy bootstrap
- **Lints**: clippy pedantic + nursery, zero warnings required
- **Docs**: `deny(missing_docs)` — all public items documented
- **Dependencies**: pure Rust only, enforced by `deny.toml`
- **Files**: all under 1000 LOC, single-responsibility modules
- **Tests**: extracted to sibling `tests.rs` files when modules grow large

## Validation Experiments

All experiments use the builder pattern:

```rust
ValidationResult::new("Title")
    .with_provenance("crate_name", "date")
    .run("Subtitle", |v| {
        v.check_bool("name", actual, expected);
    });
```

Experiments must:
- Use honest scaffolding (`check_skip` when a primal is unavailable)
- Carry structured provenance via `with_provenance()`
- Use `.or_exit()` instead of `.unwrap()` for fallible operations

## Discovery

Primal code has only self-knowledge. Other primals are discovered at runtime
via 6-tier capability-based discovery. Use `primal_names::` constants for
slugs — never hardcode primal names as string literals.

## Deploy Graphs

New graphs must use the canonical `[[graph.nodes]]` format (biomeOS-native).
The legacy `[[graph.node]]` format is accepted by the parser but should not
be used in new files. Multi-node graphs use `[graph.nodes.*]` subsections.
The `[graph]` section should include an `id` field (lowercase-hyphenated)
alongside `name`. See `graphs/profiles/nucleus.toml` as the gold standard.

All health checks must use JSON-RPC `health.liveness` — raw HTTP health
probing is deprecated.

## Quality Gates

Before submitting:

```bash
cargo check --workspace
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
cargo doc --workspace --no-deps
cargo test --workspace
cargo deny check
```

All must pass with zero warnings.

## License — scyBorg Provenance Trio

All **source code** is AGPL-3.0-or-later. The ecosystem uses the scyBorg
triple-license model (see `SCYBORG_PROVENANCE_TRIO_GUIDANCE.md` in `ecoPrimals/infra/wateringHole/`):

| Layer | License | Applies to |
|-------|---------|------------|
| Code | AGPL-3.0-or-later | All `.rs`, `.toml`, `.sh` files |
| Mechanics | ORC (Open RPG Creative) | Game mechanics if applicable (n/a for primalSpring) |
| Creative | CC-BY-SA 4.0 | Documentation, diagrams, creative content |

primalSpring is a coordination/validation spring with no game mechanics.
ORC does not apply. Documentation and specs are CC-BY-SA 4.0.
Every source file must carry `// SPDX-License-Identifier: AGPL-3.0-or-later`.
