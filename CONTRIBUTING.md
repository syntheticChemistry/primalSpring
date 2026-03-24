# Contributing to primalSpring

primalSpring is the coordination and composition validation spring for the
ecoPrimals ecosystem. Contributions should maintain the zero-warning,
zero-unsafe, zero-C-dependency standard.

## Code Standards

- **Edition**: Rust 2024, MSRV 1.87
- **Unsafe**: `forbid` at workspace level — no unsafe code anywhere
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
via 5-tier capability-based discovery. Use `primal_names::` constants for
slugs — never hardcode primal names as string literals.

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

## License

AGPL-3.0-or-later (scyBorg trio: AGPL + ORC + CC-BY-SA 4.0)
